#include <iostream>
#include <algorithm>
#include <iterator>

#include <cppunit/extensions/HelperMacros.h>

#include <disassembler.hh>

#include "test_architecture.hh"

class DisassemblerTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(DisassemblerTest);
	CPPUNIT_TEST(testSingle);
	CPPUNIT_TEST(testSub);
	CPPUNIT_TEST(testSlice);
	CPPUNIT_TEST(testDefault);
	CPPUNIT_TEST(testEmpty);
	CPPUNIT_TEST(testCapGroup);
	CPPUNIT_TEST(testEmptyCapGroup);
	CPPUNIT_TEST(testTooLongTokPat);
	CPPUNIT_TEST(testTooLongCapGroup);
	CPPUNIT_TEST(testTooShortTokPat);
	CPPUNIT_TEST(testInvalidTokPat);
	CPPUNIT_TEST_SUITE_END();

public:
	typedef po::sem_state<test_tag>& ss;
	typedef po::code_generator<test_tag>& cg;

	void setUp(void)
	{
		sub | 'B' = [](ss st)
		{
			st.mnemonic(2,"BA");
			st.jump(st.address + 2);
		};

		main | 'A' | sub = [](ss st)
		{
			;
		};
		
		main | 'A' = [](ss st)
		{
			st.mnemonic(1,"A");
			st.jump(st.address + 1);
		};

		main | "0 k@..... 11" = [](ss st)
		{
			st.mnemonic(1,"C");
			st.jump(st.address + 1);
		};

		main = [](ss st)
		{
			st.mnemonic(1,"UNK");
			st.jump(st.address + 1);
		};

		bytes = {'A','A','B','A','C','X'};
	}

	void testSingle(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(bytes.begin(),bytes.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(bytes.begin()));
		CPPUNIT_ASSERT(st.address == 0);
		CPPUNIT_ASSERT(st.tokens.size() >= 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'A');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == std::string("A"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == po::range<po::addr_t>(0,1));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().content() == 1);
		CPPUNIT_ASSERT(st.jumps.front().second.relations.empty());
	}

	void testSub(void)
	{
		po::sem_state<test_tag> st(1);
		std::vector<unsigned char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(next(bytes.begin()),bytes.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(bytes.begin(),3));
		CPPUNIT_ASSERT(st.address == 1);
		CPPUNIT_ASSERT(st.tokens.size() >= 2);
		CPPUNIT_ASSERT(st.tokens[0] == 'A');
		CPPUNIT_ASSERT(st.tokens[1] == 'B');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == std::string("BA"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == po::range<po::addr_t>(1,3));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().content() == 3);
		CPPUNIT_ASSERT(st.jumps.front().second.relations.empty());
	}

	void testDefault(void)
	{
		po::sem_state<test_tag> st(5);
		std::vector<unsigned char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(next(bytes.begin(),5),bytes.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == bytes.end());
		CPPUNIT_ASSERT(st.address == 5);
		CPPUNIT_ASSERT(st.tokens.size() == 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'X');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == std::string("UNK"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == po::range<po::addr_t>(5,6));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().second.relations.empty());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().content() == 6);
	}

	void testSlice(void)
	{
		po::sem_state<test_tag> st(1);
		std::vector<unsigned char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(next(bytes.begin()),next(bytes.begin(),2),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(bytes.begin(),2));
		CPPUNIT_ASSERT(st.address == 1);
		CPPUNIT_ASSERT(st.tokens.size() >= 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'A');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == std::string("A"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == po::range<po::addr_t>(1,2));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().second.relations.empty());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().content() == 2);
	}

	void testEmpty(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(bytes.begin(),bytes.begin(),st);

		CPPUNIT_ASSERT(!ret);
		CPPUNIT_ASSERT(i == bytes.begin());
		CPPUNIT_ASSERT(st.address == 0);
		CPPUNIT_ASSERT(st.tokens.size() == 0);
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 0);	
		CPPUNIT_ASSERT(st.jumps.size() == 0);
	}

	void testCapGroup(void)
	{
		po::sem_state<test_tag> st(4);
		std::vector<unsigned char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(next(bytes.begin(),4),bytes.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(bytes.begin(),5));
		CPPUNIT_ASSERT(st.address == 4);
		CPPUNIT_ASSERT(st.tokens.size() >= 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'C');
		CPPUNIT_ASSERT(st.capture_groups.size() == 1);
		CPPUNIT_ASSERT(st.capture_groups.count("k") == 1);
		CPPUNIT_ASSERT(st.capture_groups["k"] == 16);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == std::string("C"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == po::range<po::addr_t>(4,5));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().second.relations.empty());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().content() == 5);
	}
	
	void testEmptyCapGroup(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char> buf({127});
		std::vector<unsigned char>::iterator i;
		bool ret;
		po::disassembler<test_tag> dec;

		dec | "01 a@.. 1 b@ c@..." = [](ss s) { s.mnemonic(1,"1"); };

		tie(ret,i) = dec.match(buf.begin(),buf.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(buf.begin(),1));
		CPPUNIT_ASSERT(st.address == 0);
		CPPUNIT_ASSERT(st.tokens.size() == 1);
		CPPUNIT_ASSERT(st.tokens[0] == 127);
		CPPUNIT_ASSERT(st.capture_groups.size() == 3);
		CPPUNIT_ASSERT(st.capture_groups.count("a") == 1);
		CPPUNIT_ASSERT(st.capture_groups.count("b") == 1);
		CPPUNIT_ASSERT(st.capture_groups.count("c") == 1);
		CPPUNIT_ASSERT(st.capture_groups["a"] == 3);
		CPPUNIT_ASSERT(st.capture_groups["b"] == 0);
		CPPUNIT_ASSERT(st.capture_groups["c"] == 7);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == std::string("1"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == po::range<po::addr_t>(0,1));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 0);
	}

	void testTooLongCapGroup(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char> buf({127});
		po::disassembler<test_tag> dec;

		CPPUNIT_ASSERT_THROW(dec | "k@........." = [](ss s) {};,po::tokpat_error);
	}

	void testTooLongTokPat(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char> buf({127});
		po::disassembler<test_tag> dec;

		CPPUNIT_ASSERT_THROW(dec | "111111111" = [](ss s) {};,po::tokpat_error);
	}

	void testTooShortTokPat(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char> buf({127});
		po::disassembler<test_tag> dec;

		dec | "1111111" = [](ss s) {};

		CPPUNIT_ASSERT(dec.match(buf.begin(),buf.end(),st).first);
	}

	void testInvalidTokPat(void)
	{
		po::sem_state<test_tag> st(0);
		std::vector<unsigned char> buf({127});
		po::disassembler<test_tag> dec;

		CPPUNIT_ASSERT_THROW(dec | "a111111" = [](ss s) {};,po::tokpat_error);
	}

private:
	po::disassembler<test_tag> main, sub;
	std::vector<unsigned char> bytes;
};
