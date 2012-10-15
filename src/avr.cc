#include <iostream>
#include <iomanip>
#include <numeric>
#include <functional>
#include <algorithm>

#include "decoder.hh"
#include "avr.hh"

// memory bank ids for store/load
#define flash 0
#define sram 1

typedef sem_state<avr_tag> sm;
typedef std::function<void(sm &)> sem_action;
typedef code_generator<avr_tag> cg;

unsigned int next_unused = 0;

template<>
bool valid(avr_tag,const name &n)
{
	return (n.base.size() >= 2 && n.base.size() <= 3 && n.base[0] == 'r' && isdigit(n.base[1]) && (n.base.size() == 2 || isdigit(n.base[2]))) ||
				 (n.base.size() == 1 && (n.base[0] == 'I' || n.base[0] == 'T' || n.base[0] == 'H' || n.base[0] == 'S' || n.base[0] == 'V' ||
				 												 n.base[0] == 'N' || n.base[0] == 'Z' || n.base[0] == 'C' || n.base[0] == 'X' || n.base[0] == 'Y' || 
																 n.base[0] == 'Z'));
}

template<>
unsigned int width(avr_tag t,const value_ptr &v)
{
	var_ptr w;
	const_ptr c;

	if((w = dynamic_pointer_cast<variable>(v)))
	{
		const name &n(w->nam);
		assert(valid(t,n));
	
		if(n.base.size() >= 2 && n.base.size() <= 3 && n.base[0] == 'r' && isdigit(n.base[1]) && (n.base.size() == 2 || isdigit(n.base[2])))
			return 8;
		else if(n.base.size() == 1 && (n.base[0] == 'I' || n.base[0] == 'T' || n.base[0] == 'H' || n.base[0] == 'S' || n.base[0] == 'V' ||
																	 n.base[0] == 'N' || n.base[0] == 'Z' || n.base[0] == 'C'))
			return 1;
		else if(n.base.size() == 1 && (n.base[0] == 'X' || n.base[0] == 'Y' || n.base[0] == 'Z'))
			return 16;
		else
			return 0;
	}
	else if((c = dynamic_pointer_cast<constant>(v)))
		return 8;
	else
		return 0;
}

template<> 
name unused(avr_tag)
{
	return name("tmp" + to_string(next_unused++));
}

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

value_ptr subtract(cg &m, name R, valproxy A, valproxy B)
{
	value_ptr a = A.value, b = B.value, r = m.sub_i(R,a,b);
		
	// H: !a3•b3 + b3•r3 + r3•!a3
	m.or_b("H",m.or_b(
		m.and_b(
			m.not_b(m.slice(a,3,3)),
			m.slice(b,3,3)),
		m.and_b(
			m.slice(r,3,3),
			m.slice(b,3,3))),
		m.and_b(
			m.not_b(m.slice(a,3,3)),
			m.slice(r,3,3)));
	
	// V: a7•!b7•!r7 + !a7•b7•r7
	m.or_b("V",
		m.and_b(m.and_b(
			m.slice(a,7,7),
			m.not_b(m.slice(b,7,7))),
			m.not_b(m.slice(r,7,7))),
		m.and_b(m.and_b(
			m.not_b(m.slice(a,7,7)),
			m.slice(b,7,7)),
			m.slice(r,7,7)));

	// N: r7
	m.assign("N",m.slice(r,7,7));

	// Z: !r7•!r6•!r5•!r4•!r3•!r2•!r1•!r0
	m.and_b("Z",m.not_b(m.slice(r,0,0)),
		m.and_b(m.not_b(m.slice(r,1,1)),
			m.and_b(m.not_b(m.slice(r,2,2)),
				m.and_b(m.not_b(m.slice(r,3,3)),
					m.and_b(m.not_b(m.slice(r,4,4)),
						m.and_b(m.not_b(m.slice(r,5,5)),
							m.and_b(m.not_b(m.slice(r,6,6)),m.not_b(m.slice(r,7,7)))))))));

	// C: !a7•b7 + b7•r7 + r7•!a7
	m.or_b("C",m.or_b(
		m.and_b(
			m.not_b(m.slice(a,7,7)),
			m.slice(b,7,7)),
		m.and_b(
			m.slice(r,7,7),
			m.slice(b,7,7))),
		m.and_b(
			m.not_b(m.slice(a,7,7)),
			m.slice(r,7,7)));

	// S: N ⊕ V 
	m.xor_b("S","N","V");

	return r;
}	

flow_ptr avr_decode(vector<typename architecture_traits<avr_tag>::token_type> &bytes, addr_t entry)
{
	decoder<avr_tag> main;

	// memory operations
	main | "001011 r@. d@..... r@...." 	= binary_reg("mov",[](cg &m, const name &Rd, const name &Rr)
	{
		c.assign(Rd,Rr);
	});

	main | "00000001 d@.... r@...." 		= [](sm &st)
	{ 
		value_ptr Rd1(new reg(st.capture_groups["d"])), Rd2(new reg(st.capture_groups["d"] + 1));
		value_ptr Rr1(new reg(st.capture_groups["r"])), Rr2(new reg(st.capture_groups["r"] + 1));

		st.mnemonic(st.tokens.size(),"movw",{Rd1,Rd2,Rr1,Rr2},[&](cg &c)
		{
			c.assign(Rd1->nam,Rr1);
			c.assign(Rd2->nam,Rr2);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10110 A@.. d@..... A@...." 	= [](sm &st)
	{
		value_ptr Rd(new reg(st.capture_groups["d"])), A(new ioreg(st.capture_groups["A"]));

		st.mnemonic(st.tokens.size(),"in",Rd,A,[&](cg &c)
		{
			c.load(Rd->nam,A,io);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "10111 A@.. r@..... A@...." 	= [](sm &st) 	
	{
		value_ptr A(new ioreg(st.capture_groups["A"])), Rr(new reg(st.capture_groups["r"]));

		st.mnemonic(st.tokens.size(),"out",A,Rr,[&](cg &c)
		{
			c.store(A,io,Rr);
		});
		st.jump(st.address + st.tokens.size());
	};
	main | "1001000 d@..... 1111"				= unary_reg("pop",[](cg &c, const name &R) 
	{ 
		c.load(R,"stack",sram);
		c.add("stack","stack",value_ptr(new constant(1,8)));
	});
	main | "1001001 d@..... 1111" 			= unary_reg("push",[](cg &c, const name &R) 
	{ 
		c.add("stack","stack",value_ptr(new constant(1,8)));
		c.store("stack",sram,R); 
	});
	main | "1001010 d@..... 0010" 			= unary_reg("swap",[](cg &c, const name &R)
	{
		m.concat(R,m.slice(R,4,7),m.slice(R,0,3));
	});
	main | "1001001 r@..... 0100" 			= unary_reg("xch",[](cg &c, const name &R)
	{
		value_ptr Z = m.concat("r30","r31");
		value_ptr tmp = m.load(Z,sram);

		m.store(Z,sram,R);
		m.assign(R,tmp);
	});
	main | "11101111 d@.... 1111" 			= unary_reg("ser",[](cg &c, const name &R)
	{
		m.assign(R,0xff);
	});
	main | "1110 K@.... d@.... K@...."	= binary_regconst("ldi",[&](cg &m, const name &Rd, int K)
	{
		m.assign(Rd,K);
	});

	main | "1001001 r@..... 0110" 			= unary_reg("lac",[](cg &c, const name &R)
	{
		// TODO: flags?
		value_ptr Z = m.concat("r30","r31");
		
		// (Z) ← Rd • ($FF – (Z))
		m.store(Z,sram,
			m.and_u(Rd,
				m.sub_i(value_ptr(new constant(0xff,8)),
					m.load(Z,sram))));
	});
	main | "1001001 r@..... 0101" 			= unary_reg("las",[](cg &c, const name &R)
	{
		// TODO: flags?
		value_ptr Z = m.concat("r30","r31");
		
		// (Z) ← Rd v (Z), Rd ← (Z)
		value_ptr tmp = m.load(Z,sram);
		m.store(Z,sram,
			m.or_u(R,tmp));
		m.assign(R,tmp);
	});
	main | "1001001 r@..... 0111" 			= unary_reg("lat",[](cg &c, const name &R)
	{
		// TODO: flags?
		value_ptr Z = m.concat("r30","r31");
		
		// (Z) ← Rd ⊕ (Z), Rd ← (Z)
		value_ptr tmp = m.load(Z,sram);
		m.store(Z,sram,
			m.xor_u(R,tmp));
		m.assign(R,tmp);
	});
	main | "1001000 d@..... 0000" | "k@................" = [](sm &st)
	{
		unsigned int k = st.capture_groups["k"];
		value_ptr Rd(new reg(st.capture_groups["d"]));

		st.mnemonic(st.tokens.size(),"lds",Rd,k,[&](cg &c)
		{
			m.load(Rd->nam,value_ptr(new constant(k,16)));
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
			m.load(Rd->nam,value_ptr(new constant(k,8)));
		});
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
	main | 0x9408 = simple("sec",[](cg &m) { m.assign("C",value_ptr(new constant(1,1))); });
	main | 0x9458 = simple("seh",[](cg &m) { m.assign("H",value_ptr(new constant(1,1))); });
	main | 0x9478 = simple("sei",[](cg &m) { m.assign("I",value_ptr(new constant(1,1))); });
	main | 0x9428 = simple("sen",[](cg &m) { m.assign("N",value_ptr(new constant(1,1))); });
	main | 0x9448 = simple("ses",[](cg &m) { m.assign("S",value_ptr(new constant(1,1))); });
	main | 0x9468 = simple("set",[](cg &m) { m.assign("T",value_ptr(new constant(1,1))); });
	main | 0x9438 = simple("sev",[](cg &m) { m.assign("V",value_ptr(new constant(1,1))); });
	main | 0x9418 = simple("sez",[](cg &m) { m.assign("Z",value_ptr(new constant(1,1))); });
	main | 0x9488 = simple("clc",[](cg &m) { m.assign("C",value_ptr(new constant(0,1))); });
	main | 0x94d8 = simple("clh",[](cg &m) { m.assign("H",value_ptr(new constant(0,1))); });
	main | 0x94f8 = simple("cli",[](cg &m) { m.assign("I",value_ptr(new constant(0,1))); });
	main | 0x94a8 = simple("cln",[](cg &m) { m.assign("N",value_ptr(new constant(0,1))); });
	main | 0x94c8 = simple("cls",[](cg &m) { m.assign("S",value_ptr(new constant(0,1))); });
	main | 0x94e8 = simple("clt",[](cg &m) { m.assign("T",value_ptr(new constant(0,1))); });
	main | 0x94b8 = simple("clv",[](cg &m) { m.assign("V",value_ptr(new constant(0,1))); });
	main | 0x9498 = simple("clz",[](cg &m) { m.assign("Z",value_ptr(new constant(0,1))); });
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
		subtract(m,"R",Rd,K);
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
		m.assign(Rr,subtract(m,"R",Rd,Rr));
	});
	main | "0101 K@.... d@.... K@...." 	= binary_regconst("subi",[&](cg &m, const name &Rd, int K)
	{ 
		m.assign(Rd,subtract(m,"R",Rd,K));
	});

	main | "1001010 d@..... 0101" 			= unary_reg("asr");
	main | "000111 d@.........." 				= unary_reg("rol");
	main | "1001010 d@..... 0111" 			= unary_reg("ror");
	main | "1001010 d@..... 1010" 			= unary_reg("dec");
	main | "1001010 d@..... 0011" 			= unary_reg("inc");
	main | "000010 r@. d@..... r@...." 	= binary_reg("sbc",[](cg &m, const name &Rd, const name &Rr)
	{
		m.assign(Rr,subtract(m,"R",Rd,m.sub_i(Rr,m.concat(value_ptr(new constant(0,7)),"C"))));
	});

	main | "0100 K@.... d@.... K@...." 	= binary_regconst("sbci",[&](cg &m, const name &Rd, int K)
	{
		m.assign(Rd,subtract(m,"R",Rd,m.sub_i(K,m.concat(value_ptr(new constant(0,7)),"C"))));
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
