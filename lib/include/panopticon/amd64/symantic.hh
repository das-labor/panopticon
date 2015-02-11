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

#include <panopticon/amd64/amd64.hh>

#pragma once

namespace po
{
	namespace amd64
	{
		void aaa(cg& m);
		void aam(cg& m, rvalue a, rvalue b);
		void aad(cg& m, rvalue a, rvalue b);
		void aas(cg& m);
		void adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void add(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
		void adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
	// ADX
	// AMX
		void and_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext);
	// BOUND
	// BSF
	// BSR
	// BSWAP
	// BT
	// BTC
	// BTR
	// BTS
	// CALL
	// CALLF
	// CBW
	// CWDE
	// CWQE
	// CDQ
	// CLC
	// CLD
	// CLI
	// CMC
	// CMOVB
	// CMOVNAE
	// CMOVC
	// CMOVBE
	// CMOVNA
	// CMOVL
	// CMOVNGE
	// CMOVLE
	// CMOVNG
	// CMOVNB
	// CMOVAE
	// CMOVNC
	// CMOVNBE
	// CMOVA
	// CMOVNL
	// CMOVGE
	// CMOVNLE
	// CMOVG
	// CMOVNO
	// CMOVNP
	// CMOVPO
	// CMOVNS
	// CMOVNZ
	// CMOVNE
	// CMOVO
	// CMOVP
	// CMOVPE
	// CMOVS
	// CMOVZ
	// CMOVE
	// CMP
	// CMPS
	// CMPSW
	// CMPSB
	// CMPSD
	// CMPSQ
	// CMPXCHG
	// CMPXCHG8B
	// CMPXCHG16B
	// CPUID
	// CWD
	// CWQ
	// CDQ
	// CQO
	// CWDE
	// DAS
	// DEC
	// DIV
	// DAA
	// ENTER
	// HINT_NOP
	// IDIV
	// IMUL
	// IN
	// INC
	// INS
	// INSB
	// INSW
	// INSD
	// INT
	// INT1
	// ICEBP
	// INTO
	// IRET
	// IRETD
	// IRETQ
	// JB
	// JNAE
	// JC
	// JB
	// JBE
	// JNA
	// JCXZ
	// JECXZ
	// JRCXZ
	// JL
	// JNGE
	// JLE
	// JNG
	// JPE
	// JPF
	// JNB
	// JAE
	// JNC
	// JNBE
	// JA
	// JNL
	// JGE
	// JNLE
	// JNO
	// JNP
	// JNS
	// JNZ
	// JNE
	// JO
	// JP
	// JPE
	// JS
	// JZ
	// JE
	// LAHF
	// LDS
	// LEA
	// LEAVE
	// LES
	// LFS
	// LGS
	// LODS
	// LODSB
	// LODSW
	// LODSD
	// LODSQ
	// LOOP
	// LOOPNZ
	// LOOPNE
	// LOOPZ
	// LOOPE
	// LSS
	// MOV
	// MOVBE
	// MOVS
	// MOVSB
	// MOVSW
	// MOVSD
	// MOVSQ
	// MOVSXD
	// MOVSX
	// MOVZX
	// MUL
	// NEG
	// NOP
	// OR
	// OUT
	// OUTS
	// OUTSW
	// OUTSD
	// POP
	// POPA
	// POPAD
	// POPCNT
	// POPF
	// POPFQ
	// POPFD
	// PUSH
	// PUSHA
	// PUSHAD
	// PUSHF
	// PUSHFD
	// PUSHFQ
	// RCL
	// RCR
	// RETF
	// RETN
	// ROL
	// ROR
	// SAHF
	// SAL
	// SHL
	// SALC
	// SETALC
	// SAR
	// SBB
	// SCAS
	// SCASB
	// SCASW
	// SCASD
	// SCASQ
	// SETB
	// SETNE
	// SETNAE
	// SETC
	// SETBE
	// SETNA
	// SETL
	// SETNGE
	// SETLE
	// SETNG
	// SETNB
	// SETAE
	// SETNC
	// SETNBE
	// SETA
	// SETNL
	// SETGE
	// SETNLE
	// SETG
	// SETNO
	// SETNP
	// SETPO
	// SETNS
	// SETNZ
	// SETNE
	// SETO
	// SETP
	// SETPE
	// SETS
	// SETZ
	// SETE
	// SHL
	// SAL
	// SHLD
	// SHR
	// SHRD
	// STC
	// STD
	// STI
	// STOS
	// STOSB
	// STOSW
	// STOSD
	// STOSQ
	// SUB
	// TEST
	// UD
	// US2
	// XADD
	// XCHG
	// XOR
	}
}
