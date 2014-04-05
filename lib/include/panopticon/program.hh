#include <iostream>
#include <memory>
#include <unordered_set>
#include <mutex>

#include <procedure.hh>
#include <disassembler.hh>
#include <dprog.hh>
#include <interpreter.hh>
#include <sat.hh>
#include <marshal.hh>

#pragma once

/**
 * @file
 * @brief Function call graph
 *
 * A program hold all known procedures, their dominance trees,
 * liveness information, procedure name and locking facilities.
 */

namespace po
{
	struct program;

	using prog_loc = loc<program>;
	using prog_wloc = wloc<const program>;
	using disass_sig = std::function<void(unsigned int done, unsigned int todo)>;

	/// @return true if the program @ref prog has a procedure with entry point @ref entry
	bool has_procedure(prog_ptr prog, addr_t entry);

	/// @returns the procedure with entry point @ref entry
	proc_ptr find_procedure(prog_ptr prog, addr_t entry);

	/// looks for call instructions to find new procedures to disassemble
	std::unordered_set<addr_t> collect_calls(proc_cptr proc);

	/**
	 * @brief Call graph
	 *
	 * In general each file has one program per intruction set that holds all
	 * disassembled procedures.
	 *
	 * The results of liveness analysis amd the dominance trees of all
	 * procedures are kept in the program function.
	 */
	struct program
	{
		static prog_ptr unmarshal(const rdf::node &n, const rdf::storage &store);

		/// Contruct an empty program with name @ref name
		program(const std::string &n = "unnamed program");

		/// Set of all procedures in the graph
		std::unordered_set<proc_ptr> procedures;

		std::unordered_multimap<proc_wptr,boost::any> interpretations;

		/// Human-readable name of the program
		std::string name;

		/// Call graph of the program
		digraph<boost::variant<proc_wptr,std::string>,void> calls;

		/**
		 * Disassemble bytes from @ref tokens starting at @ref offset. The new opcodes
		 * are inserted into a new procedure. If @ref prog is NULL a new proggaph
		 * is allocated and returned, otherwise the new procedure is added to @ref prog.
		 * All @c call instructions found while disassembling are followed. If the calls
		 * point to new procedures these are disassembled too.
		 * The @ref disassemble_cb is called for each procedure disassembled successfully.
		 */
		template<typename Tag>
		static prog_ptr disassemble(const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, addr_t offset = 0, prog_ptr prog = 0, disassemble_cb signal = disassemble_cb())
		{
			prog_ptr ret = (prog ? prog : prog_ptr(new program("unnamed program")));
			std::unordered_set< std::pair<addr_t,proc_ptr>> call_targets;

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
						signal(prog->procedures.size(),call_targets.size());
			}

			return ret;
		}
	};

	odotstream &operator<<(odotstream &os, const program &f);
	oturtlestream& operator<<(oturtlestream &os, const program &f);
	ordfstream& operator<<(ordfstream &os, const program &f);
	std::string unique_name(const program &f);
}
