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

#include <panopticon/disassembler.hh>
#include <panopticon/amd64/traits.hh>

#pragma once

namespace po
{
	namespace amd64
	{
		enum condition
		{
			Less,
			LessEqual,
			Equal,
			NotEqual,
			Parity,
			NotParity,
			Carry,
			Sign,
			NotSign,
			Overflow,
			NotOverflow,
			Greater,
			GreaterEqual,
			BelowEqual,
			Above,
			AboveEqual,
		};

		void set_arithm_flags(rvalue res, rvalue res_half, rvalue a, rvalue b, cg& m);
		rvalue sign_ext(rvalue v, unsigned from, unsigned to, cg& m);
		void flagcomp(cg& m, variable const& flag);
		void flagwr(cg& m, variable const& flag,bool val);
		void do_push(rvalue v, amd64_state::Mode mode, cg& m);
		unsigned int bitwidth(rvalue a);

		// General integer
		void aaa(cg& m);
		void aam(cg& m, rvalue a);
		void aad(cg& m, rvalue a);
		void aas(cg& m);
		void adc(cg& m, rvalue a, rvalue b);
		void add(cg& m, rvalue a, rvalue b);
		void adc(cg& m, rvalue a, rvalue b);
		void adcx(cg& m, rvalue a, rvalue b);
		void and_(cg& m, rvalue a, rvalue b);
		void arpl(cg& m, rvalue a, rvalue b);
		void bound(cg& m, rvalue a, rvalue b);
		void bsf(cg& m, rvalue a, rvalue b);
		void bsr(cg& m, rvalue a, rvalue b);
		void bswap(cg& m, rvalue a);
		void bt(cg& m, rvalue a, rvalue b);
		void btc(cg& m, rvalue a, rvalue b);
		void btr(cg& m, rvalue a, rvalue b);
		void bts(cg& m, rvalue a, rvalue b);
		void near_call(cg& m, rvalue a, bool rel);
		void far_call(cg& m, rvalue a, bool rel);
		bool conv(sm&);
		bool conv2(sm&);
		void cmov(cg& m, rvalue a, rvalue b, condition c);
		void cmp(cg& m, rvalue a, rvalue b);
		void cmps(cg& m, rvalue aoff, rvalue boff);
		void cmpxchg(cg& m, rvalue a, rvalue b);
		void cmpxchg8b(cg& m, rvalue a);
		void cmpxchg16b(cg& m, rvalue a);
		void cpuid(cg&);
		void daa(cg&);
		void das(cg&);
		void dec(cg& m, rvalue a);
		void div(cg& m, rvalue a);
		void enter(cg& m, rvalue a, rvalue b);
		void hlt(cg&);
		void idiv(cg& m, rvalue a);
		void imul1(cg& m, rvalue a);
		void imul2(cg& m, rvalue a, rvalue b);
		void imul3(cg& m, rvalue a, rvalue b, rvalue c);
		void in(cg& m, rvalue a, rvalue b);
		void dec(cg& m, rvalue a);
		void icebp(cg& m);
		void inc(cg& m, rvalue a);
		void ins(cg& m, rvalue a, rvalue b);
		void int_(cg& m, rvalue a);
		void into(cg& m);
		bool iret(sm&);
		void jcc(cg&,rvalue a, condition c);
		void jmp(cg&,rvalue a);
		void jxz(cg&,rvalue a, rvalue b);
		void lahf(cg& m);
		void lar(cg& m, rvalue a, rvalue b);
		void lxs(cg& m,rvalue a, rvalue b, rvalue seg);
		void lea(cg& m,rvalue a, rvalue b);
		bool leave(sm&);
		bool lods(sm&);
		bool lodsb(sm&);
		bool loop(sm&);
		bool loope(sm&);
		bool loopne(sm&);
		void mov(cg&,rvalue a,rvalue b,bool sign_ext);
		void movbe(cg&,rvalue a,rvalue b);
		bool movs(sm&);
		bool movsb(sm&);
		void movsx(cg&,rvalue a,rvalue b);
		void movzx(cg&,rvalue a,rvalue b);
		void mul(cg& m, rvalue a);
		void neg(cg& m, rvalue a);
		void nop(cg& m);
		void not_(cg& m,rvalue);
		void or_(cg& m, rvalue a, rvalue b);
		void out(cg& m, rvalue a, rvalue b);
		bool outs(sm&);
		bool pop(sm&);
		bool popa(sm&);
		void popcnt(cg& m, rvalue a, rvalue b);
		void popf(cg& m, rvalue);
		bool push(sm&);
		bool pusha(sm&);
		void pushf(cg& m, rvalue);
		void rcl(cg& m, rvalue a, rvalue b);
		void rcr(cg& m, rvalue a, rvalue b);
		void ret(cg& m, rvalue a);
		void retf(cg& m, rvalue a);
		void ror(cg& m, rvalue a, rvalue b);
		void rol(cg& m, rvalue a, rvalue b);
		void sahf(cg& m);
		void sal(cg& m, rvalue a, rvalue b);
		void salc(cg& m);
		void sar(cg& m, rvalue a, rvalue b);
		void sbb(cg& m, rvalue a, rvalue b);
		bool scas(sm&);
		void setcc(cg& m, rvalue a, condition c);
		void shl(cg& m, rvalue a, rvalue b);
		void shr(cg& m, rvalue a, rvalue b);
		void shld(cg& m, rvalue a, rvalue b, rvalue c);
		void shrd(cg& m, rvalue a, rvalue b, rvalue c);
		void sal(cg& m, rvalue a, rvalue b);
		bool stos(sm&);
		void sub(cg& m, rvalue a, rvalue b);
		void test(cg& m,rvalue a, rvalue b);
		void ud1(cg& m);
		void ud2(cg& m);
		void xadd(cg& m, rvalue a, rvalue b);
		void xchg(cg& m, rvalue a, rvalue b);
		void xor_(cg& m, rvalue a, rvalue b);
	}
}
