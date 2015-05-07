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

#include <panopticon/program.hh>
#include <panopticon/amd64/syntax.hh>
#include <panopticon/amd64/traits.hh>
#include <panopticon/amd64/decode.hh>

#pragma once

#define f(x) token_expr(std::string(x))

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

			opsize_prfx[ 0x66 ] = [](sm& st) -> bool
			{
				switch(st.state.mode)
				{
					case amd64_state::RealMode:
						st.state.op_sz = amd64_state::OpSz_32; return true;
					case amd64_state::ProtectedMode:
						st.state.op_sz = amd64_state::OpSz_16; return true; // assumes CS.d == 1
					case amd64_state::LongMode:
						st.state.op_sz = amd64_state::OpSz_16; return true;
					default: ensure(false);
				}
			};

			addrsize_prfx[ 0x67 ] = [](sm& st) -> bool
			{
				switch(st.state.mode)
				{
					case amd64_state::RealMode:
						st.state.addr_sz = amd64_state::AddrSz_32; return true;
					case amd64_state::ProtectedMode:
						st.state.addr_sz = amd64_state::AddrSz_16; return true; // assumes CS.d == 1
					case amd64_state::LongMode:
						st.state.addr_sz = amd64_state::AddrSz_32; return true;
					default: ensure(false);
				}
			};

			rep_prfx[ 0xf3 ] = [](sm& st) -> bool { return true; };

			repx_prfx[ 0xf3 ] = [](sm& st) -> bool { return true; };
			repx_prfx[ 0xf2 ] = [](sm& st) -> bool { return true; };

			rex_prfx[ f("0100 w@. r@. x@. b@.") ] = [](sm& st) -> bool
			{
				st.state.rex = true;
				if(st.capture_groups.at("w") == 1)
					st.state.op_sz = amd64_state::OpSz_64;
				return true;
			};

			imm8 [ f("imm@........")] = [](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return true;
			};
			imm16[ imm8 >> f("imm@........")] = [](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return true;
			};
			imm32[ imm16 >> f("imm@........") >> f("imm@........")] = [](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return true;
			};
			imm48[ imm32 >> f("imm@........") >> f("imm@........") ] = [](sm& st) -> bool
			{
				uint64_t a = st.capture_groups.at("imm") & 0xffff;

				st.state.imm = constant((a << 32) | st.capture_groups.at("imm") >> 16);
				return true;
			};
			imm64[ imm32 >> f("imm@........") >> f("imm@........") >> f("imm@........") >> f("imm@........")] = [](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return true;
			};

			imm [ f("imm@........")] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_8;
			});
			imm [ f("imm@........") >>  f("imm@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_16;
			});
			imm [ f("imm@........") >>  f("imm@........") >> f("imm@........") >>  f("imm@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_32 || st.state.op_sz == amd64_state::OpSz_64;
			});

			immlong [ f("imm@........")] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_8;
			});
			immlong [ f("imm@........") >>  f("imm@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_16;
			});
			immlong [ f("imm@........") >>  f("imm@........") >> f("imm@........") >>  f("imm@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_32;
			});
			immlong [ f("imm@........") >>  f("imm@........") >> f("imm@........") >>  f("imm@........") >>
								f("imm@........") >>  f("imm@........") >> f("imm@........") >>  f("imm@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.imm = constant(st.capture_groups.at("imm"));
				return st.state.op_sz == amd64_state::OpSz_64;
			});

			moffs [ f("moffs@........") >>  f("moffs@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				return st.state.addr_sz == amd64_state::AddrSz_16;
			});
			moffs [ f("moffs@........") >>  f("moffs@........") >> f("moffs@........") >>  f("moffs@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				return st.state.addr_sz == amd64_state::AddrSz_32;
			});
			moffs [ f("moffs@........") >>  f("moffs@........") >> f("moffs@........") >>  f("moffs@........") >>
			        f("moffs@........") >>  f("moffs@........") >> f("moffs@........") >>  f("moffs@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				return st.state.addr_sz == amd64_state::AddrSz_64;
			});

			moffs8 [ f("moffs@........") >>  f("moffs@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				st.state.op_sz = amd64_state::OpSz_8;
				return st.state.addr_sz == amd64_state::AddrSz_16;
			});
			moffs8 [ f("moffs@........") >>  f("moffs@........") >> f("moffs@........") >>  f("moffs@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				st.state.op_sz = amd64_state::OpSz_8;
				return st.state.addr_sz == amd64_state::AddrSz_32;
			});
			moffs8 [ f("moffs@........") >>  f("moffs@........") >> f("moffs@........") >>  f("moffs@........") >>
			         f("moffs@........") >>  f("moffs@........") >> f("moffs@........") >>  f("moffs@........") ] = std::function<bool(sm&)>([](sm& st) -> bool
			{
				st.state.moffs = constant(st.capture_groups.at("moffs"));
				st.state.op_sz = amd64_state::OpSz_8;
				return st.state.addr_sz == amd64_state::AddrSz_64;
			});

			disp8 [ f("disp@........")] = [](sm& st) -> bool
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
				return true;
			};
			disp16[ disp8 >> f("disp@........")] = [](sm& st) -> bool
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
				return true;
			};
			disp32[ disp16 >> f("disp@........") >> f("disp@........")] = [](sm& st) -> bool
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
				return true;
			};
			disp64[ disp32 >> f("disp@........") >> f("disp@........") >> f("disp@........") >> f("disp@........")] = [](sm& st) -> bool
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
				return true;
			};

			// sib
			sib [ f("scale@.. index@... base@101") >> f("disp@........") >> f("disp@........") >> f("disp@........") >> f("disp@........")	] = [](sm& st) -> bool
			{
				st.state.disp = constant(st.capture_groups.at("disp"));
				return true;
			};
			sib [ f("scale@.. index@... base@...")] = [](sm& st) -> bool {};

			std::function<bool(boost::optional<amd64_state::OperandSize>,sm&)> rm_func = [&](boost::optional<amd64_state::OperandSize> os,sm& st) -> bool
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
				return true;
			};

			// mod = 00
			rm [ f("mod@00 reg@... rm@101") >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ f("mod@00 reg@... rm@100") >> sib    ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ f("mod@00 reg@... rm@...")           ] = std::bind(rm_func,boost::none,pls::_1);
			rmlong [ f("mod@00 reg@... rm@101") >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ f("mod@00 reg@... rm@100") >> sib    ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ f("mod@00 reg@... rm@...")           ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmbyte [ f("mod@00 reg@... rm@101") >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ f("mod@00 reg@... rm@100") >> sib    ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ f("mod@00 reg@... rm@...")           ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 00 w/ extension opcode
			rm0 [ f("mod@00 000 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ f("mod@00 000 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ f("mod@00 000 rm@...")					  ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ f("mod@00 000 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ f("mod@00 000 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ f("mod@00 000 rm@...")				  	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm1 [ f("mod@00 001 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ f("mod@00 001 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ f("mod@00 001 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte1 [ f("mod@00 001 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ f("mod@00 001 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ f("mod@00 001 rm@...")		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm2 [ f("mod@00 010 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ f("mod@00 010 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ f("mod@00 010 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte2 [ f("mod@00 010 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ f("mod@00 010 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ f("mod@00 010 rm@...")		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm3 [ f("mod@00 011 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ f("mod@00 011 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ f("mod@00 011 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte3 [ f("mod@00 011 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ f("mod@00 011 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ f("mod@00 011 rm@...")		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm4 [ f("mod@00 100 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ f("mod@00 100 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ f("mod@00 100 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte4 [ f("mod@00 100 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ f("mod@00 100 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ f("mod@00 100 rm@...")		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm5 [ f("mod@00 101 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ f("mod@00 101 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ f("mod@00 101 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte5 [ f("mod@00 101 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ f("mod@00 101 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ f("mod@00 101 rm@...")		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm6 [ f("mod@00 110 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ f("mod@00 110 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ f("mod@00 110 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte6 [ f("mod@00 110 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ f("mod@00 110 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ f("mod@00 110 rm@...")				  	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm7 [ f("mod@00 111 rm@101") >> disp32	] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ f("mod@00 111 rm@100") >> sib		] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ f("mod@00 111 rm@...")				  	] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte7 [ f("mod@00 111 rm@101") >> disp32	] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ f("mod@00 111 rm@100") >> sib		] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ f("mod@00 111 rm@...")		  			] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 01
			rm [ f("mod@01 reg@... rm@100") >> sib >> disp32     ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ f("mod@01 reg@... rm@...") >> disp8	            ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte [ f("mod@01 reg@... rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ f("mod@01 reg@... rm@...") >> disp8	        ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmlong [ f("mod@01 reg@... rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ f("mod@01 reg@... rm@...") >> disp8	        ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);

			// mod = 01 w/ opcode extension
			rm0 [ f("mod@01 000 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ f("mod@01 000 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ f("mod@01 000 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ f("mod@01 000 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm1 [ f("mod@01 001 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ f("mod@01 001 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte1 [ f("mod@01 001 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ f("mod@01 001 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm2 [ f("mod@01 010 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ f("mod@01 010 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte2 [ f("mod@01 010 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ f("mod@01 010 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm3 [ f("mod@01 011 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ f("mod@01 011 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte3 [ f("mod@01 011 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ f("mod@01 011 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm4 [ f("mod@01 100 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ f("mod@01 100 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte4 [ f("mod@01 100 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ f("mod@01 100 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm5 [ f("mod@01 101 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ f("mod@01 101 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte5 [ f("mod@01 101 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ f("mod@01 101 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm6 [ f("mod@01 110 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ f("mod@01 110 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte6 [ f("mod@01 110 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ f("mod@01 110 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm7 [ f("mod@01 111 rm@100") >> sib >> disp8 ] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ f("mod@01 111 rm@...") >> disp8	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte7 [ f("mod@01 111 rm@100") >> sib >> disp8 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ f("mod@01 111 rm@...") >> disp8	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 10
			rm [ f("mod@10 reg@... rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm [ f("mod@10 reg@... rm@...") >> disp32	      ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte [ f("mod@10 reg@... rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte [ f("mod@10 reg@... rm@...") >> disp32	      ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmlong [ f("mod@10 reg@... rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);
			rmlong [ f("mod@10 reg@... rm@...") >> disp32	      ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);

			// mod = 10 w/ opcode extension
			rm0 [ f("mod@10 000 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm0 [ f("mod@10 000 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ f("mod@10 000 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte0 [ f("mod@10 000 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm1 [ f("mod@10 001 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ f("mod@10 001 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte1 [ f("mod@10 001 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ f("mod@10 001 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm2 [ f("mod@10 010 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ f("mod@10 010 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte2 [ f("mod@10 010 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ f("mod@10 010 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm3 [ f("mod@10 011 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ f("mod@10 011 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte3 [ f("mod@10 011 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ f("mod@10 011 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm4 [ f("mod@10 100 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ f("mod@10 100 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte4 [ f("mod@10 100 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ f("mod@10 100 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm5 [ f("mod@10 101 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ f("mod@10 101 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte5 [ f("mod@10 101 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ f("mod@10 101 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm6 [ f("mod@10 110 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ f("mod@10 110 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte6 [ f("mod@10 110 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ f("mod@10 110 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			rm7 [ f("mod@10 111 rm@100") >> sib >> disp32 ] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ f("mod@10 111 rm@...") >> disp32	       ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte7 [ f("mod@10 111 rm@100") >> sib >> disp32 ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ f("mod@10 111 rm@...") >> disp32	       ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

			// mod = 11
			rm [ f("mod@11 reg@... rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte [ f("mod@11 reg@... rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmlong [ f("mod@11 reg@... rm@...") ] = std::bind(rm_func,amd64_state::OpSz_64,pls::_1);

			// mod = 11 w/ opcode extension
			rm0 [ f("mod@11 000 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm1 [ f("mod@11 001 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm2 [ f("mod@11 010 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm3 [ f("mod@11 011 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm4 [ f("mod@11 100 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm5 [ f("mod@11 101 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm6 [ f("mod@11 110 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rm7 [ f("mod@11 111 rm@...") ] = std::bind(rm_func,boost::none,pls::_1);
			rmbyte0 [ f("mod@11 000 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte1 [ f("mod@11 001 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte2 [ f("mod@11 010 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte3 [ f("mod@11 011 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte4 [ f("mod@11 100 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte5 [ f("mod@11 101 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte6 [ f("mod@11 110 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);
			rmbyte7 [ f("mod@11 111 rm@...") ] = std::bind(rm_func,amd64_state::OpSz_8,pls::_1);

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
