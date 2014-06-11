#include <gtest/gtest.h>

#include <panopticon/basic_block.hh>

using namespace po;

TEST(basic_block,marshal)
{
	mnemonic mn1(bound(0,10),"op1","{8:-:eax} nog",{constant(1),variable("a",3)},{
		instr(int_add<rvalue>{constant(1),constant(2)},variable("a",2)),
		instr(int_add<rvalue>{constant(4),constant(2)},variable("a",1)),
		instr(univ_nop<rvalue>{variable("a",2)},variable("a",3))});
	mnemonic mn2(bound(10,13),"op2","nig",{constant(1),variable("b",3,5)},{
		instr(int_add<rvalue>{constant(1),constant(2)},variable("b",2,6)),
		instr(int_add<rvalue>{constant(4),constant(2)},variable("b",1,7)),
		instr(univ_nop<rvalue>{variable("a",2)},variable("a",3))});
	mnemonic mn3(bound(13,20),"op3","{8:-:eax} {9::nol}",{constant(1),variable("c",3)},{
		instr(univ_nop<rvalue>{constant(66)},variable("c",3,5))});
	uuid uu;

	bblock_loc bb1(uu,new basic_block({mn1,mn2,mn3}));
	rdf::storage store;

	save_point(store);
	ASSERT_GT(store.count(),0);

	std::unique_ptr<basic_block> bb2(unmarshal<basic_block>(uu,store));

	ASSERT_TRUE(*bb2 == *bb1);
}

TEST(basic_block,guard_marshal)
{
	loc<guard> g1(new guard(std::list<relation>{
			relation(variable("a",8),relation::ULeq,constant(42)),
			relation(variable("c",9),relation::SGrtr,variable("b",8))}));
	loc<guard> g2(new guard(std::list<relation>{
			relation(constant(33),relation::Eq,undefined())}));

	rdf::storage store;
	save_point(store);
	ASSERT_GT(store.count(),0);

	g1.remove();
	g2.remove();

	save_point(store);
	ASSERT_EQ(store.count(),0);
}
