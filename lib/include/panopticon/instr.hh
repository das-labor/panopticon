#include <vector>
#include <iostream>

#include <boost/variant.hpp>
#include <panopticon/value.hh>

#pragma once

namespace po
{
	template<typename Symbol, typename Domain, typename Codomain>
	struct unop
	{
		rvalue right;
	};

	template<typename Symbol, typename Domain, typename Codomain>
	struct binop
	{
		rvalue left;
		rvalue right;
	};

	struct logic_domain {};
	struct integer_domain {};
	struct rational_domain {};

	struct and_symbol {};
	struct or_symbol {};
	struct negation_symbol {};
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

	using logic_and = binop<and_symbol,logic_domain,logic_domain>;
	using logic_or = binop<or_symbol,logic_domain,logic_domain>;
	using logic_neg = unop<negation_symbol,logic_domain,logic_domain>;
	using logic_nop = unop<nop_symbol,logic_domain,logic_domain>;
	using logic_impl = binop<implication_symbol,logic_domain,logic_domain>;
	using logic_equiv = binop<equivalence_symbol,logic_domain,logic_domain>;
	using logic_phi = binop<phi_symbol,logic_domain,logic_domain>;

	using int_and = binop<and_symbol,integer_domain,integer_domain>;
	using int_or = binop<or_symbol,integer_domain,integer_domain>;
	using int_neg = unop<negation_symbol,integer_domain,integer_domain>;
	using int_phi = binop<phi_symbol,integer_domain,integer_domain>;
	using int_add = binop<add_symbol,integer_domain,integer_domain>;
	using int_sub = binop<subtract_symbol,integer_domain,integer_domain>;
	using int_mul = binop<multiply_symbol,integer_domain,integer_domain>;
	using int_div = binop<divide_symbol,integer_domain,integer_domain>;
	using int_mod = binop<modulo_symbol,integer_domain,integer_domain>;
	using int_less = binop<less_symbol,integer_domain,logic_domain>;
	using int_equal = binop<equal_symbol,integer_domain,logic_domain>;
	using int_lift = unop<lift_symbol,logic_domain,integer_domain>;
	using int_call = unop<call_symbol,logic_domain,integer_domain>;
	using int_nop = unop<nop_symbol,logic_domain,integer_domain>;

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
		using operation = boost::variant<
			logic_and,
			logic_or,
			logic_neg,
			logic_impl,
			logic_equiv,
			logic_phi,
			int_and,
			int_or,
			int_neg,
			int_add,
			int_sub,
			int_mul,
			int_div,
			int_mod,
			int_less,
			int_equal,
			int_lift,
			int_call
		>;

		/// Construct a statement applying function @arg fn to @arg args. Saves the result in @arg a
		instr(const operation& op, const lvalue& a) : function(op), assignee(a) {}

		bool operator==(const instr&) const;
		bool operator<(const instr&) const;

		operation function;
		lvalue assignee;
	};

	std::vector<rvalue> operations(const instr&);
	std::ostream& operator<<(std::ostream &os, const instr &i);

	template<>
	instr* unmarshal(const uuid&, const rdf::storage&);

	template<>
	rdf::statements marshal(const instr*, const uuid&);

	/// Pretty print the function
	std::string pretty(const instr::operation& fn);

	/// Returns a string suitable for describing the function in RDF
	std::string symbolic(const instr::operation& fn);

	/// Maps a string returned from @ref symbolic back the enum value
	instr::operation from_symbolic(const std::string &s, const std::vector<rvalue>&);
};
