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

	boost::copy(filter(l1,slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,anonymous_layer)
{
	layer l1 = anonymous_layer(128,"anon 1");
	layer l2 = anonymous_layer({1,2,3,4,5,6},"anon 2");
	vector<byte> r;

	ASSERT_EQ(128,boost::size(filter(l1,slab())));
	ASSERT_EQ(6,boost::size(filter(l2,slab())));

	boost::copy(filter(l2,slab()),back_inserter(r));
	ASSERT_EQ(r,vector<byte>({1,2,3,4,5,6}));
}

TEST(layer,mutable_layer)
{
	vector<byte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({1,2,3,4,5,0,0,8,9,10,11,12,13,0,15,16});
	mutable_layer l1("mut");

	l1.data[5] = 0;
	l1.data[6] = 0;
	l1.data[13] = 0;

	boost::copy(filter(l1,slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,add)
{
	stack st;

	st.add(bound(0,6),layer_loc(boost::uuids::random_generator()(),new layer(anonymous_layer({1,2,3,4,5,6},"anon 2"))));
	auto proj = st.projection();

	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cout << p.first << ": " << name(*p.second) << std::endl;
}

/*
 * Graph:
 * [----------------- base ----------------]
 * [----xor----]            [-----zlib-----]
 *          [----add----]       [--aes--]
 *
 * Projection:
 * [--xor--][----add----][ba][z][--aes--][z]
 *
TEST(layer,projection)
{
	using bytes = std::vector<uint8_t>;

	layer_loc base_as("base",bound(0,128),std::function<bytes(const bytes&)>());
	layer_loc xor_as("xor",bound(0,64),std::function<bytes(const bytes&)>());
	layer_loc add_as("add",bound(0,27),std::function<bytes(const bytes&)>());
	layer_loc zlib_as("zlib",bound(0,128),std::function<bytes(const bytes&)>());
	layer_loc aes_as("aes",bound(0,32),std::function<bytes(const bytes&)>());
	po::graph<layer_loc,bound> g;

	auto base_vx = g.insert_node(base_as);
	auto xor_vx = g.insert_node(xor_as);
	auto add_vx = g.insert_node(add_as);
	auto zlib_vx = g.insert_node(zlib_as);
	auto aes_vx = g.insert_node(aes_as);

	g.insert_edge(bound(0,64),xor_vx,base_vx);
	g.insert_edge(bound(64,72),add_vx,base_vx);
	g.insert_edge(bound(45,64),add_vx,xor_vx);
	g.insert_edge(bound(80,128),zlib_vx,base_vx);
	g.insert_edge(bound(32,64),aes_vx,zlib_vx);

	auto proj = po::projection(base_as,g);
	auto expect = std::list<std::pair<bound,layer_loc>>{
		std::make_pair(bound(0,45),xor_as),
		std::make_pair(bound(0,27),add_as),
		std::make_pair(bound(72,80),base_as),
		std::make_pair(bound(0,32),zlib_as),
		std::make_pair(bound(0,32),aes_as),
		std::make_pair(bound(64,128),zlib_as)
	};

	*std::cerr << "proj:" << std::endl;
	for(const std::pair<bound,layer_loc> &p: proj)
		std::cerr << p.first << " => " << p.second.name << std::endl;
	std::cerr << "expect:" << std::endl;
	for(const std::pair<bound,layer_loc> &p: expect)
		std::cerr << p.first << " => " << p.second.name << std::endl;*
	CPPUNIT_ASSERT(proj == expect);
}*/
