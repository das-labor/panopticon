#include <algorithm>
#include <cassert>
#include <list>
#include <map>

#include "dflow.hh"

namespace dflow	{

void dominance_tree(cfg_ptr cfg)
{
	if(!cfg || !cfg->entry)
		return;

	// dominance tree
	cfg->entry->dominance = dtree_ptr(new domtree(cfg->entry));
	cfg->entry->dominance->intermediate = cfg->entry->dominance;

	bool mod;
	do
	{
		mod = false;
		for_each(next(cfg->basic_blocks.begin()),cfg->basic_blocks.end(),[&](bblock_ptr bb)
		{
			dtree_ptr newidom(0);

			// TODO assumes no previous run of dominance_tree()
			for_each(bb->predecessors.begin(),bb->predecessors.end(),[&](bblock_ptr p)
			{
				if(p->dominance)
				{
					if(!newidom)
					{
						newidom = p->dominance;
					}
					else if(p->dominance->intermediate)
					{
						// Intersect
						dtree_ptr f1 = p->dominance, f2 = newidom;
						auto rpo = [&](dtree_ptr d) 
							{ return distance(cfg->basic_blocks.begin(),find(cfg->basic_blocks.begin(),cfg->basic_blocks.end(),d->basic_block)); };

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

			if(!bb->dominance)
				bb->dominance = dtree_ptr(new domtree(bb));

			if(bb->dominance->intermediate != newidom)
			{
				newidom->successors.insert(bb->dominance);
				bb->dominance->intermediate	= newidom;
				mod = true;
			}
		});
	} while(mod);

	cfg->entry->dominance->intermediate = dtree_ptr();

	// dominance frontiers
	for_each(cfg->basic_blocks.begin(),cfg->basic_blocks.end(),[](bblock_ptr bb)
	{
		if(bb->predecessors.size() >= 2)
		{
			for_each(bb->predecessors.begin(),bb->predecessors.end(),[&bb](bblock_ptr p)
			{
				dtree_ptr runner = p->dominance;

				while(runner !=  bb->dominance->intermediate)
				{
					runner->frontiers.insert(bb->dominance);
					runner = runner->intermediate;
				}
			});
		}
	});
}

void liveness(cfg_ptr cfg)
{	
	// build global names and blocks that use them
	for_each(cfg->basic_blocks.begin(),cfg->basic_blocks.end(),[&](bblock_ptr bb)
	{
		for_each(bb->instructions.begin(),bb->instructions.end(),[&](pair<instr_ptr,addr_t> ip)
		{
			if(ip.first->opcode != instr::Phi)
			{
				for_each(ip.first->operands.begin(),ip.first->operands.end(),[&](value_ptr v)
				{
					shared_ptr<variable> w;

					if((w = dynamic_pointer_cast<variable>(v)))
					{
						cfg->names.insert(w->nam);
						if(!bb->varkill.count(w->nam))
							bb->uevar.insert(w->nam);
					}
				});
	
				bb->varkill.insert(ip.first->assigns->nam);
				cfg->names.insert(ip.first->assigns->nam);
				cfg->usage[ip.first->assigns->nam].insert(bb);
			}
		});
	});

	bool mod;

	do
	{
		mod = false;

		for_each(cfg->basic_blocks.begin(),cfg->basic_blocks.end(),[&cfg,&mod](bblock_ptr bb)
		{
			set<name> old_liveout = bb->liveout;
			
			bb->liveout.clear();
			
			// LiveOut = \_/ (UEVar \/ (LiveOut /\ !VarKill))
			// 					 succ
			for_each(bb->successors.begin(),bb->successors.end(),[&cfg,&bb](bblock_ptr s)
				{	bb->liveout = set_union(bb->liveout,set_union(s->uevar,set_intersection(s->liveout,set_difference(cfg->names,s->varkill)))); });

			mod |= old_liveout != bb->liveout;
		});
	} 
	while(mod);
}

set<name> set_difference(set<name> a, set<name> b)
{
	set<name> ret;
	set_difference(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}

set<name> set_union(set<name> a, set<name> b)
{
	set<name> ret;
	set_union(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}

set<name> set_intersection(set<name> a, set<name> b)
{
	set<name> ret;
	set_intersection(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}

void ssa(cfg_ptr cfg)
{
	// insert phi
	for_each(cfg->names.begin(),cfg->names.end(),[&](name name)
	{
		set<bblock_ptr> &worklist(cfg->usage[name]);

		while(!worklist.empty())
		{
			bblock_ptr bb = *worklist.begin();

			worklist.erase(worklist.begin());
			for_each(bb->dominance->frontiers.begin(),bb->dominance->frontiers.end(),[&](dtree_ptr df)
			{
				if(none_of(df->basic_block->instructions.begin(),df->basic_block->instructions.end(),[&](pair<instr_ptr,addr_t> ip)
					{	return ip.first->opcode == instr::Phi && ip.first->assigns->nam == name; }))
				{
					df->basic_block->instructions.emplace(df->basic_block->instructions.begin(),make_pair(instr_ptr(new instr(instr::Phi,"ϕ",name,
						{value_ptr(new variable(name)),value_ptr(new variable(name))})),0));
					worklist.insert(df->basic_block);
				}
			});
		}
	});

	// rename variables
	map<name,unsigned int> counter;
	map<name,list<unsigned int>> stack;

	for_each(cfg->names.begin(),cfg->names.end(),[&](name n) 
	{ 
		counter.insert(make_pair(n,1));
		stack.insert(make_pair(n,list<unsigned int>({0})));
	});
	
	auto new_name = [&](name n) -> unsigned int
	{
		int i = counter[n]++;
		
		stack[n].push_back(i);
		return i;
	};

	function<void(bblock_ptr bb)> rename = [&](bblock_ptr bb)
	{
		for_each(bb->instructions.begin(),bb->instructions.end(),[&](pair<instr_ptr,addr_t> p)
		{
			if(p.first->opcode != instr::Phi)
			{
				for_each(p.first->operands.begin(),p.first->operands.end(),[&](value_ptr v)
				{
					shared_ptr<variable> w;

					if((w = dynamic_pointer_cast<variable>(v)))
						w->nam.subscript = stack[w->nam].back();
				});
			}	
			p.first->assigns->nam.subscript = new_name(p.first->assigns->nam);
		});

		for_each(bb->outgoing.begin(),bb->outgoing.end(),[&](tuple<guard_ptr,bblock_ptr,bblock_ptr> s)
		{
			guard_ptr g = get<0>(s);

			for_each(g->relations.begin(),g->relations.end(),[&](relation &rel)
			{
				shared_ptr<variable> w;

				if((w = dynamic_pointer_cast<variable>(rel.operand1)))
					w->nam.subscript = stack[w->nam].back();
					
				if((w = dynamic_pointer_cast<variable>(rel.operand2)))
					w->nam.subscript = stack[w->nam].back();
			});

			function<void(bblock_ptr)> func = [&bb,&stack](bblock_ptr s)
			{
				int pos = distance(s->predecessors.begin(),s->predecessors.find(bb));
				
				for_each(s->instructions.begin(),s->instructions.end(),[&](pair<instr_ptr,addr_t> p)
				{
					if(p.first->opcode == instr::Phi)
					{
						shared_ptr<variable> w;
	
						if((w = dynamic_pointer_cast<variable>(p.first->operands[pos])))
							w->nam.subscript = stack[w->nam].back();
					}
				});
			};

			func(get<1>(s));
			if(get<2>(s))
				func(get<2>(s));
		});

		for_each(bb->dominance->successors.begin(),bb->dominance->successors.end(),[&](dtree_ptr dom)
			{ rename(dom->basic_block); });
		
		for_each(bb->instructions.begin(),bb->instructions.end(),[&](pair<instr_ptr,addr_t> p)
			{	stack[p.first->assigns->nam].pop_back(); });
	
	};

	rename(cfg->basic_blocks.front());
}

}; // namespace dflow
