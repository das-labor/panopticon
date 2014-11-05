#include <functional>
#include <list>
#include <string>

#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/util.hh>

#include <panopticon/code_generator.hh>

using namespace po;
using namespace po::amd64;

memory po::amd64::byte(rvalue o) { return memory(o,1,LittleEndian,"ram"); }
memory po::amd64::word(rvalue o) { return memory(o,2,LittleEndian,"ram"); }
memory po::amd64::dword(rvalue o) { return memory(o,4,LittleEndian,"ram"); }
memory po::amd64::qword(rvalue o) { return memory(o,8,LittleEndian,"ram"); }

memory po::amd64::byte(unsigned int o) { return byte(constant(o)); }
memory po::amd64::word(unsigned int o) { return word(constant(o)); }
memory po::amd64::dword(unsigned int o) { return dword(constant(o)); }
memory po::amd64::qword(unsigned int o) { return qword(constant(o)); }

variable po::amd64::decode_reg8(unsigned int reg,bool rex)
{
	ensure(reg < 0x10);

	switch(reg)
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

variable po::amd64::decode_reg16(unsigned int reg)
{
	switch(reg)
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

variable po::amd64::decode_reg32(unsigned int reg)
{
	switch(reg)
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

variable po::amd64::decode_reg64(unsigned int reg)
{
	switch(reg)
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

po::memory po::amd64::select_mem(amd64_state::AddressSize as,rvalue o)
{
	switch(as)
	{
		case amd64_state::AddrSz_16: return word(o);
		case amd64_state::AddrSz_32: return dword(o);
		case amd64_state::AddrSz_64: return qword(o);
		default: ensure(false);
	}
}

po::lvalue po::amd64::decode_modrm(unsigned int mod,unsigned int rm,boost::optional<unsigned int> disp,boost::optional<unsigned int> sib,amd64_state::OperandSize os,amd64_state::AddressSize as)
{
	ensure(mod < 0x4);
	ensure(rm < 0x8);

	switch(mod)
	{
		case 0: switch(rm)
		{
			case 0: case 1: case 2: case 3: return select_mem(as,select_reg(os,rm));
			case 4: return select_mem(as,constant(*disp));
			case 5: return select_mem(as,constant(*sib));
			case 6: case 7: return select_mem(as,select_reg(os,rm));
			default: ensure(false);
		}
		case 1: switch(rm)
		{
			default: ensure(false);
		}
		case 2: switch(rm)
		{
			default: ensure(false);
		}
		case 3: switch(rm)
		{
			default: ensure(false);
		}
		default: ensure(false);
	}
}

sem_action po::amd64::unary(std::string const& op, std::function<rvalue(sm const&)> decode, std::function<void(cg&,rvalue)> func)
{
	return [op,func,decode](sm &st)
	{
		rvalue a = decode(st);

		st.mnemonic(st.tokens.size(),op,"{64}",a,std::bind(func,std::placeholders::_1,a));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::amd64::binary(std::string const& op, std::function<std::pair<rvalue,rvalue>(sm const&)> decode, std::function<void(cg&,rvalue,rvalue)> func)
{
	return [op,func,decode](sm &st)
	{
		rvalue a, b;

		std::tie(a,b) = decode(st);
		st.mnemonic(st.tokens.size(),op,"{64} {64}",a,b,bind(func,std::placeholders::_1,a,b));
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
