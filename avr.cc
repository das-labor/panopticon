#include <iostream>
#include <iomanip>
#include <numeric>
#include <functional>
#include <algorithm>

#include "decoder.hh"
#include "avr.hh"

using namespace std;

typedef uint16_t token;
typedef vector<uint16_t>::iterator tokiter;

#define unary_reg(x) \
	[](sem_state<token,tokiter> &st)\
	{\
		if(st.capture_groups.count("d"))\
		{\
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,value_ptr(new reg((unsigned int)st.capture_groups["d"])));\
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
		}\
		else\
		{\
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,value_ptr(new reg((unsigned int)st.capture_groups["r"])));\
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
		}\
	};

#define binary_reg(x,func) \
	[](sem_state<token,tokiter> &st)\
	{\
		name Rd = reg(st.capture_groups["d"]).nam;\
		name Rr = reg(st.capture_groups["r"]).nam;\
		mne_ptr m = st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,Rd,Rr);\
		func(Rd,Rr,m);\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define branch(mnemonic,flag,set) \
	[](sem_state<token,tokiter> &st) \
	{	\
		int k = st.capture_groups["k"];\
		guard_ptr g(new guard(flag,relation::Eq,set ? 1 : 0));\
		\
		k = k <= 63 ? k : k - 128;\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),mnemonic,k);\
		st.branch(st.mnemonics.begin()->second,st.address + 1,g->negation());\
		st.branch(st.mnemonics.begin()->second,st.address + k + 1,g);\
	};

#define binary_regconst(x,func)\
	[](sem_state<token,tokiter> &st)\
	{\
		name Rd = reg(st.capture_groups["d"] + 16).nam;\
		int K = st.capture_groups["K"];\
		mne_ptr m = st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,Rd,K);\
		func(Rd,K,m);\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_st(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"st",value_ptr(r),value_ptr(new reg(st.capture_groups["r"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_stq(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"st",value_ptr(new reg(r,reg::PostDisplace,st.capture_groups["q"])),value_ptr(new reg(st.capture_groups["r"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_ld(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ld",value_ptr(new reg(st.capture_groups["r"])),value_ptr(r));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_ldq(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ld",value_ptr(new reg(st.capture_groups["r"])),value_ptr(new reg(r,reg::PostDisplace,st.capture_groups["q"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define simple(x,func)\
	[](sem_state<token,tokiter> &st)\
	{\
		mne_ptr m = st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x);\
		func(m);\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

flow_ptr avr_decode(vector<token> &bytes, addr_t entry)
{
	decoder<token,vector<token>::iterator> main;

	// memory operations
	main | "001011 r@. d@..... r@...." 	= binary_reg("mov",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});

	main | "00000001 d@.... r@...." 		= [](sem_state<token,tokiter> &st)
	{ 
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"movw",
										value_ptr(new reg(st.capture_groups["d"],st.capture_groups["d"] + 1)),
										value_ptr(new reg(st.capture_groups["r"],st.capture_groups["r"] + 1)));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "10110 A@.. d@..... A@...." 	= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"in",
										value_ptr(new reg(st.capture_groups["d"])),
										value_ptr(new ioreg(st.capture_groups["A"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "10111 A@.. r@..... A@...." 	= [](sem_state<token,tokiter> &st) 	
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"out",
										value_ptr(new ioreg(st.capture_groups["A"])),
										value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "1001000 d@..... 1111"				= unary_reg("pop");
	main | "1001001 d@..... 1111" 			= unary_reg("push");
	main | "1001010 d@..... 0010" 			= unary_reg("swap");
	main | "1001001 r@..... 0100" 			= unary_reg("xch");
	main | "1110 K@.... d@.... K@...."	= binary_regconst("ldi",[&](const name &Rd, int K, mne_ptr m)
	{
		m->assign(Rd,K);
	});

	main | "11101111 d@.... 1111" 			= unary_reg("ser");
	main | "1001001 r@..... 0110" 			= unary_reg("lac");
	main | "1001001 r@..... 0101" 			= unary_reg("las");
	main | "1001001 r@..... 0111" 			= unary_reg("lat");
	main | "1001000 d@..... 0000" | "k@................" = [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"lds",value_ptr(new reg(st.capture_groups["d"])),st.capture_groups["k"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10100 k@... d@.... k@...." 	= [](sem_state<token,tokiter> &st)
	{
		unsigned int k = st.capture_groups["k"];

		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"lds",value_ptr(new reg(st.capture_groups["d"] + 16)),(~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | 0x95c8 											= 	[](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"lpm",value_ptr(new reg(0)),value_ptr(new reg(reg::Z)));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | 0x95e8 											= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"spm");
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | 0x95f8 											= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"spm",value_ptr(new reg(reg::Z,reg::PostInc)));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "1001001 d@..... 0000" | "k@................" = [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sts",st.capture_groups["k"],value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10101 k@... d@.... k@...." 	= [](sem_state<token,tokiter> &st)
	{
		unsigned int k = st.capture_groups["k"];

		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sts",(~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15),value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10011010 A@..... b@..." 			= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbi",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10011000 A@..... b@..." 			= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"cbi",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	// SREG operations
	//main | "100101001 s@... 1000" = simple("bclr");
	//main | "100101000 s@... 1000" = simple("bset");
	main | 0x9408 = simple("sec",[](mne_ptr m) { m->assign("C",1); });
	main | 0x9458 = simple("seh",[](mne_ptr m) { m->assign("H",1); });
	main | 0x9478 = simple("sei",[](mne_ptr m) { m->assign("I",1); });
	main | 0x9428 = simple("sen",[](mne_ptr m) { m->assign("N",1); });
	main | 0x9448 = simple("ses",[](mne_ptr m) { m->assign("S",1); });
	main | 0x9468 = simple("set",[](mne_ptr m) { m->assign("T",1); });
	main | 0x9438 = simple("sev",[](mne_ptr m) { m->assign("V",1); });
	main | 0x9418 = simple("sez",[](mne_ptr m) { m->assign("Z",1); });
	main | 0x9488 = simple("clc",[](mne_ptr m) { m->assign("C",0); });
	main | 0x94d8 = simple("clh",[](mne_ptr m) { m->assign("H",0); });
	main | 0x94f8 = simple("cli",[](mne_ptr m) { m->assign("I",0); });
	main | 0x94a8 = simple("cln",[](mne_ptr m) { m->assign("N",0); });
	main | 0x94c8 = simple("cls",[](mne_ptr m) { m->assign("S",0); });
	main | 0x94e8 = simple("clt",[](mne_ptr m) { m->assign("T",0); });
	main | 0x94b8 = simple("clv",[](mne_ptr m) { m->assign("V",0); });
	main | 0x9498 = simple("clz",[](mne_ptr m) { m->assign("Z",0); });
	main | "000101 r@. d@..... r@...." 	= binary_reg("cp",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "000001 r@. d@..... r@...." 	= binary_reg("cpc",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "0011 K@.... d@.... K@...." 	= binary_regconst("cpi",[&](const name &Rd, int K, mne_ptr m)
	{
		value_ptr R = m->sub_i("R",Rd,K);
		
		// H: !Rd3•K3 + K3•R3 + R3•!Rd3
		m->or_b("H",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(K,3,3)),
			m->and_b(
				m->slice(R,3,3),
				m->slice(K,3,3))),
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(R,3,3)));
		
		// V: Rd7•!K7•!R7 + !Rd7•K7•R7
		m->or_b("V",
			m->and_b(m->and_b(
				m->slice(Rd,7,7),
				m->not_b(m->slice(K,7,7))),
				m->not_b(m->slice(R,7,7))),
			m->and_b(m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(K,7,7)),
				m->slice(R,7,7)));

		// N: R7
		m->assign("N",m->slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m->and_b("Z",m->not_b(m->slice(R,0,0)),
			m->and_b(m->not_b(m->slice(R,1,1)),
				m->and_b(m->not_b(m->slice(R,2,2)),
					m->and_b(m->not_b(m->slice(R,3,3)),
						m->and_b(m->not_b(m->slice(R,4,4)),
							m->and_b(m->not_b(m->slice(R,5,5)),
								m->and_b(m->not_b(m->slice(R,6,6)),m->not_b(m->slice(R,7,7)))))))));

		// C: !Rd7•K7 + K7•R7 + R7•!Rd7
		m->or_b("C",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(K,7,7)),
			m->and_b(
				m->slice(R,7,7),
				m->slice(K,7,7))),
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(R,7,7)));

		// S: N ⊕ V 
		m->xor_b("S","N","V");
	});

	main | "001000 d@.........." 				= unary_reg("tst");
	
	// bit-level logic
	//main | "0110 K@.... d@.... K@...." 	= binary_regconst("sbr",or_b,r,r,K);
	main | "000011 d@.........."				= unary_reg("lsl");
	main | "1001010 d@..... 0110"				= unary_reg("lsr");

	// byte-level arithmetic and logic
	main | "000111 r@. d@..... r@...."	= binary_reg("adc",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "000011 r@. d@..... r@...." 	= binary_reg("add",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "001000 r@. d@..... r@...." 	= binary_reg("and",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "0111 K@.... d@.... K@...." 	= binary_regconst("andi",[&](const name &Rd, int K, mne_ptr m)
	{
		m->add_i(Rd,Rd,K);
	});

	main | "001001 r@. d@..... r@...." 	= [](sem_state<token,tokiter> &st)
	{
		int d = st.capture_groups["d"];
		int r = st.capture_groups["r"];

		if(d == r)
		{
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"clr",value_ptr(new reg(st.capture_groups["d"])));
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		}
		else
		{
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"eor",value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["d"])));
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		}
	};
	main | "1001010 d@..... 0001"				= unary_reg("neg");
	main | "001010 r@. d@..... r@...." 	= binary_reg("or",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "0110 K@.... d@.... K@...." 	= binary_regconst("ori",[&](const name &Rd, int K, mne_ptr m)
	{
		m->or_b(Rd,Rd,K);
	});

	main | "000110 r@. d@..... r@...." 	= binary_reg("sub",[&](const name &Rd, const name &Rr, mne_ptr m)
	{
		value_ptr R = m->sub_i("R",Rd,Rr);
		
		// H: !Rd3•Rr3 + Rr3•R3 + R3•!Rd3
		m->or_b("H",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(Rr,3,3)),
			m->and_b(
				m->slice(R,3,3),
				m->slice(Rr,3,3))),
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(R,3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m->or_b("V",
			m->and_b(m->and_b(
				m->slice(Rd,7,7),
				m->not_b(m->slice(Rr,7,7))),
				m->not_b(m->slice(R,7,7))),
			m->and_b(m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(Rr,7,7)),
				m->slice(R,7,7)));

		// N: R7
		m->assign("N",m->slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m->and_b("Z",m->not_b(m->slice(R,0,0)),
			m->and_b(m->not_b(m->slice(R,1,1)),
				m->and_b(m->not_b(m->slice(R,2,2)),
					m->and_b(m->not_b(m->slice(R,3,3)),
						m->and_b(m->not_b(m->slice(R,4,4)),
							m->and_b(m->not_b(m->slice(R,5,5)),
								m->and_b(m->not_b(m->slice(R,6,6)),m->not_b(m->slice(R,7,7)))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m->or_b("C",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(Rr,7,7)),
			m->and_b(
				m->slice(R,7,7),
				m->slice(Rr,7,7))),
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(R,7,7)));

		// S: N ⊕ V 
		m->xor_b("S","N","V");
		m->assign(Rr,R);
	});
	main | "0101 K@.... d@.... K@...." 	= binary_regconst("subi",[&](const name &Rd, int K, mne_ptr m)
	{ 
		m->sub_i("R",Rd,K);
		m->and_b("h1",Rd,K);
		m->and_b("h2","R",K);
		m->and_b("h3",Rd,"R");
		m->or_b("h4","h1","h2");
		m->or_b("h4","h4","h3");
		m->slice("H","h4",3,3);
	});

	main | "1001010 d@..... 0101" 			= unary_reg("asr");
	main | "000111 d@.........." 				= unary_reg("rol");
	main | "1001010 d@..... 0111" 			= unary_reg("ror");
	main | "1001010 d@..... 1010" 			= unary_reg("dec");
	main | "1001010 d@..... 0011" 			= unary_reg("inc");
	main | "000010 r@. d@..... r@...." 	= binary_reg("sbc",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		value_ptr R = m->sub_i("R",Rd,m->sub_i(Rr,"C"));
		
		// H: !Rd3•Rr3 + Rr3•R3 + R3•!Rd3
		m->or_b("H",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(Rr,3,3)),
			m->and_b(
				m->slice(R,3,3),
				m->slice(Rr,3,3))),
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(R,3,3)));
		
		// V: Rd7•!Rr7•!R7 + !Rd7•Rr7•R7
		m->or_b("V",
			m->and_b(m->and_b(
				m->slice(Rd,7,7),
				m->not_b(m->slice(Rr,7,7))),
				m->not_b(m->slice(R,7,7))),
			m->and_b(m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(Rr,7,7)),
				m->slice(R,7,7)));

		// N: R7
		m->assign("N",m->slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m->and_b("Z",m->not_b(m->slice(R,0,0)),
			m->and_b(m->not_b(m->slice(R,1,1)),
				m->and_b(m->not_b(m->slice(R,2,2)),
					m->and_b(m->not_b(m->slice(R,3,3)),
						m->and_b(m->not_b(m->slice(R,4,4)),
							m->and_b(m->not_b(m->slice(R,5,5)),
								m->and_b(m->not_b(m->slice(R,6,6)),m->not_b(m->slice(R,7,7)))))))));

		// C: !Rd7•Rr7 + Rr7•R7 + R7•!Rd7
		m->or_b("C",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(Rr,7,7)),
			m->and_b(
				m->slice(R,7,7),
				m->slice(Rr,7,7))),
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(R,7,7)));

		// S: N ⊕ V 
		m->xor_b("S","N","V");
		m->assign(Rr,R);
	});
	main | "0100 K@.... d@.... K@...." 	= binary_regconst("sbci",[&](const name &Rd, int K, mne_ptr m)
	{
		value_ptr R = m->sub_i("R",Rd,K);
		
		// H: !Rd3•K3 + K3•R3 + R3•!Rd3
		m->or_b("H",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(K,3,3)),
			m->and_b(
				m->slice(R,3,3),
				m->slice(K,3,3))),
			m->and_b(
				m->not_b(m->slice(Rd,3,3)),
				m->slice(R,3,3)));
		
		// V: Rd7•!K7•!R7 + !Rd7•K7•R7
		m->or_b("V",
			m->and_b(m->and_b(
				m->slice(Rd,7,7),
				m->not_b(m->slice(K,7,7))),
				m->not_b(m->slice(R,7,7))),
			m->and_b(m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(K,7,7)),
				m->slice(R,7,7)));

		// N: R7
		m->assign("N",m->slice(R,7,7));

		// Z: !R7•!R6•!R5•!R4•!R3•!R2•!R1•!R0
		m->and_b("Z",m->not_b(m->slice(R,0,0)),
			m->and_b(m->not_b(m->slice(R,1,1)),
				m->and_b(m->not_b(m->slice(R,2,2)),
					m->and_b(m->not_b(m->slice(R,3,3)),
						m->and_b(m->not_b(m->slice(R,4,4)),
							m->and_b(m->not_b(m->slice(R,5,5)),
								m->and_b(m->not_b(m->slice(R,6,6)),m->not_b(m->slice(R,7,7)))))))));

		// C: !Rd7•K7 + K7•R7 + R7•!Rd7
		m->or_b("C",m->or_b(
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(K,7,7)),
			m->and_b(
				m->slice(R,7,7),
				m->slice(K,7,7))),
			m->and_b(
				m->not_b(m->slice(Rd,7,7)),
				m->slice(R,7,7)));

		// S: N ⊕ V 
		m->xor_b("S","N","V");
		m->assign(Rd,R);
	});

	main | "1001010 d@..... 0000" 			= unary_reg("com");

	// word-level arithmetic and logic
	main | "10010110 K@.. d@.. K@...." = [](sem_state<token,tokiter> &st)
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"adiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "10010111 K@.. d@.. K@...." = [](sem_state<token,tokiter> &st) 
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "0000 0011 0 d@... 1 r@..."	= binary_reg("fmul",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "000000111 d@... 0 r@..."		= binary_reg("fmuls",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "000000111 d@... 1 r@..." 		= binary_reg("fmulsu",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "100111 r@. d@..... r@...." 	= binary_reg("mul",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "00000010 d@.... r@...." 		= binary_reg("muls",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	main | "000000110 d@... 0 r@..." 		= binary_reg("muls",[](const name &Rd, const name &Rr,mne_ptr m)
	{
		// TODO
	});
	
	
	// branch branches
	// main | "111101 k@....... s@..." = simple("brbc");
	// main | "111100 k@....... s@..." = [](sem_state<token,tokiter> &st)  { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"brbs"; });
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
	main | "1111 110r@..... 0 b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbrc",value_ptr(new reg(st.capture_groups["r"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1111 111 r@..... 0 b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbrs",value_ptr(new reg(st.capture_groups["r"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "000100 r@. d@..... r@...."	= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"cpse",value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1001 A@..... b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbic",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1011 A@..... b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbis",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};

	// unconditional branches
	main | "1001010 k@..... 111 k@." | "k@................"	= [](sem_state<token,tokiter> &st) 
	{
		int k = st.capture_groups["k"];
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"call",k)
			->call("t",k);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	//	st.unconditional(st.mnemonics.begin()->second,k);
		//st.is_call = true;
	};
	main | "1001010 k@..... 110 k@." | "k@................"	= [](sem_state<token,tokiter> &st) 
	{ 
		int k = st.capture_groups["k"];
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"jmp",k);
		st.unconditional(st.mnemonics.begin()->second,k);
	};

	main | "1101 k@............" 														= [](sem_state<token,tokiter> &st) 
	{
		int k = st.capture_groups["k"];

		k = (k <= 2047 ? k : k - 4096);
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"rcall",k)
			->call("t",k + 1 + st.address);
		st.unconditional(st.mnemonics.begin()->second,st.address + 1);
		//st.unconditional(st.mnemonics.begin()->second,k + 1 + st.address);
		//st.is_call = true;
	};
	main | "1100 k@............" 														= [](sem_state<token,tokiter> &st) 
	{
		int k = st.capture_groups["k"];

		k = (k <= 2047 ? k : k - 4096);
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"rjmp",k);
		st.unconditional(st.mnemonics.begin()->second,k + 1 + st.address);
	};
	main | 0x9508 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ret"); };
	main | 0x9518 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"reti"); };
	main | 0x9409 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ijmp"); };
	main | 0x9509 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"icall"); };
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
	main | 0x9598 = simple("break",[](mne_ptr m) { /* TODO */ });
	main | "10010100 K@.... 1011" = [](sem_state<token,tokiter> &st) 
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"des",st.capture_groups["K"]);
		st.unconditional(st.mnemonics.begin()->second,st.tokens.size() + st.address);
	};

	main | (token)0x0 = simple("nop",[](mne_ptr m) { /* TODO */ });
	main | 0x9588 = simple("sleep",[](mne_ptr m) { /* TODO */ });
	main | 0x95a8 = simple("wdr",[](mne_ptr m) { /* TODO */ });

	// catch all
	main = simple("unk",[](mne_ptr m) { /* TODO */ });

	return disassemble<token,tokiter>(main,bytes);
}
