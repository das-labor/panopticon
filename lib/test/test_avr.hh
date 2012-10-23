#include <iostream>
#include <algorithm>
#include <iterator>

#include <cppunit/extensions/HelperMacros.h>
#include <flowgraph.hh>
#include <decoder.hh>
#include <avr.hh>

class CodeGeneratorTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(CodeGeneratorTest);
	CPPUNIT_TEST(test);
	CPPUNIT_TEST_SUITE_END();

public:

	void setUp(void)
	{
		return;
	}

	void tearDown(void)
	{
		return;
	}

	void test(void)
	{
		vector<char> bytes({'A','A','B','A','C'});
		decoder<test_tag> main, sub;
		proc_ptr proc(new procedure());
		
	}
};
