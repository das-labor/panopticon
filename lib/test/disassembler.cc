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
		sub['B'] = [](ss st)
		{
			st.mnemonic(2,"BA");
			st.jump(st.address + 2);
		};

		main['A' >> sub];

		main['A'] = [](ss st)
		{
			st.mnemonic(1,"A");
			st.jump(st.address + 1);
		};

		main["0 k@..... 11"] = [](ss st)
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
		bytes = po::slab(_bytes.data(),_bytes.size());
	}

	po::disassembler<test_tag> main, sub;
	std::vector<unsigned char> _bytes;
	po::slab bytes;
};

TEST_F(disassembler,single_decoder)
{
	po::sem_state<test_tag> st(0);
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = main.try_match(bytes.begin(),bytes.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(i->first, next(bytes.begin()));
	ASSERT_EQ(st.address, 0u);
	ASSERT_GE(st.tokens.size(), 1u);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,1));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1u);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 1u);
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
}

TEST_F(disassembler,sub_decoder)
{
	po::sem_state<test_tag> st(1);
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = main.try_match(bytes.begin()+1,bytes.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(std::distance(bytes.begin(), i->first), 3);
	ASSERT_EQ(st.address, 1u);
	ASSERT_GE(st.tokens.size(), 2u);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.tokens[1], 'B');
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("BA"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(1,3));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1u);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 3u);
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
}

TEST_F(disassembler,default_pattern)
{
	po::sem_state<test_tag> st(5);
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = main.try_match(bytes.begin()+5,bytes.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(i->first, bytes.end());
	ASSERT_EQ(st.address, 5u);
	ASSERT_EQ(st.tokens.size(), 1u);
	ASSERT_EQ(st.tokens[0], 'X');
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("UNK"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(5,6));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1u);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 6u);
}

TEST_F(disassembler,slice)
{
	po::sem_state<test_tag> st(1);
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = main.try_match(bytes.begin()+1,bytes.begin()+2,st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(i->first, next(bytes.begin(),2));
	ASSERT_EQ(st.address, 1u);
	ASSERT_GE(st.tokens.size(), 1u);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(1,2));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1u);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 2u);
}

TEST_F(disassembler,empty)
{
	po::sem_state<test_tag> st(0);
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = main.try_match(bytes.begin(),bytes.begin(),st);

	ASSERT_TRUE(!i);
	ASSERT_EQ(st.address, 0u);
	ASSERT_EQ(st.tokens.size(), 0u);
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 0u);
	ASSERT_EQ(st.jumps.size(), 0u);
}

TEST_F(disassembler,capture_group)
{
	po::sem_state<test_tag> st(4);
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = main.try_match(bytes.begin()+4,bytes.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(i->first, next(bytes.begin(),5));
	ASSERT_EQ(st.address, 4u);
	ASSERT_GE(st.tokens.size(), 1u);
	ASSERT_EQ(st.tokens[0], 'C');
	ASSERT_EQ(st.capture_groups.size(), 1u);
	ASSERT_EQ(st.capture_groups.count("k"), 1u);
	ASSERT_EQ(st.capture_groups["k"], 16u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("C"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(4,5));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1u);
	ASSERT_TRUE(is_constant(st.jumps.front().first));
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(to_constant(st.jumps.front().first).content(), 5u);
}

TEST_F(disassembler,empty_capture_group)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> _buf = {127};
	po::slab buf(_buf.data(),_buf.size());
	po::disassembler<test_tag> dec;

	dec["01 a@.. 1 b@ c@..."] = [](ss s) { s.mnemonic(1,"1"); };
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = dec.try_match(buf.begin(),buf.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(std::distance(buf.begin(), i->first),1);
	ASSERT_EQ(st.address, 0u);
	ASSERT_EQ(st.tokens.size(), 1u);
	ASSERT_EQ(st.tokens[0], 127);
	ASSERT_EQ(st.capture_groups.size(), 2u);
	ASSERT_EQ(st.capture_groups.count("a"), 1u);
	ASSERT_EQ(st.capture_groups.count("c"), 1u);
	ASSERT_EQ(st.capture_groups["a"], 3u);
	ASSERT_EQ(st.capture_groups["c"], 7u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("1"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,1));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 0u);
}

TEST_F(disassembler,too_long_capture_group)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf = {127};
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec["k@........."],po::tokpat_error);
}

TEST_F(disassembler,too_long_token_pattern)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf = {127};
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec["111111111"],po::tokpat_error);
}

TEST_F(disassembler,too_short_token_pattern)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> _buf = {127};
	po::slab buf(_buf.data(),_buf.size());
	po::disassembler<test_tag> dec;

	dec["1111111"];

	ASSERT_TRUE(dec.try_match(buf.begin(),buf.end(),st));
}

TEST_F(disassembler,invalid_token_pattern)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf = {127};
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec["a111111"];,po::tokpat_error);
}

using sw = po::sem_state<wtest_tag>&;

TEST_F(disassembler,wide_token)
{
	po::sem_state<wtest_tag> st(0);
	std::vector<uint8_t> _buf = {0x22,0x11, 0x44,0x33, 0x44,0x55};
	po::slab buf(_buf.data(),_buf.size());
	po::disassembler<wtest_tag> dec;

	dec[0x1122] = [](sw s)
	{
		s.mnemonic(2,"A");
		s.jump(s.address + 2);
	};

	dec[0x3344] = [](sw s)
	{
		s.mnemonic(2,"B");
		s.jump(s.address + 2);
		s.jump(s.address + 4);
	};

	dec[0x5544] = [](sw s)
	{
		s.mnemonic(2,"C");
	};

	boost::optional<std::pair<po::slab::iterator,po::sem_state<wtest_tag>>> i;

	i = dec.try_match(buf.begin(),buf.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(std::distance(buf.begin(), i->first),2);
	ASSERT_EQ(st.address, 0u);
	ASSERT_EQ(st.tokens.size(), 1u);
	ASSERT_EQ(st.tokens[0], 0x1122u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,2));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1u);
}

TEST_F(disassembler,optional)
{
	using po::operator "" _e;

	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> _buf = {127,126,125,127,125};
	po::slab buf(_buf.data(),_buf.size());
	po::disassembler<test_tag> dec;

	dec[127_e >> *126_e >> 125_e] = [](ss s) { s.mnemonic(s.tokens.size(),"1"); };
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = dec.try_match(buf.begin(),buf.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(std::distance(buf.begin(), i->first),3);
	ASSERT_EQ(st.address, 0u);
	ASSERT_EQ(st.tokens.size(), 3u);
	ASSERT_EQ(st.tokens[0], 127u);
	ASSERT_EQ(st.tokens[1], 126u);
	ASSERT_EQ(st.tokens[2], 125u);
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("1"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,3));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 0u);

	st = po::sem_state<test_tag>(3);
	i = dec.try_match(i->first,buf.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(std::distance(buf.begin(), i->first),5);
	ASSERT_EQ(st.address, 3u);
	ASSERT_EQ(st.tokens.size(), 2u);
	ASSERT_EQ(st.tokens[0], 127u);
	ASSERT_EQ(st.tokens[1], 125u);
	ASSERT_EQ(st.capture_groups.size(), 0u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("1"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(3,5));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 0u);
}

TEST_F(disassembler,fixed_capture_group_contents)
{
	using po::operator "" _e;

	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> _buf = {127,255};
	po::slab buf(_buf.data(),_buf.size());
	po::disassembler<test_tag> dec;

	dec[ "01111111"_e >> "a@11111111"_e ] = [](ss s) { s.mnemonic(1,"1"); };
	boost::optional<std::pair<po::slab::iterator,po::sem_state<test_tag>>> i;

	i = dec.try_match(buf.begin(),buf.end(),st);
	ASSERT_TRUE(i);
	st = i->second;

	ASSERT_EQ(std::distance(buf.begin(), i->first),2);
	ASSERT_EQ(st.address, 0u);
	ASSERT_EQ(st.tokens.size(), 2u);
	ASSERT_EQ(st.tokens[0], 127u);
	ASSERT_EQ(st.tokens[1], 255u);
	ASSERT_EQ(st.capture_groups.size(), 1u);
	ASSERT_EQ(st.capture_groups.count("a"), 1u);
	ASSERT_EQ(st.capture_groups["a"], 255u);
	ASSERT_EQ(st.mnemonics.size(), 1u);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("1"));
	ASSERT_EQ(st.mnemonics.front().area, po::bound(0,1));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 0u);

}
