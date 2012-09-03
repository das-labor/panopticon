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

template<typename token,typename tokiter>
flow_ptr disassemble(const decoder<token,tokiter> &main, vector<token> tokens, addr_t offset = 0, bool cf_sensitive = true)
{
	flow_ptr ret(new flowgraph());
	list<addr_t> call_targets;

	call_targets.push_back(offset);

	while(!call_targets.empty())
	{
		addr_t tgt = call_targets.back();
		proc_ptr proc;
		procedure::iterator i,iend;

		call_targets.pop_back();
		proc = disassemble_procedure(main,tokens,tgt,cf_sensitive);

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

					if(c)
					{
						cout << c->val << endl;
						call_targets.push_back(c->val);
					}
				}
			}
		}
		// XXX
		return ret;
	}

	return ret;
}

string graphviz(flow_ptr fg);

#endif
