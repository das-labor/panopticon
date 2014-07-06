#include <vector>
#include <iostream>

#include <boost/variant.hpp>
#include <panopticon/value.hh>

#pragma once

namespace po
{
	template<typename Symbol, typename Domain, typename Codomain, typename Value>
	struct unop
	{
		bool operator==(const unop<Symbol,Domain,Codomain,Value>& o) const { return right == o.right; }
		Value right;
	};

	template<typename Symbol, typename Domain, typename Codomain, typename Value>
	struct binop
	{
		bool operator==(const binop<Symbol,Domain,Codomain,Value>& o) const { return right == o.right && left == o.left; }
		Value left;
		Value right;
	};

	template<typename Symbol, typename Domain, typename Codomain, typename Value>
	struct naryop
	{
		bool operator==(const naryop<Symbol,Domain,Codomain,Value>& o) const { return operands == o.operands; }
		std::vector<Value> operands;
	};

	struct logic_domain {};
	struct integer_domain {};
	struct rational_domain {};
	using universe_domain = boost::variant<logic_domain,integer_domain>;

	struct and_symbol {};
	struct negation_symbol {};
	struct inclusive_or_symbol {};
	struct exclusive_or_symbol {};
	struct implication_symbol {};
	struct equivalence_symbol {};
	struct phi_symbol {};
	struct chi_symbol {};
	struct mu_symbol {};
	struct add_symbol {};
	struct subtract_symbol {};
	struct multiply_symbol {};
	struct divide_symbol {};
	struct modulo_symbol {};
	struct less_symbol {};
	struct equal_symbol {};
	struct lift_symbol {};
	struct call_symbol {};
	struct nop_symbol {};

	template<typename Value> using logic_and = binop<and_symbol,logic_domain,logic_domain,Value>;
	template<typename Value> using logic_or = binop<inclusive_or_symbol,logic_domain,logic_domain,Value>;
	template<typename Value> using logic_neg = unop<negation_symbol,logic_domain,logic_domain,Value>;
	template<typename Value> using logic_impl = binop<implication_symbol,logic_domain,logic_domain,Value>;
	template<typename Value> using logic_equiv = binop<equivalence_symbol,logic_domain,logic_domain,Value>;
	template<typename Value> using logic_lift = unop<lift_symbol,logic_domain,integer_domain,Value>;

	template<typename Value> using int_and = binop<and_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_or = binop<inclusive_or_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_xor = binop<exclusive_or_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_add = binop<add_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_sub = binop<subtract_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_mul = binop<multiply_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_div = binop<divide_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_mod = binop<modulo_symbol,integer_domain,integer_domain,Value>;
	template<typename Value> using int_less = binop<less_symbol,integer_domain,logic_domain,Value>;
	template<typename Value> using int_equal = binop<equal_symbol,integer_domain,logic_domain,Value>;
	template<typename Value> using int_call = unop<call_symbol,logic_domain,integer_domain,Value>;

	template<typename Value> using univ_phi = naryop<phi_symbol,universe_domain,universe_domain,Value>;
	template<typename Value> using univ_nop = unop<nop_symbol,universe_domain,universe_domain,Value>;

	template<typename T>
	struct has_symbol_visitor : public boost::static_visitor<bool>
	{
		template<typename Domain,typename Codomain,typename Value>
		bool operator()(unop<T,Domain,Codomain,Value>) const { return true; }

		template<typename Domain,typename Codomain,typename Value>
		bool operator()(binop<T,Domain,Codomain,Value>) const { return true; }

		template<typename Domain,typename Codomain,typename Value>
		bool operator()(naryop<T,Domain,Codomain,Value>) const { return true; }

		template<typename Symbol,typename Domain,typename Codomain,typename Value>
		bool operator()(unop<Symbol,Domain,Codomain,Value>) const { return false; }

		template<typename Symbol,typename Domain,typename Codomain,typename Value>
		bool operator()(binop<Symbol,Domain,Codomain,Value>) const { return false; }

		template<typename Symbol,typename Domain,typename Codomain,typename Value>
		bool operator()(naryop<Symbol,Domain,Codomain,Value>) const { return false; }
	};

	template<typename Value>
	using basic_operation = boost::variant<
		logic_and<Value>,
		logic_or<Value>,
		logic_neg<Value>,
		logic_impl<Value>,
		logic_equiv<Value>,
		logic_lift<Value>,
		univ_phi<Value>,
		univ_nop<Value>,
		int_and<Value>,
		int_or<Value>,
		int_xor<Value>,
		int_add<Value>,
		int_sub<Value>,
		int_mul<Value>,
		int_div<Value>,
		int_mod<Value>,
		int_less<Value>,
		int_equal<Value>,
		int_call<Value>
	>;

	/**
	 * @brief Single IL statement
	 *
	 * In order to allow code analysis algorithms to
	 * be implemented in a instruction set-agnostic manner,
	 * all opcodes are translated into a intermediate
	 * language first. Analysis is done on the IL and the
	 * results are mapped back to the original code.
	 *
	 * Every instance of the instr class models on IL statement.
	 * Each statement has the form a := f(b,...,z) where @c f is
	 * a @ref Function defined in the IL, @c b to @z its
	 * arguments (currently up to 3) and @c a is the variable
	 * receiving the result for @c f.
	 */
	struct instr
	{
		using operation = basic_operation<rvalue>;

		/// Construct a statement applying function @arg fn to @arg args. Saves the result in @arg a
		instr(const operation& op, const lvalue& a) : function(op), assignee(a) {}

		bool operator==(const instr& i) const { return function == i.function && assignee == i.assignee; }

		operation function;
		lvalue assignee;
	};

	std::vector<rvalue> operands(const instr&);
	void set_operands(instr&, const std::vector<rvalue>&);
	std::ostream& operator<<(std::ostream &os, const instr &i);

	/// Pretty print the function
	std::string pretty(const instr::operation& fn);

	/// Returns a string suitable for describing the function in RDF
	std::string symbolic(const instr::operation& fn);

	/// Maps a string returned from @ref symbolic back the enum value
	instr::operation from_symbolic(const std::string &s, const std::vector<rvalue>&);
}
