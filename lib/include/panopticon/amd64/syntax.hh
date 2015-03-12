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
#include <panopticon/amd64/symantic.hh>
#include <panopticon/amd64/decode.hh>

#pragma once

namespace po
{
	namespace amd64
	{
		namespace pls = std::placeholders;
		using dis = po::disassembler<po::amd64_tag>;

		// 207 generic mnemonics
		template<int Bits>
		void add_generic(
			dis& main, dis const& opsize_prfix, dis const& rex_prfix, dis const& rexw_prfix,
			dis const& lock_or_rep, dis const& addrsize_prfx, dis const& rep_prfx,
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
			dis const& imm_pri = (Bits == 16 ? imm16 : imm32);
			dis const& imm_alt = (Bits == 16 ? imm32 : imm16);
			dis const& rm_pri = (Bits == 16 ? rm16 : rm32);
			dis const& rm_pri_0 = (Bits == 16 ? rm16_0 : rm32_0);
			dis const& rm_pri_1 = (Bits == 16 ? rm16_1 : rm32_1);
			dis const& rm_pri_2 = (Bits == 16 ? rm16_2 : rm32_2);
			dis const& rm_pri_3 = (Bits == 16 ? rm16_3 : rm32_3);
			dis const& rm_pri_4 = (Bits == 16 ? rm16_4 : rm32_4);
			dis const& rm_pri_5 = (Bits == 16 ? rm16_5 : rm32_5);
			dis const& rm_pri_6 = (Bits == 16 ? rm16_6 : rm32_6);
			dis const& rm_pri_7 = (Bits == 16 ? rm16_7 : rm32_7);
			dis const& rm_alt = (Bits != 16 ? rm16 : rm32);
			dis const& rm_alt_0 = (Bits != 16 ? rm16_0 : rm32_0);
			dis const& rm_alt_1 = (Bits != 16 ? rm16_1 : rm32_1);
			dis const& rm_alt_2 = (Bits != 16 ? rm16_2 : rm32_2);
			dis const& rm_alt_3 = (Bits != 16 ? rm16_3 : rm32_3);
			dis const& rm_alt_4 = (Bits != 16 ? rm16_4 : rm32_4);
			dis const& rm_alt_5 = (Bits != 16 ? rm16_5 : rm32_5);
			dis const& rm_alt_6 = (Bits != 16 ? rm16_6 : rm32_6);
			dis const& rm_alt_7 = (Bits != 16 ? rm16_7 : rm32_7);
			amd64_state::OperandSize const opsz_pri = (Bits == 16 ? amd64_state::OpSz_16 : amd64_state::OpSz_32);
			amd64_state::OperandSize const opsz_alt = (Bits != 16 ? amd64_state::OpSz_16 : amd64_state::OpSz_32);
			size_t const pri = std::min(Bits,32);
			size_t const alt = (pri == 32 ? 16 : 32);

			// AAA, AAD, AAM and AAS (32 bits only)
			if(Bits <= 32)
			{
				main[ *rep_prfx >> 0x37_e         ] = nonary("aaa",aaa);
				main[ *rep_prfx >> 0xd5_e >> imm8 ] = unary("aad",decode_imm,aad);
				main[ *rep_prfx >> 0xd4_e >> imm8 ] = unary("aam",decode_imm,aam);
				main[ *rep_prfx >> 0x3f_e         ] = nonary("aas",aas);
			}

			// ADC
			main[ *lock_or_rep >>                 0x14_e >> imm8                ] = binary("adc",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x15_e >> imm_alt             ] = binary("adc",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x15_e >> imm_pri             ] = binary("adc",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x80_e >> rm8_2 >> imm8       ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x81_e >> rm_alt_2 >> imm_alt ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x81_e >> rm_pri_2 >> imm_pri ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x83_e >> rm_alt_2 >> imm8    ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>                 0x83_e >> rm_pri_2 >> imm8    ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>                 0x10_e >> rm8                 ] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x11_e >> rm_alt              ] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x11_e >> rm_pri              ] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x12_e >> rm8                 ] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x13_e >> rm_alt              ] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x13_e >> rm_pri              ] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix >> 0x15_e >> imm32           ] = binary("adc",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix >>  0x80_e >> rm8_2 >>  imm8  ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix >> 0x81_e >> rm64_2 >> imm32 ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix >> 0x83_e >> rm64_2 >> imm8  ] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix >>  0x10_e >> rm8             ] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix >> 0x11_e >> rm64            ] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix >>  0x12_e >> rm8             ] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix >> 0x13_e >> rm64            ] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// ADD
			main[ *lock_or_rep >>						0x04 >> imm8				] = binary("add",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x05 >> imm_alt			] = binary("add",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x05 >> imm_pri			] = binary("add",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x81 >> rm_alt_0 >> imm_alt	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x81 >> rm_pri_0 >> imm_pri	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x83 >> rm_alt_0 >> imm8	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>						0x83 >> rm_pri_0 >> imm8	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>						0x00 >> rm8					] = binary("add",decode_mr,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x01 >> rm_alt				] = binary("add",decode_mr,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x01 >> rm_pri				] = binary("add",decode_mr,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x02 >> rm8					] = binary("add",decode_rm,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x03 >> rm_alt				] = binary("add",decode_rm,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> 						0x03 >> rm_pri				] = binary("add",decode_rm,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix	>> 0x05 >> imm32				] = binary("add",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(add,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x80 >> rm8_0 >> imm8	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x81 >> rm64_0 >> imm32	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix	>> 0x83 >> rm64_0 >> imm8	] = binary("add",decode_mi,std::bind(add,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x00 >> rm8					] = binary("add",decode_mr,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x01 >> rm64				] = binary("add",decode_mr,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix		>> 0x02 >> rm8					] = binary("add",decode_rm,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x03 >> rm64				] = binary("add",decode_rm,std::bind(add,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// ADCX
			main[ 0x66_e >> 0x0f_e >> 0x38_e >> 0xf6_e >> rm32 ] = binary("adcx",decode_rm,adcx);

			if(Bits == 64)
				main[ rexw_prfix >> 0x66_e >> 0x0f_e >> 0x38_e >> 0xf6_e >> rm64 ] = binary("adcx",decode_rm,adcx);

			// AND
			main[ *lock_or_rep >>						0x24 >> imm8				] = binary("and",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x25 >> imm_alt			] = binary("and",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x25 >> imm_pri			] = binary("and",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x81 >> rm_alt_4 >> imm_alt	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x81 >> rm_pri_4 >> imm_pri	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x83 >> rm_alt_4 >> imm8	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>						0x83 >> rm_pri_4 >> imm8	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>						0x20 >> rm8					] = binary("and",decode_mr,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x21 >> rm_alt				] = binary("and",decode_mr,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x21 >> rm_pri				] = binary("and",decode_mr,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x22 >> rm8					] = binary("and",decode_rm,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x23 >> rm_alt				] = binary("and",decode_rm,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> 						0x23 >> rm_pri				] = binary("and",decode_rm,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix	>> 0x25 >> imm32				] = binary("and",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(and_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x80 >> rm8_4 >> imm8	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x81 >> rm64_4 >> imm32	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix	>> 0x83 >> rm64_4 >> imm8	] = binary("and",decode_mi,std::bind(and_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x20 >> rm8					] = binary("and",decode_mr,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x21 >> rm64				] = binary("and",decode_mr,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix		>> 0x22 >> rm8					] = binary("and",decode_rm,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x23 >> rm64				] = binary("and",decode_rm,std::bind(and_,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// BOUND
			if(Bits <= 32)
			{
				main[ 					 0x62_e >> rm_pri ] = binary("bound",decode_rm,bound);
				main[ opsize_prfix >> 0x62_e >> rm_alt ] = binary("bound",decode_rm,bound);
			}

			// BSF
			main[ 					 0x0f_e >> 0xbc_e >> rm_pri ] = binary("bsf",decode_rm,bsf);
			main[ opsize_prfix >> 0x0f_e >> 0xbc_e >> rm_alt ] = binary("bsf",decode_rm,bsf);

			if(Bits == 64)
				main[ rexw_prfix >> 0x0f_e >> 0xbc_e >> rm64 ] = binary("bsf",decode_rm,bsf);

			// BSR
			main[ 					 0x0f_e >> 0xbd_e >> rm_pri ] = binary("bsr",decode_rm,bsr);
			main[ opsize_prfix >> 0x0f_e >> 0xbd_e >> rm_alt ] = binary("bsr",decode_rm,bsr);

			if(Bits == 64)
				main[ rexw_prfix >> 0x0f_e >> 0xbd_e >> rm64 ] = binary("bsr",decode_rm,bsr);

			// BSWAP
			main[ 0x0f_e >> 0xc8_e >> rm32 ] = unary("bswap",decode_o,bswap);
			if(Bits == 64)
				main[ rexw_prfix >> 0x0f_e >> 0xc8_e >> rm64 ] = unary("bswap",decode_o,bswap);

			// BT
			main[ 					 0x0f_e >> 0xa3_e >> rm_pri			] = binary("bt",decode_rm,bt);
			main[ opsize_prfix >> 0x0f_e >> 0xa3_e >> rm_alt			] = binary("bt",decode_rm,bt);
			main[ 					 0x0f_e >> 0xba_e >> rm_pri_4 >> imm8 ] = binary("bt",decode_mi,bt);
			main[ opsize_prfix >> 0x0f_e >> 0xba_e >> rm_alt_4 >> imm8 ] = binary("bt",decode_mi,bt);

			if(Bits == 64)
			{
				main[ rexw_prfix >> 0x0f_e >> 0xa3_e >> rm64					] = binary("bt",decode_rm,bt);
				main[ rexw_prfix >> 0x0f_e >> 0xba_e >> rm64_4 >> imm8	] = binary("bt",decode_mi,bt);
			}

			// BTC
			main[ 					 0x0f_e >> 0xbb_e >> rm_pri				] = binary("btc",decode_rm,btc);
			main[ opsize_prfix >> 0x0f_e >> 0xbb_e >> rm_alt				] = binary("btc",decode_rm,btc);
			main[ 					 0x0f_e >> 0xba_e >> rm_pri_7 >> imm8	] = binary("btc",decode_mi,btc);
			main[ opsize_prfix >> 0x0f_e >> 0xba_e >> rm_alt_7 >> imm8	] = binary("btc",decode_mi,btc);

			if(Bits == 64)
			{
				main[ rexw_prfix >> 0x0f_e >> 0xbb_e >> rm64					] = binary("btc",decode_rm,btc);
				main[ rexw_prfix >> 0x0f_e >> 0xba_e >> rm64_7 >> imm8	] = binary("btc",decode_mi,btc);
			}

			// BTR
			main[ 					 0x0f_e >> 0xb3_e >> rm_pri				] = binary("btr",decode_rm,btr);
			main[ opsize_prfix >> 0x0f_e >> 0xb3_e >> rm_alt				] = binary("btr",decode_rm,btr);
			main[ 					 0x0f_e >> 0xba_e >> rm_pri_6 >> imm8	] = binary("btr",decode_mi,btr);
			main[ opsize_prfix >> 0x0f_e >> 0xba_e >> rm_alt_6 >> imm8	] = binary("btr",decode_mi,btr);

			if(Bits == 64)
			{
				main[ rexw_prfix >> 0x0f_e >> 0xb3_e >> rm64					] = binary("btr",decode_rm,btr);
				main[ rexw_prfix >> 0x0f_e >> 0xba_e >> rm64_6 >> imm8	] = binary("btr",decode_mi,btr);
			}

			// BTS
			main[                 0x0f_e >> 0xab_e >> rm_pri           ] = binary("bts",decode_rm,bts);
			main[ opsize_prfix >> 0x0f_e >> 0xab_e >> rm_alt			     ] = binary("bts",decode_rm,bts);
			main[                 0x0f_e >> 0xba_e >> rm_pri_5 >> imm8 ] = binary("bts",decode_mi,bts);
			main[ opsize_prfix >> 0x0f_e >> 0xba_e >> rm_alt_5 >> imm8 ] = binary("bts",decode_mi,bts);

			if(Bits == 64)
			{
				main[ rexw_prfix >> 0x0f_e >> 0xab_e >> rm64					] = binary("bts",decode_rm,bts);
				main[ rexw_prfix >> 0x0f_e >> 0xba_e >> rm64_5 >> imm8	] = binary("bts",decode_mi,bts);
			}

			// CALL
			if(Bits <= 32)
			{
				main[ opsize_prfix >> 0xe8_e >> rm_alt >> imm16          ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,true,opsz_alt));
				main[                 0xff_e >> rm_pri_2                 ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,false,opsz_pri));
				main[ opsize_prfix >> 0xff_e >> rm_alt_2                 ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,false,opsz_alt));
				main[                 0x9a_e >> rm_pri >> imm16 >> imm32 ] = unary("call",decode_d,std::bind(far_call,pls::_1,pls::_2,true,opsz_pri));
				main[ opsize_prfix >> 0x9a_e >> rm_alt >> imm16 >> imm16 ] = unary("call",decode_d,std::bind(far_call,pls::_1,pls::_2,true,opsz_alt));
			}

			if(Bits == 64)
			{
				main[               0xff_e >> rm64_2 ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,false,amd64_state::OpSz_64));
				main[ rexw_prfix >> 0xff_e >> rm64_3 ] = unary("call",decode_m,std::bind(far_call,pls::_1,pls::_2,false,amd64_state::OpSz_64));
			}

			main[                 0xff_e >> rm_pri_3        ] = unary("call",decode_m,std::bind(far_call,pls::_1,pls::_2,false,opsz_pri));
			main[ opsize_prfix >> 0xff_e >> rm_alt_3        ] = unary("call",decode_m,std::bind(far_call,pls::_1,pls::_2,false,opsz_alt));
			main[                 0xe8_e >> rm_pri >> imm32 ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,true,opsz_pri));

			// CBW
			main[                 0x98_e ] = nonary("cbw",cbw);
			main[ opsize_prfix >> 0x98_e ] = nonary("cwde",cwde);
			if(Bits == 64)
				main[ rexw_prfix >> 0x98_e ] = nonary("cwqe",cwqe);

			// CDQ

			// CLC
			main[ 0xf8_e ] = nonary("clc",std::bind(flagwr,pls::_1,CF,false));

			// CLD
			main[ 0xfc_e ] = nonary("cld",std::bind(flagwr,pls::_1,DF,false));

			// CLI
			main[ 0xfa_e ] = nonary("cli",std::bind(flagwr,pls::_1,IF,false));

			// CMC
			main[ 0xf5_e ] = nonary("cmc",std::bind(flagcomp,pls::_1,CF));

			// CMOVcc
			std::function<void(uint8_t, std::string const&, amd64::condition)> cmovcc = [&](uint8_t op, std::string const& suffix, amd64::condition cond)
			{
				main[                 0x0f_e >> op >> rm_pri ] = binary("cmov" + suffix,decode_rm,std::bind(cmov,pls::_1,pls::_2,pls::_3,cond));
				main[ opsize_prfix >> 0x0f_e >> op >> rm_pri ] = binary("cmov" + suffix,decode_rm,std::bind(cmov,pls::_1,pls::_2,pls::_3,cond));

				if(Bits == 64)
					main[ rexw_prfix >> 0x0f_e >> op >> rm64 ] = binary("cmov" + suffix,decode_rm,std::bind(cmov,pls::_1,pls::_2,pls::_3,cond));
			};

			cmovcc(0x40,"o",Overflow);
			cmovcc(0x41,"no",NotOverflow);
			cmovcc(0x42,"c",Carry);
			cmovcc(0x43,"ae",AboveEqual);
			cmovcc(0x44,"e",Equal);
			cmovcc(0x45,"ne",NotEqual);
			cmovcc(0x46,"be",BelowEqual);
			cmovcc(0x47,"a",Above);
			cmovcc(0x48,"s",Sign);
			cmovcc(0x49,"ns",NotSign);
			cmovcc(0x4a,"p",Parity);
			cmovcc(0x4b,"np",NotParity);
			cmovcc(0x4c,"l",Less);
			cmovcc(0x4d,"ge",GreaterEqual);
			cmovcc(0x4e,"le",LessEqual);
			cmovcc(0x4f,"g",Greater);

			// CMP
			main[ *lock_or_rep >>                 0x3b_e >> imm8                ] = binary("cmp",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x3c_e >> imm_alt             ] = binary("cmp",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x3c_e >> imm_pri             ] = binary("cmp",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x81_e >> rm_alt_7 >> imm_alt ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x81_e >> rm_pri_7 >> imm_pri ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x83_e >> rm_alt_7 >> imm8    ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>                 0x83_e >> rm_pri_7 >> imm8    ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>                 0x38_e >> rm8                 ] = binary("cmp",decode_mr,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x39_e >> rm_alt              ] = binary("cmp",decode_mr,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x39_e >> rm_pri              ] = binary("cmp",decode_mr,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x3a_e >> rm8                 ] = binary("cmp",decode_rm,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix >> 0x3b_e >> rm_alt              ] = binary("cmp",decode_rm,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>                 0x3b_e >> rm_pri              ] = binary("cmp",decode_rm,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix >> 0x3c_e >> imm32           ] = binary("cmp",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(cmp,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix >>  0x80_e >> rm8_7 >>  imm8  ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix >> 0x81_e >> rm64_7 >> imm32 ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix >> 0x83_e >> rm64_7 >> imm8  ] = binary("cmp",decode_mi,std::bind(cmp,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix >>  0x38_e >> rm8             ] = binary("cmp",decode_mr,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix >> 0x39_e >> rm64            ] = binary("cmp",decode_mr,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix >>  0x3a_e >> rm8             ] = binary("cmp",decode_rm,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix >> 0x3b_e >> rm64            ] = binary("cmp",decode_rm,std::bind(cmp,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// CMPS/CMPSW/CMPSD/CMPSQ
			variable Xsi = (Bits == 16 ? si : (Bits == 32 ? esi : rsi));
			variable Xdi = (Bits == 16 ? di : (Bits == 32 ? edi : rdi));
			variable Xa = (Bits == 16 ? ax : (Bits == 32 ? eax : rax));

			main[ *lock_or_rep >>                  0xa6_e ] = nonary("cmpsb",std::bind(cmps,pls::_1,Xdi,Xsi,8));
			main[ *lock_or_rep >>                  0xa7_e ] = nonary("cmpsw",std::bind(cmps,pls::_1,Xdi,Xsi,pri));
			main[ *lock_or_rep >> addrsize_prfx >> 0xa7_e ] = nonary("cmpsd",std::bind(cmps,pls::_1,Xdi,Xsi,alt));

			if(Bits == 64)
				main[ *lock_or_rep >> rexw_prfix >> 0xa7_e ] = nonary("cmpsq",std::bind(cmps,pls::_1,Xdi,Xsi,64));

			// CMPXCHG
			main[ *lock_or_rep >>                 0x0f_e >> 0xb0_e >> rm8 ] = binary("cmpxchg",decode_mr,std::bind(cmpxchg,pls::_1,pls::_2,pls::_3,al));
			main[ *lock_or_rep >>                 0x0f_e >> 0xb1_e >> rm_pri ] = binary("cmpxchg",decode_mr,std::bind(cmpxchg,pls::_1,pls::_2,pls::_3,Xa));
			main[ *lock_or_rep >> opsize_prfix >> 0x0f_e >> 0xb1_e >> rm8 ] = binary("cmpxchg",decode_mr,std::bind(cmpxchg,pls::_1,pls::_2,pls::_3,Xa));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rex_prfix >> 0x0f_e >> 0xb0_e >> rm_pri ] = binary("cmpxchg",decode_mr,std::bind(cmpxchg,pls::_1,pls::_2,pls::_3,al));
				main[ *lock_or_rep >> rex_prfix >> 0x0f_e >> 0xb1_e >> rm_pri ] = binary("cmpxchg",decode_mr,std::bind(cmpxchg,pls::_1,pls::_2,pls::_3,Xa));
			}

			// CMPXCHG8B
			main[ *lock_prfix >> 0x0f_e >> 0xc7_e >> rm_pri_1 >> m64 ] = unary("cmpxchg8b",decode_m,std::bind(cmpxchg,pls::_1,pls::_2,8));

			// CMPXCHG16B
			if(Bits == 64)
				main[ *lock_prfix >> rexw_prfix >> 0x0f_e >> 0xc7_e >> rm_pri_1 >> m128 ] = unary("cmpxchg16b",decode_m,std::bind(cmpxchg,pls::_1,pls::_2,16));

			// CPUID
			main[ 0x0f_e >> 0xa2_e ] = nonary("cpuid",cpuid);

			// CWD, CWQ, CQO
			if(Bits == 16)
			{
				main[                 0x99_e ] = nonary("cwd",cwd);
				main[ opsize_prfix >> 0x99_e ] = nonary("cdq",cdq);
			}
			else if(Bits >= 32)
			{
				main[                 0x99_e ] = nonary("cdq",cdq);
				main[ opsize_prfix >> 0x99_e ] = nonary("cwd",cwd);

				if(Bits == 64)
					main[ rexw_prfix >>   0x99_e ] = nonary("cqo",cqo);
			}

			// DAS
			if(Bits <= 32)
				main[ 0x2f_e ] = nonary("das",das);

			// DEC
			main[ *lock_or_rep >>                 0xfe_e >> rm8_1 ] = unary("dec",decode_m,dec);
			main[ *lock_or_rep >>                 0xff_e >> rm_pri_1 ] = unary("dec",decode_m,dec);
			main[ *lock_or_rep >> opsize_prfix >> 0xff_e >> rm_alt_1 ] = unary("dec",decode_m,dec);

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rex_prfix >> 0xfe_e >> rm8_1 ] = unary("dec",decode_m,dec);
				main[ *lock_or_rep >> rexw_prfix >> 0xff_e >> rm64_1 ] = unary("dec",decode_m,dec);
			}
			else
			{
				main[ *lock_or_rep >>                 0x48_e >> rpri ] = unary("dec",decode_o,dec);
				main[ *lock_or_rep >> opsize_prfix >> 0x48_e >> ralt ] = unary("dec",decode_o,dec);
			}

			// DIV
			main[ *lock_or_rep >>                 0xf6_e >> rm8_6 ] = unary("div",decode_m,div);
			main[ *lock_or_rep >>                 0xf7_e >> rm_pri_6 ] = unary("div",decode_m,div);
			main[ *lock_or_rep >> opsize_prfix >> 0xf7_e >> rm_alt_6 ] = unary("div",decode_m,div);

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rex_prfix >> 0xf6_e >> rm8_6 ] = unary("div",decode_m,div);
				main[ *lock_or_rep >> rexw_prfix >> 0xf7_e >> rm64_6 ] = unary("div",decode_m,div);
			}

			// DAA
			if(Bits <= 32)
				main[ 0x27_e ] = nonary("daa",daa);

			// ENTER
			main[ 0xc8_e >> imm16 >> imm8 ] = binary("enter",decode_ii,enter);

			// HLT
			main[ 0xf4_e ] = nonary("hlt",hlt);

			// IDIV
			main[ *lock_or_rep >>                 0xf6_e >> rm8_7 ] = unary("idiv",decode_m,idiv);
			main[ *lock_or_rep >>                 0xf7_e >> rm_pri_7 ] = unary("idiv",decode_m,idiv);
			main[ *lock_or_rep >> opsize_prfix >> 0xf7_e >> rm_alt_7 ] = unary("idiv",decode_m,idiv);

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rex_prfix >> 0xf6_e >> rm8_7 ] = unary("idiv",decode_m,idiv);
				main[ *lock_or_rep >> rexw_prfix >> 0xf7_e >> rm64_7 ] = unary("idiv",decode_m,idiv);
			}

			// TODO: IMUL
			// TODO: IN
			// TODO: INC
			// TODO: INS
			// TODO: INSB
			// TODO: INSW
			// TODO: INSD
			// TODO: INT
			// TODO: INT1
			// TODO: ICEBP
			// TODO: INTO
			// TODO: IRET
			// TODO: IRETD
			// TODO: IRETQ

			// J*CXZ
			if(Bits == 16)
			{
				main[                  0xe3_e >> imm8 ] = unary("jcxz",decode_imm,std::bind(jcxz,pls::_1,pls::_2,cx));
				main[ addrsize_prfx >> 0xe3_e >> imm8 ] = unary("jecxz",decode_imm,std::bind(jcxz,pls::_1,pls::_2,ecx));
			}
			else if(Bits == 32)
			{
				main[                  0xe3_e >> imm8 ] = unary("jecxz",decode_imm,std::bind(jcxz,pls::_1,pls::_2,ecx));
				main[ addrsize_prfx >> 0xe3_e >> imm8 ] = unary("jcxz",decode_imm,std::bind(jcxz,pls::_1,pls::_2,cx));
			}
			else if(Bits == 64)
			{
				main[                  0xe3_e >> imm8 ] = unary("jrcxz",decode_imm,std::bind(jcxz,pls::_1,pls::_2,rcx));
				main[ addrsize_prfx >> 0xe3_e >> imm8 ] = unary("jecxz",decode_imm,std::bind(jcxz,pls::_1,pls::_2,ecx));
			}

			// Jcc
			std::function<void(uint8_t, std::string const&, amd64::condition)> jcc = [&](uint8_t op, std::string const& suffix, amd64::condition cond)
			{
				main[            op >> imm8            ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));
				main[ 0x0f_e >> (op + 0x10) >> imm_pri ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));

				if(Bits <= 32)
					main[ opsize_prfix >> 0x0f_e >> (op + 0x10) >> imm_alt ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));
			};

			jcc(0x70,"o",Overflow);
			jcc(0x71,"no",NotOverflow);
			jcc(0x72,"c",Carry);
			jcc(0x73,"ae",AboveEqual);
			jcc(0x74,"e",Equal);
			jcc(0x75,"ne",NotEqual);
			jcc(0x76,"be",BelowEqual);
			jcc(0x77,"a",Above);
			jcc(0x78,"s",Sign);
			jcc(0x79,"ns",NotSign);
			jcc(0x7a,"p",Parity);
			jcc(0x7b,"np",NotParity);
			jcc(0x7c,"l",Less);
			jcc(0x7d,"ge",GreaterEqual);
			jcc(0x7e,"le",LessEqual);
			jcc(0x7f,"g",Greater);

			// TODO: LAHF
			// TODO: LDS
			// TODO: LEA
			// TODO: LEAVE
			// TODO: LES
			// TODO: LFS
			// TODO: LGS
			// TODO: LODS
			// TODO: LODSB
			// TODO: LODSW
			// TODO: LODSD
			// TODO: LODSQ
			// TODO: LOOP
			// TODO: LOOPNZ
			// TODO: LOOPNE
			// TODO: LOOPZ
			// TODO: LOOPE
			// TODO: LSS
			// TODO: MOV
			// TODO: MOVBE
			// TODO: MOVS
			// TODO: MOVSB
			// TODO: MOVSW
			// TODO: MOVSD
			// TODO: MOVSQ
			// TODO: MOVSXD
			// TODO: MOVSX
			// TODO: MOVZX
			// TODO: MUL
			// TODO: NEG
			// TODO: NOP
			// OR
			main[ *lock_or_rep >>						0x0c >> imm8				] = binary("or",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x0d >> imm_alt			] = binary("or",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x0d >> imm_pri			] = binary("or",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x81 >> rm_alt_1 >> imm_alt	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x81 >> rm_pri_1 >> imm_pri	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x83 >> rm_alt_1 >> imm8	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>						0x83 >> rm_pri_1 >> imm8	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>						0x08 >> rm8					] = binary("or",decode_mr,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x09 >> rm_alt				] = binary("or",decode_mr,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x09 >> rm_pri				] = binary("or",decode_mr,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x0a >> rm8					] = binary("or",decode_rm,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x0b >> rm_alt				] = binary("or",decode_rm,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> 						0x0b >> rm_pri				] = binary("or",decode_rm,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix	>> 0x0d >> imm32				] = binary("or",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(or_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x80 >> rm8_1 >> imm8	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x81 >> rm64_1 >> imm32	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix	>> 0x83 >> rm64_1 >> imm8	] = binary("or",decode_mi,std::bind(or_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x08 >> rm8					] = binary("or",decode_mr,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x09 >> rm64				] = binary("or",decode_mr,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix		>> 0x0a >> rm8					] = binary("or",decode_rm,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x0b >> rm64				] = binary("or",decode_rm,std::bind(or_,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// TODO: OUT
			// TODO: OUTS
			// TODO: OUTSW
			// TODO: OUTSD
			// TODO: POP
			// TODO: POPA
			// TODO: POPAD
			// TODO: POPCNT
			// TODO: POPF
			// TODO: POPFQ
			// TODO: POPFD
			// TODO: PUSH
			// TODO: PUSHA
			// TODO: PUSHAD
			// TODO: PUSHF
			// TODO: PUSHFD
			// TODO: PUSHFQ
			// TODO: RCL
			// TODO: RCR
			// TODO: RETF
			// TODO: RETN
			// TODO: ROL
			// TODO: ROR
			// TODO: SAHF
			// TODO: SAL
			// TODO: SHL
			// TODO: SALC
			// TODO: SETALC
			// TODO: SAR

			// SBB
			main[ *lock_or_rep >>						0x1c >> imm8				] = binary("sbb",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x1d >> imm_alt			] = binary("sbb",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x1d >> imm_pri			] = binary("sbb",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x80 >> rm8_3 >> imm8	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x81 >> rm_alt_3 >> imm_alt	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x81 >> rm_pri_3 >> imm_pri	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x83 >> rm_alt_3 >> imm8	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>						0x83 >> rm_pri_3 >> imm8	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>						0x18 >> rm8					] = binary("sbb",decode_mr,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x19 >> rm_alt				] = binary("sbb",decode_mr,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x19 >> rm_pri				] = binary("sbb",decode_mr,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x1a >> rm8					] = binary("sbb",decode_rm,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x1b >> rm_alt				] = binary("sbb",decode_rm,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> 						0x1b >> rm_pri				] = binary("sbb",decode_rm,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix	>> 0x1d >> imm32				] = binary("sbb",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(sbb,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x80 >> rm8_3 >> imm8	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x81 >> rm64_3 >> imm32	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix	>> 0x83 >> rm64_3 >> imm8	] = binary("sbb",decode_mi,std::bind(sbb,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x18 >> rm8					] = binary("sbb",decode_mr,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x19 >> rm64				] = binary("sbb",decode_mr,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix		>> 0x1a >> rm8					] = binary("sbb",decode_rm,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x1b >> rm64				] = binary("sbb",decode_rm,std::bind(sbb,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// TODO: SCAS
			// TODO: SCASB
			// TODO: SCASW
			// TODO: SCASD
			// TODO: SCASQ

			// SETcc
			std::function<void(uint8_t, std::string const&, amd64::condition)> setcc = [&](uint8_t op, std::string const& suffix, amd64::condition cond)
			{
				main[ 0x0f_e >> op >> rm8 ] = unary("set" + suffix,decode_rm,std::bind(setcc,pls::_1,pls::_2,cond));

				if(Bits == 64)
					main[ rex_prfix >> 0x0f_e >> op >> rm8 ] = unary("set" + suffix,decode_rm,std::bind(setcc,pls::_1,pls::_2,cond));
			};

			setcc(0x90,"o",Overflow);
			setcc(0x91,"no",NotOverflow);
			setcc(0x92,"c",Carry);
			setcc(0x93,"ae",AboveEqual);
			setcc(0x94,"e",Equal);
			setcc(0x95,"ne",NotEqual);
			setcc(0x96,"be",BelowEqual);
			setcc(0x97,"a",Above);
			setcc(0x98,"s",Sign);
			setcc(0x99,"ns",NotSign);
			setcc(0x9a,"p",Parity);
			setcc(0x9b,"np",NotParity);
			setcc(0x9c,"l",Less);
			setcc(0x9d,"ge",GreaterEqual);
			setcc(0x9e,"le",LessEqual);
			setcc(0x9f,"g",Greater);

			// TODO: SHL
			// TODO: SAL
			// TODO: SHLD
			// TODO: SHR
			// TODO: SHRD
			// TODO: STC
			// TODO: STD
			// TODO: STI
			// TODO: STOS
			// TODO: STOSB
			// TODO: STOSW
			// TODO: STOSD
			// TODO: STOSQ
			// SUB
			main[ *lock_or_rep >>						0x2b >> imm8				] = binary("sub",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x2c >> imm_alt			] = binary("sub",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x2c >> imm_pri			] = binary("sub",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x81 >> rm_alt_5 >> imm_alt	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x81 >> rm_pri_5 >> imm_pri	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x83 >> rm_alt_5 >> imm8	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>						0x83 >> rm_pri_5 >> imm8	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>						0x28 >> rm8					] = binary("sub",decode_mr,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x29 >> rm_alt				] = binary("sub",decode_mr,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x29 >> rm_pri				] = binary("sub",decode_mr,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x2a >> rm8					] = binary("sub",decode_rm,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x2b >> rm_alt				] = binary("sub",decode_rm,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> 						0x2b >> rm_pri				] = binary("sub",decode_rm,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix	>> 0x2c >> imm32				] = binary("sub",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(sub,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x80 >> rm8_5 >> imm8	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x81 >> rm64_5 >> imm32	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix	>> 0x83 >> rm64_5 >> imm8	] = binary("sub",decode_mi,std::bind(sub,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x28 >> rm8					] = binary("sub",decode_mr,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x29 >> rm64				] = binary("sub",decode_mr,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix		>> 0x2a >> rm8					] = binary("sub",decode_rm,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x2b >> rm64				] = binary("sub",decode_rm,std::bind(sub,pls::_1,pls::_2,pls::_3,boost::none));
			}

			// TODO: TEST
			// TODO: UD
			// TODO: US2
			// TODO: XADD
			// TODO: XCHG
			// XOR
			main[ *lock_or_rep >>						0x34 >> imm8				] = binary("xor",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																													  std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x35 >> imm_alt			] = binary("xor",std::bind(decode_i,opsz_alt,pls::_1,pls::_2),
																													  std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x35 >> imm_pri			] = binary("xor",std::bind(decode_i,opsz_pri,pls::_1,pls::_2),
																													  std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x81 >> rm_alt_6 >> imm_alt	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x81 >> rm_pri_6 >> imm_pri	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x83 >> rm_alt_6 >> imm8	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,alt)));
			main[ *lock_or_rep >>						0x83 >> rm_pri_6 >> imm8	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,pri)));
			main[ *lock_or_rep >>						0x30 >> rm8					] = binary("xor",decode_mr,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x31 >> rm_alt				] = binary("xor",decode_mr,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x31 >> rm_pri				] = binary("xor",decode_mr,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >>						0x32 >> rm8					] = binary("xor",decode_rm,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> opsize_prfix	>> 0x33 >> rm_alt				] = binary("xor",decode_rm,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			main[ *lock_or_rep >> 						0x33 >> rm_pri				] = binary("xor",decode_rm,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));

			if(Bits == 64)
			{
				main[ *lock_or_rep >> rexw_prfix	>> 0x34 >> imm32				] = binary("xor",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																														  std::bind(xor_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x80 >> rm8_6 >> imm8	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x81 >> rm64_6 >> imm32	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));
				main[ *lock_or_rep >> rexw_prfix	>> 0x83 >> rm64_6 >> imm8	] = binary("xor",decode_mi,std::bind(xor_,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));
				main[ *lock_or_rep >> rex_prfix		>> 0x30 >> rm8					] = binary("xor",decode_mr,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x31 >> rm64				] = binary("xor",decode_mr,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rex_prfix		>> 0x32 >> rm8					] = binary("xor",decode_rm,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
				main[ *lock_or_rep >> rexw_prfix	>> 0x33 >> rm64				] = binary("xor",decode_rm,std::bind(xor_,pls::_1,pls::_2,pls::_3,boost::none));
			}
		}
		/*
### AAA
###  - ASCII Adjust After Addition
		val / [0x37] | mode32? = arity0 none AAA

### AAD
###  - ASCII Adjust AX Before Division
		val / [0xd5] | mode32? = unop none AAD imm8

### AAM
###  - ASCII Adjust AX After Multiply
		val / [0xd4] | mode32? = unop none AAM imm8

### AAS
###  - ASCII Adjust AL After Subtraction
		val / [0x3f] | mode32? = arity0 none AAS

### ADC
###  - Add with Carry
		val / [0x14] = binop none ADC al imm8
		val / [0x15]
		 | rexw? = binop none ADC rax imm32
		 | opndsz? = binop none ADC ax imm16
		 | otherwise = binop none ADC eax imm32
		val / [0x80 /2] = binop-lock none ADC r/m8 imm8
		val / [0x81 /2]
		 | rexw? = binop-lock none ADC r/m64 imm32
		 | opndsz? = binop-lock none ADC r/m16 imm16
		 | otherwise = binop-lock none ADC r/m32 imm32
		val / [0x83 /2]
		 | rexw? = binop-lock none ADC r/m64 imm8
		 | opndsz? = binop-lock none ADC r/m16 imm8
		 | otherwise = binop-lock none ADC r/m32 imm8
		val / [0x10 /r] = binop-lock none ADC r/m8 r8
		val / [0x11 /r]
		 | rexw? = binop-lock none ADC r/m64 r64
		 | opndsz? = binop-lock none ADC r/m16 r16
		 | otherwise = binop-lock none ADC r/m32 r32
		val / [0x12 /r] = binop none ADC r8 r/m8
		val / [0x13 /r]
		 | rexw? = binop none ADC r64 r/m64
		 | opndsz? = binop none ADC r16 r/m16
		 | otherwise = binop none ADC r32 r/m32

### ADD
###  - Add
		val / [0x04] = binop none ADD al imm8
		val / [0x05]
		 | rexw? = binop none ADD rax imm32
		 | opndsz? = binop none ADD ax imm16
		 | otherwise = binop none ADD eax imm32
		val / [0x80 /0] = binop-lock none ADD r/m8 imm8
		val / [0x81 /0]
		 | rexw? = binop-lock none ADD r/m64 imm32
		 | opndsz? = binop-lock none ADD r/m16 imm16
		 | otherwise = binop-lock none ADD r/m32 imm32
		val / [0x83 /0]
		 | rexw? = binop-lock none ADD r/m64 imm8
		 | opndsz? = binop-lock none ADD r/m16 imm8
		 | otherwise = binop-lock none ADD r/m32 imm8
		val / [0x00 /r] = binop-lock none ADD r/m8 r8
		val / [0x01 /r]
		 | rexw? = binop-lock none ADD r/m64 r64
		 | opndsz? = binop-lock none ADD r/m16 r16
		 | otherwise = binop-lock none ADD r/m32 r32
		val / [0x02 /r] = binop none ADD r8 r/m8
		val / [0x03 /r]
		 | rexw? = binop none ADD r64 r/m64
		 | opndsz? = binop none ADD r16 r/m16
		 | otherwise = binop none ADD r32 r/m32

### ADDPD
###  - Add Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x58 /r] = binop sse2 ADDPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x58 /r]
		 | vex128? = varity3 avx VADDPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VADDPD ymm256 v/ymm ymm/m256

### ADDPS
###  - Add Packed Single-Precision Floating-Point Values
		val / [0x0f 0x58 /r] = binop sse ADDPS xmm128 xmm/m128
		val /vex/0f/vexv [0x58 /r]
		 | vex128? = varity3 avx VADDPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VADDPS ymm256 v/ymm ymm/m256

### ADDSD
###  - Add Scalar Double-Precision Floating-Point Values
		val /f2 [0x0f 0x58 /r] = binop sse2 ADDSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x58 /r] = varity3 avx VADDSD xmm128 v/xmm xmm/m64

### ADDSS
###  - Add Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0x58 /r] = binop sse ADDSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x58 /r] = varity3 avx VADDSS xmm128 v/xmm xmm/m32

### ADDSUBPD
###  - Packed Double-FP Add/Subtract
		val /66 [0x0f 0xd0 /r] = binop sse3 ADDSUBPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0xd0 /r]
		 | vex128? = varity3 avx VADDSUBPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VADDSUBPD ymm256 v/ymm ymm/m256

### ADDSUBPS
###  - Packed Single-FP Add/Subtract
		val /f2 [0x0f 0xd0 /r] = binop sse3 ADDSUBPS xmm128 xmm/m128
		val /vex/f2/0f/vexv [0xd0 /r]
		 | vex128? = varity3 avx VADDSUBPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VADDSUBPS ymm256 v/ymm ymm/m256

### AESDEC
###  - Perform One Round of an AES Decryption Flow
		val /66 [0x0f 0x38 0xde /r] = binop aes AESDEC xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0xde /r] | vex128? = varity3 (aes or avx) VAESDEC xmm128 v/xmm xmm/m128

### AESDECLAST
###  - Perform Last Round of an AES Decryption Flow
		val /66 [0x0f 0x38 0xdf /r] = binop aes AESDECLAST xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0xdf /r] | vex128? = varity3 (aes or avx) VAESDECLAST xmm128 v/xmm xmm/m128

### AESENC
###  - Perform One Round of an AES Encryption Flow
		val /66 [0x0f 0x38 0xdc /r] = binop aes AESENC xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0xdc /r] | vex128? = varity3 (aes or avx) VAESENC xmm128 v/xmm xmm/m128

### AESENCLAST
###  - Perform Last Round of an AES Encryption Flow
		val /66 [0x0f 0x38 0xdd /r] = binop aes AESENCLAST xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0xdd /r] | vex128? = varity3 (aes or avx) VAESENCLAST xmm128 v/xmm xmm/m128

### AESIMC
###  - Perform the AES InvMixColumn Transformation
		val /66 [0x0f 0x38 0xdb /r] = binop aes AESIMC xmm128 xmm/m128
		val /vex/66/0f/38 [0xdb /r] | vex128? = varity2 (aes or avx) VAESIMC xmm128 xmm/m128

### AESKEYGENASSIST
###  - AES Round Key Generation Assist
		val /66 [0x0f 0x3a 0xdf /r] = ternop aes AESKEYGENASSIST xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0xdf /r] | vex128? = varity3 (aes or avx) VAESKEYGENASSIST xmm128 xmm/m128 imm8

### AND
###  - Logical AND
		val / [0x24] = binop none AND al imm8
		val / [0x25]
		 | rexw? = binop none AND rax imm32
		 | opndsz? = binop none AND ax imm16
		 | otherwise = binop none AND eax imm32
		val / [0x80 /4] = binop-lock none AND r/m8 imm8
		val / [0x81 /4]
		 | rexw? = binop-lock none AND r/m64 imm32
		 | opndsz? = binop-lock none AND r/m16 imm16
		 | otherwise = binop-lock none AND r/m32 imm32
		val / [0x83 /4]
		 | rexw? = binop-lock none AND r/m64 imm8
		 | opndsz? = binop-lock none AND r/m16 imm8
		 | otherwise = binop-lock none AND r/m32 imm8
		val / [0x20 /r] = binop-lock none AND r/m8 r8
		val / [0x21 /r]
		 | rexw? = binop-lock none AND r/m64 r64
		 | opndsz? = binop-lock none AND r/m16 r16
		 | otherwise = binop-lock none AND r/m32 r32
		val / [0x22 /r] = binop none AND r8 r/m8
		val / [0x23 /r]
		 | rexw? = binop none AND r64 r/m64
		 | opndsz? = binop none AND r16 r/m16
		 | otherwise = binop none AND r32 r/m32

### ANDPD
###  - Bitwise Logical AND of Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x54 /r] = binop sse2 ANDPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x54 /r]
		 | vex128? = varity3 avx VANDPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VANDPD ymm256 v/ymm ymm/m256

### ANDPS
###  - Bitwise Logical AND of Packed Single-Precision Floating-Point Values
		val / [0x0f 0x54 /r] = binop sse ANDPS xmm128 xmm/m128
		val /vex/0f/vexv [0x54 /r]
		 | vex128? = varity3 avx VANDPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VANDPS ymm256 v/ymm ymm/m256

### ANDNPD
###  - Bitwise Logical AND NOT of Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x55 /r] = binop sse2 ANDNPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x55 /r]
		 | vex128? = varity3 avx VANDNPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VANDNPD ymm256 v/ymm ymm/m256

### ANDNPS
###  - Bitwise Logical AND NOT of Packed Single-Precision Floating-Point Values
		val / [0x0f 0x55 /r] = binop sse ANDNPS xmm128 xmm/m128
		val /vex/0f/vexv [0x55 /r]
		 | vex128? = varity3 avx VANDNPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VANDNPS ymm256 v/ymm ymm/m256

### ARPL
###  - Adjust RPL Field of Segment Selector
### See MOVSX/MOVSDX

### BLENDPD
###  - Blend Packed Double Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x0d /r] = ternop sse4_1 BLENDPD xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x0d /r]
		 | vex128? = varity4 avx VBLENDPD xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VBLENDPD ymm256 v/ymm ymm/m256 imm8

### BLENDPS
###  - Blend Packed Single Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x0c /r] = ternop sse4_1 BLENDPS xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x0c /r]
		 | vex128? = varity4 avx VBLENDPS xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VBLENDPS ymm256 v/ymm ymm/m256 imm8

### BLENDVPD
###  - Variable Blend Packed Double Precision Floating-Point Values
		val /66 [0x0f 0x38 0x15 /r] = ternop sse4_1 BLENDVPD xmm128 xmm/m128 xmm0
		val /vex/66/0f/3a/vexv [0x4b /r]
		 | vex128? && vexw0? = varity4 avx VBLENDVPD xmm128 v/xmm xmm/m128 imm/xmm
		 | vex256? && vexw0? = varity4 avx VBLENDVPD ymm256 v/ymm ymm/m256 imm/ymm

### BLENDVPS
###  - Variable Blend Packed Single Precision Floating-Point Values
		val /66 [0x0f 0x38 0x14 /r] = ternop sse4_1 BLENDVPS xmm128 xmm/m128 xmm0
		val /vex/66/0f/3a/vexv [0x4a /r]
		 | vex128? && vexw0? = varity4 avx VBLENDVPS xmm128 v/xmm xmm/m128 imm/xmm
		 | vex256? && vexw0? = varity4 avx VBLENDVPS ymm256 v/ymm ymm/m256 imm/ymm

### BOUND
###  - Check Array Index Against Bounds
		val / [0x62 /r-mem]
		 | opndsz? && mode32? = binop none BOUND r16 m16/16
		 | mode32? = binop none BOUND r32 m32/32

### BSF
###  - Bit Scan Forward
		val / [0x0f 0xbc /r]
		 | rexw? = binop none BSF r64 r/m64
		 | opndsz? = binop none BSF r16 r/m16
		 | otherwise = binop none BSF r32 r/m32

### BSR
###  - Bit Scan Reverse
		val / [0x0f 0xbd /r]
		 | rexw? = binop none BSR r64 r/m64
		 | opndsz? = binop none BSR r16 r/m16
		 | otherwise = binop none BSR r32 r/m32

### BSWAP
###  - Byte Swap
		val / [0x0f /1-reg]
		 | rexw? = unop none BSWAP r/reg64
		 | opndsz? = unop none BSWAP r/reg16
		 | otherwise = unop none BSWAP r/reg32
#val / [0x0f '11001 r:3']
# | rexw? = do update@{reg/opcode=r}; unop none BSWAP r64/rexb end
# | otherwise = do update@{reg/opcode=r}; unop none BSWAP r32/rexb end

### BT
###  - Bit Test
		val / [0x0f 0xa3 /r]
		 | rexw? = binop none BT r/m64 r64
		 | opndsz? = binop none BT r/m16 r16
		 | otherwise = binop none BT r/m32 r32
		val / [0x0f 0xba /4]
		 | rexw? = binop none BT r/m64 imm8
		 | opndsz? = binop none BT r/m16 imm8
		 | otherwise = binop none BT r/m32 imm8

### BTC
###  - Bit Test and Complement
		val / [0x0f 0xbb /r]
		 | rexw? = binop-lock none BTC r/m64 r64
		 | opndsz? = binop-lock none BTC r/m16 r16
		 | otherwise = binop-lock none BTC r/m32 r32
		val / [0x0f 0xba /7]
		 | rexw? = binop-lock none BTC r/m64 imm8
		 | opndsz? = binop-lock none BTC r/m16 imm8
		 | otherwise = binop-lock none BTC r/m32 imm8

### BTR
###  - Bit Test and Reset
		val / [0x0f 0xb3 /r]
		 | rexw? = binop-lock none BTR r/m64 r64
		 | opndsz? = binop-lock none BTR r/m16 r16
		 | otherwise = binop-lock none BTR r/m32 r32
		val / [0x0f 0xba /6]
		 | rexw? = binop-lock none BTR r/m64 imm8
		 | opndsz? = binop-lock none BTR r/m16 imm8
		 | otherwise = binop-lock none BTR r/m32 imm8

### BTS
###  - Bit Test and Set
		val / [0x0f 0xab /r]
		 | rexw? = binop-lock none BTS r/m64 r64
		 | opndsz? = binop-lock none BTS r/m16 r16
		 | otherwise = binop-lock none BTS r/m32 r32
		val / [0x0f 0xba /5]
		 | rexw? = binop-lock none BTS r/m64 imm8
		 | opndsz? = binop-lock none BTS r/m16 imm8
		 | otherwise = binop-lock none BTS r/m32 imm8

### CALL
###  - Call Procedure
		val / [0xe8]
		 | opndsz? = near-rel none CALL rel16
		 | otherwise = near-rel none CALL rel32
		val / [0xff /2]
		 | mode64? = near-abs none CALL r/m64
		 | opndsz? = near-abs none CALL r/m16
		 | otherwise = near-abs none CALL r/m32
		val / [0x9a]
		 | opndsz? = far-dir none CALL ptr16/16
		 | otherwise = far-dir none CALL ptr16/32
		val / [0xff /3-mem]
		 | rexw? = far-ind none CALL m16/64
		 | opndsz? = far-ind none CALL m16/16
		 | otherwise = far-ind none CALL m16/32

### CBW/CWDE/CDQE
###  - Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword to Quadword
		val / [0x98]
		 | rexw? = arity0 none CDQE
		 | opndsz? = arity0 none CBW
		 | otherwise = arity0 none CWDE

### CLC
###  - Clear Carry Flag
		val / [0xf8] = arity0 none CLC

### CLD
###  - Clear Direction Flag
		val / [0xfc] = arity0 none CLD

### CLFLUSH
###  - Flush Cache Line
		val / [0x0f 0xae /7-mem] = unop none CLFLUSH m8

### CLI
###  - Clear Interrupt Flag
		val / [0xfa] = arity0 none CLI

### CLTS
###  - Clear Task-Switched Flag in CR0
		val / [0x0f 0x06] = arity0 none CLTS

### CMC
###  - Complement Carry Flag
		val / [0xf5] = arity0 none CMC

### CMOVcc
###  - Conditional Move
		val / [0x0f 0x47 /r] # CMOVNBE
		 | rexw? = binop none CMOVA r64 r/m64
		 | opndsz? = binop none CMOVA r16 r/m16
		 | otherwise = binop none CMOVA r32 r/m32
		val / [0x0f 0x43 /r] # CMOVNB, CMOVNC
		 | rexw? = binop none CMOVAE r64 r/m64
		 | opndsz? = binop none CMOVAE r16 r/m16
		 | otherwise = binop none CMOVAE r32 r/m32
		val / [0x0f 0x42 /r] # CMOVC, CMOVNAE
		 | rexw? = binop none CMOVB r64 r/m64
		 | opndsz? = binop none CMOVB r16 r/m16
		 | otherwise = binop none CMOVB r32 r/m32
		val / [0x0f 0x46 /r] # CMOVNA
		 | rexw? = binop none CMOVBE r64 r/m64
		 | opndsz? = binop none CMOVBE r16 r/m16
		 | otherwise = binop none CMOVBE r32 r/m32
		val / [0x0f 0x44 /r] # CMOVZ
		 | rexw? = binop none CMOVE r64 r/m64
		 | opndsz? = binop none CMOVE r16 r/m16
		 | otherwise = binop none CMOVE r32 r/m32
		val / [0x0f 0x4f /r] # CMOVNLE
		 | rexw? = binop none CMOVG r64 r/m64
		 | opndsz? = binop none CMOVG r16 r/m16
		 | otherwise = binop none CMOVG r32 r/m32
		val / [0x0f 0x4d /r] # CMOVNL
		 | rexw? = binop none CMOVGE r64 r/m64
		 | opndsz? = binop none CMOVGE r16 r/m16
		 | otherwise = binop none CMOVGE r32 r/m32
		val / [0x0f 0x4c /r] # CMOVNGE
		 | rexw? = binop none CMOVL r64 r/m64
		 | opndsz? = binop none CMOVL r16 r/m16
		 | otherwise = binop none CMOVL r32 r/m32
		val / [0x0f 0x4e /r] # CMOVNG
		 | rexw? = binop none CMOVLE r64 r/m64
		 | opndsz? = binop none CMOVLE r16 r/m16
		 | otherwise = binop none CMOVLE r32 r/m32
		val / [0x0f 0x45 /r] # CMOVNZ
		 | rexw? = binop none CMOVNE r64 r/m64
		 | opndsz? = binop none CMOVNE r16 r/m16
		 | otherwise = binop none CMOVNE r32 r/m32
		val / [0x0f 0x41 /r]
		 | rexw? = binop none CMOVNO r64 r/m64
		 | opndsz? = binop none CMOVNO r16 r/m16
		 | otherwise = binop none CMOVNO r32 r/m32
		val / [0x0f 0x4b /r] # CMOVPO
		 | rexw? = binop none CMOVNP r64 r/m64
		 | opndsz? = binop none CMOVNP r16 r/m16
		 | otherwise = binop none CMOVNP r32 r/m32
		val / [0x0f 0x49 /r]
		 | rexw? = binop none CMOVNS r64 r/m64
		 | opndsz? = binop none CMOVNS r16 r/m16
		 | otherwise = binop none CMOVNS r32 r/m32
		val / [0x0f 0x40 /r]
		 | rexw? = binop none CMOVO r64 r/m64
		 | opndsz? = binop none CMOVO r16 r/m16
		 | otherwise = binop none CMOVO r32 r/m32
		val / [0x0f 0x4a /r] # CMOVPE
		 | rexw? = binop none CMOVP r64 r/m64
		 | opndsz? = binop none CMOVP r16 r/m16
		 | otherwise = binop none CMOVP r32 r/m32
		val / [0x0f 0x48 /r]
		 | rexw? = binop none CMOVS r64 r/m64
		 | opndsz? = binop none CMOVS r16 r/m16
		 | otherwise = binop none CMOVS r32 r/m32

### CMP
###  - Compare Two Operands
		val / [0x3c] = binop none CMP al imm8
		val / [0x3d]
		 | rexw? = binop none CMP rax imm32
		 | opndsz? = binop none CMP ax imm16
		 | otherwise = binop none CMP eax imm32
		val / [0x80 /7] = binop none CMP r/m8 imm8
		val / [0x81 /7]
		 | rexw? = binop none CMP r/m64 imm32
		 | opndsz? = binop none CMP r/m16 imm16
		 | otherwise = binop none CMP r/m32 imm32
		val / [0x83 /7]
		 | rexw? = binop none CMP r/m64 imm8
		 | opndsz? = binop none CMP r/m16 imm8
		 | otherwise = binop none CMP r/m32 imm8
		val / [0x38 /r] = binop none CMP r/m8 r8
		val / [0x39 /r]
		 | rexw? = binop none CMP r/m64 r64
		 | opndsz? = binop none CMP r/m16 r16
		 | otherwise = binop none CMP r/m32 r32
		val / [0x3a /r] = binop none CMP r8 r/m8
		val / [0x3b /r]
		 | rexw? = binop none CMP r64 r/m64
		 | opndsz? = binop none CMP r16 r/m16
		 | otherwise = binop none CMP r32 r/m32

### CMPPD
###  - Compare Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0xc2 /r] = ternop sse2 CMPPD xmm128 xmm/m128 imm8
		val /vex/66/0f/vexv [0xc2 /r]
		 | vex128? = varity4 avx VCMPPD xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VCMPPD ymm256 v/ymm ymm/m256 imm8

### CMPPS
###  - Compare Packed Single-Precision Floating-Point Values
		val / [0x0f 0xc2 /r] = ternop sse CMPPS xmm128 xmm/m128 imm8
		val /vex/0f/vexv [0xc2 /r]
		 | vex128? = varity4 avx VCMPPS xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VCMPPS ymm256 v/ymm ymm/m256 imm8

### CMPS/CMPSB/CMPSW/CMPSD/CMPSQ
###  - Compare String Operands
		val / [0xa6] = binop-rep-repne none CMPS (m/default/si/esi/rsi (return 8)) (m/es/di/edi/rdi (return 8))
		val / [0xa7]
		 | rexw? = binop-rep-repne none CMPS (m/default/si/esi/rsi operand-size) (m/es/di/edi/rdi operand-size) 
		 | opndsz? = binop-rep-repne none CMPS (m/default/si/esi/rsi operand-size) (m/es/di/edi/rdi operand-size)
		 | otherwise = binop-rep-repne none CMPS (m/default/si/esi/rsi operand-size) (m/es/di/edi/rdi operand-size)

### CMPSD
###  - Compare Scalar Double-Precision Floating-Point Values
		val /f2 [0x0f 0xc2 /r] = ternop sse2 CMPSD xmm128 xmm/m64 imm8
		val /vex/f2/0f/vexv [0xc2 /r] = varity4 avx VCMPSD xmm128 v/xmm xmm/m64 imm8

### CMPSS
###  - Compare Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0xc2 /r] = ternop sse CMPSS xmm128 xmm/m32 imm8
		val /vex/f3/0f/vexv [0xc2 /r] = varity4 avx VCMPSS xmm128 v/xmm xmm/m32 imm8

### CMPXCHG
###  - Compare and Exchange
		val / [0x0f 0xb0 /r] = binop-lock none CMPXCHG r/m8 r8
		val / [0x0f 0xb1 /r]
		 | rexw? = binop-lock none CMPXCHG r/m64 r64
		 | opndsz? = binop-lock none CMPXCHG r/m16 r16
		 | otherwise = binop-lock none CMPXCHG r/m32 r32

### CMPXCHG8B/CMPXCHG16B
###  - Compare and Exchange Bytes
		val / [0x0f 0xc7 /1-mem]
		 | rexw? = unop-lock none CMPXCHG16B m128
		 | otherwise = unop-lock none CMPXCHG8B m64

### COMISD
###  - Compare Scalar Ordered Double-Precision Floating-Point Values and Set EFLAGS
		val /66 [0x0f 0x2f /r] = binop sse2 COMISD xmm128 xmm/m64
		val /vex/66/0f [0x2f /r] = varity2 avx VCOMISD xmm128 xmm/m64

### COMISS
###  - Compare Scalar Ordered Single-Precision Floating-Point Values and Set EFLAGS
		val / [0x0f 0x2f /r] = binop sse COMISS xmm128 xmm/m32
		val /vex/0f [0x2f /r] = varity2 avx VCOMISS xmm128 xmm/m32

### CPUID
###  - CPU Identification
		val / [0x0f 0xa2] = arity0 none CPUID

### CRC32
###  - Accumulate CRC32 Value
		val /f2 [0x0f 0x38 0xf0 /r]
		 | rexw? = binop none CRC32 r64 r/m8
		 | otherwise = binop none CRC32 r32 r/m8
		val /f2 [0x0f 0x38 0xf1 /r]
		 | rexw? = binop none CRC32 r64 r/m64
		 | opndsz? = binop none CRC32 r32 r/m16
		 | otherwise = binop none CRC32 r32 r/m32

### CVTDQ2PD
###  - Convert Packed Dword Integers to Packed Double-Precision FP Values
		val /f3 [0x0f 0xe6 /r] = binop sse2 CVTDQ2PD xmm128 xmm/m64 # bug in Intel manual: /r is missing
		val /vex/f3/0f [0xe6 /r]
		 | vex128? = varity2 avx VCVTDQ2PD xmm128 xmm/m64
		 | vex256? = varity2 avx VCVTDQ2PD ymm256 ymm/m128

### CVTDQ2PS
###  - Convert Packed Dword Integers to Packed Single-Precision FP Values
		val / [0x0f 0x5b /r] = binop sse2 CVTDQ2PS xmm128 xmm/m128
		val /vex/0f [0x5b /r]
		 | vex128? = varity2 avx VCVTDQ2PS xmm128 xmm/m128
		 | vex256? = varity2 avx VCVTDQ2PS ymm256 ymm/m256

### CVTPD2DQ
###  - Convert Packed Double-Precision FP Values to Packed Dword Integers
		val /f2 [0x0f 0xe6 /r] = binop sse2 CVTPD2DQ xmm128 xmm/m128 # bug in Intel manual: /r is missing
		val /vex/f2/0f [0xe6 /r]
		 | vex128? = varity2 avx VCVTPD2DQ xmm128 xmm/m128
		 | vex256? = varity2 avx VCVTPD2DQ xmm128 ymm/m256

### CVTPD2PI
###  - Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers
		val /66 [0x0f 0x2d /r] = binop none CVTPD2PI mm64 xmm/m128

### CVTPD2PS
###  - Convert Packed Double-Precision FP Values to Packed Single-Precision FP Values
		val /66 [0x0f 0x5a /r] = binop sse2 CVTPD2PS xmm128 xmm/m128
		val /vex/66/0f [0x5a /r]
		 | vex128? = varity2 avx VCVTPD2PS xmm128 xmm/m128
		 | vex256? = varity2 avx VCVTPD2PS xmm128 ymm/m256

### CVTPI2PD
###  - Convert Packed Dword Integers to Packed Double-Precision FP Values
		val /66 [0x0f 0x2a /r] = binop none CVTPI2PD xmm128 mm/m64

### CVTPI2PS
###  - Convert Packed Dword Integers to Packed Single-Precision FP Values
		val / [0x0f 0x2a /r] = binop none CVTPI2PS xmm128 mm/m64

### CVTPS2DQ
###  - Convert Packed Single-Precision FP Values to Packed Dword Integers
		val /66 [0x0f 0x5b /r] = binop sse2 CVTPS2DQ xmm128 xmm/m128
		val /vex/66/0f [0x5b /r]
		 | vex128? = varity2 avx VCVTPS2DQ xmm128 xmm/m128
		 | vex256? = varity2 avx VCVTPS2DQ ymm256 ymm/m256

### CVTPS2PD
###  - Convert Packed Single-Precision FP Values to Packed Double-Precision FP Values
		val / [0x0f 0x5a /r] = binop sse2 CVTPS2PD xmm128 xmm/m64
		val /vex/0f [0x5a /r]
		 | vex128? = varity2 avx VCVTPS2PD xmm128 xmm/m64
		 | vex256? = varity2 avx VCVTPS2PD ymm256 xmm/m128

### CVTPS2PI
###  - Convert Packed Single-Precision FP Values to Packed Dword Integers
		val / [0x0f 0x2d /r] = binop none CVTPS2PI mm64 xmm/m64

### CVTSD2SI
###  - Convert Scalar Double-Precision FP Value to Integer
		val /f2 [0x0f 0x2d /r]
		 | rexw? = binop sse2 CVTSD2SI r64 xmm/m64
		 | otherwise = binop sse2 CVTSD2SI r32 xmm/m64
		val /vex/f2/0f [0x2d /r]
		 | vexw0? = varity2 avx VCVTSD2SI r32 xmm/m64
		 | vexw1? && mode64? = varity2 avx VCVTSD2SI r64 xmm/m64

### CVTSD2SS
###  - Convert Scalar Double-Precision FP Value to Scalar Single-Precision FP Value
		val /f2 [0x0f 0x5a /r] = binop sse2 CVTSD2SS xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x5a /r] = varity3 avx VCVTSD2SS xmm128 v/xmm xmm/m64

### CVTSI2SD
###  - Convert Dword Integer to Scalar Double-Precision FP Value
		val /f2 [0x0f 0x2a /r]
		 | rexw? = binop sse2 CVTSI2SD xmm128 r/m64
		 | otherwise = binop sse2 CVTSI2SD xmm128 r/m32
		val /vex/f2/0f/vexv [0x2a /r]
		 | vexw0? = varity3 avx VCVTSI2SD xmm128 v/xmm r/m32
		 | vexw1? && mode64? = varity3 avx VCVTSI2SD xmm128 v/xmm r/m64

### CVTSI2SS
###  - Convert Dword Integer to Scalar Single-Precision FP Value
		val /f3 [0x0f 0x2a /r]
		 | rexw? = binop sse CVTSI2SS xmm128 r/m64
		 | otherwise = binop sse CVTSI2SS xmm128 r/m32
		val /vex/f3/0f/vexv [0x2a /r]
		 | vexw0? = varity3 avx VCVTSI2SS xmm128 v/xmm r/m32
		 | vexw1? && mode64? = varity3 avx VCVTSI2SS xmm128 v/xmm r/m64

### CVTSS2SD
###  - Convert Scalar Single-Precision FP Value to Scalar Double-Precision FP Value
		val /f3 [0x0f 0x5a /r] = binop sse2 CVTSS2SD xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x5a /r] = varity3 avx VCVTSS2SD xmm128 v/xmm xmm/m32

### CVTSS2SI
###  - Convert Scalar Single-Precision FP Value to Dword Integer
		val /f3 [0x0f 0x2d /r]
		 | rexw? = binop sse CVTSS2SI r64 xmm/m32
		 | otherwise = binop sse CVTSS2SI r32 xmm/m32
		val /vex/f3/0f [0x2d /r]
		 | vexw0? = varity2 avx VCVTSS2SI r32 xmm/m32
		 | vexw1? && mode64? = varity2 avx VCVTSS2SI r64 xmm/m32

### CVTTPD2DQ
###  - Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers
		val /66 [0x0f 0xe6 /r] = binop sse2 CVTTPD2DQ xmm128 xmm/m128
		val /vex/66/0f [0xe6 /r]
		 | vex128? = varity2 avx VCVTTPD2DQ xmm128 xmm/m128
		 | vex256? = varity2 avx VCVTTPD2DQ xmm128 ymm/m256

### CVTTPD2PI
###  - Convert with Truncation Packed Double-Precision FP Values to Packed Dword Integers
		val /66 [0x0f 0x2c /r] = binop none CVTTPD2PI mm64 xmm/m128

### CVTTPS2DQ
###  - Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers
		val /f3 [0x0f 0x5b /r] = binop sse2 CVTTPS2DQ xmm128 xmm/m128
		val /vex/f3/0f [0x5b /r]
		 | vex128? = varity2 avx VCVTTPS2DQ xmm128 xmm/m128
		 | vex256? = varity2 avx VCVTTPS2DQ ymm256 ymm/m256

### CVTTPS2PI
###  - Convert with Truncation Packed Single-Precision FP Values to Packed Dword Integers
		val / [0x0f 0x2c /r] = binop none CVTTPS2PI mm64 xmm/m64

### CVTTSD2SI
###  - Convert with Truncation Scalar Double-Precision FP Value to Signed Integer
		val /f2 [0x0f 0x2c /r]
		 | rexw? = binop sse2 CVTTSD2SI r64 xmm/m64
		 | otherwise = binop sse2 CVTTSD2SI r32 xmm/m64
		val /vex/f2/0f [0x2c /r]
		 | vexw0? = varity2 avx VCVTTSD2SI r32 xmm/m64
		 | vexw1? && mode64? = varity2 avx VCVTTSD2SI r64 xmm/m64

### CVTTSS2SI
###  - Convert with Truncation Scalar Single-Precision FP Value to Dword Integer
		val /f3 [0x0f 0x2c /r]
		 | rexw? = binop sse CVTTSS2SI r64 xmm/m32
		 | otherwise = binop sse CVTTSS2SI r32 xmm/m32
		val /vex/f3/0f [0x2c /r]
		 | vexw0? = varity2 avx VCVTTSS2SI r32 xmm/m32
		 | vexw1? && mode64? = varity2 avx VCVTTSS2SI r64 xmm/m32

### CWD/CDQ/CQO
###  - Convert Word to Doubleword/Convert Doubleword to Quadword
		val / [0x99]
		 | rexw? = arity0 none CQO
		 | opndsz? = arity0 none CWD
		 | otherwise = arity0 none CDQ

### DAA
###  - Decimal Adjust AL after Addition
		val / [0x27] | mode32? = arity0 none DAA

### DAS
###  - Decimal Adjust AL after Subtraction
		val / [0x2f] | mode32? = arity0 none DAS

### DEC
###  - Decrement by 1
		val / [0xfe /1] = unop-lock none DEC r/m8
		val / [0xff /1]
		 | rexw? = unop-lock none DEC r/m64
		 | opndsz? = unop-lock none DEC r/m16
		 | otherwise = unop-lock none DEC r/m32
		val / ['01001 r:3']
		 | opndsz? && mode32? = do update@{reg/opcode=r}; unop-lock none DEC r16 end
		 | mode32? = do update@{reg/opcode=r}; unop-lock none DEC r32 end

### DIV
###  - Unsigned Divide
		val / [0xf6 /6] = unop none DIV r/m8
		val / [0xf7 /6]
		 | rexw? = unop none DIV r/m64
		 | opndsz? = unop none DIV r/m16
		 | otherwise = unop none DIV r/m32

### DIVPD
###  - Divide Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x5e /r] = binop sse2 DIVPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x5e /r]
		 | vex128? = varity3 avx VDIVPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VDIVPD ymm256 v/ymm ymm/m256

### DIVPS
###  - Divide Packed Single-Precision Floating-Point Values
		val / [0x0f 0x5e /r] = binop sse DIVPS xmm128 xmm/m128
		val /vex/0f/vexv [0x5e /r]
		 | vex128? = varity3 avx VDIVPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VDIVPS ymm256 v/ymm ymm/m256

### DIVSD
###  - Divide Scalar Double-Precision Floating-Point Values
		val /f2 [0x0f 0x5e /r] = binop sse2 DIVSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x5e /r] = varity3 avx VDIVSD xmm128 v/xmm xmm/m64

### DIVSS
###  - Divide Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0x5e /r] = binop sse DIVSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x5e /r] = varity3 avx VDIVSS xmm128 v/xmm xmm/m32

### DPPD
###  - Dot Product of Packed Double Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x41 /r] = ternop sse4_1 DPPD xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x41 /r] | vex128? = varity4 avx VDPPD xmm128 v/xmm xmm/m128 imm8

### DPPS
###  - Dot Product of Packed Single Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x40 /r] = ternop sse4_1 DPPS xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x40 /r]
		 | vex128? = varity4 avx VDPPS xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VDPPS ymm256 v/ymm ymm/m256 imm8

### EMMS
###  - Empty MMX Technology State
		val / [0x0f 0x77] = arity0 none EMMS

### ENTER
###  - Make Stack Frame for Procedure Parameters
		val / [0xc8] = binop none ENTER imm16 imm8

### EXTRACTPS
###  - Extract Packed Single Precision Floating-Point Value
		val /66 [0x0f 0x3a 0x17 /r] = ternop sse4_1 EXTRACTPS r/m32 xmm128 imm8
		val /vex/66/0f/3a [0x17 /r] | vex128? = varity3 avx VEXTRACTPS r/m32 xmm128 imm8

### F2XM1
###  - Compute 2^x-1
		val / [0xd9 0xf0] = arity0 none F2XM1

### FABS
###  - Absolute Value
		val / [0xd9 0xe1] = arity0 none FABS

### FADD/FADDP/FIADD
###  - Add
		val / [0xd8 /0] = binop none FADD st0 st/m32
		val / [0xdc /0-mem] = binop none FADD st0 m64
		val / [0xdc /0-reg] = binop none FADD st/reg st0
		val / [0xde /0-reg] = binop none FADDP st/reg st0
		val / [0xda /0-mem] = unop none FIADD m32
		val / [0xde /0-mem] = unop none FIADD m16

### FBLD
###  - Load Binary Coded Decimal
		val / [0xdf /4-mem] = unop none FBLD m80

### FBSTP
###  - Store BCD Integer and Pop
		val / [0xdf /6-mem] = unop none FBSTP m80

### FCHS
###  - Change Sign
		val / [0xd9 0xe0] = arity0 none FCHS

### FCLEX/FNCLEX
###  - Clear Exceptions
#val / [0x9b 0xdb 0xe2] = arity0 none FCLEX
		val / [0xdb 0xe2] = arity0 none FNCLEX

### FCMOVcc
###  - Floating-Point Conditional Move
		val / [0xda /0-reg] = binop none FCMOVB st0 st/reg
		val / [0xda /1-reg] = binop none FCMOVE st0 st/reg
		val / [0xda /2-reg] = binop none FCMOVBE st0 st/reg
		val / [0xda /3-reg] = binop none FCMOVU st0 st/reg
		val / [0xdb /0-reg] = binop none FCMOVNB st0 st/reg
		val / [0xdb /1-reg] = binop none FCMOVNE st0 st/reg
		val / [0xdb /2-reg] = binop none FCMOVNBE st0 st/reg
		val / [0xdb /3-reg] = binop none FCMOVNU st0 st/reg

### FCOM/FCOMP/FCOMPP
###  - Compare Floating Point Values
		val / [0xd8 /2] = unop none FCOM st/m32
		val / [0xdc /2-mem] = unop none FCOM m64
		val / [0xd8 /3] = unop none FCOMP st/m32
		val / [0xdc /3-mem] = unop none FCOMP m64
		val / [0xde 0xd9] = arity0 none FCOMPP

### FCOMI/FCOMIP/FUCOMI/FUCOMIP
###  - Compare Floating Point Values and Set EFLAGS
		val / [0xdb /6-reg] = binop none FCOMI st0 st/reg
		val / [0xdf /6-reg] = binop none FCOMIP st0 st/reg
		val / [0xdb /5-reg] = binop none FUCOMI st0 st/reg
		val / [0xdf /5-reg] = binop none FUCOMIP st0 st/reg

### FCOS
###  - Cosine
		val / [0xd9 0xff] = arity0 none FCOS

### FDECSTP
###  - Decrement Stack-Top Pointer
		val / [0xd9 0xf6] = arity0 none FDECSTP

### FDIV/FDIVP/FIDIV
###  - Divide
		val / [0xd8 /6] = binop none FDIV st0 st/m32
		val / [0xdc /6-mem] = binop none FDIV st0 m64
		val / [0xdc /7-reg] = binop none FDIV st/reg st0
		val / [0xde /7-reg] = binop none FDIVP st/reg st0
		val / [0xda /6-mem] = binop none FIDIV st0 m32
		val / [0xde /6-mem] = binop none FIDIV st0 m16

### FDIVR/FDIVRP/FIDIVR
###  - Reverse Divide
		val / [0xd8 /7] = binop none FDIVR st0 st/m32
		val / [0xdc /7-mem] = binop none FDIVR st0 m64
		val / [0xdc /6-reg] = binop none FDIVR st/reg st0
		val / [0xde /6-reg] = binop none FDIVRP st/reg st0
		val / [0xda /7-mem] = unop none FIDIVR m32
		val / [0xde /7-mem] = unop none FIDIVR m16

### FFREE
###  - Free Floating-Point Register
		val / [0xdd /0-reg] = unop none FFREE st/reg

### FICOM/FICOMP
###  - Compare Integer
		val / [0xde /2-mem] = unop none FICOM m16
		val / [0xda /2-mem] = unop none FICOM m32
		val / [0xde /3-mem] = unop none FICOMP m16
		val / [0xda /3-mem] = unop none FICOMP m32

### FILD
###  - Load Integer
		val / [0xdf /0-mem] = unop none FILD m16
		val / [0xdb /0-mem] = unop none FILD m32
		val / [0xdf /5-mem] = unop none FILD m64

### FINCSTP
###  - Increment Stack-Top Pointer
		val / [0xd9 0xf7] = arity0 none FINCSTP

### FINIT/FNINIT
###  - Initialize Floating-Point Unit
#val / [0x9b 0xdb 0xe3] = arity0 none FINIT
		val / [0xdb 0xe3] = arity0 none FNINIT

### FIST/FISTP
###  - Store Integer
		val / [0xdf /2-mem] = unop none FIST m16
		val / [0xdb /2-mem] = unop none FIST m32
		val / [0xdf /3-mem] = unop none FISTP m16
		val / [0xdb /3-mem] = unop none FISTP m32
		val / [0xdf /7-mem] = unop none FISTP m64

### FISTTP
###  - Store Integer with Truncation
		val / [0xdf /1-mem] = unop none FISTTP m16
		val / [0xdb /1-mem] = unop none FISTTP m32
		val / [0xdd /1-mem] = unop none FISTTP m64

### FLD
###  - Load Floating Point Value
		val / [0xd9 /0] = unop none FLD st/m32
		val / [0xdd /0-mem] = unop none FLD m64
		val / [0xdb /5-mem] = unop none FLD m80

### FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ
###  - Load Constant
		val / [0xd9 0xe8] = arity0 none FLD1
		val / [0xd9 0xe9] = arity0 none FLDL2T
		val / [0xd9 0xea] = arity0 none FLDL2E
		val / [0xd9 0xeb] = arity0 none FLDPI
		val / [0xd9 0xec] = arity0 none FLDLG2
		val / [0xd9 0xed] = arity0 none FLDLN2
		val / [0xd9 0xee] = arity0 none FLDZ

### FLDCW
###  - Load x87 FPU Control Word
		val / [0xd9 /5-mem] = unop none FLDCW m2byte

### FLDENV
###  - Load x87 FPU Environment
### Todo: fix
### http://lxr.free-electrons.com/source/arch/x86/math-emu/reg_ld_str.c#L1026
		val / [0xd9 /4-mem]
		 | mode64? = unop none FLDENV m28byte
		 | mode32? = unop none FLDENV m14byte


### FMUL/FMULP/FIMUL
###  - Multiply
		val / [0xd8 /1] = binop none FMUL st0 st/m32
		val / [0xdc /1-mem] = binop none FMUL st0 m64
		val / [0xdc /1-reg] = binop none FMUL st/reg st0
		val / [0xde /1-reg] = binop none FMULP st/reg st0
		val / [0xda /1-mem] = unop none FIMUL m32
		val / [0xde /1-mem] = unop none FIMUL m16

### FNOP
###  - No Operation
		val / [0xd9 0xd0] = arity0 none FNOP

### FPATAN
###  - Partial Arctangent
		val / [0xd9 0xf3] = arity0 none FPATAN

### FPREM
###  - Partial Remainder
		val / [0xd9 0xf8] = arity0 none FPREM

### FPREM1
###  - Partial Remainder
		val / [0xd9 0xf5] = arity0 none FPREM1

### FPTAN
###  - Partial Tangent
		val / [0xd9 0xf2] = arity0 none FPTAN

### FRNDINT
###  - Round to Integer
		val / [0xd9 0xfc] = arity0 none FRNDINT

### FRSTOR
###  - Restore x87 FPU State
### Todo: fix
		val / [0xdd /4-mem]
		 | mode64? = unop none FRSTOR m108byte
		 | otherwise = unop none FRSTOR m94byte

### FSAVE/FNSAVE
###  - Store x87 FPU State
### Todo: fix
#val / [0x9b 0xdd /6-mem]
# | mode64? = unop none FSAVE m108byte
# | mode32? = unop none FSAVE m94byte
		val / [0xdd /6-mem]
		 | mode64? = unop none FNSAVE m108byte
		 | mode32? = unop none FNSAVE m94byte

### FSCALE
###  - Scale
		val / [0xd9 0xfd] = arity0 none FSCALE

### FSIN
###  - Sine
		val / [0xd9 0xfe] = arity0 none FSIN

### FSINCOS
###  - Sine and Cosine
		val / [0xd9 0xfb] = arity0 none FSINCOS

### FSQRT
###  - Square Root
		val / [0xd9 0xfa] = arity0 none FSQRT

### FST/FSTP
###  - Store Floating Point Value
		val / [0xd9 /2-mem] = unop none FST m32
		val / [0xdd /2] = unop none FST st/m64
		val / [0xd9 /3-mem] = unop none FSTP m32
		val / [0xdd /3] = unop none FSTP st/m64
		val / [0xdb /7-mem] = unop none FSTP m80

### FSTCW/FNSTCW
###  - Store x87 FPU Control Word
#val / [0x9b 0xd9 /7-mem] = unop none FSTCW m2byte
		val / [0xd9 /7-mem] = unop none FNSTCW m2byte

### FSTENV/FNSTENV
###  - Store x87 FPU Environment
### Todo: fix
#val / [0x9b 0xd9 /6-mem]
# | mode64? = unop none FSTENV m28byte
# | mode32? = unop none FSTENV m14byte
		val / [0xd9 /6-mem]
		 | mode64? = unop none FNSTENV m28byte
		 | mode32? = unop none FNSTENV m14byte

### FSTSW/FNSTSW
###  - Store x87 FPU Status Word
#val / [0x9b 0xdd /7-mem] = unop none FSTSW m2byte
#val / [0x9b 0xdf 0xe0] = unop none FSTSW ax
		val / [0xdd /7-mem] = unop none FNSTSW m2byte
		val / [0xdf 0xe0] = unop none FNSTSW ax

### FSUB/FSUBP/FISUB
###  - Subtract
		val / [0xd8 /4] = binop none FSUB st0 st/m32
		val / [0xdc /4-mem] = binop none FSUB st0 m64
		val / [0xdc /5-reg] = binop none FSUB st/reg st0
		val / [0xde /5-reg] = binop none FSUBP st/reg st0
		val / [0xda /4-mem] = unop none FISUB m32
		val / [0xde /4-mem] = unop none FISUB m16

### FSUBR/FSUBRP/FISUBR
###  - Reverse Subtract
		val / [0xd8 /5] = binop none FSUBR st0 st/m32
		val / [0xdc /5-mem] = binop none FSUBR st0 m64
		val / [0xdc /4-reg] = binop none FSUBR st/reg st0
		val / [0xde /4-reg] = binop none FSUBRP st/reg st0
		val / [0xda /5-mem] = unop none FISUBR m32
		val / [0xde /5-mem] = unop none FISUBR m16

### FTST
###  - TEST
		val / [0xd9 0xe4] = arity0 none FTST

### FUCOM/FUCOMP/FUCOMPP
###  - Unordered Compare Floating Point Values
		val / [0xdd /4-reg] = unop none FUCOM st/reg
		val / [0xdd /5-reg] = unop none FUCOMP st/reg
		val / [0xda 0xe9] = arity0 none FUCOMPP

### FXAM
###  - Examine ModR/M
		val / [0xd9 0xe5] = arity0 none FXAM

### FXCH
###  - Exchange Register Contents
		val / [0xd9 /1-reg] = unop none FXCH st/reg

### FXRSTOR
###  - Restore x87 FPU, MMX , XMM, and MXCSR State
### Todo: fix
		val / [0x0f 0xae /1-mem]
		 | rexw? = unop none FXRSTOR64 m512byte
		 | otherwise = unop none FXRSTOR m512byte

### FXSAVE
###  - Save x87 FPU, MMX Technology, and SSE State
### Todo: fix
		val / [0x0f 0xae /0-mem]
		 | rexw? = unop none FXSAVE64 m512byte
		 | otherwise = unop none FXSAVE m512byte

### FXTRACT
###  - Extract Exponent and Significand
		val / [0xd9 0xf4] = arity0 none FXTRACT

### FYL2X
###  - Compute y*log_2(x)
		val / [0xd9 0xf1] = arity0 none FYL2X

### FYL2XP1
###  - Compute y*log_2(x +1)
		val / [0xd9 0xf9] = arity0 none FYL2XP1

### HADDPD
###  - Packed Double-FP Horizontal Add
		val /66 [0x0f 0x7c /r] = binop sse3 HADDPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x7c /r]
		 | vex128? = varity3 avx VHADDPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VHADDPD ymm256 v/ymm ymm/m256

### HADDPS
###  - Packed Single-FP Horizontal Add
		val /f2 [0x0f 0x7c /r] = binop sse3 HADDPS xmm128 xmm/m128
		val /vex/f2/0f/vexv [0x7c /r]
		 | vex128? = varity3 avx VHADDPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VHADDPS ymm256 v/ymm ymm/m256

### HLT
###  - Halt
		val / [0xf4] = arity0 none HLT

### HSUBPD
###  - Packed Double-FP Horizontal Subtract
		val /66 [0x0f 0x7d /r] = binop sse3 HSUBPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x7d /r]
		 | vex128? = varity3 avx VHSUBPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VHSUBPD ymm256 v/ymm ymm/m256

### HSUBPS
###  - Packed Single-FP Horizontal Subtract
		val /f2 [0x0f 0x7d /r] = binop sse3 HSUBPS xmm128 xmm/m128
		val /vex/f2/0f/vexv [0x7d /r]
		 | vex128? = varity3 avx VHSUBPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VHSUBPS ymm256 v/ymm ymm/m256

### IDIV
###  - Signed Divide
		val / [0xf6 /7] = unop none IDIV r/m8
		val / [0xf7 /7]
		 | rexw? = unop none IDIV r/m64
		 | opndsz? = unop none IDIV r/m16
		 | otherwise = unop none IDIV r/m32

### IMUL
###  - Signed Multiply
#val / [0xf6 /5] = unop IMUL r/m8
#val / [0xf7 /5]
# | rexw? = unop IMUL r/m64
# | opndsz? = unop IMUL r/m16
# | otherwise = unop IMUL r/m32
#val / [0x0f 0xaf /r]
# | rexw? = binop IMUL r64 r/m64
# | opndsz? = binop IMUL r16 r/m16
# | otherwise = binop IMUL r32 r/m32
#val / [0x6b /r]
# | rexw? = ternop IMUL r64 r/m64 imm8
# | opndsz? = ternop IMUL r16 r/m16 imm8
# | otherwise = ternop IMUL r32 r/m32 imm8
#val / [0x69 /r]
# | rexw? = ternop IMUL r64 r/m64 imm32
# | opndsz? = ternop IMUL r16 r/m16 imm16
# | otherwise = ternop IMUL r32 r/m32 imm32

### IMUL
###  - Signed Multiply
		val / [0xf6 /5] = varity1 none IMUL r/m8
		val / [0xf7 /5]
		 | rexw? = varity1 none IMUL r/m64
		 | opndsz? = varity1 none IMUL r/m16
		 | otherwise = varity1 none IMUL r/m32
		val / [0x0f 0xaf /r]
		 | rexw? = varity2 none IMUL r64 r/m64
		 | opndsz? = varity2 none IMUL r16 r/m16
		 | otherwise = varity2 none IMUL r32 r/m32
		val / [0x6b /r]
		 | rexw? = varity3 none IMUL r64 r/m64 imm8
		 | opndsz? = varity3 none IMUL r16 r/m16 imm8
		 | otherwise = varity3 none IMUL r32 r/m32 imm8
		val / [0x69 /r]
		 | rexw? = varity3 none IMUL r64 r/m64 imm32
		 | opndsz? = varity3 none IMUL r16 r/m16 imm16
		 | otherwise = varity3 none IMUL r32 r/m32 imm32

### IN
###  - Input from Port
		val / [0xe4] = binop none IN al imm8
		val / [0xe5]
		 | opndsz? = binop none IN ax imm8
		 | otherwise = binop none IN eax imm8
		val / [0xec] = binop none IN al dx
		val / [0xed]
		 | opndsz? = binop none IN ax dx
		 | otherwise = binop none IN eax dx

### INC
###  - Increment by 1
		val / [0xfe /0] = unop-lock none INC r/m8
		val / [0xff /0]
		 | rexw? = unop-lock none INC r/m64
		 | opndsz? = unop-lock none INC r/m16
		 | otherwise = unop-lock none INC r/m32

### INS/INSB/INSW/INSD
###  - Input from Port to String
		val / [0x6c] = arity0-rep none INSB
		val / [0x6d]
		 | opndsz? = arity0-rep none INSW
		 | otherwise = arity0-rep none INSD

### INSERTPS
###  - Insert Packed Single Precision Floating-Point Value
		val /66 [0x0f 0x3a 0x21 /r] = ternop sse4_1 INSERTPS xmm128 xmm/m32 imm8
		val /vex/66/0f/3a/vexv [0x21 /r] = varity4 avx VINSERTPS xmm128 v/xmm xmm/m32 imm8

### INT n/INTO/INT 3
###  - Call to Interrupt Procedure
		val / [0xcc] = arity0 none INT3
		val / [0xcd] = unop none INT imm8
		val / [0xce] | mode32? = arity0 none INT0

### INVD
###  - Invalidate Internal Caches
		val / [0x0f 0x08] = arity0 none INVD

### INVLPG
###  - Invalidate TLB Entry
		val / [0x0f 0x01 /7-mem] = unop none INVLPG m0

### INVPCID
###  - Invalidate Process-Context Identifier
		val /66 [0x0f 0x38 0x82 /r-mem]
		 | mode64? = binop invpcid INVPCID r64 m128
		 | mode32? = binop invpcid INVPCID r32 m128

### IRET/IRETD
###  - Interrupt Return
		val / [0xcf]
		 | rexw? = arity0 none IRETQ
		 | opndsz? = arity0 none IRET
		 | otherwise = arity0 none IRETD

### Jcc
###  - Jump if Condition Is Met
		val / [0x77] = near-rel none JA rel8  # JNBE
		val / [0x73] = near-rel none JAE rel8 # JNB, JNC
		val / [0x72] = near-rel none JC rel8  # JB, JNAE
		val / [0x76] = near-rel none JBE rel8 # JNA
		val /66 [0xe3] = near-rel none JCXZ rel8
		val / [0xe3]
		 | mode64? && addrsz? = near-rel none JECXZ rel8
		 | mode64? = near-rel none JRCXZ rel8
		 | mode32? && addrsz? = near-rel none JCXZ rel8
		 | mode32? = near-rel none JECXZ rel8
		val / [0x74] = near-rel none JE rel8  # JZ
		val / [0x7f] = near-rel none JG rel8  # JNLE
		val / [0x7d] = near-rel none JGE rel8 # JNL
		val / [0x7c] = near-rel none JL rel8  # JNGE
		val / [0x7e] = near-rel none JLE rel8 # JNG
		val / [0x75] = near-rel none JNE rel8 # JNZ
		val / [0x71] = near-rel none JNO rel8
		val / [0x7b] = near-rel none JNP rel8 # JPO
		val / [0x79] = near-rel none JNS rel8
		val / [0x70] = near-rel none JO rel8
		val / [0x7a] = near-rel none JP rel8  # JPE
		val / [0x78] = near-rel none JS rel8
		val / [0x0f 0x87] # JNBE
		 | mode32? && opndsz? = near-rel none JA rel16
		 | otherwise = near-rel none JA rel32
		val / [0x0f 0x83] # JNB, JNC
		 | mode32? && opndsz? = near-rel none JAE rel16
		 | otherwise = near-rel none JAE rel32
		val / [0x0f 0x82] # JC, JNAE
		 | mode32? && opndsz? = near-rel none JB rel16
		 | otherwise = near-rel none JB rel32
		val / [0x0f 0x86] # JNA
		 | mode32? && opndsz? = near-rel none JBE rel16
		 | otherwise = near-rel none JBE rel32
		val / [0x0f 0x84] # JZ
		 | mode32? && opndsz? = near-rel none JE rel16
		 | otherwise = near-rel none JE rel32
		val / [0x0f 0x8f] # JNLE
		 | mode32? && opndsz? = near-rel none JG rel16
		 | otherwise = near-rel none JG rel32
		val / [0x0f 0x8d] # JNL
		 | mode32? && opndsz? = near-rel none JGE rel16
		 | otherwise = near-rel none JGE rel32
		val / [0x0f 0x8c] # JNGE
		 | mode32? && opndsz? = near-rel none JL rel16
		 | otherwise = near-rel none JL rel32
		val / [0x0f 0x8e] # JNG
		 | mode32? && opndsz? = near-rel none JLE rel16
		 | otherwise = near-rel none JLE rel32
		val / [0x0f 0x85] # JNZ
		 | mode32? && opndsz? = near-rel none JNE rel16
		 | otherwise = near-rel none JNE rel32
		val / [0x0f 0x81]
		 | mode32? && opndsz? = near-rel none JNO rel16
		 | otherwise = near-rel none JNO rel32
		val / [0x0f 0x8b] # JPO
		 | mode32? && opndsz? = near-rel none JNP rel16
		 | otherwise = near-rel none JNP rel32
		val / [0x0f 0x89]
		 | mode32? && opndsz? = near-rel none JNS rel16
		 | otherwise = near-rel none JNS rel32
		val / [0x0f 0x80]
		 | mode32? && opndsz? = near-rel none JO rel16
		 | otherwise = near-rel none JO rel32
		val / [0x0f 0x8a] # JPE
		 | mode32? && opndsz? = near-rel none JP rel16
		 | otherwise = near-rel none JP rel32
		val / [0x0f 0x88] # JS
		 | mode32? && opndsz? = near-rel none JS rel16
		 | otherwise = near-rel none JS rel32

### JMP
###  - Jump
		val / [0xeb] = near-rel none JMP rel8
		val / [0xe9]
		 | mode32? && opndsz? = near-rel none JMP rel16
		 | otherwise = near-rel none JMP rel32
		val / [0xff /4]
		 | mode32? && opndsz? = near-abs none JMP r/m16
		 | mode32? = near-abs none JMP r/m32
		 | mode64? = near-abs none JMP r/m64
		val / [0xea]
		 | mode32? && opndsz? = far-dir none JMP ptr16/16
		 | mode32? = far-dir none JMP ptr16/32
		val / [0xff /5]
		 | rexw? = far-ind none JMP m16/64
		 | opndsz? = far-ind none JMP m16/16
		 | otherwise = far-ind none JMP m16/32

### LAHF
###  - Load Status Flags into AH Register
		val / [0x9f] = arity0 none LAHF

### LAR
###  - Load Access Rights Byte
		val / [0x0f 0x02 /r]
		 | rexw? = binop none LAR r64 r32/m16
		 | opndsz? = binop none LAR r16 r16/m16
		 | otherwise = binop none LAR r32 r32/m16

### LDDQU
###  - Load Unaligned Integer 128 Bits
		val /f2 [0x0f 0xf0 /r-mem] = binop sse3 LDDQU xmm128 m128
		val /vex/f2/0f [0xf0 /r-mem]
		 | vex128? = varity2 avx VLDDQU xmm128 m128
		 | vex256? = varity2 avx VLDDQU ymm256 m256

### LDMXCSR
###  - Load MXCSR Register
		val / [0x0f 0xae /2-mem] = unop sse LDMXCSR m32
		val /vex/0f [0xae /2-mem]
		 | vex128? = varity1 avx VLDMXCSR m32

### LDS/LES/LFS/LGS/LSS
###  - Load Far Pointer
#val / [0xc5 /r-mem]
# | opndsz? = binop LDS r16 m16/16
# | otherwise = binop LDS r32 m16/32
		val / [0x0f 0xb2 /r-mem]
		 | rexw? = binop none LSS r64 m16/64
		 | opndsz? = binop none LSS r16 m16/16
		 | otherwise = binop none LSS r32 m16/32
#val / [0xc4 /r-mem]
# | opndsz? = binop LES r16 m16/16
# | otherwise = binop LES r32 m16/32
		val / [0x0f 0xb4 /r-mem]
		 | rexw? = binop none LFS r64 m16/64
		 | opndsz? = binop none LFS r16 m16/16
		 | otherwise = binop none LFS r32 m16/32
		val / [0x0f 0xb5 /r-mem]
		 | rexw? = binop none LGS r64 m16/64
		 | opndsz? = binop none LGS r16 m16/16
		 | otherwise = binop none LGS r32 m16/32

### LEA
###  - Load Effective Address
		val / [0x8d /r-mem]
		 | (// mode64?) && opndsz? = binop none LEA r16 mX
		 | (// mode64?) && (// opndsz?) = binop none LEA r32 mX
		 | mode64? && (// rexw?) && opndsz? = binop none LEA r16 mX
		 | mode64? && (// rexw?) && (// opndsz?) = binop none LEA r32 mX
		 | mode64? && rexw? && (// opndsz?) = binop none LEA r64 mX

### LEAVE
###  - High Level Procedure Exit
#Todo: handle different effects to BP/EBP/RBP
		val / [0xc9]
		 | mode64? = do update@{default-operand-size=64}; arity0 none LEAVE end
		 | otherwise = arity0 none LEAVE

### LFENCE
###  - Load Fence
#Todo: -reg?
		val / [0x0f 0xae /5-reg] = arity0 none LFENCE

### LGDT/LIDT
###  - Load Global/Interrupt Descriptor Table Register
		val / [0x0f 0x01 /2-mem]
		 | mode64? = unop none LGDT m16/64
		 | otherwise = unop none LGDT m16/32
		val / [0x0f 0x01 /3-mem]
		 | mode64? = unop none LIDT m16/64
		 | otherwise = unop none LIDT m16/32

### LLDT
###  - Load Local Descriptor Table Register
		val / [0x0f 0x00 /2] = unop none LLDT r/m16

### LMSW
###  - Load Machine Status Word
		val / [0x0f 0x01 /6-mem] = unop none LMSW r/m16

### LOCK
###  - Assert LOCK# Signal Prefix

### LODS/LODSB/LODSW/LODSD/LODSQ
###  - Load String
		val / [0xac] = unop-rep none LODS (m/default/si/esi/rsi (return 8))
		val / [0xad]
		 | rexw? = unop-rep none LODS (m/default/si/esi/rsi operand-size)
		 | opndsz? = unop-rep none LODS (m/default/si/esi/rsi operand-size)
		 | otherwise = unop-rep none LODS (m/default/si/esi/rsi operand-size)

### LOOP/LOOPcc
###  - Loop According to ECX Counter
		val / [0xe2] = near-rel none LOOP rel8
		val / [0xe1] = near-rel none LOOPE rel8
		val / [0xe0] = near-rel none LOOPNE rel8

### LSL
###  - Load Segment Limit
		val / [0x0f 0x03 /r]
		 | rexw? = binop none LSL r64 r32/m16
		 | opndsz? = binop none LSL r16 r16/m16
		 | otherwise = binop none LSL r32 r32/m16

### LTR
###  - Load Task Register
		val / [0x0f 0x00 /3] = unop none LTR r/m16

### MASKMOVDQU
###  - Store Selected Bytes of Double Quadword
		val /66 [0x0f 0xf7 /r-reg] = ternop sse2 MASKMOVDQU (m/default/di/edi/rdi (return 8)) xmm128 xmm/reg128
		val /vex/66/0f [0xf7 /r-reg] | vex128? = varity3 avx VMASKMOVDQU (m/default/di/edi/rdi (return 8)) xmm128 xmm/m128

### MASKMOVQ
###  - Store Selected Bytes of Quadword
		val / [0x0f 0xf7 /r-reg] = ternop none MASKMOVQ (m/default/di/edi/rdi (return 8)) mm64 mm/reg64

### MAXPD
###  - Return Maximum Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x5f /r] = binop sse2 MAXPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x5f /r]
		 | vex128? = varity3 avx VMAXPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VMAXPD ymm256 v/ymm ymm/m256

### MAXPS
###  - Return Maximum Packed Single-Precision Floating-Point Values
		val / [0x0f 0x5f /r] = binop sse MAXPS xmm128 xmm/m128
		val /vex/0f/vexv [0x5f /r]
		 | vex128? = varity3 avx VMAXPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VMAXPS ymm256 v/ymm ymm/m256

### MAXSD
###  - Return Maximum Scalar Double-Precision Floating-Point Value
		val /f2 [0x0f 0x5f /r] = binop sse2 MAXSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x5f /r] = varity3 avx VMAXSD xmm128 v/xmm xmm/m64

### MAXSS
###  - Return Maximum Scalar Single-Precision Floating-Point Value
		val /f3 [0x0f 0x5f /r] = binop sse MAXSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x5f /r] = varity3 avx VMAXSS xmm128 v/xmm xmm/m32

### MFENCE
###  - Memory Fence
#Todo: -reg?
		val / [0x0f 0xae /6-reg] = arity0 none MFENCE

### MINPD
###  - Return Minimum Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x5d /r] = binop sse2 MINPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x5d /r]
		 | vex128? = varity3 avx VMINPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VMINPD ymm256 v/ymm ymm/m256

### MINPS
###  - Return Minimum Packed Single-Precision Floating-Point Values
		val / [0x0f 0x5d /r] = binop sse MINPS xmm128 xmm/m128
		val /vex/0f/vexv [0x5d /r]
		 | vex128? = varity3 avx VMINPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VMINPS ymm256 v/ymm ymm/m256

### MINSD
###  - Return Minimum Scalar Double-Precision Floating-Point Value
		val /f2 [0x0f 0x5d /r] = binop sse2 MINSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x5d /r] = varity3 avx VMINSD xmm128 v/xmm xmm/m64

### MINSS
###  - Return Minimum Scalar Single-Precision Floating-Point Value
		val /f3 [0x0f 0x5d /r] = binop sse MINSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x5d /r] = varity3 avx VMINSS xmm128 v/xmm xmm/m32

### MONITOR
###  - Set Up Monitor Address
		val / [0x0f 0x01 0xc8] = arity0 none MONITOR

### MOV
###  - Move
		val / [0x88 /r] = binop none MOV r/m8 r8
		val / [0x89 /r]
		 | rexw? = binop none MOV r/m64 r64
		 | opndsz? = binop none MOV r/m16 r16
		 | otherwise = binop none MOV r/m32 r32
		val / [0x8a /r] = binop none MOV r8 r/m8
		val / [0x8b /r]
		 | rexw? = binop none MOV r64 r/m64
		 | opndsz? = binop none MOV r16 r/m16
		 | otherwise = binop none MOV r32 r/m32
		val / [0x8c /r]
		 | rexw? = binop none MOV r/m64 (r/rexb sreg3?)
		 | opndsz? = binop none MOV r/m32 (r/rexb sreg3?)
		 | otherwise = binop none MOV r/m16 (r/rexb sreg3?)
		val / [0x8e /r]
		 | rexw? = binop none MOV (r/rexb sreg3?) r/m64
		 | opndsz? = binop none MOV (r/rexb sreg3?) r/m16
		 | otherwise = binop none MOV (r/rexb sreg3?) r/m32
		val / [0xa0] = binop none MOV al moffs8
		val / [0xa1]
		 | addrsz? = binop none MOV ax moffs16
		 | rexw? = binop none MOV rax moffs64
		 | otherwise = binop none MOV eax moffs32
		val / [0xa2] = binop none MOV moffs8 al
		val / [0xa3]
		 | addrsz? = binop none MOV moffs16 ax
		 | rexw? = binop none MOV moffs64 rax
		 | otherwise = binop none MOV moffs32 eax
		val / ['10110 r:3'] = do update@{reg/opcode=r}; binop none MOV r8/rexb imm8 end
		val / ['10111 r:3']
		 | rexw? = do update@{reg/opcode=r}; binop none MOV r64/rexb imm64 end
		 | opndsz? = do update@{reg/opcode=r}; binop none MOV r16/rexb imm16 end
		 | otherwise = do update@{reg/opcode=r}; binop none MOV r32/rexb imm32 end
		val / [0xc6 /0] = binop none MOV r/m8 imm8
		val / [0xc7 /0]
		 | rexw? = binop none MOV r/m64 imm32
		 | opndsz? = binop none MOV r/m16 imm16
		 | otherwise = binop none MOV r/m32 imm32

### Todo: Move to/from Debug/Control Registers

### MOVAPD
###  - Move Aligned Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x28 /r] = binop sse2 MOVAPD xmm128 xmm/m128
		val /66 [0x0f 0x29 /r] = binop sse2 MOVAPD xmm/m128 xmm128
		val /vex/66/0f [0x28 /r]
		 | vex128? = varity2 avx VMOVAPD xmm128 xmm/m128
		 | vex256? = varity2 avx VMOVAPD ymm256 ymm/m256
		val /vex/66/0f [0x29 /r]
		 | vex128? = varity2 avx VMOVAPD xmm/m128 xmm128
		 | vex256? = varity2 avx VMOVAPD ymm/m256 ymm256

### MOVAPS
###  - Move Aligned Packed Single-Precision Floating-Point Values
		val / [0x0f 0x28 /r] = binop sse MOVAPS xmm128 xmm/m128
		val / [0x0f 0x29 /r] = binop sse MOVAPS xmm/m128 xmm128
		val /vex/0f [0x28 /r]
		 | vex128? = varity2 avx VMOVAPS xmm128 xmm/m128
		 | vex256? = varity2 avx VMOVAPS ymm256 ymm/m256
		val /vex/0f [0x29 /r]
		 | vex128? = varity2 avx VMOVAPS xmm/m128 xmm128
		 | vex256? = varity2 avx VMOVAPS ymm/m256 ymm256

### MOVBE
###  - Move Data After Swapping Bytes
		val / [0x0f 0x38 0xf0 /r-mem]
		 | rexw? = binop none MOVBE r64 m64
		 | opndsz? = binop none MOVBE r16 m16
		 | otherwise = binop none MOVBE r32 m32
		val / [0x0f 0x38 0xf1 /r-mem]
		 | rexw? = binop none MOVBE m64 r64
		 | opndsz? = binop none MOVBE m16 r16
		 | otherwise = binop none MOVBE m32 r32

### MOVD/MOVQ
###  - Move Doubleword/Move Quadword
		val / [0x0f 0x6e /r]
		 | rexw? = binop mmx MOVQ mm64 r/m64
		 | otherwise = binop mmx MOVD mm64 r/m32
		val / [0x0f 0x7e /r]
		 | rexw? = binop mmx MOVQ r/m64 mm64
		 | otherwise = binop mmx MOVD r/m32 mm64
		val /vex/66/0f [0x6e /r]
		 | vex128? && vexw1? = varity2 avx VMOVQ xmm128 r/m64
		 | vex128? && vexw0? = varity2 avx VMOVD xmm128 r/m32
		val /66 [0x0f 0x6e /r]
		 | rexw? = binop sse2 MOVQ xmm128 r/m64
		 | otherwise = binop sse2 MOVD xmm128 r/m32
		val /66 [0x0f 0x7e /r]
		 | rexw? = binop sse2 MOVQ r/m64 xmm128
		 | otherwise = binop sse2 MOVD r/m32 xmm128
		val /vex/66/0f [0x7e /r]
		 | vex128? && vexw1? = varity2 avx VMOVQ r/m64 xmm128
		 | vex128? && vexw0? = varity2 avx VMOVD r/m32 xmm128

### MOVDDUP
###  - Move One Double-FP and Duplicate
		val /f2 [0x0f 0x12 /r] = binop sse3 MOVDDUP xmm128 xmm/m64
		val /vex/f2/0f [0x12 /r]
		 | vex128? = varity2 avx VMOVDDUP xmm128 xmm/m64
		 | vex256? = varity2 avx VMOVDDUP ymm256 ymm/m256

### MOVDQA
###  - Move Aligned Double Quadword
		val /66 [0x0f 0x6f /r] = binop sse2 MOVDQA xmm128 xmm/m128
		val /66 [0x0f 0x7f /r] = binop sse2 MOVDQA xmm/m128 xmm128
		val /vex/66/0f [0x6f /r]
		 | vex128? = varity2 avx VMOVDQA xmm128 xmm/m128
		 | otherwise = varity2 avx VMOVDQA ymm256 ymm/m256
		val /vex/66/0f [0x7f /r]
		 | vex128? = varity2 avx VMOVDQA xmm/m128 xmm128
		 | otherwise = varity2 avx VMOVDQA ymm/m256 ymm256

### MOVDQU
###  - Move Unaligned Double Quadword
		val /f3 [0x0f 0x6f /r] = binop sse2 MOVDQU xmm128 xmm/m128
		val /f3 [0x0f 0x7f /r] = binop sse2 MOVDQU xmm/m128 xmm128
		val /vex/f3/0f [0x6f /r]
		 | vex128? = varity2 avx VMOVDQU xmm128 xmm/m128
		 | otherwise = varity2 avx VMOVDQU ymm256 ymm/m256
		val /vex/f3/0f [0x7f /r]
		 | vex128? = varity2 avx VMOVDQU xmm/m128 xmm128
		 | otherwise = varity2 avx VMOVDQU ymm/m256 ymm256

### MOVDQ2Q
###  - Move Quadword from XMM to MMX Technology Register
		val /f2 [0x0f 0xd6 /r-reg] = binop none MOVDQ2Q mm64 xmm/reg128

### MOVHLPS
###  - Move Packed Single-Precision Floating-Point Values High to Low
		val / [0x0f 0x12 /r-reg] = binop sse3 MOVHLPS xmm128 xmm/reg128
		val /vex/0f/vexv [0x12 /r-reg] | vex128? = varity3 avx VMOVHLPS xmm128 v/xmm xmm/reg128

### MOVHPD
###  - Move High Packed Double-Precision Floating-Point Value
		val /66 [0x0f 0x16 /r-mem] = binop sse2 MOVHPD xmm128 m64
		val /66 [0x0f 0x17 /r-mem] = binop sse2 MOVHPD m64 xmm128
		val /vex/66/0f/vexv [0x16 /r-mem] | vex128? = varity3 avx VMOVHPD xmm128 v/xmm m64
		val /vex/66/0f [0x17 /r-mem] | vex128? = varity2 avx VMOVHPD m64 xmm128

### MOVHPS
###  - Move High Packed Single-Precision Floating-Point Values
		val / [0x0f 0x16 /r-mem] = binop sse MOVHPS xmm128 m64
		val / [0x0f 0x17 /r-mem] = binop sse MOVHPS m64 xmm128
		val /vex/0f/vexv [0x16 /r-mem] | vex128? = varity3 avx VMOVHPS xmm128 v/xmm m64
		val /vex/0f [0x17 /r-mem] | vex128? = varity2 avx VMOVHPS m64 xmm128

### MOVLHPS
###  - Move Packed Single-Precision Floating-Point Values Low to High
		val / [0x0f 0x16 /r-reg] = binop sse MOVLHPS xmm128 xmm/reg128
		val /vex/0f/vexv [0x16 /r-reg] | vex128? = varity3 avx VMOVLHPS xmm128 v/xmm xmm/reg128

### MOVLPD
###  - Move Low Packed Double-Precision Floating-Point Value
		val /66 [0x0f 0x12 /r-mem] = binop sse2 MOVLPD xmm128 m64
		val /66 [0x0f 0x13 /r-mem] = binop sse2 MOVLPD m64 xmm128
		val /vex/66/0f/vexv [0x12 /r-mem] | vex128? = varity3 avx VMOVLPD xmm128 v/xmm m64
		val /vex/66/0f [0x13 /r-mem] | vex128? = varity2 avx VMOVLPD m64 xmm128

### MOVLPS
###  - Move Low Packed Single-Precision Floating-Point Values
		val / [0x0f 0x12 /r-mem] = binop sse MOVLPS xmm128 m64
		val / [0x0f 0x13 /r-mem] = binop sse MOVLPS m64 xmm128
		val /vex/0f/vexv [0x12 /r-mem] | vex128? = varity3 avx VMOVLPS xmm128 v/xmm m64
		val /vex/0f [0x13 /r-mem] | vex128? = varity2 avx VMOVLPS m64 xmm128

### MOVMSKPD
###  - Extract Packed Double-Precision Floating-Point Sign Mask
		val /66 [0x0f 0x50 /r-reg]
		 | mode64? = binop sse2 MOVMSKPD r64 xmm/reg128
		 | otherwise = binop sse2 MOVMSKPD r32 xmm/reg128
		val /vex/66/0f [0x50 /r-reg]
		 | vex128? && mode64? = varity2 avx VMOVMSKPD r64 xmm/reg128
		 | vex128? = varity2 avx VMOVMSKPD r32 xmm/reg128
		 | vex256? && mode64? = varity2 avx VMOVMSKPD r64 ymm/reg256
		 | vex256? = varity2 avx VMOVMSKPD r32 ymm/reg256

### MOVMSKPS
###  - Extract Packed Single-Precision Floating-Point Sign Mask
		val / [0x0f 0x50 /r-reg]
		 | mode64? = binop sse MOVMSKPD r64 xmm/reg128
		 | otherwise = binop sse MOVMSKPD r32 xmm/reg128
		val /vex/0f [0x50 /r-reg]
		 | vex128? && mode64? = varity2 avx VMOVMSKPS r64 xmm/reg128
		 | vex128? = varity2 avx VMOVMSKPS r32 xmm/reg128
		 | vex256? && mode64? = varity2 avx VMOVMSKPS r64 ymm/reg256
		 | vex256? = varity2 avx VMOVMSKPS r32 ymm/reg256

### MOVNTDQA
###  - Load Double Quadword Non-Temporal Aligned Hint
		val /66 [0x0f 0x38 0x2a /r-mem] = binop sse4_1 MOVNTDQA xmm128 m128
		val /vex/66/0f/38 [0x2a /r-mem] | vex128? = varity2 avx VMOVNTDQA xmm128 m128

### MOVNTDQ
###  - Store Double Quadword Using Non-Temporal Hint
		val /66 [0x0f 0xe7 /r-mem] = binop sse2 MOVNTDQ m128 xmm128
		val /vex/66/0f [0xe7 /r-mem]
		 | vex128? = varity2 avx VMOVNTDQ m128 xmm128
		 | vex256? = varity2 avx VMOVNTDQ m256 ymm256

### MOVNTI
###  - Store Doubleword Using Non-Temporal Hint
		val / [0x0f 0xc3 /r-mem]
		 | rexw? = binop none MOVNTI m64 r64
		 | otherwise = binop none MOVNTI m32 r32

### MOVNTPD
###  - Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint
		val /66 [0x0f 0x2b /r-mem] = binop sse2 MOVNTPD m128 xmm128
		val /vex/66/0f [0x2b /r-mem]
		 | vex128? = varity2 avx VMOVNTPD m128 xmm128
		 | vex256? = varity2 avx VMOVNTPD m256 ymm256

### MOVNTPS
###  - Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint
		val / [0x0f 0x2b /r-mem] = binop sse MOVNTPS m128 xmm128
		val /vex/0f [0x2b /r-mem]
		 | vex128? = varity2 avx VMOVNTPS m128 xmm128
		 | vex256? = varity2 avx VMOVNTPS m256 ymm256

### MOVNTQ
###  - Store of Quadword Using Non-Temporal Hint
		val / [0x0f 0xe7 /r-mem] = binop none MOVNTQ m64 mm64

### MOVQ
###  - Move Quadword
		val / [0x0f 0x6f /r] = binop mmx MOVQ mm64 mm/m64
		val / [0x0f 0x7f /r] = binop mmx MOVQ mm/m64 mm64
		val /f3 [0x0f 0x7e /r] = binop sse2 MOVQ xmm128 xmm/m64
		val /66 [0x0f 0xd6 /r] = binop sse2 MOVQ xmm/m64 xmm128

### MOVQ2DQ
###  - Move Quadword from MMX Technology to XMM Register
		val /f3 [0x0f 0xd6 /r-reg] = binop none MOVQ2DQ xmm128 mm/reg64

### MOVS/MOVSB/MOVSW/MOVSD/MOVSQ
###  - Move Data from String to String
		val / [0xa4] = binop-rep none MOVS (m/es/di/edi/rdi (return 8)) (m/default/si/esi/rsi (return 8))
		val / [0xa5]
		 | rexw? = binop-rep none MOVS (m/es/di/edi/rdi operand-size) (m/default/si/esi/rsi operand-size)
		 | opndsz? = binop-rep none MOVS (m/es/di/edi/rdi operand-size) (m/default/si/esi/rsi operand-size)
		 | otherwise = binop-rep none MOVS (m/es/di/edi/rdi operand-size) (m/default/si/esi/rsi operand-size)

### MOVSD
###  - Move Scalar Double-Precision Floating-Point Value
		val /f2 [0x0f 0x10 /r] = binop sse2 MOVSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x10 /r-reg] = varity3 avx VMOVSD xmm128 v/xmm xmm/reg128
		val /vex/f2/0f [0x10 /r-mem] = varity2 avx VMOVSD xmm128 m64
		val /f2 [0x0f 0x11 /r] = binop sse2 MOVSD xmm/m64 xmm128
		val /vex/f2/0f/vexv [0x11 /r-reg] = varity3 avx VMOVSD xmm/reg128 v/xmm xmm128
		val /vex/f2/0f [0x11 /r-mem] = varity2 avx VMOVSD m64 xmm128

### MOVSHDUP
###  - Move Packed Single-FP High and Duplicate
		val /f3 [0x0f 0x16 /r] = binop sse3 MOVSHDUP xmm128 xmm/m128
		val /vex/f3/0f [0x16 /r]
		 | vex128? = varity2 avx VMOVSHDUP xmm128 xmm/m128
		 | vex256? = varity2 avx VMOVSHDUP ymm256 ymm/m256

### MOVSLDUP
###  - Move Packed Single-FP Low and Duplicate
		val /f3 [0x0f 0x12 /r] = binop sse3 MOVSLDUP xmm128 xmm/m128
		val /vex/f3/0f [0x12 /r]
		 | vex128? = varity2 avx VMOVSLDUP xmm128 xmm/m128
		 | vex256? = varity2 avx VMOVSLDUP ymm256 ymm/m256

### MOVSS
###  - Move Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0x10 /r] = binop sse MOVSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x10 /r-reg] = varity3 avx VMOVSS xmm128 v/xmm xmm/reg128
		val /vex/f3/0f [0x10 /r-mem] = varity2 avx VMOVSS xmm128 m32
		val /f3 [0x0f 0x11 /r] = binop sse MOVSS xmm/m32 xmm128
		val /vex/f3/0f/vexv [0x11 /r-reg] = varity3 avx VMOVSS xmm/reg128 v/xmm xmm128
		val /vex/f3/0f [0x11 /r-mem] = varity2 avx VMOVSS m32 xmm128

### MOVSX/MOVSXD
###  - Move with Sign-Extension
		val / [0x0f 0xbe /r]
		 | rexw? = binop none MOVSX r64 r/m8
		 | opndsz? = binop none MOVSX r16 r/m8
		 | otherwise = binop none MOVSX r32 r/m8
		val / [0x0f 0xbf /r]
		 | rexw? = binop none MOVSX r64 r/m16
		 | opndsz? = binop none MOVSX r16 r/m16
		 | otherwise = binop none MOVSX r32 r/m16
		val / [0x63 /r] #Partially listed in manual; the following lines resulted from test cases
		 | rexw? = binop none MOVSXD r64 r/m32
		 | mode64? && opndsz? = binop none MOVSXD r16 r/m32
		 | mode64? = binop none MOVSXD r32 r/m32
		 | mode32? = binop none ARPL r/m16 r16

### MOVUPD
###  - Move Unaligned Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x10 /r] = binop sse2 MOVUPD xmm128 xmm/m128
		val /vex/66/0f [0x10 /r]
		 | vex128? = varity2 avx VMOVUPD xmm128 xmm/m128
		 | vex256? = varity2 avx VMOVUPD ymm256 ymm/m256
		val /66 [0x0f 0x11 /r] = binop sse2 MOVUPD xmm/m128 xmm128
		val /vex/66/0f [0x11 /r]
		 | vex128? = varity2 avx VMOVUPD xmm/m128 xmm128
		 | vex256? = varity2 avx VMOVUPD ymm/m256 ymm256

### MOVUPS
###  - Move Unaligned Packed Single-Precision Floating-Point Values
		val / [0x0f 0x10 /r] = binop sse MOVUPS xmm128 xmm/m128
		val /vex/0f [0x10 /r]
		 | vex128? = varity2 avx VMOVUPS xmm128 xmm/m128
		 | vex256? = varity2 avx VMOVUPS ymm256 ymm/m256
		val / [0x0f 0x11 /r] = binop sse MOVUPS xmm/m128 xmm128
		val /vex/0f [0x11 /r]
		 | vex128? = varity2 avx VMOVUPS xmm/m128 xmm128
		 | vex256? = varity2 avx VMOVUPS ymm/m256 ymm256

### MOVZX
###  - Move with Zero-Extend
		val / [0x0f 0xb6 /r]
		 | rexw? = binop none MOVZX r64 r/m8
		 | opndsz? = binop none MOVZX r16 r/m8
		 | otherwise = binop none MOVZX r32 r/m8
		val / [0x0f 0xb7 /r]
		 | rexw? = binop none MOVZX r64 r/m16
		 | opndsz? = binop none MOVZX r16 r/m16
		 | otherwise = binop none MOVZX r32 r/m16

### MPSADBW
###  - Compute Multiple Packed Sums of Absolute Difference
		val /66 [0x0f 0x3a 0x42 /r] = ternop sse4_1 MPSADBW xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x42 /r] | vex128? = varity4 avx VMPSADBW xmm128 v/xmm xmm/m128 imm8

### MUL
###  - Unsigned Multiply
		val / [0xf6 /4] = unop none MUL r/m8
		val / [0xf7 /4]
		 | rexw? = unop none MUL r/m64
		 | opndsz? = unop none MUL r/m16
		 | otherwise = unop none MUL r/m32

### MULPD
###  - Multiply Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x59 /r] = binop sse2 MULPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x59 /r]
		 | vex128? = varity3 avx VMULPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VMULPD ymm256 v/ymm ymm/m256

### MULPS
###  - Multiply Packed Single-Precision Floating-Point Values
		val / [0x0f 0x59 /r] = binop sse MULPS xmm128 xmm/m128
		val /vex/0f/vexv [0x59 /r]
		 | vex128? = varity3 avx VMULPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VMULPS ymm256 v/ymm ymm/m256

### MULSD
###  - Multiply Scalar Double-Precision Floating-Point Values
		val /f2 [0x0f 0x59 /r] = binop sse2 MULSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x59 /r] = varity3 avx VMULSD xmm128 v/xmm xmm/m64

### MULSS
###  - Multiply Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0x59 /r] = binop sse MULSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x59 /r] = varity3 avx VMULSS xmm128 v/xmm xmm/m32

### MWAIT
###  - Monitor Wait
		val / [0x0f 0x01 0xc9] = arity0 none MWAIT

### NEG
###  - Two's Complement Negation
		val / [0xf6 /3] = unop-lock none NEG r/m8
		val / [0xf7 /3]
		 | rexw? = unop-lock none NEG r/m64
		 | opndsz? = unop-lock none NEG r/m16
		 | otherwise = unop-lock none NEG r/m32

### NOP
###  - No Operation
# The opcode `0x90` overlapps with `xchg` since
# 90 = xchg eax, eax; but consider 664590 = xchg ax,r8l!
# so we deocde 0x90 always as `xchg`
#val / [0x90] = arity0 NOP => See XCHG
#val /66 [0x90] = arity0 NOP
		val / [0x0f 0x1f /0]
		 | rexw? = varity1 none NOP r/m64
		 | opndsz? = varity1 none NOP r/m16
		 | otherwise = varity1 none NOP r/m32

### NOT
###  - One's Complement Negation
		val / [0xf6 /2] = unop-lock none NOT r/m8
		val / [0xf7 /2]
		 | rexw? = unop-lock none NOT r/m64
		 | opndsz? = unop-lock none NOT r/m16
		 | otherwise = unop-lock none NOT r/m32

### OR
###  - Logical Inclusive OR
		val / [0x0c] = binop none OR al imm8
		val / [0x0d]
		 | rexw? = binop none OR rax imm32
		 | opndsz? = binop none OR ax imm16
		 | otherwise = binop none OR eax imm32
		val / [0x80 /1] = binop-lock none OR r/m8 imm8
		val / [0x81 /1]
		 | rexw? = binop-lock none OR r/m64 imm32
		 | opndsz? = binop-lock none OR r/m16 imm16
		 | otherwise = binop-lock none OR r/m32 imm32
		val / [0x83 /1]
		 | rexw? = binop-lock none OR r/m64 imm8
		 | opndsz? = binop-lock none OR r/m16 imm8
		 | otherwise = binop-lock none OR r/m32 imm8
		val / [0x08 /r] = binop-lock none OR r/m8 r8
		val / [0x09 /r]
		 | rexw? = binop-lock none OR r/m64 r64
		 | opndsz? = binop-lock none OR r/m16 r16
		 | otherwise = binop-lock none OR r/m32 r32
		val / [0x0a /r] = binop none OR r8 r/m8
		val / [0x0b /r]
		 | rexw? = binop none OR r64 r/m64
		 | opndsz? = binop none OR r16 r/m16
		 | otherwise = binop none OR r32 r/m32

### ORPD
###  - Bitwise Logical OR of Double-Precision Floating-Point Values
		val /66 [0x0f 0x56 /r] = binop sse2 ORPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x56 /r]
		 | vex128? = varity3 avx VORPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VORPD ymm256 v/ymm ymm/m256

### ORPS
###  - Bitwise Logical OR of Single-Precision Floating-Point Values
		val / [0x0f 0x56 /r] = binop sse ORPS xmm128 xmm/m128
		val /vex/0f/vexv [0x56 /r]
		 | vex128? = varity3 avx VORPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VORPS ymm256 v/ymm ymm/m256

### OUT
###  - Output to Port
		val / [0xe6] = binop none OUT imm8 al
		val / [0xe7]
		 | opndsz? = binop none OUT imm8 ax
		 | otherwise = binop none OUT imm8 eax
		val / [0xee] = binop none OUT dx al
		val / [0xef]
		 | opndsz? = binop none OUT dx ax
		 | otherwise = binop none OUT dx eax

### OUTS/OUTSB/OUTSW/OUTSD
###  - Output String to Port
# Fix: SI ~ m8?
#val / [0x6e] = binop-rep OUTS dx (mem (REG SI))
#val / [0x6f]
# | opndsz? = binop-rep OUTS dx (mem (REG SI))
# | otherwise = binop-rep OUTS dx (mem (REG ESI))
		val / [0x6e] = arity0-rep none OUTSB
		val / [0x6f]
		 | opndsz? = arity0-rep none OUTSW
		 | otherwise = arity0-rep none OUTSD

### PABSB/PABSW/PABSD
###  - Packed Absolute Value
		val / [0x0f 0x38 0x1c /r] = binop ssse3 PABSB mm64 mm/m64
		val /66 [0x0f 0x38 0x1c /r] = binop ssse3 PABSB xmm128 xmm/m128
		val / [0x0f 0x38 0x1d /r] = binop ssse3 PABSW mm64 mm/m64
		val /66 [0x0f 0x38 0x1d /r] = binop ssse3 PABSW xmm128 xmm/m128
		val / [0x0f 0x38 0x1e /r] = binop ssse3 PABSD mm64 mm/m64
		val /66 [0x0f 0x38 0x1e /r] = binop ssse3 PABSD xmm128 xmm/m128
		val /vex/66/0f/38 [0x1c /r] | vex128? = varity2 avx VPABSB xmm128 xmm/m128
		val /vex/66/0f/38 [0x1d /r] | vex128? = varity2 avx VPABSW xmm128 xmm/m128
		val /vex/66/0f/38 [0x1e /r] | vex128? = varity2 avx VPABSD xmm128 xmm/m128

### PACKSSWB/PACKSSDW
###  - Pack with Signed Saturation
		val / [0x0f 0x63 /r] = binop mmx PACKSSWB mm64 mm/m64
		val /66 [0x0f 0x63 /r] = binop sse2 PACKSSWB xmm128 xmm/m128
		val / [0x0f 0x6b /r] = binop mmx PACKSSDW mm64 mm/m64
		val /66 [0x0f 0x6b /r] = binop sse2 PACKSSDW xmm128 xmm/m128
		val /vex/66/0f/vexv [0x63 /r] | vex128? = varity3 avx VPACKSSWB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x6b /r] | vex128? = varity3 avx VPACKSSDW xmm128 v/xmm xmm/m128

### PACKUSDW
###  - Pack with Unsigned Saturation
		val /66 [0x0f 0x38 0x2b /r] = binop sse4_1 PACKUSDW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x2b /r] | vex128? = varity3 avx VPACKUSDW xmm128 v/xmm xmm/m128

### PACKUSWB
###  - Pack with Unsigned Saturation
		val / [0x0f 0x67 /r] = binop mmx PACKUSWB mm64 mm/m64
		val /66 [0x0f 0x67 /r] = binop sse2 PACKUSWB xmm128 xmm/m128
		val /vex/66/0f/vexv [0x67 /r] | vex128? = varity3 avx VPACKUSWB xmm128 v/xmm xmm/m128

### PADDB/PADDW/PADDD
###  - Add Packed Integers
		val / [0x0f 0xfc /r] = binop mmx PADDB mm64 mm/m64
		val /66 [0x0f 0xfc /r] = binop sse2 PADDB xmm128 xmm/m128
		val / [0x0f 0xfd /r] = binop mmx PADDW mm64 mm/m64
		val /66 [0x0f 0xfd /r] = binop sse2 PADDW xmm128 xmm/m128
		val / [0x0f 0xfe /r] = binop mmx PADDD mm64 mm/m64
		val /66 [0x0f 0xfe /r] = binop sse2 PADDD xmm128 xmm/m128
		val /vex/66/0f/vexv [0xfc /r] | vex128? = varity3 avx VPADDB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xfd /r] | vex128? = varity3 avx VPADDW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xfe /r] | vex128? = varity3 avx VPADDD xmm128 v/xmm xmm/m128

### PADDQ
###  - Add Packed Quadword Integers
		val / [0x0f 0xd4 /r] = binop sse2 PADDQ mm64 mm/m64
		val /66 [0x0f 0xd4 /r] = binop sse2 PADDQ xmm128 xmm/m128
		val /vex/66/0f/vexv [0xd4 /r] | vex128? = varity3 avx VPADDQ xmm128 v/xmm xmm/m128

### PADDSB/PADDSW
###  - Add Packed Signed Integers with Signed Saturation
		val / [0x0f 0xec /r] = binop mmx PADDSB mm64 mm/m64
		val /66 [0x0f 0xec /r] = binop sse2 PADDSB xmm128 xmm/m128
		val / [0x0f 0xed /r] = binop mmx PADDSW mm64 mm/m64
		val /66 [0x0f 0xed /r] = binop sse2 PADDSW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xec /r] | vex128? = varity3 avx VPADDSB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xed /r] | vex128? = varity3 avx VPADDSW xmm128 v/xmm xmm/m128

### PADDUSB/PADDUSW
###  - Add Packed Unsigned Integers with Unsigned Saturation
		val / [0x0f 0xdc /r] = binop mmx PADDUSB mm64 mm/m64
		val /66 [0x0f 0xdc /r] = binop sse2 PADDUSB xmm128 xmm/m128
		val / [0x0f 0xdd /r] = binop mmx PADDUSW mm64 mm/m64
		val /66 [0x0f 0xdd /r] = binop sse2 PADDUSW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xdc /r] | vex128? = varity3 avx VPADDUSB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xdd /r] | vex128? = varity3 avx VPADDUSW xmm128 v/xmm xmm/m128

### PALIGNR
###  - Packed Align Right
		val / [0x0f 0x3a 0x0f /r] = ternop ssse3 PALIGNR mm64 mm/m64 imm8
		val /66 [0x0f 0x3a 0x0f /r] = ternop ssse3 PALIGNR xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x0f /r] | vex128? = varity4 avx VPALIGNR xmm128 v/xmm xmm/m128 imm8

### PAND
###  - Logical AND
		val / [0x0f 0xdb /r] = binop mmx PAND mm64 mm/m64
		val /66 [0x0f 0xdb /r] = binop sse2 PAND xmm128 xmm/m128
		val /vex/66/0f/vexv [0xdb /r] | vex128? = varity3 avx VPAND xmm128 v/xmm xmm/m128

### PANDN
###  - Logical AND NOT
		val / [0x0f 0xdf /r] = binop mmx PANDN mm64 mm/m64
		val /66 [0x0f 0xdf /r] = binop sse2 PANDN xmm128 xmm/m128
		val /vex/66/0f/vexv [0xdf /r] | vex128? = varity3 avx VPANDN xmm128 v/xmm xmm/m128

### PAUSE
###  - Spin Loop Hint
		val /f3 [0x90] = arity0 none PAUSE

### PAVGB/PAVGW
###  - Average Packed Integers
		val / [0x0f 0xe0 /r] = binop sse PAVGB mm64 mm/m64
		val /66 [0x0f 0xe0 /r] = binop sse2 PAVGB xmm128 xmm/m128
		val / [0x0f 0xe3 /r] = binop sse PAVGW mm64 mm/m64
		val /66 [0x0f 0xe3 /r] = binop sse2 PAVGW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xe0 /r] | vex128? = varity3 avx VPAVGB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xe3 /r] | vex128? = varity3 avx VPAVGW xmm128 v/xmm xmm/m128

### PBLENDVB
###  - Variable Blend Packed Bytes
# Todo: /is4? (possible meaning: 4 bits of the immediate are used to select register?)
		val /66 [0x0f 0x38 0x10 /r] = binop sse4_1 PBLENDVB xmm128 xmm/m128
		val /vex/66/0f/3a/vexv [0x4c /r] | vexw0? && vex128? = varity4 avx VPBLENDVB xmm128 v/xmm xmm/m128 imm/xmm

### PBLENDW
###  - Blend Packed Words
		val /66 [0x0f 0x3a 0x0e /r] = ternop sse4_1 PBLENDW xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x0e /r] | vex128? = varity4 avx VPBLENDW xmm128 v/xmm xmm/m128 imm8

### PCLMULQDQ
###  - Carry-Less Multiplication Quadword
		val /66 [0x0f 0x3a 0x44 /r] = ternop clmul PCLMULQDQ xmm128 xmm/m128 imm8
		val /vex/66/0f/3a/vexv [0x44 /r] | vex128? = varity4 (clmul or avx) VPCLMULQDQ xmm128 v/xmm xmm/m128 imm8

### PCMPEQB/PCMPEQW/PCMPEQD
###  - Compare Packed Data for Equal
		val / [0x0f 0x74 /r] = binop mmx PCMPEQB mm64 mm/m64
		val /66 [0x0f 0x74 /r] = binop sse2 PCMPEQB xmm128 xmm/m128
		val / [0x0f 0x75 /r] = binop mmx PCMPEQW mm64 mm/m64
		val /66 [0x0f 0x75 /r] = binop sse2 PCMPEQW xmm128 xmm/m128
		val / [0x0f 0x76 /r] = binop mmx PCMPEQD mm64 mm/m64
		val /66 [0x0f 0x76 /r] = binop sse2 PCMPEQD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x74 /r] | vex128? = varity3 avx VPCMPEQB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x75 /r] | vex128? = varity3 avx VPCMPEQW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x76 /r] | vex128? = varity3 avx VPCMPEQD xmm128 v/xmm xmm/m128

### PCMPEQQ
###  - Compare Packed Qword Data for Equal
		val /66 [0x0f 0x38 0x29 /r] = binop sse4_1 PCMPEQQ xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x29 /r] | vex128? = varity3 avx VPCMPEQQ xmm128 v/xmm xmm/m128

### PCMPESTRI
###  - Packed Compare Explicit Length Strings, Return Index
		val /66 [0x0f 0x3a 0x61 /r] = ternop sse4_2 PCMPESTRI xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0x61 /r] = varity3 avx VPCMPESTRI xmm128 xmm/m128 imm8

### PCMPESTRM
###  - Packed Compare Explicit Length Strings, Return Mask
		val /66 [0x0f 0x3a 0x60 /r] = ternop sse4_2 PCMPESTRM xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0x60 /r] | vex128? = varity3 avx VPCMPESTRM xmm128 xmm/m128 imm8

### PCMPGTB/PCMPGTW/PCMPGTD
###  - Compare Packed Signed Integers for Greater Than
		val / [0x0f 0x64 /r] = binop mmx PCMPGTB mm64 mm/m64
		val /66 [0x0f 0x64 /r] = binop sse2 PCMPGTB xmm128 xmm/m128
		val / [0x0f 0x65 /r] = binop mmx PCMPGTW mm64 mm/m64
		val /66 [0x0f 0x65 /r] = binop sse2 PCMPGTW xmm128 xmm/m128
		val / [0x0f 0x66 /r] = binop mmx PCMPGTD mm64 mm/m64
		val /66 [0x0f 0x66 /r] = binop sse2 PCMPGRD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x64 /r] | vex128? = varity3 avx VPCMPGTB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x65 /r] | vex128? = varity3 avx VPCMPGTW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x66 /r] | vex128? = varity3 avx VPCMPGTD xmm128 v/xmm xmm/m128

### PCMPGTQ
###  - Compare Packed Data for Greater Than
		val /66 [0x0f 0x38 0x37 /r] = binop sse4_2 PCMPGTQ xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x37 /r] | vex128? = varity3 avx VPCMPGTQ xmm128 v/xmm xmm/m128

### PCMPISTRI
###  - Packed Compare Implicit Length Strings, Return Index
		val /66 [0x0f 0x3a 0x63 /r] = ternop sse4_2 PCMPISTRI xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0x63 /r] | vex128? = varity3 avx VPCMPISTRI xmm128 xmm/m128 imm8

### PCMPISTRM
###  - Packed Compare Implicit Length Strings, Return Mask
		val /66 [0x0f 0x3a 0x62 /r] = ternop sse4_2 PCMPISTRM xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0x62 /r] | vex128? = varity3 avx VPCMPISTRM xmm128 xmm/m128 imm8

### PEXTRB/PEXTRD/PEXTRQ
###  - Extract Byte/Dword/Qword
		val /66 [0x0f 0x3a 0x14 /r]
		 | mode64? = ternop sse4_1 PEXTRB r64/m8 xmm128 imm8
		 | otherwise = ternop sse4_1 PEXTRB r32/m8 xmm128 imm8
		val /66 [0x0f 0x3a 0x16 /r]
		 | rexw? = ternop sse4_1 PEXTRQ r/m64 xmm128 imm8
		 | otherwise = ternop sse4_1 PEXTRD r/m32 xmm128 imm8
		val /vex/66/0f/3a [0x14 /r]
		 | vex128? && mode64? = varity3 avx VPEXTRB r64/m8 xmm128 imm8 #Footnote: vexw1 is ignored
		 | vex128? && vexw0? = varity3 avx VPEXTRB r32/m8 xmm128 imm8
		val /vex/66/0f/3a [0x16 /r]
		 | vex128? && vexw0? = varity3 avx VPEXTRD r/m32 xmm128 imm8
		 | vex128? && vexw1? = varity3 avx VPEXTRQ r/m64 xmm128 imm8

### PEXTRW
###  - Extract Word
		val / [0x0f 0xc5 /r-reg]
		 | mode64? = ternop sse PEXTRW r64 mm/reg64 imm8
		 | otherwise = ternop sse PEXTRW r32 mm/reg64 imm8
		val /66 [0x0f 0xc5 /r-reg]
		 | mode64? = ternop sse2 PEXTRW r64 xmm/reg128 imm8
		 | otherwise = ternop sse2 PEXTRW r32 xmm/reg128 imm8
		val /66 [0x0f 0x3a 0x15 /r]
		 | mode64? = ternop sse4_1 PEXTRW r64/m16 xmm128 imm8
		 | otherwise = ternop sse4_1 PEXTRW r32/m16 xmm128 imm8
		val /vex/66/0f [0xc5 /r-reg]
		 | vex128? && mode64? = varity3 avx VPEXTRW r64 xmm/reg128 imm8 #Footnote: vexw1 is ignored
		 | vex128? && vexw0? = varity3 avx VPEXTRW r32 xmm/reg128 imm8
		val /vex/66/0f/3a [0x15 /r]
		 | vex128? && mode64? && vexw0? = varity3 avx VPEXTRW r64/m16 xmm128 imm8
		 | vex128? && vexw0? = varity3 avx VPEXTRW r32/m16 xmm128 imm8

### PHADDW/PHADDD
###  - Packed Horizontal Add
		val / [0x0f 0x38 0x01 /r] = binop ssse3 PHADDW mm64 mm/m64
		val /66 [0x0f 0x38 0x01 /r] = binop ssse3 PHADDW xmm128 xmm/m128
		val / [0x0f 0x38 0x02 /r] = binop ssse3 PHADDD mm64 mm/m64
		val /66 [0x0f 0x38 0x02 /r] = binop ssse3 PHADDD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x01 /r] | vex128? = varity3 avx VPHADDW xmm128 v/xmm xmm/m128
		val /vex/66/0f/38/vexv [0x02 /r] | vex128? = varity3 avx VPHADDD xmm128 v/xmm xmm/m128

### PHADDSW
###  - Packed Horizontal Add and Saturate
		val / [0x0f 0x38 0x03 /r] = binop ssse3 PHADDSW mm64 mm/m64
		val /66 [0x0f 0x38 0x03 /r] = binop ssse3 PHADDSW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x03 /r] | vex128? = varity3 avx VPHADDSW xmm128 v/xmm xmm/m128

### PHMINPOSUW
###  - Packed Horizontal Word Minimum
		val /66 [0x0f 0x38 0x41 /r] = binop sse4_1 PHMINPOSUW xmm128 xmm/m128
		val /vex/66/0f/38 [0x41 /r] | vex128? = varity2 avx VPHMINPOSUW xmm128 xmm/m128

### PHSUBW/PHSUBD
###  - Packed Horizontal Subtract
		val / [0x0f 0x38 0x05 /r] = binop ssse3 PHSUBW mm64 mm/m64
		val /66 [0x0f 0x38 0x05 /r] = binop ssse3 PHSUBW xmm128 xmm/m128
		val / [0x0f 0x38 0x06 /r] = binop ssse3 PHSUBD mm64 mm/m64
		val /66 [0x0f 0x38 0x06 /r] = binop ssse3 PHSUBD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x05 /r] | vex128? = varity3 avx VPHSUBW xmm128 v/xmm xmm/m128
		val /vex/66/0f/38/vexv [0x06 /r] | vex128? = varity3 avx VPHSUBD xmm128 v/xmm xmm/m128

### PHSUBSW
###  - Packed Horizontal Subtract and Saturate
		val / [0x0f 0x38 0x07 /r] = binop ssse3 PHSUBSW mm64 mm/m64
		val /66 [0x0f 0x38 0x07 /r] = binop ssse3 PHSUBSW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x07 /r] | vex128? = varity3 avx VPHSUBSW xmm128 v/xmm xmm/m128

### PINSRB/PINSRD/PINSRQ
###  - Insert Byte/Dword/Qword
		val /66 [0x0f 0x3a 0x20 /r] = ternop sse4_1 PINSRB xmm128 r32/m8 imm8
		val /66 [0x0f 0x3a 0x22 /r]
		 | rexw? = ternop sse4_1 PINSRQ xmm128 r/m64 imm8
		 | otherwise = ternop sse4_1 PINSRD xmm128 r/m32 imm8
		val /vex/66/0f/3a/vexv [0x20 /r]
		 | vex128? && (mode64? || vexw0?) = varity4 avx VPINSRB xmm128 v/xmm r32/m8 imm8
		val /vex/66/0f/3a/vexv [0x22 /r]
		 | vex128? && vexw0? = varity4 avx VPINSRD xmm128 v/xmm r/m32 imm8
		 | vex128? && vexw1? = varity4 avx VPINSRQ xmm128 v/xmm r/m64 imm8

### PINSRW
###  - Insert Word
		val / [0x0f 0xc4 /r] = ternop sse PINSRW mm64 r32/m16 imm8
		val /66 [0x0f 0xc4 /r] = ternop sse2 PINSRW xmm128 r32/m16 imm8
		val /vex/66/0f/vexv [0xc4 /r]
		 | vex128? && (mode64? || vexw0?) = varity4 avx VPINSRW xmm128 v/xmm r32/m16 imm8

### PMADDUBSW
###  - Multiply and Add Packed Signed and Unsigned Bytes
		val / [0x0f 0x38 0x04 /r] = binop mmx PMADDUBSW mm64 mm/m64
		val /66 [0x0f 0x38 0x04 /r] = binop ssse3 PMADDUBSW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x04 /r] | vex128? = varity3 avx VPMADDUBSW xmm128 v/xmm xmm/m128

### PMADDWD
###  - Multiply and Add Packed Integers
		val / [0x0f 0xf5 /r] = binop mmx PMADDWD mm64 mm/m64
		val /66 [0x0f 0xf5 /r] = binop sse2 PMADDWD xmm128 xmm/m128
		val /vex/66/0f/vexv [0xf5 /r] | vex128? = varity3 avx VPMADDWD xmm128 v/xmm xmm/m128

### PMAXSB
###  - Maximum of Packed Signed Byte Integers
		val /66 [0x0f 0x38 0x3c /r] = binop sse4_1 PMAXSB xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x3c /r] | vex128? = varity3 avx VPMAXSB xmm128 v/xmm xmm/m128

### PMAXSD
###  - Maximum of Packed Signed Dword Integers
		val /66 [0x0f 0x38 0x3d /r] = binop sse4_1 PMAXSD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x3d /r] | vex128? = varity3 avx VPMAXSD xmm128 v/xmm xmm/m128

### PMAXSW
###  - Maximum of Packed Signed Word Integers
		val / [0x0f 0xee /r] = binop sse PMAXSW mm64 mm/m64
		val /66 [0x0f 0xee /r] = binop sse2 PMAXSW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xee /r] | vex128? = varity3 avx VPMAXSW xmm128 v/xmm xmm/m128

### PMAXUB
###  - Maximum of Packed Unsigned Byte Integers
		val / [0x0f 0xde /r] = binop sse PMAXUB mm64 mm/m64
		val /66 [0x0f 0xde /r] = binop sse2 PMAXUB xmm128 xmm/m128
		val /vex/66/0f/vexv [0xde /r] | vex128? = varity3 avx VPMAXUB xmm128 v/xmm xmm/m128

### PMAXUD
###  - Maximum of Packed Unsigned Dword Integers
		val /66 [0x0f 0x38 0x3f /r] = binop sse4_1 PMAXUD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x3f /r] | vex128? = varity3 avx VPMAXUD xmm128 v/xmm xmm/m128

### PMAXUW
###  - Maximum of Packed Word Integers
		val /66 [0x0f 0x38 0x3e /r] = binop sse4_1 PMAXUW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x3e /r] | vex128? = varity3 avx VPMAXUW xmm128 v/xmm xmm/m128

### PMINSB
###  - Minimum of Packed Signed Byte Integers
		val /66 [0x0f 0x38 0x38 /r] = binop sse4_1 PMINSB xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x38 /r] | vex128? = varity3 avx VPMINSB xmm128 v/xmm xmm/m128

### PMINSD
###  - Minimum of Packed Dword Integers
		val /66 [0x0f 0x38 0x39 /r] = binop sse4_1 PMINSD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x39 /r] | vex128? = varity3 avx VPMINSD xmm128 v/xmm xmm/m128

### PMINSW
###  - Minimum of Packed Signed Word Integers
		val / [0x0f 0xea /r] = binop sse PMINSW mm64 mm/m64
		val /66 [0x0f 0xea /r] = binop sse2 PMINSW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xea /r] | vex128? = varity3 avx VPMINSW xmm128 v/xmm xmm/m128

### PMINUB
###  - Minimum of Packed Unsigned Byte Integers
		val / [0x0f 0xda /r] = binop sse PMINUB mm64 mm/m64
		val /66 [0x0f 0xda /r] = binop sse2 PMINUB xmm128 xmm/m128
		val /vex/66/0f/vexv [0xda /r] | vex128? = varity3 avx VPMINUB xmm128 v/xmm xmm/m128

### PMINUD
###  - Minimum of Packed Dword Integers
		val /66 [0x0f 0x38 0x3b /r] = binop sse4_1 PMINUD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x3b /r] | vex128? = varity3 avx VPMINUD xmm128 v/xmm xmm/m128

### PMINUW
###  - Minimum of Packed Word Integers
		val /66 [0x0f 0x38 0x3a /r] = binop sse4_1 PMINUW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x3a /r] | vex128? = varity3 avx VPMINUW xmm128 v/xmm xmm/m128

### PMOVMSKB
###  - Move Byte Mask
		val / [0x0f 0xd7 /r-reg]
		 | mode64? = binop sse PMOVMSKB r64 mm/reg64
		 | otherwise = binop sse PMOVMSKB r32 mm/reg64
		val /66 [0x0f 0xd7 /r-reg]
		 | mode64? = binop sse2 PMOVMSKB r64 xmm/reg128
		 | otherwise = binop sse2 PMOVMSKB r32 xmm/reg128
		val /vex/66/0f [0xd7 /r-reg]
		 | vex128? && mode64? = varity2 avx VPMOVMSKB r64 xmm/reg128
		 | vex128? = varity2 avx VPMOVMSKB r32 xmm/reg128

### PMOVSX
###  - Packed Move with Sign Extend
		val /66 [0x0f 0x38 0x20 /r] = binop sse4_1 PMOVSXBW xmm128 xmm/m64
		val /66 [0x0f 0x38 0x21 /r] = binop sse4_1 PMOVSXBD xmm128 xmm/m32
		val /66 [0x0f 0x38 0x22 /r] = binop sse4_1 PMOVSXBQ xmm128 xmm/m16
		val /66 [0x0f 0x38 0x23 /r] = binop sse4_1 PMOVSXWD xmm128 xmm/m64
		val /66 [0x0f 0x38 0x24 /r] = binop sse4_1 PMOVSXWQ xmm128 xmm/m32
		val /66 [0x0f 0x38 0x25 /r] = binop sse4_1 PMOVSXDQ xmm128 xmm/m64
		val /vex/66/0f/38 [0x20 /r] | vex128? = varity2 avx VPMOVSXBW xmm128 xmm/m64
		val /vex/66/0f/38 [0x21 /r] | vex128? = varity2 avx VPMOVSXBD xmm128 xmm/m32
		val /vex/66/0f/38 [0x22 /r] | vex128? = varity2 avx VPMOVSXBQ xmm128 xmm/m16
		val /vex/66/0f/38 [0x23 /r] | vex128? = varity2 avx VPMOVSXWD xmm128 xmm/m64
		val /vex/66/0f/38 [0x24 /r] | vex128? = varity2 avx VPMOVSXWQ xmm128 xmm/m32
		val /vex/66/0f/38 [0x25 /r] | vex128? = varity2 avx VPMOVSXDQ xmm128 xmm/m64

### PMOVZX
###  - Packed Move with Zero Extend
		val /66 [0x0f 0x38 0x30 /r] = binop sse4_1 PMOVZXBW xmm128 xmm/m64
		val /66 [0x0f 0x38 0x31 /r] = binop sse4_1 PMOVZXBD xmm128 xmm/m32
		val /66 [0x0f 0x38 0x32 /r] = binop sse4_1 PMOVZXBQ xmm128 xmm/m16
		val /66 [0x0f 0x38 0x33 /r] = binop sse4_1 PMOVZXWD xmm128 xmm/m64
		val /66 [0x0f 0x38 0x34 /r] = binop sse4_1 PMOVZXWQ xmm128 xmm/m32
		val /66 [0x0f 0x38 0x35 /r] = binop sse4_1 PMOVZXDQ xmm128 xmm/m64
		val /vex/66/0f/38 [0x30 /r] | vex128? = varity2 avx VPMOVZXBW xmm128 xmm/m64
		val /vex/66/0f/38 [0x31 /r] | vex128? = varity2 avx VPMOVZXBD xmm128 xmm/m32
		val /vex/66/0f/38 [0x32 /r] | vex128? = varity2 avx VPMOVZXBQ xmm128 xmm/m16
		val /vex/66/0f/38 [0x33 /r] | vex128? = varity2 avx VPMOVZXWD xmm128 xmm/m64
		val /vex/66/0f/38 [0x34 /r] | vex128? = varity2 avx VPMOVZXWQ xmm128 xmm/m32
		val /vex/66/0f/38 [0x35 /r] | vex128? = varity2 avx VPMOVZXDQ xmm128 xmm/m64

### PMULDQ
###  - Multiply Packed Signed Dword Integers
		val /66 [0x0f 0x38 0x28 /r] = binop sse4_1 PMULDQ xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x28 /r] | vex128? = varity3 avx VPMULDQ xmm128 v/xmm xmm/m128

### PMULHRSW
###  - Packed Multiply High with Round and Scale
		val / [0x0f 0x38 0x0b /r] = binop ssse3 PMULHRSW mm64 mm/m64
		val /66 [0x0f 0x38 0x0b /r] = binop ssse3 PMULHRSW xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x0b /r] | vex128? = varity3 avx VPMULHRSW xmm128 v/xmm xmm/m128

### PMULHUW
###  - Multiply Packed Unsigned Integers and Store High Result
		val / [0x0f 0xe4 /r] = binop sse PMULHUW mm64 mm/m64
		val /66 [0x0f 0xe4 /r] = binop sse2 PMULHUW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xe4 /r] | vex128? = varity3 avx VPMULHUW xmm128 v/xmm xmm/m128

### PMULHW
###  - Multiply Packed Signed Integers and Store High Result
		val / [0x0f 0xe5 /r] = binop mmx PMULHW mm64 mm/m64
		val /66 [0x0f 0xe5 /r] = binop sse2 PMULHW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xe5 /r] | vex128? = varity3 avx VPMULHW xmm128 v/xmm xmm/m128

### PMULLD
###  - Multiply Packed Signed Dword Integers and Store Low Result
		val /66 [0x0f 0x38 0x40 /r] = binop sse4_1 PMULLD xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x40 /r] = varity3 avx VPMULLD xmm128 v/xmm xmm/m128

### PMULLW
###  - Multiply Packed Signed Integers and Store Low Result
		val / [0x0f 0xd5 /r] = binop mmx PMULLW mm64 mm/m64
		val /66 [0x0f 0xd5 /r] = binop sse2 PMULLW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xd5 /r] = varity3 avx VPMULLW xmm128 v/xmm xmm/m128

### PMULUDQ
###  - Multiply Packed Unsigned Doubleword Integers
		val / [0x0f 0xf4 /r] = binop sse2 PMULUDQ mm64 mm/m64
		val /66 [0x0f 0xf4 /r] = binop sse2 PMULUDQ xmm128 xmm/m128
		val /vex/66/0f/vexv [0xf4 /r] = varity3 avx VPMULUDQ xmm128 v/xmm xmm/m128

### POP
###  - Pop a Value from the Stack
		val / [0x8f /0]
		 | opndsz? = do opndsz-set-from-d; unop none POP r/m16 end
		 | mode32? = do opndsz-set-from-d; unop none POP r/m32 end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none POP r/m64 end
		val / ['01011 r:3']
		 | opndsz? = do opndsz-set-from-d; update@{reg/opcode=r}; unop none POP r16/rexb end
		 | mode32? = do opndsz-set-from-d; update@{reg/opcode=r}; unop none POP r32/rexb end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; update@{reg/opcode=r}; unop none POP r64/rexb end
		val / [0x1f] | mode32? = do opndsz-set-from-d; unop none POP ds end
		val / [0x07] | mode32? = do opndsz-set-from-d; unop none POP es end
		val / [0x17] | mode32? = do opndsz-set-from-d; unop none POP ss end
		val / [0x0f 0xa1]
		 | opndsz? = do opndsz-set-from-d; unop none POP fs end
		 | mode32? = do opndsz-set-from-d; unop none POP fs end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none POP fs end
		val / [0x0f 0xa9]
		 | opndsz? = do opndsz-set-from-d; unop none POP gs end
		 | mode32? = do opndsz-set-from-d; unop none POP gs end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none POP gs end

### POPA/POPAD
###  - Pop All General-Purpose Registers
		val / [0x61]
		 | mode32? && opndsz? = arity0 none POPA
		 | mode32? = arity0 none POPAD

### POPCNT
###  - Return the Count of Number of Bits Set to 1
		val /f3 [0x0f 0xb8 /r]
		 | rexw? = binop none POPCNT r64 r/m64
		 | opndsz? = binop none POPCNT r16 r/m16
		 | otherwise = binop none POPCNT r32 r/m32

### POPF/POPFD/POPFQ
###  - Pop Stack into EFLAGS Register
		val / [0x9d]
		 | rexw? = arity0 none POPFQ
		 | opndsz? = arity0 none POPF
		 | mode32? = arity0 none POPFD

### POR
###  - Bitwise Logical OR
		val / [0x0f 0xeb /r] = binop mmx POR mm64 mm/m64
		val /66 [0x0f 0xeb /r] = binop sse2 POR xmm128 xmm/m128
		val /vex/66/0f/vexv [0xeb /r] | vex128? = varity3 avx VPOR xmm128 v/xmm xmm/m128

#FEATURES ==> <==

### PREFETCHh
###  - Prefetch Data Into Caches
		val / [0x0f 0x18 /1-mem] = unop none PREFETCHT0 m8
		val / [0x0f 0x18 /2-mem] = unop none PREFETCHT1 m8
		val / [0x0f 0x18 /3-mem] = unop none PREFETCHT2 m8
		val / [0x0f 0x18 /0-mem] = unop none PREFETCHNTA m8

### PREFETCHW
###  - This instruction is not part of the intel manual
		val / [0x0f 0x0d /r-mem] = unop none PREFETCHW m8 #Todo: extra feature flag?

### PSADBW
###  - Compute Sum of Absolute Differences
		val / [0x0f 0xf6 /r] = binop sse PSADBW mm64 mm/m64
		val /66 [0x0f 0xf6 /r] = binop sse2 PSADBW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xf6 /r] | vex128? = varity3 avx VPSADBW xmm128 v/xmm xmm/m128

### PSHUFB
###  - Packed Shuffle Bytes
		val / [0x0f 0x38 0x00 /r] = binop ssse3 PSHUFB mm64 mm/m64
		val /66 [0x0f 0x38 0x00 /r] = binop ssse3 PSHUFB xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x00 /r] | vex128? = varity3 avx VPSHUFB xmm128 v/xmm xmm/m128

### PSHUFD
###  - Shuffle Packed Doublewords
		val /66 [0x0f 0x70 /r] = ternop sse2 PSHUFD xmm128 xmm/m128 imm8
		val /vex/66/0f [0x70 /r] | vex128? = varity3 avx VPSHUFD xmm128 xmm/m128 imm8

### PSHUFHW
###  - Shuffle Packed High Words
		val /f3 [0x0f 0x70 /r] = ternop sse2 PSHUFHW xmm128 xmm/m128 imm8
		val /vex/f3/0f [0x70 /r] | vex128? = varity3 avx VPSHUFHW xmm128 xmm/m128 imm8

### PSHUFLW
###  - Shuffle Packed Low Words
		val /f2 [0x0f 0x70 /r] = ternop sse2 PSHUFLW xmm128 xmm/m128 imm8
		val /vex/f2/0f [0x70 /r] | vex128? = varity3 avx VPSHUFLW xmm128 xmm/m128 imm8

### PSHUFW
###  - Shuffle Packed Words
		val / [0x0f 0x70 /r] = ternop none PSHUFW mm64 mm/m64 imm8

### PSIGNB/PSIGNW/PSIGND
###  - Packed SIGN
		val / [0x0f 0x38 0x08 /r] = binop ssse3 PSIGNB mm64 mm/m64
		val /66 [0x0f 0x38 0x08 /r] = binop ssse3 PSIGNB xmm128 xmm/m128
		val / [0x0f 0x38 0x09 /r] = binop ssse3 PSIGNW mm64 mm/m64
		val /66 [0x0f 0x38 0x09 /r] = binop ssse3 PSIGNW xmm128 xmm/m128
		val / [0x0f 0x38 0x0a /r] = binop ssse3 PSIGND mm64 mm/m64
		val /66 [0x0f 0x38 0x0a /r] = binop ssse3 PSIGND xmm128 xmm/m128
		val /vex/66/0f/38/vexv [0x08 /r] | vex128? = varity3 avx VPSIGNB xmm128 v/xmm xmm/m128
		val /vex/66/0f/38/vexv [0x09 /r] | vex128? = varity3 avx VPSIGNW xmm128 v/xmm xmm/m128
		val /vex/66/0f/38/vexv [0x0a /r] | vex128? = varity3 avx VPSIGND xmm128 v/xmm xmm/m128

### PSLLDQ
###  - Shift Double Quadword Left Logical
		val /66 [0x0f 0x73 /7-reg] = binop sse2 PSLLDQ xmm/reg128 imm8
		val /vex/66/0f/vexv [0x73 /7-reg] | vex128? = varity3 avx VPSLLDQ v/xmm xmm/reg128 imm8

### PSLLW/PSLLD/PSLLQ
###  - Shift Packed Data Left Logical
		val / [0x0f 0xf1 /r] = binop mmx PSLLW mm64 mm/m64
		val /66 [0x0f 0xf1 /r] = binop sse2 PSLLW xmm128 xmm/m128
		val / [0x0f 0x71 /6-reg] = binop mmx PSLLW mm/reg64 imm8
		val /66 [0x0f 0x71 /6-reg] = binop sse2 PSLLW xmm/reg128 imm8
		val / [0x0f 0xf2 /r] = binop mmx PSLLD mm64 mm/m64
		val /66 [0x0f 0xf2 /r] = binop sse2 PSLLD xmm128 xmm/m128
		val / [0x0f 0x72 /6-reg] = binop mmx PSLLD mm/reg64 imm8
		val /66 [0x0f 0x72 /6-reg] = binop sse2 PSLLD xmm/reg128 imm8
		val / [0x0f 0xf3 /r] = binop mmx PSLLQ mm64 mm/m64
		val /66 [0x0f 0xf3 /r] = binop sse2 PSLLQ xmm128 xmm/m128
		val / [0x0f 0x73 /6-reg] = binop mmx PSLLQ mm/reg64 imm8
		val /66 [0x0f 0x73 /6-reg] = binop sse2 PSLLQ xmm/reg128 imm8
		val /vex/66/0f/vexv [0xf1 /r] | vex128? = varity3 avx VPSLLW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x71 /6-reg] | vex128? = varity3 avx VPSLLW v/xmm xmm/reg128 imm8
		val /vex/66/0f/vexv [0xf2 /r] | vex128? = varity3 avx VPSLLD xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x72 /6-reg] | vex128? = varity3 avx VPSLLD v/xmm xmm/reg128 imm8
		val /vex/66/0f/vexv [0xf3 /r] | vex128? = varity3 avx VPSLLQ xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x73 /6-reg] | vex128? = varity3 avx VPSLLQ v/xmm xmm/reg128 imm8

### PSRAW/PSRAD
###  - Shift Packed Data Right Arithmetic
		val / [0x0f 0xe1 /r] = binop mmx PSRAW mm64 mm/m64
		val /66 [0x0f 0xe1 /r] = binop sse2 PSRAW xmm128 xmm/m128
		val / [0x0f 0x71 /4-reg] = binop mmx PSRAW mm/reg64 imm8
		val /66 [0x0f 0x71 /4-reg] = binop sse2 PSRAW xmm/reg128 imm8
		val / [0x0f 0xe2 /r] = binop mmx PSRAD mm64 mm/m64
		val /66 [0x0f 0xe2 /r] = binop sse2 PSRAD xmm128 xmm/m128
		val / [0x0f 0x72 /4-reg] = binop mmx PSRAD mm/reg64 imm8
		val /66 [0x0f 0x72 /4-reg] = binop sse2 PSRAD xmm/reg128 imm8
		val /vex/66/0f/vexv [0xe1 /r] | vex128? = varity3 avx VPSRAW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x71 /4-reg] | vex128? = varity3 avx VPSRAW v/xmm xmm/reg128 imm8
		val /vex/66/0f/vexv [0xe2 /r] | vex128? = varity3 avx VPSRAD xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x72 /4-reg] | vex128? = varity3 avx VPSRAD v/xmm xmm/reg128 imm8

### PSRLDQ
###  - Shift Double Quadword Right Logical
		val /66 [0x0f 0x73 /3-reg] = binop sse2 PSRLDQ xmm/reg128 imm8
		val /vex/66/0f/vexv [0x73 /3-reg] | vex128? = varity3 avx VPSRLDQ v/xmm xmm/reg128 imm8

### PSRLW/PSRLD/PSRLQ
###  - Shift Packed Data Right Logical
		val / [0x0f 0xd1 /r] = binop mmx PSRLW mm64 mm/m64
		val /66 [0x0f 0xd1 /r] = binop sse2 PSRLW xmm128 xmm/m128
		val / [0x0f 0x71 /2-reg] = binop mmx PSRLW mm/reg64 imm8
		val /66 [0x0f 0x71 /2-reg] = binop sse2 PSRLW xmm/reg128 imm8
		val / [0x0f 0xd2 /r] = binop mmx PSRLD mm64 mm/m64
		val /66 [0x0f 0xd2 /r] = binop sse2 PSRLD xmm128 xmm/m128
		val / [0x0f 0x72 /2-reg] = binop mmx PSRLD mm/reg64 imm8
		val /66 [0x0f 0x72 /2-reg] = binop sse2 PSRLD xmm/reg128 imm8
		val / [0x0f 0xd3 /r] = binop mmx PSRLQ mm64 mm/m64
		val /66 [0x0f 0xd3 /r] = binop sse2 PSRLQ xmm128 xmm/m128
		val / [0x0f 0x73 /2-reg] = binop mmx PSRLQ mm/reg64 imm8
		val /66 [0x0f 0x73 /2-reg] = binop sse2 PSRLQ xmm/reg128 imm8
		val /vex/66/0f/vexv [0xd1 /r] | vex128? = varity3 avx VPSRLW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x71 /2-reg] | vex128? = varity3 avx VPSRLW v/xmm xmm/reg128 imm8
		val /vex/66/0f/vexv [0xd2 /r] | vex128? = varity3 avx VPSRLD xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x72 /2-reg] | vex128? = varity3 avx VPSRLD v/xmm xmm/reg128 imm8
		val /vex/66/0f/vexv [0xd3 /r] | vex128? = varity3 avx VPSRLQ xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x73 /2-reg] | vex128? = varity3 avx VPSRLQ v/xmm xmm/reg128 imm8

### PSUBB/PSUBW/PSUBD
###  - Subtract Packed Integers
		val / [0x0f 0xf8 /r] = binop mmx PSUBB mm64 mm/m64
		val /66 [0x0f 0xf8 /r] = binop sse2 PSUBB xmm128 xmm/m128
		val / [0x0f 0xf9 /r] = binop mmx PSUBW mm64 mm/m64
		val /66 [0x0f 0xf9 /r] = binop sse2 PSUBW xmm128 xmm/m128
		val / [0x0f 0xfa /r] = binop mmx PSUBD mm64 mm/m64
		val /66 [0x0f 0xfa /r] = binop sse2 PSUBD xmm128 xmm/m128
		val /vex/66/0f/vexv [0xf8 /r] | vex128? = varity3 avx VPSUBB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xf9 /r] | vex128? = varity3 avx VPSUBW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xfa /r] | vex128? = varity3 avx VPSUBD xmm128 v/xmm xmm/m128

### PSUBQ
###  - Subtract Packed Quadword Integers
		val / [0x0f 0xfb /r] = binop sse2 PSUBQ mm64 mm/m64
		val /66 [0x0f 0xfb /r] = binop sse2 PSUBQ xmm128 xmm/m128
		val /vex/66/0f/vexv [0xfb /r] | vex128? = varity3 avx VPSUBQ xmm128 v/xmm xmm/m128

### PSUBSB/PSUBSW
###  - Subtract Packed Signed Integers with Signed Saturation
		val / [0x0f 0xe8 /r] = binop mmx PSUBSB mm64 mm/m64
		val /66 [0x0f 0xe8 /r] = binop sse2 PSUBSB xmm128 xmm/m128
		val / [0x0f 0xe9 /r] = binop mmx PSUBSW mm64 mm/m64
		val /66 [0x0f 0xe9 /r] = binop sse2 PSUBSW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xe8 /r] | vex128? = varity3 avx VPSUBSB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xe9 /r] | vex128? = varity3 avx VPSUBSW xmm128 v/xmm xmm/m128

### PSUBUSB/PSUBUSW
###  - Subtract Packed Unsigned Integers with Unsigned Saturation
		val / [0x0f 0xd8 /r] = binop mmx PSUBUSB mm64 mm/m64
		val /66 [0x0f 0xd8 /r] = binop sse2 PSUBUSB xmm128 xmm/m128
		val / [0x0f 0xd9 /r] = binop mmx PSUBUSW mm64 mm/m64
		val /66 [0x0f 0xd9 /r] = binop sse2 PSUBUSW xmm128 xmm/m128
		val /vex/66/0f/vexv [0xd8 /r] | vex128? = varity3 avx VPSUBUSB xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0xd9 /r] | vex128? = varity3 avx VPSUBUSW xmm128 v/xmm xmm/m128

### PTEST
###  - Logical Compare
		val /66 [0x0f 0x38 0x17 /r] = binop sse4_1 PTEST xmm128 xmm/m128
		val /vex/66/0f/38 [0x17 /r]
		 | vex128? = varity2 avx VPTEST xmm128 xmm/m128
		 | vex256? = varity2 avx VPTEST ymm256 ymm/m256

### PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ
###  - Unpack High Data
		val / [0x0f 0x68 /r] = binop mmx PUNPCKHBW mm64 mm/m64
		val /66 [0x0f 0x68 /r] = binop sse2 PUNPCKHBW xmm128 xmm/m128
		val / [0x0f 0x69 /r] = binop mmx PUNPCKHWD mm64 mm/m64
		val /66 [0x0f 0x69 /r] = binop sse2 PUNPCKHWD xmm128 xmm/m128
		val / [0x0f 0x6a /r] = binop mmx PUNPCKHDQ mm64 mm/m64
		val /66 [0x0f 0x6a /r] = binop sse2 PUNPCKHDQ xmm128 xmm/m128
		val /66 [0x0f 0x6d /r] = binop sse2 PUNPCKHQDQ xmm128 xmm/m128
		val /vex/66/0f/vexv [0x68 /r] | vex128? = varity3 avx VPUNPCKHBW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x69 /r] | vex128? = varity3 avx VPUNPCKHWD xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x6a /r] | vex128? = varity3 avx VPUNPCKHDQ xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x6d /r] | vex128? = varity3 avx VPUNPCKHQDQ xmm128 v/xmm xmm/m128

### PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ
###  - Unpack Low Data
		val / [0x0f 0x60 /r] = binop mmx PUNPCKLBW mm64 mm/m32
		val /66 [0x0f 0x60 /r] = binop sse2 PUNPCKLBW xmm128 xmm/m128
		val / [0x0f 0x61 /r] = binop mmx PUNPCKLWD mm64 mm/m32
		val /66 [0x0f 0x61 /r] = binop sse2 PUNPCKLWD xmm128 xmm/m128
		val / [0x0f 0x62 /r] = binop mmx PUNPCKLDQ mm64 mm/m32
		val /66 [0x0f 0x62 /r] = binop sse2 PUNPCKLDQ xmm128 xmm/m128
		val /66 [0x0f 0x6c /r] = binop sse2 PUNPCKLQDQ xmm128 xmm/m128
		val /vex/66/0f/vexv [0x60 /r] | vex128? = varity3 avx VPUNPCKLBW xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x61 /r] | vex128? = varity3 avx VPUNPCKLWD xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x62 /r] | vex128? = varity3 avx VPUNPCKLDQ xmm128 v/xmm xmm/m128
		val /vex/66/0f/vexv [0x6c /r] | vex128? = varity3 avx VPUNPCKLQDQ xmm128 v/xmm xmm/m128

### PUSH
###  - Push Word, Doubleword or Quadword Onto the Stack
		val / [0xff /6]
		 | opndsz? = do opndsz-set-from-d; unop none PUSH r/m16 end
		 | mode32? = do opndsz-set-from-d; unop none PUSH r/m32 end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH r/m64 end
		val / ['01010 r:3']
		 | opndsz? = do opndsz-set-from-d; update@{reg/opcode=r}; unop none PUSH r16/rexb end
		 | mode32? = do opndsz-set-from-d; update@{reg/opcode=r}; unop none PUSH r32/rexb end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; update@{reg/opcode=r}; unop none PUSH r64/rexb end
		val / [0x6a]
		 | mode32? = do opndsz-set-from-d; unop none PUSH imm8 end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH imm8 end
		val / [0x68]
		 | rexw? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH imm32 end
		 | opndsz? && mode32? = do opndsz-set-from-d; unop none PUSH imm16 end
		 | opndsz? && mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH imm16 end
		 | mode32? = do opndsz-set-from-d; unop none PUSH imm32 end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH imm32 end
		val / [0x0e] | mode32? = do opndsz-set-from-d; unop none PUSH cs end
		val / [0x16] | mode32? = do opndsz-set-from-d; unop none PUSH ds end
		val / [0x06] | mode32? = do opndsz-set-from-d; unop none PUSH es end
		val / [0x0f 0xa0]
		 | mode32? = do opndsz-set-from-d; unop none PUSH fs end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH fs end
		val / [0x0f 0xa8]
		 | mode32? = do opndsz-set-from-d; unop none PUSH gs end
		 | mode64? = do opndsz-set-from-d; update@{default-operand-size=64}; unop none PUSH gs end

### PUSHA/PUSHAD
###  - Push All General-Purpose Registers
		val / [0x60]
		 | mode32? && opndsz? = arity0 none PUSHA
		 | mode32? = arity0 none PUSHAD

### PUSHF/PUSHFD
###  - Push EFLAGS Register onto the Stack
		val / [0x9c]
		 | opndsz? = arity0 none PUSHF
		 | mode32? = arity0 none PUSHFD
		 | mode64? = arity0 none PUSHFQ

### PXOR
###  - Logical Exclusive OR
		val / [0x0f 0xef /r] = binop mmx PXOR mm64 mm/m64
		val /66 [0x0f 0xef /r] = binop sse2 PXOR xmm128 xmm/m128
		val /vex/66/0f/vexv [0xef /r] | vex128? = varity3 avx VPXOR xmm128 v/xmm xmm/m128

### RCL/RCR/ROL/ROR
###  - Rotate
		val / [0xd0 /2] = binop none RCL r/m8 one
		val / [0xd2 /2] = binop none RCL r/m8 cl
		val / [0xc0 /2] = binop none RCL r/m8 imm8
		val / [0xd1 /2]
		 | rexw? = binop none RCL r/m64 one
		 | opndsz? = binop none RCL r/m16 one
		 | otherwise = binop none RCL r/m32 one
		val / [0xd3 /2]
		 | rexw? = binop none RCL r/m64 cl
		 | opndsz? = binop none RCL r/m16 cl
		 | otherwise = binop none RCL r/m32 cl
		val / [0xc1 /2]
		 | rexw? = binop none RCL r/m64 imm8
		 | opndsz? = binop none RCL r/m16 imm8
		 | otherwise = binop none RCL r/m32 imm8
		val / [0xd0 /3] = binop none RCR r/m8 one
		val / [0xd2 /3] = binop none RCR r/m8 cl
		val / [0xc0 /3] = binop none RCR r/m8 imm8
		val / [0xd1 /3]
		 | rexw? = binop none RCR r/m64 one
		 | opndsz? = binop none RCR r/m16 one
		 | otherwise = binop none RCR r/m32 one
		val / [0xd3 /3]
		 | rexw? = binop none RCR r/m64 cl
		 | opndsz? = binop none RCR r/m16 cl
		 | otherwise = binop none RCR r/m32 cl
		val / [0xc1 /3]
		 | rexw? = binop none RCR r/m64 imm8
		 | opndsz? = binop none RCR r/m16 imm8
		 | otherwise = binop none RCR r/m32 imm8
		val / [0xd0 /0] = binop none ROL r/m8 one
		val / [0xd2 /0] = binop none ROL r/m8 cl
		val / [0xc0 /0] = binop none ROL r/m8 imm8
		val / [0xd1 /0]
		 | rexw? = binop none ROL r/m64 one
		 | opndsz? = binop none ROL r/m16 one
		 | otherwise = binop none ROL r/m32 one
		val / [0xd3 /0]
		 | rexw? = binop none ROL r/m64 cl
		 | opndsz? = binop none ROL r/m16 cl
		 | otherwise = binop none ROL r/m32 cl
		val / [0xc1 /0]
		 | rexw? = binop none ROL r/m64 imm8
		 | opndsz? = binop none ROL r/m16 imm8
		 | otherwise = binop none ROL r/m32 imm8
		val / [0xd0 /1] = binop none ROR r/m8 one
		val / [0xd2 /1] = binop none ROR r/m8 cl
		val / [0xc0 /1] = binop none ROR r/m8 imm8
		val / [0xd1 /1]
		 | rexw? = binop none ROR r/m64 one
		 | opndsz? = binop none ROR r/m16 one
		 | otherwise = binop none ROR r/m32 one
		val / [0xd3 /1]
		 | rexw? = binop none ROR r/m64 cl
		 | opndsz? = binop none ROR r/m16 cl
		 | otherwise = binop none ROR r/m32 cl
		val / [0xc1 /1]
		 | rexw? = binop none ROR r/m64 imm8
		 | opndsz? = binop none ROR r/m16 imm8
		 | otherwise = binop none ROR r/m32 imm8

### RCPPS
###  - Compute Reciprocals of Packed Single-Precision Floating-Point Values
		val / [0x0f 0x53 /r] = binop sse RCPPS xmm128 xmm/m128
		val /vex/0f [0x53 /r]
		 | vex128? = varity2 avx VRCPPS xmm128 xmm/m128
		 | vex256? = varity2 avx VRCPPS ymm256 ymm/m256

### RCPSS
###  - Compute Reciprocal of Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0x53 /r] = binop sse RCPSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x53 /r] = varity3 avx VRCPSS xmm128 v/xmm xmm/m32

### RDFSBASE/RDGSBASE
###  - Read FS/GS Segment Base
		val /f3 [0x0f 0xae /0-reg]
		 | mode64? && rexw? = unop fsgsbase RDFSBASE r/reg64
		 | mode64? = unop fsgsbase RDFSBASE r/reg32
		val /f3 [0x0f 0xae /1-reg]
		 | mode64? && rexw? = unop fsgsbase RDGSBASE r/reg64
		 | mode64? = unop fsgsbase RDGSBASE r/reg32

### RDMSR
###  - Read from Model Specific Register
		val / [0x0f 0x32] = arity0 none RDMSR

### RDPMC
###  - Read Performance-Monitoring Counters
		val / [0x0f 0x33] = arity0 none RDPMC

### RDRAND
###  - Read Random Number
		val / [0x0f 0xc7 /6-reg]
		 | rexw? = unop rdrand RDRAND r/reg64
		 | opndsz? = unop rdrand RDRAND r/reg16
		 | otherwise = unop rdrand RDRAND r/reg32

### RDTSC
###  - Read Time-Stamp Counter
		val / [0x0f 0x31] = arity0 none RDTSC

### RDTSCP
###  - Read Time-Stamp Counter and Processor ID
#val / [0x0f 0x01 0xf9] = arity0 RDTSCP
		val / [0x0f 0x01 /7-reg] = do
		  rm <- query $rm;
		  mode64 <- query mode64?;
		  if rm == '001' then
			 arity0 none RDTSCP
		  else
			 case mode64 of
				 '1': arity0 none SWAPGS
			 end
		end

### REP/REPE/REPZ/REPNE/REPNZ
###  - Repeat String Operation Prefix

### RET
###  - Return from Procedure
		val / [0xc3] = varity0-def-opnd-sz-64 none RET
		val / [0xcb] = varity0-def-opnd-sz-64 none RET_FAR
		val / [0xc2] = varity1-def-opnd-sz-64 none RET imm16
		val / [0xca] = varity1-def-opnd-sz-64 none RET_FAR imm16

### ROUNDPD
###  - Round Packed Double Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x09 /r] = ternop sse4_1 ROUNDPD xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0x09 /r]
		 | vex128? = varity3 avx VROUNDPD xmm128 xmm/m128 imm8
		 | vex256? = varity3 avx VROUNDPD ymm256 ymm/m256 imm8

### ROUNDPS
###  - Round Packed Single Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x08 /r] = ternop sse4_1 ROUNDPS xmm128 xmm/m128 imm8
		val /vex/66/0f/3a [0x08 /r]
		 | vex128? = varity3 avx VROUNDPS xmm128 xmm/m128 imm8
		 | vex256? = varity3 avx VROUNDPS ymm256 ymm/m256 imm8

### ROUNDSD
###  - Round Scalar Double Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x0b /r] = ternop sse4_1 ROUNDSD xmm128 xmm/m64 imm8
		val /vex/66/0f/3a/vexv [0x0b /r] = varity4 avx VROUNDSD xmm128 v/xmm xmm/m64 imm8

### ROUNDSS
###  - Round Scalar Single Precision Floating-Point Values
		val /66 [0x0f 0x3a 0x0a /r] = ternop sse4_1 ROUNDSS xmm128 xmm/m32 imm8
		val /vex/66/0f/3a/vexv [0x0a /r] = varity4 avx VROUNDSS xmm128 v/xmm xmm/m32 imm8

### RSM
###  - Resume from System Management Mode
		val / [0x0f 0xaa] | mode32? = arity0 none RSM

### RSQRTPS
###  - Compute Reciprocals of Square Roots of Packed Single-Precision Floating-Point Values
		val / [0x0f 0x52 /r] = binop sse RSQRTPS xmm128 xmm/m128
		val /vex/0f [0x52 /r]
		 | vex128? = varity2 avx VRSQRTPS xmm128 xmm/m128
		 | vex256? = varity2 avx VRSQRTPS ymm256 ymm/m256

### RSQRTSS
###  - Compute Reciprocal of Square Root of Scalar Single-Precision Floating-Point Value
		val /f3 [0x0f 0x52 /r] = binop sse RSQRTSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x52 /r] = varity3 avx VRSQRTSS xmm128 v/xmm xmm/m32

### SAHF
###  - Store AH into Flags
		val / [0x9e] = arity0 none SAHF

### SAL/SAR/SHL/SHR
### - Shift
#### SAL/SHL
		val / [0xd0 /4] = binop none SHL r/m8 one
#val / [0xd0 /6] = binop none SHL r/m8 one
		val / [0xd2 /4] = binop none SHL r/m8 cl
		val / [0xc0 /4] = binop none SHL r/m8 imm8
		val / [0xd1 /4]
		 | rexw? = binop none SHL r/m64 one
		 | opndsz? = binop none SHL r/m16 one
		 | otherwise = binop none SHL r/m32 one
		val / [0xd3 /4]
		 | rexw? = binop none SHL r/m64 cl
		 | opndsz? = binop none SHL r/m16 cl
		 | otherwise = binop none SHL r/m32 cl
		val / [0xc1 /4]
		 | rexw? = binop none SHL r/m64 imm8
		 | opndsz? = binop none SHL r/m16 imm8
		 | otherwise = binop none SHL r/m32 imm8
#### SAR
		val / [0xd0 /7] = binop none SAR r/m8 one
		val / [0xd2 /7] = binop none SAR r/m8 cl
		val / [0xc0 /7] = binop none SAR r/m8 imm8
		val / [0xd1 /7]
		 | rexw? = binop none SAR r/m64 one
		 | opndsz? = binop none SAR r/m16 one
		 | otherwise = binop none SAR r/m32 one
		val / [0xd3 /7]
		 | rexw? = binop none SAR r/m64 cl
		 | opndsz? = binop none SAR r/m16 cl
		 | otherwise = binop none SAR r/m32 cl
		val / [0xc1 /7]
		 | rexw? = binop none SAR r/m64 imm8
		 | opndsz? = binop none SAR r/m16 imm8
		 | otherwise = binop none SAR r/m32 imm8
#### SHR
		val / [0xd0 /5] = binop none SHR r/m8 one
		val / [0xd2 /5] = binop none SHR r/m8 cl
		val / [0xc0 /5] = binop none SHR r/m8 imm8
		val / [0xd1 /5]
		 | rexw? = binop none SHR r/m64 one
		 | opndsz? = binop none SHR r/m16 one
		 | otherwise = binop none SHR r/m32 one
		val / [0xd3 /5]
		 | rexw? = binop none SHR r/m64 cl
		 | opndsz? = binop none SHR r/m16 cl
		 | otherwise = binop none SHR r/m32 cl
		val / [0xc1 /5]
		 | rexw? = binop none SHR r/m64 imm8
		 | opndsz? = binop none SHR r/m16 imm8
		 | otherwise = binop none SHR r/m32 imm8

### SBB
###  - Integer Subtraction with Borrow
		val / [0x1c] = binop none SBB al imm8
		val / [0x1d]
		 | rexw? = binop none SBB rax imm32
		 | opndsz? = binop none SBB ax imm16
		 | otherwise = binop none SBB eax imm32
		val / [0x80 /3] = binop-lock none SBB r/m8 imm8
		val / [0x81 /3]
		 | rexw? = binop-lock none SBB r/m64 imm32
		 | opndsz? = binop-lock none SBB r/m16 imm16
		 | otherwise = binop-lock none SBB r/m32 imm32
		val / [0x83 /3]
		 | rexw? = binop-lock none SBB r/m64 imm8
		 | opndsz? = binop-lock none SBB r/m16 imm8
		 | otherwise = binop-lock none SBB r/m32 imm8
		val / [0x18 /r] = binop-lock none SBB r/m8 r8
		val / [0x19 /r]
		 | rexw? = binop-lock none SBB r/m64 r64
		 | opndsz? = binop-lock none SBB r/m16 r16
		 | otherwise = binop-lock none SBB r/m32 r32
		val / [0x1a /r] = binop none SBB r8 r/m8
		val / [0x1b /r]
		 | rexw? = binop none SBB r64 r/m64
		 | opndsz? = binop none SBB r16 r/m16
		 | otherwise = binop none SBB r32 r/m32

### SCAS/SCASB/SCASW/SCASD/SCASQ
###  - Scan String
		val / [0xae] = arity0-rep-repne none SCASB
		val / [0xaf]
		 | rexw? = arity0-rep-repne none SCASQ
		 | opndsz? = arity0-rep-repne none SCASW
		 | otherwise = arity0-rep-repne none SCASD

### SETcc
###  - Set Byte on Condition
		val / [0x0f 0x97 /r] = unop none SETA r/m8 # SETNBE
		val / [0x0f 0x93 /r] = unop none SETAE r/m8 # SETNB, SETNC
		val / [0x0f 0x92 /r] = unop none SETB r/m8 # SETC, SETNAE
		val / [0x0f 0x96 /r] = unop none SETBE r/m8 # SETNA
		val / [0x0f 0x94 /r] = unop none SETE r/m8 # SETZ
		val / [0x0f 0x9f /r] = unop none SETG r/m8 # SETNLE
		val / [0x0f 0x9d /r] = unop none SETGE r/m8 # SETNL
		val / [0x0f 0x9c /r] = unop none SETL r/m8 # SETNGE
		val / [0x0f 0x9e /r] = unop none SETLE r/m8 # SETNG
		val / [0x0f 0x95 /r] = unop none SETNE r/m8 # SETNZ
		val / [0x0f 0x91 /r] = unop none SETNO r/m8
		val / [0x0f 0x9b /r] = unop none SETNP r/m8 # SETPO
		val / [0x0f 0x99 /r] = unop none SETNS r/m8
		val / [0x0f 0x90 /r] = unop none SETO r/m8
		val / [0x0f 0x9a /r] = unop none SETP r/m8 # SETPE
		val / [0x0f 0x98 /r] = unop none SETS r/m8

### SFENCE
###  - Store Fence
		val / [0x0f 0xae /7-reg] = arity0 none SFENCE

### SGDT
###  - Store Global Descriptor Table Register
		val / [0x0f 0x01 /0-mem]
		 | mode32? = unop none SGDT m48
		 | mode64? = unop none SGDT m80

### SHLD
###  - Double Precision Shift Left
		val / [0x0f 0xa4 /r]
		 | rexw? = ternop none SHLD r/m64 r64 imm8
		 | opndsz? = ternop none SHLD r/m16 r16 imm8
		 | otherwise = ternop none SHLD r/m32 r32 imm8
		val / [0x0f 0xa5 /r]
		 | rexw? = ternop none SHLD r/m64 r64 cl
		 | opndsz? = ternop none SHLD r/m16 r16 cl
		 | otherwise = ternop none SHLD r/m32 r32 cl

### SHRD
###  - Double Precision Shift Right
		val / [0x0f 0xac /r]
		 | rexw? = ternop none SHRD r/m64 r64 imm8
		 | opndsz? = ternop none SHRD r/m16 r16 imm8
		 | otherwise = ternop none SHRD r/m32 r32 imm8
		val / [0x0f 0xad /r]
		 | rexw? = ternop none SHRD r/m64 r64 cl
		 | opndsz? = ternop none SHRD r/m16 r16 cl
		 | otherwise = ternop none SHRD r/m32 r32 cl

### SHUFPD
###  - Shuffle Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0xc6 /r] = ternop sse2 SHUFPD xmm128 xmm/m128 imm8
		val /vex/66/0f/vexv [0xc6 /r]
		 | vex128? = varity4 avx VSHUFPD xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VSHUFPD ymm256 v/ymm ymm/m256 imm8

### SHUFPS
###  - Shuffle Packed Single-Precision Floating-Point Values
		val / [0x0f 0xc6 /r] = ternop sse SHUFPS xmm128 xmm/m128 imm8
		val /vex/0f/vexv [0xc6 /r]
		 | vex128? = varity4 avx VSHUFPS xmm128 v/xmm xmm/m128 imm8
		 | vex256? = varity4 avx VSHUFPS ymm256 v/ymm ymm/m256 imm8

### SIDT
###  - Store Interrupt Descriptor Table Register
		val / [0x0f 0x01 /1-mem]
		 | mode32? = unop none SIDT m48
		 | mode64? = unop none SIDT m80

### SLDT
###  - Store Local Descriptor Table Register
		val / [0x0f 0x00 /0]
		 | rexw? = unop none SLDT r64/m16
		 | otherwise = unop none SLDT r/m16

### SMSW
###  - Store Machine Status Word
		val / [0x0f 0x01 /4]
		 | rexw? = unop none SMSW r64/m16
		 | opndsz? = unop none SMSW r/m16
		 | otherwise = unop none SMSW r32/m16

### SQRTPD
###  - Compute Square Roots of Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x51 /r] = binop sse2 SQRTPD xmm128 xmm/m128
		val /vex/66/0f [0x51 /r]
		 | vex128? = varity2 avx VSQRTPD xmm128 xmm/m128
		 | vex256? = varity2 avx VSQRTPD ymm256 ymm/m256

### SQRTPS
###  - Compute Square Roots of Packed Single-Precision Floating-Point Values
		val / [0x0f 0x51 /r] = binop sse SQRTPS xmm128 xmm/m128
		val /vex/0f [0x51 /r]
		 | vex128? = varity2 avx VSQRTPS xmm128 xmm/m128
		 | vex256? = varity2 avx VSQRTPS ymm256 ymm/m256

### SQRTSD
###  - Compute Square Root of Scalar Double-Precision Floating-Point Value
		val /f2 [0x0f 0x51 /r] = binop sse2 SQRTSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x51 /r] = varity3 avx VSQRTSD xmm128 v/xmm xmm/m64

### SQRTSS
###  - Compute Square Root of Scalar Single-Precision Floating-Point Value
		val /f3 [0x0f 0x51 /r] = binop sse SQRTSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x51 /r] = varity3 avx VSQRTSS xmm128 v/xmm xmm/m32

### STC
###  - Set Carry Flag
		val / [0xf9] = arity0 none STC

### STD
###  - Set Direction Flag
		val / [0xfd] = arity0 none STD

### STI
###  - Set Interrupt Flag
		val / [0xfb] = arity0 none STI

### STMXCSR
###  - Store MXCSR Register State
		val / [0x0f 0xae /3-mem] = unop sse STMXCSR m32
		val /vex/0f [0xae /3-mem] | vex128? = varity1 avx VSTMXCSR m32

### STOS/STOSB/STOSW/STOSD/STOSQ
###  - Store String
		val / [0xaa] = arity0-rep none STOSB
		val / [0xab]
		 | rexw? = arity0-rep none STOSQ
		 | opndsz? = arity0-rep none STOSW
		 | otherwise = arity0-rep none STOSD

### STR
###  - Store Task Register
		val / [0x0f 0x00 /1] = unop none STR r/m16

### SUB
###  - Subtract
		val / [0x2c] = binop none SUB al imm8
		val / [0x2d]
		 | rexw? = binop none SUB rax imm32
		 | opndsz? = binop none SUB ax imm16
		 | otherwise = binop none SUB eax imm32
		val / [0x80 /5] = binop-lock none SUB r/m8 imm8
		val / [0x81 /5]
		 | rexw? = binop-lock none SUB r/m64 imm32
		 | opndsz? = binop-lock none SUB r/m16 imm16
		 | otherwise = binop-lock none SUB r/m32 imm32
		val / [0x83 /5]
		 | rexw? = binop-lock none SUB r/m64 imm8
		 | opndsz? = binop-lock none SUB r/m16 imm8
		 | otherwise = binop-lock none SUB r/m32 imm8
		val / [0x28 /r] = binop-lock none SUB r/m8 r8
		val / [0x29 /r]
		 | rexw? = binop-lock none SUB r/m64 r64
		 | opndsz? = binop-lock none SUB r/m16 r16
		 | otherwise = binop-lock none SUB r/m32 r32
		val / [0x2a /r] = binop none SUB r8 r/m8
		val / [0x2b /r]
		 | rexw? = binop none SUB r64 r/m64
		 | opndsz? = binop none SUB r16 r/m16
		 | otherwise = binop none SUB r32 r/m32

### SUBPD
###  - Subtract Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x5c /r] = binop sse2 SUBPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x5c /r]
		 | vex128? = varity3 avx VSUBPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VSUBPD ymm256 v/ymm ymm/m256

### SUBPS
###  - Subtract Packed Single-Precision Floating-Point Values
		val / [0x0f 0x5c /r] = binop sse SUBPS xmm128 xmm/m128
		val /vex/0f/vexv [0x5c /r]
		 | vex128? = varity3 avx VSUBPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VSUBPS ymm256 v/ymm ymm/m256

### SUBSD
###  - Subtract Scalar Double-Precision Floating-Point Values
		val /f2 [0x0f 0x5c /r] = binop sse2 SUBSD xmm128 xmm/m64
		val /vex/f2/0f/vexv [0x5c /r] = varity3 avx VSUBSD xmm128 v/xmm xmm/m64

### SUBSS
###  - Subtract Scalar Single-Precision Floating-Point Values
		val /f3 [0x0f 0x5c /r] = binop sse SUBSS xmm128 xmm/m32
		val /vex/f3/0f/vexv [0x5c /r] = varity3 avx VSUBSS xmm128 v/xmm xmm/m32

### SWAPGS
###  - Swap GS Base Register
#See RDTSCP

### SYSCALL
###  - Fast System Call
		val / [0x0f 0x05] | mode64? = arity0 none SYSCALL

### SYSENTER
###  - Fast System Call
		val / [0x0f 0x34] = arity0 none SYSENTER

### SYSEXIT
###  - Fast Return from Fast System Call
		val / [0x0f 0x35]
		 | rexw? = arity0 none SYSEXIT
		 | otherwise = arity0 none SYSEXIT

### SYSRET
###  - Return From Fast System Call
		val / [0x0f 0x07]
		 | mode64? && rexw? = arity0 none SYSRET
		 | mode64? = arity0 none SYSRET

### TEST
###  - Logical Compare
		val / [0xa8] = binop none TEST al imm8
		val / [0xa9]
		 | rexw? = binop none TEST rax imm32
		 | opndsz? = binop none TEST ax imm16
		 | otherwise = binop none TEST eax imm32
		val / [0xf6 /0] = binop none TEST r/m8 imm8
		val / [0xf7 /0]
		 | rexw? = binop none TEST r/m64 imm32
		 | opndsz? = binop none TEST r/m16 imm16
		 | otherwise = binop none TEST r/m32 imm32
		val / [0x84 /r] = binop none TEST r/m8 r8
		val / [0x85 /r]
		 | rexw? = binop none TEST r/m64 r64
		 | opndsz? = binop none TEST r/m16 r16
		 | otherwise = binop none TEST r/m32 r32

### UCOMISD
###  - Unordered Compare Scalar Double-Precision Floating-Point Values and Set EFLAGS
		val /66 [0x0f 0x2e /r] = binop sse2 UCOMISD xmm128 xmm/m64
		val /vex/66/0f [0x2e /r] = varity2 avx VUCOMISD xmm128 xmm/m64

### UCOMISS
###  - Unordered Compare Scalar Single-Precision Floating-Point Values and Set EFLAGS
		val / [0x0f 0x2e /r] = binop sse UCOMISS xmm128 xmm/m32
		val /vex/0f [0x2e /r] = varity2 avx VUCOMISS xmm128 xmm/m32

### UD2
###  - Undefined Instruction
		val / [0x0f 0x0b] = arity0 none UD2

### UNPCKHPD
###  - Unpack and Interleave High Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x15 /r] = binop sse2 UNPCKHPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x15 /r]
		 | vex128? = varity3 avx VUNPCKHPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VUNPCKHPD ymm256 v/ymm ymm/m256

### UNPCKHPS
###  - Unpack and Interleave High Packed Single-Precision Floating-Point Values
		val / [0x0f 0x15 /r] = binop sse UNPCKHPS xmm128 xmm/m128
		val /vex/0f/vexv [0x15 /r]
		 | vex128? = varity3 avx VUNPCKHPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VUNPCKHPS ymm256 v/ymm ymm/m256

### UNPCKLPD
###  - Unpack and Interleave Low Packed Double-Precision Floating-Point Values
		val /66 [0x0f 0x14 /r] = binop sse2 UNPCKLPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x14 /r]
		 | vex128? = varity3 avx VUNPCKLPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VUNPCKLPD ymm256 v/ymm ymm/m256

### UNPCKLPS
###  - Unpack and Interleave Low Packed Single-Precision Floating-Point Values
		val / [0x0f 0x14 /r] = binop sse UNPCKLPS xmm128 xmm/m128
		val /vex/0f/vexv [0x14 /r]
		 | vex128? = varity3 avx VUNPCKLPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VUNPCKLPS ymm256 v/ymm ymm/m256

### VBROADCAST
###  - Load with Broadcast
		val /vex/66/0f/38 [0x18 /r-mem]
		 | vex128? && vexw0? && mode32? = varity2 avx VBROADCASTSS xmm128 m32
		 | vex256? && vexw0? = varity2 avx VBROADCASTSS ymm256 m32
		val /vex/66/0f/38 [0x19 /r-mem] | vex256? && vexw0? = varity2 avx VBROADCASTSD ymm256 m64
		val /vex/66/0f/38 [0x1a /r-mem] | vex256? && vexw0? = varity2 avx VBROADCASTF128 ymm256 m128

### VCVTPH2PS
###  - Convert 16-bit FP Values to Single-Precision FP Values
		val /vex/66/0f/38 [0x13 /r]
		 | vex256? && vexw0? = varity2 f16c VCVTPH2PS ymm256 xmm/m128
		 | vex128? && vexw0? = varity2 f16c VCVTPH2PS xmm128 xmm/m64

### VCVTPS2PH
###  - Convert Single-Precision FP value to 16-bit FP value
		val /vex/66/0f/3a [0x1d /r]
		 | vex256? && vexw0? = varity3 f16c VCVTPS2PH xmm/m128 ymm256 imm8
		 | vex128? && vexw0? = varity3 f16c VCVTPS2PH xmm/m64 xmm128 imm8

### VERR/VERW
###  - Verify a Segment for Reading or Writing
		val / [0x0f 0x00 /4] = unop none VERR r/m16
		val / [0x0f 0x00 /5] = unop none VERW r/m16

### VEXTRACTF128
###  - Extract Packed Floating-Point Values
		val /vex/66/0f/3a [0x19 /r] | vex256? && vexw0? = varity3 avx VEXTRACTF128 xmm/m128 ymm256 imm8

### VINSERTF128
###  - Insert Packed Floating-Point Values
		val /vex/66/0f/3a/vexv [0x18 /r] | vex256? && vexw0? = varity4 avx VINSERTF128 ymm256 v/ymm xmm/m128 imm8

### VMASKMOV
###  - Conditional SIMD Packed Loads and Stores
		val /vex/66/0f/38/vexv [0x2c /r-mem]
		 | vex128? && vexw0? = varity3 avx VMASKMOVPS xmm128 v/xmm m128
		 | vex256? && vexw0? = varity3 avx VMASKMOVPS ymm256 v/ymm m256
		val /vex/66/0f/38/vexv [0x2d /r-mem]
		 | vex128? && vexw0? = varity3 avx VMASKMOVPD xmm128 v/xmm m128
		 | vex256? && vexw0? = varity3 avx VMASKMOVPD ymm256 v/ymm m256
		val /vex/66/0f/38/vexv [0x2e /r-mem]
		 | vex128? && vexw0? = varity3 avx VMASKMOVPS m128 v/xmm xmm128
		 | vex256? && vexw0? = varity3 avx VMASKMOVPS m256 v/ymm ymm256
		val /vex/66/0f/38/vexv [0x2f /r-mem]
		 | vex128? && vexw0? = varity3 avx VMASKMOVPD m128 v/xmm xmm128
		 | vex256? && vexw0? = varity3 avx VMASKMOVPD m256 v/ymm ymm256

### VPERMILPD
###  - Permute Double-Precision Floating-Point Values
		val /vex/66/0f/38/vexv [0x0d /r]
		 | vex128? && vexw0? = varity3 avx VPERMILPD xmm128 v/xmm xmm/m128
		 | vex256? && vexw0? = varity3 avx VPERMILPD ymm256 v/ymm ymm/m256
		val /vex/66/0f/3a [0x05 /r]
		 | vex128? && vexw0? = varity3 avx VPERMILPD xmm128 xmm/m128 imm8
		 | vex256? && vexw0? = varity3 avx VPERMILPD ymm256 ymm/m256 imm8

### VPERMILPS
###  - Permute Single-Precision Floating-Point Values
		val /vex/66/0f/38/vexv [0x0c /r]
		 | vex128? && vexw0? = varity3 avx VPERMILPS xmm128 v/xmm xmm/m128
		 | vex256? && vexw0? = varity3 avx VPERMILPS ymm256 v/ymm ymm/m256
		val /vex/66/0f/3a [0x04 /r]
		 | vex128? && vexw0? = varity3 avx VPERMILPS xmm128 xmm/m128 imm8
		 | vex256? && vexw0? = varity3 avx VPERMILPS ymm256 ymm/m256 imm8

### VPERM2F128
###  - Permute Floating-Point Values
		val /vex/66/0f/3a/vexv [0x06 /r] | vex256? && vexw0? = varity4 avx VPERM2F128 ymm256 v/ymm ymm/m256 imm8

### VTESTPD/VTESTPS
###  - Packed Bit Test
		val /vex/66/0f/38 [0x0e /r]
		 | vex128? && vexw0? = varity2 avx VTESTPS xmm128 xmm/m128
		 | vex256? && vexw0? = varity2 avx VTESTPS ymm256 ymm/m256
		val /vex/66/0f/38 [0x0f /r]
		 | vex128? && vexw0? = varity2 avx VTESTPD xmm128 xmm/m128
		 | vex256? && vexw0? = varity2 avx VTESTPD ymm256 ymm/m256

### VZEROALL
###  - Zero All YMM Registers
		val /vex/0f [0x77]
		 | vex256? = varity0 avx VZEROALL

### VZEROUPPER
###  - Zero Upper Bits of YMM Registers
		 | vex128? = varity0 avx VZEROUPPER

### WAIT/FWAIT
###  - Wait
		val / [0x9b] = arity0 none WAIT

### WBINVD
###  - Write Back and Invalidate Cache
		val / [0x0f 0x09] = arity0 none WBINVD

### WRFSBASE/WRGSBASE
###  - Write FS/GS Segment Base
		val /f3 [0x0f 0xae /2-reg]
		 | mode64? && rexw? = unop fsgsbase WRFSBASE r/reg64
		 | mode64? = unop fsgsbase WRFSBASE r/reg32
		val /f3 [0x0f 0xae /3-reg]
		 | mode64? && rexw? = unop fsgsbase WRGSBASE r/reg64
		 | mode64? = unop fsgsbase WRGSBASE r/reg32

### WRMSR
###  - Write to Model Specific Register
		val / [0x0f 0x30] = arity0 none WRMSR

### XADD
###  - Exchange and Add
		val / [0x0f 0xc0 /r] = binop-lock none XADD r/m8 r8
		val / [0x0f 0xc1 /r]
		 | rexw? = binop-lock none XADD r/m64 r64
		 | opndsz? = binop-lock none XADD r/m16 r16
		 | otherwise = binop-lock none XADD r/m32 r32

### XCHG
###  - Exchange Register/Memory with Register
### Todo: Reg => No lock?
		val / ['10010 r:3']
		 | rexw? = do update@{reg/opcode=r}; binop none XCHG rax r64/rexb end
		 | opndsz? = do update@{reg/opcode=r}; binop none XCHG ax r16/rexb end
		 | otherwise = do #:-(
			  rexb <- query $rexb;
			  if r == '000' and (not rexb) then #:-(
				 varity0 none NOP
			  else do
				 update@{reg/opcode=r};
				 binop none XCHG eax r32/rexb
			  end
			end
		val / [0x86 /r] = binop-lock none XCHG r/m8 r8
		val / [0x87 /r]
		 | rexw? = binop-lock none XCHG r/m64 r64
		 | opndsz? = binop-lock none XCHG r/m16 r16
		 | otherwise = binop-lock none XCHG r/m32 r32

### XGETBV
###  - Get Value of Extended Control Register
		val / [0x0f 0x01 0xd0] = arity0 none XGETBV

### XLAT/XLATB
###  - Table Look-up Translation
		val / [0xd7]
		 | rexw? = arity0 none XLATB
		 | opndsz? = arity0 none XLATB
		 | otherwise = arity0 none XLATB

### XOR
###  - Logical Exclusive OR
		val / [0x34] = binop none XOR al imm8
		val / [0x35]
		 | rexw? = binop none XOR rax imm32
		 | opndsz? = binop none XOR ax imm16
		 | otherwise = binop none XOR eax imm32
		val / [0x80 /6] = binop-lock none XOR r/m8 imm8
		val / [0x81 /6]
		 | rexw? = binop-lock none XOR r/m64 imm32
		 | opndsz? = binop-lock none XOR r/m16 imm16
		 | otherwise = binop-lock none XOR r/m32 imm32
		val / [0x83 /6]
		 | rexw? = binop-lock none XOR r/m64 imm8
		 | opndsz? = binop-lock none XOR r/m16 imm8
		 | otherwise = binop-lock none XOR r/m32 imm8
		val / [0x30 /r] = binop-lock none XOR r/m8 r8
		val / [0x31 /r]
		 | rexw? = binop-lock none XOR r/m64 r64
		 | opndsz? = binop-lock none XOR r/m16 r16
		 | otherwise = binop-lock none XOR r/m32 r32
		val / [0x32 /r] = binop none XOR r8 r/m8
		val / [0x33 /r]
		 | rexw? = binop none XOR r64 r/m64
		 | opndsz? = binop none XOR r16 r/m16
		 | otherwise = binop none XOR r32 r/m32

### XORPD
###  - Bitwise Logical XOR for Double-Precision Floating-Point Values
		val /66 [0x0f 0x57 /r] = binop sse2 XORPD xmm128 xmm/m128
		val /vex/66/0f/vexv [0x57 /r]
		 | vex128? = varity3 avx VXORPD xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VXORPD ymm256 v/ymm ymm/m256

### XORPS
###  - Bitwise Logical XOR for Single-Precision Floating-Point Values
		val / [0x0f 0x57 /r] = binop sse XORPS xmm128 xmm/m128
		val /vex/0f/vexv [0x57 /r]
		 | vex128? = varity3 avx VXORPS xmm128 v/xmm xmm/m128
		 | vex256? = varity3 avx VXORPS ymm256 v/ymm ymm/m256

### XRSTOR
###  - Restore Processor Extended States
		val / [0x0f 0xae /5-mem]
		 | rexw? = unop none XRSTOR64 mX
		 | otherwise = unop none XRSTOR mX

### XSAVE
###  - Save Processor Extended States
		val / [0x0f 0xae /4-mem]
		 | rexw? = unop none XSAVE64 mX
		 | otherwise = unop none XSAVE mX

### XSAVEOPT
###  - Save Processor Extended States Optimized
		val / [0x0f 0xae /6-mem]
		 | rexw? = unop xsaveopt XSAVEOPT64 mX
		 | otherwise = unop xsaveopt XSAVEOPT mX

### XSETBV
###  - Set Extended Control Register
		val / [0x0f 0x01 0xd1] = arity0 none XSETBV
		*/
	}
}
