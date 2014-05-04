#include <algorithm>
#include <functional>
#include <set>
#include <iostream>
#include <cassert>

#include <panopticon/basic_block.hh>

using namespace po;
using namespace std;

relation::relation(rvalue a, Relcode c, rvalue b) : relcode(c), operand1(a), operand2(b) {}

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

template<>
rdf::statements po::marshal(const basic_block* bb, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::ns_local(to_string(u));

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("BasicBlock"));

	for(const mnemonic& m: bb->mnemonics())
	{
		uuid uu;
		rdf::statements mn = marshal<mnemonic>(&m,uu);

		std::move(mn.begin(),mn.end(),back_inserter(ret));
		ret.emplace_back(root,rdf::ns_po("include"),rdf::ns_local(to_string(uu)));
	}

	return ret;
}

template<>
basic_block* po::unmarshal(const uuid& u, const rdf::storage& store)
{
	rdf::node node(rdf::ns_local(to_string(u)));
	rdf::statements mnes = store.find(node,"include"_po);

	assert(mnes.size());
	basic_block *ret = new basic_block();
	std::list<mnemonic> mne_lst;

	// mnemoics
	for(const rdf::statement &st: mnes)
		mne_lst.emplace_back(*unmarshal<mnemonic>(boost::uuids::string_generator()(st.object.as_iri().substr(st.object.as_iri().size()-36)),store));

	mne_lst.sort([](const mnemonic &a, const mnemonic &b)
		{ return a.area.lower() < b.area.lower(); });

	std::move(mne_lst.begin(),mne_lst.end(),back_inserter(ret->mnemonics()));

	return ret;
}

basic_block::basic_block(void) : _area(boost::none), _mnemonics() {}

bound basic_block::area(void) const
{
	if(!_area)
	{
		_area = bound();

		for(auto m: _mnemonics)
			_area = boost::icl::hull(*_area,m.area);
	}

	return *_area;
}

bool basic_block::operator==(const basic_block& b) const
{
	return _mnemonics == b._mnemonics;
}

void po::execute(bblock_loc bb,function<void(const instr&)> f)
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

void po::execute(bblock_loc bb,function<void(const lvalue &left, instr::Function fn, const vector<rvalue> &right)> f)
{
	execute(bb,[&](const instr &i)
	{
		f(i.left,i.function,i.right);
	});
}

template<>
po::guard* po::unmarshal(const po::uuid& u, const po::rdf::storage& store)
{
	rdf::node node = rdf::ns_local(to_string(u)),
									 rel_head = store.first(node,rdf::ns_po("relations")).object;
	rdf::nodes rels_n = read_list(rel_head,store);

	list<relation> rels;

	transform(rels_n.begin(),rels_n.end(),back_inserter(rels),[&](const rdf::node& n)
	{
		rdf::node op1_n = store.first(n,rdf::ns_po("operand1")).object,
							op2_n = store.first(n,rdf::ns_po("operand2")).object,
							code = store.first(n,rdf::ns_po("relcode")).object;

		rvalue op1 = *unmarshal<rvalue>(op1_n.as_iri().substr(op1_n.as_iri().size()-36),store);
		rvalue op2 = *unmarshal<rvalue>(op2_n.as_iri().substr(op2_n.as_iri().size()-36),store);

		if(code == rdf::ns_po("u-less-equal"))
			return relation{op1,relation::ULeq,op2};
		else if(code == rdf::ns_po("s-less-equal"))
			return relation{op1,relation::SLeq,op2};
		else if(code == rdf::ns_po("u-greater-equal"))
			return relation{op1,relation::UGeq,op2};
		else if(code == rdf::ns_po("s-greater-equal"))
			return relation{op1,relation::SGeq,op2};
		else if(code == rdf::ns_po("u-less"))
			return relation{op1,relation::ULess,op2};
		else if(code == rdf::ns_po("s-less"))
			return relation{op1,relation::SLess,op2};
		else if(code == rdf::ns_po("u-greater"))
			return relation{op1,relation::UGrtr,op2};
		else if(code == rdf::ns_po("s-greater"))
			return relation{op1,relation::SGrtr,op2};
		else if(code == rdf::ns_po("equal"))
			return relation{op1,relation::Eq,op2};
		else if(code == rdf::ns_po("not-equal"))
			return relation{op1,relation::Neq,op2};
		else
			assert(false);
	});

	return new guard{rels};
}

template<>
rdf::statements po::marshal(const guard* g, const uuid& uu)
{
	rdf::statements ret;
	rdf::nodes rels;
	rdf::node node = rdf::ns_local(to_string(uu));
	boost::uuids::name_generator ng(uu);
	unsigned int cnt = 0;

	for(auto rel: g->relations)
	{
		uuid uu = ng(to_string(cnt++));
		uuid u1 = ng(to_string(cnt++));
		uuid u2 = ng(to_string(cnt++));
		rdf::node rn = rdf::ns_local(to_string(uu));

		rdf::statements st1 = marshal(&rel.operand1,u1);
		rdf::statements st2 = marshal(&rel.operand2,u2);

		std::move(st1.begin(),st1.end(),back_inserter(ret));
		std::move(st2.begin(),st2.end(),back_inserter(ret));

		ret.emplace_back(rn,rdf::ns_po("operand1"),rdf::ns_local(to_string(u1)));
		ret.emplace_back(rn,rdf::ns_po("operand2"),rdf::ns_local(to_string(u2)));
		rels.push_back(rn);

		switch(rel.relcode)
		{
			case relation::ULeq:	ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("u-less-equal")); break;
			case relation::SLeq: 	ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("s-less-equal")); break;
			case relation::UGeq: 	ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("u-greater-equal")); break;
			case relation::SGeq: 	ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("s-greater-equal")); break;
			case relation::ULess: ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("u-less")); break;
			case relation::SLess: ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("s-less")); break;
			case relation::UGrtr: ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("u-greater")); break;
			case relation::SGrtr: ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("s-greater")); break;
			case relation::Eq: 		ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("equal")); break;
			case relation::Neq: 	ret.emplace_back(rn,rdf::ns_po("relcode"),rdf::ns_po("not-equal")); break;
			default: assert(false);
		}
	}

	auto p = rdf::write_list(rels.begin(),rels.end(),to_string(uu));

	std::move(p.second.begin(),p.second.end(),back_inserter(ret));
	ret.emplace_back(node,rdf::ns_po("relations"),p.first);

	return ret;
}
