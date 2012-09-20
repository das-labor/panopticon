#include <algorithm>
#include <functional>
#include <cassert>
#include <iostream>

#include "procedure.hh"
#include "flowgraph.hh"

domtree::domtree(bblock_ptr b) : intermediate(0), basic_block(b) {}

procedure::procedure(void) {}

pair<procedure::iterator,procedure::iterator> procedure::rev_postorder(void) 
{
	if(basic_blocks.size() != rpo.size())
	{
		set<bblock_ptr> known;
		list<bblock_ptr> postorder;

		rpo.clear();

		function<void(bblock_ptr)> visit = [&](bblock_ptr bb)
		{
			basic_block::succ_iterator i,iend;
			
			tie(i,iend) = bb->successors();
			for_each(i,iend,[&](bblock_ptr s)
			{	
				if(known.insert(s).second)
					visit(s);
			});
			postorder.push_back(bb);
		};
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

pair<procedure::iterator,procedure::iterator> procedure::all(void) 
	{ return make_pair(basic_blocks.begin(),basic_blocks.end()); };

bblock_ptr find_bblock(proc_ptr proc, addr_t a)
{
	procedure::iterator i,e;
	
	tie(i,e) = proc->all();

	while(i != e)
	{
		bblock_ptr bb = *i++;
		if(bb->addresses().includes(a))
			return bb;
	}

	return bblock_ptr(0);
}

pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, const mne_cptr cur_mne, const mne_cptr prev_mne, bblock_ptr prev_bb, guard_ptr g)
{
	// if `prev_mne' isn't the last statement in its basic block, split the bb.
	if(prev_bb && !prev_bb->mnemonics().empty() && prev_bb->mnemonics().back()->addresses != prev_mne->addresses)
	{
		auto shreds = split(prev_bb,prev_mne->addresses.end,true);

		proc->insert_bblock(shreds.second);
		proc->insert_bblock(shreds.first);
		proc->remove_bblock(prev_bb);
		
		prev_bb = shreds.first;
	}

	// procedure and basic block occupying `cur_mne'
	bblock_ptr cur_bb = find_bblock(proc,cur_mne->addresses.begin);
	
	// `cur_mne' was disassembled previously 
	if(cur_bb)
	{
		if(cur_bb->addresses().begin == cur_mne->addresses.begin)			// refers to the start, connect `source' to `target' in the CFG
		{
			conditional_jump(prev_bb,cur_bb,g);
		}
		else 																																	// referes into the `target'. split target into two bb
		{
			auto shreds = split(cur_bb,cur_mne->addresses.begin,false);

			if(prev_bb == cur_bb)
				conditional_jump(shreds.second,shreds.second,g);
			else
				conditional_jump(prev_bb,shreds.second,g);
			
			proc->insert_bblock(shreds.second);
			proc->insert_bblock(shreds.first);
			proc->remove_bblock(cur_bb);
			cur_bb = shreds.second;
		}
		return make_pair(true,cur_bb);
	}
	else 																																		// fresh (unoccupyed) bytes. disassemble!
	{
		basic_block::succ_iterator j,jend;
		basic_block::indir_iterator k,kend;

		tie(j,jend) = prev_bb->successors();
		tie(k,kend) = prev_bb->indirect();

		/*
		 * if `prev_bb' has no succeeding basic blocks yet and this instruction 
		 * is right after it (in terms of memory addresses), extend `prev_bb' to
		 * include this instruction. Otherwise start new basic block.
		 */
		if(j == jend && k == kend && (prev_bb->mnemonics().empty() || prev_bb->mnemonics().back()->addresses.end == cur_mne->addresses.begin)) 
		{
			prev_bb->append_mnemonic(cur_mne);
			return make_pair(false,prev_bb);
		}
		else
		{
			bblock_ptr bb(new basic_block());
				
			bb->append_mnemonic(cur_mne);
			proc->insert_bblock(bb);
			conditional_jump(prev_bb,bb,g);
			
			return make_pair(false,bb);
		}
	}
}

pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, const mne_cptr cur_mne, bblock_ptr cur_bb, value_ptr v, guard_ptr g)
{
	// if `cur_mne' isn't the last statement in its basic block, split the bb.
	if(cur_bb && !cur_bb->mnemonics().empty() && cur_bb->mnemonics().back()->addresses != cur_mne->addresses)
	{
		auto shreds = split(cur_bb,cur_mne->addresses.end,true);

		proc->insert_bblock(shreds.second);
		proc->insert_bblock(shreds.first);
		proc->remove_bblock(cur_bb);
		
		cur_bb = shreds.first;
	}

	indirect_jump(cur_bb,v,g);
	return make_pair(false,cur_bb); // TODO return actual prev_known
}

string graphviz(proc_ptr proc)
{
	flow_ptr f(new flowgraph());

	f->procedures.insert(proc);
	return graphviz(f);
}
