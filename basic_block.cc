#include <algorithm>
#include <iostream>

#include "basic_block.hh"

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
			case relation::Eq: ret += " = "; break;
			case relation::Neq: ret += " ≠ "; break;
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

const vector<mne_cptr> &basic_block::mnemonics(void) const { return m_mnemonics; }
const vector<instr_ptr> &basic_block::instructions(void) const { return m_instructions; }

pair<basic_block::out_iterator,basic_block::out_iterator> basic_block::outgoing(void)
	{ return make_pair(m_outgoing.begin(),m_outgoing.end()); }

pair<basic_block::in_iterator,basic_block::in_iterator> basic_block::incoming(void)
	{ return make_pair(m_incoming.begin(),m_incoming.end()); }

pair<basic_block::indir_iterator,basic_block::indir_iterator> basic_block::indirect(void)
	{ return make_pair(m_indirect.begin(),m_indirect.end()); }

void basic_block::append_mnemonic(mne_cptr m)
{ 
	assert(m && (m_mnemonics.empty() || m_mnemonics.back()->addresses.end == m->addresses.begin));

	m_mnemonics.push_back(m);
	copy(m->instructions.begin(),m->instructions.end(),inserter(m_instructions,m_instructions.end()));
	
	if(m_addresses.size())
		m_addresses = area(min(m_addresses.begin,m->addresses.begin),max(m_addresses.end,m->addresses.end));
	else
		m_addresses = m->addresses;
}

void basic_block::insert_incoming(guard_ptr g, bblock_ptr m)
	{	assert(g && m); remove_incoming(m); m_incoming.push_back(make_pair(g,m)); };
void basic_block::insert_outgoing(guard_ptr g, bblock_ptr m)
	{	assert(g && m); remove_outgoing(m); m_outgoing.push_back(make_pair(g,m)); };
void basic_block::insert_indirect(guard_ptr g, value_ptr m)
	{	assert(g && m); remove_indirect(m); m_indirect.push_back(make_pair(g,m)); };
	
void basic_block::remove_incoming(bblock_ptr m)
{ 
	assert(m); 
	
	auto i = m_incoming.begin();

	while(i != m_incoming.end())
		if(i->second == m)
		{
			m_incoming.erase(i);
			return;
		}
		else
			++i;
}

void basic_block::remove_outgoing(bblock_ptr m)
{ 
	assert(m); 
	
	auto i = m_outgoing.begin();

	while(i != m_outgoing.end())
		if(i->second == m)
		{
			m_outgoing.erase(i);
			return;
		}
		else
			++i;
}

void basic_block::remove_indirect(value_ptr m)
{ 
	assert(m); 
	
	auto i = m_indirect.begin();

	while(i != m_indirect.end())
		if(i->second == m)
		{
			m_indirect.erase(i);
			return;
		}
		else
			++i;
}

void basic_block::replace_incoming(bblock_ptr a, bblock_ptr b)
{
	assert(a && b);

	auto i = m_incoming.begin();
	while(i != m_incoming.end())
	{
		if(i->second == a)
		{
			auto p = make_pair(i->first,b);
			i = m_incoming.erase(i);
			i = m_incoming.insert(i,p);
		}
		++i;
	}
}

void basic_block::replace_outgoing(bblock_ptr a, bblock_ptr b)
{
	assert(a && b);

	auto i = m_outgoing.begin();
	while(i != m_outgoing.end())
	{
		if(i->second == a)
		{
			auto p = make_pair(i->first,b);
			i = m_outgoing.erase(i);
			i = m_outgoing.insert(i,p);
		}
		++i;
	}
}

void basic_block::clear(void)
{
	m_incoming.clear();
	m_outgoing.clear();
	m_mnemonics.clear();
	m_instructions.clear();
	m_indirect.clear();
}

const area &basic_block::addresses(void) const
	{ return m_addresses; }

void basic_block::prepend_instr(instr_ptr i)
{
	m_instructions.insert(m_instructions.begin(),i);
}

void conditional_jump(bblock_ptr from, bblock_ptr to, guard_ptr g)
{
	assert(from && to && g);

	to->insert_incoming(g,from);
	from->insert_outgoing(g,to);
}

void unconditional_jump(bblock_ptr bb_from, bblock_ptr bb_to)
{
	assert(bb_from && bb_to); 
	guard_ptr g(new guard());

	bb_from->insert_outgoing(g,bb_to);
	bb_to->insert_incoming(g,bb_from);
}

void indirect_jump(bblock_ptr from, value_ptr v, guard_ptr g)
{
	assert(from && g && v);

	from->insert_indirect(g,v);
}

// last == true -> pos is last in `up', last == false -> pos is first in `down'
pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last)
{
	assert(bb);

	bblock_ptr up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::out_iterator j,jend;
	basic_block::in_iterator k,kend;
	basic_block::indir_iterator l,lend;

	// distribute mnemonics under `up' and `down'
	for_each(bb->mnemonics().begin(),bb->mnemonics().end(),[&](mne_cptr m)
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

	// move outgoing edges to down
	tie(j,jend) = bb->outgoing();
	for_each(j,jend,[&](pair<guard_ptr,bblock_ptr> t)
	{
		guard_ptr g;
		bblock_ptr b;

		tie(g,b) = t;

		if(b == bb)
		{
			down->insert_outgoing(g,up);
			up->insert_incoming(g,down);
		}
		else
		{
			down->insert_outgoing(g,b);
			b->insert_incoming(g,down);
			b->remove_incoming(bb);
		}
	});
	
	// ...indirect jumps too
	tie(l,lend) = bb->indirect();
	for_each(l,lend,[&](pair<guard_ptr,value_ptr> t)
	{
		guard_ptr g;
		value_ptr b;

		tie(g,b) = t;
		down->insert_indirect(g,b);
	});
	
	// move incoming edges to up
	tie(k,kend) = bb->incoming();
	for_each(k,kend,[&](pair<guard_ptr,bblock_ptr> t)
	{
		guard_ptr g;
		bblock_ptr b;

		tie(g,b) = t;

		if(b == bb)
		{
			up->insert_incoming(g,down);
			down->insert_outgoing(g,up);
		}
		else
		{
			up->insert_incoming(g,b);
			b->insert_outgoing(g,up);
			b->remove_outgoing(bb);
		}
	});

	bb->clear();
	unconditional_jump(up,down);
	return make_pair(up,down);
}		
