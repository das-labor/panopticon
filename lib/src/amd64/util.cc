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

sem_action po::amd64::unary(std::string const& op, std::function<void(cg&,rvalue)> func)
{
	return [op,func](sm &st)
	{
		rvalue a = *st.state.operand_a;

		st.mnemonic(st.tokens.size(),op,"{64}",a,std::bind(func,std::placeholders::_1,a));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::amd64::binary(std::string const& op, std::function<void(cg&,rvalue,rvalue)> func)
{
	return [op,func](sm &st)
	{
		rvalue a = *st.state.operand_a;
		rvalue b = *st.state.operand_b;

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
