#include <gtest/gtest.h>

#include "dot/layout.hh"

TEST(dot,layout_empty_graph)
{
	po::digraph<std::string,int> g;
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),0);
}

TEST(dot,layout_graph_with_single_node)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),1);
	ASSERT_EQ(ret.count(a),1);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(std::get<2>(ret.at(a)),0);
}

TEST(dot,layout_graph_with_two_nodes)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string("World"),g);
	auto ab = insert_edge(0,a,b,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),2);
	ASSERT_EQ(ret.count(a),1);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(std::get<2>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1);
	ASSERT_EQ(std::get<0>(ret.at(b)),1);
	ASSERT_EQ(std::get<1>(ret.at(b)),1);
	ASSERT_EQ(std::get<2>(ret.at(b)),0);
}

TEST(dot,layout_circle_graph)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("\n"),g);
	auto ab = insert_edge(0,a,b,g);
	auto bc = insert_edge(1,b,c,g);
	auto cd = insert_edge(2,c,d,g);
	auto da = insert_edge(3,d,a,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),4);
	ASSERT_EQ(ret.count(a),1);
	ASSERT_EQ(ret.count(b),1);
	ASSERT_EQ(ret.count(c),1);
	ASSERT_EQ(ret.count(d),1);
}

TEST(dot,layout_graph_with_two_entries)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto ac = insert_edge(0,a,c,g);
	auto bc = insert_edge(1,b,c,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),3);
	ASSERT_EQ(ret.count(a),1);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1);
	ASSERT_EQ(std::get<0>(ret.at(b)),0);
	ASSERT_EQ(std::get<1>(ret.at(b)),0);
	ASSERT_NE(std::get<2>(ret.at(a)),std::get<2>(ret.at(b)));

	ASSERT_EQ(ret.count(c),1);
	ASSERT_EQ(std::get<0>(ret.at(c)),1);
	ASSERT_EQ(std::get<1>(ret.at(c)),1);
	ASSERT_EQ(std::get<2>(ret.at(c)),0);
}

TEST(dot,layout_graph_with_two_exits)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto ab = insert_edge(0,a,b,g);
	auto ac = insert_edge(1,a,c,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),3);
	ASSERT_EQ(ret.count(a),1);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(std::get<2>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1);
	ASSERT_EQ(std::get<0>(ret.at(b)),1);
	ASSERT_EQ(std::get<1>(ret.at(b)),1);
	ASSERT_EQ(ret.count(c),1);
	ASSERT_EQ(std::get<0>(ret.at(c)),1);
	ASSERT_EQ(std::get<1>(ret.at(c)),1);
	ASSERT_NE(std::get<2>(ret.at(b)),std::get<2>(ret.at(c)));
}

TEST(dot,layout_graph_with_cycle)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("\n"),g);
	auto ab = insert_edge(0,a,b,g);
	auto bc = insert_edge(1,b,c,g);
	auto cd = insert_edge(2,c,d,g);
	auto cb = insert_edge(3,c,b,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),4);
	ASSERT_EQ(ret.count(a),1);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(std::get<2>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1);
	ASSERT_EQ(ret.count(c),1);
	ASSERT_EQ(ret.count(d),1);
	ASSERT_EQ(std::get<0>(ret.at(d)),3);
	ASSERT_EQ(std::get<1>(ret.at(d)),3);
	ASSERT_EQ(std::get<2>(ret.at(d)),0);
	ASSERT_NE(std::get<2>(ret.at(b)),std::get<2>(ret.at(c)));
}

TEST(dot,layout_graph_with_self_loops)
{
	FAIL();
}

TEST(dot,layout_graph_with_two_exits_and_two_entries)
{
	FAIL();
}

TEST(dot,layout_graph_with_a_node_spanning_two_ranks)
{
	FAIL();
}
