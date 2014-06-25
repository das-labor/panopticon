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
	struct domain_traits;

	template<typename D>
	struct interpreter;

	struct meet_t { bool operator==(const meet_t&) const { return true; } };
	struct join_t { bool operator==(const join_t&) const { return true; } };

	std::ostream& operator<<(std::ostream&,const meet_t&);
	std::ostream& operator<<(std::ostream&,const join_t&);

	extern const meet_t meet;
	extern const join_t join;

	template<typename Domain>
	using environment = std::unordered_map<variable,Domain>;

	template<typename Domain>
	typename domain_traits<Domain>::value_type supremum(typename domain_traits<Domain>::value_type,typename domain_traits<Domain>::value_type, Domain);

	template<typename Domain>
	typename domain_traits<Domain>::value_type overapproximate(rvalue,Domain);

	// infimum
	// underapproximate

	template<typename D>
	struct interpret_visitor : public boost::static_visitor<typename domain_traits<D>::value_type>
	{
		using Value = typename domain_traits<D>::value_type;

		interpret_visitor(void) : boost::static_visitor<Value>(), _environment(boost::none), _interpreter() {}
		interpret_visitor(const environment<Value>& env) : boost::static_visitor<Value>(), _environment(env), _interpreter() {}
		// w/ alias analysis

		template<typename Symbol,typename Domain,typename Codomain>
		Value operator()(const unop<Symbol,Domain,Codomain,rvalue>& op)
		{
			basic_operation<Value> bop(unop<Symbol,Domain,Codomain,Value>{normalize(op.right)});
			return boost::apply_visitor(_interpreter,bop);
		}

		template<typename Symbol,typename Domain,typename Codomain>
		Value operator()(const binop<Symbol,Domain,Codomain,rvalue>& op)
		{
			basic_operation<Value> bop(binop<Symbol,Domain,Codomain,Value>{
					normalize(op.left),
					normalize(op.right)});
			return boost::apply_visitor(_interpreter,bop);
		}

		template<typename Symbol,typename Domain,typename Codomain>
		Value operator()(const naryop<Symbol,Domain,Codomain,rvalue>& op)
		{
			std::vector<Value> vec;
			std::transform(op.operands.begin(),op.operands.end(),std::back_inserter(vec),[&](rvalue rv) { return normalize(rv); });
			basic_operation<Value> bop{naryop<Symbol,Domain,Codomain,Value>{vec}};
			return boost::apply_visitor(_interpreter,bop);
		}

	protected:
		boost::optional<const environment<Value>&> _environment;
		interpreter<D> _interpreter;

		Value normalize(const rvalue& v) const
		{
			if(is_constant(v))
				return overapproximate(to_constant(v),D{});
			else if(is_variable(v) && _environment)
			{
				auto i = _environment->find(to_variable(v));
				if(i != _environment->end())
					return i->second;
				else
					return overapproximate(v,D{});
			}
			else
				return overapproximate(v,D{});
		}
	};

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
		using vx_desc = typename boost::graph_traits<decltype(proc->control_transfers)>::vertex_descriptor;

		environment<L> ret;
		std::unordered_set<vx_desc> worklist;
		interpret_visitor<Domain> vis(ret);

		for(auto vx: iters(vertices(proc->control_transfers)))
			if(boost::get<bblock_loc>(&get_vertex(vx,proc->control_transfers)))
				worklist.emplace(vx);

		while(!worklist.empty())
		{
			vx_desc vx = *worklist.begin();
			bblock_loc bb = get<bblock_loc>(get_vertex<boost::variant<bblock_loc,rvalue>,guard>(vx,proc->control_transfers));
			bool modified = false;

			worklist.erase(worklist.begin());
			execute(bb,[&](const instr& i)
			{
				L res = boost::apply_visitor(vis,i.function);
				variable var = to_variable(i.assignee);

				modified |= !ret.count(var) || !(ret.at(var) == res);
				ret[var] = res;
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
		concrete_interpreter(void);
		concrete_interpreter(environment<result_type>&);

		result_type operator()(const logic_and<rvalue>& a);
		result_type operator()(const logic_or<rvalue>& a);
		result_type operator()(const logic_neg<rvalue>& a);
		result_type operator()(const logic_impl<rvalue>& a);
		result_type operator()(const logic_equiv<rvalue>& a);
		result_type operator()(const int_add<rvalue>& a);
		result_type operator()(const int_sub<rvalue>& a);
		result_type operator()(const int_mul<rvalue>& a);
		result_type operator()(const int_div<rvalue>& a);
		result_type operator()(const int_mod<rvalue>& a);
		result_type operator()(const int_less<rvalue>& a);
		result_type operator()(const int_equal<rvalue>& a);
		result_type operator()(const int_and<rvalue>& a);
		result_type operator()(const int_or<rvalue>& a);
		result_type operator()(const int_xor<rvalue>& a);
		result_type operator()(const int_call<rvalue>& a);
		result_type operator()(const int_lift<rvalue>& a);
		result_type operator()(const univ_nop<rvalue>& a);
		result_type operator()(const univ_phi<rvalue>& a);

	protected:
		boost::optional<const environment<rvalue>&> _environment;
		rvalue normalize(const rvalue& v) const;
	};

	template<>
	struct domain_traits<concrete_domain>
	{
		using value_type = rvalue;
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

	using kset_set = std::set<constant>;
	using kset_value = boost::variant<meet_t,kset_set,join_t>;

	std::ostream& operator<<(std::ostream&,const kset_set&);

	/**
	 * @internal
	 */
	template<unsigned int k>
	struct interpreter<kset_domain<k>> : public boost::static_visitor<kset_value>
	{
		template<typename Tag, typename Domain, typename Codomain>
		kset_value operator()(const binop<Tag,Domain,Codomain,kset_value>& a)
		{
			if(boost::get<join_t>(&a.right) || boost::get<join_t>(&a.left))
				return join;
			else if(boost::get<meet_t>(&a.right) || boost::get<meet_t>(&a.left))
				return meet;
			else
			{
				const kset_set &r = boost::get<kset_set>(a.right);
				const kset_set &l = boost::get<kset_set>(a.left);
				kset_set t;

				for(constant r_rv: r)
					for(constant l_rv: l)
					{
						instr::operation op{binop<Tag,Domain,Codomain,rvalue>{l_rv,r_rv}};
						rvalue r = boost::apply_visitor(_interpreter,op);
						if(is_constant(r))
							t.insert(to_constant(r));
					}
				return t.size() > k ? kset_value(join) : kset_value{t};
			}
		}

		template<typename Tag, typename Domain, typename Codomain>
		kset_value operator()(const unop<Tag,Domain,Codomain,kset_value>& a)
		{
			const kset_set *s = boost::get<kset_set>(&a.right);
			if(s)
			{
				kset_set t;
				for(constant c: *s)
				{
					instr::operation op{unop<Tag,Domain,Codomain,rvalue>{c}};
					rvalue rv = boost::apply_visitor(_interpreter,op);
					if(is_constant(rv))
						t.insert(to_constant(rv));
				}
				return t.size() > k ? kset_value(join) : kset_value{t};
			}
			else
				return a.right;
		}

		template<typename Tag, typename Domain, typename Codomain>
		kset_value operator()(const naryop<Tag,Domain,Codomain,kset_value>& a)
		{
			if(typeid(a) == typeid(const univ_phi<kset_value>&))
				return accumulate(a.operands.begin(),a.operands.end(),kset_value(meet),
						[&](const kset_value& a, const kset_value& b) { return supremum(a,b,kset_domain<k>()); });
			else
				return join;
		}

	protected:
		concrete_interpreter _interpreter;
	};

	template<unsigned int k>
	struct domain_traits<kset_domain<k>>
	{
		using value_type = kset_value;
	};

	/// Computes the supremum of two sscp lattice elements
	template<unsigned int k>
	kset_value supremum(const kset_value &a, const kset_value &b, kset_domain<k>)
	{
		if(get<meet_t>(&a)) return b;
		if(get<meet_t>(&b)) return a;
		if(get<join_t>(&a) || get<join_t>(&b)) return join;

		const kset_set &sa = get<kset_set>(a);
		const kset_set &sb = get<kset_set>(b);
		kset_set ret;

		std::set_union(sa.begin(),sa.end(),sb.begin(),sb.end(),std::inserter(ret,ret.begin()));
		if(ret.size() > k)
			return join;
		else
			return ret;
	}

	template<unsigned int k>
	kset_value overapproximate(rvalue v,kset_domain<k>)
	{
		if(is_constant(v))
			return kset_value(kset_set({to_constant(v).content()}));
		else if(is_undefined(v))
			return kset_value(join);
		else
			return kset_value(meet);
	}

	// intervals
	// octagons
	// ric
}
