/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
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
		void do_push(variable v, amd64_state::Mode mode, cg& m);

		// General integer
		void aaa(cg& m);
		void aam(cg& m, rvalue a);
		void aad(cg& m, rvalue a);
		void aas(cg& m);
		void adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void add(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void adcx(cg& m, rvalue a, rvalue b);
		void and_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void bound(cg& m, rvalue a, rvalue b);
		void bsf(cg& m, rvalue a, rvalue b);
		void bsr(cg& m, rvalue a, rvalue b);
		void bswap(cg& m, rvalue a);
		void bt(cg& m, rvalue a, rvalue b);
		void btc(cg& m, rvalue a, rvalue b);
		void btr(cg& m, rvalue a, rvalue b);
		void bts(cg& m, rvalue a, rvalue b);
		void near_call(cg& m, rvalue a, bool rel, amd64_state::OperandSize op);
		void far_call(cg& m, rvalue a, bool rel, amd64_state::OperandSize op);
		void cbw(cg& m);
		void cwde(cg& m);
		void cdqe(cg& m);
		void cbw(cg& m);
		void cwd(cg& m);
		void cdq(cg& m);
		void cqo(cg& m);
		void cmov(cg& m, rvalue a, rvalue b, condition c);
		void cmp(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void cmps(cg& m, rvalue aoff, rvalue boff, int bits);
		void cmpxchg(cg& m, rvalue a, rvalue b, rvalue acc);
		void cmpxchg8b(cg& m, rvalue a);
		void cmpxchg16b(cg& m, rvalue a);
		void cpuid(cg&);
		void daa(cg&);
		void das(cg&);
		void dec(cg& m, rvalue a);
		void div(cg& m, rvalue a, amd64_state::OperandSize);
		void enter(cg& m, rvalue a, rvalue b);
		void hlt(cg&);
		void idiv(cg& m, rvalue a, amd64_state::OperandSize);
		void imul1(cg& m, rvalue a);
		void imul2(cg& m, rvalue a, rvalue b);
		void imul3(cg& m, rvalue a, rvalue b, rvalue c);
		void in(cg& m, rvalue a, rvalue b);
		void dec(cg& m, rvalue a);
		void icebp(cg& m);
		void inc(cg& m, rvalue a);
		void ins(cg& m, rvalue a, amd64_state::OperandSize);
		void int_(cg& m, rvalue a);
		void into(cg& m);
		void iret(cg&,amd64_state::OperandSize);
		void jcc(cg&,rvalue a, condition c);
		void jxz(cg&,rvalue a, rvalue b);
		void lahf(cg& m);
		void lar(cg& m, rvalue a, rvalue b);
		void lxs(cg& m,rvalue a, rvalue b, rvalue seg);
		void lea(cg& m,rvalue a, rvalue b);
		void leave(cg&,amd64_state::OperandSize);
		void lods(cg&,amd64_state::OperandSize,int bytes);
		void loop(cg&,rvalue a,amd64_state::AddressSize);
		void mov(cg&,rvalue a,rvalue b,bool sign_ext);
		void movbe(cg&,rvalue a,rvalue b);
		void movs(cg&,amd64_state::AddressSize,int bytes);
		void movsx(cg&,rvalue a,rvalue b);
		void movzx(cg&,rvalue a,rvalue b);
		void mul(cg& m, rvalue a, amd64_state::OperandSize);
		void neg(cg& m, rvalue a);
		void nop(cg& m);
		void not_(cg& m,rvalue);
		void or_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void out(cg& m, rvalue a, rvalue b);
		void outs(cg& m, rvalue a, amd64_state::OperandSize);
		void pop(cg& m, rvalue a, amd64_state::AddressSize b);
		void popa(cg& m, amd64_state::OperandSize);
		void popcnt(cg& m, rvalue a, rvalue b);
		void popf(cg& m, amd64_state::OperandSize);
		void push(cg& m, rvalue a, amd64_state::AddressSize);
		void pusha(cg& m, amd64_state::OperandSize);
		void pushf(cg& m, amd64_state::OperandSize);
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
		void sbb(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void scas(cg&,amd64_state::OperandSize,int);
		void setcc(cg& m, rvalue a, condition c);
		void shl(cg& m, rvalue a, rvalue b);
		void shr(cg& m, rvalue a, rvalue b);
		void shld(cg& m, rvalue a, rvalue b, rvalue c);
		void shrd(cg& m, rvalue a, rvalue b, rvalue c);
		void sal(cg& m, rvalue a, rvalue b);
		void stos(cg&,amd64_state::OperandSize,int);
		void sub(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void test(cg& m,rvalue a, rvalue b);
		void ud2(cg& m);
		void xadd(cg& m, rvalue a, rvalue b);
		void xchg(cg& m, rvalue a, rvalue b);
		void xor_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
	}
}
