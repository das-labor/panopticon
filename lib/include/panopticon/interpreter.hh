#include <vector>
#include <cmath>
#include <memory>
#include <map>
#include <set>
#include <algorithm>

#include <boost/variant.hpp>

#include <panopticon/instr.hh>
#include <panopticon/mnemonic.hh>
#include <panopticon/basic_block.hh>
#include <panopticon/procedure.hh>

#pragma once

/**
 * @file
 * @brief Abstract Interpretation framework
 *
 * Abstract Interpretation executes program using abstract
 * values. These are less precise than concrete values but
 * the algorithms computing them are guaranteed to terminate.
 *
 * This file defines a simple Abstract Interpreter over
 * Panopticon IL that can be plugged with arbitrary abstract
 * domains.
 *
 * Currently the only abstract domain implemented is Simple
 * Sparse Constant Propagation (SSCP).
 */

namespace po
{
	template <typename T>
	struct domain_traits
	{
		using value_type = void; ///< Lattice
		using interpreter_type = void; ///< static_visitor<value_type>(aenv)
	};

	struct meet {};
	struct join {};

	template<typename Domain>
	using environment = std::unordered_map<variable,Domain>;

	template<typename Domain>
	typename domain_traits<Domain>::value_type supremum(typename domain_traits<Domain>::value_type,typename domain_traits<Domain>::value_type, Domain);

	/**
	 * @brief Compute the Abstract Interpretation
	 *
	 * The function interprets a given procedure @ref proc
	 * using the abstract domain @ref tag. The @ref tag
	 * argument must be a model of the @c AbstractDomainConcept.
	 *
	 * The returned container maps every SSA variable in
	 * @ref proc to an instance of an element in the abstract domain.
	 */
	template<typename Domain>
	std::unordered_map<variable,typename domain_traits<Domain>::value_type> interpret(const proc_loc proc, Domain domain = Domain())
	{
		using L = typename domain_traits<Domain>::value_type;
		using I = typename domain_traits<Domain>::interpreter_type;
		using vx_desc = typename boost::graph_traits<decltype(proc->control_transfers)>::vertex_descriptor;
		using e_desc = typename boost::graph_traits<decltype(proc->control_transfers)>::edge_descriptor;

		environment<L> ret;
		//const std::vector<bblock_loc>& rpo = proc->rev_postorder();
		//std::list<vx_desc> rpo_vx;
		std::unordered_set<vx_desc> worklist;
		I interp(ret);
		has_symbol_visitor<phi_symbol> phi_vis;

	//	std::transform(rpo.begin(),rpo.end(),std::back_inserter(rpo_vx),[&](bblock_loc bb) { return find_node<boost::variant<bblock_loc,rvalue>,guard>(bb,proc->control_transfers); });

		while(!worklist.empty())
		{
			vx_desc vx = *worklist.begin();
			bblock_loc bb = get<bblock_loc>(get_vertex<boost::variant<bblock_loc,rvalue>,guard>(vx,proc->control_transfers));
			bool modified = false;

			worklist.erase(worklist.begin());
			execute(bb,[&](const instr& i)
			{
				boost::optional<variable> ass = (is_variable(i.assignee) ? boost::make_optional(to_variable(i.assignee)) : boost::none);
				std::vector<L> arguments;
				L res = ass && ret.count(*ass) ? ret.at(*ass) : L();
				std::vector<rvalue> right = operands(i);

				for(const rvalue &r: right)
					if(is_variable(r) && ret.count(to_variable(r)))
						arguments.emplace_back(ret.at(to_variable(r)));
					else
						arguments.emplace_back(L());

				if(boost::apply_visitor(phi_vis,i.function))
					res = std::accumulate(arguments.begin(),arguments.end(),res,[&](const L &acc, const L &x) { return supremum(acc,x,domain); });
				else
					res = supremum(boost::apply_visitor(interp,i.function),res,domain);

				modified = ass && (!ret.count(*ass) || !(ret.at(to_variable(*ass)) == res));

				if(ass && ret.count(*ass))
					ret.erase(*ass);
				ret.emplace(*ass,res);
			});

			if(modified)
			{
				for(auto e: iters(out_edges(vx,proc->control_transfers)))
				{
					auto v = target(e,proc->control_transfers);
					auto w = get_vertex(v,proc->control_transfers);
					if(get<bblock_loc>(&w))
						worklist.insert(v);
				}
			}

			std::cout << worklist.size() << std::endl;
		}

		return ret;
	}

	/**
	 * @brief Concrete semantics
	 * @group abstract_domains
	 *
	 * Concrete semantics of Panopticon IL over then domain of
	 * integers. The domain uses the standard C++ operands like
	 * plus and bitwise OR of the type I.
	 *
	 * @note This is not an abstract domain, hence the AI algorithm
	 * may not terminate.
	 */
	struct concrete_domain {};

	/**
	 * Executes a IL statement using concrete semantics ot type I
	 * @internal
	 */
	struct concrete_interpreter : public boost::static_visitor<rvalue>
	{
		concrete_interpreter(environment<result_type>&);

		result_type operator()(const logic_and& a);
		result_type operator()(const logic_or& a);
		result_type operator()(const logic_neg& a);
		result_type operator()(const logic_impl& a);
		result_type operator()(const logic_equiv& a);
		result_type operator()(const int_add& a);
		result_type operator()(const int_sub& a);
		result_type operator()(const int_mul& a);
		result_type operator()(const int_div& a);
		result_type operator()(const int_mod& a);
		result_type operator()(const int_less& a);
		result_type operator()(const int_equal& a);
		result_type operator()(const int_and& a);
		result_type operator()(const int_or& a);
		result_type operator()(const int_neg& a);
		result_type operator()(const int_call& a);
		result_type operator()(const int_lift& a);
		result_type operator()(const univ_nop& a);
		result_type operator()(const univ_phi& a);

	protected:
		rvalue normalize(const rvalue& v) const;

		const environment<rvalue>& _environment;
	};

	template<>
	struct domain_traits<concrete_domain>
	{
		using value_type = rvalue;
		using interpreter_type = concrete_interpreter;
	};

	template<>
	rvalue supremum(rvalue, rvalue, concrete_domain);

	/**
	 * @brief K-Set domain
	 * @ingroup abstract_domain
	 *
	 * @todo
	 */
	template<unsigned int k>
	struct kset_domain {};

	/**
	 * @internal
	 */
	template<unsigned int k>
	struct kset_interpreter : public boost::static_visitor<boost::variant<meet,join,std::unordered_set<unsigned long long>>>
	{
		kset_interpreter(std::shared_ptr<std::unordered_map<rvalue,boost::variant<meet,join,std::unordered_set<unsigned long long>>>> aenv)
		: static_visitor<result_type>()/*, _environment(aenv)*/ {}
	};

	template<unsigned int k>
	struct domain_traits<kset_domain<k>>
	{
		using value_type = boost::variant<meet,join,std::unordered_set<unsigned long long>>;
		using interpreter_type = kset_interpreter<k>;
	};

	/// Computes the supremum of two sscp lattice elements
	template<unsigned int k>
	boost::variant<meet,join,std::unordered_set<unsigned long long>> supremum(const boost::variant<meet,join,std::unordered_set<unsigned long long>> &a, const boost::variant<meet,join,std::unordered_set<unsigned long long>> &b, kset_domain<k>);

	// intervals
	// octagons
	// ric
}
