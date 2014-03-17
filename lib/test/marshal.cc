#include <gtest/gtest.h>

#include <panopticon/marshal.hh>

using namespace po;

TEST(marshal,load)
{
	rdf::storage st = rdf::storage("/home/seu/panopticon/lib/test/save.panop");

	ASSERT_TRUE(st.has("A"_local,"name"_po,"Hello"_lit));
	ASSERT_TRUE(st.has("B"_local,"name"_po,"World"_lit));
	ASSERT_TRUE(st.has("A"_local,"a"_local,"B"_lit));
}

TEST(marshal,snaphot)
{
	ASSERT_TRUE(false);
}

TEST(marshal,save_load)
{
	ASSERT_TRUE(false);
}

TEST(marshal,empty)
{
	ASSERT_THROW(rdf::storage("empty.db"),marshal_exception);
}

TEST(marshal,missing_file)
{
	ASSERT_THROW(rdf::storage("non-existend.db"),marshal_exception);
}

TEST(marshal,missing_db)
{
	ASSERT_TRUE(false);
}
