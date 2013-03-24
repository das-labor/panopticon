#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <cppunit/extensions/HelperMacros.h>

#include <procedure.hh>

#include "test_architecture.hh"

using namespace std;

class ProcedureTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(ProcedureTest);
	CPPUNIT_TEST(testAddSingle);
	CPPUNIT_TEST_SUITE_END();

public:
	void testAddSingle(void)
	{
		return;
	}
};
