#include <gtest/gtest.h>

#include <panopticon/value.hh>
#include <panopticon/loc.hh>

using namespace po;

TEST(value,lvalue)
{
	lvalue l1,l2;
	ASSERT_TRUE(l1 == l2);
	ASSERT_TRUE(!(l1 != l2));
	ASSERT_TRUE(!(l1 < l2));

	lvalue l3(l1);

	ASSERT_TRUE(l1 == l2);
	ASSERT_TRUE(l1 == l3);
	ASSERT_TRUE(l2 == l3);

	ASSERT_TRUE(!is_constant(l1));
	ASSERT_TRUE(is_undefined(l1));
	ASSERT_TRUE(!is_variable(l1));
	ASSERT_TRUE(!is_memory(l1));

	undefined u;
	ASSERT_TRUE(u == l3);
}

TEST(value,constant)
{
	constant c1(1),c2(1), c3(7), c4(7);

	ASSERT_TRUE(is_constant(c1));
	ASSERT_TRUE(is_constant(c2));
	ASSERT_TRUE(is_constant(c3));
	ASSERT_TRUE(is_constant(c4));

	ASSERT_TRUE(c1 == c2);
	ASSERT_TRUE(!(c1 != c2));

	ASSERT_TRUE(c3 == c4);
	ASSERT_TRUE(!(c3 != c4));

	ASSERT_TRUE(c1 != c4);
	ASSERT_TRUE(!(c1 == c4));

	ASSERT_TRUE(c1.content() == 1);

	ASSERT_TRUE(c3.content() == 7);

	ASSERT_TRUE(c1 < c3);
	ASSERT_TRUE(to_constant(rvalue(c1)) == c1);

	rvalue rv;
	rv = c3;
	ASSERT_TRUE(to_constant(rv) == c3);
}

TEST(value,variable)
{
	variable v1("a",2,0), v2("b",3), v3("a",1,88);

	ASSERT_TRUE(v1 != v2);
	ASSERT_TRUE(v1 != v3);
	ASSERT_TRUE(v3 != v2);

	ASSERT_THROW(variable v4("",0x100), value_exception);
	ASSERT_THROW(variable v4("a",0), value_exception);

	ASSERT_TRUE(v1.name() == std::string("a"));
	ASSERT_TRUE(v1.width() == 2);
	ASSERT_TRUE(v1.subscript() == 0);

	ASSERT_TRUE(v2.name() == std::string("b"));
	ASSERT_TRUE(v2.width() == 3);
	ASSERT_TRUE(v2.subscript() == -1);

	ASSERT_TRUE(v3.name() == std::string("a"));
	ASSERT_TRUE(v3.width() == 1);
	ASSERT_TRUE(v3.subscript() == 88);

	ASSERT_TRUE(v1 != variable("a",1,0));
	ASSERT_TRUE(v1 != variable("a",2,1));

	ASSERT_TRUE(to_variable(rvalue(v1)) == v1);
	ASSERT_TRUE(to_variable(lvalue(v1)) == v1);

	v1 = v2;
	ASSERT_TRUE(v1 == v2);
	ASSERT_TRUE(v1.name() == std::string("b"));
	ASSERT_TRUE(v1.width() == 3);
	ASSERT_TRUE(v1.subscript() == -1);

	rvalue rv;
	rv = v3;
	ASSERT_TRUE(to_variable(rv) == v3);

	lvalue lv;
	lv = v2;
	ASSERT_TRUE(to_variable(lv) == v2);
}

TEST(value,memory)
{
	memory m1(constant(1),2,BigEndian,"n"), m2(constant(2),1,BigEndian,"n"), m3(constant(3),2,LittleEndian,"n");

	ASSERT_TRUE(m1 != m2);
	ASSERT_TRUE(m1 != m3);
	ASSERT_TRUE(m3 != m2);

	ASSERT_THROW(memory m4(undefined(),32,BigEndian,""), value_exception);
	ASSERT_THROW(memory m4(undefined(),0,BigEndian,""), value_exception);

	ASSERT_TRUE(m1.offset() == constant(1));
	ASSERT_TRUE(m1.name() == "n");
	ASSERT_TRUE(m1.bytes() == 2);
	ASSERT_TRUE(m1.endianess() == BigEndian);

	m1 = m2;
	ASSERT_TRUE(m1 == m2);
	ASSERT_TRUE(m1.offset() == constant(2));
	ASSERT_TRUE(m1.name() == "n");
	ASSERT_TRUE(m1.bytes() == 1);
	ASSERT_TRUE(m1.endianess() == BigEndian);

	ASSERT_TRUE(to_memory(lvalue(m1)) == m1);

	rvalue rv;
	rv = m3;
	ASSERT_TRUE(to_memory(rv) == m3);

	lvalue lv;
	lv = m2;
	ASSERT_TRUE(to_memory(lv) == m2);
}

TEST(value,marshal)
{
	auto rand = boost::uuids::random_generator();
	loc<rvalue> a(rand(),new rvalue(undefined()));
	loc<rvalue> b(rand(),new rvalue(constant(42)));
	loc<rvalue> c(rand(),new rvalue(variable("test",8)));
	loc<rvalue> d(rand(),new rvalue(memory(rvalue(constant(5)),2,LittleEndian,"bank1")));

	rdf::storage store;
	save_point(store);
	ASSERT_GT(store.count(),0);

	a.remove();
	b.remove();
	c.remove();
	d.remove();

	save_point(store);
	ASSERT_EQ(store.count(),0);
}
