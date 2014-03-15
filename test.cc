#include <gtest/gtest.h>
#include "rdf.hh"

using namespace std;

struct store : public ::testing::Test
{
	store(void) : root(node::blank()), a1(node::blank()), a2(node::blank()), b(node::blank()) {}

	void setUp(void)
	{
		empty_store.reset(new storage());
		full_store.reset(new storage());

		root = node::blank();
		full_store->insert(a1,"type"_rdf,"A"_po);
		full_store->insert(a2,"type"_rdf,"A"_po);
		full_store->insert(b,"type"_rdf,"B"_po);
		full_store->insert(a1,"name"_po,"Mr. A"_lit);
		full_store->insert(a1,"bs"_po,b);
		full_store->insert(b,"count"_po,42_lit);
		full_store->insert(root,"child"_po,a1);
		full_store->insert(root,"child"_po,a2);
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
	ASSERT_TRUE(empty_store->insert(node::blank(),"test"_po,node::blank()));
	ASSERT_EQ(empty_store->count(),1);
}

TEST_F(store,add_multiple)
{
	ASSERT_TRUE(empty_store->insert(node::blank(),"test"_po,node::blank()));
	ASSERT_TRUE(empty_store->insert(node::blank(),"test2"_po,node::blank()));
	ASSERT_TRUE(empty_store->insert(node::blank(),"test3"_po,node::blank()));
	ASSERT_EQ(empty_store->count(),3);
}

TEST_F(store,add_twice)
{
	ASSERT_TRUE(empty_store->insert("La"_po,"test"_po,"Lo"_po));
	ASSERT_FALSE(empty_store->insert("La"_po,"test"_po,"Lo"_po));
	ASSERT_EQ(empty_store->count(),1);
}

TEST_F(store,find_single)
{
	ASSERT_TRUE(full_store->has(a1,"type"_rdf,"A"_po));
}

TEST_F(store,find_multiple)
{
	auto res = full_store->find(root,"child"_po);
	list<statement> exp({
		statement(root,"child"_po,a1),
		statement(root,"child"_po,a2)
	});

	res.sort();
	exp.sort();

	ASSERT_EQ(res,exp);
}

TEST_F(store,find_none)
{
	ASSERT_FALSE(full_store->has(root,"child"_po,"NOPE"_po));
}

TEST_F(store,remove)
{
	ASSERT_TRUE(full_store->remove(a1,"type"_rdf,"A"_po));
	ASSERT_EQ(full_store->count(),7);
	ASSERT_FALSE(full_store->has(a1,"type"_rdf,"A"_po));
}

TEST_F(store,find_subject)
{
	auto res = full_store->find(a1);
	list<statement> exp({
		statement(a1,"type"_rdf,"A"_po),
		statement(a1,"name"_po,"Mr. A"_lit),
		statement(a1,"bs"_po,b)
	});

	res.sort();
	exp.sort();

	ASSERT_EQ(res,exp);
}

TEST(store,save_restore)
{
	node x = node::blank(), y = node::blank();
	{
		storage st("test");

		ASSERT_TRUE(st.insert(x,"test"_po,y));
		ASSERT_TRUE(st.insert(x,"name"_po,"Hello, World"_lit));
		ASSERT_TRUE(st.insert(y,"age"_po,23_lit));
	}

	{
		storage st("test");

		ASSERT_EQ(st.count(),3);
		ASSERT_TRUE(st.has(x,"test"_po,y));
		ASSERT_TRUE(st.has(x,"name"_po,"Hello, World"_lit));
		ASSERT_TRUE(st.has(y,"age"_po,23_lit));
	}
}

TEST(store,node_value_semantics)
{}
