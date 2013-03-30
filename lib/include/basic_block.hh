#ifndef BASIC_BLOCK_HH
#define BASIC_BLOCK_HH

#include <memory>
#include <list>
#include <map>
#include <cassert>
#include <boost/iterator/iterator_facade.hpp>
#include <boost/iterator/filter_iterator.hpp>

namespace po
{
	class basic_block;
	template<class T> class bblock_iterator;
	template<class T> class bblock_citerator;
	typedef std::shared_ptr<class basic_block> bblock_ptr;
	typedef std::shared_ptr<const class basic_block> bblock_cptr;
	typedef std::weak_ptr<class basic_block> bblock_wptr;
	typedef std::weak_ptr<const class basic_block> bblock_cwptr;
}

#include <mnemonic.hh>

namespace po
{
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

		relation(rvalue a, Relcode c, rvalue b);

		Relcode relcode;
		rvalue operand1;
		rvalue operand2;
	};

	struct guard
	{
		guard(void); // true
		guard(const guard &g);
		guard(guard &&g);
		guard(const std::list<relation> &rels);
		guard(std::list<relation> &&rels);
		guard(rvalue a, relation::Relcode, rvalue b);

		guard &operator=(const guard &g);
		guard &operator=(guard &&g);

		guard negation(void) const;
		
		std::list<relation> relations;
	};

	std::ostream& operator<<(std::ostream &os, const guard &g);
	
	struct ctrans
	{
		ctrans(guard g, rvalue v);
		ctrans(guard g, bblock_ptr b);
		
		guard guard;
		rvalue value;
		bblock_wptr bblock;
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
		: adaptee([](const ctrans &ct) -> bool { return ct.bblock.lock().get() != nullptr; },i,iend) {};
		bblock_iterator &increment(void) { ++adaptee; return *this; };
		bblock_iterator &decrement(void) { return *this; };

		bblock_ptr dereference(void) const { return adaptee->bblock.lock(); }
		bool equal(const bblock_iterator &a) const { return adaptee == a.adaptee; }

	private:
		boost::filter_iterator<std::function<bool(ctrans &ct)>,std::list<ctrans>::iterator> adaptee;
	};

	template<>
	class bblock_citerator<std::list<ctrans>> : public boost::iterator_facade<
				bblock_citerator<std::list<ctrans>>,
				bblock_cptr,
				boost::forward_traversal_tag,
				bblock_cptr>
	{
	public:
		bblock_citerator(void) {};
		explicit bblock_citerator(std::list<ctrans>::const_iterator i, std::list<ctrans>::const_iterator iend)
		: adaptee([](const ctrans &ct) -> bool { return ct.bblock.lock().get() != nullptr; },i,iend) {};
		bblock_citerator &increment(void) { ++adaptee; return *this; };
		bblock_citerator &decrement(void) { return *this; };

		bblock_cptr dereference(void) const { return adaptee->bblock.lock(); }
		bool equal(const bblock_citerator &a) const { return adaptee == a.adaptee; }

	private:
		boost::filter_iterator< std::function<bool(const ctrans &ct)>,std::list<ctrans>::const_iterator> adaptee;
	};

	class basic_block
	{
	public:
		basic_block(void);

		// iterators
		typedef bblock_iterator< std::list<ctrans>> pred_iterator;
		typedef bblock_iterator< std::list<ctrans>> succ_iterator;
		typedef bblock_citerator< std::list<ctrans>> pred_citerator;
		typedef bblock_citerator< std::list<ctrans>> succ_citerator;
		typedef std::list<ctrans>::iterator out_iterator;
		typedef std::list<ctrans>::iterator in_iterator;
		
		std::pair<pred_citerator,pred_citerator> predecessors(void) const;
		std::pair<succ_citerator,succ_citerator> successors(void) const;
		std::pair<pred_iterator,pred_iterator> predecessors(void);
		std::pair<succ_iterator,succ_iterator> successors(void);
		
		// mnemonic and instructions read
		const std::vector<mnemonic> &mnemonics(void) const;
		
		// control flow read
		const std::list<ctrans> &incoming(void) const;
		const std::list<ctrans> &outgoing(void) const;
		
		// mutates internal lists (mnemonics and incoming/outgoing) and updates m_area, checks invariants
		void mutate_mnemonics(std::function<void(std::vector<mnemonic>&)> fn);
		void mutate_incoming(std::function<void(std::list<ctrans>&)> fn);
		void mutate_outgoing(std::function<void(std::list<ctrans>&)> fn);

		// misc
		const range<addr_t> &area(void) const;
		void clear(void);

	private:
		void mutate_controlflow(std::list<ctrans> &lst, std::function<void(std::list<ctrans>&)> fn);
		
		range<addr_t> m_area;
		std::vector<mnemonic> m_mnemonics;
		
		std::list<ctrans> m_incoming;
		std::list<ctrans> m_outgoing;
	};
	
	bool operator<(const bblock_wptr &a, const bblock_wptr &b);
	bool operator<(const bblock_cwptr &a, const bblock_cwptr &b);

	odotstream &operator<<(odotstream &os, const basic_block &bb);
	std::string unique_name(const basic_block &bb);

	// reading
	void execute(bblock_cptr bb,std::function<void(const lvalue&,instr::Function,const std::vector<rvalue>&)> f);
	void execute2(bblock_cptr bb,std::function<void(const instr&)> f);
	void rewrite(bblock_ptr bb,std::function<void(lvalue&,instr::Function,std::vector<rvalue>&)> f);

	// adding (un)conditional jumps between basic blocks
	void conditional_jump(bblock_ptr from, bblock_ptr to, guard g);
	void conditional_jump(rvalue from, bblock_ptr to, guard g);
	void conditional_jump(bblock_ptr from, rvalue to, guard g);

	void unconditional_jump(bblock_ptr from, bblock_ptr to);
	void unconditional_jump(rvalue from, bblock_ptr to);
	void unconditional_jump(bblock_ptr from, rvalue to);

	// modifying jumps
	void replace_incoming(bblock_ptr to, bblock_ptr oldbb, bblock_ptr newbb);
	void replace_outgoing(bblock_ptr from, bblock_ptr oldbb, bblock_ptr newbb);
	void resolve_incoming(bblock_ptr to, rvalue v, bblock_ptr bb);
	void resolve_outgoing(bblock_ptr from, rvalue v, bblock_ptr bb);

	std::pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last);
	bblock_ptr merge(bblock_ptr up, bblock_ptr down);

	// internal
	void replace(std::list<ctrans> &lst, bblock_ptr from, bblock_ptr to);
	void resolve(std::list<ctrans> &lst, rvalue v, bblock_ptr bb);
	void conditional_jump(const ctrans &from, const ctrans &to);
}

#endif
