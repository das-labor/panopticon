#include <iostream>

#include <gtest/gtest.h>
#include <panopticon/region.hh>

using namespace po;
using namespace std;

TEST(region,tree)
{
	regions regs;
	region_loc r1 = region::undefined("base",128);
	region_loc r2 = region::undefined("zlib",64);
	region_loc r3 = region::undefined("aes",48);

	auto vx1 = regs.insert_node(r1);
	auto vx2 = regs.insert_node(r2);
	auto vx3 = regs.insert_node(r3);

	regs.insert_edge(bound(32,96),vx2,vx1);
	regs.insert_edge(bound(16,32),vx3,vx1);
	regs.insert_edge(bound(0,32),vx3,vx2);

	auto t = spanning_tree(regs);
	decltype(t) expect({
		make_pair(region_wloc(r2),region_wloc(r1)),
		make_pair(region_wloc(r3),region_wloc(r1))
	});

	for(auto i: t)
	{
		std::cout << i.first->name() << " -> " << i.second->name() << std::endl;
	}

	ASSERT_TRUE(t == expect);
}

TEST(region,proj)
{
	regions regs;
	region_loc r1 = region::undefined("base",128);
	region_loc r2 = region::undefined("zlib",64);
	region_loc r3 = region::undefined("aes",48);

	auto vx1 = regs.insert_node(r1);
	auto vx2 = regs.insert_node(r2);
	auto vx3 = regs.insert_node(r3);

	regs.insert_edge(bound(32,96),vx2,vx1);
	regs.insert_edge(bound(16,32),vx3,vx1);
	regs.insert_edge(bound(0,32),vx3,vx2);

	auto proj = projection(regs);
	decltype(proj) expect({
		make_pair(bound(0,16),region_wloc(r1)),
		make_pair(bound(0,48),region_wloc(r3)),
		make_pair(bound(32,64),region_wloc(r2)),
		make_pair(bound(96,128),region_wloc(r1))
	});

	for(auto i: proj)
	{
		std::cout << i.first << ": " << i.second->name() << std::endl;
	}

	ASSERT_TRUE(proj == expect);
}

TEST(region,read_undef)
{
	region_loc r1 = region::undefined("test",128);
	slab s = r1->read();

	ASSERT_EQ(boost::size(s),128);

	for(auto i: s)
		ASSERT_EQ(i,boost::none);
}

TEST(region,read_one_layer)
{
	region_loc r1 = region::undefined("test",128);

	r1.write().add(bound(1,7),layer_loc(new layer("anon 2",{1,2,3,4,5,6})));
	r1.write().add(bound(50,62),layer_loc(new layer("anon 2",{1,2,3,4,5,6,6,5,4,3,2,1})));
	r1.write().add(bound(62,63),layer_loc(new layer("anon 2",{byte(1)})));

	slab s = r1->read();
	ASSERT_EQ(boost::size(s),128);
	size_t idx = 0;

	for(auto i: s)
	{
		if(idx >= 1 && idx < 7)
			ASSERT_TRUE(*i == idx);
		else if(idx >= 50 && idx < 56)
			ASSERT_TRUE(*i == idx - 49);
		else if(idx >= 56 && idx < 62)
			ASSERT_TRUE(*i == 6 - (idx - 56));
		else if(idx == 62)
			ASSERT_TRUE(*i == 1);
		else
			ASSERT_TRUE(i == boost::none);
	}
}
