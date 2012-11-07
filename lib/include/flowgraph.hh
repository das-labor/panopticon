#ifndef FLOWGRAPH_HH
#define FLOWGRAPH_HH

#include <iostream>
#include <memory>
#include <set>

#include "procedure.hh"
#include "decoder.hh"
#include "dflow.hh"
#include "absinterp.hh"

typedef std::shared_ptr<struct flowgraph> flow_ptr;

struct flowgraph
{
	set<proc_ptr> procedures;
	std::map<proc_ptr,dom_ptr> dominance;
	std::map<proc_ptr,live_ptr> liveness;
	std::map<proc_ptr,std::shared_ptr<std::map<bblock_ptr,taint_lattice>>> taint;
	std::map<proc_ptr,std::shared_ptr<std::map<bblock_ptr,cprop_lattice>>> cprop;
	std::string name;
};

bool has_procedure(flow_ptr flow, addr_t entry);
proc_ptr find_procedure(flow_ptr flow, addr_t entry);

template<typename Tag>
flow_ptr disassemble(const decoder<Tag> &main, std::vector<typename rule<Tag>::token> tokens, addr_t offset = 0, bool cf_sensitive = true)
{
	flow_ptr ret(new flowgraph());
	set<pair<addr_t,proc_ptr>> call_targets;

	ret->name = "unnamed flowgraph";
	call_targets.insert(make_pair(offset,proc_ptr(new procedure())));

	while(!call_targets.empty())
	{
		auto h = call_targets.begin();
		proc_ptr proc;
		addr_t tgt;
		tie(tgt,proc) = *h;
		
		if(has_procedure(ret,tgt))
			continue;

		dom_ptr dom;
		live_ptr live;
		std::shared_ptr<std::map<bblock_ptr,taint_lattice>> taint;
		std::shared_ptr<std::map<bblock_ptr,cprop_lattice>> cprop;
		procedure::iterator i;

		call_targets.erase(h);

		// iterate until no more indirect jump targets are known
		while(true)
		{
			std::cout << "disassemble" << endl;
			disassemble_procedure(proc,main,tokens,tgt);

			if(!proc->entry)
				proc->entry = find_bblock(proc,tgt);

			// compute dominance tree
			std::cout << "dominance tree" << endl;
			dom = dominance_tree(proc);

			// compute liveness information
			std::cout << "liveness" << endl;
			live = liveness(proc);

			// rename variables and compute semi-pruned SSA form
			std::cout << "ssa" << endl;
			ssa(proc,dom,live);
			
			/* abi
			std::cout << "taint" << endl;
			taint = std::shared_ptr<std::map<bblock_ptr,taint_lattice>>(abstract_interpretation<taint_domain,taint_lattice>(proc));*/
			std::cout << "cprop" << endl;
			cprop = std::shared_ptr<std::map<bblock_ptr,cprop_lattice>>(abstract_interpretation<cprop_domain,cprop_lattice>(proc));
			std::cout << "resolve" << endl;
			procedure::iterator j = proc->basic_blocks.begin();

			while(j != proc->basic_blocks.end())
			{
				bblock_ptr bb = *j++;
				const cprop_lattice &cp(cprop->at(bb));
				basic_block::out_iterator k,kend;

				tie(k,kend) = bb->outgoing();
				while(k != kend)
				{
					ctrans &p(*k++);
					var_cptr w = p.variable();
					
					if(!p.bblock && w && cp->has(w->nam))
					{
						const cprop_element &cm(cp->get(w->nam));

						if(cm.type == cprop_element::Const)
						{
							p.value = value_ptr(new constant(w->mask() & cm.value));
							tgt = cm.value;
							goto out;
						}
					}
				}
			}
			break; out: std::cout << "new round" << endl;
		}	
		
		// finish procedure
		ret->procedures.insert(proc);
		ret->dominance.insert(make_pair(proc,dom));
		ret->liveness.insert(make_pair(proc,live));
		ret->taint.insert(make_pair(proc,taint));
		ret->cprop.insert(make_pair(proc,cprop));

		// look for call instructions to find new procedures to disassemble
		i = proc->basic_blocks.begin();
		while(i != proc->basic_blocks.end())
		{	
			bblock_ptr bb = *i++;
			size_t sz = bb->instructions().size(), pos = 0;
			const instr_ptr *j = bb->instructions().data();

			while(pos < sz)
			{
				if(j[pos]->function == instr::Call)
				{
					assert(j[pos]->arguments.size() == 1);
					std::shared_ptr<const constant> c = std::dynamic_pointer_cast<const constant>(j[pos]->arguments[0]);

					if(c)
					{	
						proc_ptr callee = find_procedure(ret,(unsigned int)c->val);

						if(!callee)
						{
							auto k = call_targets.begin(), kend = call_targets.end();
							while(k != kend)
							{
								if(k->first != (unsigned int)c->val)
									++k;
								else
								{
									callee = k->second;
									break;
								}
							}
							if(!callee)
								callee = proc_ptr(new procedure());
							call_targets.insert(make_pair(c->val,callee));
						}
						proc->callees.push_back(callee);
					}
				}
				++pos;
			}
		}

		std::cout << "procedure done" << endl;
	}

	
	return ret;
}

std::string graphviz(flow_ptr fg);
std::string turtle(flow_ptr fg);

#endif
