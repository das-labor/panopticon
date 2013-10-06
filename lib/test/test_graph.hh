#include <cppunit/extensions/HelperMacros.h>
#include <graph.hh>

class GraphTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(GraphTest);
	CPPUNIT_TEST(testConcepts);
	CPPUNIT_TEST(testUsage);
	CPPUNIT_TEST(testInIterator);
	CPPUNIT_TEST(testOutIterator);
	CPPUNIT_TEST(testIterators);
	CPPUNIT_TEST(testError);
	CPPUNIT_TEST_SUITE_END();

public:
	void testConcepts(void)
	{
		BOOST_CONCEPT_ASSERT((boost::GraphConcept<po::graph<int,std::string>>));
  	BOOST_CONCEPT_ASSERT((boost::VertexAndEdgeListGraphConcept<po::graph<int,std::string>>));
  	BOOST_CONCEPT_ASSERT((boost::BidirectionalGraphConcept<po::graph<int,std::string>>));

		BOOST_CONCEPT_ASSERT((boost::ReadablePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
  	BOOST_CONCEPT_ASSERT((boost::WritablePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
  	BOOST_CONCEPT_ASSERT((boost::ReadWritePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
  	BOOST_CONCEPT_ASSERT((boost::LvaluePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
	}

	void testUsage(void)
	{
		po::graph<int,std::string> g;

		auto n1 = g.insert_node(42);
		auto n2 = g.insert_node(13);
		auto n3 = g.insert_node(1337);

		auto e12 = g.insert_edge("a",n1,n2);
		auto e23 = g.insert_edge("b",n2,n3);
		auto e31 = g.insert_edge("c",n3,n1);

		CPPUNIT_ASSERT(n1 != n2);
		CPPUNIT_ASSERT(n1 != n3);
		CPPUNIT_ASSERT(n2 != n3);

		CPPUNIT_ASSERT(e12 != e23);
		CPPUNIT_ASSERT(e12 != e31);
		CPPUNIT_ASSERT(e23 != e31);

		CPPUNIT_ASSERT(g.get_node(n1) == 42);
		CPPUNIT_ASSERT(g.get_node(n2) == 13);
		CPPUNIT_ASSERT(g.get_node(n3) == 1337);

		CPPUNIT_ASSERT(g.get_edge(e12) == "a");
		CPPUNIT_ASSERT(g.get_edge(e23) == "b");
		CPPUNIT_ASSERT(g.get_edge(e31) == "c");

		CPPUNIT_ASSERT(g.num_edges() == 3);
		CPPUNIT_ASSERT(g.num_nodes() == 3);

		CPPUNIT_ASSERT(g.source(e12) == n1);
		CPPUNIT_ASSERT(g.target(e12) == n2);
		CPPUNIT_ASSERT(g.source(e23) == n2);
		CPPUNIT_ASSERT(g.target(e23) == n3);
		CPPUNIT_ASSERT(g.source(e31) == n3);
		CPPUNIT_ASSERT(g.target(e31) == n1);

		CPPUNIT_ASSERT(out_degree(n1,g) == 1);
		CPPUNIT_ASSERT(out_degree(n2,g) == 1);
		CPPUNIT_ASSERT(out_degree(n3,g) == 1);

		CPPUNIT_ASSERT(in_degree(n1,g) == 1);
		CPPUNIT_ASSERT(in_degree(n2,g) == 1);
		CPPUNIT_ASSERT(in_degree(n3,g) == 1);

		CPPUNIT_ASSERT(degree(n1,g) == 2);
		CPPUNIT_ASSERT(degree(n2,g) == 2);
		CPPUNIT_ASSERT(degree(n3,g) == 2);

		g.remove_edge(e12);
		g.remove_node(n1);
		g.remove_node(n2);
		g.remove_node(n3);

		CPPUNIT_ASSERT(g.num_nodes() == 0);
		CPPUNIT_ASSERT(g.num_edges() == 0);
	}

	void testOutIterator(void)
	{
		po::graph<int,std::string> g;

		auto n1 = g.insert_node(42);
		auto n2 = g.insert_node(13);
		auto n3 = g.insert_node(1337);
		auto n4 = g.insert_node(99);

		auto e12 = g.insert_edge("a",n1,n2);
		auto e23 = g.insert_edge("b",n2,n3);
		auto e21 = g.insert_edge("c",n2,n1);
		auto e14 = g.insert_edge("d",n1,n4);

		auto i = g.out_edges(n1);
		CPPUNIT_ASSERT((*i.first == e12 && *std::next(i.first) == e14) || (*i.first == e14 && *std::next(i.first) == e12));
		CPPUNIT_ASSERT(std::next(i.first,2) == i.second);

		i = g.out_edges(n2);
		CPPUNIT_ASSERT((*i.first == e23 && *std::next(i.first) == e21) || (*i.first == e21 && *std::next(i.first) == e23));
		CPPUNIT_ASSERT(std::next(i.first,2) == i.second);

		i = g.out_edges(n3);
		CPPUNIT_ASSERT(i.first == i.second);

		i = g.out_edges(n4);
		CPPUNIT_ASSERT(i.first == i.second);
	}

	void testInIterator(void)
	{
		po::graph<int,std::string> g;

		auto n1 = g.insert_node(42);
		auto n2 = g.insert_node(13);
		auto n3 = g.insert_node(1337);
		auto n4 = g.insert_node(99);

		auto e12 = g.insert_edge("a",n1,n2);
		auto e23 = g.insert_edge("b",n2,n3);
		auto e21 = g.insert_edge("c",n2,n1);
		auto e14 = g.insert_edge("d",n1,n4);

		auto i = g.in_edges(n1);
		CPPUNIT_ASSERT(*i.first == e21 && std::next(i.first) == i.second);

		i = g.in_edges(n2);
		CPPUNIT_ASSERT(*i.first == e12 && std::next(i.first) == i.second);

		i = g.in_edges(n3);
		CPPUNIT_ASSERT(*i.first == e23 && std::next(i.first) == i.second);

		i = g.in_edges(n4);
		CPPUNIT_ASSERT(*i.first == e14 && std::next(i.first) == i.second);
	}

	void testIterators(void)
	{
		po::graph<int,std::string> g;

		auto n1 = g.insert_node(42);
		auto n2 = g.insert_node(13);
		auto n3 = g.insert_node(1337);
		auto n4 = g.insert_node(99);

		g.insert_edge("a",n1,n2);
		g.insert_edge("b",n2,n3);
		g.insert_edge("c",n2,n1);
		g.insert_edge("d",n1,n4);

		auto i = g.nodes();
		std::unordered_set<po::descriptor<int>> ns;
		std::for_each(i.first,i.second,[&](const po::descriptor<int> &n) { CPPUNIT_ASSERT(ns.insert(n).second); });

		auto j = g.edges();
		std::unordered_set<po::descriptor<std::string>> es;
		std::for_each(j.first,j.second,[&](const po::descriptor<std::string> &n) { CPPUNIT_ASSERT(es.insert(n).second); });

		CPPUNIT_ASSERT(ns.size() == 4);
		CPPUNIT_ASSERT(es.size() == 4);
	}

	void testError(void)
	{
		po::graph<int,std::string> g1,g2;

		auto n1 = g1.insert_node(42);
		auto n2 = g1.insert_node(13);
		g1.insert_node(13);

		g1.insert_edge("a",n1,n2);
		g1.insert_edge("b",n1,n2);

		CPPUNIT_ASSERT(g1.num_edges() == 2);
		CPPUNIT_ASSERT(g1.num_nodes() == 2);

		auto n3 = g2.insert_node(42);
		CPPUNIT_ASSERT_THROW(g1.get_node(n3),std::out_of_range);
		CPPUNIT_ASSERT_THROW(g1.out_edges(n3),std::out_of_range);
		CPPUNIT_ASSERT_THROW(g1.in_edges(n3),std::out_of_range);
		CPPUNIT_ASSERT_THROW(out_degree(n3,g1),std::out_of_range);
		CPPUNIT_ASSERT_THROW(in_degree(n3,g1),std::out_of_range);
		CPPUNIT_ASSERT_THROW(degree(n3,g1),std::out_of_range);

		auto n4 = g2.insert_node(422);
		auto e = g2.insert_edge("dd",n3,n4);
		CPPUNIT_ASSERT_THROW(g1.get_edge(e),std::out_of_range);
		CPPUNIT_ASSERT_THROW(g1.source(e),std::out_of_range);
		CPPUNIT_ASSERT_THROW(g1.target(e),std::out_of_range);

		CPPUNIT_ASSERT(n1 != n3);
	}
};
