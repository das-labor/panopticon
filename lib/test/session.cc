#include <gtest/gtest.h>

#include <panopticon/database.hh>

using namespace po;

TEST(session,pe)
{
	session sess = pe("test.exe");
	auto p = po::projection(sess.dbase->data);

	for(auto x: p)
		std::cout << x.first << ": " << x.second->name();
}
