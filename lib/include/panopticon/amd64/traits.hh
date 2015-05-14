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

#include <panopticon/architecture.hh>

#pragma once

namespace po
{
	// architecture_traits
	struct amd64_tag {};

	struct amd64_state
	{
		enum AddressSize
		{
			AddrSz_64,
			AddrSz_32,
			AddrSz_16,
		};

		enum OperandSize
		{
			OpSz_64,
			OpSz_32,
			OpSz_16,
			OpSz_8,
		};

		enum Mode
		{
			RealMode,		// Real mode / Virtual 8086 mode
			ProtectedMode,	// Protected mode / Long compatibility mode
			LongMode			// Long 64-bit mode
		};

		amd64_state(void) = delete;
		amd64_state(Mode m)
		: addr_sz(), op_sz(), mode(m),
		  rex(false), reg(boost::none), rm(boost::none),
			imm(boost::none), disp(boost::none), moffs(boost::none)
		{
			switch(m)
			{
				case RealMode:			addr_sz = AddrSz_16; op_sz = OpSz_16; break;
				case ProtectedMode:	addr_sz = AddrSz_32; op_sz = OpSz_32; break; // assumes CS.d == 1
				case LongMode:			addr_sz = AddrSz_64; op_sz = OpSz_32; break; // assumes REX.W == 0
				default: ensure(false);
			}
		}

		AddressSize addr_sz;
		OperandSize op_sz;
		Mode mode;

		bool rex;

		boost::optional<lvalue> reg, rm;
		boost::optional<constant> imm, disp, moffs;
	};

	template<>
	struct architecture_traits<amd64_tag>
	{
		using token_type = uint8_t;
		using state_type = amd64_state;
	};

	template<>
	lvalue temporary(amd64_tag);

	template<>
	const std::vector<std::string> &registers(amd64_tag);

	template<>
	uint8_t width(std::string n, amd64_tag);

	namespace amd64
	{
		// 8 bit gp registers
		extern const rvalue al,bl,cl,dl,
				 					 ah,bh,ch,dh,
									 r8l,r9l,r10l,r11l,r12l,r13l,r14l,r15l,
									 spl,bpl,sil,dil;
		// 16 bit gp registers
		extern const rvalue ax,bx,cx,dx,
				 					 r8w,r9w,r10w,r11w,r12w,r13w,r14w,r15w,
									 si,di,sp,bp;
		// 32 bit gp registers
		extern const rvalue eax,ebx,ecx,edx,
				 					 esi,edi,
									 r8d,r9d,r10d,r11d,r12d,r13d,r14d,r15d;
		// 64 bit gp registers
		extern const rvalue rax,rbx,rcx,rdx,
				 					 rsi,rdi,
									 r4,r5,r6,r7,r8,r9,r10,r11,r12,r13,r14,r15;

		// 16 bit management registers
		extern const rvalue sp,bp,ip/*,eflags*/;

		// 32 bit management registers
		extern const rvalue esp,ebp,eip,/*eflags,*/CF,PF,AF,ZF,SF,TF,IF,DF,OF,IOPL,NT,RF,VM,AC,VIF,VIP,ID;

		// 64 bit management registers
		extern const rvalue rsp,rbp,rip,rflags;

		// segment registers
		extern const rvalue cs, ds, fs, ss, gs, es;

		// control registers
		extern const rvalue cr0, cr1, cr2, cr3, cr4, cr8, ldtr, gdtr, idtr;

		// debug registers
		extern const rvalue dr0, dr1, dr2, dr3, dr4, dr5, dr6, dr7;

		using sm = sem_state<amd64_tag>;
		using sem_action = std::function<bool(sm &)>;
		using cg = code_generator<amd64_tag>;
	}
}
