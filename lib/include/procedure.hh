#ifndef PROCEDURE_HH
#define PROCEDURE_HH

#include <memory>
#include <set>
#include <map>
#include <list>
#include <mutex>

#include <basic_block.hh>
#include <mnemonic.hh>

namespace po
{
	typedef ::std::shared_ptr<struct domtree> dtree_ptr;
	typedef ::std::shared_ptr<struct procedure> proc_ptr;
	typedef ::std::shared_ptr<const struct procedure> proc_cptr;

	void call(proc_ptr from, proc_ptr to);
	void execute(proc_cptr proc,::std::function<void(const lvalue &left, instr::Function fn, const ::std::vector<rvalue> &right)> f);
	void merge(proc_ptr proc, bblock_ptr block);
	void extend(proc_ptr proc, bblock_ptr block);
	bblock_ptr find_bblock(proc_ptr proc, addr_t a);
	std::pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, mnemonic &cur_mne, mnemonic &prev_mne, bblock_ptr prev_bb, guard_ptr g);
	std::pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, mnemonic &cur_mne, bblock_ptr cur_bb, rvalue v, guard_ptr g);
	std::string graphviz(proc_ptr proc);

	struct domtree
	{
		domtree(bblock_ptr b);

		dtree_ptr intermediate;			// e.g. parent
		::std::set<dtree_ptr> successors;
		::std::set<dtree_ptr> frontiers;
		
		bblock_ptr basic_block;
	};

	class procedure
	{
	public:
		procedure(void);
		template<typename FW> procedure(FW begin, FW end) : procedure() { copy(begin,end,inserter(basic_blocks,basic_blocks.begin())); }

		void rev_postorder(::std::function<void(bblock_ptr bb)> fn) const;

		// public fields
		std::string name;
		bblock_ptr entry;
		::std::set<bblock_ptr> basic_blocks;
		std::mutex mutex;
		
		// modified via call()
		::std::set<proc_ptr> callers;	// procedures calling this procedure
		::std::set<proc_ptr> callees;	// procedures called by this procedure
	};
}

#endif
