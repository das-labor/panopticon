#include <boost/variant/static_visitor.hpp>
#include <panopticon/interpreter.hh>

using namespace po;

concrete_interpreter::concrete_interpreter(const environment<rvalue>& env)
: boost::static_visitor<rvalue>(), _environment(env) {}

rvalue concrete_interpreter::operator()(const logic_and& a)
{
	boost::optional<constant> l = lookup(a.left);
	if(l)
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			return rvalue(constant(l->content() && r->content()));
		}
		else
		{
			if(l->content())
			{
				// right := true && right
				return rvalue(a.right);
			}
			else
			{
				// false := false && right
				return rvalue(constant(false));
			}
		}
	}
	else
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			if(r->content())
			{
				// left := left && true
				return rvalue(a.left);
			}
			else
			{
				// false := left && false
				return rvalue(constant(false));
			}
		}
	}

	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_or& a)
{
	boost::optional<constant> l = lookup(a.left);
	if(l)
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			return rvalue(constant(l->content() || r->content()));
		}
		else
		{
			if(!l->content())
			{
				return rvalue(a.right);
			}
			else
			{
				return rvalue(constant(true));
			}
		}
	}
	else
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			if(!r->content())
			{
				return rvalue(a.left);
			}
			else
			{
				return rvalue(constant(true));
			}
		}
	}

	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_neg& a)
{
	boost::optional<constant> r = lookup(a.right);
	if(r)
	{
		return rvalue(constant(!r->content()));
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_impl& a)
{
	boost::optional<constant> l = lookup(a.left);
	if(l)
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			return rvalue(constant((l->content() && r->content()) || l->content()));
		}
		else
		{
			if(l->content())
			{
				return rvalue(a.right);
			}
			else
			{
				return rvalue(constant(true));
			}
		}
	}
	else
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			if(r->content())
			{
				return rvalue(a.left);
			}
			else
			{
				return rvalue(constant(true));
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const logic_equiv& a)
{
	boost::optional<constant> l = lookup(a.left);
	if(l)
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			return rvalue(constant(l->content() == r->content()));
		}
		else
		{
			if(l->content())
			{
				return rvalue(a.right);
			}
		}
	}
	else
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			if(r->content())
			{
				return rvalue(a.left);
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_add& a)
{
	boost::optional<constant> l = lookup(a.left);
	if(l)
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			return rvalue(constant(l->content() + r->content()));
		}
		else
		{
			if(l->content() == 0)
			{
				return rvalue(a.right);
			}
		}
	}
	else
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			if(r->content() == 0)
			{
				return rvalue(a.left);
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_sub& a)
{
	if(a.left == a.right)
	{
		return rvalue(constant(0));
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				return rvalue(constant(l->content() - r->content()));
			}
		}
		else
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				if(r->content() == 0)
				{
					return rvalue(a.left);
				}
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_mul& a)
{
	boost::optional<constant> l = lookup(a.left);
	if(l)
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			return rvalue(constant(l->content() * r->content()));
		}
		else
		{
			if(l->content() == 0)
			{
				return rvalue(constant(0));
			}
			else if(l->content() == 1)
			{
				return rvalue(a.right);
			}
		}
	}
	else
	{
		boost::optional<constant> r = lookup(a.right);
		if(r)
		{
			if(r->content() == 0)
			{
				return rvalue(constant(0));
			}
			else if(r->content() == 1)
			{
				return rvalue(a.right);
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_div& a)
{
	if(a.left == a.right)
	{
		return rvalue(constant(1));
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				return rvalue(constant(l->content() / r->content()));
			}
			else if(l->content() == 0)
			{
				return rvalue(constant(0));
			}
		}
		else
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				if(r->content() == 0)
				{
					return undefined();
				}
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_mod& a)
{
	if(a.left == a.right)
	{
		return rvalue(constant(0));
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				return rvalue(constant(l->content() % r->content()));
			}
			else if(l->content() == 0)
			{
				return rvalue(constant(0));
			}
		}
		else
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				if(r->content() == 0)
				{
					return undefined();
				}
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_less& a)
{
	if(a.left == a.right)
	{
		return rvalue(constant(false));
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				return rvalue(constant(l->content() < r->content()));
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_equal& a)
{
	if(a.left == a.right)
	{
		return rvalue(constant(true));
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				return rvalue(constant(l->content() == r->content()));
			}
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_and& a)
{
	if(a.left == a.right)
	{
		rvalue(a.left);
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				rvalue(constant(l->content() & r->content()));
			}
			else if(l->content() == 0)
			{
				rvalue(constant(0));
			}
		}

		boost::optional<constant> r = lookup(a.right);
		if(r && r->content() == 0)
		{
			return rvalue(constant(0));
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_or& a)
{
	if(a.left == a.right)
	{
		return rvalue(a.left);
	}
	else
	{
		boost::optional<constant> l = lookup(a.left);
		if(l)
		{
			boost::optional<constant> r = lookup(a.right);
			if(r)
			{
				return rvalue(constant(l->content() | r->content()));
			}
			else if(l->content() == 0)
			{
				return rvalue(a.right);
			}
		}

		boost::optional<constant> r = lookup(a.right);
		if(r && r->content() == 0)
		{
			return rvalue(a.left);
		}
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_neg& a)
{
	boost::optional<constant> r = lookup(a.right);
	if(r)
		return rvalue(constant(~r->content()));
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const int_call& a)
{
	return a.right;
}

rvalue concrete_interpreter::operator()(const univ_nop& a)
{
	return a.right;
}

rvalue concrete_interpreter::operator()(const univ_phi& a)
{
	if(a.operands.empty())
		return undefined();
	else if(std::all_of(a.operands.begin(),a.operands.end(),[&](const rvalue& r) { return r == a.operands[0]; }))
		return a.operands[0];
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const int_lift& a)
{
	return a.right;
}

boost::optional<constant> concrete_interpreter::lookup(const rvalue& v) const
{ return is_constant(v) ? boost::make_optional(to_constant(v)) : boost::none; }

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
}*/
