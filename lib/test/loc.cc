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
	B* unmarshal(const uuid &u)
	{
		return new B(42);
	}

	template<>
	void marshal(const B *b, const uuid &u)
	{
		cerr << "save B w/ length " << b->length << endl;
		cerr << "po:" << u << " rdf:type " << "po:B" << endl;
		cerr << "po:" << u << " po:length " << "\"" << b->length << "\"^^xsd:integer" << endl;
	}
}

struct A
{
	A(const string &s, vector<int> il) : name(s), bs()
	{
		auto rand = random_generator();
		for(int i: il)
			bs.emplace_back(move(loc<B>(rand(),new B(i))));
	}

	string name;
	vector<loc<B>> bs;
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
	A* unmarshal(const uuid &u)
	{
		return new A("test",{});
	}

	template<>
	void marshal(const A* a, const uuid &u)
	{
		cerr << "save \"" << a->name << "\" w/ " << a->bs.size() << " B's" << endl;
		cerr << ":" << u << " rdf:type " << "po:A" << endl;
		cerr << ":" << u << " po:name " << "\"" << a->name << "\"^^xsd:string" << endl;
		for(const loc<B> &b: a->bs)
			cerr << ":" << u << " po:b " << ":" << b.tag() << endl;
	}
}

TEST(loc,shared)
{
	auto rand = random_generator();
	loc<A> a(rand(),new A("Hello",{1,2,3}));

	save_point();

	cerr << *a << endl;
	a.write().bs[0].write() = 66;

	save_point();
}

TEST(loc,weak)
{
	auto rand = random_generator();

	loc<A> a(rand(),new A("Hello",{1,2,3}));
	wloc<A> aw(a);

	save_point();

	cerr << *aw << endl;
	aw.write().bs[0].write() = 66;

	save_point();

	a = loc<A>(rand());
	ASSERT_THROW(aw.write(), std::runtime_error);
}
