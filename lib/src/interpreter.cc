#include <boost/variant/static_visitor.hpp>
#include <panopticon/interpreter.hh>

using namespace po;

concrete_interpreter::concrete_interpreter(environment<rvalue>& env)
: boost::static_visitor<rvalue>(), _environment(env) {}

rvalue concrete_interpreter::operator()(const logic_and& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(is_constant(l))
	{
		if(is_constant(r))
			return constant(to_constant(l).content() && to_constant(r).content());
		else
		{
			if(to_constant(l).content())
				return r;
			else
				return constant(false);
		}
	}
	else
	{
		if(is_constant(r))
		{
			if(to_constant(r).content())
				return l;
			else
				return constant(false);
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_or& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(is_constant(l))
	{
		if(is_constant(r))
			return constant(to_constant(l).content() || to_constant(r).content());
		else
		{
			if(!to_constant(l).content())
				return r;
			else
				return constant(true);
		}
	}
	else
	{
		if(is_constant(r))
		{
			if(!to_constant(r).content())
				return l;
			else
				return constant(true);
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_neg& a)
{
	rvalue r = normalize(a.right);
	if(is_constant(r))
		return constant(!to_constant(r).content());
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const logic_impl& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(is_constant(l))
	{
		if(is_constant(r))
			return constant((to_constant(l).content() && to_constant(r).content()) || to_constant(l).content());
		else
		{
			if(to_constant(l).content())
				return r;
			else
				return rvalue(constant(true));
		}
	}
	else
	{
		if(is_constant(r))
		{
			if(to_constant(r).content())
				return l;
			else
				return constant(true);
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_equiv& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(is_constant(l))
	{
		if(is_constant(r))
			return rvalue(constant(to_constant(l).content() == to_constant(r).content()));
		else if(to_constant(l).content())
			return rvalue(a.right);
	}
	else
	{
		rvalue r = normalize(a.right);
		if(is_constant(r))
		{
			if(to_constant(r).content())
				return rvalue(a.left);
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_add& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(is_constant(l))
	{
		if(is_constant(r))
			return rvalue(constant(to_constant(l).content() + to_constant(r).content()));
		else if(to_constant(l).content() == 0)
			return rvalue(a.right);
	}
	else
	{
		if(is_constant(r))
		{
			if(to_constant(r).content() == 0)
				return rvalue(a.left);
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_sub& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(l == r && !is_undefined(l) && !is_undefined(r))
		return constant(0);
	else
	{
		if(is_constant(l))
		{
			if(is_constant(r))
				return constant(to_constant(l).content() - to_constant(r).content());
		}
		else
		{
			if(is_constant(r))
			{
				if(to_constant(r).content() == 0)
					return l;
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_mul& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(is_constant(l))
	{
		if(is_constant(r))
			return constant(to_constant(l).content() * to_constant(r).content());
		else
		{
			if(to_constant(l).content() == 0)
				return constant(0);
			else if(to_constant(l).content() == 1)
				return r;
		}
	}
	else
	{
		if(is_constant(r))
		{
			if(to_constant(r).content() == 0)
				return constant(0);
			else if(to_constant(r).content() == 1)
				return l;
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_div& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(l == r && !is_undefined(l) && !is_undefined(r))
		return constant(1);
	else
	{
		if(is_constant(l))
		{
			if(is_constant(r))
				return constant(to_constant(l).content() / to_constant(r).content());
			else if(to_constant(l).content() == 0)
				return constant(0);
		}
		else
		{
			if(is_constant(r))
			{
				if(to_constant(r).content() == 0)
					return undefined();
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_mod& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(l == r && !is_undefined(l) && !is_undefined(r))
		return constant(0);
	else
	{
		if(is_constant(l))
		{
			if(is_constant(r))
				return constant(to_constant(l).content() % to_constant(r).content());
			else if(to_constant(l).content() == 0)
				return constant(0);
		}
		else
		{
			if(is_constant(r))
			{
				if(to_constant(r).content() == 0)
					return undefined();
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_less& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(l == r && !is_undefined(l) && !is_undefined(r))
		return constant(false);
	else
	{
		if(is_constant(l))
		{
			if(is_constant(r))
				return rvalue(constant(to_constant(l).content() < to_constant(r).content()));
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_equal& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(!is_undefined(l) && !is_undefined(r))
		return constant(l == r);
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const int_and& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(l == r && !is_undefined(l) && !is_undefined(r))
		return l;
	else
	{
		if(is_constant(l))
		{
			if(is_constant(r))
				constant(to_constant(l).content() & to_constant(r).content());
			else if(to_constant(l).content() == 0)
				constant(0);
		}

		if(is_constant(r) && to_constant(r).content() == 0)
			return constant(0);
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_or& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(l == r && !is_undefined(l) && !is_undefined(r))
		return l;
	else
	{
		if(is_constant(l))
		{
			if(is_constant(r))
				return constant(to_constant(l).content() | to_constant(r).content());
			else if(to_constant(l).content() == 0)
				return r;
		}

		if(is_constant(r) && to_constant(r).content() == 0)
			return l;
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_neg& a)
{
	rvalue r = normalize(a.right);
	if(is_constant(r))
		return constant(~to_constant(r).content());
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const int_call& a)
{
	return normalize(a.right);
}

rvalue concrete_interpreter::operator()(const univ_nop& a)
{
	return normalize(a.right);
}

rvalue concrete_interpreter::operator()(const univ_phi& a)
{
	if(a.operands.empty())
		return undefined();
	else if(a.operands.size() == 1)
		return normalize(a.operands.at(0));
	else
	{
		rvalue x = normalize(a.operands.at(0));
		if(!is_undefined(x) && std::all_of(a.operands.begin()+1,a.operands.end(),[&](const rvalue& r)
					{ rvalue y = normalize(r); return !is_undefined(y) && y == x; }))
			return x;
		else
			return undefined();
	}
}

rvalue concrete_interpreter::operator()(const int_lift& a)
{
	return normalize(a.right);
}

rvalue concrete_interpreter::normalize(const rvalue& v) const
{
	if(is_constant(v))
		return to_constant(v);
	else if(is_variable(v))
	{
		auto i = _environment.find(to_variable(v));
		if(i != _environment.end())
			return normalize(i->second);
		else
			return v;
	}
	else
		return v;
}

template<>
rvalue po::supremum(rvalue a, rvalue b, concrete_domain)
{
	if(a == b)
		return a;
	else
		return undefined();
}

/*template<typename It>
concrete_environment forward(It begin, It end, const concrete_environment& cenv)
{
	concrete_environment ret = cenv;
	concrete_interp_visitor vis(ret);

	for(auto i: po::iters(std::make_pair(begin,end)))
		boost::apply_visitor(vis,i);

	return ret;
}*/

/*
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

	if(is_constant(r)et.type == sscp_lattice::Const)
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
}*/
