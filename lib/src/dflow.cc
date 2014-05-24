#include <algorithm>
#include <cassert>
#include <list>
#include <map>
#include <iostream>

#include <panopticon/dflow.hh>
#include <panopticon/value.hh>
#include <panopticon/digraph.hh>

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
/*
live_loc po::liveness(proc_cloc proc)
{
	live_loc ret(new live());

	auto collect = [&](const rvalue &v, bblock_loc bb)
	{
		if(v.is_variable())
		{
			ret->names.insert(v.to_variable());
			if(!ret->varkill[bb].count(v.to_variable()))
				ret->uevar[bb].insert(v.to_variable());
		}
	};

	// build global names and blocks that use them
	for(bblock_loc bb: proc->basic_blocks)
	{
		execute(bb,[&](const lvalue &left, instr::Function fn, const vector<rvalue> &right)
		{
			for(const rvalue &v: right)
				collect(v,bb);

			if(left.is_variable())
			{
				ret->varkill[bb].insert(left.to_variable());
				ret->names.insert(left.to_variable());
				ret->usage[left.to_variable()].insert(bb);
			}
		});

		for(const mnemonic &m: bb->mnemonics())
		{
			for(const rvalue &v: m.operands)
				collect(v,bb);
		}

		for(const ctrans &ct: bb->outgoing())
		{
			collect(ct.value,bb);

			for(const relation &rel: ct.condition.relations)
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

		for(bblock_loc bb: proc->basic_blocks)
		{
			set<name> old_liveout = ret->liveout[bb];
			basic_block::succ_iterator j,jend;

			ret->liveout[bb].clear();
			tie(j,jend) = bb->successors();

			// LiveOut = \_/ (UEVar \/ (LiveOut /\ !VarKill))
			// 					 succ
			for_each(j,j,[&](bblock_loc s)
				{	ret->liveout[bb] = set_union(ret->liveout[bb],set_union(ret->uevar[s],set_intersection(ret->liveout[s],set_difference(ret->names,ret->varkill[s])))); });

			mod |= old_liveout != ret->liveout[bb];
		}
	}
	while(mod);

	return ret;
}

void po::ssa(proc_loc proc, dom_loc dominance, live_loc live)
{
	set<name> globals;

	for(const pair<bblock_cwloc,set<name>> &s: live->uevar)
		globals = set_union(globals,s.second);

	if(live->liveout[proc->entry].size())
	{
		cout << "uninitialized vars: ";
		for(const name &n: live->liveout[proc->entry])
			cout << n.base << " ";
		cout << endl;
		assert(false);
	}

	// insert phi
	for(const name &n: globals)
	{
		set<bblock_cwloc> &worklist(live->usage[n]);

		while(!worklist.empty())
		{
			bblock_cloc bb = worklist.begin()->lock();

			worklist.erase(worklist.begin());
			for(dtree_loc df: dominance->tree[bb]->frontiers)
			{
				bool has_phi = false;
				execute(df->basic_block.lock(),[&](lvalue left, instr::Function fn, const vector<rvalue> &right)
				{
					has_phi = has_phi || (fn == instr::Phi && left.is_variable() && left.to_variable().name() == n.base);
				});

				if(!has_phi)
				{
					df->basic_block.lock()->mutate_mnemonics([&](vector<mnemonic> &ms)
					{
						assert(ms.size());

						if(ms[0].opcode == "internal-phis")
							ms[0].instructions.emplace_back(instr(instr::Phi,variable(n.base,-1,n.width)));
						else
							ms.emplace(ms.begin(),mnemonic(range<addr_t>(ms.front().area.begin,ms.front().area.begin),"internal-phis","",{},{instr(instr::Phi,variable(n.base,-1,n.width))}));
					});
					worklist.insert(df->basic_block);
				}
			}
		}
	}

	// rename variables
	map<string,int> counter;
	map<string,list<int>> stack;

	for(const string &n: live->names)
	{
		counter.insert(make_pair(n,0));
		stack.insert(make_pair(n,list<int>({})));
	}

	auto new_name = [&](const string &n) -> int
	{
		assert(stack.count(n));
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
		rewrite(bb,[&](lvalue &left, instr::Function fn, vector<rvalue> &right)
		{
			if(fn == instr::Phi)
			{
				assert(left.is_variable());
				left = variable(left.to_variable().name(),new_name(left.to_variable().name()),left.to_variable().width());
			}
		});

		// for each mnemonic ‘‘opcode y, z’’ in bb and
		// for each operation ‘‘x ← y op z’’ in bb
		//     rewrite y with subscript top(stack[y])
		//     rewrite z with subscript top(stack[z])
		//     rewrite x as new_name(x)
		bb->mutate_mnemonics([&](vector<mnemonic> &ms)
		{
			size_t sz_mne = ms.size(), i_mne = 0;
			mnemonic *ary_mne = ms.data();

			while(i_mne < sz_mne)
			{
				mnemonic &mne = ary_mne[i_mne++];
				size_t sz_instr = mne.instructions.size(), i_instr = 0;
				instr *ary_instr = mne.instructions.data();

				for(rvalue &v: mne.operands)
				{
					if(v.is_variable())
					{
						assert(stack.count(v.to_variable().name()));
						v = variable(v.to_variable().name(),stack[v.to_variable().name()].back(),v.to_variable().width());
					}
				}

				while(i_instr < sz_instr)
				{
					instr &instr = ary_instr[i_instr++];
					lvalue &left = instr.left;
					instr::Function fn = instr.function;
					vector<rvalue> &right = instr.right;

					if(fn != instr::Phi)
					{
						unsigned int ri = 0;

						while(ri < right.size())
						{
							const rvalue &v = right[ri];

							if(v.is_variable())
							{
								assert(stack.count(v.to_variable().name()));
								right[ri] = variable(v.to_variable().name(),stack[v.to_variable().name()].back(),v.to_variable().width());
							}
							++ri;
						}

						if(left.is_variable())
							left = variable(left.to_variable().name(),new_name(left.to_variable().name()),left.to_variable().width());
					}
				}
			}
		});

		// for each successor of b in the cfg
		// 		 rewrite variables in ctrans
		//     fill in φ-function parameters
		bb->mutate_outgoing([&](list<ctrans> &out)
		{
				list<ctrans> new_out;

				for(ctrans &s: out)
				{
					// rewrite vars in relations
					for(relation &rel: s.condition.relations)
					{
						if(rel.operand1.is_variable())
						{
							const variable o1 = rel.operand1.to_variable();
							assert(stack.count(o1.name()));
							rel.operand1 = variable(o1.name(),stack[o1.name()].back(),o1.width());
						}
						if(rel.operand2.is_variable())
						{
							const variable o2 = rel.operand2.to_variable();
							assert(stack.count(o2.name()));
							rel.operand2 = variable(o2.name(),stack[o2.name()].back(),o2.width());
						}
					}

					// rewrite symbolic target in ctrans
					if(s.value.is_variable())
					{
						const variable v = s.value.to_variable();
						assert(stack.count(v.name()));
						s.value = variable(v.name(),stack[v.name()].back(),v.width());
					}

					// fill in φ-function parameters in successor
					if(s.bblock.lock())
					{
						bblock_loc succ = s.bblock.lock();

						succ->mutate_mnemonics([&](vector<mnemonic> &ms)
					{
						auto iord = find_if(succ->incoming().begin(),succ->incoming().end(),[&](const ctrans &ct) { return ct.bblock.lock() == bb; });
						assert(iord != succ->incoming().end());
						unsigned int ord = distance(succ->incoming().begin(),iord);

						if(ms.size() && ms.front().opcode == "internal-phis")
						{
							mnemonic &mne = ms.front();

							for(instr &i: mne.instructions)
							{
								assert(i.function == instr::Phi && i.left.is_variable());
								int missing = ord - i.right.size() + 1;

								while(missing > 0)
								{
									i.right.emplace_back(undefined());
									--missing;
								}
								assert(stack.count(i.left.to_variable().name()));
								i.right[ord] = variable(i.left.to_variable().name(),stack[i.left.to_variable().name()].back(),i.left.to_variable().width());
							}
						}
					});
				}
			}
		});

		// for each successor s of b in the dominator tree
		//     rename(s)
		for(dtree_loc dom: dominance->tree[bb]->successors)
			rename(dom->basic_block.lock());

		// for each operation ‘‘x ← y op z’’ in bb
		//     and each φ-function ‘‘x ← φ(· · · )’’
		//     pop(stack[x])
		execute(bb,[&](const lvalue &left, instr::Function fn, const vector<rvalue> &right)
		{
			if(left.is_variable())
			{
				assert(stack.count(left.to_variable().name()));
				stack[left.to_variable().name()].pop_back();
			}
		});
	};

	rename(proc->entry);
}*/
