#include <iostream>

#include <cppunit/extensions/HelperMacros.h>

#include <inflate.hh>
#include <procedure.hh>
#include <flowgraph.hh>

class InflateTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(InflateTest);
	CPPUNIT_TEST(testFlowgraph);
	CPPUNIT_TEST_SUITE_END();

public:
	void testFlowgraph(void)
	{
		po::flow_ptr f(new po::flowgraph());
		po::proc_ptr p1(new po::procedure()), p2(new po::procedure());

		f->name = "flow1";
		p1->name = "proc1";
		p2->name = "proc2";
		f->procedures.insert(p1);
		f->procedures.insert(p2);

		po::odotstream os;

		os << f << std::endl;
		std::cout << std::endl << os.str();
	}
};
