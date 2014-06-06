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
}
