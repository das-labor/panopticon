#ifndef FLOWGRAPH_HH
#define FLOWGRAPH_HH

#include <iostream>
#include <memory>
#include <set>
#include <mutex>

#include <procedure.hh>
#include <disassembler.hh>
#include <dflow.hh>
#include <interpreter.hh>
#include <sat.hh>
#include <marshal.hh>

namespace po
{
	struct flowgraph;

	typedef std::shared_ptr<flowgraph> flow_ptr;
	typedef std::shared_ptr<const flowgraph> flow_cptr;
	typedef std::function<void(unsigned int done, unsigned int todo)> disassemble_cb;
	
	bool has_procedure(flow_ptr flow, addr_t entry);
	proc_ptr find_procedure(flow_ptr flow, addr_t entry);

	// look for call instructions to find new procedures to disassemble
	std::set<addr_t> collect_calls(proc_cptr proc);

	struct flowgraph
	{
		static flow_ptr unmarshal(const rdf::node &n, const rdf::storage &store);

		flowgraph(const std::string &n = "unnamed flowgraph");

		std::set<proc_ptr> procedures;
		std::map<proc_ptr,dom_ptr> dominance;
		std::map<proc_ptr,live_ptr> liveness;
		std::string name;
		std::mutex mutex;

		template<typename Tag>
		static flow_ptr disassemble(const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, addr_t offset = 0, flow_ptr flow = 0, disassemble_cb signal = disassemble_cb())
		{
			flow_ptr ret = (flow ? flow : flow_ptr(new flowgraph("unnamed flowgraph")));
			std::set< std::pair<addr_t,proc_ptr>> call_targets;

			call_targets.insert(std::make_pair(offset,proc_ptr(new procedure())));

			while(!call_targets.empty())
			{
				auto h = call_targets.begin();
				proc_ptr proc;
				addr_t tgt;
				std::tie(tgt,proc) = *h;
				
				call_targets.erase(call_targets.begin());

				{
					std::lock_guard<std::mutex> guard(ret->mutex);
					if(has_procedure(ret,tgt))
						continue;
				}
				
				dom_ptr dom;
				live_ptr live;

					std::cout << "disassemble at " << tgt << std::endl;
					proc = procedure::disassemble(proc,main,tokens,tgt);

					if(!proc->entry)
						proc->entry = find_bblock(proc,tgt);
					
					// compute dominance tree
					dom = dominance_tree(proc);

					// compute liveness information
					live = po::liveness(proc);

					// finish procedure
					{	
						std::lock_guard<std::mutex> guard(ret->mutex);

						ret->procedures.insert(proc);
						ret->dominance.insert(make_pair(proc,dom));
						ret->liveness.insert(make_pair(proc,live));
						proc->name = "proc_" + std::to_string(proc->entry->area().begin);

						// insert call edges and new procedures to disassemble
						for(addr_t a: collect_calls(proc))
						{
							auto i = std::find_if(call_targets.begin(),call_targets.end(),[&](const std::pair<addr_t,proc_ptr> &p) { return p.first == a; });
							
							if(i == call_targets.end())
							{
								auto j = find_procedure(ret,a);

								if(!j)
								{
									proc_ptr q(new procedure("proc_" + std::to_string(a)));

									call_targets.insert(std::make_pair(a,q));
									call(proc,q);
								}
								else
								{
									call(proc,j);
								}
							}
							else
							{
								call(proc,i->second);
							}
						}
					}
					
					if(signal)
						signal(flow->procedures.size(),call_targets.size());
			}
			
			return ret;
		}
	};
	
	odotstream &operator<<(odotstream &os, const flowgraph &f);
	oturtlestream& operator<<(oturtlestream &os, const flowgraph &f);
	ordfstream& operator<<(ordfstream &os, const flowgraph &f);
	std::string unique_name(const flowgraph &f);
}

#endif
