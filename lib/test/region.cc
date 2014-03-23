#include <iostream>

#include <gtest/gtest.h>
#include <panopticon/region.hh>

using namespace std;

struct region : public ::testing::Test
{
	region(void) : regs(), r1(po::region::undefined("base",128)), r2(po::region::undefined("zlib",64)), r3(po::region::undefined("aes",48)) {}

	void SetUp(void)
	{
		auto vx1 = insert_node(r1,regs);
		auto vx2 = insert_node(r2,regs);
		auto vx3 = insert_node(r3,regs);

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
	{
		std::cout << i.first->name() << " -> " << i.second->name() << std::endl;
	}

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

	ASSERT_EQ(boost::size(s),128);

	for(auto i: s)
		ASSERT_EQ(i,boost::none);
}

TEST_F(region,read_one_layer)
{
	po::region_loc r1 = po::region::undefined("test",128);

	r1.write().add(po::bound(1,8),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,7})));
	r1.write().add(po::bound(50,62),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,6,5,4,3,2,1})));
	r1.write().add(po::bound(62,63),po::layer_loc(new po::layer("anon 2",{po::byte(1)})));

	po::slab s = r1->read();
	ASSERT_EQ(boost::size(s),128);
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
		else if(idx == 62)
			ASSERT_TRUE(i && *i == 1);
		else
			ASSERT_TRUE(i == boost::none);
		++idx;
	}
}

TEST_F(region,layer_proj)
{
	po::region_loc r1 = po::region::undefined("test",128);

	r1.write().add(po::bound(2,8),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,7})));
	r1.write().add(po::bound(50,62),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,6,5,4,3,2,1})));
	r1.write().add(po::bound(62,63),po::layer_loc(new po::layer("anon 2",{po::byte(1)})));

	auto proj = r1->flatten();
	list<po::bound> expect({
		po::bound(0,2),
		po::bound(2,8),
		po::bound(8,50),
		po::bound(50,62),
		po::bound(62,63),
		po::bound(63,128)
	});

	for(auto i: proj)
		std::cout << i.first << ": " << i.second->name() << std::endl;

	unsigned long i = 0;
	while(i < expect.size())
	{
		std::cout << next(proj.begin(),i)->first << " vs " << *next(expect.begin(),i) << std::endl;
		ASSERT_TRUE(next(proj.begin(),i)->first == *next(expect.begin(),i));
		++i;
	}
}

