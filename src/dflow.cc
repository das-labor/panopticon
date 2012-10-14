#include <algorithm>
#include <cassert>
#include <list>
#include <map>
#include <iostream>

#include "dflow.hh"

dom_ptr dominance_tree(proc_ptr proc)
{
	procedure::iterator i,iend;
	dom_ptr ret(new dom);
	
	if(!proc || !proc->entry)
		return ret;

	// dominance tree
	ret->root = ret->tree[proc->entry] = dtree_ptr(new domtree(proc->entry));
	ret->root->intermediate = ret->root;

	bool mod;
	do
	{	
		mod = false;
		tie(i,iend) = proc->rev_postorder();

		for_each(next(i),iend,[&](bblock_ptr bb)
		{
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
							procedure::iterator k,kend;
							tie(k,kend) = proc->rev_postorder();
							
							return distance(k,find(k,kend,d->basic_block)); 
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
		});
	} while(mod);

	ret->root->intermediate = dtree_ptr();

	// dominance frontiers
	tie(i,iend) = proc->rev_postorder();
	for_each(i,iend,[&](bblock_ptr bb)
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

live_ptr liveness(proc_ptr proc)
{	
	procedure::iterator i,iend;
	live_ptr ret(new live());

	// build global names and blocks that use them
	tie(i,iend) = proc->rev_postorder();
	for_each(i,iend,[&](bblock_ptr bb)
	{
		for_each(bb->instructions().begin(),bb->instructions().end(),[&](instr_cptr i)
		{
			if(i->function != instr::Phi)
			{
				for_each(i->arguments.begin(),i->arguments.end(),[&](value_ptr v)
				{
					shared_ptr<variable> w;

					if((w = dynamic_pointer_cast<variable>(v)))
					{
						ret->names.insert(w->nam);
						if(!ret->varkill[bb].count(w->nam))
							ret->uevar[bb].insert(w->nam);
					}
				});
	
				ret->varkill[bb].insert(i->assigns->nam);
				ret->names.insert(i->assigns->nam);
				ret->usage[i->assigns->nam].insert(bb);
			}
		});
	});

	bool mod;

	do
	{
		mod = false;

		tie(i,iend) = proc->rev_postorder();
		for_each(i,iend,[&](bblock_ptr bb)
		{
			set<name> old_liveout = ret->liveout[bb];
			basic_block::succ_iterator j,jend;
			
			ret->liveout[bb].clear();
			tie(j,jend) = bb->successors();
			
			// LiveOut = \_/ (UEVar \/ (LiveOut /\ !VarKill))
			// 					 succ
			for_each(j,j,[&](bblock_ptr s)
				{	ret->liveout[bb] = set_union(ret->liveout[bb],set_union(ret->uevar[s],set_intersection(ret->liveout[s],set_difference(ret->names,ret->varkill[s])))); });

			mod |= old_liveout != ret->liveout[bb];
		});
	} 
	while(mod);

	return ret;
}

void ssa(proc_ptr proc, dom_ptr dominance, live_ptr live)
{
	// insert phi
	for_each(live->names.begin(),live->names.end(),[&](const name &name)
	{
		set<bblock_ptr> &worklist(live->usage[name]);

		while(!worklist.empty())
		{
			bblock_ptr bb = *worklist.begin();

			worklist.erase(worklist.begin());
			for_each(dominance->tree[bb]->frontiers.begin(),dominance->tree[bb]->frontiers.end(),[&](dtree_ptr df)
			{
				if(none_of(df->basic_block->instructions().begin(),df->basic_block->instructions().end(),[&](instr_cptr i)
					{	return i->function == instr::Phi && i->assigns->nam.base == name.base; }))
				{
					df->basic_block->prepend_instr(instr_ptr(new instr(instr::Phi,"ϕ",var_ptr(new variable(name,0)),{})));
					worklist.insert(df->basic_block);
				}
			});
		}
	});

	// rename variables
	map<string,int> counter;
	map<string,list<int>> stack;

	for_each(live->names.begin(),live->names.end(),[&](name n) 
	{ 
		counter.insert(make_pair(n.base,1));
		stack.insert(make_pair(n.base,list<int>({0})));
	});
	
	auto new_name = [&](name n) -> int
	{
		int i = counter[n.base]++;
		
		stack[n.base].push_back(i);
		return i;
	};

	function<void(bblock_ptr bb)> rename = [&](bblock_ptr bb)
	{
		size_t sz = bb->instructions().size(), pos = 0;
		const instr_ptr *i = bb->instructions().data();
		basic_block::out_iterator j,jend;
		function<void(relation &)> rename_guard = [&](relation &rel)
		{
			shared_ptr<variable> w;
			
			if((w = dynamic_pointer_cast<variable>(rel.operand1)))
				w->nam.subscript = stack[w->nam.base].back();
					
			if((w = dynamic_pointer_cast<variable>(rel.operand2)))
				w->nam.subscript = stack[w->nam.base].back();
		};
		
		// rename arguments
		while(pos < sz)
		{
			if(i[pos]->function != instr::Phi)
			{
				for_each(i[pos]->arguments.begin(),i[pos]->arguments.end(),[&](value_ptr v)
				{
					shared_ptr<variable> w;

					if((w = dynamic_pointer_cast<variable>(v)))
						w->nam.subscript = stack[w->nam.base].back();
				});
			}	
			i[pos]->assigns->nam.subscript = new_name(i[pos]->assigns->nam);
			++pos;
		}

		tie(j,jend) = bb->outgoing();
		for_each(j,jend,[&](ctrans s)
		{
			// rename incoming guards
			for_each(s.guard->relations.begin(),s.guard->relations.end(),rename_guard);
		
			if(s.variable() && s.variable()->nam.subscript == -1)
				s.variable()->nam.subscript = stack[s.variable()->nam.base].back();

			if(s.bblock)
			{
				size_t sz = s.bblock->instructions().size(), pos = 0;
				const instr_ptr *i = s.bblock->instructions().data();

				while(pos < sz)
				{
					if(i[pos]->function == instr::Phi)
					{
						bool add = none_of(i[pos]->arguments.begin(),i[pos]->arguments.end(),[&](value_ptr v)
							{ var_ptr w; return (w = dynamic_pointer_cast<variable>(v)) && w->nam.subscript == stack[w->nam.base].back(); });

						if(add)
							i[pos]->arguments.push_back(value_ptr(new variable(name(i[pos]->assigns->nam.base,stack[i[pos]->assigns->nam.base].back()),0)));
					}
					++pos;
				}
			}
		});

		for_each(dominance->tree[bb]->successors.begin(),dominance->tree[bb]->successors.end(),[&](dtree_ptr dom)
			{ rename(dom->basic_block); });
		
		sz = bb->instructions().size(); pos = 0;
		i = bb->instructions().data();
		while(pos < sz)
			stack[i[pos++]->assigns->nam.base].pop_back();
	};

	rename(proc->entry);
}
