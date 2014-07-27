#include <algorithm>
#include <list>
#include <map>
#include <iostream>

#include <panopticon/dflow.hh>
#include <panopticon/value.hh>
#include <panopticon/digraph.hh>
#include <panopticon/ensure.hh>

#include <boost/graph/dominator_tree.hpp>

using namespace po;
using namespace std;

boost::optional<dom> po::dominance_tree(proc_loc proc)
{
	if(!proc->entry)
		return boost::none;

	using vx_desc = boost::graph_traits<digraph<boost::variant<bblock_loc,rvalue>,guard>>::vertex_descriptor;
	using domtree_pm_type = boost::associative_property_map<std::unordered_map<vx_desc,vx_desc>>;
	std::unordered_map<vx_desc,vx_desc> idom;
	auto ent = find_node(boost::variant<bblock_loc,rvalue>(*(proc->entry)),proc->control_transfers);

	boost::lengauer_tarjan_dominator_tree(proc->control_transfers,ent,domtree_pm_type(idom));

	// build dominance tree from idom
	dom ret;
	std::unordered_map<bblock_wloc,bblock_wloc> idom2;

	for(auto p: idom)
	{
			auto vx_a = get_vertex(p.first,proc->control_transfers);
			auto vx_b = get_vertex(p.second,proc->control_transfers);

			if(get<bblock_loc>(&vx_a) && get<bblock_loc>(&vx_b))
				idom2.emplace(get<bblock_loc>(vx_a), get<bblock_loc>(vx_b));
	}
	idom2.emplace(*(proc->entry),*(proc->entry));

	ret.dominance = po::tree<po::bblock_wloc>::from_map(idom2);
	idom.emplace(ent,ent);

	// dominance frontiers
	for(auto n: proc->rev_postorder())
	{
		auto n_vx = find_node(boost::variant<bblock_loc,rvalue>(n),proc->control_transfers);

		if(in_degree(n_vx,proc->control_transfers) >= 2)
		{
			for(auto in: iters(in_edges(n_vx,proc->control_transfers)))
			{
				bblock_loc runner = boost::get<bblock_loc>(get_vertex(source(in,proc->control_transfers),proc->control_transfers));
				auto runner_vx = find_node(boost::variant<bblock_loc,rvalue>(runner),proc->control_transfers);

				while(runner_vx != idom.at(n_vx))
				{
					ret.frontiers.emplace(runner,n);
					runner = boost::get<bblock_loc>(get_vertex(idom.at(runner_vx),proc->control_transfers));
					runner_vx = find_node(boost::variant<bblock_loc,rvalue>(runner),proc->control_transfers);
				}
			}
		}
	}

	return ret;
}

live po::liveness(proc_loc proc)
{
	live ret;

	auto collect = [&](const rvalue &v, bblock_loc bb)
	{
		if(is_variable(v))
		{
			ret.names.insert(to_variable(v).name());
			if(!ret[bb].varkill.count(to_variable(v).name()))
				ret[bb].uevar.insert(to_variable(v).name());
		}
	};

	// build global names and blocks that use them
	for(bblock_loc bb: proc->rev_postorder())
	{
		std::function<void(const instr&)> fn;
		fn = [&](const instr &i)
		{
			std::vector<rvalue> right = operands(i);
			for(const rvalue &v: right)
				collect(v,bb);

			if(is_variable(i.assignee))
			{
				ret[bb].varkill.insert(to_variable(i.assignee).name());
				ret.names.insert(to_variable(i.assignee).name());
				ret.usage.insert(std::make_pair(to_variable(i.assignee).name(),bb));
			}
		};

		execute(bb,fn);

		auto vx = find_node(boost::variant<bblock_loc,rvalue>(bb),proc->control_transfers);
		for(auto e: iters(out_edges(vx,proc->control_transfers)))
		{
			guard g = get_edge(e,proc->control_transfers);
			auto wx = target(e,proc->control_transfers);
			const rvalue *rv = boost::get<rvalue>(&get_vertex(wx,proc->control_transfers));

			if(rv)
				collect(*rv,bb);

			for(const relation &rel: g.relations)
			{
				collect(rel.operand1,bb);
				collect(rel.operand2,bb);
			}
		}
	}

	bool mod;

	do
	{
		mod = false;

		for(bblock_loc bb: proc->rev_postorder())
		{
			set<std::string> old_liveout = ret[bb].liveout;
			auto vx = find_node(boost::variant<bblock_loc,rvalue>(bb),proc->control_transfers);

			ret[bb].liveout.clear();

			// LiveOut = \_/ (UEVar \/ (LiveOut /\ !VarKill))
			// 					 succ
			for(auto e: iters(out_edges(vx,proc->control_transfers)))
			{
				bblock_loc s = get<bblock_loc>(get_vertex(target(e,proc->control_transfers),proc->control_transfers));
				ret[bb].liveout = set_union(ret[bb].liveout,set_union(ret[s].uevar,set_intersection(ret[s].liveout,set_difference(ret.names,ret[s].varkill))));
			}

			mod |= old_liveout != ret[bb].liveout;
		}
	}
	while(mod);

	return ret;
}

void po::ssa(proc_loc proc, const dom& domi, const live& li)
{
	std::set<std::string> globals;
	has_symbol_visitor<phi_symbol> phi_vis;

	for(auto s: iters(vertices(proc->control_transfers)))
	{
		auto n = get_vertex(s,proc->control_transfers);

		if(boost::get<bblock_loc>(&n))
			globals = set_union(globals,li[boost::get<bblock_loc>(n)].uevar);
	}

	// insert phi
	for(const std::string &n: globals)
	{
		auto p = li.usage.equal_range(n);
		std::unordered_set<bblock_loc> worklist;

		std::transform(p.first,p.second,inserter(worklist,worklist.begin()),[&](const std::pair<std::string,bblock_wloc>& q) { return q.second.lock(); });

		while(!worklist.empty())
		{
			bblock_loc bb = *worklist.begin();

			worklist.erase(worklist.begin());

			for(auto q: iters(domi.frontiers.equal_range(bb)))
			{
				bool has_phi = false;
				bblock_loc frontier = q.second.lock();
				std::function<void(const instr&)> fn;
				fn = [&](const instr& i)
				{
					has_phi = has_phi || (boost::apply_visitor(phi_vis,i.function) && is_variable(i.assignee) && to_variable(i.assignee).name() == n);
				};

				execute(frontier,fn);

				if(!has_phi)
				{
					std::vector<mnemonic> &ms = frontier.write().mnemonics();
					ensure(ms.size());

					if(ms[0].opcode == "internal-phis")
						ms[0].instructions.emplace_back(instr(univ_phi<rvalue>{{}},variable(n,512)));
					else
						ms.emplace(ms.begin(),mnemonic(bound(ms.front().area.lower(),ms.front().area.lower()),"internal-phis","",{},{instr(univ_phi<rvalue>{{}},variable(n,512))}));
					worklist.insert(frontier);
				}
			}
		}
	}

	// rename variables
	std::unordered_map<std::string,int> counter;
	std::unordered_map<std::string,std::list<int>> stack;

	for(const std::string &n: li.names)
	{
		counter.insert(std::make_pair(n,1));
		stack.insert(std::make_pair(n,std::list<int>({0})));
	}

	auto new_name = [&](const std::string &n) -> int
	{
		ensure(stack.count(n));
		int i = counter[n]++;

		stack[n].push_back(i);
		return i;
	};

	// rename ssa vars in a bblock
	function<void(bblock_loc bb)> rename;
	rename = [&](bblock_loc bb)
	{
		// for each φ-function in b, ‘‘x ← φ(· · · )’‘
		//     rewrite x as new_name(x)
		rewrite(bb,[&](instr &i)
		{
			if(boost::apply_visitor(phi_vis,i.function))
			{
				ensure(is_variable(i.assignee));
				i.assignee = variable(to_variable(i.assignee).name(),to_variable(i.assignee).width(),new_name(to_variable(i.assignee).name()));
			}
		});

		// for each mnemonic ‘‘opcode y, z’’ in bb and
		// for each operation ‘‘x ← y op z’’ in bb
		//     rewrite y with subscript top(stack[y])
		//     rewrite z with subscript top(stack[z])
		//     rewrite x as new_name(x)
		std::vector<mnemonic> &ms = bb.write().mnemonics();
		const size_t sz_mne = ms.size();
		size_t i_mne = 0;

		while(i_mne < sz_mne)
		{
			mnemonic &mne = ms.at(i_mne++);
			const size_t sz_instr = mne.instructions.size();
			size_t i_instr = 0;

			for(rvalue v: mne.operands)
			{
				if(is_variable(v))
				{
					ensure(stack.count(to_variable(v).name()));
					v = variable(to_variable(v).name(),to_variable(v).width(),stack[to_variable(v).name()].back());
				}
			}

			while(i_instr < sz_instr)
			{
				instr &instr = mne.instructions.at(i_instr++);
				lvalue &assignee = instr.assignee;
				instr::operation fn = instr.function;
				vector<rvalue> right = operands(instr);

				if(!boost::apply_visitor(phi_vis,instr.function))
				{
					unsigned int ri = 0;

					while(ri < right.size())
					{
						const rvalue &v = right[ri];

						if(is_variable(v))
						{
							ensure(stack.count(to_variable(v).name()));
							right[ri] = variable(to_variable(v).name(),to_variable(v).width(),stack[to_variable(v).name()].back());
						}
						++ri;
					}

					set_operands(instr,right);

					if(is_variable(assignee))
						assignee = variable(to_variable(assignee).name(),to_variable(assignee).width(),new_name(to_variable(assignee).name()));
				}
			}
		}

		// for each successor of b in the cfg
		// 		 rewrite variables in ctrans
		//     fill in φ-function parameters
		for(auto ed: iters(out_edges(find_node(boost::variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers)))
		{
			auto succ = get_vertex(target(ed,proc->control_transfers),proc->control_transfers);
			guard& g = get_edge(ed,proc.write().control_transfers);

			// rewrite vars in relations
			for(relation &rel: g.relations)
			{
				if(is_variable(rel.operand1))
				{
					const variable o1 = to_variable(rel.operand1);
					ensure(stack.count(o1.name()));
					rel.operand1 = variable(o1.name(),o1.width(),stack[o1.name()].back());
				}
				if(is_variable(rel.operand2))
				{
					const variable o2 = to_variable(rel.operand2);
					ensure(stack.count(o2.name()));
					rel.operand2 = variable(o2.name(),o2.width(),stack[o2.name()].back());
				}
			}

			// rewrite symbolic target in ctrans
			if(boost::get<rvalue>(&succ) && is_variable(get<rvalue>(succ)))
			{
				const variable v = to_variable(get<rvalue>(succ));
				ensure(stack.count(v.name()));
				get<rvalue>(succ) = variable(v.name(),v.width(),stack[v.name()].back());
			}

			// fill in φ-function parameters in successor
			if(boost::get<bblock_loc>(&succ))
			{
				bblock_loc s = boost::get<bblock_loc>(succ);
				std::vector<mnemonic> &mn_s = s.write().mnemonics();
				auto in_p = in_edges(target(ed,proc->control_transfers),proc->control_transfers);
				auto iord = find(in_p.first,in_p.second,ed);
				ensure(iord != in_p.second);
				unsigned int ord = distance(in_p.first,iord);

				if(mn_s.size() && mn_s.front().opcode == "internal-phis")
				{
					mnemonic &mne = mn_s.front();

					for(instr &i: mne.instructions)
					{
						std::vector<rvalue> right = operands(i);
						ensure(boost::apply_visitor(phi_vis,i.function) && is_variable(i.assignee));
						int missing = ord - right.size() + 1;

						while(missing > 0)
						{
							right.emplace_back(undefined());
							--missing;
						}

						variable var = to_variable(i.assignee);
						ensure(stack.count(var.name()));

						const std::list<int>& l = stack.at(var.name());
						ensure(l.size());

						right[ord] = variable(var.name(),var.width(),l.back());
						set_operands(i,right);
					}
				}
			}
		}

		// for each successor s of b in the dominator tree
		//     rename(s)
		auto dfs = tree<bblock_wloc>::depth_first_search(domi.dominance.root(),domi.dominance);
		while(dfs.first != dfs.second)
		{
			if(*dfs.first == bb)
			{
				auto j = domi.dominance.begin(dfs.first);
				while(j != domi.dominance.end(dfs.first))
					rename(j++->lock());
			}
			++dfs.first;
		}

		// for each operation ‘‘x ← y op z’’ in bb
		//     and each φ-function ‘‘x ← φ(· · · )’’
		//     pop(stack[x])
		std::function<void(const instr&)> fn;
		fn = [&](const instr &i)
		{
			if(is_variable(i.assignee))
			{
				ensure(stack.count(to_variable(i.assignee).name()) && !stack.at(to_variable(i.assignee).name()).empty());
				stack[to_variable(i.assignee).name()].pop_back();
			}
		};
		execute(bb,fn);
	};

	rename(*(proc->entry));
}
