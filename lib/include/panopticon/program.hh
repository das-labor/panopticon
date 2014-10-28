#include <iostream>
#include <memory>
#include <unordered_set>

#include <panopticon/procedure.hh>
#include <panopticon/disassembler.hh>
#include <panopticon/interpreter.hh>
#include <panopticon/marshal.hh>
#include <panopticon/region.hh>
#include <panopticon/database.hh>

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

	/// @returns the procedure with entry point @ref entry
	boost::optional<proc_loc> find_procedure_by_entry(prog_loc prog, offset entry);

	/// @returns the procedure with a basic block covering @ref off
	boost::optional<proc_loc> find_procedure_by_bblock(prog_loc prog, offset off);

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
		program(const std::string& r, const std::string &n = "unnamed program");

		/// Set of all procedures in the graph
		const std::unordered_set<proc_loc>& procedures(void) const;

		/// Call graph of the program
		const digraph<boost::variant<proc_loc,symbol>,std::nullptr_t>& calls(void) const;

		digraph<boost::variant<proc_loc,symbol>,std::nullptr_t>& calls(void);

		void insert(proc_loc p);

		//std::unordered_multimap<proc_wloc,boost::any> interpretations;

		/// Human-readable name of the program
		std::string name;

		/// Region name
		std::string reg;

		/**
		 * Disassemble bytes from @ref tokens starting at @ref r. The new opcodes
		 * are inserted into a new procedure. If @ref prog is NULL a new program
		 * is allocated and returned, otherwise a new procedure is added to @ref prog.
		 * All @c call instructions found while disassembling are followed. If the calls
		 * point to new procedures these are disassembled too.
		 * The @ref disass_sig is called for each procedure disassembled successfully.
		 */
		template<typename Tag,typename Dis>
		static boost::optional<prog_loc> disassemble(Dis const& main, po::slab data, const po::ref& r, boost::optional<prog_loc> prog = boost::none, disass_sig signal = disass_sig());

	private:
		mutable boost::optional<std::unordered_set<proc_loc>> _procedures;
		digraph<boost::variant<proc_loc,symbol>,std::nullptr_t> _calls;

		template<typename T>
		friend T* unmarshal(const uuid&, const rdf::storage&);
	};

	template<>
	program* unmarshal(const uuid&, const rdf::storage&);

	template<>
	archive marshal(const program*, const uuid&);

	template<typename Tag,typename Dis>
	boost::optional<prog_loc> program::disassemble(Dis const& main, po::slab data, const po::ref& r, boost::optional<prog_loc> prog, disass_sig signal)
	{
		if(prog && (*prog)->reg != r.reg)
			std::invalid_argument("programs can't span multiple regions. Program region: '" + (*prog)->reg + "', disassembly target: '" + r.reg + "'");

		if(prog && find_procedure_by_bblock(*prog,r.off))
			return prog;

		boost::optional<prog_loc> ret = prog;
		std::unordered_set<offset> worklist({r.off});

		while(!worklist.empty())
		{
			offset tgt = *worklist.begin();
			worklist.erase(worklist.begin());

			if(ret && find_procedure_by_entry(*ret,tgt))
				continue;

			std::cout << "disassemble at " << tgt << std::endl;
			boost::optional<proc_loc> new_proc = procedure::disassemble<Tag,Dis>(boost::none,main,data,tgt);

			if(new_proc)
			{
				procedure& wp = new_proc->write();

				ensure(wp.entry);
				ensure((*wp.entry)->area().lower() == tgt);

				wp.name = "proc_" + std::to_string((*wp.entry)->area().lower());

				// XXX: compute dominance tree
				//dom = dominance_tree(proc);
				//ret->dominance.insert(make_pair(proc,dom));

				// XXX: compute liveness information
				//live = po::liveness(proc);
				//ret->liveness.insert(make_pair(proc,live));

				// add to call graph
				if(!ret)
				{
					ret = prog_loc(new program(r.reg));
				}

				ret->write().insert(*new_proc);

				// insert call edges and new procedures to disassemble
				for(offset a: collect_calls(*new_proc))
				{
					auto maybe_proc = find_procedure_by_entry(*ret,a);

					if(!maybe_proc)
					{
						worklist.insert(a);
					}
					else
					{
						call(*ret,*new_proc,*maybe_proc);
					}
				}

				// XXX: resolve calls to address in call graph
				using ed_desc = typename decltype(program::_calls)::edge_descriptor;
				std::list<ed_desc> to_resolv;
				auto eds = po::edges((*ret)->calls());

				std::copy_if(eds.first,eds.second,std::back_inserter(to_resolv),
					[&](ed_desc e) { return boost::get<offset>(&get_vertex(po::target(e,(*ret)->calls()),(*ret)->calls())); });

				for(auto e: to_resolv)
				{
					auto caller = boost::get<proc_loc>(get_vertex(po::source(e,(*ret)->calls()),(*ret)->calls()));
					offset off = boost::get<offset>(get_vertex(po::target(e,(*ret)->calls()),(*ret)->calls()));
					auto maybe_proc = find_procedure_by_entry(*ret,off);

					if(maybe_proc)
					{
						remove_edge(e,ret->write().calls());
						call(*ret,caller,*maybe_proc);
					}
				}
			}

			if(signal)
			{
				signal((*prog)->procedures().size(),worklist.size());
			}
		}

		return ret;
	}
}
