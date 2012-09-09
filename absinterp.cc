#include "absinterp.hh"
#include "dflow.hh"

// taint domain
taint_lattice bottom(taint_domain)
{
	return taint_lattice(new map<string,set<string>>);
}

bool equal(taint_domain,const taint_lattice a, const taint_lattice b)
{
	return *a == *b;
}

taint_lattice supremum(taint_domain,const taint_lattice a, const taint_lattice b)
{
	// standard set union
	taint_lattice ret(new map<string,set<string>>(*a));

	for_each(b->cbegin(),b->cend(),[&](const pair<string,set<string>> &x)
	{
		if(ret->count(x.first))
		{
			set<string> c;
			
			set_union(x.second.begin(),x.second.end(),
								ret->at(x.first).begin(),ret->at(x.first).end(),
								inserter(c,c.begin()));
			ret->insert(make_pair(x.first,c));
		}
		else
			ret->insert(make_pair(x.first,x.second));
	});

	return ret;
}

taint_lattice abstraction(taint_domain,const taint_lattice a, instr_cptr i)
{
	taint_lattice ret(new map<string,set<string>>(*a));
	set<string> r(ret->count(i->assigns->nam.base) ? ret->at(i->assigns->nam.base) : set<string>());
	
	for_each(i->operands.begin(),i->operands.end(),[&](value_ptr v)
	{
		shared_ptr<variable> w;

		if((w = dynamic_pointer_cast<variable>(v)))
		{
			r.insert(w->nam.base);
			if(ret->count(w->nam.base))
				copy(ret->at(w->nam.base).begin(),ret->at(w->nam.base).end(),inserter(r,r.begin()));
		}
	});

	ret->insert(make_pair(i->assigns->nam.base,r));
	return ret;
}
