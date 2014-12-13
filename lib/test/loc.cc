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

#include <iostream>

#include <gtest/gtest.h>
#include <panopticon/loc.hh>
#include <panopticon/digraph.hh>

using namespace po;
using namespace std;
using namespace boost;
using namespace boost::uuids;

struct B
{
	B(int l) : length(l) {}
	int length;
};

namespace po
{
	template<>
	B* unmarshal(const uuid &u, const rdf::storage &store)
	{
		return new B(42);
	}

	template<>
	archive marshal(const B *b, const uuid &u)
	{
		rdf::statements ret;
		rdf::node root = rdf::iri(u);

		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("B"));
		ret.emplace_back(root,rdf::ns_po("length"),rdf::lit(b->length));

		return ret;
	}
}

struct A
{
	A(const string &s, vector<int> il) : name(s)
	{
		for(int i: il)
			bs.emplace_back(new B(i));
	}

	string name;
	list<loc<B>> bs;
};

ostream& operator<<(ostream &os, const A &a)
{
	os << a.name << " { ";
	for(const loc<B> &b: a.bs)
		os << b->length << " ";
	os << "}";

	return os;
}

namespace po
{
	template<>
	A* unmarshal(const uuid &u, const rdf::storage &store)
	{
		return new A("test",{});
	}

	template<>
	archive marshal(const A* a, const uuid &u)
	{
		rdf::statements ret;
		rdf::node root = rdf::iri(u);

		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("A"));
		ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(a->name));

		rdf::nodes tmp;
		for(const loc<B> &b: a->bs)
			tmp.emplace_back(rdf::iri(b.tag()));

		pair<rdf::node,rdf::statements> p = rdf::write_list(tmp.begin(),tmp.end(),to_string(u));
		ret.emplace_back(root,rdf::ns_po("bs"),p.first);
		std::move(p.second.begin(),p.second.end(),back_inserter(ret));

		return ret;
	}
}

TEST(loc,shared)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	loc<A> a(new A("Hello",{1,2,3}));

	save_point(*store);

	cerr << *a << endl;
	a.write().bs.front().write() = 66;

	save_point(*store);
}

TEST(loc,weak)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());

	loc<A> a(new A("Hello",{1,2,3}));
	wloc<A> aw(a);

	save_point(*store);

	cerr << *aw << endl;
	aw.write().bs.front().write() = 66;

	save_point(*store);

	a = loc<A>(new A("Hello",{1,2,3}));
	ASSERT_THROW(aw.write(), std::runtime_error);
}

TEST(loc,lock)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	loc<A> a(new A("Hello",{1,2,3}));
	wloc<A> aw(a);

	ASSERT_EQ(&(*a),&(*aw));

	a = loc<A>(new A("Hello",{1,2,3}));
	save_point(*store);
	ASSERT_THROW(*aw,runtime_error);
}

TEST(loc,weak_save_point)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());

	loc<A> a(new A("Hello",{1,2,3}));
	wloc<A> aw(a);

	save_point(*store);

	cerr << *aw << endl;
	aw.write().bs.front().write() = 66;

	a = loc<A>(new A("Hello",{1,2,3}));
	aw.write();

	save_point(*store);
	ASSERT_THROW(aw.write(), std::runtime_error);
}

TEST(loc,marshal_simple)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	auto gen = string_generator();

	loc<A> a(gen("{00000000-0000-0000-0000-000000000004}"),new A("Hello",{}));
	a.write().bs.push_back(loc<B>(gen("{00000000-0000-0000-0000-000000000001}"),new B(1)));
	a.write().bs.push_back(loc<B>(gen("{00000000-0000-0000-0000-000000000002}"),new B(2)));
	a.write().bs.push_back(loc<B>(gen("{00000000-0000-0000-0000-000000000003}"),new B(3)));

	save_point(*store);

	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_rdf("type"),rdf::ns_po("A")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_po("name"),rdf::lit("Hello")));

	rdf::nodes bs = rdf::read_list(store->first(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_po("bs")).object,*store);
	ASSERT_EQ(bs.size(),3u);
	ASSERT_EQ(*bs.begin(),rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")));
	ASSERT_EQ(*std::next(bs.begin()),rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")));
	ASSERT_EQ(*std::next(bs.begin(),2),rdf::iri(gen("{00000000-0000-0000-0000-000000000003}")));

	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")),rdf::ns_po("length"),rdf::lit(1)));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")),rdf::ns_po("length"),rdf::lit(2)));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000003}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000003}")),rdf::ns_po("length"),rdf::lit(3)));

	// A: 3 + 6
	// B: 3 * 2
	ASSERT_EQ(store->count(),3 + 6 + 3 * 2);
}

TEST(loc,marshal_twice)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	auto gen = string_generator();

	loc<A> a(gen("{00000000-0000-0000-0000-000000000004}"),new A("Hello",{}));
	loc<B> b1(gen("{00000000-0000-0000-0000-000000000001}"),new B(1));
	loc<B> b2(gen("{00000000-0000-0000-0000-000000000002}"),new B(2));
	loc<B> b3(gen("{00000000-0000-0000-0000-000000000003}"),new B(3));

	a.write().bs.push_back(b1);
	a.write().bs.push_back(b2);
	a.write().bs.push_back(b3);

	save_point(*store);

	b2.write().length = 99;
	a.write().name = "World";
	loc<B> b4(gen("{00000000-0000-0000-0000-000000000005}"),new B(4));

	a.write().bs.push_back(b4);

	save_point(*store);

	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_rdf("type"),rdf::ns_po("A")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_po("name"),rdf::lit("World")));

	rdf::nodes bs = rdf::read_list(store->first(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_po("bs")).object,*store);
	ASSERT_EQ(bs.size(),4u);
	ASSERT_EQ(*bs.begin(),rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")));
	ASSERT_EQ(*std::next(bs.begin()),rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")));
	ASSERT_EQ(*std::next(bs.begin(),2),rdf::iri(gen("{00000000-0000-0000-0000-000000000003}")));
	ASSERT_EQ(*std::next(bs.begin(),3),rdf::iri(gen("{00000000-0000-0000-0000-000000000005}")));

	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")),rdf::ns_po("length"),rdf::lit(1)));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")),rdf::ns_po("length"),rdf::lit(99)));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000003}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000003}")),rdf::ns_po("length"),rdf::lit(3)));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000005}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000005}")),rdf::ns_po("length"),rdf::lit(4)));

	// A: 3 + 8
	// B: 4 * 2
	ASSERT_EQ(store->count(),3 + 8 + 4 * 2);
}

TEST(loc,marshal_delete)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	auto gen = string_generator();

	loc<A> a(gen("{00000000-0000-0000-0000-000000000004}"),new A("Hello",{}));
	loc<B> b1(gen("{00000000-0000-0000-0000-000000000001}"),new B(1));
	loc<B> b2(gen("{00000000-0000-0000-0000-000000000002}"),new B(2));
	loc<B> b3(gen("{00000000-0000-0000-0000-000000000003}"),new B(3));

	a.write().bs.push_back(b1);
	a.write().bs.push_back(b2);
	a.write().bs.push_back(b3);

	save_point(*store);

	b3.write().length = 0;
	a.write().bs.pop_back();
	b3.remove();

	save_point(*store);

	ASSERT_THROW(*b3,std::runtime_error);
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_rdf("type"),rdf::ns_po("A")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_po("name"),rdf::lit("Hello")));

	rdf::nodes bs = rdf::read_list(store->first(rdf::iri(gen("{00000000-0000-0000-0000-000000000004}")),rdf::ns_po("bs")).object,*store);
	ASSERT_EQ(bs.size(),2u);
	ASSERT_EQ(*bs.begin(),rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")));
	ASSERT_EQ(*std::next(bs.begin()),rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")));

	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000001}")),rdf::ns_po("length"),rdf::lit(1)));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")),rdf::ns_rdf("type"),rdf::ns_po("B")));
	ASSERT_TRUE(store->has(rdf::iri(gen("{00000000-0000-0000-0000-000000000002}")),rdf::ns_po("length"),rdf::lit(2)));

	// A: 3 + 4
	// B: 2 * 2
	ASSERT_EQ(store->count(),3 + 4 + 2 * 2);
}

struct C
{
	digraph<int,std::string> graph;
};

namespace po
{
	template<>
	C* unmarshal(const uuid&, const rdf::storage&)
	{
		return new C();
	}

	template<>
	archive marshal(const C*, const uuid&)
	{
		return archive();
	}
}

TEST(loc,multiple)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	loc<C> c1(new C());
	loc<C> c2 = c1;

	insert_vertex(1,c1.write().graph);
	loc<C> c3(c1);
	ASSERT_EQ(num_vertices(c2->graph), 1u);
	insert_vertex(2,c2.write().graph);
	ASSERT_EQ(num_vertices(c3->graph), 2u);
}
