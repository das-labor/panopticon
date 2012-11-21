#include <algorithm>
#include <functional>
#include <set>
#include <iostream>

#include <basic_block.hh>

using namespace po;

/*
 * relation
 */
relation::relation(rvalue a, Relcode c, rvalue b) : relcode(c), operand1(a), operand2(b) {}

/*
 * guard
 */
guard::guard(void) {}
guard::guard(std::list<relation> rels) : relations(rels) {}
guard::guard(rvalue a, relation::Relcode r, rvalue b) : relations({relation(a,r,b)}) {}

guard_ptr guard::negation(void) const
{
	std::list<relation> rels;

	for_each(relations.cbegin(),relations.cend(),[&](const relation &rel)
	{
		switch(rel.relcode)
		{
			case relation::ULeq: rels.emplace_back(relation(rel.operand1,relation::UGrtr,rel.operand2)); break;
			case relation::SLeq: rels.emplace_back(relation(rel.operand1,relation::SGrtr,rel.operand2)); break;
			case relation::UGeq: rels.emplace_back(relation(rel.operand1,relation::ULess,rel.operand2)); break;
			case relation::SGeq: rels.emplace_back(relation(rel.operand1,relation::SLess,rel.operand2)); break;
			case relation::ULess: rels.emplace_back(relation(rel.operand1,relation::UGeq,rel.operand2)); break;
			case relation::SLess: rels.emplace_back(relation(rel.operand1,relation::SGeq,rel.operand2)); break;
			case relation::UGrtr: rels.emplace_back(relation(rel.operand1,relation::ULeq,rel.operand2)); break;
			case relation::SGrtr: rels.emplace_back(relation(rel.operand1,relation::SLeq,rel.operand2)); break;
			case relation::Eq: rels.emplace_back(relation(rel.operand1,relation::Neq,rel.operand2)); break;
			case relation::Neq: rels.emplace_back(relation(rel.operand1,relation::Eq,rel.operand2)); break;
			default: assert(false);
		}
	});

	return guard_ptr(new guard(rels));;
}

std::ostream& po::operator<<(std::ostream &os, const guard &g)
{
	if(g.relations.empty())
		os << "true";

	auto i = g.relations.cbegin();

	while(i != g.relations.cend())
	{
		const relation &rel(*i++);

		os << rel.operand1;
		switch(rel.relcode)
		{
			case relation::ULeq: os << " ≤ᵤ "; break;
			case relation::SLeq: os << " ≤ₛ "; break;
			case relation::UGeq: os << " ≥ᵤ "; break;
			case relation::SGeq: os << " ≥ₛ "; break;
			case relation::ULess: os << " <ᵤ "; break;
			case relation::SLess: os << " <ₛ "; break;
			case relation::UGrtr: os << " >ᵤ "; break;
			case relation::SGrtr: os << " >ₛ "; break;
			case relation::Eq: os << " = "; break;
			case relation::Neq: os << " ≠ "; break;
			default: assert(false);
		}
		os << rel.operand2;
		if(i != g.relations.cend())
			os << " ∧ ";
	}

	return os;
}

/*
 * ctrans
 */
ctrans::ctrans(guard_ptr g, rvalue v) : guard(g), value(v) {}
ctrans::ctrans(guard_ptr g, bblock_ptr b) : guard(g), bblock(b) {}

/*
 * basic_block
 */
basic_block::basic_block(range<addr_t> a) : m_area(a) {}
basic_block::basic_block(void) {}
std::pair<basic_block::pred_citerator,basic_block::pred_citerator> basic_block::predecessors(void) const
	{ return std::make_pair(pred_citerator(m_incoming.cbegin(),m_incoming.cend()),pred_citerator(m_incoming.cend(),m_incoming.cend())); }

std::pair<basic_block::pred_iterator,basic_block::pred_iterator> basic_block::predecessors(void)
	{ return std::make_pair(pred_iterator(m_incoming.begin(),m_incoming.end()),pred_iterator(m_incoming.end(),m_incoming.end())); }

std::pair<basic_block::succ_citerator,basic_block::succ_citerator> basic_block::successors(void) const
	{ return std::make_pair(succ_citerator(m_outgoing.cbegin(),m_outgoing.cend()),succ_citerator(m_outgoing.cend(),m_outgoing.cend())); }

std::pair<basic_block::succ_iterator,basic_block::succ_iterator> basic_block::successors(void)
	{ return std::make_pair(succ_iterator(m_outgoing.begin(),m_outgoing.end()),succ_iterator(m_outgoing.end(),m_outgoing.end())); }

const std::vector<mnemonic> &basic_block::mnemonics(void) const { return m_mnemonics; }

const std::list<ctrans> &basic_block::incoming(void) const { return m_incoming; }
const std::list<ctrans> &basic_block::outgoing(void) const { return m_outgoing; }

void basic_block::mutate_mnemonics(std::function<void(std::vector<mnemonic>&)> fn)
{
	fn(m_mnemonics);

	// check invariants:
	// 	- mnemonics span a consecutive area

	int next = -1;
	assert(all_of(m_mnemonics.begin(),m_mnemonics.end(),[&](mnemonic &m)
	{
		bool ret = true;
		if(next >= 0)
			ret = !m.area.size() || (unsigned int)next == m.area.begin;
		next = m.area.end;
		return ret;
	}));

	// update m_area
	if(m_mnemonics.empty())
		m_area = range<unsigned int>();
	else
		m_area = range<unsigned int>(m_mnemonics.front().area.begin,m_mnemonics.back().area.end);
}

void basic_block::mutate_incoming(std::function<void(std::list<ctrans>&)> fn)
{
	fn(m_incoming);

	// check invariants:
	// 	- guard non-null
	// 	- no paralell edges
	std::set<bblock_ptr> bbs;
	for(const ctrans &ct: incoming())
	{
		assert(ct.guard);
		assert(!ct.bblock || bbs.insert(ct.bblock).second);
	}
}

void basic_block::mutate_outgoing(std::function<void(std::list<ctrans>&)> fn)
{
	fn(m_outgoing);
	
	// check invariants:
	// 	- guard non-null
	// 	- no paralell edges
	std::set<bblock_ptr> bbs;
	for(const ctrans &ct: outgoing())
	{
		assert(ct.guard);
		assert(!ct.bblock || bbs.insert(ct.bblock).second);
	}
}

void basic_block::clear(void)
{
	m_incoming.clear();
	m_outgoing.clear();
	m_mnemonics.clear();
}

const range<addr_t> &basic_block::area(void) const { return m_area; }

/*
 * free functions
 */
void po::execute(bblock_cptr bb,std::function<void(const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)> f)
{
	size_t sz_mne = bb->mnemonics().size(), i_mne = 0;
	const mnemonic *ary_mne = bb->mnemonics().data();

	while(i_mne < sz_mne)
	{
		const mnemonic &mne = ary_mne[i_mne++];
		size_t sz_instr = mne.instructions.size(), i_instr = 0;
		const instr *ary_instr = mne.instructions.data();

		while(i_instr < sz_instr)
		{
			const instr &instr = ary_instr[i_instr++];

			f(instr.left,instr.function,instr.right);
		}
	}
}

void po::rewrite(bblock_ptr bb,std::function<void(lvalue &,instr::Function,std::vector<rvalue>&)> f)
{
	bb->mutate_mnemonics([&](std::vector<mnemonic> &ms)
	{
		size_t sz_mne = ms.size(), i_mne = 0;
		mnemonic *ary_mne = ms.data();

		while(i_mne < sz_mne)
		{
			mnemonic &mne = ary_mne[i_mne++];
			size_t sz_instr = mne.instructions.size(), i_instr = 0;
			instr *ary_instr = mne.instructions.data();

			while(i_instr < sz_instr)
			{
				instr &instr = ary_instr[i_instr++];

				f(instr.left,instr.function,instr.right);
			}
		}
	});
}

void po::conditional_jump(bblock_ptr from, bblock_ptr to, guard_ptr g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void po::conditional_jump(rvalue from, bblock_ptr to, guard_ptr g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void po::conditional_jump(bblock_ptr from, rvalue to, guard_ptr g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }

void po::unconditional_jump(bblock_ptr from, bblock_ptr to) { conditional_jump(from,to,guard_ptr(new guard())); }
void po::unconditional_jump(rvalue from, bblock_ptr to) { conditional_jump(from,to,guard_ptr(new guard())); }
void po::unconditional_jump(bblock_ptr from, rvalue to) { conditional_jump(from,to,guard_ptr(new guard())); }

void po::replace_incoming(bblock_ptr to, bblock_ptr oldbb, bblock_ptr newbb)
{ 
	assert(to && oldbb && newbb);
	to->mutate_incoming([&](std::list<ctrans> &in)
	{ 
		replace(in,oldbb,newbb); 
	}); 
}

void po::replace_outgoing(bblock_ptr from, bblock_ptr oldbb, bblock_ptr newbb)
{
	assert(from && oldbb && newbb);
	from->mutate_outgoing([&](std::list<ctrans> &out) 
	{ 
		replace(out,oldbb,newbb); 
	}); 
}

void po::resolve_incoming(bblock_ptr to, rvalue v, bblock_ptr bb) 
{ 
	assert(to && bb);
	to->mutate_incoming([&](std::list<ctrans> &in)
	{ 
		resolve(in,v,bb); 
	}); 
}

void po::resolve_outgoing(bblock_ptr from, rvalue v, bblock_ptr bb)
{
	assert(from && bb);
	from->mutate_outgoing([&](std::list<ctrans> &out) 
	{ 
		resolve(out,v,bb); 
	}); 
}

// last == true -> pos is last in `up', last == false -> pos is first in `down'
std::pair<bblock_ptr,bblock_ptr> po::split(bblock_ptr bb, addr_t pos, bool last)
{
	assert(bb);

	bblock_ptr up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::out_iterator j,jend;
	basic_block::in_iterator k,kend;
	std::function<void(bool,bblock_ptr,ctrans)> append = [](bool in, bblock_ptr bb, ctrans ct)
	{
		if(in)
			bb->mutate_incoming([&](std::list<ctrans> &l) { l.push_back(ct); });
		else
			bb->mutate_outgoing([&](std::list<ctrans> &l) { l.push_back(ct); });
	};

	// distribute mnemonics under `up' and `down'
	for_each(bb->mnemonics().begin(),bb->mnemonics().end(),[&](const mnemonic &m)
	{	
		assert(!m.area.includes(pos) || m.area.begin == pos);

		if(!last)
			sw |= m.area.includes(pos);
		
		if(sw)
			down->mutate_mnemonics([&](std::vector<mnemonic> &ms) { ms.push_back(m); });
		else	
			up->mutate_mnemonics([&](std::vector<mnemonic> &ms) { ms.push_back(m); });
		
		if(last)
			sw |= m.area.includes(pos);
	});
	assert(sw);

	// move outgoing ctrans to down
	for_each(bb->outgoing().begin(),bb->outgoing().end(),[&](const ctrans &ct)
	{
		if(ct.bblock == bb)
		{
			append(false,down,ctrans(ct.guard,up));
			append(true,up,ctrans(ct.guard,up));
		}
		else
		{
			if(ct.bblock)
			{
				append(false,down,ctrans(ct.guard,ct.bblock));
				ct.bblock->mutate_incoming([&](std::list<ctrans> &in)
				{
					in.emplace_back(ctrans(ct.guard,down));
					in.erase(find_if(in.begin(),in.end(),[&](const ctrans &ct)
						{ return ct.bblock == bb; }));
				});
			}
			else
				append(false,down,ctrans(ct.guard,ct.value));
		}
	});
	
	// move incoming edges to up
	for_each(bb->incoming().begin(),bb->incoming().end(),[&](const ctrans &ct)
	{
		if(ct.bblock == bb)
		{
			append(true,up,ctrans(ct.guard,down));
			append(false,down,ctrans(ct.guard,up));
		}
		else
		{
			if(ct.bblock)
			{
				append(true,up,ctrans(ct.guard,ct.bblock));
				ct.bblock->mutate_outgoing([&](std::list<ctrans> &out)
				{
					out.emplace_back(ctrans(ct.guard,up));
					out.erase(find_if(out.begin(),out.end(),[&](const ctrans &ct)
						{ return ct.bblock == bb; }));
				});
			}
			else
				append(true,up,ctrans(ct.guard,ct.value));
		}
	});

	bb->clear();
	unconditional_jump(up,down);
	return std::make_pair(up,down);
}

bblock_ptr po::merge(bblock_ptr up, bblock_ptr down)
{
	assert(up && down);
	if(up->area().begin == down->area().end) tie(up,down) = std::make_pair(down,up);
	assert(up->area().end == down->area().begin);

	bblock_ptr ret(new basic_block());
	auto fn = [&ret](const bblock_ptr &bb, const mnemonic &m) { ret->mutate_mnemonics([&](std::vector<mnemonic> &ms)
		{ ms.push_back(m); }); };

	for_each(up->mnemonics().begin(),up->mnemonics().end(),std::bind(fn,up,std::placeholders::_1));
	for_each(down->mnemonics().begin(),down->mnemonics().end(),std::bind(fn,down,std::placeholders::_1));

	for_each(up->incoming().begin(),up->incoming().end(),[&](const ctrans &ct)
	{
		if(ct.bblock)
			replace_outgoing(ct.bblock,up,ret);
		ret->mutate_incoming([&](std::list<ctrans> &in) { in.emplace_back(ct); });
	});
			
	for_each(down->outgoing().begin(),down->outgoing().end(),[&](const ctrans &ct)
	{
		if(ct.bblock)
			replace_incoming(ct.bblock,down,ret);
		ret->mutate_outgoing([&](std::list<ctrans> &out) { out.emplace_back(ct); });
	});
	
	up->clear();
	down->clear();
	return ret;
}

void po::replace(std::list<ctrans> &lst, bblock_ptr from, bblock_ptr to)
{
	assert(from && to);

	auto i = lst.begin();
	while(i != lst.end())
	{
		ctrans ct = *i;
		if(ct.bblock == from)
			i = lst.insert(lst.erase(i),ctrans(ct.guard,to));
		++i;
	}
}

void po::resolve(std::list<ctrans> &lst, rvalue v, bblock_ptr bb)
{
	assert(bb);

	auto i = lst.begin();
	while(i != lst.end())
	{
		ctrans ct = *i;
		if(ct.value == v)
			i = lst.insert(lst.erase(i),ctrans(ct.guard,bb));
		++i;
	}
}

void po::conditional_jump(const ctrans &from, const ctrans &to)
{
	if(from.bblock)
		from.bblock->mutate_outgoing([&](std::list<ctrans> &out) { out.emplace_back(to); });
	if(to.bblock)
		to.bblock->mutate_incoming([&](std::list<ctrans> &in) { in.emplace_back(from); });
}
