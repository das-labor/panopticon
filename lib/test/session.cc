#include <gtest/gtest.h>

#include <panopticon/database.hh>

using namespace po;

TEST(session,pe)
{
	session sess = pe("test.exe");
}
