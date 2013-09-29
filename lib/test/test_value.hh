#include <iostream>
#include <algorithm>
#include <iterator>

#include <cppunit/extensions/HelperMacros.h>

#include <value.hh>

class ValueTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(ValueTest);
	CPPUNIT_TEST(testLValue);
	CPPUNIT_TEST(testConstant);
	CPPUNIT_TEST(testVariable);
	CPPUNIT_TEST(testMemory);
	CPPUNIT_TEST_SUITE_END();

public:
	void testLValue(void)
	{
		po::lvalue l1,l2;

		CPPUNIT_ASSERT(l1 == l2);
		CPPUNIT_ASSERT(!(l1 != l2));
		CPPUNIT_ASSERT(!(l1 < l2));

		po::lvalue l3(l1);

		CPPUNIT_ASSERT(l1 == l2);
		CPPUNIT_ASSERT(l1 == l3);
		CPPUNIT_ASSERT(l2 == l3);

		CPPUNIT_ASSERT(!l1.is_constant());
		CPPUNIT_ASSERT(l1.is_undefined());
		CPPUNIT_ASSERT(!l1.is_variable());
		CPPUNIT_ASSERT(!l1.is_memory());

		po::undefined u;
		CPPUNIT_ASSERT(u == l3);
	}

	void testConstant(void)
	{
		po::constant c1(1,3),c2(1,30), c3(0xff,3), c4(7,3);

		CPPUNIT_ASSERT(c1.is_constant());
		CPPUNIT_ASSERT(c2.is_constant());
		CPPUNIT_ASSERT(c3.is_constant());
		CPPUNIT_ASSERT(c4.is_constant());

		CPPUNIT_ASSERT(c1 == c2);
		CPPUNIT_ASSERT(!(c1 != c2));

		CPPUNIT_ASSERT(c3 == c4);
		CPPUNIT_ASSERT(!(c3 != c4));

		CPPUNIT_ASSERT(c1 != c4);
		CPPUNIT_ASSERT(!(c1 == c4));

		CPPUNIT_ASSERT(c1.content() == 1);
		CPPUNIT_ASSERT(c1.width() == 3);

		CPPUNIT_ASSERT(c3.content() == 7);
		CPPUNIT_ASSERT(c3.width() == 3);

		CPPUNIT_ASSERT(c1 < c3);
		CPPUNIT_ASSERT(po::rvalue(c1).to_constant() == c1);

		po::rvalue rv;
		rv = c3;
		CPPUNIT_ASSERT(rv.to_constant() == c3);
	}

	void testVariable(void)
	{
		po::variable v1("a",2,0), v2("b",3), v3("a",1,88);

		CPPUNIT_ASSERT(v1 != v2);
		CPPUNIT_ASSERT(v1 != v3);
		CPPUNIT_ASSERT(v3 != v2);

		CPPUNIT_ASSERT_THROW(po::variable v4("aaaaaa",32), po::value_exception);
		CPPUNIT_ASSERT_THROW(po::variable v4("aaaaa",0x100), po::value_exception);
		CPPUNIT_ASSERT_THROW(po::variable v4("",0x100), po::value_exception);

		CPPUNIT_ASSERT(v1.name() == std::string("a"));
		CPPUNIT_ASSERT(v1.width() == 2);
		CPPUNIT_ASSERT(v1.subscript() == 0);

		CPPUNIT_ASSERT(v2.name() == std::string("b"));
		CPPUNIT_ASSERT(v2.width() == 3);
		CPPUNIT_ASSERT(v2.subscript() == -1);

		CPPUNIT_ASSERT(v3.name() == std::string("a"));
		CPPUNIT_ASSERT(v3.width() == 1);
		CPPUNIT_ASSERT(v3.subscript() == 88);

		CPPUNIT_ASSERT(v1 != po::variable("a",1,0));
		CPPUNIT_ASSERT(v1 != po::variable("a",2,1));

		CPPUNIT_ASSERT(po::rvalue(v1).to_variable() == v1);
		CPPUNIT_ASSERT(po::lvalue(v1).to_variable() == v1);

		v1 = v2;
		CPPUNIT_ASSERT(v1 == v2);
		CPPUNIT_ASSERT(v1.name() == std::string("b"));
		CPPUNIT_ASSERT(v1.width() == 3);
		CPPUNIT_ASSERT(v1.subscript() == -1);

		po::rvalue rv;
		rv = v3;
		CPPUNIT_ASSERT(rv.to_variable() == v3);

		po::lvalue lv;
		lv = v2;
		CPPUNIT_ASSERT(lv.to_variable() == v2);
	}

	void testMemory(void)
	{
		po::memory m1(po::constant(1,1),2,po::memory::BigEndian,"n"), m2(po::constant(2,44),1,po::memory::BigEndian,"n"), m3(po::constant(3,3),2,po::memory::LittleEndian,"n");

		CPPUNIT_ASSERT(m1 != m2);
		CPPUNIT_ASSERT(m1 != m3);
		CPPUNIT_ASSERT(m3 != m2);

		CPPUNIT_ASSERT_THROW(po::memory m4(po::undefined(),32,po::memory::BigEndian,""), po::value_exception);
		CPPUNIT_ASSERT_THROW(po::memory m4(po::undefined(),0,po::memory::BigEndian,""), po::value_exception);

		CPPUNIT_ASSERT(m1.offset() == po::constant(1,1));
		CPPUNIT_ASSERT(m1.name() == "n");
		CPPUNIT_ASSERT(m1.bytes() == 2);
		CPPUNIT_ASSERT(m1.endianess() == po::memory::BigEndian);

		m1 = m2;
		CPPUNIT_ASSERT(m1 == m2);
		CPPUNIT_ASSERT(m1.offset() == po::constant(2,44));
		CPPUNIT_ASSERT(m1.name() == "n");
		CPPUNIT_ASSERT(m1.bytes() == 1);
		CPPUNIT_ASSERT(m1.endianess() == po::memory::BigEndian);

		CPPUNIT_ASSERT(po::lvalue(m1).to_memory() == m1);

		po::rvalue rv;
		rv = m3;
		CPPUNIT_ASSERT(rv.to_memory() == m3);

		po::lvalue lv;
		lv = m2;
		CPPUNIT_ASSERT(lv.to_memory() == m2);
	}
};
