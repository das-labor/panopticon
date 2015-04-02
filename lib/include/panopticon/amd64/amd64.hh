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

#include <panopticon/program.hh>
#include <panopticon/amd64/syntax.hh>
#include <panopticon/amd64/traits.hh>
#include <panopticon/amd64/decode.hh>

#pragma once

namespace po
{
	namespace amd64
	{
		template<int Bits>
		boost::optional<prog_loc> disassemble(boost::optional<prog_loc> prog, po::slab bytes, const po::ref& r)
		{
			disassembler<amd64_tag>
				main, opsize_prfx, rex_prfx, rexw_prfx, rexr_prfx,
				addrsize_prfx, rep_prfx, repx_prfx, lock_prfx,
				imm8, imm16, imm32, imm48, imm64,
				sib,
				rm8, rm16, rm32, rm64,
				rm8_0, rm16_0, rm32_0, rm64_0,
				rm8_1, rm16_1, rm32_1, rm64_1,
				rm8_2, rm16_2, rm32_2, rm64_2,
				rm8_3, rm16_3, rm32_3, rm64_3,
				rm8_4, rm16_4, rm32_4, rm64_4,
				rm8_5, rm16_5, rm32_5, rm64_5,
				rm8_6, rm16_6, rm32_6, rm64_6,
				rm8_7, rm16_7, rm32_7, rm64_7,
				disp8, disp16, disp32, disp64,
				m64, m128,
				r16, r32, r64;

			opsize_prfx[ 0x66 ] = [](sm& st)
			{
				switch(st.state.mode)
				{
					case amd64_state::RealMode:		st.state.op_sz = amd64_state::OpSz_32; break;
					case amd64_state::ProtectedMode:	st.state.op_sz = amd64_state::OpSz_16; break; // assumes CS.d == 1
					case amd64_state::LongMode:		st.state.op_sz = amd64_state::OpSz_16; break;
					default: ensure(false);
				}
			};

			addrsize_prfx[ 0x67 ] = [](sm& st)
			{
				switch(st.state.mode)
				{
					case amd64_state::RealMode:		st.state.addr_sz = amd64_state::AddrSz_32; break;
					case amd64_state::ProtectedMode:	st.state.addr_sz = amd64_state::AddrSz_16; break; // assumes CS.d == 1
					case amd64_state::LongMode:		st.state.addr_sz = amd64_state::AddrSz_32; break;
					default: ensure(false);
				}
			};

			rep_prfx[ 0xf3 ] = [](sm& st) {};

			repx_prfx[ 0xf3 ] = [](sm& st) {};
			repx_prfx[ 0xf2 ] = [](sm& st) {};

			rex_prfx [ "0100 w@0 r@. x@. b@."_e ] = [](sm& st) { st.state.rex = true; };
			rexw_prfx[ "0100 w@1 r@. x@. b@."_e ] = [](sm& st) { st.state.rex = true; st.state.op_sz = amd64_state::OpSz_64; };
			rexr_prfx[ "0100 w@. r@1 x@. b@."_e ] = [](sm& st) { st.state.rex = true; };

			imm8 [ "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
			};
			imm16[ imm8 >> "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(be16toh(st.capture_groups.at("imm")));
			};
			imm32[ imm16 >> "imm@........"_e >> "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(be32toh(st.capture_groups.at("imm")));
			};
			imm48[ imm32 >> "imm@........"_e >> "imm@........"_e ] = [](sm& st)
			{
				uint64_t a = be16toh(st.capture_groups.at("imm") & 0xffff);

				st.state.imm = constant((a << 32) | be32toh(st.capture_groups.at("imm") >> 16));
			};
			imm64[ imm32 >> "imm@........"_e >> "imm@........"_e >> "imm@........"_e >> "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(be64toh(st.capture_groups.at("imm")));
			};

			disp8 [ "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
			};
			disp16[ disp8 >> "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(be16toh(st.capture_groups.at("disp")));
			};
			disp32[ disp16 >> "disp@........"_e >> "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(be32toh(st.capture_groups.at("disp")));
			};
			disp64[ disp32 >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(be64toh(st.capture_groups.at("disp")));
			};

			// sib
			sib [ "scale@.. index@... base@101"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e	] = [](sm& st)
			{
				st.state.disp = constant(be32toh(st.capture_groups.at("disp")));
			};
			sib [ "scale@.. index@... base@..."_e] = [](sm& st) {};

			std::function<void(sm&)> rm8_func = [&](sm& st)
			{
				ensure(!st.state.reg && !st.state.rm);

				if(st.capture_groups.count("reg"))
					st.state.reg = decode_reg8((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"),st.state.rex);

				boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
				unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

				if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
				{
					unsigned int scale = st.capture_groups.at("scale");
					unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
					unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

					sib = std::make_tuple(scale,x_index,b_base);
				}

				st.mnemonic(0,"internal-rm8","",[&](cg& c) -> std::list<rvalue>
				{
					st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_8,st.state.addr_sz,c);
					return {};
				});
			};

			std::function<void(sm&)> rm16_func = [&](sm& st)
			{
				ensure(!st.state.reg && !st.state.rm);

				if(st.capture_groups.count("reg"))
					st.state.reg = decode_reg16((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"));

				boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
				unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

				if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
				{
					unsigned int scale = st.capture_groups.at("scale");
					unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
					unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

					sib = std::make_tuple(scale,x_index,b_base);
				}

				st.mnemonic(0,"internal-rm16","",[&](cg& c) -> std::list<rvalue>
				{
					st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_16,st.state.addr_sz,c);
					return {};
				});
			};

			std::function<void(sm&)> rm32_func = [&](sm& st)
			{
				ensure(!st.state.reg && !st.state.rm);

				if(st.capture_groups.count("reg"))
					st.state.reg = decode_reg32((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"));

				boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
				unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

				if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
				{
					unsigned int scale = st.capture_groups.at("scale");
					unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
					unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

					sib = std::make_tuple(scale,x_index,b_base);
				}

				st.mnemonic(0,"internal-rm32","",[&](cg& c) -> std::list<rvalue>
				{
					st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_32,st.state.addr_sz,c);
					return {};
				});
			};

			std::function<void(sm&)> rm64_func = [&](sm& st)
			{
				ensure(!st.state.reg && !st.state.rm);

				if(st.capture_groups.count("reg"))
					st.state.reg = decode_reg64((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"));

				boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
				unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

				if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
				{
					unsigned int scale = st.capture_groups.at("scale");
					unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
					unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

					sib = std::make_tuple(scale,x_index,b_base);
				}

				st.mnemonic(0,"internal-rm64","",[&](cg& c) -> std::list<rvalue>
				{
					st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_64,st.state.addr_sz,c);
					return {};
				});
			};

			// mod = 00
			rm8 [ "mod@00 reg@... rm@101"_e >> disp32	] = rm8_func;
			rm8 [ "mod@00 reg@... rm@100"_e >> sib		] = rm8_func;
			rm8 [ "mod@00 reg@... rm@..."_e				] = rm8_func;
			rm16[ "mod@00 reg@... rm@101"_e >> disp32	] = rm16_func;
			rm16[ "mod@00 reg@... rm@100"_e >> sib		] = rm16_func;
			rm16[ "mod@00 reg@... rm@..."_e				] = rm16_func;
			rm32[ "mod@00 reg@... rm@101"_e >> disp32	] = rm32_func;
			rm32[ "mod@00 reg@... rm@100"_e >> sib		] = rm32_func;
			rm32[ "mod@00 reg@... rm@..."_e				] = rm32_func;
			rm64[ "mod@00 reg@... rm@101"_e >> disp32	] = rm64_func;
			rm64[ "mod@00 reg@... rm@100"_e >> sib		] = rm64_func;
			rm64[ "mod@00 reg@... rm@..."_e				] = rm64_func;

			// mod = 00 w/ extension opcode
			rm8_0 [ "mod@00 000 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 000 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 000 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 000 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 000 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 000 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 000 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 000 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 000 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 000 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 000 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 000 rm@..."_e					] = rm64_func;

			rm8_1 [ "mod@00 001 rm@101"_e >> disp32	] = rm8_func;
			rm8_1 [ "mod@00 001 rm@100"_e >> sib		] = rm8_func;
			rm8_1 [ "mod@00 001 rm@..."_e					] = rm8_func;
			rm16_1[ "mod@00 001 rm@101"_e >> disp32	] = rm16_func;
			rm16_1[ "mod@00 001 rm@100"_e >> sib		] = rm16_func;
			rm16_1[ "mod@00 001 rm@..."_e					] = rm16_func;
			rm32_1[ "mod@00 001 rm@101"_e >> disp32	] = rm32_func;
			rm32_1[ "mod@00 001 rm@100"_e >> sib		] = rm32_func;
			rm32_1[ "mod@00 001 rm@..."_e					] = rm32_func;
			rm64_1[ "mod@00 001 rm@101"_e >> disp32	] = rm64_func;
			rm64_1[ "mod@00 001 rm@100"_e >> sib		] = rm64_func;
			rm64_1[ "mod@00 001 rm@..."_e					] = rm64_func;

			rm8_0 [ "mod@00 010 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 010 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 010 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 010 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 010 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 010 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 010 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 010 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 010 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 010 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 010 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 010 rm@..."_e					] = rm64_func;

			rm8_0 [ "mod@00 011 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 011 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 011 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 011 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 011 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 011 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 011 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 011 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 011 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 011 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 011 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 011 rm@..."_e					] = rm64_func;

			rm8_0 [ "mod@00 100 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 100 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 100 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 100 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 100 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 100 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 100 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 100 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 100 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 100 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 100 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 100 rm@..."_e					] = rm64_func;

			rm8_0 [ "mod@00 101 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 101 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 101 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 101 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 101 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 101 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 101 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 101 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 101 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 101 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 101 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 101 rm@..."_e					] = rm64_func;

			rm8_0 [ "mod@00 110 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 110 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 110 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 110 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 110 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 110 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 110 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 110 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 110 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 110 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 110 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 110 rm@..."_e					] = rm64_func;

			rm8_0 [ "mod@00 111 rm@101"_e >> disp32	] = rm8_func;
			rm8_0 [ "mod@00 111 rm@100"_e >> sib		] = rm8_func;
			rm8_0 [ "mod@00 111 rm@..."_e					] = rm8_func;
			rm16_0[ "mod@00 111 rm@101"_e >> disp32	] = rm16_func;
			rm16_0[ "mod@00 111 rm@100"_e >> sib		] = rm16_func;
			rm16_0[ "mod@00 111 rm@..."_e					] = rm16_func;
			rm32_0[ "mod@00 111 rm@101"_e >> disp32	] = rm32_func;
			rm32_0[ "mod@00 111 rm@100"_e >> sib		] = rm32_func;
			rm32_0[ "mod@00 111 rm@..."_e					] = rm32_func;
			rm64_0[ "mod@00 111 rm@101"_e >> disp32	] = rm64_func;
			rm64_0[ "mod@00 111 rm@100"_e >> sib		] = rm64_func;
			rm64_0[ "mod@00 111 rm@..."_e					] = rm64_func;

			// mod = 01
			rm8 [ "mod@01 reg@... rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8 [ "mod@01 reg@... rm@..."_e >> disp8	] = rm8_func;
			rm16[ "mod@01 reg@... rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16[ "mod@01 reg@... rm@..."_e >> disp8	] = rm16_func;
			rm32[ "mod@01 reg@... rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32[ "mod@01 reg@... rm@..."_e >> disp8	] = rm32_func;
			rm64[ "mod@01 reg@... rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64[ "mod@01 reg@... rm@..."_e >> disp8	] = rm64_func;

			// mod = 01 w/ opcode extension
			rm8_0 [ "mod@01 000 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_0 [ "mod@01 000 rm@..."_e >> disp8	] = rm8_func;
			rm16_0[ "mod@01 000 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_0[ "mod@01 000 rm@..."_e >> disp8	] = rm16_func;
			rm32_0[ "mod@01 000 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_0[ "mod@01 000 rm@..."_e >> disp8	] = rm32_func;
			rm64_0[ "mod@01 000 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_0[ "mod@01 000 rm@..."_e >> disp8	] = rm64_func;

			rm8_1 [ "mod@01 001 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_1 [ "mod@01 001 rm@..."_e >> disp8	] = rm8_func;
			rm16_1[ "mod@01 001 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_1[ "mod@01 001 rm@..."_e >> disp8	] = rm16_func;
			rm32_1[ "mod@01 001 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_1[ "mod@01 001 rm@..."_e >> disp8	] = rm32_func;
			rm64_1[ "mod@01 001 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_1[ "mod@01 001 rm@..."_e >> disp8	] = rm64_func;

			rm8_2 [ "mod@01 010 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_2 [ "mod@01 010 rm@..."_e >> disp8	] = rm8_func;
			rm16_2[ "mod@01 010 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_2[ "mod@01 010 rm@..."_e >> disp8	] = rm16_func;
			rm32_2[ "mod@01 010 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_2[ "mod@01 010 rm@..."_e >> disp8	] = rm32_func;
			rm64_2[ "mod@01 010 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_2[ "mod@01 010 rm@..."_e >> disp8	] = rm64_func;

			rm8_3 [ "mod@01 011 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_3 [ "mod@01 011 rm@..."_e >> disp8	] = rm8_func;
			rm16_3[ "mod@01 011 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_3[ "mod@01 011 rm@..."_e >> disp8	] = rm16_func;
			rm32_3[ "mod@01 011 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_3[ "mod@01 011 rm@..."_e >> disp8	] = rm32_func;
			rm64_3[ "mod@01 011 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_3[ "mod@01 011 rm@..."_e >> disp8	] = rm64_func;

			rm8_4 [ "mod@01 100 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_4 [ "mod@01 100 rm@..."_e >> disp8	] = rm8_func;
			rm16_4[ "mod@01 100 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_4[ "mod@01 100 rm@..."_e >> disp8	] = rm16_func;
			rm32_4[ "mod@01 100 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_4[ "mod@01 100 rm@..."_e >> disp8	] = rm32_func;
			rm64_4[ "mod@01 100 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_4[ "mod@01 100 rm@..."_e >> disp8	] = rm64_func;

			rm8_5 [ "mod@01 101 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_5 [ "mod@01 101 rm@..."_e >> disp8	] = rm8_func;
			rm16_5[ "mod@01 101 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_5[ "mod@01 101 rm@..."_e >> disp8	] = rm16_func;
			rm32_5[ "mod@01 101 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_5[ "mod@01 101 rm@..."_e >> disp8	] = rm32_func;
			rm64_5[ "mod@01 101 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_5[ "mod@01 101 rm@..."_e >> disp8	] = rm64_func;

			rm8_6 [ "mod@01 110 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_6 [ "mod@01 110 rm@..."_e >> disp8	] = rm8_func;
			rm16_6[ "mod@01 110 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_6[ "mod@01 110 rm@..."_e >> disp8	] = rm16_func;
			rm32_6[ "mod@01 110 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_6[ "mod@01 110 rm@..."_e >> disp8	] = rm32_func;
			rm64_6[ "mod@01 110 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_6[ "mod@01 110 rm@..."_e >> disp8	] = rm64_func;

			rm8_7 [ "mod@01 111 rm@100"_e >> sib >> disp8 ] = rm8_func;
			rm8_7 [ "mod@01 111 rm@..."_e >> disp8	] = rm8_func;
			rm16_7[ "mod@01 111 rm@100"_e >> sib >> disp8 ] = rm16_func;
			rm16_7[ "mod@01 111 rm@..."_e >> disp8	] = rm16_func;
			rm32_7[ "mod@01 111 rm@100"_e >> sib >> disp8 ] = rm32_func;
			rm32_7[ "mod@01 111 rm@..."_e >> disp8	] = rm32_func;
			rm64_7[ "mod@01 111 rm@100"_e >> sib >> disp8 ] = rm64_func;
			rm64_7[ "mod@01 111 rm@..."_e >> disp8	] = rm64_func;

			// mod = 10
			rm8 [ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8 [ "mod@10 reg@... rm@..."_e >> disp32	] = rm8_func;
			rm16[ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16[ "mod@10 reg@... rm@..."_e >> disp32	] = rm16_func;
			rm32[ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32[ "mod@10 reg@... rm@..."_e >> disp32	] = rm32_func;
			rm64[ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64[ "mod@10 reg@... rm@..."_e >> disp32	] = rm64_func;

			// mod = 10 w/ opcode extension
			rm8_0 [ "mod@10 000 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_0 [ "mod@10 000 rm@..."_e >> disp32	] = rm8_func;
			rm16_0[ "mod@10 000 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_0[ "mod@10 000 rm@..."_e >> disp32	] = rm16_func;
			rm32_0[ "mod@10 000 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_0[ "mod@10 000 rm@..."_e >> disp32	] = rm32_func;
			rm64_0[ "mod@10 000 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_0[ "mod@10 000 rm@..."_e >> disp32	] = rm64_func;

			rm8_1 [ "mod@10 001 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_1 [ "mod@10 001 rm@..."_e >> disp32	] = rm8_func;
			rm16_1[ "mod@10 001 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_1[ "mod@10 001 rm@..."_e >> disp32	] = rm16_func;
			rm32_1[ "mod@10 001 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_1[ "mod@10 001 rm@..."_e >> disp32	] = rm32_func;
			rm64_1[ "mod@10 001 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_1[ "mod@10 001 rm@..."_e >> disp32	] = rm64_func;

			rm8_2 [ "mod@10 010 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_2 [ "mod@10 010 rm@..."_e >> disp32	] = rm8_func;
			rm16_2[ "mod@10 010 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_2[ "mod@10 010 rm@..."_e >> disp32	] = rm16_func;
			rm32_2[ "mod@10 010 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_2[ "mod@10 010 rm@..."_e >> disp32	] = rm32_func;
			rm64_2[ "mod@10 010 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_2[ "mod@10 010 rm@..."_e >> disp32	] = rm64_func;

			rm8_3 [ "mod@10 011 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_3 [ "mod@10 011 rm@..."_e >> disp32	] = rm8_func;
			rm16_3[ "mod@10 011 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_3[ "mod@10 011 rm@..."_e >> disp32	] = rm16_func;
			rm32_3[ "mod@10 011 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_3[ "mod@10 011 rm@..."_e >> disp32	] = rm32_func;
			rm64_3[ "mod@10 011 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_3[ "mod@10 011 rm@..."_e >> disp32	] = rm64_func;

			rm8_4 [ "mod@10 100 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_4 [ "mod@10 100 rm@..."_e >> disp32	] = rm8_func;
			rm16_4[ "mod@10 100 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_4[ "mod@10 100 rm@..."_e >> disp32	] = rm16_func;
			rm32_4[ "mod@10 100 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_4[ "mod@10 100 rm@..."_e >> disp32	] = rm32_func;
			rm64_4[ "mod@10 100 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_4[ "mod@10 100 rm@..."_e >> disp32	] = rm64_func;

			rm8_5 [ "mod@10 101 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_5 [ "mod@10 101 rm@..."_e >> disp32	] = rm8_func;
			rm16_5[ "mod@10 101 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_5[ "mod@10 101 rm@..."_e >> disp32	] = rm16_func;
			rm32_5[ "mod@10 101 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_5[ "mod@10 101 rm@..."_e >> disp32	] = rm32_func;
			rm64_5[ "mod@10 101 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_5[ "mod@10 101 rm@..."_e >> disp32	] = rm64_func;

			rm8_6 [ "mod@10 110 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_6 [ "mod@10 110 rm@..."_e >> disp32	] = rm8_func;
			rm16_6[ "mod@10 110 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_6[ "mod@10 110 rm@..."_e >> disp32	] = rm16_func;
			rm32_6[ "mod@10 110 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_6[ "mod@10 110 rm@..."_e >> disp32	] = rm32_func;
			rm64_6[ "mod@10 110 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_6[ "mod@10 110 rm@..."_e >> disp32	] = rm64_func;

			rm8_7 [ "mod@10 111 rm@100"_e >> sib >> disp32 ] = rm8_func;
			rm8_7 [ "mod@10 111 rm@..."_e >> disp32	] = rm8_func;
			rm16_7[ "mod@10 111 rm@100"_e >> sib >> disp32 ] = rm16_func;
			rm16_7[ "mod@10 111 rm@..."_e >> disp32	] = rm16_func;
			rm32_7[ "mod@10 111 rm@100"_e >> sib >> disp32 ] = rm32_func;
			rm32_7[ "mod@10 111 rm@..."_e >> disp32	] = rm32_func;
			rm64_7[ "mod@10 111 rm@100"_e >> sib >> disp32 ] = rm64_func;
			rm64_7[ "mod@10 111 rm@..."_e >> disp32	] = rm64_func;

			// mod = 11
			rm8 [ "mod@11 reg@... rm@..."_e ] = rm8_func;
			rm16[ "mod@11 reg@... rm@..."_e ] = rm16_func;
			rm32[ "mod@11 reg@... rm@..."_e ] = rm32_func;
			rm64[ "mod@11 reg@... rm@..."_e ] = rm64_func;

			// mod = 11 w/ opcode extension
			rm8_0 [ "mod@11 000 rm@..."_e ] = rm8_func;
			rm16_0[ "mod@11 000 rm@..."_e ] = rm16_func;
			rm32_0[ "mod@11 000 rm@..."_e ] = rm32_func;
			rm64_0[ "mod@11 000 rm@..."_e ] = rm64_func;

			rm8_1 [ "mod@11 001 rm@..."_e ] = rm8_func;
			rm16_1[ "mod@11 001 rm@..."_e ] = rm16_func;
			rm32_1[ "mod@11 001 rm@..."_e ] = rm32_func;
			rm64_1[ "mod@11 001 rm@..."_e ] = rm64_func;

			rm8_2 [ "mod@11 010 rm@..."_e ] = rm8_func;
			rm16_2[ "mod@11 010 rm@..."_e ] = rm16_func;
			rm32_2[ "mod@11 010 rm@..."_e ] = rm32_func;
			rm64_2[ "mod@11 010 rm@..."_e ] = rm64_func;

			rm8_3 [ "mod@11 011 rm@..."_e ] = rm8_func;
			rm16_3[ "mod@11 011 rm@..."_e ] = rm16_func;
			rm32_3[ "mod@11 011 rm@..."_e ] = rm32_func;
			rm64_3[ "mod@11 011 rm@..."_e ] = rm64_func;

			rm8_4 [ "mod@11 100 rm@..."_e ] = rm8_func;
			rm16_4[ "mod@11 100 rm@..."_e ] = rm16_func;
			rm32_4[ "mod@11 100 rm@..."_e ] = rm32_func;
			rm64_4[ "mod@11 100 rm@..."_e ] = rm64_func;

			rm8_5 [ "mod@11 101 rm@..."_e ] = rm8_func;
			rm16_5[ "mod@11 101 rm@..."_e ] = rm16_func;
			rm32_5[ "mod@11 101 rm@..."_e ] = rm32_func;
			rm64_5[ "mod@11 101 rm@..."_e ] = rm64_func;

			rm8_6 [ "mod@11 110 rm@..."_e ] = rm8_func;
			rm16_6[ "mod@11 110 rm@..."_e ] = rm16_func;
			rm32_6[ "mod@11 110 rm@..."_e ] = rm32_func;
			rm64_6[ "mod@11 110 rm@..."_e ] = rm64_func;

			rm8_7 [ "mod@11 111 rm@..."_e ] = rm8_func;
			rm16_7[ "mod@11 111 rm@..."_e ] = rm16_func;
			rm32_7[ "mod@11 111 rm@..."_e ] = rm32_func;
			rm64_7[ "mod@11 111 rm@..."_e ] = rm64_func;

			add_generic<Bits>(
				main, opsize_prfx, rex_prfx, rexw_prfx, rexr_prfx,
				lock_prfx, addrsize_prfx, rep_prfx, repx_prfx,
				imm8, imm16, imm32, imm48, imm64,
				sib,
				rm8, rm16, rm32, rm64,
				rm8_0, rm16_0, rm32_0, rm64_0,
				rm8_1, rm16_1, rm32_1, rm64_1,
				rm8_2, rm16_2, rm32_2, rm64_2,
				rm8_3, rm16_3, rm32_3, rm64_3,
				rm8_4, rm16_4, rm32_4, rm64_4,
				rm8_5, rm16_5, rm32_5, rm64_5,
				rm8_6, rm16_6, rm32_6, rm64_6,
				rm8_7, rm16_7, rm32_7, rm64_7,
				disp8, disp16, disp32, disp64,
				m64, m128,
				r16, r32, r64);

			return program::disassemble<amd64_tag>(main,bytes,r,prog);
		}
	}
}
