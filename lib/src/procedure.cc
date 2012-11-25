#include <algorithm>
#include <functional>
#include <cassert>
#include <iostream>

#include <procedure.hh>
#include <flowgraph.hh>

using namespace po;

domtree::domtree(bblock_ptr b) : intermediate(0), basic_block(b) {}

procedure::procedure(void) : name("unnamed") {}

void procedure::rev_postorder(std::function<void(bblock_ptr bb)> fn) const
{
	std::set<bblock_ptr> known;
	std::list<bblock_ptr> postorder;

	assert(entry);

	//cout << "rpo: " << basic_blocks.size() << ", entry: " << entry->area() << endl;
	//for_each(basic_blocks.begin(),basic_blocks.end(),[](const bblock_ptr bb) { cout << bb->area() << endl; });

	std::function<void(bblock_ptr)> visit = [&](bblock_ptr bb)
	{
	//	cout << "visit " << bb->area() << endl;
		basic_block::succ_iterator i,iend;
		
		tie(i,iend) = bb->successors();
		for_each(i,iend,[&](bblock_ptr s)
		{	
		//	cout << "check " << s->area() << endl;
			if(known.insert(s).second)
				visit(s);
		});
		postorder.push_back(bb);
	};

	known.insert(entry);
	visit(entry);
	assert(basic_blocks.size() == postorder.size());
	for_each(postorder.rbegin(),postorder.rend(),fn);
}

bblock_ptr po::find_bblock(proc_ptr proc, addr_t a)
{
	auto i = proc->basic_blocks.begin();

	while(i != proc->basic_blocks.end())
	{
		bblock_ptr bb = *i++;
		
		if(bb->area().includes(a))
			return bb;
	}

	return bblock_ptr(0);
}

void po::extend(proc_ptr proc, bblock_ptr block)
{
	auto i = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&block](const bblock_ptr &p) 
		{ return p->area().overlap(block->area()); });

	if(i !=  proc->basic_blocks.end())
	{
		/*
		 * Overlap:
		 *
		 *  block
		 * +-----+
		 * | pre |    tgt
		 * +- - -+  +-----+
		 * | mid |  |     |
		 * |     |  |     |
		 * +- - -+  +-----+
		 * |post |
		 * +-----+
		 */
		bblock_ptr tgt = *i;
		bblock_ptr pre, post;

		// pre
		if(tgt->area().begin > block->area().begin)
		{
			tie(pre,block) = split(block,tgt->area().begin,false);
			unconditional_jump(pre,tgt);
		}

		// post
		if(tgt->area().end < block->area().end)
		{
			tie(block,post) = split(block,std::find_if(block->mnemonics().begin(),block->mnemonics().end(),[&tgt](const mnemonic &m)
																	 { return m.area.includes(tgt->area().begin); })->area.begin,false);
			unconditional_jump(tgt,post);
		}

		// mid
		// TODO refine mnemonics
			
		if(pre) merge(proc,pre);
		if(post) merge(proc,post);
	}
	else
		merge(proc,block);
}

void po::merge(proc_ptr proc, bblock_ptr block)
{
	assert(proc && block);

	// Try to connect in/out edge from/to bb to/from addr. Returns true if bb was split
	auto connect = [&proc](bblock_ptr bb, ctrans &ct, bool out) -> bool
	{
		assert(bb && ct.value.is_constant() && !ct.bblock);

		addr_t addr = ct.value.constant().value();
		guard_ptr g = ct.guard;
		bool ret = false;

		auto i = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](const bblock_ptr p) { return p->area().includes(addr); });

		if(i == proc->basic_blocks.end()) return ret;
		bblock_ptr tgt = *i, old = *i;

		// split tgt if needed
		if((out ? tgt->area().begin : tgt->mnemonics().back().area.begin) != addr)
		{
			bblock_ptr up;
			
			proc->basic_blocks.erase(tgt);
			tie(up,tgt) = split(tgt,addr,!out);

			if(!out)
			{
				up->mutate_outgoing([&](std::list<ctrans> o)
				{
					o.erase(std::find_if(o.begin(),o.end(),[&](const ctrans &c) { return c.bblock == tgt; }));
				});
				
				tgt->mutate_incoming([&](std::list<ctrans> o)
				{
					o.erase(std::find_if(o.begin(),o.end(),[&](const ctrans &c) { return c.bblock == up; }));
				});

				conditional_jump(up,tgt,g->negation());
			}
			proc->basic_blocks.insert(up);
			proc->basic_blocks.insert(tgt);
			
			ret = old == bb;
		}

		// no loop
		if(bb != old)
		{
			if(out)
			{
				bb->mutate_outgoing([&](std::list<ctrans> &outs)
				{
					outs.erase(std::find_if(outs.begin(),outs.end(),[&](const ctrans &c) { return c.value == ct.value; }));
				});

				conditional_jump(bb,tgt,ct.guard);
			}
			else
			{
				bb->mutate_incoming([&](std::list<ctrans> &ins)
				{
					ins.erase(std::find_if(ins.begin(),ins.end(),[&](const ctrans &c) { return c.value == ct.value; }));
				});

				conditional_jump(tgt,bb,ct.guard);
			}
		}
		else // loop
		{
			if(out)
			{
				tgt->mutate_outgoing([&](std::list<ctrans> &outs)
				{
					outs.erase(std::find_if(outs.begin(),outs.end(),[&](const ctrans &c) { return c.value == ct.value; }));
				});
			}
			else
			{
				tgt->mutate_incoming([&](std::list<ctrans> &ins)
				{
					ins.erase(std::find_if(ins.begin(),ins.end(),[&](const ctrans &c) { return c.value == ct.value; }));
				});
			}
			conditional_jump(tgt,tgt,ct.guard);
		}

		return ret;
	};
	
	std::set<bblock_ptr> done;
	
	proc->basic_blocks.insert(block);
	while(true)
	{
		bblock_ptr bb;
		auto i = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&done](const bblock_ptr p) { return done.count(p) == 0; });

		if(i != proc->basic_blocks.end())
		{
			bb = *i;

			bb->mutate_incoming([&](std::list<ctrans> &in)
			{
				auto j = in.begin();
				while(j != in.end()) 
				{ 
					ctrans &ct(*j++);
					if(!ct.bblock && ct.value.is_constant())
						if(connect(bb,ct,false))
							continue;
				}
			});
			
			bb->mutate_outgoing([&](std::list<ctrans> &out)
			{
				auto k = out.begin();
				while(k != out.end())
				{
					ctrans &ct(*k++);
					if(!ct.bblock && ct.value.is_constant())
						if(connect(bb,ct,true))
							continue;
				}
			});

			done.insert(bb);
		}
		else
			break;
	}

	auto j = block->incoming().begin();
	if(block->incoming().size() == 1 && j->guard->relations.empty() && j->bblock && j->bblock->area().end == block->area().begin)
	{
		if(j->bblock->outgoing().size() == 1)
		{
			proc->basic_blocks.erase(block);
			proc->basic_blocks.erase(j->bblock);
			block = merge(j->bblock,block);
			proc->basic_blocks.insert(block);
		}
	}

	auto k = block->outgoing().begin();
	if(block->outgoing().size() && k->guard->relations.empty() && k->bblock && block->area().end == k->bblock->area().begin)
	{
		if(k->bblock->incoming().size() == 1)
		{
			proc->basic_blocks.erase(block);
			proc->basic_blocks.erase(j->bblock);
			block = merge(block,k->bblock);
			proc->basic_blocks.insert(block);
		}
	}
}

std::string po::graphviz(proc_ptr proc)
{
	flow_ptr f(new flowgraph());

	f->procedures.insert(proc);
	return graphviz(f);
}

void po::call(proc_ptr from, proc_ptr to)
{
	assert(from && to);

	from->callees.insert(to);
	to->callers.insert(from);
}

void po::execute(proc_cptr proc,std::function<void(const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)> f)
{
	for(const bblock_ptr &bb: proc->basic_blocks)
	{
		size_t sz_mne = bb->mnemonics().size(), i_mne = 0;
		const mnemonic *ary_mne = bb->mnemonics().data();

		while(i_mne < sz_mne)
		{
			const mnemonic &mne = ary_mne[i_mne++];
			size_t sz_instr = mne.instructions.size(), i_instr = 0;
			const instr *ary_instr = mne.instructions.data();

			while(i_instr < sz_instr)
			{
				const instr &instr = ary_instr[i_instr++];

				f(instr.left,instr.function,instr.right);
			}
		}
	}
}
