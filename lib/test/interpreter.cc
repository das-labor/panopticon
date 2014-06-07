#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <gtest/gtest.h>

#include <panopticon/interpreter.hh>

using namespace po;
using namespace boost;

TEST(interpreter,concrete_semantics)
{
	environment<domain_traits<concrete_domain>::value_type> env;
	concrete_interpreter i(env);
	instr::operation op = logic_and{constant(true),constant(true)};

	ASSERT_EQ(constant(true), boost::apply_visitor(i,op));
	op = logic_or{constant(true),undefined()};
	ASSERT_EQ(constant(true), boost::apply_visitor(i,op));
	op = int_add{variable("a",8),constant(0)};
	ASSERT_EQ(variable("a",8), boost::apply_visitor(i,op));
	env[variable("b",8)] = constant(33);
	op = int_add{variable("b",8),constant(11)};
	ASSERT_EQ(constant(44), boost::apply_visitor(i,op));
	op = univ_phi{{constant(11),constant(11)}};
	ASSERT_EQ(constant(11), boost::apply_visitor(i,op));
	op = univ_phi{{constant(33),variable("b",8)}};
	ASSERT_EQ(constant(33), boost::apply_visitor(i,op));
}
