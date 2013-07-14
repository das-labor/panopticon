#include <algorithm>
#include <functional>
#include <set>
#include <iostream>
#include <cassert>

#include <basic_block.hh>
#ifdef NDEBUG
#error sss
#endif
using namespace po;
using namespace std;

/*
 * relation
 */
relation::relation(rvalue a, Relcode c, rvalue b) : relcode(c), operand1(a), operand2(b) {}

/*
 * guard
 */
guard::guard(void) : relations() {}
guard::guard(const guard &g) : relations(g.relations) {}
guard::guard(guard &&g) : relations(move(g.relations)) {}
guard::guard(const list<relation> &rels) : relations(rels) {}
guard::guard(list<relation> &&rels) : relations(move(rels)) {}
guard::guard(rvalue a, relation::Relcode r, rvalue b) : relations({relation(a,r,b)}) {}

guard &guard::operator=(const guard &g)
{
	if(&g != this)
		relations = g.relations;
	return *this;
}

guard &guard::operator=(guard &&g)
{
	relations = move(g.relations);
	return *this;
}

guard guard::negation(void) const
{
	list<relation> rels;

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

	return guard(rels);
}

string po::pretty(relation::Relcode r)
{
	switch(r)
	{
		case relation::ULeq: 	return " ≤ᵤ ";
		case relation::SLeq: 	return " ≤ₛ ";
		case relation::UGeq: 	return " ≥ᵤ ";
		case relation::SGeq: 	return " ≥ₛ ";
		case relation::ULess: return " <ᵤ ";
		case relation::SLess: return " <ₛ ";
		case relation::UGrtr: return " >ᵤ ";
		case relation::SGrtr: return " >ₛ ";
		case relation::Eq: 		return " = ";
		case relation::Neq: 	return " ≠ ";
		default: assert(false);
	}
}

string po::symbolic(relation::Relcode r)
{
	switch(r)
	{
		case relation::ULeq:	return "po:u-less-equal";
		case relation::SLeq: 	return "po:s-less-equal";
		case relation::UGeq: 	return "po:u-greater-equal";
		case relation::SGeq: 	return "po:s-greater-equal";
		case relation::ULess: return "po:u-less";
		case relation::SLess: return "po:s-less";
		case relation::UGrtr: return "po:u-greater";
		case relation::SGrtr: return "po:s-greater";
		case relation::Eq: 		return "po:equal";
		case relation::Neq: 	return "po:not-equal";
		default: assert(false);
	}
}

ostream& po::operator<<(ostream &os, const guard &g)
{
	if(g.relations.empty())
		os << "true";

	auto i = g.relations.cbegin();

	while(i != g.relations.cend())
	{
		const relation &rel(*i++);

		os << rel.operand1 << pretty(rel.relcode) << rel.operand2;
		if(i != g.relations.cend())
			os << " ∧ ";
	}

	return os;
}

odotstream &po::operator<<(odotstream &os, const guard &g)
{
	static_cast<ostringstream &>(os) << g;
	return os;
}

/*
 * ctrans
 */
ctrans::ctrans(struct guard g, rvalue v) : condition(g), value(v), bblock() {}
ctrans::ctrans(struct guard g, bblock_ptr b) : condition(g), value(), bblock(b) {}

/*
 * basic_block
 */
bblock_ptr basic_block::unmarshal(const rdf::node &node, proc_cptr proc, const rdf::storage &store)
{
	rdf::statement type = store.first(node,"type"_rdf,"BasicBlock"_po);
	rdf::stream mnes = store.select(node,"include"_po,nullptr);
	rdf::stream cts = store.select(node,"preceds"_po,nullptr);
	bblock_ptr ret(new basic_block());
	std::list<mnemonic> mne_lst;

	// mnemoics
	while(!mnes.eof())
	{
		rdf::statement st;

		mnes >> st;
		mne_lst.emplace_back(mnemonic::unmarshal(st.object(),store));
	}

	mne_lst.sort([](const mnemonic &a, const mnemonic &b)
		{ return a.area.begin < b.area.begin; });

	ret->mutate_mnemonics([&](vector<mnemonic> &ms)
		{ move(mne_lst.begin(),mne_lst.end(),inserter(ms,ms.end())); });

	// constrol transfers
	while(!cts.eof())
	{
		rdf::statement st;

		cts >> st;
	}

	return ret;
}

basic_block::basic_block(void)
: m_area(), m_mnemonics(), m_incoming(), m_outgoing()
{}

pair<basic_block::pred_citerator,basic_block::pred_citerator> basic_block::predecessors(void) const
	{ return make_pair(pred_citerator(m_incoming.cbegin(),m_incoming.cend()),pred_citerator(m_incoming.cend(),m_incoming.cend())); }

pair<basic_block::pred_iterator,basic_block::pred_iterator> basic_block::predecessors(void)
	{ return make_pair(pred_iterator(m_incoming.begin(),m_incoming.end()),pred_iterator(m_incoming.end(),m_incoming.end())); }

pair<basic_block::succ_citerator,basic_block::succ_citerator> basic_block::successors(void) const
	{ return make_pair(succ_citerator(m_outgoing.cbegin(),m_outgoing.cend()),succ_citerator(m_outgoing.cend(),m_outgoing.cend())); }

pair<basic_block::succ_iterator,basic_block::succ_iterator> basic_block::successors(void)
	{ return make_pair(succ_iterator(m_outgoing.begin(),m_outgoing.end()),succ_iterator(m_outgoing.end(),m_outgoing.end())); }

const vector<mnemonic> &basic_block::mnemonics(void) const { return m_mnemonics; }

const list<ctrans> &basic_block::incoming(void) const { return m_incoming; }
const list<ctrans> &basic_block::outgoing(void) const { return m_outgoing; }

void basic_block::mutate_mnemonics(function<void(vector<mnemonic>&)> fn)
{
	fn(m_mnemonics);

	// check invariants:
	// 	- mnemonics span a consecutive area

	addr_t first = naddr, last = naddr;
	assert(all_of(m_mnemonics.begin(),m_mnemonics.end(),[&](mnemonic &m)
	{
		bool ret = true;

		if(first == naddr) 
			first = m.area.begin;
		else
			ret &= first < m.area.begin;

		if(last != naddr)
			ret &= last == m.area.begin;

		last = m.area.end;
		return ret;
	}));

	// update m_area
	if(m_mnemonics.empty())
		m_area = range<addr_t>();
	else
		m_area = range<addr_t>(first,last);
}

void basic_block::mutate_incoming(function<void(list<ctrans>&)> fn)
{
	fn(m_incoming);

	// check invariants:
	// 	- condition non-null
	// 	- no paralell edges
	set<bblock_ptr> bbs;
	for(const ctrans &ct: incoming())
	{
		assert(!ct.bblock.lock() || bbs.insert(ct.bblock.lock()).second);
	}
}

void basic_block::mutate_outgoing(function<void(list<ctrans>&)> fn)
{
	fn(m_outgoing);
	
	// check invariants:
	// 	- condition non-null
	// 	- no paralell edges
	set<bblock_ptr> bbs;
	for(const ctrans &ct: outgoing())
	{
		assert(!ct.bblock.lock() || bbs.insert(ct.bblock.lock()).second);
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
bool po::operator<(const bblock_wptr &a, const bblock_wptr &b)
{
	return owner_less<bblock_wptr>()(a, b);
}

bool po::operator<(const bblock_cwptr &a, const bblock_cwptr &b)
{
	return owner_less<bblock_cwptr>()(a, b);
}

odotstream &po::operator<<(odotstream &os, const basic_block &bb)
{
	os << unique_name(bb) << " [shape=record,label=<<table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";

	for(const mnemonic &m: bb.mnemonics())
		os << m;

	os << "</table>>];" << endl;

	for(const ctrans &ct: bb.outgoing())
		if(ct.bblock.lock())
		{
			os << unique_name(bb) << " -> " << unique_name(*ct.bblock.lock()) << " [label=\"" << ct.condition << "\"];" << endl;
		}
		else
		{
			os << unique_name(bb) << " -> " << unique_name(ct.value) << " [label=\"" << ct.condition << "\"];" << endl;
			os << unique_name(ct.value) << " [label=\"" << ct.value << "\"];" << endl;
		}

	return os;
}

oturtlestream &po::operator<<(oturtlestream &os, const basic_block &bb)
{
	os << "[" << " rdf:type po:BasicBlock;" << endl;
	for(const mnemonic &mne: bb.mnemonics())
		os << " po:include " << mne << ";" << endl;

	/*
	for(const ctrans &ct: bb.outgoing())
	{
		os << " po:preceds " << ct << endl;
	}
	/
		if(ct.bblock.lock())
			os << g << " po:target :" << unique_name(*ct.bblock.lock()) << "." << endl;
		else
			os << g << " po:target " << ct.value << "." << endl;

		for(const relation &rel: ct.condition.relations)
			os << g << " po:condition [ po:left " << rel.operand1 
													 << "; po:right " << rel.operand2 
													 << "; po:relation " << symbolic(rel.relcode)
													 << "]." << endl;
	}*/

	os << "];";

	return os;
}

string po::unique_name(const basic_block &bb)
{
	return "bblock_" + to_string(bb.area().begin) + "_" + to_string(bb.area().end);
}

void po::execute2(bblock_cptr bb,function<void(const instr&)> f)
{
	size_t sz_mne = bb->mnemonics().size(), i_mne = 0;
	const mnemonic *ary_mne = bb->mnemonics().data();

	while(i_mne < sz_mne)
	{
		const mnemonic &mne = ary_mne[i_mne++];
		size_t sz_instr = mne.instructions.size(), i_instr = 0;
		const instr *ary_instr = mne.instructions.data();

		while(i_instr < sz_instr)
			f(ary_instr[i_instr++]);
	}
}

void po::execute(bblock_cptr bb,function<void(const lvalue &left, instr::Function fn, const vector<rvalue> &right)> f)
{
	execute2(bb,[&](const instr &i)
	{
		f(i.left,i.function,i.right);
	});
}

void po::rewrite(bblock_ptr bb,function<void(lvalue &,instr::Function,vector<rvalue>&)> f)
{
	bb->mutate_mnemonics([&](vector<mnemonic> &ms)
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

void po::conditional_jump(bblock_ptr from, bblock_ptr to, guard g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void po::conditional_jump(rvalue from, bblock_ptr to, guard g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void po::conditional_jump(bblock_ptr from, rvalue to, guard g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }

void po::unconditional_jump(bblock_ptr from, bblock_ptr to) { conditional_jump(from,to,guard()); }
void po::unconditional_jump(rvalue from, bblock_ptr to) { conditional_jump(from,to,guard()); }
void po::unconditional_jump(bblock_ptr from, rvalue to) { conditional_jump(from,to,guard()); }

void po::replace_incoming(bblock_ptr to, bblock_ptr oldbb, bblock_ptr newbb)
{ 
	assert(to && oldbb && newbb);
	to->mutate_incoming([&](list<ctrans> &in)
	{ 
		replace(in,oldbb,newbb); 
	}); 
}

void po::replace_outgoing(bblock_ptr from, bblock_ptr oldbb, bblock_ptr newbb)
{
	assert(from && oldbb && newbb);
	from->mutate_outgoing([&](list<ctrans> &out) 
	{ 
		replace(out,oldbb,newbb); 
	}); 
}

void po::resolve_incoming(bblock_ptr to, rvalue v, bblock_ptr bb) 
{ 
	assert(to && bb);
	to->mutate_incoming([&](list<ctrans> &in)
	{ 
		resolve(in,v,bb); 
	}); 
}

void po::resolve_outgoing(bblock_ptr from, rvalue v, bblock_ptr bb)
{
	assert(from && bb);
	from->mutate_outgoing([&](list<ctrans> &out) 
	{ 
		resolve(out,v,bb); 
	}); 
}

// last == true -> pos is last in `up', last == false -> pos is first in `down'
pair<bblock_ptr,bblock_ptr> po::split(bblock_ptr bb, addr_t pos, bool last)
{
	assert(bb);

	bblock_ptr up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::out_iterator j,jend;
	basic_block::in_iterator k,kend;
	function<void(bool,bblock_ptr,ctrans)> append = [](bool in, bblock_ptr bb, ctrans ct)
	{
		if(in)
			bb->mutate_incoming([&](list<ctrans> &l) { l.push_back(ct); });
		else
			bb->mutate_outgoing([&](list<ctrans> &l) { l.push_back(ct); });
	};

	// distribute mnemonics under `up' and `down'
	for_each(bb->mnemonics().begin(),bb->mnemonics().end(),[&](const mnemonic &m)
	{	
		assert(!m.area.includes(pos) || m.area.begin == pos);

		if(!last)
			sw |= m.area.includes(pos);
		
		if(sw)
			down->mutate_mnemonics([&](vector<mnemonic> &ms) { ms.push_back(m); });
		else	
			up->mutate_mnemonics([&](vector<mnemonic> &ms) { ms.push_back(m); });
		
		if(last)
			sw |= m.area.includes(pos);
	});
	assert(sw);

	// move outgoing ctrans to down
	for_each(bb->outgoing().begin(),bb->outgoing().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock() == bb)
		{
			append(false,down,ctrans(ct.condition,up));
			append(true,up,ctrans(ct.condition,up));
		}
		else
		{
			if(ct.bblock.lock())
			{
				append(false,down,ctrans(ct.condition,ct.bblock.lock()));
				ct.bblock.lock()->mutate_incoming([&](list<ctrans> &in)
				{
					in.emplace_back(ctrans(ct.condition,down));
					in.erase(find_if(in.begin(),in.end(),[&](const ctrans &ct)
						{ return ct.bblock.lock() == bb; }));
				});
			}
			else
				append(false,down,ctrans(ct.condition,ct.value));
		}
	});
	
	// move incoming edges to up
	for_each(bb->incoming().begin(),bb->incoming().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock() == bb)
		{
			append(true,up,ctrans(ct.condition,down));
			append(false,down,ctrans(ct.condition,up));
		}
		else
		{
			if(ct.bblock.lock())
			{
				append(true,up,ctrans(ct.condition,ct.bblock.lock()));
				ct.bblock.lock()->mutate_outgoing([&](list<ctrans> &out)
				{
					out.emplace_back(ctrans(ct.condition,up));
					out.erase(find_if(out.begin(),out.end(),[&](const ctrans &ct)
						{ return ct.bblock.lock() == bb; }));
				});
			}
			else
				append(true,up,ctrans(ct.condition,ct.value));
		}
	});

	bb->clear();
	unconditional_jump(up,down);
	return make_pair(up,down);
}

bblock_ptr po::merge(bblock_ptr up, bblock_ptr down)
{
	assert(up && down);
	if(up->area().begin == down->area().end) tie(up,down) = make_pair(down,up);
	assert(up->area().end == down->area().begin);

	bblock_ptr ret(new basic_block());
	auto fn = [&ret](const bblock_ptr &bb, const mnemonic &m) { ret->mutate_mnemonics([&](vector<mnemonic> &ms)
		{ ms.push_back(m); }); };

	for_each(up->mnemonics().begin(),up->mnemonics().end(),bind(fn,up,placeholders::_1));
	for_each(down->mnemonics().begin(),down->mnemonics().end(),bind(fn,down,placeholders::_1));

	for_each(up->incoming().begin(),up->incoming().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock())
			replace_outgoing(ct.bblock.lock(),up,ret);
		ret->mutate_incoming([&](list<ctrans> &in) { in.emplace_back(ct); });
	});
			
	for_each(down->outgoing().begin(),down->outgoing().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock())
			replace_incoming(ct.bblock.lock(),down,ret);
		ret->mutate_outgoing([&](list<ctrans> &out) { out.emplace_back(ct); });
	});
	
	up->clear();
	down->clear();
	return ret;
}

void po::replace(list<ctrans> &lst, bblock_ptr from, bblock_ptr to)
{
	assert(from && to);

	auto i = lst.begin();
	while(i != lst.end())
	{
		ctrans ct = *i;
		if(ct.bblock.lock() == from)
			i = lst.insert(lst.erase(i),ctrans(ct.condition,to));
		++i;
	}
}

void po::resolve(list<ctrans> &lst, rvalue v, bblock_ptr bb)
{
	assert(bb);

	auto i = lst.begin();
	while(i != lst.end())
	{
		ctrans ct = *i;
		if(ct.value == v)
			i = lst.insert(lst.erase(i),ctrans(ct.condition,bb));
		++i;
	}
}

void po::conditional_jump(const ctrans &from, const ctrans &to)
{
	if(from.bblock.lock())
		from.bblock.lock()->mutate_outgoing([&](list<ctrans> &out) { out.emplace_back(to); });
	if(to.bblock.lock())
		to.bblock.lock()->mutate_incoming([&](list<ctrans> &in) { in.emplace_back(from); });
}
