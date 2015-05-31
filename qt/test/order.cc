/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <gtest/gtest.h>

#include "dot/dot.hh"

TEST(order,empty_graph)
{
	po::digraph<std::string,int> g;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),0u);
}

TEST(order,graph_with_single_node)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);

	lambda.emplace(a,std::make_pair(0,0));
	widths.emplace(a,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),1u);
	ASSERT_EQ(ret.count(a),1u);
}

TEST(order,graph_with_two_nodes)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string("World"),g);
	/*auto ab = */insert_edge(0,a,b,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(1,1));
	widths.emplace(a,10);
	widths.emplace(b,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),2u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
}

TEST(order,circle_graph)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("\n"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto bc = */insert_edge(1,b,c,g);
	/*auto cd = */insert_edge(2,c,d,g);
	/*auto da = */insert_edge(3,d,a,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(0,0));
	lambda.emplace(c,std::make_pair(0,0));
	lambda.emplace(d,std::make_pair(0,0));
	widths.emplace(a,10);
	widths.emplace(b,10);
	widths.emplace(c,10);
	widths.emplace(d,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),4u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(ret.count(d),1u);
}

TEST(order,graph_with_two_entries)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	/*auto ac = */insert_edge(0,a,c,g);
	/*auto bc = */insert_edge(1,b,c,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(1,1));
	lambda.emplace(c,std::make_pair(2,2));
	widths.emplace(a,10);
	widths.emplace(b,10);
	widths.emplace(c,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),3u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
}

TEST(order,graph_with_two_exits)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto ac = */insert_edge(1,a,c,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(1,1));
	lambda.emplace(c,std::make_pair(2,2));
	widths.emplace(a,10);
	widths.emplace(b,10);
	widths.emplace(c,10);
	auto ret = dot::order(lambda,widths,100,g);


	ASSERT_EQ(ret.size(),3u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
}

TEST(order,graph_with_cycle)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("\n"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto bc = */insert_edge(1,b,c,g);
	/*auto cd = */insert_edge(2,c,d,g);
	/*auto cb = */insert_edge(3,c,b,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(1,1));
	lambda.emplace(c,std::make_pair(2,2));
	lambda.emplace(d,std::make_pair(3,3));
	widths.emplace(a,10);
	widths.emplace(b,10);
	widths.emplace(c,10);
	widths.emplace(d,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),4u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(ret.count(d),1u);
}

TEST(order,graph_with_self_loops)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto bc = */insert_edge(1,b,c,g);
	/*auto bb = */insert_edge(2,b,b,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(1,1));
	lambda.emplace(c,std::make_pair(2,2));
	widths.emplace(a,10);
	widths.emplace(b,10);
	widths.emplace(c,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),3u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
}

TEST(order,graph_with_two_exits_and_two_entries)
{
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,std::pair<int,int>> lambda;
	std::unordered_map<typename po::digraph<std::string,int>::vertex_descriptor,int> widths;
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("Goodbye"),g);
	auto e = insert_vertex(std::string(", World"),g);

	/*auto ab = */insert_edge(0,a,b,g);
	/*auto ac = */insert_edge(1,a,c,g);
	/*auto cd = */insert_edge(3,c,d,g);
	/*auto ce = */insert_edge(4,c,e,g);

	lambda.emplace(a,std::make_pair(0,0));
	lambda.emplace(b,std::make_pair(1,1));
	lambda.emplace(c,std::make_pair(2,2));
	lambda.emplace(d,std::make_pair(3,3));
	lambda.emplace(e,std::make_pair(3,3));
	widths.emplace(a,10);
	widths.emplace(b,10);
	widths.emplace(c,10);
	widths.emplace(d,10);
	widths.emplace(e,10);
	auto ret = dot::order(lambda,widths,100,g);

	ASSERT_EQ(ret.size(),5u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(ret.count(d),1u);
	ASSERT_EQ(ret.count(e),1u);
}

TEST(order,multiple_components)
{
	FAIL();
}
