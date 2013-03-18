#include <iostream>
#include <algorithm>
#include <iterator>

#include <cppunit/extensions/HelperMacros.h>
#include <disassembler.hh>

using namespace po;
using namespace std;

struct test_tag {};
unsigned int ununsed = 0;
vector<string> regs({"a","b","c","d"});

template<>
struct architecture_traits<test_tag>
{
	typedef char token_type;
};

namespace po
{
	template<>
	lvalue temporary(test_tag)
	{
		return variable("t" + to_string(ununsed++),16);
	}

	template<>
	const vector<string> &registers(test_tag)
	{
		return regs;
	}

	template<>
	uint8_t width(string n, test_tag)
	{
		return 8;
	}
};

class CodeGeneratorTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(CodeGeneratorTest);
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
	typedef sem_state<test_tag>& ss;
	typedef code_generator<test_tag>& cg;

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

	void tearDown(void)
	{
		return;
	}

	void testSingle(void)
	{
		sem_state<test_tag> st(0);
		vector<char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(bytes.begin(),bytes.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(bytes.begin()));
		CPPUNIT_ASSERT(st.address == 0);
		CPPUNIT_ASSERT(st.tokens.size() >= 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'A');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == string("A"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == range<addr_t>(0,1));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().value() == 1);
		CPPUNIT_ASSERT(st.jumps.front().second->relations.empty());
	}

	void testSub(void)
	{
		sem_state<test_tag> st(1);
		vector<char>::iterator i;
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
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == string("BA"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == range<addr_t>(1,3));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().value() == 3);
		CPPUNIT_ASSERT(st.jumps.front().second->relations.empty());
	}

	void testDefault(void)
	{
		sem_state<test_tag> st(5);
		vector<char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(next(bytes.begin(),5),bytes.end(),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == bytes.end());
		CPPUNIT_ASSERT(st.address == 5);
		CPPUNIT_ASSERT(st.tokens.size() == 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'X');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == string("UNK"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == range<addr_t>(5,6));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().value() == 6);
		CPPUNIT_ASSERT(st.jumps.front().second->relations.empty());
	}

	void testSlice(void)
	{
		sem_state<test_tag> st(1);
		vector<char>::iterator i;
		bool ret;

		tie(ret,i) = main.match(next(bytes.begin()),next(bytes.begin(),2),st);

		CPPUNIT_ASSERT(ret);
		CPPUNIT_ASSERT(i == next(bytes.begin(),2));
		CPPUNIT_ASSERT(st.address == 1);
		CPPUNIT_ASSERT(st.tokens.size() >= 1);
		CPPUNIT_ASSERT(st.tokens[0] == 'A');
		CPPUNIT_ASSERT(st.capture_groups.size() == 0);
		CPPUNIT_ASSERT(st.mnemonics.size() == 1);	
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == string("A"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == range<addr_t>(1,2));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().value() == 2);
		CPPUNIT_ASSERT(st.jumps.front().second->relations.empty());
	}

	void testEmpty(void)
	{
		sem_state<test_tag> st(0);
		vector<char>::iterator i;
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
		sem_state<test_tag> st(4);
		vector<char>::iterator i;
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
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == string("C"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == range<addr_t>(4,5));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 1);
		CPPUNIT_ASSERT(st.jumps.front().first.is_constant());
		CPPUNIT_ASSERT(st.jumps.front().first.constant().value() == 5);
		CPPUNIT_ASSERT(st.jumps.front().second->relations.empty());
	}
	
	void testEmptyCapGroup(void)
	{
		sem_state<test_tag> st(0);
		vector<char> buf({127});
		vector<char>::iterator i;
		bool ret;
		disassembler<test_tag> dec;

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
		CPPUNIT_ASSERT(st.mnemonics.front().opcode == string("1"));	
		CPPUNIT_ASSERT(st.mnemonics.front().area == range<addr_t>(0,1));	
		CPPUNIT_ASSERT(st.mnemonics.front().instructions.empty());	
		CPPUNIT_ASSERT(st.jumps.size() == 0);
	}

	void testTooLongCapGroup(void)
	{
		sem_state<test_tag> st(0);
		vector<char> buf({127});
		disassembler<test_tag> dec;

		dec | "k@........." = [](ss s) {};

		CPPUNIT_ASSERT(!dec.match(buf.begin(),buf.end(),st).first);
	}

	void testTooLongTokPat(void)
	{
		sem_state<test_tag> st(0);
		vector<char> buf({127});
		disassembler<test_tag> dec;

		dec | "111111111" = [](ss s) {};

		CPPUNIT_ASSERT(dec.match(buf.begin(),buf.end(),st).first);
	}

	void testTooShortTokPat(void)
	{
		sem_state<test_tag> st(0);
		vector<char> buf({127});
		disassembler<test_tag> dec;

		dec | "1111111" = [](ss s) {};

		CPPUNIT_ASSERT(dec.match(buf.begin(),buf.end(),st).first);
	}

	void testInvalidTokPat(void)
	{
		sem_state<test_tag> st(0);
		vector<char> buf({127});
		disassembler<test_tag> dec;

		dec | "a111111" = [](ss s) {};

		CPPUNIT_ASSERT(dec.match(buf.begin(),buf.end(),st).first);
	}

private:
	disassembler<test_tag> main, sub;
	vector<char> bytes;
};
