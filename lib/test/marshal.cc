#include <gtest/gtest.h>

#include <panopticon/marshal.hh>

using namespace po;

namespace po
{
	namespace rdf
	{
		static rdf::node ns_local(const std::string& s) { return rdf::iri("http://localhost/" + s); }
	}
}

TEST(marshal,load)
{
	rdf::storage st = rdf::storage("save.panop");

	ASSERT_TRUE(st.has(rdf::ns_local("A"),rdf::ns_po("name"),rdf::lit("Hello")));
	ASSERT_TRUE(st.has(rdf::ns_local("B"),rdf::ns_po("name"),rdf::lit("World")));
	ASSERT_TRUE(st.has(rdf::ns_local("A"),rdf::ns_local("a"),rdf::lit("B")));
}

TEST(marshal,snaphot)
{
	rdf::storage st;

	ASSERT_TRUE(st.insert(rdf::ns_local("A"),rdf::ns_po("name"),rdf::lit("Hello")));
	ASSERT_TRUE(st.insert(rdf::ns_local("B"),rdf::ns_po("name"),rdf::lit("World")));
	ASSERT_TRUE(st.insert(rdf::ns_local("A"),rdf::ns_local("a"),rdf::lit("B")));

	auto p = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%-%%%%.panop");
	st.snapshot(p);

	ASSERT_TRUE(boost::filesystem::exists(p));
	boost::filesystem::remove(p);
}

TEST(marshal,save_load)
{
	auto p = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%-%%%%.panop");

	{
		rdf::storage st;

		ASSERT_TRUE(st.insert(rdf::ns_local("A"),rdf::ns_po("name"),rdf::lit("Hello")));
		ASSERT_TRUE(st.insert(rdf::ns_local("B"),rdf::ns_po("name"),rdf::lit("World")));
		ASSERT_TRUE(st.insert(rdf::ns_local("A"),rdf::ns_local("a"),rdf::lit("B")));

		st.snapshot(p);
	}

	{
		rdf::storage st(p);

		ASSERT_TRUE(st.has(rdf::ns_local("A"),rdf::ns_po("name"),rdf::lit("Hello")));
		ASSERT_TRUE(st.has(rdf::ns_local("B"),rdf::ns_po("name"),rdf::lit("World")));
		ASSERT_TRUE(st.has(rdf::ns_local("A"),rdf::ns_local("a"),rdf::lit("B")));
	}

	boost::filesystem::remove(p);
}

TEST(marshal,empty)
{
	ASSERT_TRUE(boost::filesystem::exists("empty.panop"));
	ASSERT_THROW(rdf::storage("empty.panop"),marshal_exception);
}

TEST(marshal,missing_file)
{
	ASSERT_FALSE(boost::filesystem::exists("non-existend.panop"));
	ASSERT_THROW(rdf::storage("non-existend.panop"),marshal_exception);
}
