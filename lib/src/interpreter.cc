#include <boost/variant/static_visitor.hpp>
#include <panopticon/interpreter.hh>

using namespace po;

const meet_t po::meet{};
const join_t po::join{};

std::ostream& po::operator<<(std::ostream& os,const kset_set& s)
{
	os << "( ";
	for(auto c: s)
		os << c << " ";
	return os << ")";
}

std::ostream& po::operator<<(std::ostream& os,const meet_t&) { return os << "⋀S"; }
std::ostream& po::operator<<(std::ostream& os,const join_t&) { return os << "⋁S"; }

concrete_interpreter::concrete_interpreter(void)
: boost::static_visitor<rvalue>(), _environment(boost::none) {}

concrete_interpreter::concrete_interpreter(environment<rvalue>& env)
: boost::static_visitor<rvalue>(), _environment(env) {}

rvalue concrete_interpreter::operator()(const logic_and<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const logic_or<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const logic_neg<rvalue>& a)
{
	rvalue r = normalize(a.right);
	if(is_constant(r))
		return constant(!to_constant(r).content());
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const logic_impl<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const logic_equiv<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_add<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_sub<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_mul<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_div<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_mod<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_less<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_equal<rvalue>& a)
{
	rvalue l = normalize(a.left);
	rvalue r = normalize(a.right);

	if(!is_undefined(l) && !is_undefined(r))
		return constant(l == r);
	else
		return undefined();
}

rvalue concrete_interpreter::operator()(const int_and<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_or<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const int_xor<rvalue>& a)
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
				return constant(to_constant(l).content() ^ to_constant(r).content());
			else if(to_constant(l).content() == 0)
				return r;
		}

		if(is_constant(r) && to_constant(r).content() == 0)
			return l;
	}
	return undefined();
}

rvalue concrete_interpreter::operator()(const int_call<rvalue>& a)
{
	return normalize(a.right);
}

rvalue concrete_interpreter::operator()(const univ_nop<rvalue>& a)
{
	return normalize(a.right);
}

rvalue concrete_interpreter::operator()(const univ_phi<rvalue>& a)
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

rvalue concrete_interpreter::operator()(const logic_lift<rvalue>& a)
{
	return normalize(a.right);
}

rvalue concrete_interpreter::normalize(const rvalue& v) const
{
	if(is_constant(v))
		return to_constant(v);
	else if(is_variable(v) && _environment)
	{
		auto i = _environment->find(to_variable(v));
		if(i != _environment->end())
			return normalize(i->second);
		else
			return v;
	}
	else
		return v;
}
