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
		cerr << "save B w/ length " << b->length << endl;
		cerr << "po:" << u << " rdf:type " << "po:B" << endl;
		cerr << "po:" << u << " po:length " << "\"" << b->length << "\"^^xsd:integer" << endl;

		return rdf::statements();
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
		cerr << "save \"" << a->name << "\" w/ " << a->bs.size() << " B's" << endl;
		cerr << ":" << u << " rdf:type " << "po:A" << endl;
		cerr << ":" << u << " po:name " << "\"" << a->name << "\"^^xsd:string" << endl;
		for(const loc<B> &b: a->bs)
			cerr << ":" << u << " po:b " << ":" << b.tag() << endl;
		return rdf::statements();
	}
}

TEST(loc,shared)
{
	rdf::storage store;
	auto rand = random_generator();
	loc<A> a(rand(),new A("Hello",{1,2,3}));

	save_point(store);

	cerr << *a << endl;
	a.write().bs.front().write() = 66;

	save_point(store);
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

TEST(loc,marshal_simple){}
TEST(loc,marshal_twice){}
TEST(loc,marshal_delete){}
