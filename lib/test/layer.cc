#include <iostream>

#include <gtest/gtest.h>
#include <boost/range/algorithm/copy.hpp>
#include <panopticon/region.hh>

using namespace po;
using namespace std;

TEST(layer,map_layer)
{
	layer l1 = layer("add 1",[](tryte i) { return *i + 1; });
	vector<tryte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17});

	boost::copy(l1.filter(slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,anonymous_layer)
{
	layer l1 = layer("anon 1",128);
	layer l2 = layer("anon 2",{1,2,3,4,5,6});
	vector<tryte> r;

	ASSERT_EQ(128,boost::size(l1.filter(slab())));
	ASSERT_EQ(6,boost::size(l2.filter(slab())));

	boost::copy(l2.filter(slab()),back_inserter(r));
	ASSERT_EQ(r,vector<tryte>({1,2,3,4,5,6}));
}

TEST(layer,mutable_layer)
{
	vector<tryte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({1,2,3,4,5,1,1,8,9,10,11,12,13,1,15,16});
	layer l1("mut",std::unordered_map<offset,tryte>());

	l1.write(5,1);
	l1.write(6,1);
	l1.write(13,1);

	boost::copy(l1.filter(slab(d)),back_inserter(r));
	ASSERT_EQ(r,e);
}

TEST(layer,add)
{
	region_loc st = region::undefined("",12);

	st.write().add(bound(0,6),layer_loc(new layer("anon 2", {1,2,3,4,5,6})));
	st.write().add(bound(10,40),layer_loc(new layer("anon 3", {1,2,3,4,5,6})));
	st.write().add(bound(4,12),layer_loc(new layer("anon 4", {1,2,3,4,5,6})));
	auto proj = st->projection();

	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cout << p.first << ": " << p.second->name() << std::endl;
}

TEST(layer,projection)
{
	region_loc st = region::undefined("",134);
	layer_loc base(new layer("base",128));
	layer_loc xor1(new layer("xor",64));
	layer_loc add(new layer("add",27));
	layer_loc zlib(new layer("zlib",48));
	layer_loc aes(new layer("aes",32));

	st.write().add(bound(0,128),base);
	st.write().add(bound(0,64),xor1);
	st.write().add(bound(45,72),add);
	st.write().add(bound(80,128),zlib);
	st.write().add(bound(102,134),aes);

	auto proj = st->projection();
	boost::icl::interval_map<offset,layer_wloc> expect;

	expect += std::make_pair(bound(0,45),layer_wloc(xor1));
	expect += std::make_pair(bound(45,72),layer_wloc(add));
	expect += std::make_pair(bound(72,80),layer_wloc(base));
	expect += std::make_pair(bound(80,102),layer_wloc(zlib));
	expect += std::make_pair(bound(102,134),layer_wloc(aes));

	std::cerr << "proj:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cerr << p.first << " => " << p.second->name() << std::endl;
	std::cerr << "expect:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: expect)
		std::cerr << p.first << " => " << p.second->name() << std::endl;
	ASSERT_TRUE(proj == expect);
}
