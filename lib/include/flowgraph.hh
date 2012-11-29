#ifndef FLOWGRAPH_HH
#define FLOWGRAPH_HH

#include <iostream>
#include <memory>
#include <set>

#include <procedure.hh>
#include <disassembler.hh>
#include <dflow.hh>
#include <interpreter.hh>

namespace po
{
	typedef ::std::shared_ptr<struct flowgraph> flow_ptr;

	struct flowgraph
	{
		::std::set<proc_ptr> procedures;
		::std::map<proc_ptr,dom_ptr> dominance;
		::std::map<proc_ptr,live_ptr> liveness;
		//::std::map<proc_ptr,::std::shared_ptr< ::std::map<bblock_ptr,taint_lattice>>> taint;
		::std::map<proc_ptr,::std::shared_ptr< ::std::map<rvalue,sscp_lattice>>> simple_sparse_constprop;
		::std::string name;
	};

	bool has_procedure(flow_ptr flow, addr_t entry);
	proc_ptr find_procedure(flow_ptr flow, addr_t entry);

	template<typename Tag>
	flow_ptr disassemble(const disassembler<Tag> &main, ::std::vector<typename rule<Tag>::token> tokens, addr_t offset = 0, bool cf_sensitive = true)
	{
		flow_ptr ret(new flowgraph());
		::std::set< ::std::pair<addr_t,proc_ptr>> call_targets;

		ret->name = "unnamed flowgraph";
		call_targets.insert(::std::make_pair(offset,proc_ptr(new procedure())));

		while(!call_targets.empty())
		{
			auto h = call_targets.begin();
			proc_ptr proc;
			addr_t tgt;
			::std::tie(tgt,proc) = *h;
			
			if(has_procedure(ret,tgt))
				continue;

			dom_ptr dom;
			live_ptr live;
			::std::shared_ptr< ::std::map<rvalue,sscp_lattice>> sscp;
			//::std::shared_ptr< ::std::map<bblock_ptr,taint_lattice>> taint;

			call_targets.erase(h);

			// iterate until no more indirect jump targets are known
			while(true)
			{
				::std::cout << "disassemble at " << tgt << ::std::endl;
				disassemble_procedure(proc,main,tokens,tgt);

				if(!proc->entry)
					proc->entry = find_bblock(proc,tgt);

				// compute dominance tree
				::std::cout << "dominance tree" << ::std::endl;
				dom = dominance_tree(proc);

				// compute liveness information
				::std::cout << "liveness" << ::std::endl;
				live = liveness(proc);

				// rename variables and compute semi-pruned SSA form
				::std::cout << "ssa" << ::std::endl;
				ssa(proc,dom,live);
				
				// simple sparse constant propagation
				::std::cout << "sscp" << ::std::endl;
				sscp = interpret<simple_sparse_constprop>(proc);
/*
				::std::cout << "resolve" << ::std::endl;
				bool resolved_targets = false;
				for(bblock_ptr bb: proc->basic_blocks)
				{
					bb->mutate_outgoing([&](::std::list<ctrans> &out)
					{
						auto k = out.begin();

						while(k != out.end())
						{
							ctrans &p(*k++);
							
							if(!p.bblock && p.value.is_variable() && sscp->count(p.value))
							{
								const sscp_lattice &l = sscp->at(p.value);

								if(l.type == sscp_lattice::Const)
								{
									p = ctrans(p.guard,constant(l.value & 0xffff));
									tgt = l.value & 0xffff;
									::std::cout << "resolve to " << (l.value & 0xffff) << ::std::endl;
									//resolved_targets = true;
									return;
								}
							}
						}
					});
					
					if(resolved_targets)
						goto out;
				}
				*/break; //out: ::std::cout << "new round" << ::std::endl;
			}	
			
			// finish procedure
			ret->procedures.insert(proc);
			ret->dominance.insert(make_pair(proc,dom));
			ret->liveness.insert(make_pair(proc,live));
			//ret->taint.insert(make_pair(proc,taint));
			ret->simple_sparse_constprop.insert(make_pair(proc,sscp));

			// look for call instructions to find new procedures to disassemble
			execute(proc,[&](const lvalue &legt, instr::Function fn, const ::std::vector<rvalue> &right)
			{
				if(fn == instr::Call)
				{
					assert(right.size() == 1);

					if(right[0].is_constant())
					{
						const constant &c = right[0].constant();
						proc_ptr callee = find_procedure(ret,(unsigned int)c.value());

						if(!callee)
						{
							auto k = call_targets.begin(), kend = call_targets.end();
							while(k != kend)
							{
								if(k->first != (unsigned int)c.value())
									++k;
								else
								{
									callee = k->second;
									break;
								}
							}
							if(!callee)
								callee = proc_ptr(new procedure());
							call_targets.insert(make_pair(c.value(),callee));
						}
						call(proc,callee);
					}
				}
			});

			::std::cout << "procedure done" << ::std::endl;
		}

		
		return ret;
	}

	::std::string graphviz(flow_ptr fg);
	::std::string turtle(flow_ptr fg);
}

#endif
