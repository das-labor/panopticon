#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <cppunit/extensions/HelperMacros.h>

#include <procedure.hh>

#include "test_architecture.hh"

class disassembler_mockup : public po::disassembler<test_tag>
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
};

class ProcedureTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(ProcedureTest);
	CPPUNIT_TEST(testAddSingle);
	CPPUNIT_TEST(testContinuous);
	CPPUNIT_TEST(testBranch);
	CPPUNIT_TEST(testLoop);
	CPPUNIT_TEST(testEmpty);
	//CPPUNIT_TEST(testRefine);
	CPPUNIT_TEST(testContinue);
	CPPUNIT_TEST(testEntrySplit);
	CPPUNIT_TEST(testVariable);
	CPPUNIT_TEST_SUITE_END();

public:
	void testAddSingle(void)
	{
		std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0});
		std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;

		{
			po::sem_state<test_tag> st(0);
			st.mnemonic(1,"test");
			states.insert(std::make_pair(0,st));
		}

		disassembler_mockup mockup(states);
		po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

		CPPUNIT_ASSERT(proc);
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 1);
		
		po::bblock_cptr bb = *proc->basic_blocks.begin();

		CPPUNIT_ASSERT(bb->mnemonics().size() == 1);
		CPPUNIT_ASSERT(bb->mnemonics()[0].opcode == "test");
		CPPUNIT_ASSERT(bb->mnemonics()[0].area == po::range<po::addr_t>(0,1));
		CPPUNIT_ASSERT(bb->incoming().size() == 0);
		CPPUNIT_ASSERT(bb->outgoing().size() == 0);
		CPPUNIT_ASSERT(bb->area() == po::range<po::addr_t>(0,1));
		CPPUNIT_ASSERT(bb == proc->entry);
		CPPUNIT_ASSERT(proc->callees.size() == 0);
		CPPUNIT_ASSERT(proc->callers.size() == 0);
		CPPUNIT_ASSERT(proc->name != "");
	}

	void testContinuous(void)
	{
		std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,3,4,5});
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
			CPPUNIT_ASSERT(m.opcode == n);
			CPPUNIT_ASSERT(m.operands.empty());
			CPPUNIT_ASSERT(m.instructions.empty());
			CPPUNIT_ASSERT(m.area == po::range<po::addr_t>(p,p+1));
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
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 1);
		
		po::bblock_cptr bb = *proc->basic_blocks.begin();

		CPPUNIT_ASSERT(bb->mnemonics().size() == 6);
		
		check(bb->mnemonics()[0],"test0",0);
		check(bb->mnemonics()[1],"test1",1);
		check(bb->mnemonics()[2],"test2",2);
		check(bb->mnemonics()[3],"test3",3);
		check(bb->mnemonics()[4],"test4",4);
		check(bb->mnemonics()[5],"test5",5);

		CPPUNIT_ASSERT(bb->incoming().size() == 0);
		CPPUNIT_ASSERT(bb->outgoing().size() == 1);
		CPPUNIT_ASSERT(bb->outgoing().front().bblock.lock() == 0);
		CPPUNIT_ASSERT(bb->outgoing().front().guard.relations.empty());
		CPPUNIT_ASSERT(bb->outgoing().front().value.is_constant());
		CPPUNIT_ASSERT(bb->outgoing().front().value.to_constant().content() == 6);
		CPPUNIT_ASSERT(bb->area() == po::range<po::addr_t>(0,6));
		CPPUNIT_ASSERT(bb == proc->entry);
		CPPUNIT_ASSERT(proc->callees.size() == 0);
		CPPUNIT_ASSERT(proc->callers.size() == 0);
		CPPUNIT_ASSERT(proc->name != "");
	}

	void testBranch(void)
	{
		std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
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
			CPPUNIT_ASSERT(m.opcode == n);
			CPPUNIT_ASSERT(m.operands.empty());
			CPPUNIT_ASSERT(m.instructions.empty());
			CPPUNIT_ASSERT(m.area == po::range<po::addr_t>(p,p+1));
		};

		add(0,"test0",1,2);
		add(1,"test1",3,po::naddr);
		add(2,"test2",1,po::naddr);

		disassembler_mockup mockup(states);
		po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

		CPPUNIT_ASSERT(proc);
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 3);
		
		auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
		auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 1; });
		auto i2 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 2; });

		CPPUNIT_ASSERT(i0 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i1 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i2 != proc->basic_blocks.end());

		po::bblock_cptr bb0 = *i0;
		po::bblock_cptr bb1 = *i1;
		po::bblock_cptr bb2 = *i2;

		CPPUNIT_ASSERT(bb0->mnemonics().size() == 1);
		CPPUNIT_ASSERT(bb1->mnemonics().size() == 1);
		CPPUNIT_ASSERT(bb2->mnemonics().size() == 1);
		
		CPPUNIT_ASSERT(bb0->incoming().size() == 0);
		check(bb0->mnemonics()[0],"test0",0);
		CPPUNIT_ASSERT(bb0->outgoing().size() == 2);

		CPPUNIT_ASSERT(bb1->incoming().size() == 2);
		check(bb1->mnemonics()[0],"test1",1);
		CPPUNIT_ASSERT(bb1->outgoing().size() == 1);
		
		CPPUNIT_ASSERT(bb2->incoming().size() == 1);
		check(bb2->mnemonics()[0],"test2",2);
		CPPUNIT_ASSERT(bb2->outgoing().size() == 1);
	}

	void testLoop(void)
	{
		std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
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
			CPPUNIT_ASSERT(m.opcode == n);
			CPPUNIT_ASSERT(m.operands.empty());
			CPPUNIT_ASSERT(m.instructions.empty());
			CPPUNIT_ASSERT(m.area == po::range<po::addr_t>(p,p+1));
		};

		add(0,"test0",1,po::naddr);
		add(1,"test1",2,po::naddr);
		add(2,"test2",0,po::naddr);

		disassembler_mockup mockup(states);
		po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

		CPPUNIT_ASSERT(proc);
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 1);
		
		po::bblock_cptr bb = *proc->basic_blocks.begin();

		CPPUNIT_ASSERT(bb->mnemonics().size() == 3);
		
		check(bb->mnemonics()[0],"test0",0);
		check(bb->mnemonics()[1],"test1",1);
		check(bb->mnemonics()[2],"test2",2);

		CPPUNIT_ASSERT(bb->incoming().size() == 1);
		CPPUNIT_ASSERT(bb->outgoing().size() == 1);
	}

	void testEmpty(void)
	{
		std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({});
		std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
		disassembler_mockup mockup(states);
		po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

		CPPUNIT_ASSERT(proc);
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 0);	
	}

	void testRefine(void)
	{
		std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
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
			CPPUNIT_ASSERT(m.opcode == n);
			CPPUNIT_ASSERT(m.operands.empty());
			CPPUNIT_ASSERT(m.instructions.empty());
			CPPUNIT_ASSERT(m.area == po::range<po::addr_t>(p,p+1));
		};

		/*
		 * test0
		 *  -"-  test1
		 * test2
		 */
		add(0,2,"test0",2);
		add(2,1,"test2",1);
		add(1,1,"test1",2);

		disassembler_mockup mockup(states);
		po::proc_ptr proc = po::procedure::disassemble(0,mockup,bytes,0);

		CPPUNIT_ASSERT(proc);
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 2);
		
		auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
		auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 1; });

		CPPUNIT_ASSERT(i0 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i1 != proc->basic_blocks.end());

		po::bblock_cptr bb0 = *i0;
		po::bblock_cptr bb1 = *i1;

		CPPUNIT_ASSERT(bb0->mnemonics().size() == 1);
		CPPUNIT_ASSERT(bb1->mnemonics().size() == 2);
		
		CPPUNIT_ASSERT(bb0->incoming().size() == 0);
		check(bb0->mnemonics()[0],"test0",0);
		CPPUNIT_ASSERT(bb0->outgoing().size() == 1);

		CPPUNIT_ASSERT(bb1->incoming().size() == 2);
		check(bb1->mnemonics()[0],"test1",1);
		check(bb1->mnemonics()[1],"test2",2);
		CPPUNIT_ASSERT(bb1->outgoing().size() == 1);
	}

	void testContinue(void)
	{
		po::proc_ptr proc(new po::procedure());
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
			CPPUNIT_ASSERT(m.opcode == n);
			CPPUNIT_ASSERT(m.operands.empty());
			CPPUNIT_ASSERT(m.instructions.empty());
			CPPUNIT_ASSERT(m.area == po::range<po::addr_t>(p,p+1));
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
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 4);
		
		auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
		auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 2; });
		auto i2 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 6; });
		auto i3 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 40; });

		CPPUNIT_ASSERT(i0 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i1 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i2 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i3 != proc->basic_blocks.end());

		po::bblock_cptr bbo0 = *i0;
		po::bblock_cptr bbo1 = *i1;
		po::bblock_cptr bbo2 = *i2;
		po::bblock_cptr bbo3 = *i3;

		CPPUNIT_ASSERT(bbo0->incoming().size() == 1);
		CPPUNIT_ASSERT(bbo0->incoming().begin()->bblock.lock() == bbo3);
		CPPUNIT_ASSERT(bbo0->mnemonics().size() == 2);
		check(bbo0->mnemonics()[0],"test0",0);
		check(bbo0->mnemonics()[1],"test1",1);
		CPPUNIT_ASSERT(bbo0->outgoing().size() == 2);
		CPPUNIT_ASSERT(bbo0->outgoing().begin()->bblock.lock() == bbo1 || bbo0->outgoing().begin()->bblock.lock() == bbo2);
		CPPUNIT_ASSERT(std::next(bbo0->outgoing().begin())->bblock.lock() == bbo1 || std::next(bbo0->outgoing().begin())->bblock.lock() == bbo2);
		
		CPPUNIT_ASSERT(bbo1->incoming().size() == 1);
		CPPUNIT_ASSERT(bbo1->incoming().begin()->bblock.lock() == bbo0);
		CPPUNIT_ASSERT(bbo1->mnemonics().size() == 1);
		check(bbo1->mnemonics()[0],"test2",2);
		CPPUNIT_ASSERT(bbo1->outgoing().size() == 0);
		
		CPPUNIT_ASSERT(bbo2->incoming().size() == 1);
		CPPUNIT_ASSERT(bbo2->incoming().begin()->bblock.lock() == bbo0);
		CPPUNIT_ASSERT(bbo2->mnemonics().size() == 1);
		check(bbo2->mnemonics()[0],"test6",6);
		CPPUNIT_ASSERT(bbo2->outgoing().size() == 1);
		CPPUNIT_ASSERT(bbo2->outgoing().begin()->bblock.lock() == bbo3);

		CPPUNIT_ASSERT(bbo3->incoming().size() == 1);
		CPPUNIT_ASSERT(bbo3->incoming().begin()->bblock.lock() == bbo2);
		CPPUNIT_ASSERT(bbo3->mnemonics().size() == 3);
		check(bbo3->mnemonics()[0],"test40",40);
		check(bbo3->mnemonics()[1],"test41",41);
		check(bbo3->mnemonics()[2],"test42",42);
		CPPUNIT_ASSERT(bbo3->outgoing().size() == 2);
		CPPUNIT_ASSERT(bbo3->outgoing().begin()->bblock.lock() == bbo0 || bbo3->outgoing().begin()->value.to_constant().content() == 55);
		CPPUNIT_ASSERT(std::next(bbo3->outgoing().begin())->bblock.lock() == bbo0 || std::next(bbo3->outgoing().begin())->value.to_constant().content() == 55);

		CPPUNIT_ASSERT(proc->entry == bbo0);
	}

	void testEntrySplit(void)
	{
		po::proc_ptr proc(new po::procedure());
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
			CPPUNIT_ASSERT(m.opcode == n);
			CPPUNIT_ASSERT(m.operands.empty());
			CPPUNIT_ASSERT(m.instructions.empty());
			CPPUNIT_ASSERT(m.area == po::range<po::addr_t>(p,p+1));
		};

		add(0,"test0",1,po::naddr);
		add(1,"test1",2,po::naddr);
		
		add(2,"test2",1,po::naddr);

		disassembler_mockup mockup(states);
		proc = po::procedure::disassemble(proc,mockup,bytes,2);

		CPPUNIT_ASSERT(proc);
		CPPUNIT_ASSERT(proc->basic_blocks.size() == 2);
		
		auto i0 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 0; });
		auto i1 = std::find_if(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](po::bblock_ptr bb) { return bb->area().begin == 1; });

		CPPUNIT_ASSERT(i0 != proc->basic_blocks.end());
		CPPUNIT_ASSERT(i1 != proc->basic_blocks.end());

		po::bblock_cptr bbo0 = *i0;
		po::bblock_cptr bbo1 = *i1;

		CPPUNIT_ASSERT(proc->entry == bbo0);
		CPPUNIT_ASSERT(bbo0->mnemonics().size() == 1);
		check(bbo0->mnemonics()[0],"test0",0);
		CPPUNIT_ASSERT(bbo1->mnemonics().size() == 2);
	}

	void testVariable(void)
	{
		CPPUNIT_ASSERT(false);
	}
};
