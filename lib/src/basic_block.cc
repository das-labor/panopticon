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

ctrans::ctrans(struct guard g, rvalue v) : condition(g), value(v), bblock() {}
ctrans::ctrans(struct guard g, bblock_loc b) : condition(g), value(), bblock(b) {}

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
			_area = *_area & m.area;
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
