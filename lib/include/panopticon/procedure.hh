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
#include <panopticon/tree.hh>
#include <panopticon/hash.hh>

#pragma once

namespace po
{
	using proc_loc = loc<struct procedure>;
	using proc_wloc = wloc<struct procedure>;
	using prog_loc = loc<struct program>;
	using prog_wloc = wloc<struct program>;

	/// Run @arg f on all IL statements. Basic blocks a traversed in undefined order.
	void execute(proc_loc proc,std::function<void(const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)> f);

	/// Returns basic block occuping address @arg a
	boost::optional<bblock_loc> find_bblock(proc_loc proc, offset a);

	/**
	 * @brief Function
	 *
	 * Panopticon groups basic blocks into procedures. Each basic
	 * block belongs to exactly one procedure.
	 *
	 * The procedures itself are saved in the call graph structure
	 * called @ref program.
	 */
	struct procedure
	{
		/// Constructs an empty procedure with name @arg n
		procedure(const std::string &n);

		/// Calls @arg fn for every basic block in the procedure in reverse postorder.
		const std::vector<bblock_loc>& rev_postorder(void) const;
		const tree<bblock_loc>& domiance(void) const;

		std::string name;	///< Human-readable name
		boost::optional<bblock_loc> entry;	///< Entry point
		digraph<boost::variant<bblock_loc,rvalue>,guard> control_transfers;

		/// Create or extend a procedure by starting to disassemble using @arg main at offset @arg start in @arg tokens
		template<typename Tag>
		static proc_loc disassemble(boost::optional<proc_loc>, const disassembler<Tag>&, std::vector<typename rule<Tag>::token>, offset);

	private:
		mutable boost::optional<std::vector<bblock_loc>> _rev_postorder;
		mutable boost::optional<tree<bblock_loc>> _dominance;
	};

	template<>
	procedure* unmarshal(const uuid&, const rdf::storage&);

	template<>
	rdf::statements marshal(const procedure*, const uuid&);

	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(proc_loc p, bblock_loc from, bblock_loc to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(proc_loc p, bblock_loc from, rvalue to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(proc_loc p, bblock_loc from, bblock_loc to);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(proc_loc p, bblock_loc from, rvalue to);

	std::pair<typename boost::shared_container_iterator<std::set<typename boost::graph_traits<po::digraph<boost::variant<bblock_loc,rvalue>,guard>>::edge_descriptor>>,typename boost::shared_container_iterator<std::set<typename boost::graph_traits<po::digraph<boost::variant<bblock_loc,rvalue>,guard>>::edge_descriptor>>>
	incoming(proc_loc p, bblock_loc bb);

	std::pair<typename boost::graph_traits<decltype(procedure::control_transfers)>::out_edge_iterator,typename boost::graph_traits<decltype(procedure::control_transfers)>::out_edge_iterator>
	outgoing(proc_loc p, bblock_loc bb);

	/// Replaces the source basic block @ref oldbb with @ref newbb in all outgoing control transfers of @ref to.
	void replace_incoming(bblock_loc to, bblock_loc oldbb, bblock_loc newbb);
	/// Replaces the destination basic block @ref oldbb with @ref newbb in all outgoing control transfers of @ref from.
	void replace_outgoing(bblock_loc from, bblock_loc oldbb, bblock_loc newbb);
	/// Sets the source basic block to @ref bb in every incoming control transfer of @ref to that has a source value equal to @ref v
	void resolve_incoming(bblock_loc to, rvalue v, bblock_loc bb);
	/// Sets the destination basic block to @ref bb in every outgoing control transfer of @ref from that has a destination value equal to @ref v
	void resolve_outgoing(bblock_loc from, rvalue v, bblock_loc bb);

	/**
	 * Splits the @ref bb into two. If @ref last is true all mnemonics in @ref bb
	 * up to @ref pos are includes into the first. Otherwise the mnemonic at @ref pos
	 * is the first in the second basic block.
	 * @returns Pair of basic blocks.
	 */
	std::pair<bblock_loc,bblock_loc> split(bblock_loc bb, offset pos, bool last);

	/// Merges two adjacent basic blocks into one.
	bblock_loc merge(bblock_loc up, bblock_loc down);

	template<typename Tag>
	proc_loc procedure::disassemble(boost::optional<proc_loc> proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, offset start)
	{
		std::unordered_set<offset> todo;
		std::map<offset,mnemonic> mnemonics;
		std::unordered_multimap<offset,std::pair<boost::optional<offset>,guard>> source;
		std::unordered_multimap<offset,std::pair<offset,guard>> destination;
		proc_loc ret = (proc ? *proc : proc_loc(new procedure("proc_noname")));
		std::function<boost::optional<bound>(const boost::variant<rvalue,bblock_wloc>&)> get_off = [&](const boost::variant<rvalue,bblock_wloc>& v) -> boost::optional<bound>
		{
			if(boost::get<rvalue>(&v))
			{
				rvalue rv = boost::get<rvalue>(v);
				if(is_constant(rv))
					return boost::optional<bound>((to_constant(rv).content(),to_constant(rv).content() + 1));
				else
					return boost::none;
			}
			else if(boost::get<bblock_wloc>(&v))
			{
				return boost::make_optional(boost::get<bblock_wloc>(v).lock().read()->area());
			}
			else
				return boost::none;
		};

		// copy exsisting mnemonics and jumps into tables. TODO: cache tables in proc
		if(proc)
		{
			for(const bblock_loc bb: (*proc)->rev_postorder())
			{
				for(const mnemonic &m: bb->mnemonics())
				{
					assert(boost::icl::size(m.area));
					mnemonics.emplace(m.area.upper() - 1,m);
				}
			}

			for(auto e: iters(edges((*proc)->control_transfers)))
			{
				auto tgt = target(e,(*proc)->control_transfers);
				auto src = boost::source(e,(*proc)->control_transfers);

				auto src_b = get_off(get_node(src,(*proc)->control_transfers));
				auto tgt_b = get_off(get_node(tgt,(*proc)->control_transfers));

				if(src_b && tgt_b)
				{
					std::pair<offset,guard> p(tgt_b->lower(),get_edge(e,(*proc)->control_transfers));

					source.emplace(src_b->upper() - 1,p);
					destination.emplace(p.first,std::make_pair(src_b->upper() - 1,p.second));
				}
			}

			(*proc).write().control_transfers = digraph<boost::variant<bblock_loc, rvalue>, po::guard>();
		}

		todo.insert(start);

		while(!todo.empty())
		{
			offset cur_addr = *todo.begin();
			sem_state<Tag> state(cur_addr);
			typename rule<Tag>::tokiter i = tokens.begin();
			auto j = mnemonics.lower_bound(cur_addr);

			todo.erase(todo.begin());

			if(cur_addr >= tokens.size())
			{
				std::cout << "boundary err" << std::endl;
				continue;
			}

			if(j == mnemonics.end() || !boost::icl::contains(j->second.area,cur_addr))
			{
				advance(i,cur_addr);
				auto mi = main.match(i,tokens.end(),state);

				if(mi)
				{
					i = *mi;
					offset last = 0;

					for(const mnemonic &m: state.mnemonics)
					{
						last = std::max(last,m.area.upper() - 1);
						assert(mnemonics.insert(std::make_pair(m.area.lower(),m)).second);
					}

					for(const std::pair<rvalue,guard> &p: state.jumps)
					{
						if(is_constant(p.first))
						{
							offset target = to_constant(p.first).content();

							source.insert(std::make_pair(last,std::make_pair(target,p.second)));
							destination.insert(std::make_pair(target,std::make_pair(last,p.second)));
							todo.insert(target);
						}
						else
						{
							source.emplace(last,std::make_pair(boost::none,p.second));
						}
					}
				}
				else
				{
					std::cerr << "Failed to match anything at " << cur_addr << std::endl;
				}
			}
			else if(j->second.area.lower() != cur_addr)
			{
				std::cerr << "Overlapping mnemonics at " << cur_addr << " with \"" << "[" << j->second.area << "] " << j->second << "\"" << std::endl;
			}
		}

		auto cur_mne = mnemonics.begin(), first_mne = cur_mne;
		std::map<offset,bblock_loc> bblocks;
		std::function<void(std::map<offset,mnemonic>::iterator,std::map<offset,mnemonic>::iterator)> make_bblock;
		make_bblock = [&](std::map<offset,mnemonic>::iterator begin,std::map<offset,mnemonic>::iterator end)
		{
			bblock_loc bb(new basic_block());

			std::for_each(begin,end,[&](const std::pair<offset,mnemonic> &p)
				{ bb.write().mnemonics().push_back(p.second); });

			insert_node<boost::variant<bblock_loc,rvalue>,guard>(bb,ret.write().control_transfers);
			assert(bblocks.emplace(bb->area().upper() - 1,bb).second);
		};

		while(cur_mne != mnemonics.end())
		{
			auto next_mne = std::next(cur_mne);
			const mnemonic &mne = cur_mne->second;
			offset div = mne.area.upper();
			auto sources = source.equal_range(mne.area.upper() - 1);
			auto destinations = destination.equal_range(div);

			if(next_mne != mnemonics.end() && boost::icl::size(mne.area))
			{
				bool new_bb;

				// if next mnemonic is adjacent
				new_bb = next_mne->first != div;

				// or any following jumps aren't to adjacent mnemonics
				new_bb |= std::any_of(sources.first,sources.second,[&](const std::pair<offset,std::pair<boost::optional<offset>,guard>> &p)
				{
					return p.second.first && *p.second.first != div;
				});

				// or any jumps pointing to the next that aren't from here
				new_bb |= std::any_of(destinations.first,destinations.second,[&](const std::pair<offset,std::pair<offset,guard>> &p)
				{
					return p.second.first != mne.area.upper() - 1;
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
		for(const std::pair<offset,std::pair<boost::optional<offset>,guard>> &p: source)
		{
			if(p.second.first)
			{
				auto from = bblocks.find(p.first), to = bblocks.lower_bound(*p.second.first);

				assert(from != bblocks.end());
				if(to != bblocks.end() && to->second->area().lower() == *p.second.first)
					conditional_jump(ret,from->second,to->second,p.second.second);
				else
					conditional_jump(ret,from->second,po::constant(*p.second.first),p.second.second);
			}
		}

		auto q = vertices(ret->control_transfers);

		if(std::distance(q.first,q.second) == 1 &&
			 get<bblock_loc>(&get_node(*q.first,ret->control_transfers)) &&
			 get<bblock_loc>(get_node(*q.first,ret->control_transfers))->mnemonics().empty())
			ret.write().control_transfers = digraph<boost::variant<bblock_loc,rvalue>,guard>();

		q = vertices(ret->control_transfers);

		// entry may have been split
		if((proc && (*proc)->entry) || std::distance(q.first,q.second))
		{
			offset entry = proc && (*proc)->entry ? (*(*proc)->entry)->area().lower() : start;
			auto i = bblocks.lower_bound(entry);

			if(i != bblocks.end() && i->second->area().lower() == entry)
				ret.write().entry = i->second;
			else
				ret.write().entry = bblocks.lower_bound(start)->second;
		}
		else if(!proc)
		{
			ret.write().entry = bblocks.lower_bound(start)->second;
		}
		else
		{
			ret.write().entry = boost::none;
		}

		q = vertices(ret->control_transfers);

		if(!proc && std::distance(q.first,q.second) > 0)
			ret.write().name = "proc_" + std::to_string((*ret->entry)->area().lower());
		else if(std::distance(q.first,q.second) > 0)
			ret.write().name = "proc_(empty)";

		return ret;
	}
}
