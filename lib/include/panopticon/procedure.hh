#include <memory>
#include <set>
#include <map>
#include <list>
#include <mutex>
#include <cstring>

#include <panopticon/basic_block.hh>
#include <panopticon/mnemonic.hh>
#include <panopticon/disassembler.hh>
#include <panopticon/marshal.hh>

#pragma once

namespace po
{
	typedef std::shared_ptr<class procedure> proc_ptr;
	typedef std::weak_ptr<class procedure> proc_wptr;

	typedef std::shared_ptr<struct program> prog_ptr;

	bool operator<(const proc_wptr &a, const proc_wptr &b);
	bool operator<(const proc_cwptr &a, const proc_cwptr &b);

	/// Insert call graph edge from @arg from to @to
	void call(proc_ptr from, proc_ptr to);

	/// Run @arg f on all IL statements. Basic blocks a traversed in undefined order.
	void execute(proc_cptr proc,std::function<void(const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)> f);

	/// Returns basic block occuping address @arg a
	bblock_ptr find_bblock(proc_ptr proc, offset_t a);

	/**
	 * @brief Function
	 *
	 * Panopticon groups basic blocks into procedures. Each basic
	 * block belongs to exactly one procedure.
	 *
	 * The procedures itself are saved in the call graph structure
	 * called @ref flowgraph.
	 */
	class procedure
	{
	public:
		static proc_ptr unmarshal(const rdf::node &n, flow_ptr flow, const rdf::storage &store);

		/// Constructs an empty procedure with name @arg n
		procedure(const std::string &n = std::string("proc_noname"));a

		/// Construct a new procedure. Copies basic blocks betwee @arg begin and @arg end into the instance
		template<typename FW> procedure(FW begin, FW end) : procedure() { copy(begin,end,inserter(basic_blocks,basic_blocks.begin())); }

		/// Calls @arg fn for every basic block in the procedure in reverse postorder.
		void rev_postorder(std::function<void(bblock_ptr bb)> fn) const;

		digraph<boost::variant<bblock_wptr,rvalue>,guard> control_transfers;
		std::vector<bblock_ptr> basic_blocks;

		// public fields
		std::string name;	///< Human-readable name
		bblock_wptr entry;	///< Entry point
		prog_wptr parent;

		/// Create or extend a procedure by starting to disassemble using @arg main at offset @arg start in @arg tokens
		template<typename Tag>
		static proc_ptr disassemble(const proc_ptr proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, offset_t start);
	};

	odotstream &operator<<(odotstream &os, const procedure &p);
	oturtlestream &operator<<(oturtlestream &os, const procedure &p);
	std::string unique_name(const procedure &f);

	template<typename Tag>
	proc_ptr procedure::disassemble(const proc_ptr proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, offset_t start)
	{
		assert(start != naddr);

		std::unordered_set<offset_t> todo;
		std::unordered_map<offset_t,mnemonic> mnemonics;
		std::unordered_multimap<offset_t,std::pair<offset_t,guard>> source, destination;
		proc_ptr ret = (proc ? proc : proc_ptr(new procedure()));

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
						source.insert(std::make_pair(bb->area().last(),std::make_pair(ct.bblock.lock()->area().begin,ct.condition)));
						destination.insert(std::make_pair(ct.bblock.lock()->area().begin,std::make_pair(bb->area().last(),ct.condition)));
					}
					else if(ct.value.is_constant())
					{
						source.insert(std::make_pair(bb->area().last(),std::make_pair(ct.value.to_constant().content(),ct.condition)));
						destination.insert(std::make_pair(ct.value.to_constant().content(),std::make_pair(bb->area().last(),ct.condition)));
					}
				}
			}

			proc->basic_blocks.clear();
		}

		todo.insert(start);

		while(!todo.empty())
		{
			offset_t cur_addr = *todo.begin();
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

			if(j == mnemonics.end() || !j->second.area.includes(cur_addr))
			{
				advance(i,cur_addr);
				tie(ret,i) = main.match(i,tokens.end(),state);

				if(ret)
				{
					offset_t last = 0;

					for(const mnemonic &m: state.mnemonics)
					{
						last = std::max(last,m.area.last());
						assert(mnemonics.insert(std::make_pair(m.area.begin,m)).second);
					}

					for(const std::pair<rvalue,guard> &p: state.jumps)
					{
						if(p.first.is_constant())
						{
							offset_t target = p.first.to_constant().content();

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
			else if(j->second.area.begin != cur_addr)
			{
				std::cerr << "Overlapping mnemonics at " << cur_addr << " with \"" << "[" << j->second.area << "] " << j->second << "\"" << std::endl;
			}
		}

		auto cur_mne = mnemonics.begin(), first_mne = cur_mne;
		std::unordered_map<offset_t,bblock_ptr> bblocks;
		std::function<void(std::unordered_map<offset_t,mnemonic>::iterator,std::unordered_map<offset_t,mnemonic>::iterator)> make_bblock;
		make_bblock = [&](std::unordered_map<offset_t,mnemonic>::iterator begin,std::unordered_map<offset_t,mnemonic>::iterator end)
		{
			bblock_ptr bb(new basic_block());

			// copy mnemonics
			bb->mutate_mnemonics([&](std::vector<mnemonic> &ms)
			{
				std::for_each(begin,end,[&](const std::pair<offset_t,mnemonic> &p)
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
			offset_t div = mne.area.end;
			auto sources = source.equal_range(mne.area.last());
			auto destinations = destination.equal_range(div);

			if(next_mne != mnemonics.end() && mne.area.size())
			{
				bool new_bb;

				// if next mnemonic is adjacent
				new_bb = next_mne->first != div;

				// or any following jumps aren't to adjacent mnemonics
				new_bb |= std::any_of(sources.first,sources.second,[&](const std::pair<offset_t,std::pair<offset_t,guard>> &p)
				{
					return p.second.first != div;
				});

				// or any jumps pointing to the next that aren't from here
				new_bb |= std::any_of(destinations.first,destinations.second,[&](const std::pair<offset_t,std::pair<offset_t,guard>> &p)
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
		for(const std::pair<offset_t,std::pair<offset_t,guard>> &p: source)
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
			offset_t entry = proc && proc->entry ? proc->entry->area().begin : start;
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

		if(!proc && ret->basic_blocks.size() > 0)
			ret->name = "proc_" + std::to_string(ret->entry->area().begin);
		else if(ret->basic_blocks.size() > 0)
			ret->name = "proc_(empty)";

		return ret;
	}
}

/*
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

	**
	 * Splits the @ref bb into two. If @ref last is true all mnemonics in @ref bb
	 * up to @ref pos are includes into the first. Otherwise the mnemonic at @ref pos
	 * is the first in the second basic block.
	 * @returns Pair of basic blocks.
	 *
	std::pair<bblock_ptr,bblock_ptr> split(bblock_ptr bb, offset pos, bool last);

	/// Merges two adjacent basic blocks into one.
	bblock_ptr merge(bblock_ptr up, bblock_ptr down);

	/// @internal
	void replace(std::list<ctrans> &lst, bblock_ptr from, bblock_ptr to);
	/// @internal
	void resolve(std::list<ctrans> &lst, rvalue v, bblock_ptr bb);
	/// @internal
	void conditional_jump(const ctrans &from, const ctrans &to);


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
}*/
