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

#include <panopticon/value.hh>
#include <panopticon/loc.hh>

using namespace po;

TEST(value,lvalue)
{
	lvalue l1,l2;
	ASSERT_TRUE(l1 == l2);
	ASSERT_TRUE(!(l1 != l2));
	ASSERT_TRUE(!(l1 < l2));

	lvalue l3(l1);

	ASSERT_TRUE(l1 == l2);
	ASSERT_TRUE(l1 == l3);
	ASSERT_TRUE(l2 == l3);

	ASSERT_TRUE(!is_constant(l1));
	ASSERT_TRUE(is_undefined(l1));
	ASSERT_TRUE(!is_variable(l1));
	ASSERT_TRUE(!is_memory(l1));

	undefined u;
	ASSERT_TRUE(u == l3);
}

TEST(value,constant)
{
	constant c1(1),c2(1), c3(7), c4(7);

	ASSERT_TRUE(is_constant(c1));
	ASSERT_TRUE(is_constant(c2));
	ASSERT_TRUE(is_constant(c3));
	ASSERT_TRUE(is_constant(c4));

	ASSERT_TRUE(c1 == c2);
	ASSERT_TRUE(!(c1 != c2));

	ASSERT_TRUE(c3 == c4);
	ASSERT_TRUE(!(c3 != c4));

	ASSERT_TRUE(c1 != c4);
	ASSERT_TRUE(!(c1 == c4));

	ASSERT_TRUE(c1.content() == 1);

	ASSERT_TRUE(c3.content() == 7);

	ASSERT_TRUE(c1 < c3);
	ASSERT_TRUE(to_constant(rvalue(c1)) == c1);

	rvalue rv;
	rv = c3;
	ASSERT_TRUE(to_constant(rv) == c3);
}

TEST(value,variable)
{
	variable v1("a",2,0), v2("b",3), v3("a",1,88);

	ASSERT_TRUE(v1 != v2);
	ASSERT_TRUE(v1 != v3);
	ASSERT_TRUE(v3 != v2);

	ASSERT_THROW(variable v4("",0x100), value_exception);
	ASSERT_THROW(variable v4("a",0), value_exception);

	ASSERT_TRUE(v1.name() == std::string("a"));
	ASSERT_TRUE(v1.width() == 2);
	ASSERT_TRUE(v1.subscript() == 0);

	ASSERT_TRUE(v2.name() == std::string("b"));
	ASSERT_TRUE(v2.width() == 3);
	ASSERT_TRUE(v2.subscript() == -1);

	ASSERT_TRUE(v3.name() == std::string("a"));
	ASSERT_TRUE(v3.width() == 1);
	ASSERT_TRUE(v3.subscript() == 88);

	ASSERT_TRUE(v1 != variable("a",1,0));
	ASSERT_TRUE(v1 != variable("a",2,1));

	ASSERT_TRUE(to_variable(rvalue(v1)) == v1);
	ASSERT_TRUE(to_variable(lvalue(v1)) == v1);

	v1 = v2;
	ASSERT_TRUE(v1 == v2);
	ASSERT_TRUE(v1.name() == std::string("b"));
	ASSERT_TRUE(v1.width() == 3);
	ASSERT_TRUE(v1.subscript() == -1);

	rvalue rv;
	rv = v3;
	ASSERT_TRUE(to_variable(rv) == v3);

	lvalue lv;
	lv = v2;
	ASSERT_TRUE(to_variable(lv) == v2);
}

TEST(value,memory)
{
	memory m1(constant(1),2,BigEndian,"n"), m2(constant(2),1,BigEndian,"n"), m3(constant(3),2,LittleEndian,"n");

	ASSERT_TRUE(m1 != m2);
	ASSERT_TRUE(m1 != m3);
	ASSERT_TRUE(m3 != m2);

	ASSERT_THROW(memory m4(undefined(),32,BigEndian,""), value_exception);
	ASSERT_THROW(memory m4(undefined(),0,BigEndian,""), value_exception);

	ASSERT_TRUE(m1.offset() == constant(1));
	ASSERT_TRUE(m1.name() == "n");
	ASSERT_TRUE(m1.bytes() == 2);
	ASSERT_TRUE(m1.endianess() == BigEndian);

	m1 = m2;
	ASSERT_TRUE(m1 == m2);
	ASSERT_TRUE(m1.offset() == constant(2));
	ASSERT_TRUE(m1.name() == "n");
	ASSERT_TRUE(m1.bytes() == 1);
	ASSERT_TRUE(m1.endianess() == BigEndian);

	ASSERT_TRUE(to_memory(lvalue(m1)) == m1);

	rvalue rv;
	rv = m3;
	ASSERT_TRUE(to_memory(rv) == m3);

	lvalue lv;
	lv = m2;
	ASSERT_TRUE(to_memory(lv) == m2);
}

TEST(value,marshal)
{
	auto rand = boost::uuids::random_generator();

	rvalue a = undefined(),
			 b = constant(42),
			 c = variable("test",8),
			 d = memory(rvalue(constant(5)),2,LittleEndian,"bank1");

	uuid uua = rand(), uub = rand(), uuc = rand(), uud = rand();
	archive st1a = marshal(a,uua);
	archive st1b = marshal(b,uub);
	archive st1c = marshal(c,uuc);
	archive st1d = marshal(d,uud);

	ASSERT_GT(st1a.triples.size(),0u);
	ASSERT_GT(st1b.triples.size(),0u);
	ASSERT_GT(st1c.triples.size(),0u);
	ASSERT_GT(st1d.triples.size(),0u);
	ASSERT_EQ(st1a.blobs.size(),0u);
	ASSERT_EQ(st1b.blobs.size(),0u);
	ASSERT_EQ(st1c.blobs.size(),0u);
	ASSERT_EQ(st1d.blobs.size(),0u);

	archive st2a = marshal(a,uua);
	archive st2b = marshal(b,uub);
	archive st2c = marshal(c,uuc);
	archive st2d = marshal(d,uud);

	ASSERT_TRUE(st1a == st2a);
	ASSERT_TRUE(st1b == st2b);
	ASSERT_TRUE(st1c == st2c);
	ASSERT_TRUE(st1d == st2d);

	{
		rdf::storage store;

		for(auto s: st1a.triples)
		{
			std::cerr << s << std::endl;
			store.insert(s);
		}

		rvalue a2 = *std::unique_ptr<rvalue>(unmarshal<rvalue>(uua,store));
		ASSERT_TRUE(a2 == a);
	}

	{
		rdf::storage store;

		for(auto s: st1b.triples)
		{
			std::cerr << s << std::endl;
			store.insert(s);
		}

		rvalue b2 = *std::unique_ptr<rvalue>(unmarshal<rvalue>(uub,store));
		ASSERT_TRUE(b2 == b);
	}

	{
		rdf::storage store;

		for(auto s: st1c.triples)
		{
			std::cerr << s << std::endl;
			store.insert(s);
		}

		rvalue c2 = *std::unique_ptr<rvalue>(unmarshal<rvalue>(uuc,store));
		ASSERT_TRUE(c2 == c);
	}

	{
		rdf::storage store;

		for(auto s: st1d.triples)
		{
			std::cerr << s << std::endl;
			store.insert(s);
		}

		rvalue d2 = *std::unique_ptr<rvalue>(unmarshal<rvalue>(uud,store));
		ASSERT_TRUE(d2 == d);
	}
}
