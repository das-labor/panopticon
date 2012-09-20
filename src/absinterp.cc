#include <iostream>

#include "absinterp.hh"
#include "dflow.hh"

// taint domain
taint_lattice bottom(taint_domain)
{
	return taint_lattice(new persistent_map<name,set<name>>);
}

bool equal(taint_domain,const taint_lattice a, const taint_lattice b)
{
	return *a == *b;
}

taint_lattice supremum(taint_domain,const taint_lattice a, const taint_lattice b)
{
	// standard set union
	taint_lattice ret(new persistent_map<name,set<name>>(*a));

	for_each(b->begin(),b->end(),[&](const pair<name,set<name>> &x)
	{
		if(ret->has(x.first))
		{
			set<name> c;

			copy(ret->get(x.first).begin(),ret->get(x.first).end(),inserter(c,c.begin()));
			copy(x.second.begin(),x.second.end(),inserter(c,c.begin()));
	//		ret->mutate(x.first,c);
		}
		else
			;//ret->mutate(x.first,x.second);
	});
	
	return ret;
}

taint_lattice abstraction(taint_domain,const taint_lattice a, instr_cptr i)
{
	taint_lattice ret(new persistent_map<name,set<name>>(*a));
	set<name> r(ret->has(i->assigns->nam) ? ret->get(i->assigns->nam) : set<name>());

	for_each(i->operands.begin(),i->operands.end(),[&](value_ptr v)
	{
		shared_ptr<variable> w;

		if((w = dynamic_pointer_cast<variable>(v)))
		{
			r.insert(w->nam);
			if(ret->has(w->nam))
				copy(ret->get(w->nam).begin(),ret->get(w->nam).end(),inserter(r,r.begin()));
		}
	});

	//ret->mutate(i->assigns->nam,r);
	return ret;
}

ostream& operator<<(ostream &os, const taint_lattice l)
{
	if(l)
	{
		for_each(l->begin(),l->end(),[&](const pair<name,set<name>> &p)
		{
			set<name>::const_iterator i = p.second.cbegin();

			os << p.first.inspect() << ": ";
			while(i != p.second.cend())
			{
				os << (i++)->inspect();
				if(i != p.second.cend())
					os << ", ";
			}
			os << " ";
		});
	}
	else
		os << "NULL lattice";
	return os;
}

// cprop domain
cprop_lattice bottom(cprop_domain)
{
	return cprop_lattice(new persistent_map<name,cprop_element>);
}

bool equal(cprop_domain,const cprop_lattice a, const cprop_lattice b)
{
	return all_of(a->begin(),a->end(),[&](const pair<name,cprop_element> &p) { return b->has(p.first) && b->get(p.first) == a->get(p.first); }) &&
				 all_of(b->begin(),b->end(),[&](const pair<name,cprop_element> &p) { return a->has(p.first) && a->get(p.first) == b->get(p.first); });
}

cprop_element supremum(const cprop_element a, const cprop_element b)
{
	if(a.type == cprop_element::Bottom || b.type == cprop_element::NonConst)
		return b;
	if(b.type == cprop_element::Bottom || a.type == cprop_element::NonConst)
		return a;
	if(a.value == b.value)
		return a;
	else
		return cprop_element(cprop_element::NonConst);
}

cprop_lattice supremum(cprop_domain,const cprop_lattice a, const cprop_lattice b)
{
	// Bot < Const < NonConst
	cprop_lattice ret(new persistent_map<name,cprop_element>(*a));

	for_each(b->begin(),b->end(),[&](const pair<name,cprop_element> &x)
	{
		if(ret->has(x.first))
		{
			cprop_element a = x.second;
			cprop_element b = ret->get(x.first);
			cprop_element c = supremum(a,b);

			assert(a.type <= c.type && b.type <= c.type);

			ret->mutate(x.first,c);
		}
		else
			ret->mutate(x.first,x.second);
	});
	
	return ret;
}

bool operator==(const cprop_element &a, const cprop_element &b) 
{ 
	/*if(a.type != b.type)
		cout << "type " << a.type << " != type " << b.type << endl;
	else if(b.type == cprop_element::Const && a.value != b.value)
		cout << a.value << " != " << b.value << endl;*/
	return a.type == b.type && (b.type != cprop_element::Const || a.value == b.value); 
}

bool operator!=(const cprop_element &a, const cprop_element &b) { return !(a == b); }

cprop_lattice abstraction(cprop_domain,const cprop_lattice a, instr_cptr i)
{
	// 99%
	cprop_lattice ret(new persistent_map<name,cprop_element>(*a));

	// ssa variable is aready at Top
	if(ret->has(i->assigns->nam) && ret->get(i->assigns->nam).type == cprop_element::NonConst)
		return ret;

	vector<unsigned int> ops;
	cprop_element::Type type = cprop_element::Const;
	auto j = i->operands.cbegin(), jend = i->operands.cend();

	// read operand values
	while(j != jend)
	{
		const value_ptr v = *j++;
		shared_ptr<constant> w;
		shared_ptr<variable> x;
		const cprop_element *ce;

		if((w = dynamic_pointer_cast<constant>(v)))
			ops.insert(ops.end(),w->val);
		else if((x = dynamic_pointer_cast<variable>(v)) && ret->has(x->nam) && (ce = &ret->get(x->nam)))
		{
			ops.insert(ops.end(),ce->value);
			if((type = ce->type) != cprop_element::Const)
			{
				//cout << "operator " << distance(i->operands.cbegin(),j) - 1 << " is " << type << endl;
				break;
			}
		}
		else
		{
			// undefined
			//cout << "operator " << distance(i->operands.cbegin(),j) - 1 << " is undef" << endl;
			type = cprop_element::Bottom;
			break;
		}
	}

	if(type != cprop_element::Const)
	{
		//cout << i->assigns->nam.inspect() << " is not const" << endl;
		ret->mutate(i->assigns->nam,cprop_element(type));
		return ret;
	}
	else
	{
		// is const, calc concrete value
		unsigned int val;

		switch(i->opcode)
		{
		case instr::Assign:
			val = ops[0]; break;
		case instr::Not:
			val = ~ops[0]; break;
		case instr::Phi:
			if(ops[0] != ops[1])
			{
				ret->mutate(i->assigns->nam,cprop_element(cprop_element::NonConst));
				return ret;
			}
			else
			val = ops[0]; break;
		case instr::Or:
			val = ops[0] | ops[1]; break;
		case instr::And:
			val = ops[0] & ops[1]; break;
		case instr::Sub:
			val = ops[0] - ops[1]; break;
		case instr::Slice:
		{
			assert(ops[1] <= ops[2]);
			unsigned int i = 0;
						
			val = 0;
			while(i < 32)
			{
				val |= ops[0] & ((i >= ops[1] && i <= ops[2]) << i);
				++i;
			}
			val = val >> ops[1]; break;
		}	
		case instr::Concat:
			val = (ops[0] << 8) | ops[1]; break;
		default:
				;
		}
		
		//cout << i->assigns->nam.inspect() << " is const" << endl;
		ret->mutate(i->assigns->nam,cprop_element(val));
		return ret;
	}
}

ostream& operator<<(ostream &os, const cprop_element &e)
{
	switch(e.type)
	{
	case cprop_element::Bottom:
		os << "Bot"; break;
	case cprop_element::NonConst:
		os << "NonConst"; break;
	case cprop_element::Const:
		os << e.value; break;
	default:
		os << "Err"; break;
	}
	return os;
}

ostream& operator<<(ostream &os, const cprop_lattice l)
{
	if(l)
	{
		auto i = l->begin();

		while(i != l->end())
		{
			if(i->second.type != cprop_element::Bottom)
			{
				os << i->first.inspect() << ": ";
			
				switch(i->second.type)
				{
				case cprop_element::Bottom:
					os << "Bot"; break;
				case cprop_element::NonConst:
					os << "NonConst"; break;
				case cprop_element::Const:
					os << i->second.value; break;
				default:
					os << "Err (" << i->second.type << ")"; break;
				}
		
				++i;
				if(i != l->end())
					os << ", ";
			}
			else	
				++i;
		}
	}
	else
		os << "Empty";
	return os;
}
