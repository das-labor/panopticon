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
				main, mainrep, mainrepx, rex_prfx, opsize_prfx,
				addrsize_prfx, rep_prfx, repx_prfx, lock_prfx,
				imm8, imm16, imm32, imm48, imm64, imm, immlong,
				moffs8, moffs,
				sib,
				rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7,
				rmbyte, rmbyte0, rmbyte1, rmbyte2, rmbyte3,
				rmbyte4, rmbyte5, rmbyte6, rmbyte7,
				rmlong,
				disp8, disp16, disp32, disp64,
				m64, m128,
				r16, r32, r64;

			opsize_prfx[ 0x66 ] = [](sm& st)
			{
				switch(st.state.mode)
				{
					case amd64_state::RealMode:
						st.state.op_sz = amd64_state::OpSz_32; break;
					case amd64_state::ProtectedMode:
						st.state.op_sz = amd64_state::OpSz_16; break; // assumes CS.d == 1
					case amd64_state::LongMode:
						st.state.op_sz = amd64_state::OpSz_16; break;
					default: ensure(false);
				}
			};

			addrsize_prfx[ 0x67 ] = [](sm& st)
			{
				switch(st.state.mode)
				{
					case amd64_state::RealMode:
						st.state.addr_sz = amd64_state::AddrSz_32; break;
					case amd64_state::ProtectedMode:
						st.state.addr_sz = amd64_state::AddrSz_16; break; // assumes CS.d == 1
					case amd64_state::LongMode:
						st.state.addr_sz = amd64_state::AddrSz_32; break;
					default: ensure(false);
				}
			};

			rep_prfx[ 0xf3 ] = [](sm& st) {};

			repx_prfx[ 0xf3 ] = [](sm& st) {};
			repx_prfx[ 0xf2 ] = [](sm& st) {};

			rex_prfx[ "0100 w@. r@. x@. b@."_e ] = [](sm& st)
			{
				st.state.rex = true;
				if(st.capture_groups.at("w") == 1)
					st.state.op_sz = amd64_state::OpSz_64;
			};

			imm8 [ "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
			};
			imm16[ imm8 >> "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
			};
			imm32[ imm16 >> "imm@........"_e >> "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
			};
			imm48[ imm32 >> "imm@........"_e >> "imm@........"_e ] = [](sm& st)
			{
				uint64_t a = st.capture_groups.at("imm") & 0xffff;

				st.state.imm = constant((a << 32) | st.capture_groups.at("imm") >> 16);
			};
			imm64[ imm32 >> "imm@........"_e >> "imm@........"_e >> "imm@........"_e >> "imm@........"_e] = [](sm& st)
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
			};

			imm [ "imm@........"_e] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_8;
			});
			imm [ "imm@........"_e >>  "imm@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_16;
			});
			imm [ "imm@........"_e >>  "imm@........"_e >> "imm@........"_e >>  "imm@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_32 || st.state.op_sz == amd64_state::OpSz_64;
			});

			immlong [ "imm@........"_e] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_8;
			});
			immlong [ "imm@........"_e >>  "imm@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_16;
			});
			immlong [ "imm@........"_e >>  "imm@........"_e >> "imm@........"_e >>  "imm@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_32;
			});
			immlong [ "imm@........"_e >>  "imm@........"_e >> "imm@........"_e >>  "imm@........"_e >>
								"imm@........"_e >>  "imm@........"_e >> "imm@........"_e >>  "imm@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_64;
			});

			moffs [ "moffs@........"_e >>  "moffs@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				return st.state.addr_sz == amd64_state::AddrSz_16;
			});
			moffs [ "moffs@........"_e >>  "moffs@........"_e >> "moffs@........"_e >>  "moffs@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				return st.state.addr_sz == amd64_state::AddrSz_32;
			});
			moffs [ "moffs@........"_e >>  "moffs@........"_e >> "moffs@........"_e >>  "moffs@........"_e >>
			        "moffs@........"_e >>  "moffs@........"_e >> "moffs@........"_e >>  "moffs@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				return st.state.addr_sz == amd64_state::AddrSz_64;
			});

			moffs8 [ "moffs@........"_e >>  "moffs@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				st.state.op_sz = amd64_state::OpSz_8;
				return st.state.addr_sz == amd64_state::AddrSz_16;
			});
			moffs8 [ "moffs@........"_e >>  "moffs@........"_e >> "moffs@........"_e >>  "moffs@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				st.state.op_sz = amd64_state::OpSz_8;
				return st.state.addr_sz == amd64_state::AddrSz_32;
			});
			moffs8 [ "moffs@........"_e >>  "moffs@........"_e >> "moffs@........"_e >>  "moffs@........"_e >>
			         "moffs@........"_e >>  "moffs@........"_e >> "moffs@........"_e >>  "moffs@........"_e ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				st.state.op_sz = amd64_state::OpSz_8;
				return st.state.addr_sz == amd64_state::AddrSz_64;
			});

			disp8 [ "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
			};
			disp16[ disp8 >> "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
			};
			disp32[ disp16 >> "disp@........"_e >> "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
			};
			disp64[ disp32 >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e] = [](sm& st)
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
			};

			// sib
			sib [ "scale@.. index@... base@101"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e	] = [](sm& st)
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
			};
			sib [ "scale@.. index@... base@..."_e] = [](sm& st) {};

			std::function<void(boost::optional<amd64_state::OperandSize>,sm&)> rm_func = [&](boost::optional<amd64_state::OperandSize> os,sm& st)
			{
				ensure(!st.state.reg && !st.state.rm);

				if(os)
				{
					if(*os == amd64_state::OpSz_64)
						st.state.op_sz = st.state.mode == amd64_state::LongMode ? *os : amd64_state::OpSz_32;
					else
						st.state.op_sz = *os;
				}

				if(st.capture_groups.count("reg"))
				{
					unsigned int reg = (st.capture_groups.count("r") && st.capture_groups.at("r") == 1 ? 8 : 0) + st.capture_groups.at("reg");
					st.state.reg = select_reg(st.state.op_sz,reg,st.state.rex);
				}

				boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
				unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

				if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
				{
					unsigned int scale = st.capture_groups.at("scale");
					unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
					unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

					sib = std::make_tuple(scale,x_index,b_base);
				}

				st.mnemonic(0,"internal-rm","",[&](cg& c) -> std::list<rvalue>
				{
					st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,st.state.op_sz,st.state.addr_sz,st.state.mode,st.state.rex,c);
					return {};
				});
			};

			// mod = 00
			rm [ "mod@00 reg@... rm@101"_e >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ "mod@00 reg@... rm@100"_e >> sib    ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ "mod@00 reg@... rm@..."_e           ] = std::bind(rm_func,boost::none,pls::_1);
			rmlong [ "mod@00 reg@... rm@101"_e >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ "mod@00 reg@... rm@100"_e >> sib    ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ "mod@00 reg@... rm@..."_e           ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmbyte [ "mod@00 reg@... rm@101"_e >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ "mod@00 reg@... rm@100"_e >> sib    ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ "mod@00 reg@... rm@..."_e           ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 00 w/ extension opcode
			rm0 [ "mod@00 000 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ "mod@00 000 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ "mod@00 000 rm@..."_e					  ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ "mod@00 000 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ "mod@00 000 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ "mod@00 000 rm@..."_e				  	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm1 [ "mod@00 001 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ "mod@00 001 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ "mod@00 001 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte1 [ "mod@00 001 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ "mod@00 001 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ "mod@00 001 rm@..."_e		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm2 [ "mod@00 010 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ "mod@00 010 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ "mod@00 010 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte2 [ "mod@00 010 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ "mod@00 010 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ "mod@00 010 rm@..."_e		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm3 [ "mod@00 011 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ "mod@00 011 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ "mod@00 011 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte3 [ "mod@00 011 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ "mod@00 011 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ "mod@00 011 rm@..."_e		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm4 [ "mod@00 100 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ "mod@00 100 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ "mod@00 100 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte4 [ "mod@00 100 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ "mod@00 100 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ "mod@00 100 rm@..."_e		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm5 [ "mod@00 101 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ "mod@00 101 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ "mod@00 101 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte5 [ "mod@00 101 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ "mod@00 101 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ "mod@00 101 rm@..."_e		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm6 [ "mod@00 110 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ "mod@00 110 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ "mod@00 110 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte6 [ "mod@00 110 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ "mod@00 110 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ "mod@00 110 rm@..."_e				  	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm7 [ "mod@00 111 rm@101"_e >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ "mod@00 111 rm@100"_e >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ "mod@00 111 rm@..."_e				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte7 [ "mod@00 111 rm@101"_e >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ "mod@00 111 rm@100"_e >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ "mod@00 111 rm@..."_e		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 01
			rm [ "mod@01 reg@... rm@100"_e >> sib >> disp32     ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ "mod@01 reg@... rm@..."_e >> disp8	            ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte [ "mod@01 reg@... rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ "mod@01 reg@... rm@..."_e >> disp8	        ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmlong [ "mod@01 reg@... rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ "mod@01 reg@... rm@..."_e >> disp8	        ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);

			// mod = 01 w/ opcode extension
			rm0 [ "mod@01 000 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ "mod@01 000 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ "mod@01 000 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ "mod@01 000 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm1 [ "mod@01 001 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ "mod@01 001 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte1 [ "mod@01 001 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ "mod@01 001 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm2 [ "mod@01 010 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ "mod@01 010 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte2 [ "mod@01 010 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ "mod@01 010 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm3 [ "mod@01 011 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ "mod@01 011 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte3 [ "mod@01 011 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ "mod@01 011 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm4 [ "mod@01 100 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ "mod@01 100 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte4 [ "mod@01 100 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ "mod@01 100 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm5 [ "mod@01 101 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ "mod@01 101 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte5 [ "mod@01 101 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ "mod@01 101 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm6 [ "mod@01 110 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ "mod@01 110 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte6 [ "mod@01 110 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ "mod@01 110 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm7 [ "mod@01 111 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ "mod@01 111 rm@..."_e >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte7 [ "mod@01 111 rm@100"_e >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ "mod@01 111 rm@..."_e >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 10
			rm [ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ "mod@10 reg@... rm@..."_e >> disp32	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte [ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ "mod@10 reg@... rm@..."_e >> disp32	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmlong [ "mod@10 reg@... rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ "mod@10 reg@... rm@..."_e >> disp32	      ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);

			// mod = 10 w/ opcode extension
			rm0 [ "mod@10 000 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ "mod@10 000 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ "mod@10 000 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ "mod@10 000 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm1 [ "mod@10 001 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ "mod@10 001 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte1 [ "mod@10 001 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ "mod@10 001 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm2 [ "mod@10 010 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ "mod@10 010 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte2 [ "mod@10 010 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ "mod@10 010 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm3 [ "mod@10 011 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ "mod@10 011 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte3 [ "mod@10 011 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ "mod@10 011 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm4 [ "mod@10 100 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ "mod@10 100 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte4 [ "mod@10 100 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ "mod@10 100 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm5 [ "mod@10 101 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ "mod@10 101 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte5 [ "mod@10 101 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ "mod@10 101 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm6 [ "mod@10 110 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ "mod@10 110 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte6 [ "mod@10 110 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ "mod@10 110 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm7 [ "mod@10 111 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ "mod@10 111 rm@..."_e >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte7 [ "mod@10 111 rm@100"_e >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ "mod@10 111 rm@..."_e >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 11
			rm [ "mod@11 reg@... rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte [ "mod@11 reg@... rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmlong [ "mod@11 reg@... rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);

			// mod = 11 w/ opcode extension
			rm0 [ "mod@11 000 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ "mod@11 001 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ "mod@11 010 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ "mod@11 011 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ "mod@11 100 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ "mod@11 101 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ "mod@11 110 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ "mod@11 111 rm@..."_e ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ "mod@11 000 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ "mod@11 001 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ "mod@11 010 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ "mod@11 011 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ "mod@11 100 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ "mod@11 101 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ "mod@11 110 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ "mod@11 111 rm@..."_e ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			add_generic<Bits>(
				main, mainrep, mainrepx,
				lock_prfx,
				imm8, imm16, imm32, imm48, imm64, imm, immlong,
				moffs8, moffs,
				sib,
				rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7,
				rmbyte, rmbyte0, rmbyte1, rmbyte2, rmbyte3,
				rmbyte4, rmbyte5, rmbyte6, rmbyte7,
				rmlong,
				disp8, disp16, disp32, disp64,
				m64, m128,
				r16, r32, r64);

			if(Bits == 64)
			{
				dis amd64;

				amd64[ *opsize_prfx >> *rex_prfx >> main ];
				amd64[ *rep_prfx >> *opsize_prfx >> *rep_prfx >> *rex_prfx >> mainrep ];
				amd64[ *rep_prfx >> *opsize_prfx >> *repx_prfx >> *rex_prfx >> mainrepx ];
				return program::disassemble<amd64_tag>(amd64,amd64_state(amd64_state::LongMode),bytes,r,prog);
			}
			else
			{
				dis intel;

				intel[ *rep_prfx >> *opsize_prfx >> *rep_prfx >> mainrep ];
				intel[ *rep_prfx >> *opsize_prfx >> *repx_prfx >> mainrepx ];
				intel[ *opsize_prfx >> main ];
				return program::disassemble<amd64_tag>(intel,amd64_state(Bits == 32 ? amd64_state::ProtectedMode : amd64_state::RealMode),bytes,r,prog);
			}
		}
	}
}
