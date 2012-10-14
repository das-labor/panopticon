#include <algorithm>
#include <functional>
#include <iostream>

#include "basic_block.hh"

/*
 * relation
 */
relation::relation(valproxy a, Relcode c, valproxy b) : relcode(c), operand1(a.value), operand2(b.value) {}

/*
 * guard
 */
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

/*
 * ctrans
 */
ctrans::ctrans(guard_ptr g, value_ptr v) : guard(g), value(v) {}
ctrans::ctrans(guard_ptr g, bblock_ptr b) : guard(g), bblock(b) {}

var_cptr ctrans::variable(void) const { return std::dynamic_pointer_cast< ::variable>(value); };
const_cptr ctrans::constant(void) const { return dynamic_pointer_cast< ::constant>(value); };

var_ptr ctrans::variable(void) { return dynamic_pointer_cast< ::variable>(value); };
const_ptr ctrans::constant(void) { return dynamic_pointer_cast< ::constant>(value); };

/*
 * basic_block
 */
pair<basic_block::pred_iterator,basic_block::pred_iterator> basic_block::predecessors(void)
	{ return make_pair(pred_iterator(m_incoming.begin(),m_incoming.end()),pred_iterator(m_incoming.end(),m_incoming.end())); }

pair<basic_block::succ_iterator,basic_block::succ_iterator> basic_block::successors(void)
	{ return make_pair(succ_iterator(m_outgoing.begin(),m_outgoing.end()),succ_iterator(m_outgoing.end(),m_outgoing.end())); }

pair<basic_block::out_iterator,basic_block::out_iterator> basic_block::outgoing(void)
	{ return make_pair(m_outgoing.begin(),m_outgoing.end()); }

pair<basic_block::in_iterator,basic_block::in_iterator> basic_block::incoming(void)
	{ return make_pair(m_incoming.begin(),m_incoming.end()); }

const vector<mne_cptr> &basic_block::mnemonics(void) const { return m_mnemonics; }
const vector<instr_ptr> &basic_block::instructions(void) const { return m_instructions; }
std::pair<instr_citerator,instr_citerator> basic_block::instructions(mne_cptr m) const 
{ 
	auto p = m_map.equal_range(m); 
	return make_pair(instr_citerator(p.first),instr_citerator(p.second));
}

void basic_block::prepend_instr(instr_ptr i) { m_instructions.insert(m_instructions.begin(),i); }

void basic_block::insert_incoming(guard_ptr g, bblock_ptr bb) { ctrans ct(g,bb); insert(m_incoming,ct); }
void basic_block::insert_incoming(guard_ptr g, value_ptr v) { ctrans ct(g,v); insert(m_incoming,ct); }
void basic_block::insert_outgoing(guard_ptr g, bblock_ptr bb) { ctrans ct(g,bb); insert(m_outgoing,ct); }
void basic_block::insert_outgoing(guard_ptr g, value_ptr v) { ctrans ct(g,v); insert(m_outgoing,ct); }
void basic_block::insert_incoming(const ctrans &ct) { insert(m_incoming,ct); }
void basic_block::insert_outgoing(const ctrans &ct) { insert(m_outgoing,ct); }

void basic_block::remove_incoming(bblock_ptr m) { remove(m_incoming,[&m](const ctrans &ct) { return ct.bblock == m; }); }
void basic_block::remove_incoming(value_ptr v) { remove(m_incoming,[&v](const ctrans &ct) { return ct.value == v; }); }
void basic_block::remove_outgoing(bblock_ptr m) { remove(m_outgoing,[&m](const ctrans &ct) { return ct.bblock == m; }); }
void basic_block::remove_outgoing(value_ptr v) { remove(m_outgoing,[&v](const ctrans &ct) { return ct.value == v; }); }
void basic_block::replace_incoming(bblock_ptr a, bblock_ptr b) { replace(m_incoming,a,b); }
void basic_block::replace_outgoing(bblock_ptr a, bblock_ptr b) { replace(m_outgoing,a,b); }

void basic_block::resolve_incoming(value_ptr v, bblock_ptr bb) { resolve(m_incoming,v,bb); }
void basic_block::resolve_outgoing(value_ptr v, bblock_ptr bb) { resolve(m_outgoing,v,bb); }

void basic_block::clear(void)
{
	m_incoming.clear();
	m_outgoing.clear();
	m_mnemonics.clear();
	m_instructions.clear();
}

const area &basic_block::addresses(void) const { return m_addresses; }

void basic_block::insert(std::list<ctrans> &lst, const ctrans &ct)
{	
	if(ct.bblock)
		remove(lst,[&ct](const ctrans &c) { return c.bblock == ct.bblock; });
	if(ct.value)
		remove(lst,[&ct](const ctrans &c) { return c.value == ct.value; });
	lst.push_back(ctrans(ct));
}

void basic_block::remove(std::list<ctrans> &lst, std::function<bool(const ctrans &)> p)
{ 
	auto i = lst.begin();

	while(i != lst.end())
		if(p(*i))
			i = lst.erase(i);
		else
			++i;
}

void basic_block::replace(std::list<ctrans> &lst, bblock_ptr from, bblock_ptr to)
{
	assert(from && to);

	auto i = lst.begin();
	while(i != lst.end())
	{
		if(i->bblock == from)
			i->bblock = to;
		++i;
	}
}

void basic_block::resolve(std::list<ctrans> &lst, value_ptr v, bblock_ptr bb)
{
	assert(v && bb);

	auto i = lst.begin();
	while(i != lst.end())
	{
		if(i->value == v)
			i->bblock = bb;
		++i;
	}
}

void conditional_jump(bblock_ptr from, bblock_ptr to, guard_ptr g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void conditional_jump(value_ptr from, bblock_ptr to, guard_ptr g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void conditional_jump(bblock_ptr from, value_ptr to, guard_ptr g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void conditional_jump(ctrans &from, ctrans &to)
{
	if(from.bblock)
		from.bblock->insert_outgoing(to);
	if(to.bblock)
		to.bblock->insert_incoming(from);
}

void unconditional_jump(bblock_ptr from, bblock_ptr to) { conditional_jump(from,to,guard_ptr(new guard())); }
void unconditional_jump(value_ptr from, bblock_ptr to) { conditional_jump(from,to,guard_ptr(new guard())); }
void unconditional_jump(bblock_ptr from, value_ptr to) { conditional_jump(from,to,guard_ptr(new guard())); }

// last == true -> pos is last in `up', last == false -> pos is first in `down'
pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last)
{
	assert(bb);

	bblock_ptr up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::out_iterator j,jend;
	basic_block::in_iterator k,kend;

	// distribute mnemonics under `up' and `down'
	for_each(bb->mnemonics().begin(),bb->mnemonics().end(),[&](mne_cptr m)
	{	
		assert(!m->addresses.includes(pos) || m->addresses.begin == pos);

		if(!last)
			sw |= m->addresses.includes(pos);
		
		if(sw)
			down->append_mnemonic(m,bb->instructions(m));
		else	
			up->append_mnemonic(m,bb->instructions(m));
		
		if(last)
			sw |= m->addresses.includes(pos);
	});
	assert(sw);

	// move outgoing ctranss to down
	tie(j,jend) = bb->outgoing();
	for_each(j,jend,[&](const ctrans &ct)
	{
		if(ct.bblock == bb)
		{
			down->insert_outgoing(ct.guard,up);
			up->insert_incoming(ct.guard,down);
		}
		else
		{
			if(ct.bblock)
			{
				down->insert_outgoing(ct.guard,ct.bblock);
				ct.bblock->insert_incoming(ct.guard,down);
				ct.bblock->remove_incoming(bb);
			}
			else
				down->insert_outgoing(ct.guard,ct.value);
		}
	});
	
	// move incoming edges to up
	tie(k,kend) = bb->incoming();
	for_each(k,kend,[&](const ctrans &ct)
	{
		if(ct.bblock == bb)
		{
			up->insert_incoming(ct.guard,down);
			down->insert_outgoing(ct.guard,up);
		}
		else
		{
			if(ct.bblock)
			{
				up->insert_incoming(ct.guard,ct.bblock);
				ct.bblock->insert_outgoing(ct.guard,up);
				ct.bblock->remove_outgoing(bb);
			}
			else
				up->insert_incoming(ct.guard,ct.value);
		}
	});

	bb->clear();
	unconditional_jump(up,down);
	return make_pair(up,down);
}

bblock_ptr merge(bblock_ptr up, bblock_ptr down)
{
	assert(up && down);
	if(up->addresses().begin == down->addresses().end) tie(up,down) = make_pair(down,up);
	assert(up->addresses().end == down->addresses().begin);

	bblock_ptr ret(new basic_block());
	auto fn = [&ret](const bblock_ptr &bb, const mne_cptr &m) { ret->append_mnemonic(m,bb->instructions(m)); };

	for_each(up->mnemonics().begin(),up->mnemonics().end(),std::bind(fn,up,std::placeholders::_1));
	for_each(down->mnemonics().begin(),down->mnemonics().end(),std::bind(fn,down,std::placeholders::_1));

	basic_block::in_iterator i,iend;
	tie(i,iend) = up->incoming();
	for_each(i,iend,[&](const ctrans &ct)
	{
		if(ct.bblock)
			ct.bblock->replace_outgoing(up,ret);
		ret->insert_incoming(ct);
	});
			
	basic_block::out_iterator j,jend;
	tie(j,jend) = down->outgoing();
	for_each(j,jend,[&](const ctrans &ct)
	{
		if(ct.bblock)
			ct.bblock->replace_incoming(down,ret);
		ret->insert_outgoing(ct);
	});
	
	up->clear();
	down->clear();
	return ret;
}
