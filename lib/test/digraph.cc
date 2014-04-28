#include <gtest/gtest.h>
#include <panopticon/digraph.hh>
#include <panopticon/loc.hh>

#include <boost/graph/graph_concepts.hpp>

using namespace po;
using namespace std;

TEST(digraph,concepts)
{
	BOOST_CONCEPT_ASSERT((boost::GraphConcept<po::digraph<int,std::string>>));
	BOOST_CONCEPT_ASSERT((boost::VertexAndEdgeListGraphConcept<po::digraph<int,std::string>>));
}

TEST(digraph,node_attribute)
{
	po::digraph<int,std::string> g;

	auto n1 = insert_vertex(42,g);
	auto n2 = insert_vertex(13,g);
	auto n3 = insert_vertex(1337,g);

	ASSERT_EQ(n1, find_node(42,g));
	ASSERT_EQ(n2, find_node(13,g));
	ASSERT_EQ(n3, find_node(1337,g));

	ASSERT_THROW(find_node(69,g),out_of_range);
}

/*TEST(digraph,copy)
{
	using int_loc = loc<int>;
	po::digraph<int_loc,std::string> g1;

	auto n1 = insert_vertex(int_loc(new int(42)),g1);
	insert_vertex(int_loc(new int(13)),g1);
	insert_vertex(int_loc(new int(1337)),g1);

	po::digraph<int_loc,std::string> g2 = g1;

	ASSERT_EQ(3, num_vertices(g2));
	ASSERT_EQ(0, num_edges(g2));
	ASSERT_EQ(g1, g2);

	int_loc a = get_vertex(n1,g1);
	a.write() = 41;

	ASSERT_NE(g1, g2);
}*/

TEST(digraph,usage)
{
	po::digraph<int,std::string> g;

	auto n1 = insert_vertex(42,g);
	auto n2 = insert_vertex(13,g);
	auto n3 = insert_vertex(1337,g);

	auto e12 = insert_edge(string("a"),n1,n2,g);
	auto e23 = insert_edge(string("b"),n2,n3,g);
	auto e31 = insert_edge(string("c"),n3,n1,g);

	ASSERT_NE(n1, n2);
	ASSERT_NE(n1, n3);
	ASSERT_NE(n2, n3);

	ASSERT_NE(e12, e23);
	ASSERT_NE(e12, e31);
	ASSERT_NE(e23, e31);

	ASSERT_EQ(get_vertex(n1,g), 42);
	ASSERT_EQ(get_vertex(n2,g), 13);
	ASSERT_EQ(get_vertex(n3,g), 1337);

	ASSERT_EQ(get_edge(e12,g), string("a"));
	ASSERT_EQ(get_edge(e23,g), string("b"));
	ASSERT_EQ(get_edge(e31,g), string("c"));

	ASSERT_EQ(num_edges(g), 3);
	ASSERT_EQ(num_vertices(g), 3);

	ASSERT_EQ(source(e12,g), n1);
	ASSERT_EQ(target(e12,g), n2);
	ASSERT_EQ(source(e23,g), n2);
	ASSERT_EQ(target(e23,g), n3);
	ASSERT_EQ(source(e31,g), n3);
	ASSERT_EQ(target(e31,g), n1);

	ASSERT_EQ(out_degree(n1,g), 1);
	ASSERT_EQ(out_degree(n2,g), 1);
	ASSERT_EQ(out_degree(n3,g), 1);

	remove_edge(e12,g);

	remove_vertex(n1,g);
	remove_vertex(n2,g);
	remove_vertex(n3,g);

	ASSERT_EQ(num_vertices(g), 0);
	ASSERT_EQ(num_edges(g), 0);
}

TEST(digraph,out_iterator)
{
	po::digraph<int,std::string> g;

	auto n1 = insert_vertex(42,g);
	auto n2 = insert_vertex(13,g);
	auto n3 = insert_vertex(1337,g);
	auto n4 = insert_vertex(99,g);

	auto e12 = insert_edge(string("a"),n1,n2,g);
	auto e23 = insert_edge(string("b"),n2,n3,g);
	auto e21 = insert_edge(string("c"),n2,n1,g);
	auto e14 = insert_edge(string("d"),n1,n4,g);

	auto i = out_edges(n1,g);
	ASSERT_TRUE((*i.first == e12 && *(i.first + 1) == e14) || (*i.first == e14 && *(i.first + 1) == e12));
	ASSERT_EQ(i.first + 2, i.second);

	i = out_edges(n2,g);
	ASSERT_TRUE((*i.first == e23 && *(i.first + 1) == e21) || (*i.first == e21 && *(i.first + 1) == e23));
	ASSERT_EQ(i.first + 2, i.second);

	i = out_edges(n3,g);
	ASSERT_EQ(i.first, i.second);

	i = out_edges(n4,g);
	ASSERT_EQ(i.first, i.second);
}

TEST(digraph,in_iterator)
{
	po::digraph<int,std::string> g;

	auto n1 = insert_vertex(42,g);
	auto n2 = insert_vertex(13,g);
	auto n3 = insert_vertex(1337,g);
	auto n4 = insert_vertex(99,g);

	auto e12 = insert_edge(string("a"),n1,n2,g);
	auto e23 = insert_edge(string("b"),n2,n3,g);
	auto e21 = insert_edge(string("c"),n2,n1,g);
	auto e14 = insert_edge(string("d"),n1,n4,g);

	auto i = in_edges(n1,g);
	ASSERT_TRUE(*i.first == e21);
	ASSERT_EQ(i.first + 1, i.second);

	i = in_edges(n2,g);
	ASSERT_TRUE(*i.first == e12);
	ASSERT_EQ(i.first + 1, i.second);

	i = in_edges(n3,g);
	ASSERT_TRUE(*i.first == e23);
	ASSERT_EQ(i.first + 1, i.second);

	i = in_edges(n4,g);
	ASSERT_TRUE(*i.first == e14);
	ASSERT_EQ(i.first + 1, i.second);
}

TEST(digraph,adj_iterator)
{
	po::digraph<int,std::string> g;

	auto n1 = insert_vertex(42,g);
	auto n2 = insert_vertex(13,g);
	auto n3 = insert_vertex(1337,g);
	auto n4 = insert_vertex(99,g);

	insert_edge(string("a"),n1,n2,g);
	insert_edge(string("b"),n2,n3,g);
	insert_edge(string("c"),n2,n1,g);
	insert_edge(string("d"),n1,n4,g);

	auto i = adjacent_vertices(n1,g);
	ASSERT_TRUE((*i.first == n2 && *(i.first + 1) == n4) || (*i.first == n4 && *(i.first + 1) == n2));
	ASSERT_EQ(std::distance(i.first ,i.second), 2);

	i = adjacent_vertices(n2,g);
	ASSERT_TRUE((*i.first == n1 && *(i.first + 1) == n3) || (*i.first == n3 && *(i.first + 1) == n1));
	ASSERT_EQ(std::distance(i.first ,i.second), 2);

	i = adjacent_vertices(n3,g);
	ASSERT_TRUE(*i.first == n2);
	ASSERT_EQ(std::distance(i.first ,i.second), 1);

	i = adjacent_vertices(n4,g);
	ASSERT_TRUE(*i.first == n1);
	ASSERT_EQ(std::distance(i.first ,i.second), 1);
}

TEST(digraph,iterators)
{
	po::digraph<int,std::string> g;

	auto n1 = insert_vertex(42,g);
	auto n2 = insert_vertex(13,g);
	auto n3 = insert_vertex(1337,g);
	auto n4 = insert_vertex(99,g);

	insert_edge(string("a"),n1,n2,g);
	insert_edge(string("b"),n2,n3,g);
	insert_edge(string("c"),n2,n1,g);
	insert_edge(string("d"),n1,n4,g);

	auto i = vertices(g);
	std::set<decltype(g)::vertex_descriptor> ns;
	std::for_each(i.first,i.second,[&](const decltype(g)::vertex_descriptor &n) { ASSERT_TRUE(ns.insert(n).second); });

	auto j = edges(g);
	std::set<decltype(g)::edge_descriptor> es;
	std::for_each(j.first,j.second,[&](const decltype(g)::edge_descriptor &n) { ASSERT_TRUE(es.insert(n).second); });

	ASSERT_EQ(ns.size(), 4);
	ASSERT_EQ(es.size(), 4);
}

TEST(digraph,error)
{
	po::digraph<int,std::string> g1,g2;

	auto n1 = insert_vertex(42,g1);
	auto n2 = insert_vertex(13,g1);
	insert_vertex(13,g1);

	insert_edge(string("a"),n1,n2,g1);
	insert_edge(string("b"),n1,n2,g1);

	ASSERT_EQ(num_edges(g1), 2);
	ASSERT_EQ(num_vertices(g1), 3);
}
