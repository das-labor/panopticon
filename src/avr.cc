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

unsigned int next_unused = 0;

sem_action unary_reg(string x)
{
	return [x](sm &st)
	{
		value_ptr op;

		if(st.capture_groups.count("d"))
			op = value_ptr(new reg((unsigned int)st.capture_groups["d"]));
		else
			op = value_ptr(new reg((unsigned int)st.capture_groups["r"]));
		
		st.mnemonic(st.tokens.size(),x,op);
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_reg(string x, std::function<void(cg &c, name &rd, name &rr)> func)
{
	return [x,func](sm &st)
	{
		name Rd = reg(st.capture_groups["d"]).nam;
		name Rr = reg(st.capture_groups["r"]).nam;

		st.mnemonic(st.tokens.size(),x,Rd,Rr,bind(func,placeholders::_1,Rd,Rr));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action branch(string m, const char *flag, bool set)
{
	return [m,flag,set](sm &st)
	{
		int k = st.capture_groups["k"];
		guard_ptr g(new guard(flag,relation::Eq,set ? 1 : 0));
		
		k = k <= 63 ? k : k - 128;
		st.mnemonic(st.tokens.size(),m,k);
		st.jump(st.address + 1,g->negation());
		st.jump(st.address + k + 1,g);
	};
}

sem_action binary_regconst(string x, std::function<void(cg &c, name &rd, int k)> func)
{
	return [x,func](sm &st)
	{
		name Rd = reg(st.capture_groups["d"] + 16).nam;
		int K = st.capture_groups["K"];\

		st.mnemonic(st.tokens.size(),x,Rd,K,bind(func,placeholders::_1,Rd,K));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action binary_st(reg *r)
{
	return [r](sm &st)
	{
		st.mnemonic(st.tokens.size(),"st",value_ptr(r),value_ptr(new reg(st.capture_groups["r"])),std::function<void(cg &c)>());
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

sem_action binary_ld(reg *r)
{
	return [r](sm &st)
	{
		st.mnemonic(st.tokens.size(),"ld",value_ptr(new reg(st.capture_groups["r"])),value_ptr(r),std::function<void(cg &c)>());
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

template<>
bool valid(avr_tag,const name &n)
{
	return (n.base.size() >= 2 && n.base.size() <= 3 && n.base[0] == 'r' && isdigit(n.base[1]) && isdigit(n.base[2])) ||
				 (n.base.size() == 1 && (n.base[0] == 'I' || n.base[0] == 'T' || n.base[0] == 'H' || n.base[0] == 'S' || n.base[0] == 'V' ||
				 												 n.base[0] == 'N' || n.base[0] == 'Z' || n.base[0] == 'C' || n.base[0] == 'X' || n.base[0] == 'Y' || 
																 n.base[0] == 'Z'));
}

template<>
unsigned int width(avr_tag t,const name &n)
{
	assert(valid(t,n));
	
	if(n.base.size() >= 2 && n.base.size() <= 3 && n.base[0] == 'r' && isdigit(n.base[1]) && isdigit(n.base[2]))
		return 8;
	else if(n.base.size() == 1 && (n.base[0] == 'I' || n.base[0] == 'T' || n.base[0] == 'H' || n.base[0] == 'S' || n.base[0] == 'V' ||
				 												 n.base[0] == 'N' || n.base[0] == 'Z' || n.base[0] == 'C'))
		return 1;
	else if(n.base.size() == 1 && (n.base[0] == 'X' || n.base[0] == 'Y' || n.base[0] == 'Z'))
		return 16;
	else
		return 0;
}

template<> 
name unused(avr_tag)
{
	return name("tmp" + to_string(next_unused++));
}

flow_ptr avr_decode(vector<typename architecture_traits<avr_tag>::token_type> &bytes, addr_t entry)
{
	decoder<avr_tag> main;

	// memory operations
	main | "001011 r@. d@..... r@...." 	= binary_reg("mov",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});

	main | "00000001 d@.... r@...." 		= [](sm &st)
	{ 
		st.mnemonic(st.tokens.size(),"movw",
								value_ptr(new reg(st.capture_groups["d"],st.capture_groups["d"] + 1)),
								value_ptr(new reg(st.capture_groups["r"],st.capture_groups["r"] + 1)),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
	main | "10110 A@.. d@..... A@...." 	= [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"in",
										value_ptr(new reg(st.capture_groups["d"])),
										value_ptr(new ioreg(st.capture_groups["A"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
	main | "10111 A@.. r@..... A@...." 	= [](sm &st) 	
	{
		st.mnemonic(st.tokens.size(),"out",
										value_ptr(new ioreg(st.capture_groups["A"])),
										value_ptr(new reg(st.capture_groups["r"])),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
	main | "1001000 d@..... 1111"				= unary_reg("pop");
	main | "1001001 d@..... 1111" 			= unary_reg("push");
	main | "1001010 d@..... 0010" 			= unary_reg("swap");
	main | "1001001 r@..... 0100" 			= unary_reg("xch");
	main | "1110 K@.... d@.... K@...."	= binary_regconst("ldi",[&](cg &m, const name &Rd, int K)
	{
		m.assign(Rd,K);
	});

	main | "11101111 d@.... 1111" 			= unary_reg("ser");
	main | "1001001 r@..... 0110" 			= unary_reg("lac");
	main | "1001001 r@..... 0101" 			= unary_reg("las");
	main | "1001001 r@..... 0111" 			= unary_reg("lat");
	main | "1001000 d@..... 0000" | "k@................" = [](sm &st)
	{
		st.mnemonic(st.tokens.size(),"lds",value_ptr(new reg(st.capture_groups["d"])),st.capture_groups["k"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | "10100 k@... d@.... k@...." 	= [](sm &st)
	{
		unsigned int k = st.capture_groups["k"];

		st.mnemonic(st.tokens.size(),"lds",value_ptr(new reg(st.capture_groups["d"] + 16)),(~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};

	main | 0x95c8 											= 	[](sm &st)
	{
		st.mnemonic(st.tokens.size(),"lpm",value_ptr(new reg(0)),value_ptr(new reg(reg::Z)),std::function<void(cg &c)>());
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
	main | 0x9408 = simple("sec",[](cg &m) { m.assign("C",1); });
	main | 0x9458 = simple("seh",[](cg &m) { m.assign("H",1); });
	main | 0x9478 = simple("sei",[](cg &m) { m.assign("I",1); });
	main | 0x9428 = simple("sen",[](cg &m) { m.assign("N",1); });
	main | 0x9448 = simple("ses",[](cg &m) { m.assign("S",1); });
	main | 0x9468 = simple("set",[](cg &m) { m.assign("T",1); });
	main | 0x9438 = simple("sev",[](cg &m) { m.assign("V",1); });
	main | 0x9418 = simple("sez",[](cg &m) { m.assign("Z",1); });
	main | 0x9488 = simple("clc",[](cg &m) { m.assign("C",0); });
	main | 0x94d8 = simple("clh",[](cg &m) { m.assign("H",0); });
	main | 0x94f8 = simple("cli",[](cg &m) { m.assign("I",0); });
	main | 0x94a8 = simple("cln",[](cg &m) { m.assign("N",0); });
	main | 0x94c8 = simple("cls",[](cg &m) { m.assign("S",0); });
	main | 0x94e8 = simple("clt",[](cg &m) { m.assign("T",0); });
	main | 0x94b8 = simple("clv",[](cg &m) { m.assign("V",0); });
	main | 0x9498 = simple("clz",[](cg &m) { m.assign("Z",0); });
	main | "000101 r@. d@..... r@...." 	= binary_reg("cp",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "000001 r@. d@..... r@...." 	= binary_reg("cpc",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "0011 K@.... d@.... K@...." 	= binary_regconst("cpi",[&](cg &m, const name &Rd, int K)
	{
		value_ptr R = m.sub_i("R",Rd,K);
		
		// H: !Rd3•K3 + K3•R3 + R3•!Rd3
		m.or_b("H",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(K,3,3)),
			m.and_b(
				m.slice(R,3,3),
				m.slice(K,3,3))),
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(R,3,3)));
		
		// V: Rd7•!K7•!R7 + !Rd7•K7•R7
		m.or_b("V",
			m.and_b(m.and_b(
				m.slice(Rd,7,7),
				m.not_b(m.slice(K,7,7))),
				m.not_b(m.slice(R,7,7))),
			m.and_b(m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(K,7,7)),
				m.slice(R,7,7)));

		// N: R7
		m.assign("N",m.slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b("Z",m.not_b(m.slice(R,0,0)),
			m.and_b(m.not_b(m.slice(R,1,1)),
				m.and_b(m.not_b(m.slice(R,2,2)),
					m.and_b(m.not_b(m.slice(R,3,3)),
						m.and_b(m.not_b(m.slice(R,4,4)),
							m.and_b(m.not_b(m.slice(R,5,5)),
								m.and_b(m.not_b(m.slice(R,6,6)),m.not_b(m.slice(R,7,7)))))))));

		// C: !Rd7•K7 + K7•R7 + R7•!Rd7
		m.or_b("C",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(K,7,7)),
			m.and_b(
				m.slice(R,7,7),
				m.slice(K,7,7))),
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(R,7,7)));

		// S: N ⊕ V 
		m.xor_b("S","N","V");
	});

	main | "001000 d@.........." 				= unary_reg("tst");	// TODO: d w/o offset
	
	// bit-level logic
	//main | "0110 K@.... d@.... K@...." 	= binary_regconst("sbr",or_b,r,r,K);
	main | "000011 d@.........."				= unary_reg("lsl");
	main | "1001010 d@..... 0110"				= unary_reg("lsr");

	// byte-level arithmetic and logic
	main | "000111 r@. d@..... r@...."	= binary_reg("adc",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "000011 r@. d@..... r@...." 	= binary_reg("add",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "001000 r@. d@..... r@...." 	= binary_reg("and",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "0111 K@.... d@.... K@...." 	= binary_regconst("andi",[&](cg &m, const name &Rd, int K)
	{
		m.add_i(Rd,Rd,K);
	});

	main | "001001 r@. d@..... r@...." 	= [](sm &st)
	{
		int d = st.capture_groups["d"];
		int r = st.capture_groups["r"];

		if(d == r)
		{
			st.mnemonic(st.tokens.size(),"clr",value_ptr(new reg(st.capture_groups["d"])),std::function<void(cg &c)>());
			st.jump(st.address + st.tokens.size());
		}
		else
		{
			st.mnemonic(st.tokens.size(),"eor",value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["d"])),std::function<void(cg &c)>());
			st.jump(st.address + st.tokens.size());
		}
	};
	main | "1001010 d@..... 0001"				= unary_reg("neg");
	main | "001010 r@. d@..... r@...." 	= binary_reg("or",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "0110 K@.... d@.... K@...." 	= binary_regconst("ori",[&](cg &m, const name &Rd, int K)
	{
		m.or_b(Rd,Rd,K);
	});

	main | "000110 r@. d@..... r@...." 	= binary_reg("sub",[&](cg &m, const name &Rd, const name &Rr)
	{
		value_ptr R = m.sub_i("R",Rd,Rr);
		
		// H: !Rd3•Rr3 + Rr3•R3 + R3•!Rd3
		m.or_b("H",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(Rr,3,3)),
			m.and_b(
				m.slice(R,3,3),
				m.slice(Rr,3,3))),
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(R,3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m.or_b("V",
			m.and_b(m.and_b(
				m.slice(Rd,7,7),
				m.not_b(m.slice(Rr,7,7))),
				m.not_b(m.slice(R,7,7))),
			m.and_b(m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(Rr,7,7)),
				m.slice(R,7,7)));

		// N: R7
		m.assign("N",m.slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b("Z",m.not_b(m.slice(R,0,0)),
			m.and_b(m.not_b(m.slice(R,1,1)),
				m.and_b(m.not_b(m.slice(R,2,2)),
					m.and_b(m.not_b(m.slice(R,3,3)),
						m.and_b(m.not_b(m.slice(R,4,4)),
							m.and_b(m.not_b(m.slice(R,5,5)),
								m.and_b(m.not_b(m.slice(R,6,6)),m.not_b(m.slice(R,7,7)))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m.or_b("C",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(Rr,7,7)),
			m.and_b(
				m.slice(R,7,7),
				m.slice(Rr,7,7))),
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(R,7,7)));

		// S: N ⊕ V 
		m.xor_b("S","N","V");
		m.assign(Rr,R);
	});
	main | "0101 K@.... d@.... K@...." 	= binary_regconst("subi",[&](cg &m, const name &Rd, int K)
	{ 
		m.sub_i("R",Rd,K);
		m.and_b("h1",Rd,K);
		m.and_b("h2","R",K);
		m.and_b("h3",Rd,"R");
		m.or_b("h4","h1","h2");
		m.or_b("h4","h4","h3");
		m.slice("H","h4",3,3);
	});

	main | "1001010 d@..... 0101" 			= unary_reg("asr");
	main | "000111 d@.........." 				= unary_reg("rol");
	main | "1001010 d@..... 0111" 			= unary_reg("ror");
	main | "1001010 d@..... 1010" 			= unary_reg("dec");
	main | "1001010 d@..... 0011" 			= unary_reg("inc");
	main | "000010 r@. d@..... r@...." 	= binary_reg("sbc",[](cg &m, const name &Rd, const name &Rr)
	{
		value_ptr R = m.sub_i("R",Rd,m.sub_i(Rr,"C"));
		
		// H: !Rd3•Rr3 + Rr3•R3 + R3•!Rd3
		m.or_b("H",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(Rr,3,3)),
			m.and_b(
				m.slice(R,3,3),
				m.slice(Rr,3,3))),
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(R,3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m.or_b("V",
			m.and_b(m.and_b(
				m.slice(Rd,7,7),
				m.not_b(m.slice(Rr,7,7))),
				m.not_b(m.slice(R,7,7))),
			m.and_b(m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(Rr,7,7)),
				m.slice(R,7,7)));

		// N: R7
		m.assign("N",m.slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b("Z",m.not_b(m.slice(R,0,0)),
			m.and_b(m.not_b(m.slice(R,1,1)),
				m.and_b(m.not_b(m.slice(R,2,2)),
					m.and_b(m.not_b(m.slice(R,3,3)),
						m.and_b(m.not_b(m.slice(R,4,4)),
							m.and_b(m.not_b(m.slice(R,5,5)),
								m.and_b(m.not_b(m.slice(R,6,6)),m.not_b(m.slice(R,7,7)))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m.or_b("C",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(Rr,7,7)),
			m.and_b(
				m.slice(R,7,7),
				m.slice(Rr,7,7))),
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(R,7,7)));

		// S: N ⊕ V 
		m.xor_b("S","N","V");
		m.assign(Rr,R);
	});
	main | "0100 K@.... d@.... K@...." 	= binary_regconst("sbci",[&](cg &m, const name &Rd, int K)
	{
		value_ptr R = m.sub_i("R",Rd,K);
		
		// H: !Rd3•K3 + K3•R3 + R3•!Rd3
		m.or_b("H",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(K,3,3)),
			m.and_b(
				m.slice(R,3,3),
				m.slice(K,3,3))),
			m.and_b(
				m.not_b(m.slice(Rd,3,3)),
				m.slice(R,3,3)));
		
		// V: Rd7•!K7•!R7 + !Rd7•K7•R7
		m.or_b("V",
			m.and_b(m.and_b(
				m.slice(Rd,7,7),
				m.not_b(m.slice(K,7,7))),
				m.not_b(m.slice(R,7,7))),
			m.and_b(m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(K,7,7)),
				m.slice(R,7,7)));

		// N: R7
		m.assign("N",m.slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m.and_b("Z",m.not_b(m.slice(R,0,0)),
			m.and_b(m.not_b(m.slice(R,1,1)),
				m.and_b(m.not_b(m.slice(R,2,2)),
					m.and_b(m.not_b(m.slice(R,3,3)),
						m.and_b(m.not_b(m.slice(R,4,4)),
							m.and_b(m.not_b(m.slice(R,5,5)),
								m.and_b(m.not_b(m.slice(R,6,6)),m.not_b(m.slice(R,7,7)))))))));

		// C: !Rd7•K7 + K7•R7 + R7•!Rd7
		m.or_b("C",m.or_b(
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(K,7,7)),
			m.and_b(
				m.slice(R,7,7),
				m.slice(K,7,7))),
			m.and_b(
				m.not_b(m.slice(Rd,7,7)),
				m.slice(R,7,7)));

		// S: N ⊕ V 
		m.xor_b("S","N","V");
		m.assign(Rd,R);
	});

	main | "1001010 d@..... 0000" 			= unary_reg("com");

	// word-level arithmetic and logic
	main | "10010110 K@.. d@.. K@...." = [](sm &st)
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.mnemonic(st.tokens.size(),"adiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
	main | "10010111 K@.. d@.. K@...." = [](sm &st) 
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.mnemonic(st.tokens.size(),"sbiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"],std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size());
	};
	main | "0000 0011 0 d@... 1 r@..."	= binary_reg("fmul",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "000000111 d@... 0 r@..."		= binary_reg("fmuls",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "000000111 d@... 1 r@..." 		= binary_reg("fmulsu",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "100111 r@. d@..... r@...." 	= binary_reg("mul",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "00000010 d@.... r@...." 		= binary_reg("muls",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	main | "000000110 d@... 0 r@..." 		= binary_reg("muls",[](cg &m, const name &Rd, const name &Rr)
	{
		// TODO
	});
	
	
	// branch branches
	// main | "111101 k@....... s@..." = simple("brbc");
	// main | "111100 k@....... s@..." = [](sm &st)  { st.mnemonic(st.tokens.size(),"brbs"; });
	main | "111101 k@....... 000" 			= branch("brcc","C",false);
	main | "111100 k@....... 000" 			= branch("brcs","C",true);
	main | "111100 k@....... 001" 			= branch("breq","Z",true);
	main | "111101 k@....... 100" 			= branch("brge","S",false);
	main | "111101 k@....... 101" 			= branch("brhc","H",false);
	main | "111100 k@....... 101" 			= branch("brhs","H",true);
	main | "111101 k@....... 111" 			= branch("brid","I",false);
	main | "111100 k@....... 111" 			= branch("brie","I",true);
	main | "111100 k@....... 000" 			= branch("brlo","C",true);
	main | "111100 k@....... 100" 			= branch("brlt","S",true);
	main | "111100 k@....... 010" 			= branch("brmi","N",true);
	main | "111101 k@....... 001"		 		= branch("brne","Z",false);
	main | "111101 k@....... 010" 			= branch("brpl","N",false);
	main | "111101 k@....... 000" 			= branch("brsh","C",false);
	main | "111101 k@....... 110" 			= branch("brtc","T",false);
	main | "111100 k@....... 110" 			= branch("brts","T",true);
	main | "111101 k@....... 011" 			= branch("brvc","V",false);
	main | "111100 k@....... 011" 			= branch("brvs","V",true);
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
			c.call("t",k);
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
			c.call("t",k + 1 + st.address);
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
		st.mnemonic(st.tokens.size(),"ijmp",{},[](cg &c)
		{
			c.concat("J","r31","r30");
		});
		st.jump("J");
	};

	main | 0x9509 = [](sm &st) { st.mnemonic(st.tokens.size(),"icall"); };
	// icall
	
	// store and load with x,y,z
	main | "1001 001r@. r@.... 1100" = binary_st(new reg(reg::X));
	main | "1001 001r@. r@.... 1101" = binary_st(new reg(reg::X,reg::PostInc));
	main | "1001 001r@. r@.... 1110" = binary_st(new reg(reg::X,reg::PreDec));
	main | "1000 001r@. r@.... 1000" = binary_st(new reg(reg::Y));
	main | "1001 001r@. r@.... 1001" = binary_st(new reg(reg::Y,reg::PostInc));
	main | "1001 001r@. r@.... 1010" = binary_st(new reg(reg::Y,reg::PreDec));
	main | "10q@.0 q@..1r@. r@.... 1q@..." = binary_stq(reg::Y);
	main | "1000 001r@. r@.... 0000" = binary_st(new reg(reg::Z));
	main | "1001 001r@. r@.... 0001" = binary_st(new reg(reg::Z,reg::PostInc));
	main | "1001 001r@. r@.... 0010" = binary_st(new reg(reg::Z,reg::PreDec));
	main | "10q@.0 q@..1r@. r@.... 0q@..." = binary_stq(reg::Z);
	main | "1001 000d@. d@.... 1100" = binary_ld(new reg(reg::X));
	main | "1001 000d@. d@.... 1101" = binary_ld(new reg(reg::X,reg::PostInc));
	main | "1001 000d@. d@.... 1110" = binary_ld(new reg(reg::X,reg::PreDec));
	main | "1000 000d@. d@.... 1000" = binary_ld(new reg(reg::Y));
	main | "1001 000d@. d@.... 1001" = binary_ld(new reg(reg::Y,reg::PostInc));
	main | "1001 000d@. d@.... 1010" = binary_ld(new reg(reg::Y,reg::PreDec));
	main | "10 q@. 0 q@.. 0 d@..... 1 q@..." = binary_ldq(reg::Y);
	main | "1000 000d@. d@.... 0000" = binary_ld(new reg(reg::Z));
	main | "1001 000 d@..... 0001" = binary_ld(new reg(reg::Z,reg::PostInc));
	main | "1001 000d@. d@.... 0010" = binary_ld(new reg(reg::Z,reg::PreDec));
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
