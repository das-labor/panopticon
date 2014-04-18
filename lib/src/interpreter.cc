#include <interpreter.hh>

using namespace po;

sscp_lattice po::execute(const lvalue &left, instr::Function fn, const std::vector<rvalue> &concrete, const std::vector<sscp_lattice> &abstract, simple_sparse_constprop)
{
	assert(concrete.size() == abstract.size());
	sscp_lattice ret;
	std::vector<uint64_t> args(concrete.size(),0);
	size_t i = 0;
	concrete_interp<uint64_t> ci_tag;

	if(fn == instr::Call || left.is_memory())
	{
		ret.type = sscp_lattice::NonConst;
		return ret;
	}

	// all arguments must be constants of variables with Const abstract type
	while(i < concrete.size() && ret.type != sscp_lattice::NonConst)
	{
		const rvalue &c = concrete[i];
		const sscp_lattice &a = abstract[i];

		if(c.is_constant())
		{
			args[i] = c.to_constant().content();
			ret.type = sscp_lattice::Const;
		}
		else if(a.type == sscp_lattice::Const)
		{
			args[i] = a.value;
			ret.type = sscp_lattice::Const;
		}
		else
		{
			ret.type = a.type;
			break;
		}

		++i;
	}

	if(ret.type == sscp_lattice::Const)
		ret.value = execute(left,fn,concrete,args,ci_tag);

	return ret;
}

sscp_lattice po::supremum(const sscp_lattice &a, const sscp_lattice &b, simple_sparse_constprop)
{
	if(a.type == sscp_lattice::Bottom || b.type == sscp_lattice::NonConst)
		return b;
	if(b.type == sscp_lattice::Bottom || a.type == sscp_lattice::NonConst)
		return a;
	if(a.value == b.value)
		return a;
	else
		return sscp_lattice(sscp_lattice::NonConst);
}

std::ostream &operator<<(std::ostream &os, const sscp_lattice &l)
{
	switch(l.type)
	{
	case sscp_lattice::Bottom: os << "Bot"; return os;
	case sscp_lattice::NonConst: os << "NonConst"; return os;
	case sscp_lattice::Const: os << l.value; return os;
	default: assert(false);
	}
}
