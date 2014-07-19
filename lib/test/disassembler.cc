#include <iostream>
#include <algorithm>
#include <iterator>

#include <gtest/gtest.h>
#include <panopticon/architecture.hh>
#include "architecture.hh"
#include <panopticon/disassembler.hh>

class disassembler : public ::testing::Test
{
public:
	typedef po::sem_state<test_tag>& ss;
	typedef po::code_generator<test_tag>& cg;

protected:
	void SetUp(void)
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

		_bytes = {'A','A','B','A','C','X'};
		bytes = po::slab(_bytes.begin(),_bytes.end());
	}

	po::disassembler<test_tag> main, sub;
	std::vector<unsigned char> _bytes;
	po::slab bytes;
};

TEST_F(disassembler,single_decoder)
{
	po::sem_state<test_tag> st(0);
	boost::optional<po::slab::const_iterator> i = main.match(boost::begin(bytes),boost::end(bytes),st);

	ASSERT_TRUE(i);
	ASSERT_EQ(i, next(bytes.begin()));
	ASSERT_EQ(st.address, 0);
	ASSERT_GE(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,1));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 1);
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
}

TEST_F(disassembler,sub_decoder)
{
	po::sem_state<test_tag> st(1);
	boost::optional<po::slab::const_iterator> i = main.match(boost::begin(bytes)+1,boost::end(bytes),st);

	ASSERT_TRUE(i);
	ASSERT_EQ(std::distance(bytes.begin(), *i), 3);
	ASSERT_EQ(st.address, 1);
	ASSERT_GE(st.tokens.size(), 2);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.tokens[1], 'B');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("BA"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(1,3));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 3);
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
}

TEST_F(disassembler,default_pattern)
{
	po::sem_state<test_tag> st(5);
	boost::optional<po::slab::const_iterator> i = main.match(boost::begin(bytes)+5,boost::end(bytes),st);

	ASSERT_TRUE(i);
	ASSERT_EQ(*i, bytes.end());
	ASSERT_EQ(st.address, 5);
	ASSERT_EQ(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'X');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("UNK"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(5,6));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 6);
}

TEST_F(disassembler,slice)
{
	po::sem_state<test_tag> st(1);
	boost::optional<po::slab::const_iterator> i = main.match(boost::begin(bytes)+1,boost::begin(bytes)+2,st);

	ASSERT_TRUE(i);
	ASSERT_EQ(*i, next(bytes.begin(),2));
	ASSERT_EQ(st.address, 1);
	ASSERT_GE(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(1,2));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 2);
}

TEST_F(disassembler,empty)
{
	po::sem_state<test_tag> st(0);
	boost::optional<po::slab::const_iterator> i = main.match(boost::begin(bytes),boost::begin(bytes),st);

	ASSERT_TRUE(!i);
	ASSERT_EQ(st.address, 0);
	ASSERT_EQ(st.tokens.size(), 0);
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 0);
	ASSERT_EQ(st.jumps.size(), 0);
}

TEST_F(disassembler,capture_group)
{
	po::sem_state<test_tag> st(4);
	boost::optional<po::slab::const_iterator> i = main.match(boost::begin(bytes)+4,boost::end(bytes),st);

	ASSERT_TRUE(i);
	ASSERT_EQ(*i, next(bytes.begin(),5));
	ASSERT_EQ(st.address, 4);
	ASSERT_GE(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'C');
	ASSERT_EQ(st.capture_groups.size(), 1);
	ASSERT_EQ(st.capture_groups.count("k"), 1);
	ASSERT_EQ(st.capture_groups["k"], 16);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("C"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(4,5));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 5);
}

TEST_F(disassembler,empty_capture_group)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> _buf = {127};
	po::slab buf(_buf.begin(),_buf.end());
	po::disassembler<test_tag> dec;

	dec | "01 a@.. 1 b@ c@..." = [](ss s) { s.mnemonic(1,"1"); };
	boost::optional<po::slab::const_iterator> i = dec.match(boost::begin(buf),boost::end(buf),st);

	ASSERT_TRUE(i);
	ASSERT_EQ(std::distance(boost::begin(buf), *i),1);
	ASSERT_EQ(st.address, 0);
	ASSERT_EQ(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 127);
	ASSERT_EQ(st.capture_groups.size(), 3);
	ASSERT_EQ(st.capture_groups.count("a"), 1);
	ASSERT_EQ(st.capture_groups.count("b"), 1);
	ASSERT_EQ(st.capture_groups.count("c"), 1);
	ASSERT_EQ(st.capture_groups["a"], 3);
	ASSERT_EQ(st.capture_groups["b"], 0);
	ASSERT_EQ(st.capture_groups["c"], 7);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("1"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,1));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 0);
}

TEST_F(disassembler,too_long_capture_group)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf = {127};
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec | "k@........." = [](ss s) {};,po::tokpat_error);
}

TEST_F(disassembler,too_long_token_pattern)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf = {127};
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec | "111111111" = [](ss s) {};,po::tokpat_error);
}

TEST_F(disassembler,too_short_token_pattern)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> _buf = {127};
	po::slab buf(_buf.begin(),_buf.end());
	po::disassembler<test_tag> dec;

	dec | "1111111" = [](ss s) {};

	ASSERT_TRUE(dec.match(boost::begin(buf),boost::end(buf),st));
}

TEST_F(disassembler,invalid_token_pattern)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf = {127};
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec | "a111111" = [](ss s) {};,po::tokpat_error);
}

using sw = po::sem_state<wtest_tag>&;

TEST_F(disassembler,wide_token)
{
	po::sem_state<wtest_tag> st(0);
	std::vector<uint8_t> _buf = {0x11,0x22, 0x33,0x44, 0x55,0x44};
	po::slab buf(_buf.begin(),_buf.end());
	po::disassembler<wtest_tag> dec;

	dec | 0x1122 = [](sw s)
	{
		s.mnemonic(2,"A");
		s.jump(s.address + 2);
	};

	dec | 0x3344 = [](sw s)
	{
		s.mnemonic(2,"B");
		s.jump(s.address + 2);
		s.jump(s.address + 4);
	};

	dec | 0x5544 = [](sw s)
	{
		s.mnemonic(2,"C");
	};

	boost::optional<po::slab::const_iterator> i = dec.match(boost::begin(buf),boost::end(buf),st);

	ASSERT_TRUE(i);
	ASSERT_EQ(std::distance(boost::begin(buf), *i),1);
	ASSERT_EQ(st.address, 0);
	ASSERT_EQ(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 0x1122);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,2));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
}
