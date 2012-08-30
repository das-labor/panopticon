#include <algorithm>

#include "basic_block.hh"

/*
void basic_block::accept_instr(instr_ptr i) 
{ 
	instructions.push_back(make_pair(i,0)); 
	
	if(addresses.isset)
		addresses = area(min(addresses.begin,i->addresses.begin),max(addresses.end,i->addresses.end));
	else
		addresses = i->addresses;
}*/

relation::relation(valproxy a, Relcode c, valproxy b) : relcode(c), operand1(a.value), operand2(b.value) {}

guard::guard(void) {}
guard::guard(vector<relation> rels) : relations(rels) {}
guard::guard(valproxy a, relation::Relcode r, valproxy b) : relations({relation(a,r,b)}) {}

guard_ptr guard::negation(void) const
{
	guard_ptr g(new guard());
	g->relations.reserve(relations.size());

	for_each(relations.cbegin(),relations.cend(),[&](const relation &rel)
	{
		switch(rel.relcode)
		{
			case relation::ULeq: g->relations.emplace_back(relation(rel.operand1,relation::UGrtr,rel.operand2)); break;
			case relation::SLeq: g->relations.emplace_back(relation(rel.operand1,relation::SGrtr,rel.operand2)); break;
			case relation::UGeq: g->relations.emplace_back(relation(rel.operand1,relation::ULess,rel.operand2)); break;
			case relation::SGeq: g->relations.emplace_back(relation(rel.operand1,relation::SLess,rel.operand2)); break;
			case relation::ULess: g->relations.emplace_back(relation(rel.operand1,relation::UGeq,rel.operand2)); break;
			case relation::SLess: g->relations.emplace_back(relation(rel.operand1,relation::SGeq,rel.operand2)); break;
			case relation::UGrtr: g->relations.emplace_back(relation(rel.operand1,relation::ULeq,rel.operand2)); break;
			case relation::SGrtr: g->relations.emplace_back(relation(rel.operand1,relation::SLeq,rel.operand2)); break;
			case relation::Eq: g->relations.emplace_back(relation(rel.operand1,relation::Neq,rel.operand2)); break;
			case relation::Neq: g->relations.emplace_back(relation(rel.operand1,relation::Eq,rel.operand2)); break;
			default: assert(false);
		}
	});

	return g;
}

string guard::inspect(void) const
{
	string ret(relations.empty() ? "true" : "");
	auto i = relations.cbegin();

	while(i != relations.cend())
	{
		const relation &rel(*i++);

		ret += rel.operand1->inspect();
		switch(rel.relcode)
		{
			case relation::ULeq: ret += " ≤ᵤ "; break;
			case relation::SLeq: ret += " ≤ₛ "; break;
			case relation::UGeq: ret += " ≥ᵤ "; break;
			case relation::SGeq: ret += " ≥ₛ "; break;
			case relation::ULess: ret += " <ᵤ "; break;
			case relation::SLess: ret += " <ₛ "; break;
			case relation::UGrtr: ret += " >ᵤ "; break;
			case relation::SGrtr: ret += " >ₛ "; break;
			case relation::Eq: ret += " =ᵤ "; break;
			case relation::Neq: ret += " ≠ₛ "; break;
			default: assert(false);
		}
		ret += rel.operand2->inspect();
		if(i != relations.cend())
			ret += " ∧ ";
	}

	return ret;
}

pair<basic_block::pred_iterator,basic_block::pred_iterator> basic_block::predecessors(void)
	{ return make_pair(pred_iterator(m_incoming.begin()),pred_iterator(m_incoming.end())); }

pair<basic_block::succ_iterator,basic_block::succ_iterator> basic_block::successors(void)
	{ return make_pair(succ_iterator(m_outgoing.begin()),succ_iterator(m_outgoing.end())); }

pair<basic_block::iterator,basic_block::iterator> basic_block::mnemonics(void)
	{ return make_pair(m_mnemonics.begin(),m_mnemonics.end()); }

pair<basic_block::out_iterator,basic_block::out_iterator> basic_block::outgoing(void)
	{ return make_pair(m_outgoing.begin(),m_outgoing.end()); }

pair<basic_block::in_iterator,basic_block::in_iterator> basic_block::incoming(void)
	{ return make_pair(m_incoming.begin(),m_incoming.end()); }

void basic_block::append_mnemonic(mne_cptr m)
{ 
	assert(m_mnemonics.empty() || m_mnemonics.back()->addresses.end == m->addresses.begin);

	m_mnemonics.push_back(m);
	
	if(m_addresses.isset)
		m_addresses = area(min(m_addresses.begin,m->addresses.begin),max(m_addresses.end,m->addresses.end));
	else
		m_addresses = m->addresses;
}

void basic_block::insert_incoming(guard_cptr g, bblock_ptr m)
	{	m_incoming.push_back(make_pair(g,m)); };
void basic_block::insert_outgoing(guard_cptr g, bblock_ptr m)
	{	m_outgoing.push_back(make_pair(g,m)); };
	
void basic_block::remove_incoming(bblock_ptr m)
	{ remove_if(m_incoming.begin(),m_incoming.end(),[&m](pair<guard_cptr,bblock_ptr> p) { return p.second == m; }); }
void basic_block::remove_outgoing(bblock_ptr m)
	{ remove_if(m_outgoing.begin(),m_outgoing.end(),[&m](pair<guard_cptr,bblock_ptr> p) { return p.second == m; }); }

void basic_block::replace_incoming(bblock_ptr a, bblock_ptr b)
{
	auto i = m_incoming.begin();
	while(i != m_incoming.end())
		if(i->second == a)
			*i++ = make_pair(i->first,b);
		else
			++i;
}

void basic_block::replace_outgoing(bblock_ptr a, bblock_ptr b)
{
	auto i = m_outgoing.begin();
	while(i != m_outgoing.end())
		if(i->second == a)
			*i++ = make_pair(i->first,b);
		else
			++i;
}

const area &basic_block::addresses(void) const
	{ return m_addresses; }

pair<bblock_ptr,bblock_ptr> conditional(bblock_ptr bb, guard_ptr g, bblock_ptr trueb, bblock_ptr falseb)
{
	assert(bb);
	bblock_ptr tret(trueb), fret(falseb);

	if(!tret) tret = bblock_ptr(new basic_block());
	if(!fret) fret = bblock_ptr(new basic_block());

	tret->insert_incoming(g,bb);
	fret->insert_incoming(g->negation(),bb);

	bb->insert_outgoing(	g,tret);
	bb->insert_outgoing(g->negation(),fret);

	return make_pair(tret,fret);
}

void unconditional(bblock_ptr bb_from, bblock_ptr bb_to)
{
	assert(bb_from && bb_to); 

	guard_ptr g(new guard());

	bb_from->insert_outgoing(g,bb_to);
	bb_to->insert_incoming(g,bb_to);
}

// last == true -> pos is last in `up', last == false -> pos is first in `down'
pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last)
{
	bblock_ptr up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::iterator i,iend;

	tie(i,iend) = bb->mnemonics();

	for_each(i,iend,[&](mne_cptr m)
	{	
		if(!last)
			sw |= m->addresses.includes(pos);
		
		if(sw)
			down->append_mnemonic(m);
		else	
			up->append_mnemonic(m);
		
		if(last)
			sw |= m->addresses.includes(pos);
	});
	assert(sw);

	// connect self references (loops) to `up'
	reroute_out(bb,bb,up);

	// connect outgoing to `down'
	basic_block::out_iterator j,jend;

	tie(j,jend) = bb->outgoing();

	for_each(j,jend,[&](pair<guard_cptr,bblock_ptr> t)
	{
		bblock_ptr p = t.second;
		basic_block::pred_iterator k,kend;

		p->replace_incoming(bb,down);
		down->insert_outgoing(t.first,t.second);
	});

	// connect incoming to `up'
	basic_block::pred_iterator k,kend;
	tie(k,kend) = bb->predecessors();
	
	for_each(k,kend,[&](bblock_ptr pred) { reroute_out(pred,bb,up); });

	unconditional(up,down);
	return make_pair(up,down);
}		

void reroute_out(bblock_ptr from, bblock_ptr old_to, bblock_ptr new_to)
{
	basic_block::out_iterator i,iend;

	tie(i,iend) = from->outgoing();
	
	while(i != iend)
	{
		pair<guard_cptr,bblock_ptr> &t(*i);

		if(t.second == old_to)
		{
			from->replace_outgoing(old_to,new_to);
			from->insert_outgoing(t.first,old_to);

			new_to->insert_incoming(t.first,from);
			old_to->remove_incoming(from);
		}
		++i;
	}
}
