#include <cppunit/extensions/HelperMacros.h>
#include <graph.hh>

class GraphTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(GraphTest);
	CPPUNIT_TEST(testConcepts);
	CPPUNIT_TEST_SUITE_END();

public:
	void testConcepts(void)
	{
		BOOST_CONCEPT_ASSERT((boost::GraphConcept<po::graph<int,std::string>>));
  	BOOST_CONCEPT_ASSERT((boost::VertexAndEdgeListGraphConcept<po::graph<int,std::string>>));
  	BOOST_CONCEPT_ASSERT((boost::BidirectionalGraphConcept<po::graph<int,std::string>>));
	}
};
