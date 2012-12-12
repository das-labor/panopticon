#include <algorithm>
#include <cassert>
#include <list>
#include <map>
#include <iostream>

#include <dflow.hh>
#include <value.hh>

using namespace po;

dom_ptr po::dominance_tree(proc_ptr proc)
{
	dom_ptr ret(new dom);
	
	if(!proc || !proc->entry)
		return ret;

	// dominance tree
	ret->root = ret->tree[proc->entry] = dtree_ptr(new domtree(proc->entry));
	ret->root->intermediate = ret->root;

	std::list<bblock_ptr> rpo_lst;
	proc->rev_postorder([&](bblock_ptr bb) { rpo_lst.push_back(bb); });

	bool mod;
	do
	{	
		bool skip = true;
		mod = false;
		for(bblock_ptr bb: rpo_lst)
		{
			// skip the first
			if(skip)
			{
				skip = false;
				continue;
			}

			dtree_ptr newidom(0);
			basic_block::pred_iterator j,jend;

			tie(j,jend) = bb->predecessors();
			for_each(j,jend,[&](bblock_ptr p)
			{
				if(ret->tree.count(p))
				{
					if(!newidom)
					{
						newidom = ret->tree[p];
					}
					else if(ret->tree[p]->intermediate)
					{
						// Intersect
						dtree_ptr f1 = ret->tree[p], f2 = newidom;
						auto rpo = [&](dtree_ptr d) 
						{ 
							return distance(rpo_lst.begin(),find(rpo_lst.begin(),rpo_lst.end(),d->basic_block));
						};

						while(f1 != f2)
						{
							int d1, d2 = rpo(f2);

							while((d1 = rpo(f1)) > d2)
								f1 = f1->intermediate;
							while(d2 > d1)
							{
								f2 = f2->intermediate;
								d2 = rpo(f2);
							}
						}

						newidom = f1;
					}
				}
			});

			if(!ret->tree.count(bb))
				ret->tree[bb] = dtree_ptr(new domtree(bb));

			if(ret->tree[bb]->intermediate != newidom)
			{
				newidom->successors.insert(ret->tree[bb]);
				ret->tree[bb]->intermediate	= newidom;
				mod = true;
			}
		}
	} while(mod);

	ret->root->intermediate = dtree_ptr();

	// dominance frontiers
	proc->rev_postorder([&](bblock_ptr bb)
	{
		basic_block::pred_iterator j,jend;
		tie(j,jend) = bb->predecessors();

		if(distance(j,jend) >= 2)
		{
			for_each(j,jend,[&](bblock_ptr p)
			{
				dtree_ptr runner = ret->tree[p];

				while(runner !=  ret->tree[bb]->intermediate)
				{
					runner->frontiers.insert(ret->tree[bb]);
					runner = runner->intermediate;
				}
			});
		}
	});

	return ret;
}

live_ptr po::liveness(proc_cptr proc)
{	
	live_ptr ret(new live());

	auto collect = [&](const rvalue &v, bblock_ptr bb)
	{
		if(v.is_variable())
		{
			ret->names.insert(v.variable());
			if(!ret->varkill[bb].count(v.variable()))
				ret->uevar[bb].insert(v.variable());
		}
	};

	// build global names and blocks that use them
	for(bblock_ptr bb: proc->basic_blocks)
	{
		execute(bb,[&](const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)
		{
			for(const rvalue &v: right)
				collect(v,bb);
	
			if(left.is_variable())
			{
				ret->varkill[bb].insert(left.variable());
				ret->names.insert(left.variable());
				ret->usage[left.variable()].insert(bb);
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

			for(const relation &rel: ct.guard->relations)
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

		for(bblock_ptr bb: proc->basic_blocks)
		{
			std::set<name> old_liveout = ret->liveout[bb];
			basic_block::succ_iterator j,jend;
			
			ret->liveout[bb].clear();
			tie(j,jend) = bb->successors();
			
			// LiveOut = \_/ (UEVar \/ (LiveOut /\ !VarKill))
			// 					 succ
			for_each(j,j,[&](bblock_ptr s)
				{	ret->liveout[bb] = set_union(ret->liveout[bb],set_union(ret->uevar[s],set_intersection(ret->liveout[s],set_difference(ret->names,ret->varkill[s])))); });

			mod |= old_liveout != ret->liveout[bb];
		}
	} 
	while(mod);

	return ret;
}

void po::ssa(proc_ptr proc, dom_ptr dominance, live_ptr live)
{
	std::set<name> globals;

	for(const std::pair<bblock_cptr,std::set<name>> &s: live->uevar)
		globals = set_union(globals,s.second);

	if(live->liveout[proc->entry].size())
	{
		std::cout << "uninitialized vars: ";
		for(const name &n: live->liveout[proc->entry])
			std::cout << n.base << " ";
		std::cout << std::endl;
		assert(false);
	}

	// insert phi
	for(const name &n: globals)
	{
		std::set<bblock_cptr> &worklist(live->usage[n]);

		while(!worklist.empty())
		{
			bblock_cptr bb = *worklist.begin();

			worklist.erase(worklist.begin());
			for(dtree_ptr df: dominance->tree[bb]->frontiers)
			{
				bool has_phi = false;
				execute(df->basic_block,[&](lvalue left, instr::Function fn, const std::vector<rvalue> &right)
				{	
					has_phi = has_phi || (fn == instr::Phi && left.is_variable() && left.variable().name() == n.base); 
				});

				if(!has_phi)
				{
					df->basic_block->mutate_mnemonics([&](std::vector<mnemonic> &ms)
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
	std::map<std::string,int> counter;
	std::map<std::string,std::list<int>> stack;

	for(const std::string &n: live->names) 
	{ 
		counter.insert(std::make_pair(n,0));
		stack.insert(std::make_pair(n,std::list<int>({})));
	}
	
	auto new_name = [&](const std::string &n) -> int
	{
		assert(stack.count(n));
		int i = counter[n]++;

		stack[n].push_back(i);
		return i;
	};

	// rename ssa vars in a bblock
	std::function<void(bblock_ptr bb)> rename = [&](bblock_ptr bb)
	{
		// for each φ-function in b, ‘‘x ← φ(· · · )’‘
		//     rewrite x as new_name(x)
		rewrite(bb,[&](lvalue &left, instr::Function fn, std::vector<rvalue> &right)
		{
			if(fn == instr::Phi)
			{
				assert(left.is_variable());
				left = variable(left.variable().name(),new_name(left.variable().name()),left.variable().width());
			}
		});

		// for each mnemonic ‘‘opcode y, z’’ in bb and
		// for each operation ‘‘x ← y op z’’ in bb
		//     rewrite y with subscript top(stack[y])
		//     rewrite z with subscript top(stack[z])
		//     rewrite x as new_name(x)
		bb->mutate_mnemonics([&](std::vector<mnemonic> &ms)
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
						assert(stack.count(v.variable().name()));
						v = variable(v.variable().name(),stack[v.variable().name()].back(),v.variable().width());
					}
				}

				while(i_instr < sz_instr)
				{
					instr &instr = ary_instr[i_instr++];
					lvalue &left = instr.left;
					po::instr::Function fn = instr.function;
					std::vector<rvalue> &right = instr.right;

					if(fn != instr::Phi)
					{
						unsigned int ri = 0;

						while(ri < right.size())
						{
							const rvalue &v = right[ri];

							if(v.is_variable())
							{
								assert(stack.count(v.variable().name()));
								right[ri] = variable(v.variable().name(),stack[v.variable().name()].back(),v.variable().width());
							}
							++ri;
						}
					
						if(left.is_variable())
							left = variable(left.variable().name(),new_name(left.variable().name()),left.variable().width());
					}
				}
			}
		});

		// for each successor of b in the cfg
		// 		 rewrite variables in ctrans
		//     fill in φ-function parameters
		bb->mutate_outgoing([&](std::list<ctrans> &out)
		{
				std::list<ctrans> new_out;

				for(ctrans &s: out)
				{
					// rewrite vars in relations
					for(relation &rel: s.guard->relations)
					{
						if(rel.operand1.is_variable())
						{
							const variable o1 = rel.operand1.variable();
							assert(stack.count(o1.name()));
							rel.operand1 = variable(o1.name(),stack[o1.name()].back(),o1.width());
						}
						if(rel.operand2.is_variable())
						{
							const variable o2 = rel.operand2.variable();
							assert(stack.count(o2.name()));
							rel.operand2 = variable(o2.name(),stack[o2.name()].back(),o2.width());
						}
					}

					// rewrite symbolic target in ctrans
					if(s.value.is_variable())
					{
						const variable v = s.value.variable();
						assert(stack.count(v.name()));
						s.value = variable(v.name(),stack[v.name()].back(),v.width());
					}

					// fill in φ-function parameters in successor
					if(s.bblock)
					{
						bblock_ptr succ = s.bblock;

						succ->mutate_mnemonics([&](std::vector<mnemonic> &ms)
					{
						auto iord = std::find_if(succ->incoming().begin(),succ->incoming().end(),[&](const ctrans &ct) { return ct.bblock == bb; });
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
								assert(stack.count(i.left.variable().name()));
								i.right[ord] = variable(i.left.variable().name(),stack[i.left.variable().name()].back(),i.left.variable().width());
							}
						}
					});
				}
			}
		});

		// for each successor s of b in the dominator tree
		//     rename(s)
		for(dtree_ptr dom: dominance->tree[bb]->successors)
			rename(dom->basic_block);
		
		// for each operation ‘‘x ← y op z’’ in bb
		//     and each φ-function ‘‘x ← φ(· · · )’’
		//     pop(stack[x])
		execute(bb,[&](const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)
		{
			if(left.is_variable()) 
			{
				assert(stack.count(left.variable().name()));
				stack[left.variable().name()].pop_back();
			}
		});
	};

	rename(proc->entry);
}
