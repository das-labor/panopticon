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
