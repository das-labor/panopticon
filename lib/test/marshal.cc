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

TEST(marshal,blob)
{
	boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
	boost::filesystem::path p2 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
	uuid u1;

	std::ofstream s1(p1.string()), s2(p2.string());
	std::vector<uint8_t> d = {1,2,3,4,5,6,7,8,9};

	ASSERT_TRUE(s1.is_open());
	ASSERT_TRUE(s2.is_open());

	s1 << "Hello, World" << std::flush;
	s2 << "Goodbye, World" << std::flush;

	s1.close();
	s2.close();

	blob mf1(p1,u1);
	blob mf4(d);

	{
		blob mf2(p2);

		ASSERT_NE(u1,mf2.tag());
		ASSERT_EQ(mf1.tag(),u1);

		ASSERT_NE(mf1.data(), nullptr);
		ASSERT_NE(mf2.data(), nullptr);
		ASSERT_NE(mf4.data(), nullptr);

		ASSERT_EQ(mf1.size(), 12);
		ASSERT_EQ(mf2.size(), 14);
		ASSERT_EQ(mf4.size(), 9);

		ASSERT_EQ(memcmp(mf1.data(),"Hello, World",mf1.size()), 0);
		ASSERT_EQ(memcmp(mf2.data(),"Goodbye, World",mf2.size()), 0);
		ASSERT_EQ(memcmp(mf4.data(),"\x01\x02\x03\x04\x05\x06\x07\x08\x09",mf4.size()), 0);

		blob mf3(mf1);

		ASSERT_EQ(mf1, mf3);
		ASSERT_EQ(mf1.data(), mf3.data());
		ASSERT_EQ(mf1.size(), mf3.size());
		ASSERT_EQ(mf1.tag(), mf3.tag());
	}

	boost::filesystem::remove(p1);
	boost::filesystem::remove(p2);
}
