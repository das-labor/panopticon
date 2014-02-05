#include <gtest/gtest.h>
#include <panopticon/digraph.hh>

TEST(digraph,concepts)
{
	BOOST_CONCEPT_ASSERT((boost::GraphConcept<po::digraph<int,std::string>>));
	BOOST_CONCEPT_ASSERT((boost::VertexAndEdgeListGraphConcept<po::digraph<int,std::string>>));
	BOOST_CONCEPT_ASSERT((boost::BidirectionalGraphConcept<po::digraph<int,std::string>>));

	BOOST_CONCEPT_ASSERT((boost::ReadablePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
	BOOST_CONCEPT_ASSERT((boost::WritablePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
	BOOST_CONCEPT_ASSERT((boost::ReadWritePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
	BOOST_CONCEPT_ASSERT((boost::LvaluePropertyMapConcept<po::unordered_pmap<int,std::string>,int>));
}

TEST(digraph,usage)
{
	po::digraph<int,std::string> g;

	auto n1 = g.insert_node(42);
	auto n2 = g.insert_node(13);
	auto n3 = g.insert_node(1337);

	auto e12 = g.insert_edge("a",n1,n2);
	auto e23 = g.insert_edge("b",n2,n3);
	auto e31 = g.insert_edge("c",n3,n1);

	ASSERT_NE(n1, n2);
	ASSERT_NE(n1, n3);
	ASSERT_NE(n2, n3);

	ASSERT_NE(e12, e23);
	ASSERT_NE(e12, e31);
	ASSERT_NE(e23, e31);

	ASSERT_EQ(g.get_node(n1), 42);
	ASSERT_EQ(g.get_node(n2), 13);
	ASSERT_EQ(g.get_node(n3), 1337);

	ASSERT_EQ(g.get_edge(e12), "a");
	ASSERT_EQ(g.get_edge(e23), "b");
	ASSERT_EQ(g.get_edge(e31), "c");

	ASSERT_EQ(g.num_edges(), 3);
	ASSERT_EQ(g.num_nodes(), 3);

	ASSERT_EQ(g.source(e12), n1);
	ASSERT_EQ(g.target(e12), n2);
	ASSERT_EQ(g.source(e23), n2);
	ASSERT_EQ(g.target(e23), n3);
	ASSERT_EQ(g.source(e31), n3);
	ASSERT_EQ(g.target(e31), n1);

	ASSERT_EQ(out_degree(n1,g), 1);
	ASSERT_EQ(out_degree(n2,g), 1);
	ASSERT_EQ(out_degree(n3,g), 1);

	ASSERT_EQ(in_degree(n1,g), 1);
	ASSERT_EQ(in_degree(n2,g), 1);
	ASSERT_EQ(in_degree(n3,g), 1);

	ASSERT_EQ(degree(n1,g), 2);
	ASSERT_EQ(degree(n2,g), 2);
	ASSERT_EQ(degree(n3,g), 2);

	g.remove_edge(e12);
	g.remove_node(n1);
	g.remove_node(n2);
	g.remove_node(n3);

	ASSERT_EQ(g.num_nodes(), 0);
	ASSERT_EQ(g.num_edges(), 0);
}

TEST(digraph,out_iterator)
{
	po::digraph<int,std::string> g;

	auto n1 = g.insert_node(42);
	auto n2 = g.insert_node(13);
	auto n3 = g.insert_node(1337);
	auto n4 = g.insert_node(99);

	auto e12 = g.insert_edge("a",n1,n2);
	auto e23 = g.insert_edge("b",n2,n3);
	auto e21 = g.insert_edge("c",n2,n1);
	auto e14 = g.insert_edge("d",n1,n4);

	auto i = g.out_edges(n1);
	ASSERT_TRUE((*i.first == e12 && *std::next(i.first) == e14) || (*i.first == e14 && *std::next(i.first) == e12));
	ASSERT_EQ(std::next(i.first,2), i.second);

	i = g.out_edges(n2);
	ASSERT_TRUE((*i.first == e23 && *std::next(i.first) == e21) || (*i.first == e21 && *std::next(i.first) == e23));
	ASSERT_EQ(std::next(i.first,2), i.second);

	i = g.out_edges(n3);
	ASSERT_EQ(i.first, i.second);

	i = g.out_edges(n4);
	ASSERT_EQ(i.first, i.second);
}

TEST(digraph,in_iterator)
{
	po::digraph<int,std::string> g;

	auto n1 = g.insert_node(42);
	auto n2 = g.insert_node(13);
	auto n3 = g.insert_node(1337);
	auto n4 = g.insert_node(99);

	auto e12 = g.insert_edge("a",n1,n2);
	auto e23 = g.insert_edge("b",n2,n3);
	auto e21 = g.insert_edge("c",n2,n1);
	auto e14 = g.insert_edge("d",n1,n4);

	auto i = g.in_edges(n1);
	ASSERT_TRUE(*i.first == e21 && std::next(i.first) == i.second);

	i = g.in_edges(n2);
	ASSERT_TRUE(*i.first == e12 && std::next(i.first) == i.second);

	i = g.in_edges(n3);
	ASSERT_TRUE(*i.first == e23 && std::next(i.first) == i.second);

	i = g.in_edges(n4);
	ASSERT_TRUE(*i.first == e14 && std::next(i.first) == i.second);
}

TEST(digraph,iterators)
{
	po::digraph<int,std::string> g;

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
	std::for_each(i.first,i.second,[&](const po::descriptor<int> &n) { ASSERT_TRUE(ns.insert(n).second); });

	auto j = g.edges();
	std::unordered_set<po::descriptor<std::string>> es;
	std::for_each(j.first,j.second,[&](const po::descriptor<std::string> &n) { ASSERT_TRUE(es.insert(n).second); });

	ASSERT_EQ(ns.size(), 4);
	ASSERT_EQ(es.size(), 4);
}

TEST(digraph,error)
{
	po::digraph<int,std::string> g1,g2;

	auto n1 = g1.insert_node(42);
	auto n2 = g1.insert_node(13);
	g1.insert_node(13);

	g1.insert_edge("a",n1,n2);
	g1.insert_edge("b",n1,n2);

	ASSERT_EQ(g1.num_edges(), 2);
	ASSERT_EQ(g1.num_nodes(), 2);

	auto n3 = g2.insert_node(42);
	ASSERT_THROW(g1.get_node(n3),std::out_of_range);
	ASSERT_THROW(g1.out_edges(n3),std::out_of_range);
	ASSERT_THROW(g1.in_edges(n3),std::out_of_range);
	ASSERT_THROW(out_degree(n3,g1),std::out_of_range);
	ASSERT_THROW(in_degree(n3,g1),std::out_of_range);
	ASSERT_THROW(degree(n3,g1),std::out_of_range);

	auto n4 = g2.insert_node(422);
	auto e = g2.insert_edge("dd",n3,n4);
	ASSERT_THROW(g1.get_edge(e),std::out_of_range);
	ASSERT_THROW(g1.source(e),std::out_of_range);
	ASSERT_THROW(g1.target(e),std::out_of_range);

	ASSERT_NE(n1, n3);
}
