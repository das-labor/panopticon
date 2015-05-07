/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <memory>
#include <set>
#include <map>
#include <list>
#include <cstring>
#include <stdexcept>
#include <algorithm>

#include <panopticon/basic_block.hh>
#include <panopticon/mnemonic.hh>
#include <panopticon/disassembler.hh>
#include <panopticon/marshal.hh>
#include <panopticon/tree.hh>
#include <panopticon/hash.hh>
#include <panopticon/ensure.hh>

#pragma once

namespace po
{
	using proc_loc = loc<struct procedure>;
	using proc_wloc = wloc<struct procedure>;
	using prog_loc = loc<struct program>;
	using prog_wloc = wloc<struct program>;

	/// Run @arg f on all IL statements. Basic blocks a traversed in undefined order.
	void execute(proc_loc proc,std::function<void(const instr&)> f);

	/// Returns basic block occuping address @arg a
	boost::optional<bblock_loc> find_bblock(proc_loc proc, offset a);

	std::list<mnemonic>& operator+=(std::list<mnemonic>& a, const std::list<mnemonic>& b);

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
		using graph_type = digraph<boost::variant<bblock_loc,rvalue>,guard>;

		/// Constructs an empty procedure with name @arg n
		procedure(const std::string &n);
		procedure(const procedure& p);

		procedure& operator=(const procedure& p);
		bool operator==(const procedure&) const;
		bool operator!=(const procedure&) const;

		/// Calls @arg fn for every basic block in the procedure in reverse postorder.
		const std::vector<bblock_loc>& rev_postorder(void) const;
		const tree<bblock_loc>& domiance(void) const;

		std::string name;	///< Human-readable name
		boost::optional<bblock_loc> entry;	///< Entry point
		graph_type control_transfers;

		/// Create or extend a procedure by starting to disassemble using @arg main at offset @arg start in @arg tokens
		template<typename Tag,typename Dis>
		static boost::optional<proc_loc> disassemble(boost::optional<proc_loc>, Dis const&, typename architecture_traits<Tag>::state_type const&, po::slab, offset);

	private:
		mutable boost::optional<std::vector<bblock_loc>> _rev_postorder;
		mutable boost::optional<tree<bblock_loc>> _dominance;
	};

	template<>
	std::unique_ptr<procedure> unmarshal(const uuid&, const rdf::storage&);

	template<>
	archive marshal(procedure const&, const uuid&);

	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(proc_loc p, bblock_loc from, bblock_loc to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(proc_loc p, bblock_loc from, rvalue to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(proc_loc p, bblock_loc from, bblock_loc to);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(proc_loc p, bblock_loc from, rvalue to);

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

	template<typename Tag,typename Dis>
	boost::optional<proc_loc> procedure::disassemble(boost::optional<proc_loc> proc, Dis const& main, typename architecture_traits<Tag>::state_type const& init, po::slab data, offset start)
	{
		std::unordered_set<offset> todo;
		std::map<offset,std::list<mnemonic>> mnemonics;
		std::unordered_multimap<offset,std::pair<boost::optional<offset>,guard>> source;
		std::unordered_multimap<offset,std::pair<offset,guard>> destination;
		boost::optional<proc_loc> ret = boost::none;
		std::function<boost::optional<bound>(const boost::variant<rvalue,bblock_wloc>&)> get_off = [&](const boost::variant<rvalue,bblock_wloc>& v) -> boost::optional<bound>
		{
			if(boost::get<rvalue>(&v))
			{
				rvalue rv = boost::get<rvalue>(v);
				if(is_constant(rv))
					return boost::optional<bound>((to_constant(rv).content(),to_constant(rv).content()));
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
			for(auto vx: iters(vertices((*proc)->control_transfers)))
			{
				auto nd = get_vertex(vx,(*proc)->control_transfers);

				if(get<bblock_loc>(&nd))
				{
					for(const mnemonic &m: get<bblock_loc>(nd)->mnemonics())
					{
						po::offset o = boost::icl::size(m.area) ? m.area.upper() - 1 : m.area.lower();
						mnemonics[o].push_back(m);
					}
				}
				else if(get<rvalue>(&nd) && is_constant(get<rvalue>(nd)))
				{
					todo.emplace(to_constant(get<rvalue>(nd)).content());
				}
			}

			for(auto e: iters(edges((*proc)->control_transfers)))
			{
				auto tgt = target(e,(*proc)->control_transfers);
				auto src = po::source(e,(*proc)->control_transfers);

				auto src_b = get_off(get_vertex(src,(*proc)->control_transfers));
				auto tgt_b = get_off(get_vertex(tgt,(*proc)->control_transfers));

				if(src_b && tgt_b)
				{
					guard g = get_edge(e,(*proc)->control_transfers);
					po::offset last = boost::icl::size(*src_b) ? src_b->upper() - 1 : src_b->lower();

					source.emplace(last,std::make_pair(tgt_b->lower(),g));
					destination.emplace(tgt_b->lower(),std::make_pair(last,g));
				}
			}

			//(*proc).write().control_transfers = digraph<boost::variant<bblock_loc, rvalue>, po::guard>();
		}

		ensure(source.size() == destination.size());
		todo.emplace(start);

		// disassemble targets
		while(!todo.empty())
		{
			offset cur_addr = *todo.begin();
			slab::iterator i = data.begin();
			auto j = mnemonics.lower_bound(cur_addr);
			boost::optional<po::bound> area = boost::none;

			if(j != mnemonics.end())
			{
				ensure(!j->second.empty());

				area = std::accumulate(j->second.begin(),j->second.end(),j->second.front().area,
					[](po::bound acc, mnemonic const& x) -> po::bound { return boost::icl::hull(x.area,acc); });
			}

			todo.erase(todo.begin());

			if(cur_addr >= data.size())
			{
				std::cerr << "boundary err: " << cur_addr << " not inside " << data.size() << " byte large slab" << std::endl;
				continue;
			}

			if(j == mnemonics.end() || (area && !boost::icl::contains(*area,cur_addr)))
			{
				i += cur_addr;
				sem_state<Tag> state(cur_addr,init);
				slab::iterator e = (j == mnemonics.end() ? data.end() : (data.begin() + j->first + 1));

				auto mi = main.try_match(i,e,state);

				if(mi)
				{
					i = mi->first;
					state = mi->second;
					offset last = 0;

					for(const mnemonic &m: state.mnemonics)
					{
						last = std::max<po::offset>(last,boost::icl::size(m.area) ? m.area.upper() - 1 : m.area.lower());
						mnemonics[m.area.lower()].push_back(m);
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
			else if(area && area->lower() != cur_addr)
			{
				std::cerr << "Overlapping mnemonics at " << cur_addr << " with \"" << "[" << *area << "] " << j->second.front() << "\"" << std::endl;
			}
		}

		if(!mnemonics.empty())
		{
			if(!ret)
			{
				ret = proc_loc(new procedure("(unnamed proc)"));
			}

			// rebuild basic blocks
			auto cur_mne = mnemonics.begin(), first_mne = cur_mne;
			std::map<offset,bblock_loc> bblocks;
			std::function<void(std::map<offset,std::list<mnemonic>>::iterator,
									 std::map<offset,std::list<mnemonic>>::iterator)> make_bblock;
			make_bblock = [&](std::map<offset,std::list<mnemonic>>::iterator begin,
									std::map<offset,std::list<mnemonic>>::iterator end)
			{
				bblock_loc bb(new basic_block());

				std::for_each(begin,end,[&](const std::pair<offset,std::list<mnemonic>> &p)
					{ std::copy(p.second.begin(),p.second.end(),std::back_inserter(bb.write().mnemonics())); });

				insert_vertex<boost::variant<bblock_loc,rvalue>,guard>(bb,ret->write().control_transfers);
				ensure(bblocks.insert(std::make_pair(boost::icl::size(bb->area()) ? bb->area().upper() - 1 : bb->area().lower(),bb)).second);
			};

			while(cur_mne != mnemonics.end())
			{
				auto next_mne = std::next(cur_mne);
				std::list<mnemonic> const& mne = cur_mne->second;
				po::bound area = std::accumulate(mne.begin(),mne.end(),mne.front().area,
					[](po::bound acc, mnemonic const& x) -> po::bound { return boost::icl::hull(x.area,acc); });
				auto sources = source.equal_range(boost::icl::size(area)? area.upper() - 1 : area.lower());
				auto destinations = destination.equal_range(area.upper());

				if(next_mne != mnemonics.end() && boost::icl::size(area))
				{
					bool new_bb;

					// if next mnemonic isn't adjacent
					new_bb = next_mne->first != area.upper();

					// or any following jumps aren't to adjacent mnemonics
					new_bb |= std::any_of(sources.first,sources.second,[&](const std::pair<offset,std::pair<boost::optional<offset>,guard>> &p)
					{
						return p.second.first && *p.second.first != area.upper();
					});

					// or any jumps pointing to the next that aren't from here
					new_bb |= std::any_of(destinations.first,destinations.second,[&](const std::pair<offset,std::pair<offset,guard>> &p)
					{
						return p.second.first != (boost::icl::size(area)? area.upper() - 1 : area.lower());
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

					if(from == bblocks.end())
						std::cerr << from->first << " is not part of bblocks" << std::endl;

					ensure(from != bblocks.end());
					if(to != bblocks.end() && to->second->area().lower() == *p.second.first)
						conditional_jump(*ret,from->second,to->second,p.second.second);
					else
						conditional_jump(*ret,from->second,po::constant(*p.second.first),p.second.second);
				}
			}

			auto q = vertices((*ret)->control_transfers);

			if(std::distance(q.first,q.second) == 1 &&
				 get<bblock_loc>(&get_vertex(*q.first,(*ret)->control_transfers)) &&
				 get<bblock_loc>(get_vertex(*q.first,(*ret)->control_transfers))->mnemonics().empty())
				ret->write().control_transfers = procedure::graph_type();

			q = vertices((*ret)->control_transfers);

			// entry may have been split
			if((proc && (*proc)->entry) || std::distance(q.first,q.second))
			{
				offset entry = proc && (*proc)->entry ? (*(*proc)->entry)->area().lower() : start;
				auto i = bblocks.lower_bound(entry);

				if(i != bblocks.end() && i->second->area().lower() == entry)
					ret->write().entry = i->second;
				else
					ret->write().entry = bblocks.lower_bound(start)->second;
			}
			else if(!proc)
			{
				auto j = bblocks.lower_bound(start);

				ensure(j != bblocks.end());
				ret->write().entry = j->second;
			}
			else
			{
				ret->write().entry = boost::none;
			}

			ensure(!proc || !(*proc)->entry || (*ret)->entry);

			q = vertices((*ret)->control_transfers);

			if(!proc && std::distance(q.first,q.second) > 0)
				ret->write().name = "proc_" + std::to_string((*(*ret)->entry)->area().lower());
			else if(std::distance(q.first,q.second) > 0)
				ret->write().name = "proc_(empty)";
		}

		return ret;
	}
}
