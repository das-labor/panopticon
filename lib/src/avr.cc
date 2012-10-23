#include <iostream>
#include <iomanip>
#include <numeric>
#include <functional>
#include <algorithm>

#include "decoder.hh"
#include "avr.hh"

typedef sem_state<avr_tag> sm;
typedef std::function<void(sm &)> sem_action;
typedef code_generator<avr_tag> cg;

// architecture_traits
unsigned int next_unused = 0;

template<> 
var_ptr temporary(avr_tag)
{
	return var_ptr(new variable(name("tmp" + to_string(next_unused++)),16));
}

// register and memory declarations
const variable_decl 
r1("r1",8), 	r2("r2",8), 	r3("r3",8), 	r4("r4",8), 	r5("r5",8), 	r6("r6",8), 	r7("r7",8), 	
r8("r8",8), 	r9("r9",8), 	r10("r10",8), r11("r11",8), r12("r12",8), r13("r13",8),	r14("r14",8), 
r15("r15",8), r16("r16",8), r17("r17",8), r18("r18",8), r19("r19",8), r20("r20",8), r21("r21",8), 
r22("r22",8), r23("r23",8), r24("r24",8), r25("r25",8), r26("r26",8),	r27("r27",8),	r28("r28",8),	
r29("r29",8),	r30("r30",8),	r31("r31",8),	r0("r0",8),
C("C",1), Z("Z",1), N("N",1), V("V",1), S("S",1), H("H",1), T("T",1), I("I",1);

#define IO_OFFSET 0x20
#define SPL IO_OFFSET + 0
#define SPH IO_OFFSET + 1

const memory_decl flash("flash",memory::Big,1), sram("sram",memory::Big,1);

// helper
const variable_decl &decl(unsigned int r)
{
	switch(r)
	{
		case 0: return r0;
		case 1: return r1;
		case 2: return r2;
		case 3: return r3;
		case 4: return r4;
		case 5: return r5;
		case 6: return r6;
		case 7: return r7;
		case 8: return r8;
		case 9: return r9;
		case 10: return r10;
		case 11: return r11;
		case 12: return r12;
		case 13: return r13;
		case 14: return r14;
		case 15: return r15;
		case 16: return r16;
		case 17: return r17;
		case 18: return r18;
		case 19: return r19;
		case 20: return r20;
		case 21: return r21;
		case 22: return r22;
		case 23: return r23;
		case 24: return r24;
		case 25: return r25;
		case 26: return r26;
		case 27: return r27;
		case 28: return r28;
		case 29: return r29;
		case 30: return r30;
		case 31: return r31;
		default: assert(false);
	}
}

sem_action unary_reg(string x, std::function<void(cg &c, variable_decl &v)> func = std::function<void(cg &c, variable_decl &R)>())
{
	return [x,func](sm &st)
	{
		cout << x << endl;
		const variable_decl &op = st.capture_groups.count("d") ? decl((unsigned int)st.capture_groups["d"]) : 
																														 decl((unsigned int)st.capture_groups["r"]);
		if(func)
			st.mnemonic(st.tokens.size(),x,op,std::bind(func,std::placeholders::_1,op));
		else
			st.mnemonic(st.tokens.size(),x,op);
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_reg(string x, std::function<void(cg &c, const variable_decl &Rd, const variable_decl &Rr)> func)
{
	return [x,func](sm &st)
	{
		const variable_decl &Rd = decl(st.capture_groups["d"]);
		const variable_decl &Rr = decl(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size(),x,Rd,Rr,bind(func,placeholders::_1,Rd,Rr));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action branch(string m, valproxy flag, bool set)
{
	return [m,flag,set](sm &st)
	{
		int k = st.capture_groups["k"];
		guard_ptr g(new guard(flag.value,relation::Eq,set ? 1 : 0));
		
		k = k <= 63 ? k : k - 128;
		st.mnemonic(st.tokens.size(),m,k);
		st.jump(st.address + 1,g->negation());
		st.jump(st.address + k + 1,g);
	};
}

sem_action binary_regconst(string x, std::function<void(cg &c, const variable_decl &Rd, unsigned int k)> func)
{
	return [x,func](sm &st)
	{
		const variable_decl &Rd = decl(st.capture_groups["d"] + 16);
		unsigned int K = st.capture_groups["K"];

		st.mnemonic(st.tokens.size(),x,{Rd.instantiate(),value_ptr(new constant(K))},bind(func,placeholders::_1,Rd,K));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_st(const variable_decl &Rd1, const variable_decl &Rd2, bool pre_dec = false, bool post_inc = false)
{
	return [&](sm &st)
	{
		const variable_decl &Rr = decl(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size(),"st",{Rd2.instantiate(),Rd1.instantiate(),Rr.instantiate()},[&](cg &c)
		{
			variable_decl X("ptr",16);
			c.concat(X,Rd2,Rd1);

			if(pre_dec) c.sub_i(X,X,1);
			c.assign(sram(X),Rr);
			if(post_inc) c.add_i(X,X,1);
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_ld(const variable_decl &Rr1, const variable_decl &Rr2, bool pre_dec = false, bool post_inc = false)
{
	return [&](sm &st)
	{
		const variable_decl &Rd = decl(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size(),"ld",{Rd.instantiate(),Rr2.instantiate(),Rr1.instantiate()},[&](cg &c)
		{
			variable_decl X("ptr",16);
			c.concat(X,Rr2,Rr1);

			if(pre_dec) c.sub_i(X,X,1);
			c.assign(Rd,sram(X));
			if(post_inc) c.add_i(X,X,1);
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_stq(reg::IndirectReg r)
{
	return [r](sm &st)
	{
		st.mnemonic(st.tokens.size(),"st",value_ptr(new reg(r,reg::PostDisplace,st.capture_groups["q"])),value_ptr(new reg(st.capture_groups["r"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_ldq(reg::IndirectReg r)
{
	return [r](sm &st)
	{
		st.mnemonic(st.tokens.size(),"ld",value_ptr(new reg(st.capture_groups["r"])),value_ptr(new reg(r,reg::PostDisplace,st.capture_groups["q"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
}

sem_action simple(string x, std::function<void(cg &c)> fn)
{
	return [x,fn](sm &st)
	{
		st.mnemonic(st.tokens.size(),x,{},fn);
		st.jump(st.address + st.tokens.size());
	};
}

flow_ptr avr_decode(vector<typename architecture_traits<avr_tag>::token_type> &bytes, addr_t entry)
{
	decoder<avr_tag> main;

	// memory operations
	main | "001011 r@. d@..... r@...." 	= binary_reg("mov",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		m.assign(Rd,Rr);
	});

	main | "00000001 d@.... r@...." 		= [](sm &st)
	{ 
		const variable_decl &Rd1 = decl(st.capture_groups["d"]), Rd2 = decl(st.capture_groups["d"] + 1);
		const variable_decl &Rr1 = decl(st.capture_groups["r"]), Rr2 = decl(st.capture_groups["r"] + 1);

		st.mnemonic(st.tokens.size(),"movw",{Rd1.instantiate(),Rd2.instantiate(),Rr1.instantiate(),Rr2.instantiate()},[&](cg &c)
		{
			c.assign(Rd1,Rr1);
			c.assign(Rd2,Rr2);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10110 A@.. d@..... A@...." 	= [](sm &st)
	{
		const variable_decl &Rd = decl(st.capture_groups["d"]);
		unsigned int off = st.capture_groups["A"] + IO_OFFSET;

		st.mnemonic(st.tokens.size(),"in",Rd,off,[&](cg &c)
		{
			c.assign(Rd,sram(off));
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10111 A@.. r@..... A@...." 	= [](sm &st) 	
	{
		unsigned int off = st.capture_groups["A"] + IO_OFFSET;
		const variable_decl &Rr = decl(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size(),"out",off,Rr,[&](cg &c)
		{
			c.assign(sram(off),Rr);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "1001000 d@..... 1111"				= unary_reg("pop",[](cg &c, const variable_decl &r) 
	{
		value_ptr sp = c.sub_i(c.concat(SPH,SPL),1);
		c.assign(r,sram(sp));
		c.assign(sram(SPL),sp);
		c.assign(sram(SPH),c.shiftr_u(sp,8));
	});
	main | "1001001 d@..... 1111" 			= unary_reg("push",[](cg &c, const variable_decl &r) 
	{ 
		value_ptr sp = c.concat(SPH,SPL);
		c.assign(sram(sp),r);
		sp = c.add_i(sp,1);
		c.assign(sram(SPL),sp);
		c.assign(sram(SPH),c.shiftr_u(sp,8));
	});
	main | "1001010 d@..... 0010" 			= unary_reg("swap",[](cg &c, const variable_decl &r)
	{
		c.concat(r,r(4,7),r(0,3));
	});
	main | "1001001 r@..... 0100" 			= unary_reg("xch",[](cg &c, const variable_decl &r)
	{
		value_ptr z = c.concat(r30,r31);
		value_ptr tmp = sram(z);
		c.assign(sram(z),r);
		c.assign(r,tmp);
	});
	main | "11101111 d@.... 1111" 			= unary_reg("ser",[](cg &c, const variable_decl &r)
	{
		c	.assign(r,0xff);
	});
	main | "1110 K@.... d@.... K@...."	= binary_regconst("ldi",[&](cg &m, const variable_decl &Rd, unsigned int K)
	{
		m.assign(Rd,K);
	});

	main | "1001001 r@..... 0110" 			= unary_reg("lac",[](cg &c, const variable_decl &r)
	{
		value_ptr z = c.concat(r30,r31);
		c.assign(sram(z),c.and_b(r,c.sub_i(0xff,sram(z))));
	});
	main | "1001001 r@..... 0101" 			= unary_reg("las",[](cg &c, const variable_decl &r)
	{
		value_ptr z = c.concat(r30,r31);
		value_ptr tmp = sram(z);
		c.assign(sram(z),c.or_b(r,tmp));
		c.assign(r,tmp);
	});
	main | "1001001 r@..... 0111" 			= unary_reg("lat",[](cg &c, const variable_decl &r)
	{
		value_ptr z = c.concat(r30,r31);
		value_ptr tmp = sram(z);
		c.assign(sram(z),c.xor_b(r,tmp));
		c.assign(r,tmp);
	});
	main | "1001000 d@..... 0000" | "k@................" = [](sm &st)
	{
		unsigned int k = st.capture_groups["k"];
		value_ptr Rd(new reg(st.capture_groups["d"]));

		st.mnemonic(st.tokens.size(),"lds",Rd,k,[&](cg &c)
		{
			// TODO
		});
		st.jump(st.address + st.tokens.size());
	};

	main | "10100 k@... d@.... k@...." 	= [](sm &st)
	{
		unsigned int k = st.capture_groups["k"];
		value_ptr Rd(new reg(st.capture_groups["d"] + 16));

		k = (~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15);
		st.mnemonic(st.tokens.size(),"lds",Rd,k,[&](cg &c)
		{
			// TODO
		});
		st.jump(st.address + st.tokens.size());
	};

	main | 0x95c8 											= 	[](sm &st)
	{
		st.mnemonic(st.tokens.size(),"lpm",{},[&](cg &c)
		{
			value_ptr z = c.concat(r30,r31);
			c.assign(r1,flash(z));
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
		st.mnemonic(st.tokens.size(),"spm",value_ptr(new reg(reg::Z,reg::PostInc)),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "1001001 d@..... 0000" | "k@................" = [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sts",st.capture_groups["k"],value_ptr(new reg(st.capture_groups["r"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10101 k@... d@.... k@...." 	= [](sm &st)
	{
		unsigned int k = st.capture_groups["k"];

		st.mnemonic(st.tokens.size(),"sts",(~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15),value_ptr(new reg(st.capture_groups["r"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10011010 A@..... b@..." 			= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sbi",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10011000 A@..... b@..." 			= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"cbi",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	// SREG operations
	//main | "100101001 s@... 1000" = simple("bclr");
	//main | "100101000 s@... 1000" = simple("bset");
	main | 0x9408 = simple("sec",[](cg &m) { m.assign(C,1); });
	main | 0x9458 = simple("seh",[](cg &m) { m.assign(H,1); });
	main | 0x9478 = simple("sei",[](cg &m) { m.assign(I,1); });
	main | 0x9428 = simple("sen",[](cg &m) { m.assign(N,1); });
	main | 0x9448 = simple("ses",[](cg &m) { m.assign(S,1); });
	main | 0x9468 = simple("set",[](cg &m) { m.assign(T,1); });
	main | 0x9438 = simple("sev",[](cg &m) { m.assign(V,1); });
	main | 0x9418 = simple("sez",[](cg &m) { m.assign(Z,1); });
	main | 0x9488 = simple("clc",[](cg &m) { m.assign(C,0); });
	main | 0x94d8 = simple("clh",[](cg &m) { m.assign(H,0); });
	main | 0x94f8 = simple("cli",[](cg &m) { m.assign(I,0); });
	main | 0x94a8 = simple("cln",[](cg &m) { m.assign(N,0); });
	main | 0x94c8 = simple("cls",[](cg &m) { m.assign(S,0); });
	main | 0x94e8 = simple("clt",[](cg &m) { m.assign(T,0); });
	main | 0x94b8 = simple("clv",[](cg &m) { m.assign(V,0); });
	main | 0x9498 = simple("clz",[](cg &m) { m.assign(Z,0); });
	main | "000101 r@. d@..... r@...." 	= binary_reg("cp",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{	
		variable_decl R("R",8);

		m.sub_i(R,Rd,Rr);
			
		// H: !Rd3•Rr3 + Rr3•R3 + r3•!Rd3
		m.or_b(H,m.or_b(
			m.and_b(
				m.not_b(Rd(3,3)),
				Rr(3,3)),
			m.and_b(
				R(3,3),
				Rr(3,3))),
			m.and_b(
				m.not_b(Rd(3,3)),
				R(3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m.or_b(V,
			m.and_b(m.and_b(
				Rd(7,7),
				m.not_b(Rr(7,7))),
				m.not_b(R(7,7))),
			m.and_b(m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
				R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b(Z,m.not_b(R(0,0)),
			m.and_b(m.not_b(R(1,1)),
				m.and_b(m.not_b(R(2,2)),
					m.and_b(m.not_b(R(3,3)),
						m.and_b(m.not_b(R(4,4)),
							m.and_b(m.not_b(R(5,5)),
								m.and_b(m.not_b(R(6,6)),m.not_b(R(7,7)))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m.or_b(C,m.or_b(
			m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
			m.and_b(
				R(7,7),
				Rr(7,7))),
			m.and_b(
				m.not_b(Rd(7,7)),
				R(7,7)));

		// S: N ⊕ V 
		m.xor_b(S,N,V);
	});
	main | "000001 r@. d@..... r@...." 	= binary_reg("cpc",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{	
		variable_decl R("R",8);

		m.sub_i(R,Rd,m.sub_i(Rr,C));
			
		// H: !Rd3•Rr3 + Rr3•R3 + r3•!Rd3
		m.or_b(H,m.or_b(
			m.and_b(
				m.not_b(Rd(3,3)),
				Rr(3,3)),
			m.and_b(
				R(3,3),
				Rr(3,3))),
			m.and_b(
				m.not_b(Rd(3,3)),
				R(3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m.or_b(V,
			m.and_b(m.and_b(
				Rd(7,7),
				m.not_b(Rr(7,7))),
				m.not_b(R(7,7))),
			m.and_b(m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
				R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0•Z
		m.and_b(Z,m.not_b(R(0,0)),
			m.and_b(m.not_b(R(1,1)),
				m.and_b(m.not_b(R(2,2)),
					m.and_b(m.not_b(R(3,3)),
						m.and_b(m.not_b(R(4,4)),
							m.and_b(m.not_b(R(5,5)),
								m.and_b(m.not_b(R(6,6)),
									m.and_b(m.not_b(R(7,7)),Z))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m.or_b(C,m.or_b(
			m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
			m.and_b(
				R(7,7),
				Rr(7,7))),
			m.and_b(
				m.not_b(Rd(7,7)),
				R(7,7)));

		// S: N ⊕ V 
		m.xor_b(S,N,V);
	});
	main | "0011 K@.... d@.... K@...." 	= binary_regconst("cpi",[&](cg &m, const variable_decl &Rd, unsigned int K)
	{	
		const variable_decl R("R",8);

		m.sub_i(R,Rd,K);
			
		// H: !Rd3•K3 + K3•R3 + R3•!Rd3
		m.or_b(H,m.or_b(
			m.and_b(
				m.not_b(Rd(3,3)),
				(K >> 3) & 1),
			m.and_b(
				R(3,3),
				(K >> 3) & 1)),
			m.and_b(
				m.not_b(Rd(3,3)),
				R(3,3)));
		
		// V: Rd7•!K7•!R7 + !Rd7•K7•R7
		m.or_b(V,
			m.and_b(m.and_b(
				Rd(7,7),
				m.not_b((K >> 7) & 1)),
				m.not_b(R(7,7))),
			m.and_b(m.and_b(
				m.not_b(Rd(7,7)),
				(K >> 7) & 1),
				R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b(Z,m.not_b(R(0,0)),
			m.and_b(m.not_b(R(1,1)),
				m.and_b(m.not_b(R(2,2)),
					m.and_b(m.not_b(R(3,3)),
						m.and_b(m.not_b(R(4,4)),
							m.and_b(m.not_b(R(5,5)),
								m.and_b(m.not_b(R(6,6)),m.not_b(R(7,7)))))))));

		// C: !Rd7•K7 + K7•R7 + R7•!Rd7
		m.or_b(C,m.or_b(
			m.and_b(
				m.not_b(Rd(7,7)),
				(K >> 7) & 1),
			m.and_b(
				R(7,7),
				(K >> 7) & 1)),
			m.and_b(
				m.not_b(Rd(7,7)),
				R(7,7)));

		// S: N ⊕ V 
		m.xor_b(S,N,V);
	});

	// main | "001000 d@.........." 				= tst (alias for and)
	
	// bit-level logic
	// main | "0110 K@.... d@.... K@...." = sbr (alias for ori)
	// main | "000011 d@.........."				= lsl (alias for add X,X);
	main | "1001010 d@..... 0110"				= unary_reg("lsr");

	// byte-level arithmetic and logic
	main | "000111 r@. d@..... r@...."	= binary_reg("adc",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		variable_decl R("R",8);

		m.add_i(R,Rd,m.add_i(Rr,C));
		
		// H: Rd3•Rr3 + Rr3•!R3 + !R3•Rd3
		m.or_b(H,m.or_b(
			m.and_b(Rd(3,3),Rr(3,3)),
			m.and_b(Rr(3,3),m.not_b(R(3,3)))),
			m.and_b(m.not_b(R(3,3)),Rd(3,3)));
    
		// V: Rd7•Rr7•!R7 + !Rd7•!Rr7•R7
		m.or_b(V,
			m.and_b(m.and_b(Rd(7,7),Rr(7,7)),m.not_b(R(7,7))),
			m.and_b(m.and_b(m.not_b(Rd(7,7)),m.not_b(Rr(7,7))),R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b(Z,m.not_b(R(0,0)),
				m.and_b(m.not_b(R(1,1)),
					m.and_b(m.not_b(R(2,2)),
						m.and_b(m.not_b(R(3,3)),
							m.and_b(m.not_b(R(4,4)),
								m.and_b(m.not_b(R(5,5)),
									m.and_b(m.not_b(R(6,6)),m.not_b(R(7,7)))))))));

		// C: Rd7•Rr7 + Rr7•R7 + R7•Rd7
		m.or_b(C,m.or_b(
			m.and_b(Rd(7,7),Rr(7,7)),
			m.and_b(Rr(7,7),m.not_b(R(7,7)))),
			m.and_b(m.not_b(R(7,7)),Rd(7,7)));

		// S: N ⊕ V
		m.xor_b(S,N,V);

		m.assign(Rd,R);
	});
	main | "000011 r@. d@..... r@...." 	= binary_reg("add",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		variable_decl R("R",8);

		m.add_i(R,Rd,Rr);
		
		// H: Rd3•Rr3 + Rr3•!R3 + !R3•Rd3
		m.or_b(H,m.or_b(
			m.and_b(Rd(3,3),Rr(3,3)),
			m.and_b(Rr(3,3),m.not_b(R(3,3)))),
			m.and_b(m.not_b(R(3,3)),Rd(3,3)));
    
		// V: Rd7•Rr7•!R7 + !Rd7•!Rr7•R7
		m.or_b(V,
			m.and_b(m.and_b(Rd(7,7),Rr(7,7)),m.not_b(R(7,7))),
			m.and_b(m.and_b(m.not_b(Rd(7,7)),m.not_b(Rr(7,7))),R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b(Z,m.not_b(R(0,0)),
				m.and_b(m.not_b(R(1,1)),
					m.and_b(m.not_b(R(2,2)),
						m.and_b(m.not_b(R(3,3)),
							m.and_b(m.not_b(R(4,4)),
								m.and_b(m.not_b(R(5,5)),
									m.and_b(m.not_b(R(6,6)),m.not_b(R(7,7)))))))));

		// C: Rd7•Rr7 + Rr7•R7 + R7•Rd7
		m.or_b(C,m.or_b(
			m.and_b(Rd(7,7),Rr(7,7)),
			m.and_b(Rr(7,7),m.not_b(R(7,7)))),
			m.and_b(m.not_b(R(7,7)),Rd(7,7)));

		// S: N ⊕ V
		m.xor_b(S,N,V);

		m.assign(Rd,R);
	});
	main | "001000 r@. d@..... r@...." 	= binary_reg("and",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		m.and_b(Rd,Rd,Rr);
	
		// V: 0
		m.assign(V,0);
		
		// N: Rd7
		m.assign(N,Rd(7,7));

		// S: N ⊕ V
		m.xor_b(S,N,V);

		// Z: !Rd7•!Rd6•!Rd5•!Rd4•!Rd3•!Rd2•!Rd1•!Rd0
		m.and_b(Z,m.not_b(Rd(0,0)),
				m.and_b(m.not_b(Rd(1,1)),
					m.and_b(m.not_b(Rd(2,2)),
						m.and_b(m.not_b(Rd(3,3)),
							m.and_b(m.not_b(Rd(4,4)),
								m.and_b(m.not_b(Rd(5,5)),
									m.and_b(m.not_b(Rd(6,6)),m.not_b(Rd(7,7)))))))));
	});
	main | "0111 K@.... d@.... K@...." 	= binary_regconst("andi",[&](cg &m, const variable_decl &Rd, unsigned int K)
	{
		m.and_b(Rd,Rd,K);
	
		// V: 0
		m.assign(V,0);
		
		// N: Rd7
		m.assign(N,Rd(7,7));

		// S: N ⊕ V
		m.xor_b(S,N,V);

		// Z: !Rd7•!Rd6•!Rd5•!Rd4•!Rd3•!Rd2•!Rd1•!Rd0
		m.and_b(Z,m.not_b(Rd(0,0)),
				m.and_b(m.not_b(Rd(1,1)),
					m.and_b(m.not_b(Rd(2,2)),
						m.and_b(m.not_b(Rd(3,3)),
							m.and_b(m.not_b(Rd(4,4)),
								m.and_b(m.not_b(Rd(5,5)),
									m.and_b(m.not_b(Rd(6,6)),m.not_b(Rd(7,7)))))))));
	});

	main | "001001 r@. d@..... r@...." 	= [](sm &st)
	{
		const variable_decl &Rd = decl(st.capture_groups["d"]);
		const variable_decl &Rr = decl(st.capture_groups["r"]);

		if(&Rd == &Rr)
		{
			st.mnemonic(st.tokens.size(),"clr",Rd,[&](cg &m)
			{
				m.assign(Rd,0);
				
				// V: 0
				m.assign(V,0);
		
				// N: Rd7
				m.assign(N,0);

				// S: N ⊕ V
				m.assign(S,0);

				// Z: !Rd7•!Rd6•!Rd5•!Rd4•!Rd3•!Rd2•!Rd1•!Rd0
				m.assign(Z,0);
			});
			st.jump(st.address + st.tokens.size());
		}
		else
		{
			st.mnemonic(st.tokens.size(),"eor",Rd,Rr,[&](cg &m)
			{
				m.xor_b(Rd,Rd,Rr);

				// V: 0
				m.assign(V,0);
		
				// N: Rd7
				m.assign(N,Rd(7,7));

				// S: N ⊕ V
				m.xor_b(S,N,V);

				// Z: !Rd7•!Rd6•!Rd5•!Rd4•!Rd3•!Rd2•!Rd1•!Rd0
				m.and_b(Z,m.not_b(Rd(0,0)),
						m.and_b(m.not_b(Rd(1,1)),
							m.and_b(m.not_b(Rd(2,2)),
								m.and_b(m.not_b(Rd(3,3)),
									m.and_b(m.not_b(Rd(4,4)),
										m.and_b(m.not_b(Rd(5,5)),
											m.and_b(m.not_b(Rd(6,6)),m.not_b(Rd(7,7)))))))));
			});
			st.jump(st.address + st.tokens.size());
		}
	};
	main | "1001010 d@..... 0001"				= unary_reg("neg",[](cg &m, const variable_decl &Rd)
	{
	});
		
	main | "001010 r@. d@..... r@...." 	= binary_reg("or",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	main | "0110 K@.... d@.... K@...." 	= binary_regconst("ori",[&](cg &m, const variable_decl &Rd, unsigned int K)
	{
		//m.or_b(Rd,Rd,K);
	});

	main | "000110 r@. d@..... r@...." 	= binary_reg("sub",[&](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		variable_decl R("R",8);

		m.sub_i(R,Rd,Rr);
			
		// H: !Rd3•Rr3 + Rr3•R3 + r3•!Rd3
		m.or_b(H,m.or_b(
			m.and_b(
				m.not_b(Rd(3,3)),
				Rr(3,3)),
			m.and_b(
				R(3,3),
				Rr(3,3))),
			m.and_b(
				m.not_b(Rd(3,3)),
				R(3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m.or_b(V,
			m.and_b(m.and_b(
				Rd(7,7),
				m.not_b(Rr(7,7))),
				m.not_b(R(7,7))),
			m.and_b(m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
				R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b(Z,m.not_b(R(0,0)),
			m.and_b(m.not_b(R(1,1)),
				m.and_b(m.not_b(R(2,2)),
					m.and_b(m.not_b(R(3,3)),
						m.and_b(m.not_b(R(4,4)),
							m.and_b(m.not_b(R(5,5)),
								m.and_b(m.not_b(R(6,6)),m.not_b(R(7,7)))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m.or_b(C,m.or_b(
			m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
			m.and_b(
				R(7,7),
				Rr(7,7))),
			m.and_b(
				m.not_b(Rd(7,7)),
				R(7,7)));

		// S: N ⊕ V 
		m.xor_b(S,N,V);

		m.assign(Rd,R);
	});
	main | "0101 K@.... d@.... K@...." 	= binary_regconst("subi",[&](cg &m, const variable_decl &Rd, unsigned int K)
	{ 
		const variable_decl R("R",8);

		m.sub_i(R,Rd,K);
			
		// H: !Rd3•K3 + K3•R3 + R3•!Rd3
		m.or_b(H,m.or_b(
			m.and_b(
				m.not_b(Rd(3,3)),
				(K >> 3) & 1),
			m.and_b(
				R(3,3),
				(K >> 3) & 1)),
			m.and_b(
				m.not_b(Rd(3,3)),
				R(3,3)));
		
		// V: Rd7•!K7•!R7 + !Rd7•K7•R7
		m.or_b(V,
			m.and_b(m.and_b(
				Rd(7,7),
				m.not_b((K >> 7) & 1)),
				m.not_b(R(7,7))),
			m.and_b(m.and_b(
				m.not_b(Rd(7,7)),
				(K >> 7) & 1),
				R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b(Z,m.not_b(R(0,0)),
			m.and_b(m.not_b(R(1,1)),
				m.and_b(m.not_b(R(2,2)),
					m.and_b(m.not_b(R(3,3)),
						m.and_b(m.not_b(R(4,4)),
							m.and_b(m.not_b(R(5,5)),
								m.and_b(m.not_b(R(6,6)),m.not_b(R(7,7)))))))));

		// C: !Rd7•K7 + K7•R7 + R7•!Rd7
		m.or_b(C,m.or_b(
			m.and_b(
				m.not_b(Rd(7,7)),
				(K >> 7) & 1),
			m.and_b(
				R(7,7),
				(K >> 7) & 1)),
			m.and_b(
				m.not_b(Rd(7,7)),
				R(7,7)));

		// S: N ⊕ V 
		m.xor_b(S,N,V);
		
		m.assign(Rd,R);
	});

	main | "1001010 d@..... 0101" 			= unary_reg("asr");
	main | "000111 d@.........." 				= unary_reg("rol");
	main | "1001010 d@..... 0111" 			= unary_reg("ror");
	main | "1001010 d@..... 1010" 			= unary_reg("dec");
	main | "1001010 d@..... 0011" 			= unary_reg("inc");
	main | "000010 r@. d@..... r@...." 	= binary_reg("sbc",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		variable_decl R("R",8);

		m.sub_i(R,Rd,m.sub_i(Rr,C));
			
		// H: !Rd3•Rr3 + Rr3•R3 + r3•!Rd3
		m.or_b(H,m.or_b(
			m.and_b(
				m.not_b(Rd(3,3)),
				Rr(3,3)),
			m.and_b(
				R(3,3),
				Rr(3,3))),
			m.and_b(
				m.not_b(Rd(3,3)),
				R(3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m.or_b(V,
			m.and_b(m.and_b(
				Rd(7,7),
				m.not_b(Rr(7,7))),
				m.not_b(R(7,7))),
			m.and_b(m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
				R(7,7)));

		// N: R7
		m.assign(N,R(7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0•Z
		m.and_b(Z,m.not_b(R(0,0)),
			m.and_b(m.not_b(R(1,1)),
				m.and_b(m.not_b(R(2,2)),
					m.and_b(m.not_b(R(3,3)),
						m.and_b(m.not_b(R(4,4)),
							m.and_b(m.not_b(R(5,5)),
								m.and_b(m.not_b(R(6,6)),
									m.and_b(m.not_b(R(7,7)),Z))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m.or_b(C,m.or_b(
			m.and_b(
				m.not_b(Rd(7,7)),
				Rr(7,7)),
			m.and_b(
				R(7,7),
				Rr(7,7))),
			m.and_b(
				m.not_b(Rd(7,7)),
				R(7,7)));

		// S: N ⊕ V 
		m.xor_b(S,N,V);

		m.assign(Rd,R);
	});

	main | "0100 K@.... d@.... K@...." 	= binary_regconst("sbci",[&](cg &m, const variable_decl &Rd, unsigned int K)
	{
		// TODO
	});

	main | "1001010 d@..... 0000" 			= unary_reg("com");

	// word-level arithmetic and logic
	main | "10010110 K@.. d@.. K@...." = [](sm &st)
	{
		unsigned int K = (unsigned int)st.capture_groups["K"];
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		const variable_decl &Rd1 = decl(d);
		const variable_decl &Rd2 = decl(d+1);
		
		st.mnemonic(st.tokens.size(),"adiw",{value_ptr(Rd2.instantiate()),value_ptr(Rd1.instantiate()),valproxy(K).value},[&](cg &c)
		{
			variable_decl R("R",8);

			c.add_i(R,c.concat(Rd2,Rd1),K);

			// V: !Rdh7•R15
			c.and_b(V,c.not_b(Rd1(7,7)),R(15,15));
			
			// N: R15
			c.assign(N,R(15,15));

			// Z: !R15•!R14•!R13•!R12•!R11•!R10•!R9•!R8•!R7•R6•!R5•!R4•!R3•!R2•!R1•!R0
			c.and_b(Z,c.not_b(R(15,15)),
				c.and_b(c.not_b(R(0,0)),
					c.and_b(c.not_b(R(14,14)),
						c.and_b(c.not_b(R(13,13)),
							c.and_b(c.not_b(R(12,12)),
								c.and_b(c.not_b(R(11,11)),
									c.and_b(c.not_b(R(10,10)),
										c.and_b(c.not_b(R(9,9)),
											c.and_b(c.not_b(R(8,8)),
												c.and_b(c.not_b(R(7,7)),
													c.and_b(c.not_b(R(6,6)),
														c.and_b(c.not_b(R(5,5)),
															c.and_b(c.not_b(R(4,4)),
																c.and_b(c.not_b(R(3,3)),
																	c.and_b(c.not_b(R(2,2)),c.not_b(R(1,1)))))))))))))))));
			// C: !R15•Rdh7
			c.and_b(V,Rd1(7,7),c.not_b(R(15,15)));

			// S: N ⊕ V
			c.xor_b(S,N,V);

			c.assign(Rd2,R(8,15));
			c.assign(Rd1,R(0,7));
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10010111 K@.. d@.. K@...." = [](sm &st) 
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.mnemonic(st.tokens.size(),"sbiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
	main | "0000 0011 0 d@... 1 r@..."	= binary_reg("fmul",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	main | "000000111 d@... 0 r@..."		= binary_reg("fmuls",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	main | "000000111 d@... 1 r@..." 		= binary_reg("fmulsu",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	main | "100111 r@. d@..... r@...." 	= binary_reg("mul",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	main | "00000010 d@.... r@...." 		= binary_reg("muls",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	main | "000000110 d@... 0 r@..." 		= binary_reg("muls",[](cg &m, const variable_decl &Rd, const variable_decl &Rr)
	{
		// TODO
	});
	
	// branch branches
	// main | "111101 k@....... s@..." = simple("brbc");
	// main | "111100 k@....... s@..." = [](sm &st)  { st.mnemonic(st.tokens.size(),"brbs"; });
	main | "111101 k@....... 000" 			= branch("brcc",C,false);
	main | "111100 k@....... 000" 			= branch("brcs",C,true);
	main | "111100 k@....... 001" 			= branch("breq",Z,true);
	main | "111101 k@....... 100" 			= branch("brge",S,false);
	main | "111101 k@....... 101" 			= branch("brhc",H,false);
	main | "111100 k@....... 101" 			= branch("brhs",H,true);
	main | "111101 k@....... 111" 			= branch("brid",I,false);
	main | "111100 k@....... 111" 			= branch("brie",I,true);
	main | "111100 k@....... 000" 			= branch("brlo",C,true);
	main | "111100 k@....... 100" 			= branch("brlt",S,true);
	main | "111100 k@....... 010" 			= branch("brmi",N,true);
	main | "111101 k@....... 001"		 		= branch("brne",Z,false);
	main | "111101 k@....... 010" 			= branch("brpl",N,false);
	main | "111101 k@....... 000" 			= branch("brsh",C,false);
	main | "111101 k@....... 110" 			= branch("brtc",T,false);
	main | "111100 k@....... 110" 			= branch("brts",T,true);
	main | "111101 k@....... 011" 			= branch("brvc",V,false);
	main | "111100 k@....... 011" 			= branch("brvs",V,true);
	main | "1111 110r@..... 0 b@..." 		= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sbrc",value_ptr(new reg(st.capture_groups["r"])),st.capture_groups["b"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1111 111 r@..... 0 b@..." 		= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sbrs",value_ptr(new reg(st.capture_groups["r"])),st.capture_groups["b"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "000100 r@. d@..... r@...."	= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"cpse",value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["r"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1001 A@..... b@..." 		= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sbic",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1011 A@..... b@..." 		= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"sbis",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
		//st.skip_next = true;
	};

	// jump branches
	main | "1001010 k@..... 111 k@." | "k@................"	= [](sm &st) 
	{
		int k = st.capture_groups["k"];
		
		st.mnemonic(st.tokens.size(),"call",k,[&](cg &c)
		{
			c.call(k);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "1001010 k@..... 110 k@." | "k@................"	= [](sm &st) 
	{ 
		int k = st.capture_groups["k"];
		
		st.mnemonic(st.tokens.size(),"jmp",k,std::function<void(cg &c)>());
		st.jump(k);
	};

	main | "1101 k@............" 														= [](sm &st) 
	{
		int k = st.capture_groups["k"];

		k = (k <= 2047 ? k : k - 4096);
		
		st.mnemonic(st.tokens.size(),"rcall",k,[&](cg &c)
		{
			c.call(k + 1 + st.address);
		});
		st.jump(st.address + 1);
	};
	main | "1100 k@............" 														= [](sm &st) 
	{
		int k = st.capture_groups["k"];

		k = (k <= 2047 ? k : k - 4096);
		st.mnemonic(st.tokens.size(),"rjmp",k,std::function<void(cg &c)>());
		st.jump(k + 1 + st.address);
	};
	main | 0x9508 = [](sm &st) { st.mnemonic(st.tokens.size(),"ret"); };
	main | 0x9518 = [](sm &st) { st.mnemonic(st.tokens.size(),"reti"); };
	main | 0x9409 = [](sm &st) 
	{ 
		const variable_decl J("J",16);

		st.mnemonic(st.tokens.size(),"ijmp",{},[&](cg &c)
		{
			c.concat(J,r31,r30);
		});
		st.jump(J);
	};

	main | 0x9509 = [](sm &st) { st.mnemonic(st.tokens.size(),"icall"); };
	// icall
	
	// store and load with x,y,z
	main | "1001 001r@. r@.... 1100" = binary_st(r26,r27,false,false);
	main | "1001 001r@. r@.... 1101" = binary_st(r26,r27,false,true);
	main | "1001 001r@. r@.... 1110" = binary_st(r26,r27,true,false);
	main | "1000 001r@. r@.... 1000" = binary_st(r28,r29,false,false);
	main | "1001 001r@. r@.... 1001" = binary_st(r28,r29,false,true);
	main | "1001 001r@. r@.... 1010" = binary_st(r28,r29,true,false);
	main | "10q@.0 q@..1r@. r@.... 1q@..." = binary_stq(reg::Y);
	main | "1000 001r@. r@.... 0000" = binary_st(r30,r31,false,false);
	main | "1001 001r@. r@.... 0001" = binary_st(r30,r31,false,true);
	main | "1001 001r@. r@.... 0010" = binary_st(r30,r31,true,false);
	main | "10q@.0 q@..1r@. r@.... 0q@..." = binary_stq(reg::Z);
	
	main | "1001 000d@. d@.... 1100" = binary_ld(r26,r27,false,false);
	main | "1001 000d@. d@.... 1101" = binary_ld(r26,r27,false,true);
	main | "1001 000d@. d@.... 1110" = binary_ld(r26,r27,true,false);
	main | "1000 000d@. d@.... 1000" = binary_ld(r28,r29,false,false);
	main | "1001 000d@. d@.... 1001" = binary_ld(r28,r29,false,true);
	main | "1001 000d@. d@.... 1010" = binary_ld(r28,r29,true,false);
	main | "10 q@. 0 q@.. 0 d@..... 1 q@..." = binary_ldq(reg::Y);
	main | "1000 000d@. d@.... 0000" = binary_ld(r30,r31,false,false);
	main | "1001 000 d@..... 0001" = binary_ld(r30,r31,false,true);
	main | "1001 000d@. d@.... 0010" = binary_ld(r30,r31,true,false);
	main | "10q@.0 q@..0d@. d@.... 0q@..." = binary_ldq(reg::Z);

	// misc
	main | 0x9598 = simple("break",[](cg &m) { /* TODO */ });
	main | "10010100 K@.... 1011" = [](sm &st) 
	{
		st.mnemonic(st.tokens.size(),"des",st.capture_groups["K"],std::function<void(cg &c)>());
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

	return disassemble<avr_tag>(main,bytes);
}
