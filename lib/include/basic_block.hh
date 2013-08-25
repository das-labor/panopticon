#ifndef BASIC_BLOCK_HH
#define BASIC_BLOCK_HH

#include <memory>
#include <list>
#include <map>
#include <cassert>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Weffc++"
#include <boost/iterator/iterator_facade.hpp>
#include <boost/iterator/filter_iterator.hpp>
#pragma GCC diagnostic pop

#include <marshal.hh>

/**
 * @file
 * @brief Basic block structure and related classes
 *
 * A basic block is a sequence of mnemonics that are executed as a sequence.
 * The code of a function is partitioned into basic blocks. All its mnemonics are in one
 * continious block in memory and no mnemonic but the last is a jump instruction.
 *
 * Basic blocks have incoming and outgoing control transfers that model jump instructions.
 * Each control transfer has a source and a destination value and a guard. The guard is a
 * collection of conditions that are all true if this control transfer is taken. Both guards
 * and source/destination values can be symbolic i.e. variables or memory references. These
 * need to be evaluated in the context of the source basic block.
 */

namespace po
{
	class basic_block;
	template<class T> class bblock_iterator;
	template<class T> class bblock_citerator;
	typedef std::shared_ptr<class basic_block> bblock_ptr;
	typedef std::shared_ptr<const class basic_block> bblock_cptr;
	typedef std::weak_ptr<class basic_block> bblock_wptr;
	typedef std::weak_ptr<const class basic_block> bblock_cwptr;

	typedef std::shared_ptr<const class procedure> proc_cptr;
}

#include <mnemonic.hh>

namespace po
{
	/**
	 * @brief A logical relation.
	 */
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

		/**
		 * Constructs a new relation with left side @ref a,
		 * right side @ref b and relation code @ref c.
		 */
		relation(rvalue a, Relcode c, rvalue b);

		Relcode relcode;
		rvalue operand1;	///< Left side
		rvalue operand2;	///< Right side
	};

	/**
	 * @brief Condition of a control transfer.
	 *
	 * A guard instance is a conjunction of relations that is
	 * true if a its associated control transfer is taken.
	 *
	 * @note A empty guard i.e. one w/o any relations is always true.
	 */
	struct guard
	{
		/// Constructs a guard that is always true
		guard(void);
		guard(const guard &g);
		guard(guard &&g);
		guard(const std::list<relation> &rels);
		guard(std::list<relation> &&rels);
		guard(rvalue a, relation::Relcode, rvalue b);

		guard &operator=(const guard &g);
		guard &operator=(guard &&g);

		/**
		 * @returns the negation of this guard
		 * @todo Only works with guards that have a single relation
		 */
		guard negation(void) const;
		
		std::list<relation> relations; ///< Conjunction of relations
	};

	std::string symbolic(relation::Relcode r);

	/// @returns The relation @ref r as UTF-8 string.
	std::string pretty(relation::Relcode r);
	std::ostream& operator<<(std::ostream &os, const guard &g);
	odotstream &operator<<(odotstream &os, const guard &g);
	
	/**
	 * @brief A jump between two basic blocks.
	 *
	 * A control transfer is a jump from one address to another with
	 * an optional condition. The address can be any value (constant, 
	 * memory, variables,...).
	 *
	 * If the target of the jump is known and has been disassembled
	 * the instance includes a pointer to the basic blocks that spans
	 * this address.
	 */
	struct ctrans
	{
		/// Jump to address @ref v under condition @ref g
		ctrans(guard g, rvalue v);

		/// Jump to basic block @ref b under condition @ref g
		ctrans(guard g, bblock_ptr b);
		
		guard condition;
		rvalue value;
		bblock_wptr bblock;
	};

	oturtlestream& operator<<(oturtlestream &os, const ctrans &ct);
	std::string unique_name(const ctrans &ct);

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Weffc++"

	/// @internal
	template<>
	class bblock_iterator<std::list<ctrans>> : public boost::iterator_facade<
				bblock_iterator<std::list<ctrans>>,
				bblock_ptr,
				boost::forward_traversal_tag,
				bblock_ptr>
	{
	public:
		bblock_iterator(void) : adaptee() {};
		explicit bblock_iterator(std::list<ctrans>::iterator i, std::list<ctrans>::iterator iend)
		: adaptee([](const ctrans &ct) -> bool { return ct.bblock.lock().get() != nullptr; },i,iend) {};
		bblock_iterator &increment(void) { ++adaptee; return *this; };
		bblock_iterator &decrement(void) { return *this; };

		bblock_ptr dereference(void) const { return adaptee->bblock.lock(); }
		bool equal(const bblock_iterator &a) const { return adaptee == a.adaptee; }

	private:
		boost::filter_iterator<std::function<bool(ctrans &ct)>,std::list<ctrans>::iterator> adaptee;
	};

	/// @internal
	template<>
	class bblock_citerator<std::list<ctrans>> : public boost::iterator_facade<
				bblock_citerator<std::list<ctrans>>,
				bblock_cptr,
				boost::forward_traversal_tag,
				bblock_cptr>
	{
	public:
		bblock_citerator(void) : adaptee() {};
		explicit bblock_citerator(std::list<ctrans>::const_iterator i, std::list<ctrans>::const_iterator iend)
		: adaptee([](const ctrans &ct) -> bool { return ct.bblock.lock().get() != nullptr; },i,iend) {};
		bblock_citerator &increment(void) { ++adaptee; return *this; };
		bblock_citerator &decrement(void) { return *this; };

		bblock_cptr dereference(void) const { return adaptee->bblock.lock(); }
		bool equal(const bblock_citerator &a) const { return adaptee == a.adaptee; }

	private:
		boost::filter_iterator< std::function<bool(const ctrans &ct)>,std::list<ctrans>::const_iterator> adaptee;
	};

#pragma GCC diagnostic pop

	/**
	 * @brief Sequence of mnemonics with no jumps inbetween.
	 *
	 * A basic block is a span of mnemonics that are executed in sequence.
	 */
	class basic_block
	{
	public:
		static bblock_ptr unmarshal(const rdf::node &n, proc_cptr proc, const rdf::storage &store);

		basic_block(void);

		// iterators
		typedef bblock_iterator< std::list<ctrans>> pred_iterator;
		typedef bblock_iterator< std::list<ctrans>> succ_iterator;
		typedef bblock_citerator< std::list<ctrans>> pred_citerator;
		typedef bblock_citerator< std::list<ctrans>> succ_citerator;
		typedef std::list<ctrans>::iterator out_iterator;
		typedef std::list<ctrans>::iterator in_iterator;
		
		/// @returns a pair of iterators pointing to the beginning and the end of the list of predecessor basic blocks
		std::pair<pred_citerator,pred_citerator> predecessors(void) const;
		/// @returns a pair of iterators pointing to the beginning and the end of the list of successor basic blocks
		std::pair<succ_citerator,succ_citerator> successors(void) const;
		/// @returns a pair of iterators pointing to the beginning and the end of the list of predecessor basic blocks
		std::pair<pred_iterator,pred_iterator> predecessors(void);
		/// @returns a pair of iterators pointing to the beginning and the end of the list of successor basic blocks
		std::pair<succ_iterator,succ_iterator> successors(void);
		
		/// @returns mnemonics this basic block includes
		const std::vector<mnemonic> &mnemonics(void) const;
		
		/// @returns all control transfer instances with this basic block as source 
		const std::list<ctrans> &incoming(void) const;
		/// @returns all control transfer instances with this basic block as destination
		const std::list<ctrans> &outgoing(void) const;
		
		// mutates internal lists (mnemonics and incoming/outgoing) and updates m_area, checks invariants
		//
		/**
		 * Calls a given function, supplying a mutable reference to the list of mnemonics.
		 * The function sanity checks the list after the call and update @ref area.
		 */
		void mutate_mnemonics(std::function<void(std::vector<mnemonic>&)> fn);
		
		/**
		 * Calls a given function, supplying a mutable reference to the list of incoming control transfers.
		 * The function sanity checks the list after the call.
		 */
		void mutate_incoming(std::function<void(std::list<ctrans>&)> fn);
		
		/**
		 * Calls a given function, supplying a mutable reference to the list of outgoing control transfers.
		 * The function sanity checks the list after the call.
		 */
		void mutate_outgoing(std::function<void(std::list<ctrans>&)> fn);

		/// @returns the address range this basic block spans
		const range<addr_t> &area(void) const;

		/// Deletes all control transfers and mnemonics inside
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
	oturtlestream &operator<<(oturtlestream &os, const basic_block &bb);
	std::string unique_name(const basic_block &bb);

	/// Iterates all mnemoics in @ref bb, calling @ref f for each instruction.
	void execute(bblock_cptr bb,std::function<void(const lvalue&,instr::Function,const std::vector<rvalue>&)> f);

	/// Iterates all mnemoics in @ref bb, calling @ref f for each instruction.
	void execute2(bblock_cptr bb,std::function<void(const instr&)> f);
	
	/**
	 * Iterates all mnemoics in @ref bb, calling @ref f for each instruction.
	 * @note Allows modification of the instructions
	 */
	void rewrite(bblock_ptr bb,std::function<void(lvalue&,instr::Function,std::vector<rvalue>&)> f);

	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(bblock_ptr from, bblock_ptr to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(rvalue from, bblock_ptr to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(bblock_ptr from, rvalue to, guard g);

	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(bblock_ptr from, bblock_ptr to);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(rvalue from, bblock_ptr to);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(bblock_ptr from, rvalue to);

	/// Replaces the source basic block @ref oldbb with @ref newbb in all outgoing control transfers of @ref to.
	void replace_incoming(bblock_ptr to, bblock_ptr oldbb, bblock_ptr newbb);
	/// Replaces the destination basic block @ref oldbb with @ref newbb in all outgoing control transfers of @ref from.
	void replace_outgoing(bblock_ptr from, bblock_ptr oldbb, bblock_ptr newbb);
	/// Sets the source basic block to @ref bb in every incoming control transfer of @ref to that has a source value equal to @ref v
	void resolve_incoming(bblock_ptr to, rvalue v, bblock_ptr bb);
	/// Sets the destination basic block to @ref bb in every outgoing control transfer of @ref from that has a destination value equal to @ref v
	void resolve_outgoing(bblock_ptr from, rvalue v, bblock_ptr bb);

	/**
	 * Splits the @ref bb into two. If @ref last is true all mnemonics in @ref bb 
	 * up to @ref pos are includes into the first. Otherwise the mnemonic at @ref pos
	 * is the first in the second basic block.
	 * @returns Pair of basic blocks.
	 */
	std::pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, addr_t pos, bool last);

	/// Merges two adjacent basic blocks into one.
	bblock_ptr merge(bblock_ptr up, bblock_ptr down);

	/// @internal
	void replace(std::list<ctrans> &lst, bblock_ptr from, bblock_ptr to);
	/// @internal
	void resolve(std::list<ctrans> &lst, rvalue v, bblock_ptr bb);
	/// @internal
	void conditional_jump(const ctrans &from, const ctrans &to);
}

#endif
