#ifndef PROCEDURE_HH
#define PROCEDURE_HH

#include <memory>
#include <set>
#include <map>
#include <list>
#include <mutex>
#include <cstring>

#include <basic_block.hh>
#include <mnemonic.hh>
#include <disassembler.hh>

namespace po
{
	typedef std::shared_ptr<struct domtree> dtree_ptr;
	typedef std::shared_ptr<class procedure> proc_ptr;
	typedef std::shared_ptr<const class procedure> proc_cptr;
	typedef std::weak_ptr<class procedure> proc_wptr;
	typedef std::weak_ptr<const class procedure> proc_cwptr;

	bool operator<(const proc_wptr &a, const proc_wptr &b);
	bool operator<(const proc_cwptr &a, const proc_cwptr &b);

	void call(proc_ptr from, proc_ptr to);
	void execute(proc_cptr proc,std::function<void(const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)> f);
	bblock_ptr find_bblock(proc_ptr proc, addr_t a);
	std::string graphviz(proc_ptr proc);

	struct domtree
	{
		domtree(bblock_ptr b);

		dtree_ptr intermediate;			// e.g. parent
		std::set<dtree_ptr> successors;
		std::set<dtree_ptr> frontiers;
		
		bblock_wptr basic_block;
	};

	class procedure
	{
	public:
		procedure(void);
		template<typename FW> procedure(FW begin, FW end) : procedure() { copy(begin,end,inserter(basic_blocks,basic_blocks.begin())); }

		void rev_postorder(std::function<void(bblock_ptr bb)> fn) const;

		// public fields
		std::string name;
		bblock_ptr entry;
		std::set<bblock_ptr> basic_blocks;
		std::mutex mutex;
		
		// modified via call()
		std::set<proc_wptr> callers;	// procedures calling this procedure
		std::set<proc_wptr> callees;	// procedures called by this procedure

		template<typename Tag>
		static proc_ptr disassemble(const proc_ptr proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, addr_t start);	
	};
	
	template<typename Tag>
	proc_ptr procedure::disassemble(const proc_ptr proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, addr_t start)
	{
		assert(start != naddr);

		std::set<addr_t> todo;
		std::map<addr_t,mnemonic> mnemonics;
		std::multimap<addr_t,std::pair<addr_t,guard>> source, destination;
		proc_ptr ret(new procedure());

		// copy exsisting mnemonics and jumps into tables. TODO: cache tables in proc
		if(proc)
		{
			for(const bblock_ptr bb: proc->basic_blocks)
			{
				assert(bb);

				for(const mnemonic &m: bb->mnemonics())
				{
					assert(m.area.size());
					mnemonics.insert(std::make_pair(m.area.last(),m));
				}

				for(const ctrans &ct: bb->outgoing())
				{
					if(ct.bblock.lock())
					{
						source.insert(std::make_pair(bb->area().last(),std::make_pair(ct.bblock.lock()->area().begin,ct.guard)));
						destination.insert(std::make_pair(ct.bblock.lock()->area().begin,std::make_pair(bb->area().last(),ct.guard)));
					}
					else if(ct.value.is_constant())
					{
						source.insert(std::make_pair(bb->area().last(),std::make_pair(ct.value.to_constant().content(),ct.guard)));
						destination.insert(std::make_pair(ct.value.to_constant().content(),std::make_pair(bb->area().last(),ct.guard)));
					}
				}	
			}
		}

		todo.insert(start);

		while(!todo.empty())
		{
			addr_t cur_addr = *todo.begin();
			sem_state<Tag> state(cur_addr);
			bool ret;
			typename rule<Tag>::tokiter i = tokens.begin();
			auto j = mnemonics.lower_bound(cur_addr);
		
			todo.erase(todo.begin());

			if(cur_addr >= tokens.size())
			{
				std::cout << "boundary err" << std::endl;
				continue;
			}

			if(j == mnemonics.end() || j->second.area.begin != cur_addr)
			{
				advance(i,cur_addr);
				tie(ret,i) = main.match(i,tokens.end(),state);
			
				if(ret)
				{
					addr_t last = 0;
					
					for(const mnemonic &m: state.mnemonics)
					{
						last = std::max(last,m.area.last());
						assert(mnemonics.insert(std::make_pair(m.area.begin,m)).second);
					}
							
					for(const std::pair<rvalue,guard> &p: state.jumps)
					{
						if(p.first.is_constant())
						{
							addr_t target = p.first.to_constant().content();

							source.insert(std::make_pair(last,std::make_pair(target,p.second)));
							destination.insert(std::make_pair(target,std::make_pair(last,p.second)));
							todo.insert(target);
						}
						else
						{
							source.insert(std::make_pair(last,std::make_pair(naddr,p.second)));
						}
					}
				}
				else
				{
					std::cerr << "Failed to match anything at " << cur_addr << std::endl;
				}
			}
			else
			{
				std::cerr << "Overlapping mnemonics at " << cur_addr << " with \"" << "[" << j->second.area << "] " << j->second << "\"" << std::endl;
			}
		}
		
		auto cur_mne = mnemonics.begin(), first_mne = cur_mne;
		std::map<addr_t,bblock_ptr> bblocks;
		std::function<void(std::map<addr_t,mnemonic>::iterator,std::map<addr_t,mnemonic>::iterator)> make_bblock;
		make_bblock = [&](std::map<addr_t,mnemonic>::iterator begin,std::map<addr_t,mnemonic>::iterator end)
		{
			bblock_ptr bb(new basic_block());

			// copy mnemonics
			bb->mutate_mnemonics([&](std::vector<mnemonic> &ms) 
			{
				std::for_each(begin,end,[&](const std::pair<addr_t,mnemonic> &p)
				{ 
					ms.push_back(p.second); 
				}); 
			});

			ret->basic_blocks.insert(bb);
			assert(bblocks.insert(std::make_pair(bb->area().last(),bb)).second);
		};

		while(cur_mne != mnemonics.end())
		{
			auto next_mne = std::next(cur_mne);
			const mnemonic &mne = cur_mne->second;
			addr_t div = mne.area.end;
			auto sources = source.equal_range(mne.area.last());
			auto destinations = destination.equal_range(div);
			
			if(next_mne != mnemonics.end() && mne.area.size())
			{
				bool new_bb;

				// if next mnemonic is adjacent
				new_bb = next_mne->first != div;

				// or any following jumps aren't to adjacent mnemonics
				new_bb |= std::any_of(sources.first,sources.second,[&](const std::pair<addr_t,std::pair<addr_t,guard>> &p) 
				{ 
					return p.second.first != div; 
				});
				
				// or any jumps pointing to the next that aren't from here
				new_bb |= std::any_of(destinations.first,destinations.second,[&](const std::pair<addr_t,std::pair<addr_t,guard>> &p) 
				{ 
					return p.second.first != mne.area.last();
				});
			
				// construct a new basic block
				if(new_bb)
				{
					make_bblock(first_mne,next_mne);
					
					first_mne = next_mne;
				}
				else
				{
					while(sources.first != sources.second)
						source.erase(sources.first++);
					while(destinations.first != destinations.second)
						destination.erase(destinations.first++);
				}
			}

			cur_mne = next_mne;
		}
	
		// last bblock
		make_bblock(first_mne,cur_mne);
				
		// connect basic blocks
		for(const std::pair<addr_t,std::pair<addr_t,guard>> &p: source)
		{
			if(p.second.first != naddr)
			{
				auto from = bblocks.find(p.first), to = bblocks.lower_bound(p.second.first);

				assert(from != bblocks.end());
				if(to != bblocks.end() && to->second->area().begin == p.second.first)
					conditional_jump(from->second,to->second,p.second.second);
				else
					conditional_jump(from->second,po::constant(p.second.first,flsll(p.second.first)),p.second.second);
			}
		}

		if(ret->basic_blocks.size() == 1 && (*ret->basic_blocks.begin())->mnemonics().empty())
			ret->basic_blocks.clear();
		
		// entry may have been split
		if((proc && proc->entry) || ret->basic_blocks.size())
		{
			addr_t entry = proc && proc->entry ? proc->entry->area().begin : start;
			auto i = bblocks.lower_bound(entry);

			if(i != bblocks.end() && i->second->area().begin == entry)
				ret->entry = i->second;
			else
				ret->entry = bblocks.lower_bound(start)->second;
		}
		else
		{
			ret->entry = bblock_ptr(0);
		}

		if(proc && proc->name.size() > 0)
			ret->name = proc->name;
		else if(ret->basic_blocks.size() > 0)
			ret->name = "proc_" + std::to_string(ret->entry->area().begin);
		else
			ret->name = "proc_(empty)";

		return ret;
	}
}

#endif
