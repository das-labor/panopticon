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

#include <panopticon/amd64/generic.hh>
#include <panopticon/amd64/decode.hh>

namespace pls = std::placeholders;

using dis = po::disassembler<po::amd64_tag>;

void po::amd64::add_generic(
	dis& main, dis const& opsize_prfix, dis const& rex_prfix, dis const& rexw_prfix,
	dis const& generic_prfx, dis const& addrsize_prfx, dis const& rep_prfx,
	dis const& imm8, dis const& imm16, dis const& imm32, dis const& imm64,
	dis const& sib,
	dis const& rm8, dis const& rm16, dis const& rm32, dis const& rm64,
	dis const& rm8_0, dis const& rm16_0, dis const& rm32_0, dis const& rm64_0,
	dis const& rm8_1, dis const& rm16_1, dis const& rm32_1, dis const& rm64_1,
	dis const& rm8_2, dis const& rm16_2, dis const& rm32_2, dis const& rm64_2,
	dis const& rm8_3, dis const& rm16_3, dis const& rm32_3, dis const& rm64_3,
	dis const& rm8_4, dis const& rm16_4, dis const& rm32_4, dis const& rm64_4,
	dis const& rm8_5, dis const& rm16_5, dis const& rm32_5, dis const& rm64_5,
	dis const& rm8_6, dis const& rm16_6, dis const& rm32_6, dis const& rm64_6,
	dis const& rm8_7, dis const& rm16_7, dis const& rm32_7, dis const& rm64_7,
	dis const& disp8, dis const& disp16, dis const& disp32, dis const& disp64)
{
	// AAA, AAD, AAM and AAS (32 bits only)
	main[ *generic_prfx >> 0x37			] = [](sm& m) { m.mnemonic(m.tokens.size(),"aaa","",std::list<rvalue>(),[](cg&) {}); };
	main[ *generic_prfx >> 0xd5 >> imm8	] = [](sm& m) { m.mnemonic(m.tokens.size(),"aad","{8}",*m.state.imm,[](cg&) {}); };
	main[ *generic_prfx >> 0xd4 >> imm8	] = [](sm& m) { m.mnemonic(m.tokens.size(),"aam","{8}",*m.state.imm,[](cg&) {}); };
	main[ *generic_prfx >> 0x3f			] = [](sm& m) { m.mnemonic(m.tokens.size(),"aas","",std::list<rvalue>(),[](cg&) {}); };

	// ADC
	std::function<void(cg&,rvalue,rvalue,boost::optional<std::pair<uint8_t,uint8_t>>)> adc = [](cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext)
	{
		using dsl::operator%;
		using dsl::operator/;
		using dsl::operator+;
		using dsl::operator-;
		using dsl::operator*;

		if(sign_ext)
		{
			rvalue sign = b / (1 << (sign_ext->first - 1));
			rvalue rest = b % (1 << (sign_ext->first - 1));
			rvalue ex = (sign * (1 << (sign_ext->second - 1))) + rest;

			m.assign(to_lvalue(a),a + ex + CF);
		}
		else
		{
			m.assign(to_lvalue(a),a + b + CF);
		}
		// set OF, SF, ZF, AF, CF, and PF
	};

	main[ *generic_prfx >>						0x14 >> imm8				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x15 >> imm16				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_16,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >>						0x15 >> imm32				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_32,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x15 >> imm32				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));

	main[ *generic_prfx >>						0x80 >> rm8_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rex_prfix		>> 0x80 >> rm8_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x81 >> rm16_2 >> imm16	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >>						0x81 >> rm32_2 >> imm32	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x81 >> rm64_2 >> imm32	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));

	main[ *generic_prfx >> opsize_prfix	>> 0x83 >> rm16_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,16)));
	main[ *generic_prfx >>						0x83 >> rm32_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,32)));
	main[ *generic_prfx >> rexw_prfix	>> 0x83 >> rm64_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));

	main[ *generic_prfx >>						0x10 >> rm8					] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rex_prfix		>> 0x10 >> rm8					] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x11 >> rm16				] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >>						0x11 >> rm32				] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x11 >> rm64				] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >>						0x12 >> rm8					] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rex_prfix		>> 0x12 >> rm8					] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x13 >> rm16				] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> 						0x13 >> rm32				] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x13 >> rm64				] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	// ADD
	// ADX
	// AMX
	// AND
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
