#include <iostream>

#include <gtest/gtest.h>
#include <boost/range/algorithm/copy.hpp>
#include <panopticon/region.hh>

using namespace po;
using namespace std;

TEST(layer,map_layer)
{
	layer l1 = map_layer("add 1",[](tryte i) { return *i + 1; });
	vector<tryte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17});

	boost::copy(filter(l1,slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,anonymous_layer)
{
	layer l1 = anonymous_layer(128,"anon 1");
	layer l2 = anonymous_layer({1,2,3,4,5,6},"anon 2");
	vector<tryte> r;

	ASSERT_EQ(128,boost::size(filter(l1,slab())));
	ASSERT_EQ(6,boost::size(filter(l2,slab())));

	boost::copy(filter(l2,slab()),back_inserter(r));
	ASSERT_EQ(r,vector<tryte>({1,2,3,4,5,6}));
}

TEST(layer,mutable_layer)
{
	vector<tryte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({1,2,3,4,5,1,1,8,9,10,11,12,13,1,15,16});
	mutable_layer l1("mut");

	l1.data[5] = 1;
	l1.data[6] = 1;
	l1.data[13] = 1;

	boost::copy(filter(l1,slab(d)),back_inserter(r));
	ASSERT_EQ(r,e);
}

TEST(layer,add)
{
	region st("",12);

	st.add(bound(0,6),layer_loc(new layer(anonymous_layer({1,2,3,4,5,6},"anon 2"))));
	st.add(bound(10,40),layer_loc(new layer(anonymous_layer({1,2,3,4,5,6},"anon 3"))));
	st.add(bound(4,12),layer_loc(new layer(anonymous_layer({1,2,3,4,5,6},"anon 4"))));
	auto proj = st.projection();

	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cout << p.first << ": " << name(*p.second) << std::endl;
}

TEST(layer,projection)
{
	region st("",134);
	layer_loc base(new layer(anonymous_layer({},"base")));
	layer_loc xor1(new layer(anonymous_layer({},"xor")));
	layer_loc add(new layer(anonymous_layer({},"add")));
	layer_loc zlib(new layer(anonymous_layer({},"zlib")));
	layer_loc aes(new layer(anonymous_layer({},"aes")));

	st.add(bound(0,128),base);
	st.add(bound(0,64),xor1);
	st.add(bound(45,72),add);
	st.add(bound(80,128),zlib);
	st.add(bound(102,134),aes);

	auto proj = st.projection();
	boost::icl::interval_map<offset,layer_wloc> expect;

	expect += std::make_pair(bound(0,45),layer_wloc(xor1));
	expect += std::make_pair(bound(45,72),layer_wloc(add));
	expect += std::make_pair(bound(72,80),layer_wloc(base));
	expect += std::make_pair(bound(80,102),layer_wloc(zlib));
	expect += std::make_pair(bound(102,134),layer_wloc(aes));

	std::cerr << "proj:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cerr << p.first << " => " << name(*p.second) << std::endl;
	std::cerr << "expect:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: expect)
		std::cerr << p.first << " => " << name(*p.second) << std::endl;
	ASSERT_TRUE(proj == expect);
}
