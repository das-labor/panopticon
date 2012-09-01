#ifndef BASIC_BLOCK_HH
#define BASIC_BLOCK_HH

#include <memory>
#include <tuple>
#include <list>
#include <cassert>
#include <boost/iterator/iterator_facade.hpp>

using namespace std;
using namespace boost;

class basic_block;
template<class T> class bblock_iterator;
typedef shared_ptr<struct basic_block> bblock_ptr;
typedef shared_ptr<struct guard> guard_ptr;
typedef shared_ptr<const struct guard> guard_cptr;

#include "mnemonic.hh"

struct relation
{
	enum Relcode
	{
		ULeq,
		SLeq,
		UGeq,
		SGeq,
		ULess,
		SLess,
		UGrtr,
		SGrtr,
		Eq,
		Neq,
	};

	relation(valproxy a, Relcode c, valproxy b);

	Relcode relcode;
	value_ptr operand1;
	value_ptr operand2;
};

struct guard
{
	guard(void); // true
	guard(vector<relation> rels);
	guard(valproxy a, relation::Relcode, valproxy b);
	guard_ptr negation(void) const;
	string inspect(void) const;

	vector<relation> relations;
};

template<>
class bblock_iterator<list<pair<guard_cptr,bblock_ptr>>> : public iterator_facade<
			bblock_iterator<list<pair<guard_cptr,bblock_ptr>>>,
			bblock_ptr,
			bidirectional_traversal_tag,
			bblock_ptr>
{
public:
	bblock_iterator(void) {};
	explicit bblock_iterator(list<pair<guard_cptr,bblock_ptr>>::iterator i) : adaptee(i) {};
	bblock_iterator &increment(void) { ++adaptee; return *this; };
	bblock_iterator &decrement(void) { --adaptee; return *this; };

	bblock_ptr dereference(void) const { return adaptee->second; }
	bool equal(const bblock_iterator &a) const { return adaptee == a.adaptee; }

private:
	list<pair<guard_cptr,bblock_ptr>>::iterator adaptee;
};
/*
template<>
class bblock_iterator<list<tuple<guard_cptr,bblock_ptr,bblock_ptr>>> : public iterator_facade<
			bblock_iterator,
			bblock_ptr,
			bidirectional_traversal_tag,
			bblock_ptr>
{
public:
	bblock_iterator(void) {};
	explicit bblock_iterator(list<tuple<guard_cptr,bblock_ptr,bblock_ptr>>::iterator &i) : adaptee(i), first(true) {};
	bblock_iterator &increment(void) 
	{ 
		first = !first;
		if(first)
			++adaptee; 
		return *this;
	};

	bblock_iterator &decrement(void)
	{ 
		first = !first;
		if(first)
			--adaptee; 
		return *this;
	};

	bblock_ptr dereference(void) const { return (first ? get<1>(*i) : get<2>(*i)); }
	bool equal(const bblock_iterator &a) const { return adaptee == a.adaptee && first == a.first; }

private:
	bool first;
	list<tuple<guard_cptr,bblock_ptr,bblock_ptr>>::iterator adaptee;
};*/

class basic_block
{
public:
	typedef list<mne_cptr>::iterator iterator;
	typedef bblock_iterator<list<pair<guard_cptr,bblock_ptr>>> pred_iterator;
	typedef bblock_iterator<list<pair<guard_cptr,bblock_ptr>>> succ_iterator;
	typedef list<pair<guard_cptr,bblock_ptr>>::iterator out_iterator;
	typedef list<pair<guard_cptr,bblock_ptr>>::iterator in_iterator;

	pair<pred_iterator,pred_iterator> predecessors(void);
	pair<succ_iterator,succ_iterator> successors(void);
	//pair<iter,iter> instructions(void) const;
	pair<iterator,iterator> mnemonics(void);
	pair<out_iterator,out_iterator> outgoing(void);
	pair<in_iterator,in_iterator> incoming(void);
	
	void append_mnemonic(mne_cptr m);
	void insert_incoming(guard_cptr, bblock_ptr m);
	void insert_outgoing(guard_cptr, bblock_ptr m);
	
	void remove_mnemonic(mne_cptr m);
	void remove_incoming(bblock_ptr m);
	void remove_outgoing(bblock_ptr m);
	
	void replace_incoming(bblock_ptr from, bblock_ptr to);
	void replace_outgoing(bblock_ptr from, bblock_ptr to);

	const area &addresses(void) const;
	void clear(void);

protected:
	area m_addresses;
	list<mne_cptr> m_mnemonics;
	
	list<pair<guard_cptr,bblock_ptr>> m_incoming;
	list<pair<guard_cptr,bblock_ptr>> m_outgoing;
//	list<tuple<guard_cptr,bblock_ptr,bblock_ptr>> m_outgoing;
	
	// dflow::dominance
	//dtree_ptr dominance;	// node in the dominance tree

	// dflow::liveness
	//set<name> uevar;		// up exposed variables
	//set<name> varkill;	// overwritten vars
	//set<name> liveout;
	
//protected:
	//virtual void accept_instr(instr_ptr i);
};

pair<bblock_ptr,bblock_ptr> conditional(bblock_ptr bb, guard_ptr g, bblock_ptr trueb = bblock_ptr(), bblock_ptr falseb = bblock_ptr() );
void unconditional(bblock_ptr bb_from, bblock_ptr bb_to);

pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last);

#endif
