#include <gtest/gtest.h>

#include <panopticon/mnemonic.hh>

using namespace po;

TEST(mnemonic,marshal)
{
	mnemonic mn1(bound(0,10),"op1","{8:-:eax} nog",{constant(1),variable("a",3)},{
		instr(int_add<rvalue>{constant(1),constant(2)},variable("a",2)),
		instr(int_add<rvalue>{constant(4),constant(2)},variable("a",1)),
		instr(univ_nop<rvalue>{variable("a",2)},variable("a",3))});

	uuid uu;
	rdf::statements st1 = marshal(&mn1,uu);

	ASSERT_GT(st1.size(),0);
	rdf::statements st2 = marshal(&mn1,uu);

	ASSERT_EQ(st1.size(),st2.size());
	ASSERT_TRUE(st1 == st2);

	rdf::storage store;

	for(auto s: st1)
	{
		std::cerr << s << std::endl;
		store.insert(s);
	}

	mnemonic mn2 = *std::unique_ptr<mnemonic>(unmarshal<mnemonic>(uu,store));

	ASSERT_TRUE(mn2 == mn1);
}
