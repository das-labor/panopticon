#include <iostream>
#include <algorithm>
#include <iterator>

#include <panopticon/disassembler.hh>

#include "test/architecture.hh"

class DisassemblerTest : public ::testing::Test
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

		bytes = {'A','A','B','A','C','X'};
	}

private:
	po::disassembler<test_tag> main, sub;
	std::vector<unsigned char> bytes;
};

TEST_F(DisassemblerTest,single_decoder)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char>::iterator i;
	bool ret;

	tie(ret,i) = main.match(bytes.begin(),bytes.end(),st);

	ASSERT_TRUE(ret);
	ASSERT_EQ(i, next(bytes.begin()));
	ASSERT_EQ(st.address, 0);
	ASSERT_GE(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcod, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::range<po::addr_t>(0,1));
	ASSERT_EQ(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(st.jumps.front().first.is_constant());
	ASSERT_EQ(st.jumps.front().first.to_constant().content(), 1);
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
}

TEST_F(DisassemblerTest,sub_decoder)
{
	po::sem_state<test_tag> st(1);
	std::vector<unsigned char>::iterator i;
	bool ret;

	tie(ret,i) = main.match(next(bytes.begin()),bytes.end(),st);

	ASSERT_TRUE(ret);
	ASSERT_EQ(i, next(bytes.begin(),3));
	ASSERT_EQ(st.address, 1);
	ASSERT_GE(st.tokens.size(), 2);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.tokens[1], 'B');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("BA"));
	ASSERT_EQ(st.mnemonics.front().area, po::range<po::addr_t>(1,3));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_EQ(st.jumps.front().first.is_constant());
	ASSERT_EQ(st.jumps.front().first.to_constant().content(), 3);
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
}

TEST_F(DisassemblerTest,Default)
{
	po::sem_state<test_tag> st(5);
	std::vector<unsigned char>::iterator i;
	bool ret;

	tie(ret,i) = main.match(next(bytes.begin(),5),bytes.end(),st);

	ASSERT_TRUE(ret);
	ASSERT_EQ(i, bytes.end());
	ASSERT_EQ(st.address, 5);
	ASSERT_EQ(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'X');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("UNK"));
	ASSERT_EQ(st.mnemonics.front().area, po::range<po::addr_t>(5,6));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(st.jumps.front().first.is_constant());
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(st.jumps.front().first.to_constant().content(), 6);
}

TEST_F(DisassemblerTest,Slice)
{
	po::sem_state<test_tag> st(1);
	std::vector<unsigned char>::iterator i;
	bool ret;

	tie(ret,i) = main.match(next(bytes.begin()),next(bytes.begin(),2),st);

	ASSERT_TRUE(ret);
	ASSERT_EQ(i, next(bytes.begin(),2));
	ASSERT_EQ(st.address, 1);
	ASSERT_GE(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'A');
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("A"));
	ASSERT_EQ(st.mnemonics.front().area, po::range<po::addr_t>(1,2));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(st.jumps.front().first.is_constant());
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(st.jumps.front().first.to_constant().content(), 2);
}

TEST_F(DisassemblerTest,Empty)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char>::iterator i;
	bool ret;

	tie(ret,i) = main.match(bytes.begin(),bytes.begin(),st);

	ASSERT_TRUE(!ret);
	ASSERT_EQ(i, bytes.begin());
	ASSERT_EQ(st.address, 0);
	ASSERT_EQ(st.tokens.size(), 0);
	ASSERT_EQ(st.capture_groups.size(), 0);
	ASSERT_EQ(st.mnemonics.size(), 0);
	ASSERT_EQ(st.jumps.size(), 0);
}

TEST_F(DisassemblerTest,CapGroup)
{
	po::sem_state<test_tag> st(4);
	std::vector<unsigned char>::iterator i;
	bool ret;

	tie(ret,i) = main.match(next(bytes.begin(),4),bytes.end(),st);

	ASSERT_TRUE(ret);
	ASSERT_EQ(i, next(bytes.begin(),5));
	ASSERT_EQ(st.address, 4);
	ASSERT_GE(st.tokens.size(), 1);
	ASSERT_EQ(st.tokens[0], 'C');
	ASSERT_EQ(st.capture_groups.size(), 1);
	ASSERT_EQ(st.capture_groups.count("k"), 1);
	ASSERT_EQ(st.capture_groups["k"], 16);
	ASSERT_EQ(st.mnemonics.size(), 1);
	ASSERT_EQ(st.mnemonics.front().opcode, std::string("C"));
	ASSERT_EQ(st.mnemonics.front().area, po::range<po::addr_t>(4,5));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 1);
	ASSERT_TRUE(st.jumps.front().first.is_constant());
	ASSERT_TRUE(st.jumps.front().second.relations.empty());
	ASSERT_EQ(st.jumps.front().first.to_constant().content(), 5);
}

TEST_F(DisassemblerTest,EmptyCapGroup)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf({127});
	std::vector<unsigned char>::iterator i;
	bool ret;
	po::disassembler<test_tag> dec;

	dec | "01 a@.. 1 b@ c@..." = [](ss s) { s.mnemonic(1,"1"); };

	tie(ret,i) = dec.match(buf.begin(),buf.end(),st);

	ASSERT_TRUE(ret);
	ASSERT_EQ(i, next(buf.begin(),1));
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
	ASSERT_EQ(st.mnemonics.front().area, po::range<po::addr_t>(0,1));
	ASSERT_TRUE(st.mnemonics.front().instructions.empty());
	ASSERT_EQ(st.jumps.size(), 0);
}

TEST_F(DisassemblerTest,TooLongCapGroup)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf({127});
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec | "k@........." = [](ss s) {};,po::tokpat_error);
}

TEST_F(DisassemblerTest,TooLongTokPat)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf({127});
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec | "111111111" = [](ss s) {};,po::tokpat_error);
}

TEST_F(DisassemblerTest,TooShortTokPat)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf({127});
	po::disassembler<test_tag> dec;

	dec | "1111111" = [](ss s) {};

	ASSERT_TRUE(dec.match(buf.begin(),buf.end(),st).first);
}

TEST_F(DisassemblerTest,InvalidTokPat)
{
	po::sem_state<test_tag> st(0);
	std::vector<unsigned char> buf({127});
	po::disassembler<test_tag> dec;

	ASSERT_THROW(dec | "a111111" = [](ss s) {};,po::tokpat_error);
}
