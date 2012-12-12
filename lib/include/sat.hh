#ifndef SAT_HH
#define SAT_HH

#include <procedure.hh>
#include <value.hh>

#include <cvc4/cvc4.h>

namespace po
{
	struct expr
	{
		expr(void);
		expr(::CVC4::Expr e);
		//void operator=(const expr &e);
		//void operator=(const ::CVC4::Expr &e);

		::CVC4::Expr bitvector;
		unsigned int width;
	};
	
	::std::ostream &operator<<(::std::ostream &os, const po::expr &e);
	expr adjust_width(const expr &e, unsigned int w);

	struct formula
	{
		::CVC4::ExprManager manager;
		::std::map<variable,expr> expressions;
	};
	
	typedef ::std::shared_ptr<formula> formula_ptr;
	::std::ostream &operator<<(::std::ostream &os, const po::formula &f);
	formula_ptr sat(proc_ptr p);
}


#endif
