#ifndef BASIC_BLOCK_HH
#define BASIC_BLOCK_HH

#include <memory>
#include <list>
#include <map>
#include <cassert>
#include <boost/iterator/iterator_facade.hpp>
#include <boost/iterator/filter_iterator.hpp>

class basic_block;
template<class T> class bblock_iterator;
typedef std::shared_ptr<struct basic_block> bblock_ptr;
typedef std::shared_ptr<struct guard> guard_ptr;
typedef std::shared_ptr<const struct guard> guard_cptr;

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
	guard(std::vector<relation> rels);
	guard(valproxy a, relation::Relcode, valproxy b);
	guard_ptr negation(void) const;
	string inspect(void) const;

	std::vector<relation> relations;
};

struct ctrans
{
	ctrans(guard_ptr g, value_ptr v);
	ctrans(guard_ptr g, bblock_ptr b);
	
	var_cptr variable(void) const;
	var_ptr variable(void);
	const_cptr constant(void) const;
	const_ptr constant(void);

	guard_ptr guard;
	value_ptr value;
	bblock_ptr bblock;
};

template<>
class bblock_iterator<std::list<ctrans>> : public boost::iterator_facade<
			bblock_iterator<std::list<ctrans>>,
			bblock_ptr,
			boost::forward_traversal_tag,
			bblock_ptr>
{
public:
	bblock_iterator(void) {};
	explicit bblock_iterator(std::list<ctrans>::iterator i, std::list<ctrans>::iterator iend)
	: adaptee([](const ctrans &ct) -> bool { return !!ct.bblock; },i,iend) {};
	bblock_iterator &increment(void) { ++adaptee; return *this; };
	bblock_iterator &decrement(void) { return *this; };

	bblock_ptr dereference(void) const { return adaptee->bblock; }
	bool equal(const bblock_iterator &a) const { return adaptee == a.adaptee; }

private:
	boost::filter_iterator<function<bool(const ctrans &ct)>,std::list<ctrans>::iterator> adaptee;
};

class instr_citerator : public boost::iterator_facade<
			instr_citerator,
			instr_ptr,
			boost::bidirectional_traversal_tag,
			instr_ptr>
{
public:
	instr_citerator(void) {};
	explicit instr_citerator(std::multimap<mne_cptr,instr_ptr>::const_iterator i) : adaptee(i) {};
	instr_citerator &increment(void) { ++adaptee; return *this; };
	instr_citerator &decrement(void) { --adaptee; return *this; };

	instr_ptr dereference(void) const { return adaptee->second; }
	bool equal(const instr_citerator &a) const { return adaptee == a.adaptee; }

private:
	std::multimap<mne_cptr,instr_ptr>::const_iterator adaptee;
};

class basic_block
{
public:
	// TODO: remove
	basic_block(range<addr_t> a);
	basic_block(void);

	// iterators
	typedef bblock_iterator<std::list<ctrans>> pred_iterator;
	typedef bblock_iterator<std::list<ctrans>> succ_iterator;
	typedef std::list<ctrans>::iterator out_iterator;
	typedef std::list<ctrans>::iterator in_iterator;
	
	std::pair<pred_iterator,pred_iterator> predecessors(void);
	std::pair<succ_iterator,succ_iterator> successors(void);
	std::pair<out_iterator,out_iterator> outgoing(void);
	std::pair<in_iterator,in_iterator> incoming(void);
	
	// mnemonic/instruction read
	const std::vector<mne_cptr> &mnemonics(void) const;
	const std::vector<instr_ptr> &instructions(void) const;
	std::pair<instr_citerator,instr_citerator> instructions(mne_cptr m) const;
	
	// mnemonic/instruction write
	template<class T> void append_mnemonic(mne_cptr m, std::pair<T,T> iters)
	{
		assert(m && (m_mnemonics.empty() || m_mnemonics.back()->area.end == m->area.begin));

		m_mnemonics.push_back(m);
		copy(iters.first,iters.second,inserter(m_instructions,m_instructions.end()));
		for_each(iters.first,iters.second,[&](const instr_ptr &ii) { m_map.insert(make_pair(m,ii)); });

		if(m_area.size())
			m_area = range<addr_t>(min(m_area.begin,m->area.begin),max(m_area.end,m->area.end));
		else
			m_area = m->area;
	}
	
	void prepend_instr(instr_ptr i);
	
	// control flow edge read/write
	void insert_incoming(guard_ptr g, bblock_ptr bb);
	void insert_incoming(guard_ptr g, value_ptr v);	
	void insert_outgoing(guard_ptr g, bblock_ptr bb);
	void insert_outgoing(guard_ptr g, value_ptr v);
	void insert_incoming(const ctrans &ct);
	void insert_outgoing(const ctrans &ct);
	void remove_incoming(bblock_ptr m);
	void remove_incoming(value_ptr v);
	void remove_outgoing(bblock_ptr m);
	void remove_outgoing(value_ptr v);
	void replace_incoming(bblock_ptr from, bblock_ptr to);
	void replace_outgoing(bblock_ptr from, bblock_ptr to);
	void resolve_incoming(value_ptr v, bblock_ptr bb);
	void resolve_outgoing(value_ptr v, bblock_ptr bb);

	// misc
	const range<addr_t> &area(void) const;
	void clear(void);

protected:
	range<addr_t> m_area;
	std::vector<mne_cptr> m_mnemonics;
	std::vector<instr_ptr> m_instructions;
	std::multimap<mne_cptr,instr_ptr> m_map;
	
	std::list<ctrans> m_incoming;
	std::list<ctrans> m_outgoing;

private:
	void insert(std::list<ctrans> &lst, const ctrans &ct);
	void remove(std::list<ctrans> &lst, std::function<bool(const ctrans &)> p);
	void replace(std::list<ctrans> &lst, bblock_ptr from, bblock_ptr to);
	void resolve(std::list<ctrans> &lst, value_ptr v, bblock_ptr bb);
};

// adding (un)conditional jumps between basic blocks
void conditional_jump(bblock_ptr from, bblock_ptr to, guard_ptr g);
void conditional_jump(value_ptr from, bblock_ptr to, guard_ptr g);
void conditional_jump(bblock_ptr from, value_ptr to, guard_ptr g);
void conditional_jump(ctrans &from, ctrans &to);

void unconditional_jump(bblock_ptr from, bblock_ptr to);
void unconditional_jump(value_ptr from, bblock_ptr to);
void unconditional_jump(bblock_ptr from, value_ptr to);

std::pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last);
bblock_ptr merge(bblock_ptr up, bblock_ptr down);
#endif
