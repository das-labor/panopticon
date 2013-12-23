#include <iostream>

#include <gtest/gtest.h>
#include <boost/range/algorithm/copy.hpp>
#include <panopticon/layer.hh>

using namespace po;
using namespace std;

TEST(layer,simple_map)
{
	layer l1 = map_layer("add 1",[](uint8_t i) { return i + 1; });
	vector<uint8_t> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16};

	boost::copy(l1.filter(slab(d)),ostream_iterator<int>(cout,", "));
	cout << endl;
}
