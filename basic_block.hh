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
class bblock_iterator<list<pair<guard_ptr,bblock_ptr>>> : public iterator_facade<
			bblock_iterator<list<pair<guard_ptr,bblock_ptr>>>,
			bblock_ptr,
			bidirectional_traversal_tag,
			bblock_ptr>
{
public:
	bblock_iterator(void) {};
	explicit bblock_iterator(list<pair<guard_ptr,bblock_ptr>>::iterator i) : adaptee(i) {};
	bblock_iterator &increment(void) { ++adaptee; return *this; };
	bblock_iterator &decrement(void) { --adaptee; return *this; };

	bblock_ptr dereference(void) const { return adaptee->second; }
	bool equal(const bblock_iterator &a) const { return adaptee == a.adaptee; }

private:
	list<pair<guard_ptr,bblock_ptr>>::iterator adaptee;
};

class basic_block
{
public:
	typedef vector<mne_cptr>::iterator mne_iterator;
	typedef vector<instr_ptr>::iterator instr_iterator;
	typedef bblock_iterator<list<pair<guard_ptr,bblock_ptr>>> pred_iterator;
	typedef bblock_iterator<list<pair<guard_ptr,bblock_ptr>>> succ_iterator;
	typedef list<pair<guard_ptr,bblock_ptr>>::iterator out_iterator;
	typedef list<pair<guard_ptr,bblock_ptr>>::iterator in_iterator;
	typedef list<pair<guard_ptr,value_ptr>>::iterator indir_iterator;

	pair<pred_iterator,pred_iterator> predecessors(void);
	pair<succ_iterator,succ_iterator> successors(void);
	pair<out_iterator,out_iterator> outgoing(void);
	pair<in_iterator,in_iterator> incoming(void);
	pair<indir_iterator,indir_iterator> indirect(void);
	
	const vector<mne_cptr> &mnemonics(void) const;
	const vector<instr_ptr> &instructions(void) const;
	
	void append_mnemonic(mne_cptr m);
	void insert_incoming(guard_ptr, bblock_ptr m);
	void insert_outgoing(guard_ptr, bblock_ptr m);
	void insert_indirect(guard_ptr, value_ptr v);
	void prepend_instr(instr_ptr i);
	
	void remove_incoming(bblock_ptr m);
	void remove_outgoing(bblock_ptr m);
	void remove_indirect(value_ptr v);
	
	void replace_incoming(bblock_ptr from, bblock_ptr to);
	void replace_outgoing(bblock_ptr from, bblock_ptr to);

	void resolve_indirect(indir_iterator, addr_t);

	const area &addresses(void) const;
	void clear(void);

protected:
	area m_addresses;
	vector<mne_cptr> m_mnemonics;
	vector<instr_ptr> m_instructions;
	
	list<pair<guard_ptr,bblock_ptr>> m_incoming;
	list<pair<guard_ptr,bblock_ptr>> m_outgoing;
	list<pair<guard_ptr,value_ptr>> m_indirect;
};

void conditional_jump(bblock_ptr from, bblock_ptr to, guard_ptr g);
void unconditional_jump(bblock_ptr bb_from, bblock_ptr bb_to);
void indirect_jump(bblock_ptr from, value_ptr v, guard_ptr g);

pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last);

#endif
