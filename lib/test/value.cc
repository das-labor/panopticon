#include <gtest/gtest.h>
#include <panopticon/value.hh>

TEST(value,lvalue)
{
	po::lvalue l1,l2;
	ASSERT_TRUE(l1 == l2);
	ASSERT_TRUE(!(l1 != l2));
	ASSERT_TRUE(!(l1 < l2));

	po::lvalue l3(l1);

	ASSERT_TRUE(l1 == l2);
	ASSERT_TRUE(l1 == l3);
	ASSERT_TRUE(l2 == l3);

	ASSERT_TRUE(!po::is_constant(l1));
	ASSERT_TRUE(po::is_undefined(l1));
	ASSERT_TRUE(!po::is_variable(l1));
	ASSERT_TRUE(!po::is_memory(l1));

	po::undefined u;
	ASSERT_TRUE(u == l3);
}

TEST(value,constant)
{
	po::constant c1(1),c2(1), c3(7), c4(7);

	ASSERT_TRUE(po::is_constant(c1));
	ASSERT_TRUE(po::is_constant(c2));
	ASSERT_TRUE(po::is_constant(c3));
	ASSERT_TRUE(po::is_constant(c4));

	ASSERT_TRUE(c1 == c2);
	ASSERT_TRUE(!(c1 != c2));

	ASSERT_TRUE(c3 == c4);
	ASSERT_TRUE(!(c3 != c4));

	ASSERT_TRUE(c1 != c4);
	ASSERT_TRUE(!(c1 == c4));

	ASSERT_TRUE(c1.content() == 1);

	ASSERT_TRUE(c3.content() == 7);

	ASSERT_TRUE(c1 < c3);
	ASSERT_TRUE(po::to_constant(po::rvalue(c1)) == c1);

	po::rvalue rv;
	rv = c3;
	ASSERT_TRUE(po::to_constant(rv) == c3);
}

TEST(value,variable)
{
	po::variable v1("a",2,0), v2("b",3), v3("a",1,88);

	ASSERT_TRUE(v1 != v2);
	ASSERT_TRUE(v1 != v3);
	ASSERT_TRUE(v3 != v2);

	ASSERT_THROW(po::variable v4("",0x100), po::value_exception);
	ASSERT_THROW(po::variable v4("a",0), po::value_exception);

	ASSERT_TRUE(v1.name() == std::string("a"));
	ASSERT_TRUE(v1.width() == 2);
	ASSERT_TRUE(v1.subscript() == 0);

	ASSERT_TRUE(v2.name() == std::string("b"));
	ASSERT_TRUE(v2.width() == 3);
	ASSERT_TRUE(v2.subscript() == -1);

	ASSERT_TRUE(v3.name() == std::string("a"));
	ASSERT_TRUE(v3.width() == 1);
	ASSERT_TRUE(v3.subscript() == 88);

	ASSERT_TRUE(v1 != po::variable("a",1,0));
	ASSERT_TRUE(v1 != po::variable("a",2,1));

	ASSERT_TRUE(po::to_variable(po::rvalue(v1)) == v1);
	ASSERT_TRUE(po::to_variable(po::lvalue(v1)) == v1);

	v1 = v2;
	ASSERT_TRUE(v1 == v2);
	ASSERT_TRUE(v1.name() == std::string("b"));
	ASSERT_TRUE(v1.width() == 3);
	ASSERT_TRUE(v1.subscript() == -1);

	po::rvalue rv;
	rv = v3;
	ASSERT_TRUE(po::to_variable(rv) == v3);

	po::lvalue lv;
	lv = v2;
	ASSERT_TRUE(po::to_variable(lv) == v2);
}

TEST(value,memory)
{
	po::memory m1(po::constant(1),2,po::memory::BigEndian,"n"), m2(po::constant(2),1,po::memory::BigEndian,"n"), m3(po::constant(3),2,po::memory::LittleEndian,"n");

	ASSERT_TRUE(m1 != m2);
	ASSERT_TRUE(m1 != m3);
	ASSERT_TRUE(m3 != m2);

	ASSERT_THROW(po::memory m4(po::undefined(),32,po::memory::BigEndian,""), po::value_exception);
	ASSERT_THROW(po::memory m4(po::undefined(),0,po::memory::BigEndian,""), po::value_exception);

	ASSERT_TRUE(m1.offset() == po::constant(1));
	ASSERT_TRUE(m1.name() == "n");
	ASSERT_TRUE(m1.bytes() == 2);
	ASSERT_TRUE(m1.endianess() == po::memory::BigEndian);

	m1 = m2;
	ASSERT_TRUE(m1 == m2);
	ASSERT_TRUE(m1.offset() == po::constant(2));
	ASSERT_TRUE(m1.name() == "n");
	ASSERT_TRUE(m1.bytes() == 1);
	ASSERT_TRUE(m1.endianess() == po::memory::BigEndian);

	ASSERT_TRUE(po::to_memory(po::lvalue(m1)) == m1);

	po::rvalue rv;
	rv = m3;
	ASSERT_TRUE(po::to_memory(rv) == m3);

	po::lvalue lv;
	lv = m2;
	ASSERT_TRUE(po::to_memory(lv) == m2);
}
