/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <iostream>
#include <iomanip>
#include <numeric>
#include <functional>
#include <algorithm>

#include <panopticon/disassembler.hh>

#define AVR_PRIVATE
#include <panopticon/avr/avr.hh>
#include <panopticon/avr/util.hh>

#define e(x) token_expr(std::string(x))
#define f(x) token_expr(x)

using namespace po;
using namespace po::avr;
using namespace po::dsl;

namespace po
{
	namespace avr
	{
		unsigned int next_unused = 0;
		std::vector<std::string> registers({
			"r0","r1","r2","r3","r4","r5","r6","r7",
			"r8","r9","r10","r11","r12","r13","r14","r15",
			"r16","r17","r18","r19","r20","r21","r22","r23",
			"r24","r25","r26","r27","r28","r29","r30","r31",
			"I","T","H","S","V","N","Z","C"
		});
	}
}

template<>
lvalue po::temporary(avr_tag)
{
	return variable("t" + std::to_string(po::avr::next_unused++),16);
}

template<>
const std::vector<std::string> &po::registers(avr_tag)
{
	return po::avr::registers;
}

template<>
uint8_t po::width(std::string n, avr_tag)
{
	if(n.c_str()[0] == 'r')
		return 8;
	else if(n.size() == 1)
		return 1;
	else if(n.c_str()[0] == 't')
		return 16;
	else
		ensure(false);
}

// registers
const variable r0 = variable("r0",8), r1 = variable("r1",8), r2 = variable("r2",8), r3 = variable("r3",8), r4 = variable("r4",8), r5 = variable("r5",8), r6 = variable("r6",8),
							 r7 = variable("r7",8), r8 = variable("r8",8), r9 = variable("r9",8), r10 = variable("r10",8), r11 = variable("r11",8), r12 = variable("r12",8),
							 r13 = variable("r13",8), r14 = variable("r14",8), r15 = variable("r15",8), r16 = variable("r16",8), r17 = variable("r17",8), r18 = variable("r18",8),
							 r19 = variable("r19",8), r20 = variable("r20",8), r21 = variable("r21",8), r22 = variable("r22",8), r23 = variable("r23",8), r24 = variable("r24",8),
							 r25 = variable("r25",8), r26 = variable("r26",8), r27 = variable("r27",8), r28 = variable("r28",8), r29 = variable("r29",8), r30 = variable("r30",8),
							 r31 = variable("r31",1), I = variable("I",1), T = variable("T",1), H = variable("H",1), S = variable("S",1), V = variable("V",1), N = variable("N",1), Z = variable("Z",1), C = variable("C",1);


boost::optional<prog_loc> po::avr::disassemble(po::avr_state const& st, boost::optional<prog_loc> prog, po::slab bytes, const po::ref& r)
{
	using po::dsl::operator*;
	using po::dsl::operator+;

	disassembler<avr_tag> main;

	// memory operations
	main[e("001011 r@. d@..... r@....")] = binary_reg("mov",[](cg &m, const variable &Rd, const variable &Rr)
	{
		m.assign(Rd,Rr);
	});

	main[e("00000001 d@.... r@....")] = sem_action([](sm &st)
	{
		variable Rd1 = decode_reg(st.capture_groups["d"] * 2), Rd2 = decode_reg(st.capture_groups["d"] * 2 + 1);
		variable Rr1 = decode_reg(st.capture_groups["r"] * 2), Rr2 = decode_reg(st.capture_groups["r"] * 2 + 1);

		st.mnemonic(st.tokens.size() * 2,"movw","{8}:{8}, {8}:{8}",{Rd1,Rd2,Rr1,Rr2},[&](cg &c)
		{
			c.assign(Rd1,Rr1);
			c.assign(Rd2,Rr2);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});
	main[e("10110 A@.. d@..... A@....")] = sem_action([](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		variable io = decode_ioreg(st.capture_groups["A"]);
		constant off(st.capture_groups["A"]);

		st.mnemonic(st.tokens.size() * 2,"in","{8}, {8::" + io.name() + "}",Rd,off,[&](cg &c)
		{
			c.assign(Rd,sram(off));
		});
		st.jump(st.address + st.tokens.size() * 2);
	});
	main[e("10111 A@.. r@..... A@....")] = sem_action([](sm &st)
	{
		constant off = constant(st.capture_groups["A"]);
		variable io = decode_ioreg(st.capture_groups["A"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size() * 2,"out","{8::" + io.name() + "}, {8}",off,Rr,[&](cg &c)
		{
			c.assign(sram(off),Rr);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});
	main[e("1001000 d@..... 1111")] = unary_reg("pop",[](cg &c, const variable &r)
	{
		memory sp(constant(0x3d),2,BigEndian,"sram");
		c.assign(sp,sp - 1);
		c.assign(r,sram(sp));
	});
	main[e("1001001 d@..... 1111")] = unary_reg("push",[](cg &c, const variable &r)
	{
		memory sp(constant(0x3d),2,BigEndian,"sram");
		c.assign(sram(sp),r);
		c.assign(sp,sp + 1);
	});
	main[e("1001010 d@..... 0010")] = unary_reg("swap",[](cg &c, const variable &r)
	{
		c.assign(r,r / 128 + ((r * 128) % 0x100));
	});
	main[e("1001001 r@..... 0100")] = unary_reg("xch",[](cg &c, const variable &r)
	{
		rvalue z = r30 * 0x100 + r31;
		rvalue tmp = sram(z);
		c.assign(sram(z),r);
		c.assign(r,tmp);
	});
	main[e("11101111 d@.... 1111")] = unary_reg("ser",[](cg &c, const variable &r)
	{
		c	.assign(r,constant(0xff));
	});
	main[e("1110 K@.... d@.... K@....")] = binary_regconst("ldi",[&](cg &m, const variable &Rd, const constant &K)
	{
		m.assign(Rd,K);
	});

	main[e("1001001 r@..... 0110")] = unary_reg("lac",[](cg &c, const variable &r)
	{
		rvalue z = r30 * 0x100 + r31;
		c.assign(sram(z),r & (0xff - sram(z)));
	});
	main[e("1001001 r@..... 0101")] = unary_reg("las",[](cg &c, const variable &r)
	{
		rvalue z = r30 * 0x100 + r31;
		rvalue tmp = sram(z);
		c.assign(sram(z),r | tmp);
		c.assign(r,tmp);
	});
	main[e("1001001 r@..... 0111")] = unary_reg("lat",[](cg &c, const variable &r)
	{
		rvalue z = r30 * 0x100 + r31;
		rvalue tmp = sram(z);
		c.assign(sram(z),r ^ tmp);
		c.assign(r,tmp);
	});
	main[e("1001000 d@..... 0000") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k = constant(st.capture_groups["k"]);
		variable Rd = decode_reg(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size() * 2,"lds","{8}, {8}",Rd,k,[&](cg &c)
		{
			c.assign(Rd,sram(k));
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[e("10100 k@... d@.... k@....")] = sem_action([](sm &st)
	{
		unsigned int k_ = st.capture_groups["k"];
		variable Rd = decode_reg(st.capture_groups["d"] + 16);
		constant k = constant((~k_ & 16) | (k_ & 16) | (k_ & 64) | (k_ & 32) | (k_ & 15));

		st.mnemonic(st.tokens.size() * 2,"lds","{8}, {16}",Rd,k,[&](cg &c)
		{
			c.assign(Rd,sram(k));
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[f(0x95c8)] = sem_action([](sm &st)
	{
		std::list<rvalue> nop;
		st.mnemonic(st.tokens.size() * 2,"lpm","",nop,[&](cg &c)
		{
			rvalue z = r30 * 0x100 + r31;
			c.assign(r1,flash(z));
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[f(0x95e8)] = sem_action([](sm &st)
	{
		// TODO
		st.mnemonic(st.tokens.size() * 2,"spm");
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[f(0x95f8)] = sem_action([](sm &st)
	{
		// TODO
		st.mnemonic(st.tokens.size() * 2,"spm","",decode_preg(30,PostInc),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[e("1001001 d@..... 0000") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k(st.capture_groups["k"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size() * 2,"sts","{8}, {8}",{k,Rr},[&](cg &c)
		{
			c.assign(sram(k),Rr);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[e("10101 k@... d@.... k@....")] = sem_action([](sm &st)
	{
		unsigned int _k = st.capture_groups["k"];
		constant k = constant((~_k & 16) | (_k & 16) | (_k & 64) | (_k & 32) | (_k & 15));
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size() * 2,"sts","{16}, {8}",{k,Rr},[&](cg &c)
		{
			c.assign(sram(k),Rr);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[e("10011010 A@..... b@...")] = sem_action([](sm &st)
	{
		constant k = constant(st.capture_groups["A"]);
		constant b = constant(1 << (st.capture_groups["b"] - 1));

		st.mnemonic(st.tokens.size() * 2,"sbi","{8}, {8}",k,b,[&](cg &c)
		{
			c.assign(sram(k),sram(k) | b);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	main[e("10011000 A@..... b@...")] = sem_action([](sm &st)
	{
		constant k = constant(st.capture_groups["A"]);
		constant b = constant((~(1 << (st.capture_groups["b"] - 1))) & 0xff);

		st.mnemonic(st.tokens.size() * 2,"cbi","{8}, {8}",k,b,[&](cg &c)
		{
			c.assign(sram(k),sram(k) & b);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});

	// SREG operations
	//main[e("100101001 s@... 1000")] = simple("bclr");
	//main[e("100101000 s@... 1000")] = simple("bset");
	main[f(0x9408)] = simple("sec",[](cg &m) { m.assign(C,constant(1)); });
	main[f(0x9458)] = simple("seh",[](cg &m) { m.assign(H,constant(1)); });
	main[f(0x9478)] = simple("sei",[](cg &m) { m.assign(I,constant(1)); });
	main[f(0x9428)] = simple("sen",[](cg &m) { m.assign(N,constant(1)); });
	main[f(0x9448)] = simple("ses",[](cg &m) { m.assign(S,constant(1)); });
	main[f(0x9468)] = simple("set",[](cg &m) { m.assign(T,constant(1)); });
	main[f(0x9438)] = simple("sev",[](cg &m) { m.assign(V,constant(1)); });
	main[f(0x9418)] = simple("sez",[](cg &m) { m.assign(Z,constant(1)); });
	main[f(0x9488)] = simple("clc",[](cg &m) { m.assign(C,constant(0)); });
	main[f(0x94d8)] = simple("clh",[](cg &m) { m.assign(H,constant(0)); });
	main[f(0x94f8)] = simple("cli",[](cg &m) { m.assign(I,constant(0)); });
	main[f(0x94a8)] = simple("cln",[](cg &m) { m.assign(N,constant(0)); });
	main[f(0x94c8)] = simple("cls",[](cg &m) { m.assign(S,constant(0)); });
	main[f(0x94e8)] = simple("clt",[](cg &m) { m.assign(T,constant(0)); });
	main[f(0x94b8)] = simple("clv",[](cg &m) { m.assign(V,constant(0)); });
	main[f(0x9498)] = simple("clz",[](cg &m) { m.assign(Z,constant(0)); });
	main[e("000101 r@. d@..... r@....")] = binary_reg("cp",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = (Rd - Rr) % 0x100;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd, Rr);
		m.equal_i(Z,R,constant(0));
		m.less_i(N,constant(0x7f),R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
	});
	main[e("000001 r@. d@..... r@....")] = binary_reg("cpc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = (Rd - Rr - Cr) % 0x100;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd,Rr);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
	});
	main[e("0011 K@.... d@.... K@....")] = binary_regconst("cpi",[&](cg &m, const variable &Rd, const constant &K)
	{
		rvalue R = (Rd - K) % 0x100;

		m.less_i(H,Rd % 0x10,K % 0x10);
		m.less_i(C,Rd,K);
		m.equal_i(Z,R,constant(0));
		m.less_i(N,constant(0x7f),R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
	});

	// main[e("001000 d@..........")] = tst (alias for and)

	// bit-level logic
	// main[e("0110 K@.... d@.... K@....")] = sbr (alias for ori)
	// main[e("000011 d@..........")] = lsl (alias for add X,X);
	main[e("1001010 d@..... 0110")] = unary_reg("lsr");

	// byte-level arithmetic and logic
	main[e("000111 r@. d@..... r@....")] = binary_reg("adc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd + Rr + Cr;

		m.less_i(H,constant(16),(Rd % 0x10) + (Rr % 0x10));
		m.or_b(V,
			m.and_b(m.less_i(Rr,constant(0x80),m.and_b(m.less_i(Rd,constant(0x80)),m.less_i(constant(0x7f),R))),
			m.and_b(m.less_i(constant(0x7f),Rr),m.and_b(m.less_i(constant(0x7f),Rd),m.less_i(R,constant(0x80))))));
		m.less_i(N,R,constant(0x7f));
		m.equal_i(Z,constant(0),R);
		m.less_i(C,constant(0x100),R);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R % 0x100);
	});
	main[e("000011 r@. d@..... r@....")] = binary_reg("add",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = Rd + Rr;

		m.less_i(H,constant(16),(Rd % 0x10) + (Rr % 0x10));
		m.or_b(V,
			m.and_b(m.less_i(Rr,constant(0x80),m.and_b(m.less_i(Rd,constant(0x80)),m.less_i(constant(0x7f),R))),
			m.and_b(m.less_i(constant(0x7f),Rr),m.and_b(m.less_i(constant(0x7f),Rd),m.less_i(R,constant(0x80))))));
		m.less_i(N,R,constant(0x7f));
		m.equal_i(Z,constant(0),R);
		m.less_i(C,constant(0x100),R);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R % 0x100);
	});
	main[e("001000 r@. d@..... r@....")] = binary_reg("and",[](cg &m, const variable &Rd, const variable &Rr)
	{
		m.and_i(Rd,Rd & Rr);

		m.assign(V,constant(0));										// V: 0
		m.less_i(N,Rd,constant(0x7f));
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.equal_i(Z,constant(0),Rd);
	});
	main[e("0111 K@.... d@.... K@....")] = binary_regconst("andi",[&](cg &m, const variable &Rd, const constant &K)
	{
		m.and_i(Rd,Rd & K);

		m.assign(V,constant(0));										// V: 0
		m.less_i(N,Rd,constant(0x7f));
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.equal_i(Z,constant(0),Rd);
	});

	main[e("001001 r@. d@..... r@....")] = sem_action([](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		if(Rd == Rr)
		{
			st.mnemonic(st.tokens.size() * 2,"clr","",Rd,[&](cg &m)
			{
				m.assign(Rd,constant(0));
				m.assign(V,constant(0));
				m.assign(N,constant(0));
				m.assign(S,constant(0));
				m.assign(Z,constant(0));
			});
			st.jump(st.address + st.tokens.size() * 2);
		}
		else
		{
			st.mnemonic(st.tokens.size() * 2,"eor","",Rd,Rr,[&](cg &m)
			{
				m.xor_i(Rd,Rd,Rr);
				m.assign(V,constant(0));										// V: 0
				m.less_i(N,Rd,constant(0x7f));
				m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
				m.equal_i(Z,constant(0),Rd);
			});
			st.jump(st.address + st.tokens.size() * 2);
		}
	};)
	main[e("1001010 d@..... 0001")] = unary_reg("neg",[](cg &m, const variable &Rd)
	{
		//TODO: m.assign(Rd,Rd ^ 0xff);
	});

	main[e("001010 r@. d@..... r@....")] = binary_reg("or",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("0110 K@.... d@.... K@....")] = binary_regconst("ori",[&](cg &m, const variable &Rd, const constant &K)
	{
		//m.or_b(Rd,Rd,K);
	});

	main[e("000110 r@. d@..... r@....")] = binary_reg("sub",[&](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = (Rd - Rr) % 0x100;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd, Rr);
		m.equal_i(Z,R,constant(0));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R);
	});
	main[e("0101 K@.... d@.... K@....")] = binary_regconst("subi",[&](cg &m, const variable &Rd, const constant &K)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd - K - Cr;

		m.less_i(H,Rd % 0x10, K % 0x10);
		m.less_i(C,Rd, K);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R);
	});

	main[e("1001010 d@..... 0101")] = unary_reg("asr");
	main[e("000111 d@..........")] = unary_reg("rol");
	main[e("1001010 d@..... 0111")] = unary_reg("ror");
	main[e("1001010 d@..... 1010")] = unary_reg("dec");
	main[e("1001010 d@..... 0011")] = unary_reg("inc");
	main[e("000010 r@. d@..... r@....")] = binary_reg("sbc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd - Rr - Cr;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd,Rr);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f),R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));

		m.assign(Rd,R % 0x100);
	});

	main[e("0100 K@.... d@.... K@....")] = binary_regconst("sbci",[&](cg &m, const variable &Rd, const constant &K)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd - K - Cr;

		m.less_i(H,Rd % 0x10, K % 0x10);
		m.less_i(C,Rd,K);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));

		m.assign(Rd,R % 0x100);
	});

	main[e("1001010 d@..... 0000")] = unary_reg("com");

	// word-level arithmetic and logic
	main[e("10010110 K@.. d@.. K@....")] = sem_action([](sm &st)
	{
		constant K = constant((unsigned int)st.capture_groups["K"]);
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		variable Rd1 = decode_reg(d);
		variable Rd2 = decode_reg(d+1);

		st.mnemonic(st.tokens.size() * 2,"adiw","{8}:{8}, {16}",{Rd2,Rd1,K},[&](cg &c)
		{
			rvalue R = Rd2 * 0x100 + Rd1 + K;

			// V: !Rdh7•R15
			c.and_b(V,c.less_i(Rd2,constant(0x80)),c.not_b(c.less_i(R,constant(0x8000))));

			// N: R15
			c.less_i(N,R,constant(0x8000));

			// Z: !R15•!R14•!R13•!R12•!R11•!R10•!R9•!R8•!R7•R6•!R5•!R4•!R3•!R2•!R1•!R0
			c.equal_i(Z,constant(0),R);

			// C: !R15•Rdh7
			c.and_b(V,c.not_b(c.less_i(Rd2,constant(0x80))),c.less_i(R,constant(0x8000)));

			// S: N ⊕ V
			c.or_b(S,c.and_b(c.not_b(N),V),c.and_b(N,c.not_b(V)));

			c.assign(Rd2,R / 0x100);
			c.assign(Rd1,R % 0x100);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});
	main[e("10010111 K@.. d@.. K@....")] = sem_action([](sm &st)
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		constant K = constant((unsigned int)st.capture_groups["K"]);
		variable Rd1 = decode_reg(d);
		variable Rd2 = decode_reg(d+1);

		st.mnemonic(st.tokens.size() * 2,"sbiw","{8}:{8}, {16}",{Rd1,Rd2,K});
		st.jump(st.address + st.tokens.size() * 2);
	});
	main[e("0000 0011 0 d@... 1 r@...")] = binary_reg("fmul",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("000000111 d@... 0 r@...")] = binary_reg("fmuls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("000000111 d@... 1 r@...")] = binary_reg("fmulsu",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("100111 r@. d@..... r@....")] = binary_reg("mul",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("00000010 d@.... r@....")] = binary_reg("muls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("000000110 d@... 0 r@...")] = binary_reg("muls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});

	// branches
	// main[e("111101 k@....... s@...")] = simple("brbc");
	// main[e("111100 k@....... s@...")] = [](sm &st)  { st.mnemonic(st.tokens.size() * 2,"brbs"; });
	main[e("111101 k@....... 000")] = branch("brcc",C,false);
	main[e("111100 k@....... 000")] = branch("brcs",C,true);
	main[e("111100 k@....... 001")] = branch("breq",Z,true);
	main[e("111101 k@....... 100")] = branch("brge",S,false);
	main[e("111101 k@....... 101")] = branch("brhc",H,false);
	main[e("111100 k@....... 101")] = branch("brhs",H,true);
	main[e("111101 k@....... 111")] = branch("brid",I,false);
	main[e("111100 k@....... 111")] = branch("brie",I,true);
	main[e("111100 k@....... 000")] = branch("brlo",C,true);
	main[e("111100 k@....... 100")] = branch("brlt",S,true);
	main[e("111100 k@....... 010")] = branch("brmi",N,true);
	main[e("111101 k@....... 001")] = branch("brne",Z,false);
	main[e("111101 k@....... 010")] = branch("brpl",N,false);
	main[e("111101 k@....... 000")] = branch("brsh",C,false);
	main[e("111101 k@....... 110")] = branch("brtc",T,false);
	main[e("111100 k@....... 110")] = branch("brts",T,true);
	main[e("111101 k@....... 011")] = branch("brvc",V,false);
	main[e("111100 k@....... 011")] = branch("brvs",V,true);
	main[e("1111 110r@..... 0 b@...")] = sem_action([](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbrc","",Rr,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
	});
	main[e("1111 111 r@..... 0 b@...")] = sem_action([](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbrs","",Rr,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
	});
	main[e("000100 r@. d@..... r@....")] = sem_action([](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		variable Rd = decode_reg(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size() * 2,"cpse","",Rd,Rr,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
	});
	main[e("1001 1001 A@..... b@...")] = sem_action([](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbic","",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
	});
	main[e("1001 1011 A@..... b@...")] = sem_action([](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbis","",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
	});

	// jump branches
	main[e("1001010 k@..... 111 k@.") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k = constant((st.capture_groups["k"] * 2) % (st.state.flash_sz));

		st.mnemonic(st.tokens.size() * 2,"call","",k,[&](cg &c)
		{
			c.call_i(k);
		});
		st.jump(st.address + st.tokens.size() * 2);
	});
	main[e("1001010 k@..... 110 k@.") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k = constant((st.capture_groups["k"] * 2) % (st.state.flash_sz));

		st.mnemonic(st.tokens.size() * 2,"jmp","",k,std::function<void(cg &c)>());
		st.jump(k);
	});

	main[e("1101 k@............")] = sem_action([](sm &st)
	{
		int _k = st.capture_groups["k"];
		constant k = constant(((_k <= 2047 ? _k : _k - 4096) * 2 + 2 + st.address) % st.state.flash_sz);

		st.mnemonic(st.tokens.size() * 2,"rcall","",k,[&](cg &c)
		{
			c.call_i(k);
		});
		st.jump(st.address + 2);
	});
	main[e("1100 k@............")] = sem_action([](sm &st)
	{
		int _k = st.capture_groups["k"];
		constant k = constant(((_k <= 2047 ? _k : _k - 4096) * 2 + 2 + st.address) % st.state.flash_sz);

		st.mnemonic(st.tokens.size() * 2,"rjmp","",k,std::function<void(cg &c)>());
		std::cerr << k << " " << _k << " " << (_k <= 2047 ? _k : _k - 4096) << " " << (_k <= 2047 ? _k : _k - 4096) * 2 + 2 + st.address << std::endl;
		st.jump(k);
	});
	main[f(0x9508)] = sem_action([](sm &st) { st.mnemonic(st.tokens.size() * 2,"ret"); });
	main[f(0x9518)] = sem_action([](sm &st) { st.mnemonic(st.tokens.size() * 2,"reti"); });
	main[f(0x9409)] = sem_action([](sm &st)
	{
		variable J(variable("J",16));
		std::list<rvalue> nop;

		st.mnemonic(st.tokens.size() * 2,"ijmp","",nop,[&](cg &c)
		{
			c.assign(J,((r31 * 0x100 + r30) * 2) % constant(st.state.flash_sz));
		});
		st.jump(J);
	});

	// TODO: icall
	main[f(0x9509)] = sem_action([](sm &st) { st.mnemonic(st.tokens.size() * 2,"icall"); });

	// store and load with x,y,z
	main[e("1001 001r@. r@.... 1100")] = binary_st(r26,r27,false,false);
	main[e("1001 001r@. r@.... 1101")] = binary_st(r26,r27,false,true);
	main[e("1001 001r@. r@.... 1110")] = binary_st(r26,r27,true,false);

	main[e("1000 001r@. r@.... 1000")] = binary_st(r28,r29,false,false);
	main[e("1001 001r@. r@.... 1001")] = binary_st(r28,r29,false,true);
	main[e("1001 001r@. r@.... 1010")] = binary_st(r28,r29,true,false);
	main[e("10q@.0 q@..1r@. r@.... 1q@...")] = binary_stq(r28,r29);

	main[e("1000 001r@. r@.... 0000")] = binary_st(r30,r31,false,false);
	main[e("1001 001r@. r@.... 0001")] = binary_st(r30,r31,false,true);
	main[e("1001 001r@. r@.... 0010")] = binary_st(r30,r31,true,false);
	main[e("10q@.0 q@..1r@. r@.... 0q@...")] = binary_stq(r30,r31);

	main[e("1001 000d@. d@.... 1100")] = binary_ld(r26,r27,false,false);
	main[e("1001 000d@. d@.... 1101")] = binary_ld(r26,r27,false,true);
	main[e("1001 000d@. d@.... 1110")] = binary_ld(r26,r27,true,false);

	main[e("1000 000d@. d@.... 1000")] = binary_ld(r28,r29,false,false);
	main[e("1001 000d@. d@.... 1001")] = binary_ld(r28,r29,false,true);
	main[e("1001 000d@. d@.... 1010")] = binary_ld(r28,r29,true,false);
	main[e("10 q@. 0 q@.. 0 d@..... 1 q@...")] = binary_ldq(r28,r29);

	main[e("1000 000d@. d@.... 0000")] = binary_ld(r30,r31,false,false);
	main[e("1001 000 d@..... 0001")] = binary_ld(r30,r31,false,true);
	main[e("1001 000d@. d@.... 0010")] = binary_ld(r30,r31,true,false);
	main[e("10q@.0 q@..0d@. d@.... 0q@...")] = binary_ldq(r30,r31);

	// misc
	main[f(0x9598)] = simple("break",[](cg &m) { /* TODO */ });
	main[e("10010100 K@.... 1011")] = sem_action([](sm &st)
	{
		st.mnemonic(st.tokens.size() * 2,"des","",constant(st.capture_groups["K"]),std::function<void(cg &c)>());
		st.jump(st.tokens.size() * 2 + st.address);
	});

	main[f(0x0)] = simple("nop",[](cg &m) { /* TODO */ });
	main[f(0x9588)] = simple("sleep",[](cg &m) { /* TODO */ });
	main[f(0x95a8)] = simple("wdr",[](cg &m) { /* TODO */ });

	// catch all
	main = sem_action([](sm &st)
	{
		st.mnemonic(1,"unk");
	});

	return program::disassemble<avr_tag>(main,st,bytes,r,prog);
}
