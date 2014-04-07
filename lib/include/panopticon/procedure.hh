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
	using proc_loc = loc<struct procedure>;
	using proc_wloc = wloc<struct procedure>;
	using prog_wloc = wloc<struct program>;

	/// Run @arg f on all IL statements. Basic blocks a traversed in undefined order.
	void execute(proc_loc proc,std::function<void(const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)> f);

	/// Returns basic block occuping address @arg a
	bblock_loc find_bblock(proc_loc proc, offset a);

	/**
	 * @brief Function
	 *
	 * Panopticon groups basic blocks into procedures. Each basic
	 * block belongs to exactly one procedure.
	 *
	 * The procedures itself are saved in the call graph structure
	 * called @ref flowgraph.
	 */
	struct procedure
	{
		/// Constructs an empty procedure with name @arg n
		procedure(const std::string &n = std::string("proc_noname"));

		/// Calls @arg fn for every basic block in the procedure in reverse postorder.
		const std::vector<bblock_loc>& rev_postorder(void) const;

		prog_wloc parent;
		std::string name;	///< Human-readable name
		boost::optional<bblock_loc> entry;	///< Entry point
		digraph<boost::variant<bblock_loc,rvalue>,guard> control_transfers;

		/// Create or extend a procedure by starting to disassemble using @arg main at offset @arg start in @arg tokens
		template<typename Tag>
		static proc_loc disassemble(boost::optional<proc_loc> proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, offset start);

	private:
		boost::optional<std::vector<bblock_loc>> _rev_postorder;
	};

	template<>
	procedure* unmarshal(const uuid&, const rdf::storage&);

	template<>
	rdf::statements marshal(const basic_block*, const uuid&);

	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(bblock_loc from, bblock_loc to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(rvalue from, bblock_loc to, guard g);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void conditional_jump(bblock_loc from, rvalue to, guard g);

	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(bblock_loc from, bblock_loc to);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(rvalue from, bblock_loc to);
	/// Adds an control transfer with @ref from as source and @ref to as destination
	void unconditional_jump(bblock_loc from, rvalue to);

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

	/// @internal
	void replace(std::list<ctrans> &lst, bblock_loc from, bblock_loc to);
	/// @internal
	void resolve(std::list<ctrans> &lst, rvalue v, bblock_loc bb);
	/// @internal
	void conditional_jump(const ctrans &from, const ctrans &to);

	template<typename Tag>
	proc_loc procedure::disassemble(boost::optional<proc_loc> proc, const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, offset start)
	{
		std::unordered_set<offset> todo;
		std::map<offset,mnemonic> mnemonics;
		std::unordered_multimap<offset,std::pair<boost::optional<offset>,guard>> source;
		std::unordered_multimap<offset,std::pair<offset,guard>> destination;
		proc_loc ret = (proc ? *proc : proc_loc(new procedure()));
		std::function<boost::optional<bound>(const boost::variant<rvalue,bblock_wloc>&)> get_off = [&](const boost::variant<rvalue,bblock_wloc>& v) -> boost::optional<bound>
		{
			if(boost::get<rvalue>(&v))
			{
				rvalue rv = boost::get<rvalue>(v);
				if(is_constant(rv))
					return boost::optional<bound>((to_constant(rv).content(),to_constant(rv).content() + 1));
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

				auto src_b = get_off(src);
				auto tgt_b = get_off(tgt);

				if(src_b && tgt_b)
				{
					std::pair<offset,guard> p(tgt_b->lower(),get_edge(e,(*proc)->control_transfers));

					source.emplace(src_b->upper() - 1,p);
					destination.emplace(p,src_b->upper() - 1);
				}
			}

			(*proc).write().control_transfers = digraph<boost::variant<bblock_loc, rvalue>, po::guard>();
		}

		todo.insert(start);

		while(!todo.empty())
		{
			offset cur_addr = *todo.begin();
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

			if(j == mnemonics.end() || !boost::icl::contains(j->second.area,cur_addr))
			{
				advance(i,cur_addr);
				tie(ret,i) = main.match(i,tokens.end(),state);

				if(ret)
				{
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
				new_bb |= std::any_of(sources.first,sources.second,[&](const std::pair<offset,std::pair<offset,guard>> &p)
				{
					return p.second.first != div;
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
					conditional_jump(from->second,to->second,p.second.second);
				else
					conditional_jump(from->second,po::constant(*p.second.first),p.second.second);
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

/*
void po::conditional_jump(bblock_loc from, bblock_loc to, guard g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void po::conditional_jump(rvalue from, bblock_loc to, guard g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }
void po::conditional_jump(bblock_loc from, rvalue to, guard g) { ctrans ct_from(g,from), ct_to(g,to); conditional_jump(ct_from,ct_to); }

void po::unconditional_jump(bblock_loc from, bblock_loc to) { conditional_jump(from,to,guard()); }
void po::unconditional_jump(rvalue from, bblock_loc to) { conditional_jump(from,to,guard()); }
void po::unconditional_jump(bblock_loc from, rvalue to) { conditional_jump(from,to,guard()); }

void po::replace_incoming(bblock_loc to, bblock_loc oldbb, bblock_loc newbb)
{
	assert(to && oldbb && newbb);
	to->mutate_incoming([&](list<ctrans> &in)
	{
		replace(in,oldbb,newbb);
	});
}

void po::replace_outgoing(bblock_loc from, bblock_loc oldbb, bblock_loc newbb)
{
	assert(from && oldbb && newbb);
	from->mutate_outgoing([&](list<ctrans> &out)
	{
		replace(out,oldbb,newbb);
	});
}

void po::resolve_incoming(bblock_loc to, rvalue v, bblock_loc bb)
{
	assert(to && bb);
	to->mutate_incoming([&](list<ctrans> &in)
	{
		resolve(in,v,bb);
	});
}

void po::resolve_outgoing(bblock_loc from, rvalue v, bblock_loc bb)
{
	assert(from && bb);
	from->mutate_outgoing([&](list<ctrans> &out)
	{
		resolve(out,v,bb);
	});
}

// last == true -> pos is last in `up', last == false -> pos is first in `down'
pair<bblock_loc,bblock_loc> po::split(bblock_loc bb, addr_t pos, bool last)
{
	assert(bb);

	bblock_loc up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::out_iterator j,jend;
	basic_block::in_iterator k,kend;
	function<void(bool,bblock_loc,ctrans)> append = [](bool in, bblock_loc bb, ctrans ct)
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

bblock_loc po::merge(bblock_loc up, bblock_loc down)
{
	assert(up && down);
	if(up->area().begin == down->area().end) tie(up,down) = make_pair(down,up);
	assert(up->area().end == down->area().begin);

	bblock_loc ret(new basic_block());
	auto fn = [&ret](const bblock_loc &bb, const mnemonic &m) { ret->mutate_mnemonics([&](vector<mnemonic> &ms)
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

void po::replace(list<ctrans> &lst, bblock_loc from, bblock_loc to)
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

void po::resolve(list<ctrans> &lst, rvalue v, bblock_loc bb)
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
