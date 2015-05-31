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

TEST(rank,layout_empty_graph)
{
	po::digraph<std::string,int> g;
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),0u);
}

TEST(rank,layout_graph_with_single_node)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),1u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
}

TEST(rank,layout_graph_with_two_nodes)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string("World"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),2u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(std::get<0>(ret.at(b)),1);
	ASSERT_EQ(std::get<1>(ret.at(b)),1);
}

TEST(rank,layout_circle_graph)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("\n"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto bc = */insert_edge(1,b,c,g);
	/*auto cd = */insert_edge(2,c,d,g);
	/*auto da = */insert_edge(3,d,a,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),4u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(ret.count(d),1u);
}

TEST(rank,layout_graph_with_two_entries)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	/*auto ac = */insert_edge(0,a,c,g);
	/*auto bc = */insert_edge(1,b,c,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),3u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(std::get<0>(ret.at(b)),0);
	ASSERT_EQ(std::get<1>(ret.at(b)),0);

	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(std::get<0>(ret.at(c)),1);
	ASSERT_EQ(std::get<1>(ret.at(c)),1);
}

TEST(rank,layout_graph_with_two_exits)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto ac = */insert_edge(1,a,c,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),3u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(std::get<0>(ret.at(b)),1);
	ASSERT_EQ(std::get<1>(ret.at(b)),1);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(std::get<0>(ret.at(c)),1);
	ASSERT_EQ(std::get<1>(ret.at(c)),1);
}

TEST(rank,layout_graph_with_cycle)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	auto d = insert_vertex(std::string("\n"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto bc = */insert_edge(1,b,c,g);
	/*auto cd = */insert_edge(2,c,d,g);
	/*auto cb = */insert_edge(3,c,b,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),4u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(ret.count(d),1u);
	ASSERT_EQ(std::get<0>(ret.at(d)),3);
	ASSERT_EQ(std::get<1>(ret.at(d)),3);
}

TEST(rank,layout_graph_with_self_loops)
{
	po::digraph<std::string,int> g;
	auto a = insert_vertex(std::string("Hello"),g);
	auto b = insert_vertex(std::string(","),g);
	auto c = insert_vertex(std::string("World"),g);
	/*auto ab = */insert_edge(0,a,b,g);
	/*auto bc = */insert_edge(1,b,c,g);
	/*auto bb = */insert_edge(2,b,b,g);
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),3u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(std::get<0>(ret.at(b)),1);
	ASSERT_EQ(std::get<1>(ret.at(b)),1);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(std::get<0>(ret.at(c)),2);
	ASSERT_EQ(std::get<1>(ret.at(c)),2);
}

TEST(rank,layout_graph_with_two_exits_and_two_entries)
{
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
	auto ret = dot::layout(g);

	ASSERT_EQ(ret.size(),5u);
	ASSERT_EQ(ret.count(a),1u);
	ASSERT_EQ(std::get<0>(ret.at(a)),0);
	ASSERT_EQ(std::get<1>(ret.at(a)),0);
	ASSERT_EQ(ret.count(b),1u);
	ASSERT_EQ(std::get<0>(ret.at(b)),1);
	ASSERT_EQ(std::get<1>(ret.at(b)),1);
	ASSERT_EQ(ret.count(c),1u);
	ASSERT_EQ(std::get<0>(ret.at(c)),1);
	ASSERT_EQ(std::get<1>(ret.at(c)),1);
	ASSERT_EQ(ret.count(d),1u);
	ASSERT_EQ(std::get<0>(ret.at(d)),2);
	ASSERT_EQ(std::get<1>(ret.at(d)),2);
	ASSERT_EQ(ret.count(e),1u);
	ASSERT_EQ(std::get<0>(ret.at(e)),2);
	ASSERT_EQ(std::get<1>(ret.at(e)),2);
}

TEST(rank,layout_graph_with_a_node_spanning_two_ranks)
{
	po::digraph<std::string,int> graph;
	auto a = insert_vertex(std::string("A"),graph);
	auto b = insert_vertex(std::string("B"),graph);
	auto c = insert_vertex(std::string("C"),graph);
	auto d = insert_vertex(std::string("D"),graph);
	auto e = insert_vertex(std::string("E"),graph);
	auto f = insert_vertex(std::string("F"),graph);
	auto g = insert_vertex(std::string("G"),graph);
	/*auto ab = */insert_edge(0,a,b,graph);
	/*auto bc = */insert_edge(0,b,c,graph);
	/*auto cd = */insert_edge(0,c,d,graph);
	/*auto ae = */insert_edge(0,a,e,graph);
	/*auto ed = */insert_edge(0,e,d,graph);
	/*auto fe = */insert_edge(0,f,e,graph);
	/*auto eg = */insert_edge(0,e,g,graph);
	/*auto ret = */dot::layout(graph);
}

TEST(rank,layout_real_cfg)
{
	po::digraph<std::string,int> graph;
	auto a = insert_vertex(std::string("A"),graph); // jmp 84
	auto b = insert_vertex(std::string("B"),graph); // clr r1
	auto c = insert_vertex(std::string("C"),graph); // cpi r16, 98
	auto d = insert_vertex(std::string("D"),graph); // andi r19, 0
	auto e = insert_vertex(std::string("E"),graph); // ldi r17, 0
	auto f = insert_vertex(std::string("F"),graph); // cpi r26, 134
	auto g = insert_vertex(std::string("G"),graph); // call 7164
	auto h = insert_vertex(std::string("H"),graph); // cli
	auto i = insert_vertex(std::string("I"),graph); // rjmp 7718
	auto j = insert_vertex(std::string("J"),graph); // pop r28
	auto k = insert_vertex(std::string("K"),graph); // rjmp 260
	auto l = insert_vertex(std::string("L"),graph); // cpi r25, 3
	auto m = insert_vertex(std::string("M"),graph); // ldi r24, 0
	/*auto ab = */insert_edge(0,a,b,graph);
	/*auto bc = */insert_edge(1,b,c,graph);
	/*auto cd = */insert_edge(2,c,d,graph);
	/*auto ce = */insert_edge(3,c,e,graph);
	/*auto ef = */insert_edge(3,e,f,graph);
	/*auto fg = */insert_edge(3,f,g,graph);
	/*auto gh = */insert_edge(3,g,h,graph);
	/*auto hi = */insert_edge(3,h,i,graph);
	/*auto dj = */insert_edge(3,d,j,graph);
	/*auto dk = */insert_edge(3,d,k,graph);
	/*auto lm = */insert_edge(3,l,m,graph);
	/*auto mj = */insert_edge(3,m,j,graph);
	/*auto fl = */insert_edge(3,f,l,graph);
	/*auto lj = */insert_edge(3,l,j,graph);
	/*auto ret = */dot::layout(graph);
}
