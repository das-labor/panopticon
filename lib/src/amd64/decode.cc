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

#include <functional>
#include <list>
#include <string>

#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/decode.hh>

#include <panopticon/code_generator.hh>

using namespace po;
using namespace po::amd64;

memory po::amd64::byte(rvalue o) { return memory(o,1,LittleEndian,"ram"); }
memory po::amd64::word(rvalue o) { return memory(o,2,LittleEndian,"ram"); }
memory po::amd64::dword(rvalue o) { return memory(o,4,LittleEndian,"ram"); }
memory po::amd64::qword(rvalue o) { return memory(o,8,LittleEndian,"ram"); }

memory po::amd64::byte(uint64_t o) { return byte(constant(o)); }
memory po::amd64::word(uint64_t o) { return word(constant(o)); }
memory po::amd64::dword(uint64_t o) { return dword(constant(o)); }
memory po::amd64::qword(uint64_t o) { return qword(constant(o)); }

variable po::amd64::decode_reg8(unsigned int r_reg,bool rex)
{
	switch(r_reg)
	{
		case 0: return al;
		case 1: return cl;
		case 2: return dl;
		case 3: return bl;
		case 4: return rex ? spl : ah;
		case 5: return rex ? bpl : ch;
		case 6: return rex ? sil : dh;
		case 7: return rex ? dil : bh;
		case 8: return r8l;
		case 9: return r9l;
		case 10: return r10l;
		case 11: return r11l;
		case 12: return r12l;
		case 13: return r13l;
		case 14: return r14l;
		case 15: return r15l;
		default: ensure(false);
	}
}

variable po::amd64::decode_reg16(unsigned int r_reg)
{
	switch(r_reg)
	{
		case 0: return ax;
		case 1: return cx;
		case 2: return dx;
		case 3: return bx;
		case 4: return sp;
		case 5: return bp;
		case 6: return si;
		case 7: return di;
		case 8: return r8w;
		case 9: return r9w;
		case 10: return r10w;
		case 11: return r11w;
		case 12: return r12w;
		case 13: return r13w;
		case 14: return r14w;
		case 15: return r15w;
		default: ensure(false);
	}
}

variable po::amd64::decode_reg32(unsigned int r_reg)
{
	switch(r_reg)
	{
		case 0: return eax;
		case 1: return ecx;
		case 2: return edx;
		case 3: return ebx;
		case 4: return esp;
		case 5: return ebp;
		case 6: return esi;
		case 7: return edi;
		case 8: return r8d;
		case 9: return r9d;
		case 10: return r10d;
		case 11: return r11d;
		case 12: return r12d;
		case 13: return r13d;
		case 14: return r14d;
		case 15: return r15d;
		default: ensure(false);
	}
}

variable po::amd64::decode_reg64(unsigned int r_reg)
{
	switch(r_reg)
	{
		case 0: return rax;
		case 1: return rcx;
		case 2: return rdx;
		case 3: return rbx;
		case 4: return rsp;
		case 5: return rbp;
		case 6: return rsi;
		case 7: return rdi;
		case 8: return r8;
		case 9: return r9;
		case 10: return r10;
		case 11: return r11;
		case 12: return r12;
		case 13: return r13;
		case 14: return r14;
		case 15: return r15;
		default: ensure(false);
	}
}

po::variable po::amd64::select_reg(amd64_state::OperandSize os,unsigned int r)
{
	switch(os)
	{
		case amd64_state::OpSz_8: return decode_reg8(r,false);
		case amd64_state::OpSz_16: return decode_reg16(r);
		case amd64_state::OpSz_32: return decode_reg32(r);
		case amd64_state::OpSz_64: return decode_reg64(r);
		default: ensure(false);
	}
}

po::memory po::amd64::select_mem(amd64_state::OperandSize os,rvalue o)
{
	switch(os)
	{
		case amd64_state::OpSz_8: return byte(o);
		case amd64_state::OpSz_16: return word(o);
		case amd64_state::OpSz_32: return dword(o);
		case amd64_state::OpSz_64: return qword(o);
		default: ensure(false);
	}
}

po::lvalue po::amd64::decode_modrm(
		unsigned int mod,
		unsigned int b_rm,	// B.R/M
		boost::optional<constant> disp,
		boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib, // scale, X.index, B.base
		amd64_state::OperandSize os,
		amd64_state::AddressSize as,
		cg& c)
{
	ensure(mod < 0x4);
	ensure(b_rm < 0x8);

	switch(as)
	{
		case amd64_state::AddrSz_16:
		{
			switch(mod)
			{
				case 0: case 1: case 2:
				{
					if(b_rm == 6)
					{
						if(mod == 0)
							return select_mem(os,*disp);
						else
							return c.add_i(select_mem(os,bp),*disp);
					}
					else
					{
						lvalue base = undefined();

						switch(b_rm)
						{
							case 0: base = select_mem(os,c.add_i(bx,si)); break;
							case 1: base = select_mem(os,c.add_i(bx,di)); break;
							case 2: base = select_mem(os,c.add_i(bp,si)); break;
							case 3: base = select_mem(os,c.add_i(bp,di)); break;
							case 4: base = select_mem(os,si); break;
							case 5: base = select_mem(os,di); break;
							case 7: base = select_mem(os,bx); break;
							default: ensure(false);
						}

						if(mod == 0)
							return base;
						else
							return c.add_i(base,*disp);
					}
				}

				case 3:
				{
					return select_reg(os,b_rm);
				}

				default: ensure(false);
			}
		}

		case amd64_state::AddrSz_32:
		case amd64_state::AddrSz_64:
		{
			switch(mod)
			{
				case 0: switch(b_rm)
				{
					case 0: case 1: case 2: case 3:
					case 6: case 7: case 8: case 9: case 10: case 11:
					case 14: case 15:
						return select_mem(os,select_reg(os,b_rm));

					case 4:
					case 12:
						return decode_sib(mod,std::get<0>(*sib),std::get<1>(*sib),std::get<2>(*sib),disp,os,c);

					case 5:
					case 13:
						return select_mem(os,*disp);

					default: ensure(false);
				}
				case 1: switch(b_rm)
				{
					default: ensure(false);
				}
				case 2: switch(b_rm)
				{
					default: ensure(false);
				}
				case 3:
				{
					return select_reg(os,b_rm);
				}
				default: ensure(false);
			}
		}
		default: ensure(false);
	}
}

po::memory po::amd64::decode_sib(
		unsigned int mod,
		unsigned int scale,
		unsigned int x_index,
		unsigned int b_base,
		boost::optional<constant> disp,
		amd64_state::OperandSize os,
		cg& c)
{
	ensure(mod <= 3 && scale <= 3 && x_index <= 15 && b_base <= 15);

	switch(mod)
	{
		case 0:
		{
			switch(b_base)
			{
				case 0: case 1: case 2: case 3: case 4:
				case 6: case 7: case 8: case 9: case 10: case 11: case 12:
				case 14: case 15:
				{
					switch(x_index)
					{
						case 0: case 1: case 2: case 3:
						case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15:
							return select_mem(os,c.add_i(decode_reg64(b_base & 7),constant((x_index & 7) * ((1 << (scale & 3)) / 2))));
						case 4:
							return select_mem(os,constant(b_base & 7));
						default: ensure(false);
					}
				}
				case 5:
				case 13:
				{
					switch(x_index)
					{
						case 0: case 1: case 2: case 3:
						case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15:
							return select_mem(os,constant((x_index & 7) * ((1 << (scale & 3)) / 2) + disp->content()));
						case 4:
							return select_mem(os,*disp);
						default: ensure(false);
					}
				}
				default: ensure(false);
			}
		}
		case 1:
		case 2:
		{
			switch(x_index)
			{
				case 0: case 1: case 2: case 3:
				case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15:
					return select_mem(os,c.add_i(decode_reg64(b_base & 7),constant((x_index & 7) * ((1 << (scale & 3)) / 2) + disp->content())));
				case 4:
					return select_mem(os,c.add_i(decode_reg64(b_base & 7),*disp));
				default: ensure(false);
			}
		}
		default: ensure(false);
	}
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_rm(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	return std::make_pair(*st.state.reg,*st.state.rm);
}

std::pair<rvalue,rvalue> po::amd64::decode_mr(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	return std::make_pair(*st.state.rm,*st.state.reg);
}

std::pair<rvalue,rvalue> po::amd64::decode_mi(sm const& st,cg&)
{
	ensure(st.state.rm && st.state.imm);
	return std::make_pair(*st.state.rm,*st.state.imm);
}

std::pair<rvalue,rvalue> po::amd64::decode_i(amd64_state::OperandSize os, sm const& st,cg&)
{
	ensure(st.state.imm);
	switch(os)
	{
		case amd64_state::OpSz_8: return std::make_pair(ah,*st.state.imm);
		case amd64_state::OpSz_16: return std::make_pair(ax,*st.state.imm);
		case amd64_state::OpSz_32: return std::make_pair(eax,*st.state.imm);
		case amd64_state::OpSz_64: return std::make_pair(rax,*st.state.imm);
		default: ensure(false);
	}
}

rvalue po::amd64::decode_m(sm const& st,cg&)
{
	ensure(false);
}

rvalue po::amd64::decode_d(sm const& st,cg&)
{
	ensure(false);
}

rvalue po::amd64::decode_o(sm const& st,cg&)
{
	ensure(false);
}

rvalue po::amd64::decode_imm(sm const& st,cg&)
{
	ensure(st.state.imm);
	return *st.state.imm;
}

sem_action po::amd64::nonary(std::string const& op, std::function<void(cg&)> func)
{
	return [op,func](sm &st)
	{
		st.mnemonic(st.tokens.size(),op,"",[func,st,op](cg& c)
		{
			func(c);

			std::cout << op << std::endl;
			return std::list<rvalue>({});
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::amd64::unary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode, std::function<void(cg&,rvalue)> func)
{
	return [op,func,decode](sm &st)
	{
		st.mnemonic(st.tokens.size(),op,"{64}",[decode,func,st,op](cg& c)
		{
			rvalue a = decode(st,c);
			func(c,a);

			std::cout << op << " " << a << std::endl;
			return std::list<rvalue>({a});
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::amd64::binary(std::string const& op, std::function<std::pair<rvalue,rvalue>(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue)> func)
{
	return [op,func,decode](sm &st)
	{
		st.mnemonic(st.tokens.size(),op,"{64} {64}",[decode,func,st,op](cg& c)
		{
			rvalue a,b;

			std::tie(a,b) = decode(st,c);
			func(c,a,b);

			std::cout << op << " " << a << ", " << b << std::endl;
			return std::list<rvalue>({a,b});
		});

		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::amd64::branch(std::string const& m, rvalue flag, bool set)
{
	return [m,flag,set](sm &st)
	{
		/*int64_t _k = st.capture_groups["k"] * 2;
		guard g(flag,relation::Eq,set ? constant(1) : constant(0));
		constant k((int8_t)(_k <= 63 ? _k : _k - 128));*/

		st.mnemonic(st.tokens.size() * 2,m,"");
		st.jump(st.address + st.tokens.size());//,g.negation());
		//st.jump(undefined(),g);//st.address + k.content() + 2,g);
	};
}
