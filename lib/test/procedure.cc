#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <boost/graph/isomorphism.hpp>
#include <gtest/gtest.h>
#include <panopticon/procedure.hh>
#include "architecture.hh"

using namespace po;
using namespace boost;

/*class disassembler_mockup : public po::disassembler<test_tag>
{
public:
	disassembler_mockup(const std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> &states)
	: m_states(states) {}

	virtual std::pair<bool,typename po::rule<test_tag>::tokiter> match(typename po::rule<test_tag>::tokiter begin, typename po::rule<test_tag>::tokiter end, po::sem_state<test_tag> &state) const
	{
		if(begin == end)
			return std::make_pair(false,end);

		auto i = m_states.find(*begin);

		if(i != m_states.end())
		{
			state.mnemonics = i->second.mnemonics;
			state.jumps = i->second.jumps;

			return std::make_pair(true,std::next(begin,std::accumulate(state.mnemonics.begin(),state.mnemonics.end(),0,[](size_t acc, const po::mnemonic &m) { return m.area.size() + acc; })));
		}
		else
			return std::make_pair(false,begin);
	}

private:
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> m_states;
};*/

TEST(procedure,add_single)
{
	/*std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;

	{
		po::sem_state<test_tag> st(0);
		st.mnemonic(1,"test");
		states.insert(std::make_pair(0,st));
	}

	disassembler_mockup mockup(states);
	po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 1);

	po::bblock_cptr bb = *proc->basic_blocks.begin();

	ASSERT_EQ(bb->mnemonics().size(), 1);
	ASSERT_EQ(bb->mnemonics()[0].opcode, "test");
	ASSERT_EQ(bb->mnemonics()[0].area, po::range<po::addr_t>(0,1));
	ASSERT_EQ(bb->incoming().size(), 0);
	ASSERT_EQ(bb->outgoing().size(), 0);
	ASSERT_EQ(bb->area(), po::range<po::addr_t>(0,1));
	ASSERT_EQ(bb, proc->entry);
	ASSERT_EQ(proc->callees.size(), 0);
	ASSERT_EQ(proc->callers.size(), 0);
	ASSERT_NE(proc->name, "");*/

	ASSERT_TRUE(false);
}

TEST(procedure,continuous)
{
	/*std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,3,4,5});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::addr_t p, const std::string &n) -> void
	{
		po::sem_state<test_tag> st(p);
		st.mnemonic(1,n);
		st.jump(p+1);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::addr_t p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		CPPUNIT_ASSERT(m.operands.empty());
		CPPUNIT_ASSERT(m.instructions.empty());
		ASSERT_EQ(m.area, po::range<po::addr_t>(p,p+1));
	};

	add(0,"test0");
	add(1,"test1");
	add(2,"test2");
	add(3,"test3");
	add(4,"test4");
	add(5,"test5");

	disassembler_mockup mockup(states);
	po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 1);

	po::bblock_cptr bb = *proc->basic_blocks.begin();

	ASSERT_EQ(bb->mnemonics().size(), 6);

	check(bb->mnemonics()[0],"test0",0);
	check(bb->mnemonics()[1],"test1",1);
	check(bb->mnemonics()[2],"test2",2);
	check(bb->mnemonics()[3],"test3",3);
	check(bb->mnemonics()[4],"test4",4);
	check(bb->mnemonics()[5],"test5",5);

	ASSERT_EQ(bb->incoming().size(), 0);
	ASSERT_EQ(bb->outgoing().size(), 1);
	ASSERT_EQ(bb->outgoing().front().bblock.lock(), 0);
	CPPUNIT_ASSERT(bb->outgoing().front().condition.relations.empty());
	CPPUNIT_ASSERT(bb->outgoing().front().value.is_constant());
	ASSERT_EQ(bb->outgoing().front().value.to_constant().content(), 6);
	ASSERT_EQ(bb->area(), po::range<po::addr_t>(0,6));
	ASSERT_EQ(bb, proc->entry);
	ASSERT_EQ(proc->callees.size(), 0);
	ASSERT_EQ(proc->callers.size(), 0);
	ASSERT_NE(proc->name, "");*/

	ASSERT_TRUE(false);
}

TEST(procedure,branch)
{
	/*std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::addr_t p, const std::string &n, po::addr_t b1, po::addr_t b2) -> void
	{
		po::sem_state<test_tag> st(p);
		st.mnemonic(1,n);
		st.jump(b1);
		if(b2 != po::naddr)
			st.jump(b2);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::addr_t p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		CPPUNIT_ASSERT(m.operands.empty());
		CPPUNIT_ASSERT(m.instructions.empty());
		ASSERT_EQ(m.area, po::range<po::addr_t>(p,p+1));
	};

	add(0,"test0",1,2);
	add(1,"test1",3,po::naddr);
	add(2,"test2",1,po::naddr);

	disassembler_mockup mockup(states);
	po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 3);

	auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
	auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 1; });
	auto i2 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 2; });

	ASSERT_NE(i0, proc->basic_blocks.end());
	ASSERT_NE(i1, proc->basic_blocks.end());
	ASSERT_NE(i2, proc->basic_blocks.end());

	po::bblock_cptr bb0 = *i0;
	po::bblock_cptr bb1 = *i1;
	po::bblock_cptr bb2 = *i2;

	ASSERT_EQ(bb0->mnemonics().size(), 1);
	ASSERT_EQ(bb1->mnemonics().size(), 1);
	ASSERT_EQ(bb2->mnemonics().size(), 1);

	ASSERT_EQ(bb0->incoming().size(), 0);
	check(bb0->mnemonics()[0],"test0",0);
	ASSERT_EQ(bb0->outgoing().size(), 2);

	ASSERT_EQ(bb1->incoming().size(), 2);
	check(bb1->mnemonics()[0],"test1",1);
	ASSERT_EQ(bb1->outgoing().size(), 1);

	ASSERT_EQ(bb2->incoming().size(), 1);
	check(bb2->mnemonics()[0],"test2",2);
	ASSERT_EQ(bb2->outgoing().size(), 1);*/

	ASSERT_TRUE(false);
}

TEST(procedure,loop)
{
	/*std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::addr_t p, const std::string &n, po::addr_t b1, po::addr_t b2) -> void
	{
		po::sem_state<test_tag> st(p);
		st.mnemonic(1,n);
		st.jump(b1);
		if(b2 != po::naddr)
			st.jump(b2);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::addr_t p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		CPPUNIT_ASSERT(m.operands.empty());
		CPPUNIT_ASSERT(m.instructions.empty());
		ASSERT_EQ(m.area, po::range<po::addr_t>(p,p+1));
	};

	add(0,"test0",1,po::naddr);
	add(1,"test1",2,po::naddr);
	add(2,"test2",0,po::naddr);

	disassembler_mockup mockup(states);
	po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 1);

	po::bblock_cptr bb = *proc->basic_blocks.begin();

	ASSERT_EQ(bb->mnemonics().size(), 3);

	check(bb->mnemonics()[0],"test0",0);
	check(bb->mnemonics()[1],"test1",1);
	check(bb->mnemonics()[2],"test2",2);

	ASSERT_EQ(bb->incoming().size(), 1);
	ASSERT_EQ(bb->outgoing().size(), 1);*/

	ASSERT_TRUE(false);
}

TEST(procedure,empty)
{
	/*std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	disassembler_mockup mockup(states);
	po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 0);*/

	ASSERT_TRUE(false);
}

TEST(procedure,refine)
{
	/*std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::addr_t p, size_t l, const std::string &n, po::addr_t b1) -> void
	{
		po::sem_state<test_tag> st(p);
		st.mnemonic(l,n);
		st.jump(b1);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::addr_t p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		CPPUNIT_ASSERT(m.operands.empty());
		CPPUNIT_ASSERT(m.instructions.empty());
		ASSERT_EQ(m.area, po::range<po::addr_t>(p,p+1));
	};

	*
	 * test0
	 *  -"-  test1
	 * test2
	 *
	add(0,2,"test0",2);
	add(2,1,"test2",1);
	add(1,1,"test1",2);

	disassembler_mockup mockup(states);
	po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 2);

	auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
	auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 1; });

	ASSERT_NE(i0, proc->basic_blocks.end());
	ASSERT_NE(i1, proc->basic_blocks.end());

	po::bblock_cptr bb0 = *i0;
	po::bblock_cptr bb1 = *i1;

	ASSERT_EQ(bb0->mnemonics().size(), 1);
	ASSERT_EQ(bb1->mnemonics().size(), 2);

	ASSERT_EQ(bb0->incoming().size(), 0);
	check(bb0->mnemonics()[0],"test0",0);
	ASSERT_EQ(bb0->outgoing().size(), 1);

	ASSERT_EQ(bb1->incoming().size(), 2);
	check(bb1->mnemonics()[0],"test1",1);
	check(bb1->mnemonics()[1],"test2",2);
	ASSERT_EQ(bb1->outgoing().size(), 1);*/

	ASSERT_TRUE(false);
}

TEST(procedure,continue)
{
	/*po::proc_ptr proc(new po::procedure());
	po::mnemonic mne0(po::range<po::addr_t>(0,1),"test0","",{},{});
	po::mnemonic mne1(po::range<po::addr_t>(1,2),"test1","",{},{});
	po::mnemonic mne2(po::range<po::addr_t>(2,3),"test2","",{},{});
	po::mnemonic mne3(po::range<po::addr_t>(6,7),"test6","",{},{});
	po::bblock_ptr bb0(new po::basic_block());
	po::bblock_ptr bb1(new po::basic_block());
	po::bblock_ptr bb2(new po::basic_block());

	bb0->mutate_mnemonics([&](std::vector<po::mnemonic> &ms)
	{
		ms.push_back(mne0);
		ms.push_back(mne1);
	});

	bb1->mutate_mnemonics([&](std::vector<po::mnemonic> &ms)
	{
		ms.push_back(mne2);
	});

	bb2->mutate_mnemonics([&](std::vector<po::mnemonic> &ms)
	{
		ms.push_back(mne3);
	});

	bb0->mutate_incoming([&](std::list<po::ctrans> &in)
	{
		in.push_back(po::ctrans(po::guard(),po::constant(42,32)));
	});

	bb2->mutate_outgoing([&](std::list<po::ctrans> &out)
	{
		out.push_back(po::ctrans(po::guard(),po::constant(40,32)));
	});

	po::unconditional_jump(bb0,bb1);
	po::unconditional_jump(bb0,bb2);

	proc->basic_blocks.insert(bb0);
	proc->basic_blocks.insert(bb1);
	proc->basic_blocks.insert(bb2);
	proc->entry = bb0;

	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,40,41,42});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::addr_t p, const std::string &n, po::addr_t b1, po::addr_t b2) -> void
	{
		po::sem_state<test_tag> st(p);
		st.mnemonic(1,n);
		if(b1 != po::naddr)
			st.jump(b1);
		if(b2 != po::naddr)
			st.jump(b2);

		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::addr_t p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		CPPUNIT_ASSERT(m.operands.empty());
		CPPUNIT_ASSERT(m.instructions.empty());
		ASSERT_EQ(m.area, po::range<po::addr_t>(p,p+1));
	};

	add(0,"test0",1,po::naddr);
	add(1,"test1",2,6);
	add(2,"test2",po::naddr,po::naddr);
	add(6,"test6",40,po::naddr);

	add(40,"test40",41,po::naddr);
	add(41,"test41",42,po::naddr);
	add(42,"test42",55,0);

	disassembler_mockup mockup(states);
	proc = po::procedure::disassemble(proc,mockup,bytes,40);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 4);

	auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
	auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 2; });
	auto i2 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 6; });
	auto i3 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 40; });

	ASSERT_NE(i0, proc->basic_blocks.end());
	ASSERT_NE(i1, proc->basic_blocks.end());
	ASSERT_NE(i2, proc->basic_blocks.end());
	ASSERT_NE(i3, proc->basic_blocks.end());

	po::bblock_cptr bbo0 = *i0;
	po::bblock_cptr bbo1 = *i1;
	po::bblock_cptr bbo2 = *i2;
	po::bblock_cptr bbo3 = *i3;

	ASSERT_EQ(bbo0->incoming().size(), 1);
	ASSERT_EQ(bbo0->incoming().begin()->bblock.lock(), bbo3);
	ASSERT_EQ(bbo0->mnemonics().size(), 2);
	check(bbo0->mnemonics()[0],"test0",0);
	check(bbo0->mnemonics()[1],"test1",1);
	ASSERT_EQ(bbo0->outgoing().size(), 2);
	ASSERT_EQ(bbo0->outgoing().begin()->bblock.lock() == bbo1 || bbo0->outgoing().begin()->bblock.lock(), bbo2);
	ASSERT_EQ(std::next(bbo0->outgoing().begin())->bblock.lock() == bbo1 || std::next(bbo0->outgoing().begin())->bblock.lock(), bbo2);

	ASSERT_EQ(bbo1->incoming().size(), 1);
	ASSERT_EQ(bbo1->incoming().begin()->bblock.lock(), bbo0);
	ASSERT_EQ(bbo1->mnemonics().size(), 1);
	check(bbo1->mnemonics()[0],"test2",2);
	ASSERT_EQ(bbo1->outgoing().size(), 0);

	ASSERT_EQ(bbo2->incoming().size(), 1);
	ASSERT_EQ(bbo2->incoming().begin()->bblock.lock(), bbo0);
	ASSERT_EQ(bbo2->mnemonics().size(), 1);
	check(bbo2->mnemonics()[0],"test6",6);
	ASSERT_EQ(bbo2->outgoing().size(), 1);
	ASSERT_EQ(bbo2->outgoing().begin()->bblock.lock(), bbo3);

	ASSERT_EQ(bbo3->incoming().size(), 1);
	ASSERT_EQ(bbo3->incoming().begin()->bblock.lock(), bbo2);
	ASSERT_EQ(bbo3->mnemonics().size(), 3);
	check(bbo3->mnemonics()[0],"test40",40);
	check(bbo3->mnemonics()[1],"test41",41);
	check(bbo3->mnemonics()[2],"test42",42);
	ASSERT_EQ(bbo3->outgoing().size(), 2);
	ASSERT_EQ(bbo3->outgoing().begin()->bblock.lock() == bbo0 || bbo3->outgoing().begin()->value.to_constant().content(), 55);
	ASSERT_EQ(std::next(bbo3->outgoing().begin())->bblock.lock() == bbo0 || std::next(bbo3->outgoing().begin())->value.to_constant().content(), 55);

	ASSERT_EQ(proc->entry, bbo0);*/

	ASSERT_TRUE(false);
}

TEST(procedure,entry_split)
{
	/*po::proc_ptr proc(new po::procedure());
	po::mnemonic mne0(po::range<po::addr_t>(0,1),"test0","",{},{});
	po::mnemonic mne1(po::range<po::addr_t>(1,2),"test1","",{},{});
	po::bblock_ptr bb0(new po::basic_block());

	bb0->mutate_mnemonics([&](std::vector<po::mnemonic> &ms)
	{
		ms.push_back(mne0);
		ms.push_back(mne1);
	});

	bb0->mutate_outgoing([&](std::list<po::ctrans> &out)
	{
		out.push_back(po::ctrans(po::guard(),po::constant(2,32)));
	});

	proc->basic_blocks.insert(bb0);
	proc->entry = bb0;

	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::addr_t p, const std::string &n, po::addr_t b1, po::addr_t b2) -> void
	{
		po::sem_state<test_tag> st(p);
		st.mnemonic(1,n);
		if(b1 != po::naddr)
			st.jump(b1);
		if(b2 != po::naddr)
			st.jump(b2);

		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::addr_t p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		CPPUNIT_ASSERT(m.operands.empty());
		CPPUNIT_ASSERT(m.instructions.empty());
		ASSERT_EQ(m.area, po::range<po::addr_t>(p,p+1));
	};

	add(0,"test0",1,po::naddr);
	add(1,"test1",2,po::naddr);

	add(2,"test2",1,po::naddr);

	disassembler_mockup mockup(states);
	proc = po::procedure::disassemble(proc,mockup,bytes,2);

	CPPUNIT_ASSERT(proc);
	ASSERT_EQ(proc->basic_blocks.size(), 2);

	auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
	auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 1; });

	ASSERT_NE(i0, proc->basic_blocks.end());
	ASSERT_NE(i1, proc->basic_blocks.end());

	po::bblock_cptr bbo0 = *i0;
	po::bblock_cptr bbo1 = *i1;

	ASSERT_EQ(proc->entry, bbo0);
	ASSERT_EQ(bbo0->mnemonics().size(), 1);
	check(bbo0->mnemonics()[0],"test0",0);
	ASSERT_EQ(bbo1->mnemonics().size(), 2);*/

	ASSERT_TRUE(false);
}

TEST(procedure,variable)
{
	ASSERT_TRUE(false);
}

/*
 *   bb0 ----+
 *    |  \   |
 *   bb1  a  |
 *   /  \    |
 *   bb2 \   |
 *   \   /   |
 * +-bb3---2 |
 * +/ |      |
 *    bb4----+
 */
TEST(procedure,marshal)
{
	bblock_loc bb0(new basic_block({mnemonic(bound(0,5),"test","",{},{})}));
	bblock_loc bb1(new basic_block({mnemonic(bound(5,10),"test","",{},{})}));
	bblock_loc bb2(new basic_block({mnemonic(bound(10,12),"test","",{},{})}));
	bblock_loc bb3(new basic_block({mnemonic(bound(12,20),"test","",{},{})}));
	bblock_loc bb4(new basic_block({mnemonic(bound(20,21),"test","",{},{})}));
	rvalue rv1 = variable("a",8);
	rvalue rv2 = constant(42);
	proc_loc proc(new procedure("p1"));

	auto vx0 = insert_node<variant<bblock_loc,rvalue>,guard>(bb0,proc.write().control_transfers);
	auto vx1 = insert_node<variant<bblock_loc,rvalue>,guard>(bb1,proc.write().control_transfers);
	auto vx2 = insert_node<variant<bblock_loc,rvalue>,guard>(bb2,proc.write().control_transfers);
	auto vx3 = insert_node<variant<bblock_loc,rvalue>,guard>(bb3,proc.write().control_transfers);
	auto vx4 = insert_node<variant<bblock_loc,rvalue>,guard>(bb4,proc.write().control_transfers);
	auto vx5 = insert_node<variant<bblock_loc,rvalue>,guard>(rv1,proc.write().control_transfers);
	auto vx6 = insert_node<variant<bblock_loc,rvalue>,guard>(rv2,proc.write().control_transfers);

	insert_edge(guard(),vx0,vx1,proc.write().control_transfers);
	insert_edge(guard(),vx0,vx5,proc.write().control_transfers);
	insert_edge(guard(),vx1,vx2,proc.write().control_transfers);
	insert_edge(guard(),vx2,vx3,proc.write().control_transfers);
	insert_edge(guard(),vx1,vx3,proc.write().control_transfers);
	insert_edge(guard(),vx3,vx3,proc.write().control_transfers);
	insert_edge(guard(),vx3,vx6,proc.write().control_transfers);
	insert_edge(guard(),vx3,vx4,proc.write().control_transfers);
	insert_edge(guard(),vx4,vx0,proc.write().control_transfers);

	proc.write().entry = bb0;

	rdf::storage st;
	save_point(st);

	std::unique_ptr<procedure> p2(unmarshal<procedure>(proc.tag(),st));

	ASSERT_EQ(proc->name, p2->name);
	ASSERT_TRUE(**proc->entry == **p2->entry);
	ASSERT_TRUE(boost::isomorphism(proc->control_transfers,p2->control_transfers));
	ASSERT_EQ(proc->rev_postorder(), p2->rev_postorder());
}
