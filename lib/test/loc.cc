#include <iostream>

#include <gtest/gtest.h>
#include <panopticon/loc.hh>

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
	rdf::statements marshal(const B *b, const uuid &u)
	{
		rdf::statements ret;
		rdf::node root = rdf::ns_local(to_string(u));

		ret.emplace_back(root,"type"_rdf,"B"_po);
		ret.emplace_back(root,"length"_po,rdf::lit(b->length));

		return ret;
	}
}

struct A
{
	A(const string &s, vector<int> il) : name(s)
	{
		auto rand = random_generator();
		for(int i: il)
			bs.emplace_back(rand(),new B(i));
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
	rdf::statements marshal(const A* a, const uuid &u)
	{
		rdf::statements ret;
		rdf::node root = rdf::ns_local(to_string(u));

		ret.emplace_back(root,"type"_rdf,"A"_po);
		ret.emplace_back(root,"name"_po,rdf::lit(a->name));

		rdf::nodes tmp;
		for(const loc<B> &b: a->bs)
			tmp.emplace_back(rdf::ns_local(to_string(b.tag())));

		pair<rdf::node,rdf::statements> p = rdf::write_list(tmp.begin(),tmp.end(),to_string(u));
		ret.emplace_back(root,"bs"_po,p.first);
		move(p.second.begin(),p.second.end(),back_inserter(ret));

		return ret;
	}
}

TEST(loc,shared)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	auto rand = random_generator();
	loc<A> a(rand(),new A("Hello",{1,2,3}));

	save_point(*store);

	cerr << *a << endl;
	a.write().bs.front().write() = 66;

	save_point(*store);
}

TEST(loc,weak)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	auto rand = random_generator();

	loc<A> a(rand(),new A("Hello",{1,2,3}));
	wloc<A> aw(a);

	save_point(*store);

	cerr << *aw << endl;
	aw.write().bs.front().write() = 66;

	save_point(*store);

	a = loc<A>(rand(),*store);
	ASSERT_THROW(aw.write(), std::runtime_error);
}

TEST(loc,lock)
{
	ASSERT_EQ(true,false);
}

TEST(loc,weak_save_point)
{
	std::shared_ptr<rdf::storage> store(new rdf::storage());
	auto rand = random_generator();

	loc<A> a(rand(),new A("Hello",{1,2,3}));
	wloc<A> aw(a);

	save_point(*store);

	cerr << *aw << endl;
	aw.write().bs.front().write() = 66;

	a = loc<A>(rand(),*store);
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

	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"type"_rdf,"A"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"name"_po,"Hello"_lit));

	rdf::nodes bs = rdf::read_list(store->first(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"bs"_po,none).object(),*store);
	ASSERT_EQ(bs.size(),3);
	ASSERT_EQ(*bs.begin(),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))));
	ASSERT_EQ(*std::next(bs.begin()),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))));
	ASSERT_EQ(*std::next(bs.begin(),2),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000003}"))));

	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))),"length"_po,1_lit));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))),"length"_po,2_lit));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000003}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000003}"))),"length"_po,3_lit));

	// A: 3 + 6
	// B: 3 * 2
	int i = 0;
	rdf::stream all = store->select(none,none,none);
	while(!all.eof())
	{
		rdf::statement s;
		all >> s;
		++i;
	}

	ASSERT_EQ(i,3 + 6 + 3 * 2);
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

	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"type"_rdf,"A"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"name"_po,"World"_lit));

	rdf::nodes bs = rdf::read_list(store->first(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"bs"_po,none).object(),*store);
	ASSERT_EQ(bs.size(),4);
	ASSERT_EQ(*bs.begin(),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))));
	ASSERT_EQ(*std::next(bs.begin()),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))));
	ASSERT_EQ(*std::next(bs.begin(),2),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000003}"))));
	ASSERT_EQ(*std::next(bs.begin(),3),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000005}"))));

	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))),"length"_po,1_lit));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))),"length"_po,99_lit));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000003}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000003}"))),"length"_po,3_lit));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000005}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000005}"))),"length"_po,4_lit));

	// A: 3 + 8
	// B: 4 * 2
	int i = 0;
	rdf::stream all = store->select(none,none,none);
	while(!all.eof())
	{
		rdf::statement s;
		all >> s;
		++i;
	}

	ASSERT_EQ(i,3 + 8 + 4 * 2);
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
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"type"_rdf,"A"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"name"_po,"Hello"_lit));

	rdf::nodes bs = rdf::read_list(store->first(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000004}"))),"bs"_po,none).object(),*store);
	ASSERT_EQ(bs.size(),2);
	ASSERT_EQ(*bs.begin(),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))));
	ASSERT_EQ(*std::next(bs.begin()),rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))));

	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000001}"))),"length"_po,1_lit));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))),"type"_rdf,"B"_po));
	ASSERT_TRUE(store->has(rdf::ns_local(to_string(gen("{00000000-0000-0000-0000-000000000002}"))),"length"_po,2_lit));

	// A: 3 + 4
	// B: 2 * 2
	int i = 0;
	rdf::stream all = store->select(none,none,none);
	while(!all.eof())
	{
		rdf::statement s;
		all >> s;
		++i;
	}

	ASSERT_EQ(i,3 + 4 + 2 * 2);
}
