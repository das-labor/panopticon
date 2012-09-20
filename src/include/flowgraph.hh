#ifndef FLOWGRAPH_HH
#define FLOWGRAPH_HH

#include <memory>
#include <set>

#include "procedure.hh"
#include "decoder.hh"
#include "dflow.hh"
#include "absinterp.hh"

using namespace std;

typedef shared_ptr<struct flowgraph> flow_ptr;

struct flowgraph
{
	set<proc_ptr> procedures;
	map<proc_ptr,dom_ptr> dominance;
	map<proc_ptr,live_ptr> liveness;
	map<proc_ptr,shared_ptr<map<bblock_ptr,taint_lattice>>> taint;
	map<proc_ptr,shared_ptr<map<bblock_ptr,cprop_lattice>>> cprop;
};

bool has_procedure(flow_ptr flow, addr_t entry);

template<typename token,typename tokiter>
flow_ptr disassemble(const decoder<token,tokiter> &main, vector<token> tokens, addr_t offset = 0xdfe, bool cf_sensitive = true)
{
	flow_ptr ret(new flowgraph());
	set<addr_t> call_targets;

	call_targets.insert(offset);

	while(!call_targets.empty())
	{
		auto h = call_targets.begin();
		addr_t tgt = *h;
		dom_ptr dom;
		live_ptr live;
		shared_ptr<map<bblock_ptr,taint_lattice>> taint;
		shared_ptr<map<bblock_ptr,cprop_lattice>> cprop;
		
		if(has_procedure(ret,tgt))
			continue;
		
		proc_ptr proc(new procedure());
		bblock_ptr entry(new basic_block());
		procedure::iterator i,iend;

		call_targets.erase(h);
		proc->insert_bblock(entry);
		proc->entry = entry;

		// iterate until no more indirect jump targets are known
		while(true)
		{
			cout << "disassemble" << endl;
			disassemble_procedure(proc,main,tokens,tgt,entry);
	
			// compute dominance tree
			cout << "dominance tree" << endl;
			dom = dominance_tree(proc);

			// compute liveness information
			cout << "liveness" << endl;
			live = liveness(proc);

			// rename variables and compute semi-pruned SSA form
			cout << "ssa" << endl;
			ssa(proc,dom,live);
			
			// abi
			//cout << "taint" << endl;
			//taint = shared_ptr<map<bblock_ptr,taint_lattice>>(abstract_interpretation<taint_domain,taint_lattice>(proc));
			cout << "cprop" << endl;
			cprop = shared_ptr<map<bblock_ptr,cprop_lattice>>(abstract_interpretation<cprop_domain,cprop_lattice>(proc));
			cout << "resolve" << endl;
			procedure::iterator j,jend;
			tie(j,jend) = proc->all();

			while(j != jend)
			{
				bblock_ptr bb = *j++;
				const cprop_lattice &cp(cprop->at(bb));
				basic_block::indir_iterator k,kend;

				tie(k,kend) = bb->indirect();
				while(k != kend)
				{
					pair<guard_ptr,value_ptr> p = *k++;
					var_ptr w;
					
					if((w = dynamic_pointer_cast<variable>(p.second)) && cp->has(w->nam))
					{
						const cprop_element &cm(cp->get(w->nam));

						if(cm.type == cprop_element::Const)
						{
							bb->remove_indirect(p.second);
							tgt = cm.value;
							entry = bb;
							goto out;
						}
					}
				}
			}
			break; out: cout << "new round" << endl;
		}	
		
		// finish procedure
		ret->procedures.insert(proc);
		ret->dominance.insert(make_pair(proc,dom));
		ret->liveness.insert(make_pair(proc,live));
		ret->taint.insert(make_pair(proc,taint));
		ret->cprop.insert(make_pair(proc,cprop));

		// look for call instructions to find new procedures to disassemble
		tie(i,iend) = proc->all();
		while(i != iend)
		{	
			bblock_ptr bb = *i++;
			size_t sz = bb->instructions().size(), pos = 0;
			const instr_ptr *j = bb->instructions().data();

			while(pos < sz)
			{
				if(j[pos]->opcode == instr::Call)
				{
					assert(j[pos]->operands.size() == 1);
					shared_ptr<const constant> c = dynamic_pointer_cast<const constant>(j[pos]->operands[0]);

					if(c && !has_procedure(ret,(unsigned int)c->val))
						;//call_targets.insert(c->val);
				}
				++pos;
			}
		}

		//cout << "procedure done" << endl;
	}

	return ret;
}

string graphviz(flow_ptr fg);

#endif
