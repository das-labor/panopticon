#include <iostream>

#include <gtest/gtest.h>
#include <boost/range/algorithm/copy.hpp>
#include <panopticon/layer.hh>

using namespace po;
using namespace std;

TEST(layer,map_layer)
{
	layer l1 = map_layer("add 1",[](uint8_t i) { return i + 1; });
	vector<byte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17});

	boost::copy(l1.filter(slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,anonymous_layer)
{
	layer l1 = anonymous_layer(128,"anon 1");
	layer l2 = anonymous_layer({1,2,3,4,5,6},"anon 2");
	vector<byte> r;

	ASSERT_EQ(128,boost::size(l1.filter(slab())));
	ASSERT_EQ(6,boost::size(l2.filter(slab())));

	boost::copy(l2.filter(slab()),back_inserter(r));
	ASSERT_EQ(r,vector<byte>({1,2,3,4,5,6}));
}

TEST(layer,mutable_layer)
{
	vector<byte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({1,2,3,4,5,0,0,8,9,10,11,12,13,0,15,16});
	mutable_layer l1("mut");

	l1.data[5] = 0;
	l1.data[6] = 0;
	l1.data[13] = 0;

	boost::copy(l1.filter(slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}
