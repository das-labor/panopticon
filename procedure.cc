#include <algorithm>
#include <functional>
#include <cassert>
#include <iostream>

#include "procedure.hh"

domtree::domtree(bblock_ptr b) : intermediate(0), basic_block(b) {}

procedure::procedure(void) {}
/*procedure::procedure(list<bblock_ptr> &e)
{ 
	if(e.size())
	{
		set<bblock_ptr> known;
		vector<bblock_ptr> po;

		entry = e[0]; 
		basic_blocks.reserve(e.size());
		po.reserve(e.size());

		function<void(bblock_ptr)> visit = [&,this](bblock_ptr bb)
		{
			for_each(bb->successors.begin(),bb->successors.end(),[&](bblock_ptr s)
			{	
				if(known.insert(s).second)
					visit(s);
			});
			po.push_back(bb);

			for_each(bb->instructions.begin(),bb->instructions.end(),[this](pair<instr_ptr,addr_t> ip)
			{	
				names.insert(ip.first->assigns->nam);
				for_each(ip.first->operands.begin(),ip.first->operands.end(),[this](value_ptr v)
				{
					shared_ptr<variable> w;
					if((w = dynamic_pointer_cast<variable>(v)))
						names.insert(w->nam);
				});
			});
		};
		visit(entry);

		copy(po.rbegin(),po.rend(),inserter(basic_blocks,basic_blocks.begin()));
		assert(basic_blocks.size() == e.size());
	}
}*/

void procedure::insert_bblock(bblock_ptr m)
	{ basic_blocks.push_back(m); };

void procedure::remove_bblock(bblock_ptr m)
 { basic_blocks.remove(m); };

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
	if(prev_bb)
	{
		basic_block::iterator i,e;

		tie(i,e) = prev_bb->mnemonics();
		
		if(i != e && (*(--e))->addresses != prev_mne->addresses)
		{
			cout << " split prev_bb " << prev_bb.get() << endl;
			auto shreds = split(prev_bb,prev_mne->addresses.end,true);

			//cout << " into " <<  shreds.first.start << ":" << shreds.first.end;
			//cout << " and " <<  shreds.second.start << ":" << shreds.second.end << endl;
			
			proc->insert_bblock(shreds.second);
			proc->insert_bblock(shreds.first);
			proc->remove_bblock(prev_bb);
			
			prev_bb = shreds.first;
			//cout << " new src_bb is " << src_bb->start << ":" << src_bb->end << dec << endl;
		}
	}

	// procedure and basic block occupying `cur_mne'
	bblock_ptr cur_bb = find_bblock(proc,cur_mne->addresses.begin);
	
	// `cur_mne' was disassembled previously 
	if(cur_bb)
	{
		cout << "prev known...";
		if((*cur_bb->mnemonics().first)->addresses == cur_mne->addresses)			// refers to the start, connect `source' to `target' in the CFG
		{
			cout << " connect to existing basic block" << endl;
			// TODO add guards
			branch(prev_bb,cur_bb,g);
		}
		else 													// referes into the `target'. split target into two bb
		{
			cout << " split basic block " << cur_bb.get() << endl;
			auto shreds = split(cur_bb,cur_mne->addresses.begin,false);
			

			if(prev_bb == cur_bb)
				branch(shreds.second,shreds.second,g);
			else
				branch(cur_bb,shreds.second,g);
			
			proc->insert_bblock(shreds.second);
			proc->insert_bblock(shreds.first);
			proc->remove_bblock(cur_bb);
			cur_bb = shreds.second;
		}
		cout << endl;
		return make_pair(true,cur_bb);
	}
	else // fresh (unoccupyed) bytes. disassemble!
	{
		basic_block::iterator i,iend;
		basic_block::succ_iterator j,jend;

		tie(i,iend) = prev_bb->mnemonics();
		tie(j,jend) = prev_bb->successors();

		cout << "fresh bytes..." << hex << cur_mne->addresses.begin;// << " " << src_bb->end;
		/*
		 * if `prev_bb' has no succeeding basic blocks yet and this instruction 
		 * is right after it (in terms of memory addresses), extend `prev_bb' to
		 * include this instruction. Otherwise start new basic block.
		 */
		if(j == jend && (i == iend || (*--iend)->addresses.end == cur_mne->addresses.begin)) 
		{
			prev_bb->append_mnemonic(cur_mne);
			cout << " extend basic block to " << prev_bb.get() << endl;
			
			cout << endl;
			return make_pair(false,prev_bb);
		}
		else
		{
			bblock_ptr bb(new basic_block());
			cout << " new basic block " << bb.get() << endl;
				
			bb->append_mnemonic(cur_mne);
			proc->insert_bblock(bb);
			branch(prev_bb,bb,g);
			
			cout << endl;
			return make_pair(false,bb);
		}
	}
};
