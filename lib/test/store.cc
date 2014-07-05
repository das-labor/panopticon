#include <gtest/gtest.h>
#include <panopticon/marshal.hh>

using namespace std;
using namespace po;
using namespace rdf;

struct store : public ::testing::Test
{
	store(void) : root(node::blank()), a1(node::blank()), a2(node::blank()), b(node::blank()), empty_store(), full_store() {}

	virtual void SetUp(void)
	{
		empty_store.reset(new storage());
		full_store.reset(new storage());

		root = node::blank();
		full_store->insert(a1, rdf::ns_rdf("type"), rdf::ns_po("A"));
		full_store->insert(a2, rdf::ns_rdf("type"), rdf::ns_po("A"));
		full_store->insert(b, rdf::ns_rdf("type"), rdf::ns_po("B"));
		full_store->insert(a1, rdf::ns_po("name"), rdf::lit("Mr. A"));
		full_store->insert(a1, rdf::ns_po("bs"), b);
		full_store->insert(b, rdf::ns_po("count"), rdf::lit(42));
		full_store->insert(root, rdf::ns_po("child"), a1);
		full_store->insert(root, rdf::ns_po("child"), a2);
	}

	node root;
	node a1, a2;
	node b;
	std::unique_ptr<storage> empty_store;
	std::unique_ptr<storage> full_store;
};

TEST_F(store,construct)
{
	ASSERT_TRUE(!!empty_store);
	ASSERT_EQ(empty_store->count(),0);
}

TEST_F(store,add_single)
{
	ASSERT_TRUE(empty_store->insert(node::blank(),rdf::ns_po("test"),node::blank()));
	ASSERT_EQ(empty_store->count(),1);
}

TEST_F(store,add_multiple)
{
	ASSERT_TRUE(empty_store->insert(node::blank(), rdf::ns_po("test"), node::blank()));
	ASSERT_TRUE(empty_store->insert(node::blank(), rdf::ns_po("test2"), node::blank()));
	ASSERT_TRUE(empty_store->insert(node::blank(), rdf::ns_po("test3"), node::blank()));
	ASSERT_EQ(empty_store->count(),3);
}

TEST_F(store,add_twice)
{
	ASSERT_TRUE(empty_store->insert(rdf::ns_po("La"), rdf::ns_po("test"), rdf::ns_po("Lo")));
	ASSERT_FALSE(empty_store->insert(rdf::ns_po("La"), rdf::ns_po("test"), rdf::ns_po("Lo")));
	ASSERT_EQ(empty_store->count(),1);
}

TEST_F(store,find_single)
{
	ASSERT_TRUE(full_store->has(a1, rdf::ns_rdf("type"), rdf::ns_po("A")));
}

TEST_F(store,find_multiple)
{
	auto res = full_store->find(root, rdf::ns_po("child"));
	list<statement> exp({
		statement(root, rdf::ns_po("child"), a1),
		statement(root, rdf::ns_po("child"), a2)
	});

	res.sort();
	exp.sort();

	ASSERT_EQ(res,exp);
}

TEST_F(store,find_none)
{
	ASSERT_FALSE(full_store->has(root, rdf::ns_po("child"), rdf::ns_po("NOPE")));
}

TEST_F(store,remove)
{
	ASSERT_TRUE(full_store->remove(a1, rdf::ns_rdf("type"), rdf::ns_po("A")));
	ASSERT_EQ(full_store->count(),7);
	ASSERT_FALSE(full_store->has(a1, rdf::ns_rdf("type"), rdf::ns_po("A")));
}

TEST_F(store,find_subject)
{
	auto res = full_store->find(a1);
	list<statement> exp({
		statement(a1, rdf::ns_rdf("type"), rdf::ns_po("A")),
		statement(a1, rdf::ns_po("name"), rdf::lit("Mr. A")),
		statement(a1, rdf::ns_po("bs"), b)
	});

	res.sort();
	exp.sort();

	ASSERT_EQ(res,exp);
}

TEST_F(store,node_value_semantics)
{
	{
		node a = node::blank();
		node b = a;

		ASSERT_EQ(a,b);
	}

	{
		node a = node::blank();
		node b = node::blank();

		b = a;

		ASSERT_EQ(a,b);
	}

	{
		node c = node::blank();
		node d = node::blank();
		node a = c;
		node b = c;
		a = d;
		ASSERT_EQ(b,c);
	}

	{
		node c = node::blank();
		node b = node::blank();

		{
			node a = c;
			b = c;
		}

		assert(b == c);
	}
}

TEST_F(store,varint)
{
	string a;

	a = storage::encode_varint(1);
	ASSERT_EQ("\x01",a);
	ASSERT_EQ(1,storage::decode_varint(a.begin(),a.end()).first);

	a = storage::encode_varint(0x7f);
	ASSERT_EQ("\x7f",a);
	ASSERT_EQ(0x7f,storage::decode_varint(a.begin(),a.end()).first);

	a = storage::encode_varint(0x80);
	ASSERT_EQ(string("\x81\x00",2),a);
	ASSERT_EQ(0x80,storage::decode_varint(a.begin(),a.end()).first);

	a = storage::encode_varint(0x81);
	ASSERT_EQ(a.size(),2);
	ASSERT_EQ(storage::decode_varint(a.begin(),a.end()).first,0x81);

	a = storage::encode_varint(0x3fff);
	ASSERT_EQ(a.size(),2);
	ASSERT_EQ(storage::decode_varint(a.begin(),a.end()).first,0x3fff);

	a = storage::encode_varint(0x4000);
	ASSERT_EQ(a.size(),3);
	ASSERT_EQ(storage::decode_varint(a.begin(),a.end()).first,0x4000);

	a = storage::encode_varint(0x4001);
	ASSERT_EQ(a.size(),3);
	ASSERT_EQ(storage::decode_varint(a.begin(),a.end()).first,0x4001);

	a = storage::encode_varint(0);
	ASSERT_EQ(a.size(),1);
	ASSERT_EQ(storage::decode_varint(a.begin(),a.end()).first,0);
}

TEST_F(store,node)
{
	node a = node::blank(), b = rdf::ns_po("node"), c = rdf::lit(1), d = rdf::lit("Hello"), e = rdf::lit("");

	string aa = storage::encode_node(a);
	string bb = storage::encode_node(b);
	string cc = storage::encode_node(c);
	string dd = storage::encode_node(d);
	string ee = storage::encode_node(e);

	node a2 = storage::decode_node(aa.begin(),aa.end()).first;
	node b2 = storage::decode_node(bb.begin(),bb.end()).first;
	node c2 = storage::decode_node(cc.begin(),cc.end()).first;
	node d2 = storage::decode_node(dd.begin(),dd.end()).first;
	node e2 = storage::decode_node(ee.begin(),ee.end()).first;

	ASSERT_EQ(a,a2);
	ASSERT_EQ(b,b2);
	ASSERT_EQ(c,c2);
	ASSERT_EQ(d,d2);
	ASSERT_EQ(e,e2);
}

TEST_F(store,blob)
{
	boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
	boost::filesystem::path p2 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
	std::ofstream s1(p1.string());

	ASSERT_TRUE(s1.is_open());

	s1 << "Hello, World" << std::flush;
	s1.close();

	uuid u1;
	mapped_file mf1(p1,u1);
	rdf::storage store1;

	ASSERT_TRUE(store1.register_blob(mf1));
	ASSERT_FALSE(store1.register_blob(mf1));

	mapped_file mf2 = store1.fetch_blob(u1);
	ASSERT_EQ(mf1, mf2);

	store1.snapshot(p2);

	rdf::storage store2(p2);

	ASSERT_FALSE(store2.register_blob(mf1));
	mapped_file mf3 = store2.fetch_blob(u1);

	ASSERT_EQ(mf1.size(), mf3.size());

	size_t i = 0;
	while(i < mf3.size())
	{
		ASSERT_EQ(mf1.data()[i], mf3.data()[i]);
		++i;
	}
}
