#include <gtest/gtest.h>

#include <panopticon/database.hh>

using namespace po;

TEST(session,pe)
{
	session sess = pe("test.exe");
	auto p = po::projection(sess.dbase->data);

	region_loc undef = region::undefined("undef",0xc0000000ull);
	undef->read();

	for(auto x: p)
	{
		std::cout << x.first << ": " << x.second->name() << std::endl;
		for(auto y: x.second->flatten())
		{
			std::cout << "\t" << y.first << ": " << y.second->name() << std::endl;
			slab sl = y.second->filter(slab());
		}

		size_t i = 100;
		while(i--)
			slab sl = x.second->read();
	}
}
