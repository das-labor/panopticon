#include <iostream>
#include <memory>
#include <unordered_set>
#include <mutex>

#include <panopticon/procedure.hh>
#include <panopticon/disassembler.hh>
#include <panopticon/interpreter.hh>
#include <panopticon/marshal.hh>
#include <panopticon/region.hh>

#include <boost/any.hpp>

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
	using prog_wloc = wloc<program>;
	using disass_sig = std::function<void(unsigned int done, unsigned int todo)>;
	using symbol = std::string;

	/// Insert call graph edge from @arg from to @to
	void call(prog_loc p, proc_loc from, proc_loc to);
	void call(prog_loc p, proc_loc from, const symbol& to);

	/// @return true if the program @ref prog has a procedure with entry point @ref entry
	bool has_procedure(prog_loc prog, offset entry);

	/// @returns the procedure with entry point @ref entry
	boost::optional<proc_loc> find_procedure(prog_loc prog, offset entry);

	/// looks for call instructions to find new procedures to disassemble
	std::unordered_set<offset> collect_calls(proc_loc proc);

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
		/// Contruct an empty program with name @ref name
		program(const std::string &n = "unnamed program");

		/// Set of all procedures in the graph
		const std::unordered_set<proc_loc>& procedures(void) const;

		/// Call graph of the program
		const digraph<boost::variant<proc_loc,symbol>,std::nullptr_t>& calls(void) const;

		digraph<boost::variant<proc_loc,symbol>,std::nullptr_t>& calls(void);

		void insert(proc_loc p);

		//std::unordered_multimap<proc_wloc,boost::any> interpretations;

		/// Human-readable name of the program
		std::string name;


		/**
		 * Disassemble bytes from @ref tokens starting at @ref offset. The new opcodes
		 * are inserted into a new procedure. If @ref prog is NULL a new proggaph
		 * is allocated and returned, otherwise the new procedure is added to @ref prog.
		 * All @c call instructions found while disassembling are followed. If the calls
		 * point to new procedures these are disassembled too.
		 * The @ref disass_sig is called for each procedure disassembled successfully.
		 */
		template<typename Tag>
		static prog_loc disassemble(const disassembler<Tag> &main, std::vector<typename rule<Tag>::token> tokens, offset off = 0, boost::optional<prog_loc> prog = boost::none, disass_sig signal = disass_sig())
		{
			prog_loc ret = (prog ? *prog : prog_loc(new program("unnamed program")));
			std::unordered_set<std::pair<offset,proc_loc>> call_targets;

			call_targets.insert(std::make_pair(off,proc_loc(new procedure("proc_noname"))));

			while(!call_targets.empty())
			{
				auto h = call_targets.begin();
				offset tgt;
				tgt = h->first;
				proc_loc proc = h->second;

				call_targets.erase(call_targets.begin());

				if(has_procedure(ret,tgt))
					continue;

				//live_ptr live;

				std::cout << "disassemble at " << tgt << std::endl;
				proc = procedure::disassemble(proc,main,tokens,tgt);

				procedure &wp = proc.write();

				if(!wp.entry)
					wp.entry = find_bblock(proc,tgt);

				// compute dominance tree
				//dom = dominance_tree(proc);

				// compute liveness information
				//live = po::liveness(proc);

				// finish procedure
				ret.write().insert(proc);
				//ret->dominance.insert(make_pair(proc,dom));
				//ret->liveness.insert(make_pair(proc,live));
				wp.name = "proc_" + std::to_string((*proc->entry)->area().lower());

				// insert call edges and new procedures to disassemble
				for(offset a: collect_calls(proc))
				{
					auto i = std::find_if(call_targets.begin(),call_targets.end(),[&](const std::pair<offset,proc_loc> &p) { return p.first == a; });

					if(i == call_targets.end())
					{
						auto j = find_procedure(ret,a);

						if(!j)
						{
							proc_loc q(new procedure("proc_" + std::to_string(a)));

							call_targets.insert(std::make_pair(a,q));
							call(ret,proc,q);
						}
						else
						{
							call(ret,proc,*j);
						}
					}
					else
					{
						call(proc,i->second);
					}
				}

				if(signal)
					signal((*prog)->procedures().size(),call_targets.size());
			}

			return ret;
		}

	private:
		mutable boost::optional<std::unordered_set<proc_loc>> _procedures;
		digraph<boost::variant<proc_loc,symbol>,std::nullptr_t> _calls;

		template<typename T>
		friend T* unmarshal(const uuid&, const rdf::storage&);
	};

	template<>
	program* unmarshal(const uuid&, const rdf::storage&);

	template<>
	rdf::statements marshal(const program*, const uuid&);
}
