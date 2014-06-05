#include <vector>
#include <cmath>
#include <memory>
#include <map>
#include <set>

#include <boost/variant.hpp>

#include <panopticon/instr.hh>
#include <panopticon/mnemonic.hh>
#include <panopticon/basic_block.hh>
#include <panopticon/procedure.hh>

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
	std::shared_ptr<std::unordered_map<rvalue,typename domain_traits<Domain>::value_type>> interpret(const proc_loc proc, Domain domain = Domain())
	{
		using L = typename domain_traits<Domain>::value_type;
		using I = typename domain_traits<Domain>::interpreter_type;
		using vx_desc = typename boost::graph_traits<decltype(proc->control_transfers)>::vertex_descriptor;

		std::shared_ptr<std::unordered_map<rvalue,L>> ret = std::make_shared<std::unordered_map<rvalue,L>>();
		const std::vector<bblock_loc>& rpo = proc->rev_postorder();
		std::unordered_set<vx_desc> worklist;
		I interp(ret);
		has_symbol_visitor<phi_symbol> phi_vis;

		std::transform(rpo.begin(),rpo.end(),[&](bblock_loc bb) { return find_node<boost::variant<bblock_loc,rvalue>,guard>(bb,proc->control_transfers); });

		while(!worklist.empty())
		{
			vx_desc vx = *worklist.begin();
			bblock_loc bb = get<bblock_loc>(get_vertex<boost::variant<bblock_loc,rvalue>,guard>(vx,proc->control_transfers));
			bool modified = false;

			worklist.erase(worklist.begin());
			execute(bb,[&](const instr& i)
			{
				std::vector<L> arguments;
				L res = ret->count(i.assignee) ? ret->at(i.assignee) : L();
				std::vector<rvalue> right = operands(i);

				for(const rvalue &r: right)
					if(ret->count(r))
						arguments.emplace_back(ret->at(r));
					else
						arguments.emplace_back(L());

				if(boost::apply_visitor(phi_vis,i.function))
					res = std::accumulate(arguments.begin(),arguments.end(),res,[&](const L &acc, const L &x) { return supremum(acc,x,domain); });
				else
					res = supremum(boost::apply_visitor(interp,i,arguments),res,domain);

				modified = (!ret->count(i.assignee) || !(ret->at(i.assignee) == res));

				if(ret->count(i.assignee))
					ret->erase(i.assignee);
				ret->emplace(i.assignee,res);
			});

			if(modified)
			{
				auto p = out_edges(vx,proc->control_transfers);
				std::copy_if(p.first,p.second,std::back_inserter(worklist),[&](vx_desc v)
					{ get<bblock_loc>(&get_vertex<boost::variant<bblock_loc,rvalue>,guard>(v,proc->control_transfers)); });
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
	struct concrete_interpreter : public boost::static_visitor<unsigned long long>
	{
		concrete_interpreter(std::shared_ptr<std::unordered_map<rvalue,unsigned long long>> aenv) : static_visitor<result_type>() /*, _environment(aenv)*/ {}
	};

	template<>
	struct domain_traits<concrete_domain>
	{
		using value_type = unsigned long long;
		using interpreter_type = concrete_interpreter;
	};

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
