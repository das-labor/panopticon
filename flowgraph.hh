#ifndef FLOWGRAPH_HH
#define FLOWGRAPH_HH

#include <memory>
#include <set>

#include "procedure.hh"
#include "decoder.hh"

using namespace std;

typedef shared_ptr<struct flowgraph> flow_ptr;

struct flowgraph
{
	set<proc_ptr> procedures;
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

		cerr << "dissass " << tgt << endl;
		proc = disassemble_procedure(main,tokens,tgt,cf_sensitive);
		ret->procedures.insert(proc);

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
					{
						cerr << "add " << c->val << endl;
						call_targets.insert(c->val);
					}
				}
			}
		}
	}

	return ret;
}

string graphviz(flow_ptr fg);

#endif
