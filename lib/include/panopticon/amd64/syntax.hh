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
#include <panopticon/amd64/semantics.hh>
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
			dis& main, dis& mainrep, dis& mainrepx,
			dis const& lock_prfx,
			dis const& imm8, dis const& imm16, dis const& imm32, dis const& imm48, dis const& imm64, dis const& imm, dis const& immlong,
			dis const& moffs8, dis const& moffs,
			dis const& sib,
			dis const& rm, dis const& rm0, dis const& rm1, dis const& rm2, dis const& rm3,
			dis const& rm4, dis const& rm5, dis const& rm6, dis const& rm7,
			dis const& rmbyte, dis const& rmbyte0, dis const& rmbyte1, dis const& rmbyte2,
			dis const& rmbyte3, dis const& rmbyte4, dis const& rmbyte5, dis const& rmbyte6, dis const& rmbyte7,
			dis const& rmlong,
			dis const& disp8, dis const& disp16, dis const& disp32, dis const& disp64,
			dis const& m64, dis const& m128,
			dis const& r16, dis const& r32, dis const& r64)
		{
			// AAA, AAD, AAM and AAS (32 bits only)
			if(Bits <= 32)
			{
				main[ 0x37_e         ] = nonary("aaa",aaa);
				main[ 0xd5_e >> imm8 ] = unary("aad",decode_imm,aad);
				main[ 0xd4_e >> imm8 ] = unary("aam",decode_imm,aam);
				main[ 0x3f_e         ] = nonary("aas",aas);
			}

			// ADC
			main[ *lock_prfx >> 0x14_e >> imm8            ] = binary("adc",al,decode_imm,adc);
			main[ *lock_prfx >> 0x15_e >> imm             ] = binary("adc",decode_i,adc);
			main[ *lock_prfx >> 0x80_e >> rmbyte2 >> imm8 ] = binary("adc",decode_mi,adc);
			main[ *lock_prfx >> 0x81_e >> rm2 >> imm      ] = binary("adc",decode_mi,adc);
			main[ *lock_prfx >> 0x83_e >> rm2 >> imm8     ] = binary("adc",decode_mi,adc);
			main[ *lock_prfx >> 0x10_e >> rmbyte          ] = binary("adc",decode_mr,adc);
			main[ *lock_prfx >> 0x11_e >> rm              ] = binary("adc",decode_mr,adc);
			main[ *lock_prfx >> 0x12_e >> rmbyte          ] = binary("adc",decode_rm,adc);
			main[ *lock_prfx >> 0x13_e >> rm              ] = binary("adc",decode_rm,adc);

			// ADD
			main[ *lock_prfx >> 0x04_e >> imm8            ] = binary("add",al,decode_imm,add);
			main[ *lock_prfx >> 0x05_e >> imm             ] = binary("add",decode_i,add);
			main[ *lock_prfx >> 0x80_e >> rmbyte0 >> imm8 ] = binary("add",decode_mi,add);
			main[ *lock_prfx >> 0x81_e >> rm0 >> imm      ] = binary("add",decode_mi,add);
			main[ *lock_prfx >> 0x83_e >> rm0 >> imm8     ] = binary("add",decode_mi,add);
			main[ *lock_prfx >> 0x00_e >> rmbyte          ] = binary("add",decode_mr,add);
			main[ *lock_prfx >> 0x01_e >> rm              ] = binary("add",decode_mr,add);
			main[ *lock_prfx >> 0x02_e >> rmbyte          ] = binary("add",decode_rm,add);
			main[ *lock_prfx >> 0x03_e >> rm              ] = binary("add",decode_rm,add);

			// ADCX
			main[ 0x66_e >> 0x0f_e >> 0x38_e >> 0xf6_e >> rm ] = binary("adcx",decode_rm,adcx);

			// AND
			main[ *lock_prfx >>	0x24_e >> imm8				] = binary("and",al,decode_imm,and_);
			main[ *lock_prfx >>	0x25_e >> imm         ] = binary("and",decode_i,and_);
			main[ *lock_prfx >>	0x81_e >> rm4 >> imm  ] = binary("and",decode_mi,and_);
			main[ *lock_prfx >>	0x83_e >> rm4 >> imm8 ] = binary("and",decode_mi,and_);
			main[ *lock_prfx >>	0x20_e >> rmbyte			] = binary("and",decode_mr,and_);
			main[ *lock_prfx >>	0x21_e >> rm				  ] = binary("and",decode_mr,and_);
			main[ *lock_prfx >>	0x22_e >> rmbyte			] = binary("and",decode_rm,and_);
			main[ *lock_prfx >> 0x23_e >> rm			   	] = binary("and",decode_rm,and_);

			// ARPL
			if(Bits <= 32)
				main[ 0x63_e >> rm ] = binary("arpl",decode_mr,arpl);

			// BOUND
			if(Bits <= 32)
				main[ 0x62_e >> rm ] = binary("bound",decode_rm,bound);

			// BSF
			main[ 0x0f_e >> 0xbc_e >> rm ] = binary("bsf",decode_rm,bsf);

			// BSR
			main[ 0x0f_e >> 0xbd_e >> rm ] = binary("bsr",decode_rm,bsr);

			// BSWAP
			main[ 0x0f_e >> 0xc8_e ] = unary("bswap",reg_a,bswap);
			main[ 0x0f_e >> 0xc9_e ] = unary("bswap",reg_c,bswap);
			main[ 0x0f_e >> 0xca_e ] = unary("bswap",reg_d,bswap);
			main[ 0x0f_e >> 0xcb_e ] = unary("bswap",reg_b,bswap);
			main[ 0x0f_e >> 0xcc_e ] = unary("bswap",reg_sp,bswap);
			main[ 0x0f_e >> 0xcd_e ] = unary("bswap",reg_bp,bswap);
			main[ 0x0f_e >> 0xce_e ] = unary("bswap",reg_si,bswap);
			main[ 0x0f_e >> 0xcf_e ] = unary("bswap",reg_di,bswap);

			// BT
			main[ 0x0f_e >> 0xa3_e >> rm          ] = binary("bt",decode_rm,bt);
			main[ 0x0f_e >> 0xba_e >> rm4 >> imm8 ] = binary("bt",decode_mi,bt);

			// BTC
			main[ *lock_prfx >> 0x0f_e >> 0xbb_e >> rm          ] = binary("btc",decode_rm,btc);
			main[ *lock_prfx >> 0x0f_e >> 0xba_e >> rm7 >> imm8 ] = binary("btc",decode_mi,btc);

			// BTR
			main[ *lock_prfx >> 0x0f_e >> 0xb3_e >> rm			    ] = binary("btr",decode_rm,btr);
			main[ *lock_prfx >> 0x0f_e >> 0xba_e >> rm6 >> imm8	] = binary("btr",decode_mi,btr);

			// BTS
			main[ *lock_prfx >> 0x0f_e >> 0xab_e >> rm           ] = binary("bts",decode_rm,bts);
			main[ *lock_prfx >> 0x0f_e >> 0xba_e >> rm5 >> imm8  ] = binary("bts",decode_mi,bts);

			// CALL
			if(Bits <= 32)
			{
				main[ 0xff_e >> rm2   ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,false));
				main[ 0x9a_e >> imm48 ] = unary("call",decode_d,std::bind(far_call,pls::_1,pls::_2,true));
			}

			main[ 0xff_e >> rm3 ] = unary("call",decode_m,std::bind(far_call,pls::_1,pls::_2,false));
			main[ 0xe8_e >> moffs ] = unary("call",decode_moffs,std::bind(near_call,pls::_1,pls::_2,true));

			// CBW
			main[ 0x98_e ] = conv;

			main[ 0x99_e ] = conv2;

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
				main[                0x0f_e >> op >> rm ] = binary("cmov" + suffix,decode_rm,std::bind(cmov,pls::_1,pls::_2,pls::_3,cond));
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
			main[ 0x3c_e >> imm8        ] = binary("cmp",al,decode_imm,cmp);
			main[ 0x3d_e >> imm         ] = binary("cmp",decode_i,cmp);
			main[ 0x81_e >> rm7 >> imm  ] = binary("cmp",decode_mi,cmp);
			main[ 0x83_e >> rm7 >> imm8 ] = binary("cmp",decode_mi,cmp);
			main[ 0x38_e >> rmbyte      ] = binary("cmp",decode_mr,cmp);
			main[ 0x39_e >> rm          ] = binary("cmp",decode_mr,cmp);
			main[ 0x3a_e >> rmbyte      ] = binary("cmp",decode_rm,cmp);
			main[ 0x3b_e >> rm          ] = binary("cmp",decode_rm,cmp);

			// CMPS/CMPSW/CMPSD/CMPSQ (rep*)
			mainrepx[ 0xa6_e ] = binary("cmpsb",reg_di,reg_si,cmps);
			mainrepx[ 0xa7_e ] = binary("cmpsw",reg_di,reg_si,cmps);

			// CMPXCHG
			main[ *lock_prfx >> 0x0f_e >> 0xb0_e >> rmbyte ] = binary("cmpxchg",decode_mr,cmpxchg);
			main[ *lock_prfx >> 0x0f_e >> 0xb1_e >> rm ] = binary("cmpxchg",decode_mr,cmpxchg);

			// CMPXCHG8B
			main[ *lock_prfx >> 0x0f_e >> 0xc7_e >> rm1 >> m64 ] = unary("cmpxchg8b",decode_m,std::bind(cmpxchg8b,pls::_1,pls::_2));

			// CMPXCHG16B
			if(Bits == 64)
				main[ *lock_prfx >> 0x0f_e >> 0xc7_e >> rm1 >> m128 ] = unary("cmpxchg16b",decode_m,std::bind(cmpxchg16b,pls::_1,pls::_2));

			// CPUID
			main[ 0x0f_e >> 0xa2_e ] = nonary("cpuid",cpuid);

			// DAS
			if(Bits <= 32)
				main[ 0x2f_e ] = nonary("das",das);

			// DEC
			main[ *lock_prfx >> 0xfe_e >> rmbyte1 ] = unary("dec",decode_m,dec);
			main[ *lock_prfx >> 0xff_e >> rm1 ] = unary("dec",decode_m,dec);

			if(Bits < 64)
			{
				main[ *lock_prfx >> 0x48_e ] = unary("dec",reg_a,dec);
				main[ *lock_prfx >> 0x49_e ] = unary("dec",reg_c,dec);
				main[ *lock_prfx >> 0x4a_e ] = unary("dec",reg_d,dec);
				main[ *lock_prfx >> 0x4b_e ] = unary("dec",reg_b,dec);
				main[ *lock_prfx >> 0x4c_e ] = unary("dec",reg_sp,dec);
				main[ *lock_prfx >> 0x4d_e ] = unary("dec",reg_bp,dec);
				main[ *lock_prfx >> 0x4e_e ] = unary("dec",reg_si,dec);
				main[ *lock_prfx >> 0x4f_e ] = unary("dec",reg_di,dec);
			}

			// DIV
			main[ 0xf6_e >> rmbyte6 ] = unary("div",decode_m,div);
			main[ 0xf7_e >> rm6 ] = unary("div",decode_m,div);

			// DAA
			if(Bits <= 32)
				main[ 0x27_e ] = nonary("daa",daa);

			// ENTER
			main[ 0xc8_e >> imm16 >> imm8 ] = binary("enter",decode_ii,enter);

			// HLT
			main[ 0xf4_e ] = nonary("hlt",hlt);

			// IDIV
			main[ 0xf6_e >> rmbyte7 ] = unary("idiv",decode_m,idiv);
			main[ 0xf7_e >> rm7     ] = unary("idiv",decode_m,idiv);

			// IMUL
			main[ 0xf6_e >> rmbyte5      ] = unary("imul",decode_m,imul1);
			main[ 0xf7_e >> rm5          ] = unary("imul",decode_m,imul1);
			main[ 0x6b_e >> rm >> imm8   ] = trinary("imul",decode_rmi,imul3);
			main[ 0x69_e >> rm >> imm    ] = trinary("imul",decode_rmi,imul3);
			main[ 0x0f_e >> 0xaf_e >> rm ] = binary("imul",decode_rm,imul2);

			// IN
			main[ 0xe4_e >> imm8 ] = binary("in",al,decode_imm,in);
			main[ 0xe5_e >> imm8 ] = binary("in",decode_i,in);
			main[ 0xec_e         ] = binary("in",al,dx,in);
			main[ 0xed_e         ] = binary("in",reg_a,dx,in);

			// INC
			main[ *lock_prfx >> 0xfe_e >> rmbyte0 ] = unary("inc",decode_m,inc);
			main[ *lock_prfx >> 0xff_e >> rm0 ] = unary("inc",decode_m,inc);

			if(Bits < 64)
			{
				main[ *lock_prfx >> 0x40_e ] = unary("inc",reg_a,inc);
				main[ *lock_prfx >> 0x41_e ] = unary("inc",reg_c,inc);
				main[ *lock_prfx >> 0x42_e ] = unary("inc",reg_d,inc);
				main[ *lock_prfx >> 0x43_e ] = unary("inc",reg_b,inc);
				main[ *lock_prfx >> 0x44_e ] = unary("inc",reg_sp,inc);
				main[ *lock_prfx >> 0x45_e ] = unary("inc",reg_bp,inc);
				main[ *lock_prfx >> 0x46_e ] = unary("inc",reg_si,inc);
				main[ *lock_prfx >> 0x47_e ] = unary("inc",reg_di,inc);
			}

			// INS* (rep)
			mainrep[ 0x6c_e ] = binary("insb",reg_di,dx,ins);
			mainrep[ 0x6d_e ] = binary("ins",reg_di,dx,ins);

			// INT
			main[ 0xcc_e         ] = unary("int",constant(3),int_);
			main[ 0xce_e         ] = nonary("into",into);
			main[ 0xcd_e >> imm8 ] = unary("int",decode_imm,int_);

			// ICEBP
			main[ 0xf1_e ] = nonary("icebp",icebp);

			// IRET*
			main[ 0xcf_e ] = iret;

			// J*CXZ
			if(Bits == 16)
			{
				main[ 0xe3_e >> imm8 ] = unary("jcxz",decode_imm,std::bind(jxz,pls::_1,pls::_2,cx));
			}
			else if(Bits == 32)
			{
				main[ 0xe3_e >> imm8 ] = unary("jecxz",decode_imm,std::bind(jxz,pls::_1,pls::_2,ecx));
			}
			else if(Bits == 64)
			{
				main[ 0xe3_e >> imm8 ] = unary("jrcxz",decode_imm,std::bind(jxz,pls::_1,pls::_2,rcx));
			}

			// Jcc
			std::function<void(uint8_t, std::string const&, amd64::condition)> _jcc = [&](uint8_t op, std::string const& suffix, amd64::condition cond)
			{
				main[            op >> imm8        ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));
				main[ 0x0f_e >> (op + 0x10) >> imm ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));
			};

			_jcc(0x70,"o",Overflow);
			_jcc(0x71,"no",NotOverflow);
			_jcc(0x72,"c",Carry);
			_jcc(0x73,"ae",AboveEqual);
			_jcc(0x74,"e",Equal);
			_jcc(0x75,"ne",NotEqual);
			_jcc(0x76,"be",BelowEqual);
			_jcc(0x77,"a",Above);
			_jcc(0x78,"s",Sign);
			_jcc(0x79,"ns",NotSign);
			_jcc(0x7a,"p",Parity);
			_jcc(0x7b,"np",NotParity);
			_jcc(0x7c,"l",Less);
			_jcc(0x7d,"ge",GreaterEqual);
			_jcc(0x7e,"le",LessEqual);
			_jcc(0x7f,"g",Greater);

			// JMP
			main[ 0xeb_e >> imm8   ] = unary("jmp",decode_d,jmp);

			if(Bits == 16)
			{
				main[ 0xe9_e >> moffs ] = unary("jmp",decode_moffs,jmp);
				main[ 0xea_e >> imm32 ] = unary("jmp",decode_d,jmp);
				main[ 0xff_e >> rm4   ] = unary("jmp",decode_m,jmp);
				main[ 0xff_e >> rm5   ] = unary("jmp",decode_d,jmp);
			}
			else if(Bits == 32)
			{
				main[ 0xe9_e >> moffs ] = unary("jmp",decode_moffs,jmp);
				main[ 0xea_e >> imm48 ] = unary("jmp",decode_d,jmp);
				main[ 0xff_e >> rm4   ] = unary("jmp",decode_m,jmp);
				main[ 0xff_e >> rm5   ] = unary("jmp",decode_d,jmp);
			}
			else if(Bits == 64)
			{
				main[ 0xe9_e >> moffs ] = unary("jmp",decode_moffs,jmp);
				main[ 0xff_e >> rm4   ] = unary("jmp",decode_m,jmp);
				main[ 0xff_e >> rm5   ] = unary("jmp",decode_d,jmp);
			}

			// LAHF
			if(Bits <= 32)
				main[ 0x9f_e ] = nonary("lahf",lahf);

			// LAR
			main[ 0x0f_e >> 0x02_e >> rm ] = binary("lar",decode_rm,lar);

			// LDS
			if(Bits <= 32)
			{
				main[ 0xc5_e >> rm ] = binary("lds",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,ds));
			}

			// LEA
			main[ 0x8d_e >> rm ] = binary("lea",decode_rm,lea);

			// LEAVE
			main[ 0xc9_e ] = leave;

			// LES
			if(Bits <= 32)
				main[ 0xc4_e >> rm ] = binary("les",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,es));

			// LFS
			main[ 0x0f_e >> 0xb4_e >> rm ] = binary("lfs",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,fs));

			// LGS
			main[ 0x0f_e >> 0xb5_e >> rm ] = binary("lgs",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,gs));

			// LODS*
			mainrep[ 0xac_e ] = lodsb;
			mainrep[ 0xad_e ] = lods;

			// LOOP
			main[ 0xe2_e >> imm8 ] = loop;

			// LOOPNE
			main[ 0xe0_e >> imm8 ] = loopne;

			// LOOPE
			main[ 0xe1_e >> imm8 ] = loope;

			// LSS
			main[ 0x0f_e >> 0xb2_e >> rm ] = binary("lss",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,ss));

			// MOV
			main[ 0x88_e >> rmbyte ] = binary("mov",decode_mr,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x89_e >> rm     ] = binary("mov",decode_mr,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x8a_e >> rmbyte ] = binary("mov",decode_rm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x8b_e >> rm     ] = binary("mov",decode_rm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x8e_e >> rm     ] = binary("mov",decode_msreg,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x8c_e >> rm     ] = binary("mov",decode_sregm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xa0_e >> moffs8 ] = binary("mov",decode_fd,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xa1_e >> moffs  ] = binary("mov",decode_fd,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xa2_e >> moffs8 ] = binary("mov",decode_td,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xa3_e >> moffs  ] = binary("mov",decode_td,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

			main[ 0xb0_e >> imm8 ] = binary("mov",regb_a,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb1_e >> imm8 ] = binary("mov",regb_c,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb2_e >> imm8 ] = binary("mov",regb_d,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb3_e >> imm8 ] = binary("mov",regb_b,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb4_e >> imm8 ] = binary("mov",regb_sp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb5_e >> imm8 ] = binary("mov",regb_bp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb6_e >> imm8 ] = binary("mov",regb_si,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb7_e >> imm8 ] = binary("mov",regb_di,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

			main[ 0xb8_e >> immlong ] = binary("mov",reg_a,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xb9_e >> immlong ] = binary("mov",reg_c,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xba_e >> immlong ] = binary("mov",reg_d,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xbb_e >> immlong ] = binary("mov",reg_b,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xbc_e >> immlong ] = binary("mov",reg_sp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xbd_e >> immlong ] = binary("mov",reg_bp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xbe_e >> immlong ] = binary("mov",reg_si,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xbf_e >> immlong ] = binary("mov",reg_di,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

			main[ 0xc6_e >> rmbyte0 >> imm8 ] = binary("mov",decode_mi,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0xc7_e >> rm0 >> imm      ] = binary("mov",decode_mi,std::bind(mov,pls::_1,pls::_2,pls::_3,true));

			main[ 0x0f_e >> 0x20_e >> rmlong ] = binary("mov",decode_rmctrl,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x0f_e >> 0x22_e >> rmlong ] = binary("mov",decode_ctrlrm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x0f_e >> 0x21_e >> rmlong ] = binary("mov",decode_rmdbg,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
			main[ 0x0f_e >> 0x23_e >> rmlong ] = binary("mov",decode_dbgrm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

			// MOVBE
			main[ 0x0f_e >> 0x38_e >> 0xf0_e >> rm ] = binary("movbe",decode_rm,movbe);
			main[ 0x0f_e >> 0x38_e >> 0xf1_e >> rm ] = binary("movbe",decode_mr,movbe);

			// MOVS*
			mainrep[ 0xa4_e ] = movsb;
			mainrep[ 0xa5_e ] = movs;

			// MOVSX*
			main[ 0x0f_e >> 0xbe_e >> rm ] = binary("movsx",decode_rm,movsx);
			main[ 0x0f_e >> 0xbf_e >> rm ] = binary("movsx",decode_rm,movsx);

			if(Bits == 64)
				main[ 0x63_e >> rm ] = binary("movsxd",decode_rm,movsx);

			// MOVZX
			main[ 0x0f_e >> 0xb6_e >> rm ] = binary("movzx",decode_rm,movzx);
			main[ 0x0f_e >> 0xb7_e >> rm ] = binary("movzx",decode_rm,movzx);


			// MUL
			main[ 0xf6_e >> rmbyte4 ] = unary("mul",decode_m,mul);
			main[ 0xf7_e >> rm4     ] = unary("mul",decode_m,mul);

			// NEG
			main[ *lock_prfx >> 0xf6_e >> rmbyte3 ] = unary("neg",decode_m,neg);
			main[ *lock_prfx >> 0xf7_e >> rm3     ] = unary("neg",decode_m,neg);

			// NOP
			main[ 0x0f_e >> 0x1f_e >> rm0 ] = nonary("nop",nop);

			// NOT (lock)
			main[ *lock_prfx >> 0xf6_e >> rmbyte2 ] = unary("not",decode_m,not_);
			main[ *lock_prfx >> 0xf7_e >> rm2     ] = unary("not",decode_m,not_);

			// OR
			main[ *lock_prfx >> 0x0c_e >> imm8        ] = binary("or",al,decode_imm,or_);
			main[ *lock_prfx >> 0x0d_e >> imm         ] = binary("or",decode_i,or_);
			main[ *lock_prfx >> 0x81_e >> rm1 >> imm  ] = binary("or",decode_mi,or_);
			main[ *lock_prfx >> 0x83_e >> rm1 >> imm8 ] = binary("or",decode_mi,or_);
			main[ *lock_prfx >> 0x08_e >> rmbyte      ] = binary("or",decode_mr,or_);
			main[ *lock_prfx >> 0x09_e >> rm          ] = binary("or",decode_mr,or_);
			main[ *lock_prfx >> 0x0a_e >> rmbyte      ] = binary("or",decode_rm,or_);
			main[ *lock_prfx >> 0x0b_e >> rm          ] = binary("or",decode_rm,or_);

			// OUT
			main[ 0xe6_e >> imm8 ] = binary("out",al,decode_imm,out);
			main[ 0xe7_e >> imm8 ] = binary("out",decode_i,out);

			main[ 0xee_e ] = binary("out",al,dx,out);
			main[ 0xef_e ] = binary("out",reg_a,dx,out);

			// OUTS* (rep)
			mainrep[ 0x6e_e ] = outs;
			mainrep[ 0x6f_e ] = outs;

			main[ 0x8f_e >> rm0    ] = pop;
			main[ 0x58_e           ] = pop;
			main[ 0x59_e           ] = pop;
			main[ 0x5a_e           ] = pop;
			main[ 0x5b_e           ] = pop;
			main[ 0x5c_e           ] = pop;
			main[ 0x5d_e           ] = pop;
			main[ 0x5e_e           ] = pop;
			main[ 0x5f_e           ] = pop;
			main[ 0x0f_e >> 0xa1_e ] = pop;
			main[ 0x0f_e >> 0xa9_e ] = pop;

			if(Bits <= 32)
			{
				main[  0x1f_e ] = pop;
				main[  0x07_e ] = pop;
				main[  0x17_e ] = pop;
			}

			// POPA*
			if(Bits != 64)
				main[ 0x61_e ] = popa;

			// POPCNT
			main[ 0xf3_e >> 0x0f_e >> 0xb8_e >> rm ] = binary("popcnt",decode_rm,popcnt);

			// POPF*
			main[ 0x9d_e ] = unary("popf",decode_m,popf);

			// PUSH
			main[ 0xff_e >> rm6    ] = push;
			main[ 0x50_e           ] = push;
			main[ 0x51_e           ] = push;
			main[ 0x52_e           ] = push;
			main[ 0x53_e           ] = push;
			main[ 0x54_e           ] = push;
			main[ 0x55_e           ] = push;
			main[ 0x56_e           ] = push;
			main[ 0x57_e           ] = push;
			main[ 0x0f_e >> 0xa0_e ] = push;
			main[ 0x0f_e >> 0xa8_e ] = push;

			if(Bits <= 32)
			{
				main[ 0x0e_e ] = push;
				main[ 0x1e_e ] = push;
				main[ 0x06_e ] = push;
				main[ 0x16_e ] = push;
			}

			main[ 0x6a_e >> imm8 ] = push;
			main[ 0x68_e >> imm  ] = push;

			// PUSHA*
			if(Bits != 64)
				main[ 0x60_e ] = pusha;

			// PUSHF*
			main[ 0x9d_e ] = unary("push",decode_m,pushf);

			// RCL
			main[ 0xd0_e >> rmbyte2         ] = binary("rcl",decode_m,constant(1),rcl);
			main[ 0xd1_e >> rm2             ] = binary("rcl",decode_m,constant(1),rcl);
			main[ 0xd2_e >> rmbyte2         ] = binary("rcl",decode_m,CF,rcl);
			main[ 0xd3_e >> rm2             ] = binary("rcl",decode_m,CF,rcl);
			main[ 0xc0_e >> rmbyte2 >> imm8 ] = binary("rcl",decode_mi,rcl);
			main[ 0xc1_e >> rm2 >> imm8     ] = binary("rcl",decode_mi,rcl);

			// RCR
			main[ 0xd0_e >> rmbyte3         ] = binary("rcr",decode_m,constant(1),rcr);
			main[ 0xd1_e >> rm3             ] = binary("rcr",decode_m,constant(1),rcr);
			main[ 0xd2_e >> rmbyte3         ] = binary("rcr",decode_m,CF,rcr);
			main[ 0xd3_e >> rm3             ] = binary("rcr",decode_m,CF,rcr);
			main[ 0xc0_e >> rmbyte3 >> imm8 ] = binary("rcr",decode_mi,rcr);
			main[ 0xc1_e >> rm3 >> imm8     ] = binary("rcr",decode_mi,rcr);

			// RET*
			main[ 0xc3_e          ] = unary("ret",constant(0),ret);
			main[ 0xcb_e          ] = unary("retf",constant(0),retf);
			main[ 0xc2_e >> imm16 ] = unary("ret",decode_imm,ret);
			main[ 0xca_e >> imm16 ] = unary("retf",decode_imm,retf);

			// ROL
			main[ 0xd0_e >> rmbyte0         ] = binary("rol",decode_m,constant(1),rol);
			main[ 0xd1_e >> rm0             ] = binary("rol",decode_m,constant(1),rol);
			main[ 0xd2_e >> rmbyte0         ] = binary("rol",decode_m,CF,rol);
			main[ 0xd3_e >> rm0             ] = binary("rol",decode_m,CF,rol);
			main[ 0xc0_e >> rmbyte0 >> imm8 ] = binary("rol",decode_mi,rol);
			main[ 0xc1_e >> rm0 >> imm8     ] = binary("rol",decode_mi,rol);

			// ROR
			main[ 0xd0_e >> rmbyte1         ] = binary("ror",decode_m,constant(1),ror);
			main[ 0xd1_e >> rm1             ] = binary("ror",decode_m,constant(1),ror);
			main[ 0xd2_e >> rmbyte1         ] = binary("ror",decode_m,CF,ror);
			main[ 0xd3_e >> rm1             ] = binary("ror",decode_m,CF,ror);
			main[ 0xc0_e >> rmbyte1 >> imm8 ] = binary("ror",decode_mi,ror);
			main[ 0xc1_e >> rm1 >> imm8     ] = binary("ror",decode_mi,ror);

			// SAHF
			main[ 0x9e_e ] = nonary("sahf",sahf);

			// SAL
			main[ 0xd0_e >> rmbyte4         ] = binary("sal",decode_m,constant(1),sal);
			main[ 0xd1_e >> rm4             ] = binary("sal",decode_m,constant(1),sal);
			main[ 0xd2_e >> rmbyte4         ] = binary("sal",decode_m,CF,sal);
			main[ 0xd3_e >> rm4             ] = binary("sal",decode_m,CF,sal);
			main[ 0xc0_e >> rmbyte4 >> imm8 ] = binary("sal",decode_mi,sal);
			main[ 0xc1_e >> rm4 >> imm8     ] = binary("sal",decode_mi,sal);

			// SALC/SETALC
			main[ 0xd6_e ] = nonary("salc",salc);

			// SAR
			main[ 0xd0_e >> rmbyte7         ] = binary("sar",decode_m,constant(1),sar);
			main[ 0xd1_e >> rm7             ] = binary("sar",decode_m,constant(1),sar);
			main[ 0xd2_e >> rmbyte7         ] = binary("sar",decode_m,CF,sar);
			main[ 0xd3_e >> rm7             ] = binary("sar",decode_m,CF,sar);
			main[ 0xc0_e >> rmbyte7 >> imm8 ] = binary("sar",decode_mi,sar);
			main[ 0xc1_e >> rm7 >> imm8     ] = binary("sar",decode_mi,sar);

			// SBB
			main[ *lock_prfx >> 0x1c_e >> imm8            ] = binary("sbb",al,decode_imm,sbb);
			main[ *lock_prfx >> 0x1d_e >> imm             ] = binary("sbb",decode_i,sbb);
			main[ *lock_prfx >> 0x80_e >> rmbyte3 >> imm8 ] = binary("sbb",decode_mi,sbb);
			main[ *lock_prfx >> 0x81_e >> rm3 >> imm      ] = binary("sbb",decode_mi,sbb);
			main[ *lock_prfx >> 0x83_e >> rm3 >> imm8	    ] = binary("sbb",decode_mi,sbb);
			main[ *lock_prfx >> 0x18_e >> rmbyte          ] = binary("sbb",decode_mr,sbb);
			main[ *lock_prfx >> 0x19_e >> rm              ] = binary("sbb",decode_mr,sbb);
			main[ *lock_prfx >> 0x1a_e >> rmbyte          ] = binary("sbb",decode_rm,sbb);
			main[ *lock_prfx >> 0x1b_e >> rm              ] = binary("sbb",decode_rm,sbb);

			// SCAS* (rep*)
			mainrep[ 0xae_e ] = scas;
			mainrep[ 0xaf_e ] = scas;

			// SETcc
			main[ 0x0f_e >> 0x90_e >> rmbyte ] = unary("seto",decode_m,std::bind(setcc,pls::_1,pls::_2,Overflow));
			main[ 0x0f_e >> 0x91_e >> rmbyte ] = unary("setno",decode_m,std::bind(setcc,pls::_1,pls::_2,NotOverflow));
			main[ 0x0f_e >> 0x92_e >> rmbyte ] = unary("setc",decode_m,std::bind(setcc,pls::_1,pls::_2,Carry));
			main[ 0x0f_e >> 0x93_e >> rmbyte ] = unary("setae",decode_m,std::bind(setcc,pls::_1,pls::_2,AboveEqual));
			main[ 0x0f_e >> 0x94_e >> rmbyte ] = unary("sete",decode_m,std::bind(setcc,pls::_1,pls::_2,Equal));
			main[ 0x0f_e >> 0x95_e >> rmbyte ] = unary("setne",decode_m,std::bind(setcc,pls::_1,pls::_2,NotEqual));
			main[ 0x0f_e >> 0x96_e >> rmbyte ] = unary("setbe",decode_m,std::bind(setcc,pls::_1,pls::_2,BelowEqual));
			main[ 0x0f_e >> 0x97_e >> rmbyte ] = unary("seta",decode_m,std::bind(setcc,pls::_1,pls::_2,Above));
			main[ 0x0f_e >> 0x98_e >> rmbyte ] = unary("sets",decode_m,std::bind(setcc,pls::_1,pls::_2,Sign));
			main[ 0x0f_e >> 0x99_e >> rmbyte ] = unary("setns",decode_m,std::bind(setcc,pls::_1,pls::_2,NotSign));
			main[ 0x0f_e >> 0x9a_e >> rmbyte ] = unary("setp",decode_m,std::bind(setcc,pls::_1,pls::_2,Parity));
			main[ 0x0f_e >> 0x9b_e >> rmbyte ] = unary("setnp",decode_m,std::bind(setcc,pls::_1,pls::_2,NotParity));
			main[ 0x0f_e >> 0x9c_e >> rmbyte ] = unary("setl",decode_m,std::bind(setcc,pls::_1,pls::_2,Less));
			main[ 0x0f_e >> 0x9d_e >> rmbyte ] = unary("setge",decode_m,std::bind(setcc,pls::_1,pls::_2,GreaterEqual));
			main[ 0x0f_e >> 0x9e_e >> rmbyte ] = unary("setle",decode_m,std::bind(setcc,pls::_1,pls::_2,LessEqual));
			main[ 0x0f_e >> 0x9f_e >> rmbyte ] = unary("setg",decode_m,std::bind(setcc,pls::_1,pls::_2,Greater));

			// SHLD
			main[ 0x0f_e >> 0xa4_e >> rm >> imm8 ] = trinary("shld",decode_mri,shld);
			main[ 0x0f_e >> 0xa5_e >> rm         ] = trinary("shld",decode_mr,CF,shld);

			// SHR
			main[ 0xd0_e >> rmbyte5         ] = binary("shr",decode_m,constant(1),shr);
			main[ 0xd1_e >> rm5             ] = binary("shr",decode_m,constant(1),shr);
			main[ 0xd2_e >> rmbyte5         ] = binary("shr",decode_m,CF,shr);
			main[ 0xd3_e >> rm5             ] = binary("shr",decode_m,CF,shr);
			main[ 0xc0_e >> rmbyte5 >> imm8 ] = binary("shr",decode_mi,shr);
			main[ 0xc1_e >> rm5 >> imm8     ] = binary("shr",decode_mi,shr);

			// SHRD
			main[ 0x0f_e >> 0xac_e >> rm >> imm8 ] = trinary("shrd",decode_mri,shrd);
			main[ 0x0f_e >> 0xad_e >> rm         ] = trinary("shrd",decode_mr,CF,shrd);

			// STC
			main[ 0xf9_e ] = nonary("stc",std::bind(flagwr,pls::_1,CF,true));

			// STD
			main[ 0xfd_e ] = nonary("std",std::bind(flagwr,pls::_1,DF,true));

			// STI
			main[ 0xfb_e ] = nonary("sti",std::bind(flagwr,pls::_1,IF,true));

			// STOS* (rep)
			mainrep[ 0xaa_e ] = stos;
			mainrep[ 0xab_e ] = stos;

			// SUB
			main[ *lock_prfx >> 0x2c_e >> imm8        ] = binary("sub",al,decode_imm,sub);
			main[ *lock_prfx >> 0x2d_e >> imm         ] = binary("sub",decode_i,sub);
			main[ *lock_prfx >> 0x81_e >> rm5 >> imm  ] = binary("sub",decode_mi,sub);
			main[ *lock_prfx >> 0x83_e >> rm5 >> imm8 ] = binary("sub",decode_mi,sub);
			main[ *lock_prfx >> 0x28_e >> rmbyte      ] = binary("sub",decode_mr,sub);
			main[ *lock_prfx >> 0x29_e >> rm          ] = binary("sub",decode_mr,sub);
			main[ *lock_prfx >> 0x2a_e >> rmbyte      ] = binary("sub",decode_rm,sub);
			main[ *lock_prfx >> 0x2b_e >> rm          ] = binary("sub",decode_rm,sub);

			// TEST
			main[  0xa8_e >> imm8            ] = binary("test",al,decode_imm,test);
			main[  0xa9_e >> imm             ] = binary("test",decode_i,test);
			main[  0xf6_e >> rmbyte0 >> imm8 ] = binary("test",decode_mi,test);
			main[  0xf7_e >> rm0 >> imm      ] = binary("test",decode_mi,test);
			main[ 0x84_e >> rmbyte           ] = binary("test",decode_mr,test);
			main[  0x85_e >> rm              ] = binary("test",decode_mr,test);

			// UD1
			main[ 0x0f_e >> 0xb9_e ] = nonary("ud1",ud1);

			// UD2
			main[ 0x0f_e >> 0x0b_e ] = nonary("ud2",ud2);

			// XADD (lock)
			main[ 0x0f_e >> 0xc0_e >> rmbyte ] = binary("xadd",decode_mr,xadd);
			main[ 0x0f_e >> 0xc1_e >> rm     ] = binary("xadd",decode_mr,xadd);

			// XCHG (lock)
			main[ 0x90_e           ] = binary("xchg",regb_a,regd_a,xchg);
			main[ 0x91_e           ] = binary("xchg",regb_a,regd_c,xchg);
			main[ 0x92_e           ] = binary("xchg",regb_a,regd_d,xchg);
			main[ 0x93_e           ] = binary("xchg",regb_a,regd_b,xchg);
			main[ 0x94_e           ] = binary("xchg",regb_a,regd_sp,xchg);
			main[ 0x95_e           ] = binary("xchg",regb_a,regd_bp,xchg);
			main[ 0x96_e           ] = binary("xchg",regb_a,regd_si,xchg);
			main[ 0x97_e           ] = binary("xchg",regb_a,regd_di,xchg);
			main[ 0x86_e >> rmbyte ] = binary("xchg",decode_mr,xchg);
			main[ 0x87_e >> rm     ] = binary("xchg",decode_mr,xchg);

			// XOR
			main[ *lock_prfx >> 0x34_e >> imm8        ] = binary("xor",al,decode_imm,xor_);
			main[ *lock_prfx >> 0x35_e >> imm         ] = binary("xor",decode_i,xor_);
			main[ *lock_prfx >> 0x81_e >> rm6 >> imm  ] = binary("xor",decode_mi,xor_);
			main[ *lock_prfx >> 0x83_e >> rm6 >> imm8 ] = binary("xor",decode_mi,xor_);
			main[ *lock_prfx >> 0x30_e >> rmbyte      ] = binary("xor",decode_mr,xor_);
			main[ *lock_prfx >> 0x31_e >> rm          ] = binary("xor",decode_mr,xor_);
			main[ *lock_prfx >> 0x32_e >> rmbyte      ] = binary("xor",decode_rm,xor_);
			main[ *lock_prfx >> 0x33_e >> rm          ] = binary("xor",decode_rm,xor_);
		}
	}
}
