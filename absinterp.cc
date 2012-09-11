#include <iostream>

#include "absinterp.hh"
#include "dflow.hh"

// taint domain
taint_lattice bottom(taint_domain)
{
	return taint_lattice(new map<name,set<name>>);
}

bool equal(taint_domain,const taint_lattice a, const taint_lattice b)
{
	cout << "equal? " << (*a == *b) << endl;
	return *a == *b;
}

taint_lattice supremum(taint_domain,const taint_lattice a, const taint_lattice b)
{
	// standard set union
	taint_lattice ret(new map<name,set<name>>(*a));

	for_each(b->cbegin(),b->cend(),[&](const pair<name,set<name>> &x)
	{
		if(ret->count(x.first))
		{
			set<name> c;

			copy(ret->at(x.first).begin(),ret->at(x.first).end(),inserter(c,c.begin()));
			copy(x.second.begin(),x.second.end(),inserter(c,c.begin()));
			ret->erase(x.first);
			ret->insert(make_pair(x.first,c));
		}
		else
			ret->insert(make_pair(x.first,x.second));
	});
	
	return ret;
}

taint_lattice abstraction(taint_domain,const taint_lattice a, instr_cptr i)
{
	taint_lattice ret(new map<name,set<name>>(*a));
	set<name> r(ret->count(i->assigns->nam) ? ret->at(i->assigns->nam) : set<name>());

	for_each(i->operands.begin(),i->operands.end(),[&](value_ptr v)
	{
		shared_ptr<variable> w;

		if((w = dynamic_pointer_cast<variable>(v)))
		{
			r.insert(w->nam);
			if(ret->count(w->nam))
				copy(ret->at(w->nam).begin(),ret->at(w->nam).end(),inserter(r,r.begin()));
		}
	});

	ret->erase(i->assigns->nam);
	ret->insert(make_pair(i->assigns->nam,r));
	return ret;
}

ostream& operator<<(ostream &os, const taint_lattice l)
{
	if(l)
	{
		for_each(l->cbegin(),l->cend(),[&](const pair<name,set<name>> &p)
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
	return cprop_lattice(new map<name,cprop_element>);
}

bool equal(cprop_domain,const cprop_lattice a, const cprop_lattice b)
{
	cout << a << endl << b << endl;
	cout << "equal? " << (*a == *b) << endl;
	return *a == *b;
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
	cprop_lattice ret(new map<name,cprop_element>(*a));

	for_each(b->cbegin(),b->cend(),[&](const pair<name,cprop_element> &x)
	{
		if(ret->count(x.first))
		{
			cprop_element a = x.second;
			cprop_element b = ret->at(x.first);
			cprop_element c = supremum(a,b);

			ret->erase(x.first);
			cout << "join(" << a << ", " << b << ") = " << c << endl;
			ret->insert(make_pair(x.first,supremum(a,b)));
		}
		else
			ret->insert(x);
	});
	
	return ret;
}

bool operator==(const cprop_element &a, const cprop_element &b) 
{ 
	return a.type == b.type && (b.type != cprop_element::Const || a.value == b.value); 
}

cprop_lattice abstraction(cprop_domain,const cprop_lattice a, instr_cptr i)
{
	cprop_lattice ret(new map<name,cprop_element>(*a));
	vector<cprop_element> ops;

	if(ret->count(i->assigns->nam) && ret->at(i->assigns->nam).type == cprop_element::NonConst)
		return ret;

	transform(i->operands.begin(),i->operands.end(),inserter(ops,ops.begin()),[&](value_ptr v)
	{
		shared_ptr<constant> w;
		shared_ptr<variable> x;

		if((w = dynamic_pointer_cast<constant>(v)))
			return cprop_element(w->val);
		else if((x = dynamic_pointer_cast<variable>(v)))
			return ret->count(x->nam) ? ret->at(x->nam) : cprop_element();
		else // undefined
			return cprop_element();
	});

	bool is_const = all_of(ops.cbegin(),ops.cend(),[](const cprop_element &x) { return x.type == cprop_element::Const; });

	if(!is_const)
	{
		bool is_bot = any_of(ops.cbegin(),ops.cend(),[](const cprop_element &x) { return x.type == cprop_element::Bottom; });
		
		ret->erase(i->assigns->nam);
		ret->insert(make_pair(i->assigns->nam,cprop_element(is_bot ? cprop_element::Bottom : cprop_element::NonConst)));
		return ret;
	}

	// is_const
	unsigned int val;
	switch(i->opcode)
	{
	case instr::Assign:
		val = ops[0].value; break;
	case instr::Not:
		val = ~ops[0].value; break;
	case instr::Phi:
		if(ops[0].value != ops[1].value)
		{
			ret->erase(i->assigns->nam);
			ret->insert(make_pair(i->assigns->nam,cprop_element(cprop_element::NonConst)));
			return ret;
		}
		else
		val = ops[0].value; break;
	case instr::Or:
		val = ops[0].value | ops[1].value; break;
	case instr::And:
		val = ops[0].value & ops[1].value; break;
	case instr::Sub:
		val = ops[0].value - ops[1].value; break;
	case instr::Slice:
	{
		assert(ops[1].value <= ops[2].value);
		unsigned int i = 0;
					
		val = 0;
		while(i < 32)
		{
			val |= ops[0].value & ((i >= ops[1].value && i <= ops[2].value) << i);
			++i;
		}
		val = val >> ops[1].value; break;
	}
	default:
			;
	}
	
	ret->erase(i->assigns->nam);
	ret->insert(make_pair(i->assigns->nam,cprop_element(val)));
	return ret;
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
		auto i = l->cbegin();

		while(i != l->cend())
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
				os << "Err"; break;
			}

			++i;
			if(i != l->cend())
				os << ", ";
		}
	}
	else
		os << "Empty";
	return os;
}
