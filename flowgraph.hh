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
};

bool has_procedure(flow_ptr flow, addr_t entry);

template<typename token,typename tokiter>
flow_ptr disassemble(const decoder<token,tokiter> &main, vector<token> tokens, addr_t offset = 0, bool cf_sensitive = true)
{
	flow_ptr ret(new flowgraph());
	set<addr_t> call_targets;

	call_targets.insert(offset);

	while(!call_targets.empty())
	{
		auto h = call_targets.begin();
		addr_t tgt = *h;
		proc_ptr proc;
		procedure::iterator i,iend;

		call_targets.erase(h);

		if(has_procedure(ret,tgt))
			continue;

		proc = disassemble_procedure(main,tokens,tgt,cf_sensitive);
		ret->procedures.insert(proc);

		// look for call instructions to find new procedures to disassemble
		tie(i,iend) = proc->all();
		while(i != iend)
		{	
			instr_iterator j,jend;
			bblock_ptr bb = *i++;

			tie(j,jend) = bb->instructions();
			while(j != jend)
			{
				instr_cptr in = *j++;
				if(in->opcode == instr::Call)
				{
					assert(in->operands.size() == 1);
					shared_ptr<const constant> c = dynamic_pointer_cast<const constant>(in->operands[0]);

					if(c && !has_procedure(ret,(unsigned int)c->val))
						call_targets.insert(c->val);
				}
			}
		}

		// compute dominance tree
		dom_ptr dom = dominance_tree(proc);
		ret->dominance.insert(make_pair(proc,dom));

		// compute liveness information
		live_ptr live = liveness(proc);
		ret->liveness.insert(make_pair(proc,live));

		// rename variables and compute semi-pruned SSA form
		ssa(proc,dom,live);

		// abi
		ret->taint.insert(make_pair(proc,shared_ptr<map<bblock_ptr,taint_lattice>>(abstract_interpretation<taint_domain,taint_lattice>(proc))));
	}

	return ret;
}

string graphviz(flow_ptr fg);

#endif
