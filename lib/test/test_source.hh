#include <cppunit/extensions/HelperMacros.h>
#include <source.hh>

class SourceTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(SourceTest);
	CPPUNIT_TEST(testRange);
	CPPUNIT_TEST_SUITE_END();

public:
	void testRange(void)
	{
		po::range<po::addr_t> r1;
	}
};
