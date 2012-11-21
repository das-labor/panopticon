#include <iostream>
#include <iomanip>
#include <numeric>
#include <functional>
#include <algorithm>

#include <disassembler.hh>

#define AVR_PRIVATE
#include <avr/avr.hh>
#include <avr/until.hh>

using namespace po;
using namespace po::avr;

namespace po
{
	namespace avr
	{
		unsigned int next_unused = 0;
	}
}

template<>
lvalue po::temporary(avr_tag)
{
	return variable("t" + std::to_string(po::avr::next_unused++));
}

flow_ptr po::avr::disassemble(std::vector<typename architecture_traits<avr_tag>::token_type> &bytes, addr_t entry)
{
	disassembler<avr_tag> main;

	// memory operations
	main | "001011 r@. d@..... r@...." 	= binary_reg("mov",[](cg &m, const variable &Rd, const variable &Rr)
	{
		m.assign(Rd,Rr);
	});

	main | "00000001 d@.... r@...." 		= [](sm &st)
	{ 
		variable Rd1 = decode_reg(st.capture_groups["d"]), Rd2 = decode_reg(st.capture_groups["d"] + 1);
		variable Rr1 = decode_reg(st.capture_groups["r"]), Rr2 = decode_reg(st.capture_groups["r"] + 1);

		st.mnemonic(st.tokens.size(),"movw",{Rd1,Rd2,Rr1,Rr2},[&](cg &c)
		{
			c.assign(Rd1,Rr1);
			c.assign(Rd2,Rr2);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10110 A@.. d@..... A@...." 	= [](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		constant off(st.capture_groups["A"]);

		st.mnemonic(st.tokens.size(),"in",Rd,off,[&](cg &c)
		{
			c.assign(Rd,sram(off));
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10111 A@.. r@..... A@...." 	= [](sm &st) 	
	{
		constant off = st.capture_groups["A"];
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size(),"out",off,Rr,[&](cg &c)
		{
			c.assign(sram(off),Rr);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "1001000 d@..... 1111"				= unary_reg("pop",[](cg &c, const variable &r) 
	{
		rvalue sp = c.sub_i(c.or_b(c.shiftl_u("sph"_var,8_val),"spl"_var),1_val);
		c.assign(r,sram(sp));
		c.assign(sram("spl"_var),sp);
		c.assign(sram("sph"_var),c.shiftr_u(sp,8_val));
	});
	main | "1001001 d@..... 1111" 			= unary_reg("push",[](cg &c, const variable &r) 
	{ 
		rvalue sp = c.or_b(c.shiftl_u("sph"_var,8_val),"spl"_var);
		c.assign(sram(sp),r);
		sp = c.add_i(sp,1_val);
		c.assign(sram("spl"_var),sp);
		c.assign(sram("sph"_var),c.shiftr_u(sp,8_val));
	});
	main | "1001010 d@..... 0010" 			= unary_reg("swap",[](cg &c, const variable &r)
	{
		c.or_b(r,c.shiftl_u(c.slice(r,4_val,7_val),4_val),c.slice(r,0_val,3_val));
	});
	main | "1001001 r@..... 0100" 			= unary_reg("xch",[](cg &c, const variable &r)
	{
		rvalue z = c.or_b(c.shiftl_u("r30"_var,8_val),"r31"_var);
		rvalue tmp = sram(z);
		c.assign(sram(z),r);
		c.assign(r,tmp);
	});
	main | "11101111 d@.... 1111" 			= unary_reg("ser",[](cg &c, const variable &r)
	{
		c	.assign(r,0xff_val);
	});
	main | "1110 K@.... d@.... K@...."	= binary_regconst("ldi",[&](cg &m, const variable &Rd, const constant &K)
	{
		m.assign(Rd,K);
	});

	main | "1001001 r@..... 0110" 			= unary_reg("lac",[](cg &c, const variable &r)
	{
		rvalue z = c.or_b(c.shiftl_u("r30"_var,8_val),"r31"_var);
		c.assign(sram(z),c.and_b(r,c.sub_i(0xff_val,sram(z))));
	});
	main | "1001001 r@..... 0101" 			= unary_reg("las",[](cg &c, const variable &r)
	{
		rvalue z = c.or_b(c.shiftl_u("r30"_var,8_val),"r31"_var);
		rvalue tmp = sram(z);
		c.assign(sram(z),c.or_b(r,tmp));
		c.assign(r,tmp);
	});
	main | "1001001 r@..... 0111" 			= unary_reg("lat",[](cg &c, const variable &r)
	{
		rvalue z = c.or_b(c.shiftl_u("r30"_var,8_val),"r31"_var);
		rvalue tmp = sram(z);
		c.assign(sram(z),c.xor_b(r,tmp));
		c.assign(r,tmp);
	});
	main | "1001000 d@..... 0000" | "k@................" = [](sm &st)
	{
		constant k = st.capture_groups["k"];
		variable Rd = decode_reg(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size(),"lds",Rd,k,[&](cg &c)
		{
			// TODO
		});
		st.jump(st.address + st.tokens.size());
	};

	main | "10100 k@... d@.... k@...." 	= [](sm &st)
	{
		unsigned int k = st.capture_groups["k"];
		variable Rd = decode_reg(st.capture_groups["d"] + 16);

		k = (~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15);
		st.mnemonic(st.tokens.size(),"lds",Rd,constant(k),[&](cg &c)
		{
			// TODO
		});
		st.jump(st.address + st.tokens.size());
	};

	main | 0x95c8 											= 	[](sm &st)
	{
		std::list<rvalue> nop;
		st.mnemonic(st.tokens.size(),"lpm",nop,[&](cg &c)
		{
			rvalue z = c.or_b(c.shiftl_u("r30"_var,8_val),"r31"_var);
			c.assign("r1"_var,flash(z));
		});
		st.jump(st.address + st.tokens.size());
	};

	main | 0x95e8 											= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"spm");
		st.jump(st.address + st.tokens.size());
	};

	main | 0x95f8 											= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"spm",decode_preg(30,PostInc),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "1001001 d@..... 0000" | "k@................" = [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sts",constant(st.capture_groups["k"]),
																			 decode_reg(st.capture_groups["r"]),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10101 k@... d@.... k@...." 	= [](sm &st)
	{
		unsigned int _k = st.capture_groups["k"];
		constant k = (~_k & 16) | (_k & 16) | (_k & 64) | (_k & 32) | (_k & 15);

		st.mnemonic(st.tokens.size(),"sts",k,decode_reg(st.capture_groups["r"]),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10011010 A@..... b@..." 			= [](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = st.capture_groups["b"];

		st.mnemonic(st.tokens.size(),"sbi",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10011000 A@..... b@..." 			= [](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = st.capture_groups["b"];

		st.mnemonic(st.tokens.size(),"cbi",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	// SREG operations
	//main | "100101001 s@... 1000" = simple("bclr");
	//main | "100101000 s@... 1000" = simple("bset");
	main | 0x9408 = simple("sec",[](cg &m) { m.assign("C"_var,1_val); });
	main | 0x9458 = simple("seh",[](cg &m) { m.assign("H"_var,1_val); });
	main | 0x9478 = simple("sei",[](cg &m) { m.assign("I"_var,1_val); });
	main | 0x9428 = simple("sen",[](cg &m) { m.assign("N"_var,1_val); });
	main | 0x9448 = simple("ses",[](cg &m) { m.assign("S"_var,1_val); });
	main | 0x9468 = simple("set",[](cg &m) { m.assign("T"_var,1_val); });
	main | 0x9438 = simple("sev",[](cg &m) { m.assign("V"_var,1_val); });
	main | 0x9418 = simple("sez",[](cg &m) { m.assign("Z"_var,1_val); });
	main | 0x9488 = simple("clc",[](cg &m) { m.assign("C"_var,0_val); });
	main | 0x94d8 = simple("clh",[](cg &m) { m.assign("H"_var,0_val); });
	main | 0x94f8 = simple("cli",[](cg &m) { m.assign("I"_var,0_val); });
	main | 0x94a8 = simple("cln",[](cg &m) { m.assign("N"_var,0_val); });
	main | 0x94c8 = simple("cls",[](cg &m) { m.assign("S"_var,0_val); });
	main | 0x94e8 = simple("clt",[](cg &m) { m.assign("T"_var,0_val); });
	main | 0x94b8 = simple("clv",[](cg &m) { m.assign("V"_var,0_val); });
	main | 0x9498 = simple("clz",[](cg &m) { m.assign("Z"_var,0_val); });
	main | "000101 r@. d@..... r@...." 	= binary_reg("cp",[](cg &m, const variable &Rd, const variable &Rr)
	{	
		rvalue R = m.sub_i(Rd,Rr);
			
		half_carry(Rd,Rr,R,m);
		two_complement_overflow(Rd,Rr,R,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));	// N: R7
		is_zero(R,m);
		carry(Rd,Rr,R,m);
		m.xor_b("S"_var,"N"_var,"V"_var);					// S: N ⊕ V 
	});
	main | "000001 r@. d@..... r@...." 	= binary_reg("cpc",[](cg &m, const variable &Rd, const variable &Rr)
	{	
		rvalue R = m.sub_i(Rd,m.sub_i(Rr,"C"_var));
		
		half_carry(Rd,Rr,R,m);
		two_complement_overflow(Rd,Rr,R,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));			// N: R7
		m.assign("Z"_var,m.or_b(zero(R,m),"Z"_var));
		carry(Rd,Rr,R,m);
		m.xor_b("S"_var,"N"_var,"V"_var);							// S: N ⊕ V 
	});
	main | "0011 K@.... d@.... K@...." 	= binary_regconst("cpi",[&](cg &m, const variable &Rd, const constant &K)
	{	
		rvalue R = m.sub_i(Rd,K);

		half_carry(Rd,K,R,m);
		two_complement_overflow(Rd,K,R,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));	// N: R7
		is_zero(R,m);
		carry(Rd,K,R,m);
		m.xor_b("S"_var,"N"_var,"V"_var);					// S: N ⊕ V 
	});

	// main | "001000 d@.........." 				= tst (alias for and)
	
	// bit-level logic
	// main | "0110 K@.... d@.... K@...." = sbr (alias for ori)
	// main | "000011 d@.........."				= lsl (alias for add X,X);
	main | "1001010 d@..... 0110"				= unary_reg("lsr");

	// byte-level arithmetic and logic
	main | "000111 r@. d@..... r@...."	= binary_reg("adc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = m.add_i(Rd,m.add_i(Rr,"C"_var));

		half_carry(R,Rd,Rr,m);
		two_complement_overflow(R,Rd,Rr,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));	// N: R7
		is_zero(R,m);
		carry(R,Rd,Rr,m);
		m.xor_b("S"_var,"N"_var,"V"_var);					// S: N ⊕ V 
		m.assign(Rd,R);
	});
	main | "000011 r@. d@..... r@...." 	= binary_reg("add",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = m.add_i(Rd,Rr);

		half_carry(R,Rd,Rr,m);
		two_complement_overflow(R,Rd,Rr,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));	// N: R7
		is_zero(R,m);
		carry(R,Rd,Rr,m);
		m.xor_b("S"_var,"N"_var,"V"_var);					// S: N ⊕ V 

		m.assign(Rd,R);
	});
	main | "001000 r@. d@..... r@...." 	= binary_reg("and",[](cg &m, const variable &Rd, const variable &Rr)
	{
		m.and_b(Rd,Rd,Rr);
	
		m.assign("V"_var,0_val);										// V: 0
		m.assign("N"_var,m.slice(Rd,7_val,7_val));	// N: Rd7
		m.xor_b("S"_var,"N"_var,"V"_var);						// S: N ⊕ V
		is_zero(Rd,m);
	});
	main | "0111 K@.... d@.... K@...." 	= binary_regconst("andi",[&](cg &m, const variable &Rd, const constant &K)
	{
		m.and_b(Rd,Rd,K);
	
		m.assign("V"_var,0_val);										// V: 0
		m.assign("N"_var,m.slice(Rd,7_val,7_val));	// N: Rd7
		m.xor_b("S"_var,"N"_var,"V"_var);						// S: N ⊕ V
		is_zero(Rd,m);
	});

	main | "001001 r@. d@..... r@...." 	= [](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		if(Rd == Rr)
		{
			st.mnemonic(st.tokens.size(),"clr",Rd,[&](cg &m)
			{
				m.assign(Rd,0_val);
				m.assign("V"_var,0_val);
				m.assign("N"_var,0_val);
				m.assign("S"_var,0_val);
				m.assign("Z"_var,0_val);
			});
			st.jump(st.address + st.tokens.size());
		}
		else
		{
			st.mnemonic(st.tokens.size(),"eor",Rd,Rr,[&](cg &m)
			{
				m.xor_b(Rd,Rd,Rr);
				m.assign("V"_var,0_val);										// V: 0
				m.assign("N"_var,m.slice(Rd,7_val,7_val));	// N: Rd7
				m.xor_b("S"_var,"N"_var,"V"_var);						// S: N ⊕ V
				is_zero(Rd,m);
			});
			st.jump(st.address + st.tokens.size());
		}
	};
	main | "1001010 d@..... 0001"				= unary_reg("neg",[](cg &m, const variable &Rd)
	{
	});
		
	main | "001010 r@. d@..... r@...." 	= binary_reg("or",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main | "0110 K@.... d@.... K@...." 	= binary_regconst("ori",[&](cg &m, const variable &Rd, const constant &K)
	{
		//m.or_b(Rd,Rd,K);
	});

	main | "000110 r@. d@..... r@...." 	= binary_reg("sub",[&](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = m.sub_i(Rd,Rr);
			
		half_carry(Rd,Rr,R,m);
		two_complement_overflow(Rd,Rr,R,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));	// N: R7
		is_zero(R,m);
		carry(Rd,Rr,R,m);
		m.xor_b("S"_var,"N"_var,"V"_var);					// S: N ⊕ V 
		m.assign(Rd,R);
	});
	main | "0101 K@.... d@.... K@...." 	= binary_regconst("subi",[&](cg &m, const variable &Rd, const constant &K)
	{ 
		rvalue R = m.sub_i(Rd,K);
			
		half_carry(Rd,K,R,m);
		two_complement_overflow(Rd,K,R,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));	// N: R7
		is_zero(R,m);
		carry(Rd,K,R,m);
		m.xor_b("S"_var,"N"_var,"V"_var);					// S: N ⊕ V 
		m.assign(Rd,R);
	});

	main | "1001010 d@..... 0101" 			= unary_reg("asr");
	main | "000111 d@.........." 				= unary_reg("rol");
	main | "1001010 d@..... 0111" 			= unary_reg("ror");
	main | "1001010 d@..... 1010" 			= unary_reg("dec");
	main | "1001010 d@..... 0011" 			= unary_reg("inc");
	main | "000010 r@. d@..... r@...." 	= binary_reg("sbc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = m.sub_i(Rd,m.sub_i(Rr,"C"_var));
		
		half_carry(Rd,Rr,R,m);
		two_complement_overflow(Rd,Rr,R,m);
		m.assign("N"_var,m.slice(R,7_val,7_val));			// N: R7
		m.assign("Z"_var,m.or_b(zero(R,m),"Z"_var));
		carry(Rd,Rr,R,m);
		m.xor_b("S"_var,"N"_var,"V"_var);							// S: N ⊕ V 

		m.assign(Rd,R);
	});

	main | "0100 K@.... d@.... K@...." 	= binary_regconst("sbci",[&](cg &m, const variable &Rd, const constant &K)
	{
		// TODO
	});

	main | "1001010 d@..... 0000" 			= unary_reg("com");

	// word-level arithmetic and logic
	main | "10010110 K@.. d@.. K@...." = [](sm &st)
	{
		constant K = (unsigned int)st.capture_groups["K"];
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		variable Rd1 = decode_reg(d);
		variable Rd2 = decode_reg(d+1);
		
		st.mnemonic(st.tokens.size(),"adiw",{Rd2,Rd1,K},[&](cg &c)
		{
			rvalue R = c.add_i(c.or_b(c.shiftl_u(Rd2,8_val),Rd1),K);

			// V: !Rdh7•R15
			c.and_b("V"_var,c.not_b(c.slice(Rd1,7_val,7_val)),c.slice(R,15_val,15_val));
			
			// N: R15
			c.assign("N"_var,c.slice(R,15_val,15_val));

			// Z: !R15•!R14•!R13•!R12•!R11•!R10•!R9•!R8•!R7•R6•!R5•!R4•!R3•!R2•!R1•!R0
			c.and_b("Z"_var,zero(R,c),zero(c.shiftr_u(R,8_val),c));
			
			// C: !R15•Rdh7
			c.and_b("V"_var,c.slice(Rd1,7_val,7_val),c.not_b(c.slice(R,15_val,15_val)));

			// S: N ⊕ V
			c.xor_b("S"_var,"N"_var,"V"_var);

			c.assign(Rd2,c.slice(R,8_val,15_val));
			c.assign(Rd1,c.slice(R,0_val,7_val));
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10010111 K@.. d@.. K@...." = [](sm &st) 
	{
	//	unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		// value_ptr(new reg(d,d+1)),st.capture_groups["K"]
		// TODO
		st.mnemonic(st.tokens.size(),"sbiw");
		st.jump(st.address + st.tokens.size());
	};
	main | "0000 0011 0 d@... 1 r@..."	= binary_reg("fmul",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main | "000000111 d@... 0 r@..."		= binary_reg("fmuls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main | "000000111 d@... 1 r@..." 		= binary_reg("fmulsu",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main | "100111 r@. d@..... r@...." 	= binary_reg("mul",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main | "00000010 d@.... r@...." 		= binary_reg("muls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main | "000000110 d@... 0 r@..." 		= binary_reg("muls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	
	// branches
	// main | "111101 k@....... s@..." = simple("brbc");
	// main | "111100 k@....... s@..." = [](sm &st)  { st.mnemonic(st.tokens.size(),"brbs"; });
	main | "111101 k@....... 000" 			= branch("brcc","C"_var,false);
	main | "111100 k@....... 000" 			= branch("brcs","C"_var,true);
	main | "111100 k@....... 001" 			= branch("breq","Z"_var,true);
	main | "111101 k@....... 100" 			= branch("brge","S"_var,false);
	main | "111101 k@....... 101" 			= branch("brhc","H"_var,false);
	main | "111100 k@....... 101" 			= branch("brhs","H"_var,true);
	main | "111101 k@....... 111" 			= branch("brid","I"_var,false);
	main | "111100 k@....... 111" 			= branch("brie","I"_var,true);
	main | "111100 k@....... 000" 			= branch("brlo","C"_var,true);
	main | "111100 k@....... 100" 			= branch("brlt","S"_var,true);
	main | "111100 k@....... 010" 			= branch("brmi","N"_var,true);
	main | "111101 k@....... 001"		 		= branch("brne","Z"_var,false);
	main | "111101 k@....... 010" 			= branch("brpl","N"_var,false);
	main | "111101 k@....... 000" 			= branch("brsh","C"_var,false);
	main | "111101 k@....... 110" 			= branch("brtc","T"_var,false);
	main | "111100 k@....... 110" 			= branch("brts","T"_var,true);
	main | "111101 k@....... 011" 			= branch("brvc","V"_var,false);
	main | "111100 k@....... 011" 			= branch("brvs","V"_var,true);
	main | "1111 110r@..... 0 b@..." 		= [](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		constant b = st.capture_groups["b"];

		st.mnemonic(st.tokens.size(),"sbrc",Rr,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1111 111 r@..... 0 b@..." 		= [](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		constant b = st.capture_groups["b"];

		st.mnemonic(st.tokens.size(),"sbrs",Rr,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "000100 r@. d@..... r@...."	= [](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		variable Rd = decode_reg(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size(),"cpse",Rd,Rr,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1001 A@..... b@..." 		= [](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = st.capture_groups["b"];

		st.mnemonic(st.tokens.size(),"sbic",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1011 A@..... b@..." 		= [](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = st.capture_groups["b"];

		st.mnemonic(st.tokens.size(),"sbis",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};

	// jump branches
	main | "1001010 k@..... 111 k@." | "k@................"	= [](sm &st) 
	{
		constant k = st.capture_groups["k"];
		
		st.mnemonic(st.tokens.size(),"call",k,[&](cg &c)
		{
			c.call(k);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "1001010 k@..... 110 k@." | "k@................"	= [](sm &st) 
	{ 
		constant k = st.capture_groups["k"];
		
		st.mnemonic(st.tokens.size(),"jmp",k,std::function<void(cg &c)>());
		st.jump(k);
	};

	main | "1101 k@............" 														= [](sm &st) 
	{
		int _k = st.capture_groups["k"];
		constant k = (_k <= 2047 ? _k : _k - 4096) + 1 + st.address ;
		
		st.mnemonic(st.tokens.size(),"rcall",k,[&](cg &c)
		{
			c.call(k);
		});
		st.jump(st.address + 1);
	};
	main | "1100 k@............" 														= [](sm &st) 
	{
		int _k = st.capture_groups["k"];
		constant k = (_k <= 2047 ? _k : _k - 4096) + 1 + st.address;

		st.mnemonic(st.tokens.size(),"rjmp",k,std::function<void(cg &c)>());
		st.jump(k);
	};
	main | 0x9508 = [](sm &st) { st.mnemonic(st.tokens.size(),"ret"); };
	main | 0x9518 = [](sm &st) { st.mnemonic(st.tokens.size(),"reti"); };
	main | 0x9409 = [](sm &st) 
	{ 
		variable J("J"_var);
		std::list<rvalue> nop;

		st.mnemonic(st.tokens.size(),"ijmp",nop,[&](cg &c)
		{
			c.or_b(J,c.shiftl_u("r31"_var,8_val),"r30"_var);
		});
		st.jump(J);
	};

	main | 0x9509 = [](sm &st) { st.mnemonic(st.tokens.size(),"icall"); };
	// icall
	
	// store and load with x,y,z
	main | "1001 001r@. r@.... 1100" = binary_st("r26"_var,"r27"_var,false,false);
	main | "1001 001r@. r@.... 1101" = binary_st("r26"_var,"r27"_var,false,true);
	main | "1001 001r@. r@.... 1110" = binary_st("r26"_var,"r27"_var,true,false);
	main | "1000 001r@. r@.... 1000" = binary_st("r28"_var,"r29"_var,false,false);
	main | "1001 001r@. r@.... 1001" = binary_st("r28"_var,"r29"_var,false,true);
	main | "1001 001r@. r@.... 1010" = binary_st("r28"_var,"r29"_var,true,false);
	main | "10q@.0 q@..1r@. r@.... 1q@..." = binary_stq("r28"_var);
	main | "1000 001r@. r@.... 0000" = binary_st("r30"_var,"r31"_var,false,false);
	main | "1001 001r@. r@.... 0001" = binary_st("r30"_var,"r31"_var,false,true);
	main | "1001 001r@. r@.... 0010" = binary_st("r30"_var,"r31"_var,true,false);
	main | "10q@.0 q@..1r@. r@.... 0q@..." = binary_stq("r30"_var);
	
	main | "1001 000d@. d@.... 1100" = binary_ld("r26"_var,"r27"_var,false,false);
	main | "1001 000d@. d@.... 1101" = binary_ld("r26"_var,"r27"_var,false,true);
	main | "1001 000d@. d@.... 1110" = binary_ld("r26"_var,"r27"_var,true,false);
	main | "1000 000d@. d@.... 1000" = binary_ld("r28"_var,"r29"_var,false,false);
	main | "1001 000d@. d@.... 1001" = binary_ld("r28"_var,"r29"_var,false,true);
	main | "1001 000d@. d@.... 1010" = binary_ld("r28"_var,"r29"_var,true,false);
	main | "10 q@. 0 q@.. 0 d@..... 1 q@..." = binary_ldq("r28"_var);
	main | "1000 000d@. d@.... 0000" = binary_ld("r30"_var,"r31"_var,false,false);
	main | "1001 000 d@..... 0001" = binary_ld("r30"_var,"r31"_var,false,true);
	main | "1001 000d@. d@.... 0010" = binary_ld("r30"_var,"r31"_var,true,false);
	main | "10q@.0 q@..0d@. d@.... 0q@..." = binary_ldq("r30"_var);

	// misc
	main | 0x9598 = simple("break",[](cg &m) { /* TODO */ });
	main | "10010100 K@.... 1011" = [](sm &st) 
	{
		st.mnemonic(st.tokens.size(),"des",constant(st.capture_groups["K"]),std::function<void(cg &c)>());
		st.jump(st.tokens.size() + st.address);
	};

	main | (architecture_traits<avr_tag>::token_type)0x0 = simple("nop",[](cg &m) { /* TODO */ });
	main | 0x9588 = simple("sleep",[](cg &m) { /* TODO */ });
	main | 0x95a8 = simple("wdr",[](cg &m) { /* TODO */ });

	// catch all
	main = [](sm &st)
	{
		st.mnemonic(1,"unk");
	};

	return ::disassemble<avr_tag>(main,bytes);
}
