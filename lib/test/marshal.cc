#include <gtest/gtest.h>

#include <panopticon/marshal.hh>

using namespace po;

TEST(marshal,load)
{
	rdf::storage st = rdf::storage("/home/seu/panopticon/lib/test/save.panop");

	ASSERT_TRUE(st.has(rdf::ns_local("A"),rdf::ns_po("name"),rdf::lit("Hello")));
	ASSERT_TRUE(st.has(rdf::ns_local("B"),rdf::ns_po("name"),rdf::lit("World")));
	ASSERT_TRUE(st.has(rdf::ns_local("A"),rdf::ns_local("a"),rdf::lit("B")));
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
