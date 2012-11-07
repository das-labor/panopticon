#include <algorithm>
#include <functional>
#include <cassert>
#include <iostream>

#include "procedure.hh"
#include "flowgraph.hh"

domtree::domtree(bblock_ptr b) : intermediate(0), basic_block(b) {}

procedure::procedure(void) : name("unnamed") {}

pair<procedure::iterator,procedure::iterator> procedure::rev_postorder(void) 
{
	if(basic_blocks.size() != rpo.size())
	{
		set<bblock_ptr> known;
		list<bblock_ptr> postorder;

		assert(entry);
		rpo.clear();

		//cout << "rpo: " << basic_blocks.size() << ", entry: " << entry->area() << endl;
		//for_each(basic_blocks.begin(),basic_blocks.end(),[](const bblock_ptr bb) { cout << bb->area() << endl; });

		function<void(bblock_ptr)> visit = [&](bblock_ptr bb)
		{
			//cout << "visit " << bb->area() << endl;
			basic_block::succ_iterator i,iend;
			
			tie(i,iend) = bb->successors();
			for_each(i,iend,[&](bblock_ptr s)
			{	
				//cout << "check " << s->area() << endl;
				if(known.insert(s).second)
					visit(s);
			});
			postorder.push_back(bb);
		};

		known.insert(entry);
		visit(entry);

		copy(postorder.rbegin(),postorder.rend(),inserter(rpo,rpo.begin()));
		assert(basic_blocks.size() == rpo.size());
	}

	return make_pair(rpo.begin(),rpo.end());
}

void procedure::insert_bblock(bblock_ptr m)
	{ rpo.clear(); basic_blocks.push_back(m); };

void procedure::remove_bblock(bblock_ptr m)
	{ rpo.clear(); basic_blocks.remove(m); };

bblock_ptr find_bblock(proc_ptr proc, addr_t a)
{
	procedure::iterator i = proc->basic_blocks.begin();

	while(i != proc->basic_blocks.end())
	{
		bblock_ptr bb = *i++;
		
		if(bb->area().includes(a))
			return bb;
	}

	return bblock_ptr(0);
}

void extend(proc_ptr proc, bblock_ptr block)
{
	procedure::iterator i = find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&block](const bblock_ptr &p) 
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
			tie(block,post) = split(block,(*find_if(block->mnemonics().begin(),block->mnemonics().end(),[&tgt](const mne_cptr &m)
																	 { return m->area.includes(tgt->area().begin); }))->area.begin,false);
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

void merge(proc_ptr proc, bblock_ptr block)
{
	// Try to connect in/out edge from/to bb to/from addr. Returns true if bb was split
	auto connect = [&proc](bblock_ptr bb, ctrans &ct, bool out) -> bool
	{
		procedure::iterator i;
		addr_t addr = ct.constant()->val;
		guard_ptr g = ct.guard;
		bool ret = false;

		i = find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](const bblock_ptr p) { return p->area().includes(addr); });

		if(i == proc->basic_blocks.end()) return ret;
		bblock_ptr tgt = *i, old = *i;
		ctrans cs(g,bb);

		// split tgt if needed
		if((out ? tgt->area().begin : tgt->mnemonics().back()->area.begin) != addr)
		{
			bblock_ptr up;
			
			proc->remove_bblock(tgt);
			tie(up,tgt) = split(tgt,addr,!out);
			conditional_jump(up,tgt,!out ? g->negation() : guard_ptr(new guard()));
			proc->insert_bblock(up);
			proc->insert_bblock(tgt);
			
			ret = old == bb;
		}

		if(out)
		{
			if(bb == old)
				conditional_jump(bb,tgt,g);
			else
				tgt->insert_incoming(cs);
		}
		else
		{
			if(bb == old)
				conditional_jump(tgt,bb,g);
			else
				tgt->insert_outgoing(cs);
		}

		if(bb != old)
			ct.bblock = tgt;
		else
		{
			if(out)
				old->remove_outgoing(ct.value);
			else
				old->remove_incoming(ct.value);
		}

		return ret;
	};
	
	std::set<bblock_ptr> done;
	basic_block::in_iterator j,jend;
	basic_block::out_iterator k,kend;

	proc->insert_bblock(block);
	while(true)
	{
		procedure::iterator i;
		bblock_ptr bb;
		
		i = find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&done](const bblock_ptr p) { return done.count(p) == 0; });

		if(i != proc->basic_blocks.end())
		{
			bb = *i;

			tie(j,jend) = bb->incoming();
			while(j != jend) 
			{ 
				ctrans &ct(*j++);
				if(!ct.bblock && ct.constant())
					if(connect(bb,ct,false))
						continue;
			}
			
			tie(k,kend) = bb->outgoing();
			while(k != kend)
			{
				ctrans &ct(*k++);
				if(!ct.bblock && ct.constant())
					if(connect(bb,ct,true))
						continue;
			}

			done.insert(bb);
		}
		else
			break;
	}

	tie(j,jend) = block->incoming();
	if(distance(j,jend) == 1 && j->guard->relations.empty() && j->bblock && j->bblock->area().end == block->area().begin)
	{
		tie(k,kend) = j->bblock->outgoing();
		if(distance(k,kend) == 1)
		{
			proc->remove_bblock(block);
			proc->remove_bblock(j->bblock);
			block = merge(j->bblock,block);
			proc->insert_bblock(block);
		}
	}

	tie(k,kend) = block->outgoing();
	if(distance(k,kend) == 1 && k->guard->relations.empty() && k->bblock && block->area().end == k->bblock->area().begin)
	{
		tie(j,jend) = k->bblock->incoming();
		if(distance(j,jend) == 1)
		{
			proc->remove_bblock(block);
			proc->remove_bblock(j->bblock);
			block = merge(block,k->bblock);
			proc->insert_bblock(block);
		}
	}
}

string graphviz(proc_ptr proc)
{
	flow_ptr f(new flowgraph());

	f->procedures.insert(proc);
	return graphviz(f);
}
