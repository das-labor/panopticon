#include <iostream>

#include <gtest/gtest.h>
#include <panopticon/region.hh>

using namespace std;

struct region : public ::testing::Test
{
	region(void) : regs(), r1(po::region::undefined("base",128)), r2(po::region::undefined("zlib",64)), r3(po::region::undefined("aes",48)) {}

	void SetUp(void)
	{
		auto vx1 = insert_vertex(r1,regs);
		auto vx2 = insert_vertex(r2,regs);
		auto vx3 = insert_vertex(r3,regs);

		insert_edge(po::bound(32,96),vx1,vx2,regs);
		insert_edge(po::bound(16,32),vx1,vx3,regs);
		insert_edge(po::bound(0,32),vx2,vx3,regs);
	}

	po::regions regs;

	po::region_loc r1;
	po::region_loc r2;
	po::region_loc r3;

	using vx = boost::graph_traits<po::regions>::vertex_descriptor;

	vx vx1;
	vx vx2;
	vx vx3;
};

TEST_F(region,tree)
{
	auto t = po::spanning_tree(regs);
	decltype(t) expect({
		make_pair(po::region_wloc(r2),po::region_wloc(r1)),
		make_pair(po::region_wloc(r3),po::region_wloc(r1))
	});

	for(auto i: t)
		std::cout << i.first->name() << " -> " << i.second->name() << std::endl;

	ASSERT_TRUE(t == expect);
}

TEST_F(region,proj)
{
	auto proj = po::projection(regs);
	decltype(proj) expect({
		make_pair(po::bound(0,16),po::region_wloc(r1)),
		make_pair(po::bound(0,48),po::region_wloc(r3)),
		make_pair(po::bound(32,64),po::region_wloc(r2)),
		make_pair(po::bound(96,128),po::region_wloc(r1))
	});

	for(auto i: proj)
	{
		std::cout << i.first << ": " << i.second->name() << std::endl;
	}

	ASSERT_TRUE(proj == expect);
}

TEST_F(region,read_undef)
{
	po::region_loc r1 = po::region::undefined("test",128);
	po::slab s = r1->read();

	ASSERT_EQ(s.size(),128u);

	for(auto i: s)
		ASSERT_EQ(i,boost::none);
}

TEST_F(region,read_one_layer)
{
	boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
	po::region_loc r1 = po::region::undefined("test",128);
	std::ofstream s1(p1.string());

	ASSERT_TRUE(s1.is_open());
	s1 << "Hello, World" << std::flush;
	s1.close();

	r1.write().add(po::bound(1,8),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,7})));
	r1.write().add(po::bound(50,62),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,6,5,4,3,2,1})));
	r1.write().add(po::bound(62,63),po::layer_loc(new po::layer("anon 2",{po::byte(1)})));
	r1.write().add(po::bound(70,82),po::layer_loc(new po::layer("anon 2",po::blob(p1))));

	po::slab s = r1->read();
	ASSERT_EQ(s.size(),128u);
	size_t idx = 0;

	for(auto i: s)
	{
		cout << idx << ": " << (i ? to_string((unsigned int)(*i)) : "none") << endl;
		if(idx >= 1 && idx < 8)
			ASSERT_TRUE(i && *i == idx);
		else if(idx >= 50 && idx < 56)
			ASSERT_TRUE(i && *i == idx - 49);
		else if(idx >= 56 && idx < 62)
			ASSERT_TRUE(i && *i == 6 - (idx - 56));
		else if(idx >= 70 && idx < 82)
			EXPECT_TRUE(i && *i == std::string("Hello, World").substr(idx - 70,1)[0]);
		else if(idx == 62)
			ASSERT_TRUE(i && *i == 1);
		else
			ASSERT_TRUE(i == boost::none);
		++idx;
	}
}

TEST_F(region,marshal)
{
	boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");

	{
		po::region_loc r1 = po::region::undefined("test",128);
		std::ofstream s1(p1.string());

		ASSERT_TRUE(s1.is_open());
		s1 << "Hello, World" << std::flush;
		s1.close();

		r1.write().add(po::bound(1,8),po::layer_loc(new po::layer("anon 1",5)));
		r1.write().add(po::bound(1,8),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,7})));
		r1.write().add(po::bound(50,62),po::layer_loc(new po::layer("anon 3",{
			make_pair(1,1),
			make_pair(0,boost::none),
			make_pair(3,0xff),
			make_pair(4,boost::none),
			make_pair(2,2)
		})));
		r1.write().add(po::bound(70,82),po::layer_loc(new po::layer("anon 2",po::blob(p1))));

		po::rdf::storage st;
		po::save_point(st);

		std::unique_ptr<po::region> r1b(po::unmarshal<po::region>(r1.tag(),st));

		ASSERT_TRUE(*r1b == *r1);
		po::discard_changes();
	}

	boost::filesystem::remove(p1);
}
