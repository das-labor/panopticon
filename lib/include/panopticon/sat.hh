#include <panopticon/procedure.hh>
#include <panopticon/value.hh>

#include <cvc4/cvc4.h>

#pragma once

/**
 * @file SMT Solver interface
 *
 * Various analysis algorithms depend on SAT/SMT solvers.
 * This file defines an interface to abstract the solver
 * from the rest of Panopticon.
 *
 * @note Currently ony CVC4 is supported
 */

namespace po
{
   /**
	* @brief Single Bitvector expression
	*
	* Wraps the CVC4 Expr class. Includes the desired
	* width of the result.
	*/
	struct expr
	{
		expr(void);
		expr(const CVC4::Expr &e);

		expr &operator=(const CVC4::Expr &e);

		CVC4::Expr bitvector;
		unsigned int width;
	};

	std::ostream &operator<<(std::ostream &os, const po::expr &e);

	/// Truncate/Extend a Bitvector expression
	expr adjust_width(const expr &e, unsigned int w);

	/**
	 * @brief Formula over QF_BV
	 *
	 * A conjunction of bit vector expressions of the
	 * form x := Phi(X).
	 */
	struct formula
	{
		formula(void) : manager(), expressions() {}
		CVC4::ExprManager manager;
		std::unordered_map<variable,expr> expressions; ///< Keyed by variable defined by the expression
	};

	typedef std::shared_ptr<formula> formula_ptr;
	std::ostream &operator<<(std::ostream &os, const po::formula &f);

	/// Converts a procedure into a SAT expression over the QF_BV theory
	formula_ptr sat(proc_loc p);
}
