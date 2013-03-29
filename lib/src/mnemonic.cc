#include <sstream>
#include <algorithm>
#include <functional>

#include <mnemonic.hh>

using namespace po;
using namespace std;

const addr_t po::naddr = -1;

ostream &po::operator<<(ostream &os, const instr &i)
{
	os << i.left << " ≔ ";
	if(i.right.size() == 0)
		os << i.fnname;
	else if(i.function == instr::Call)
		os << i.fnname << "(" << i.right[0] << ")";
	else if(i.right.size() == 1)
		os << i.fnname << i.right[0];
	else if(i.function == instr::Phi)
		os << i.fnname << "(" << i.right[0] << ", " << i.right[1] << ")";
	else if(i.function == instr::Slice)
		os << i.right[0] << "[" << i.right[1] << i.fnname << i.right[2] << "]";
	else if(i.right.size() == 3)
		os << i.fnname << "(" << i.right[0] << ", " << i.right[1] << ", " << i.right[2] << ")";
	else
		os << i.right[0] << i.fnname << i.right[1];
	return os;
}

mnemonic::mnemonic(const range<addr_t> &a, const string &n, const string &fmt, initializer_list<rvalue> ops, initializer_list<instr> instrs)
: area(a), opcode(n), operands(ops), instructions(instrs)
{
	function<string::const_iterator(string::const_iterator,string::const_iterator)> plain_or_meta;
	function<string::const_iterator(string::const_iterator,string::const_iterator,token &)> escape_seq, modifiers, alias;
	function<string::const_iterator(string::const_iterator,string::const_iterator,unsigned int &)> digits;

	// FormatString -> ('{' EscapeSequence '}') | PlainAscii
	plain_or_meta = [&](string::const_iterator cur, string::const_iterator end)
	{
		if(cur == end)
			return cur;
		else if(*cur == '{')
		{
			token tok;
			cur = escape_seq(next(cur),end,tok);
			assert(cur != end && *cur == '}');
			format.push_back(tok);

			return plain_or_meta(next(cur),end);
		}
		else
		{
			if(format.empty() || !format.back().is_literal)
			{
				token tok;
				tok.is_literal = true;
				tok.alias = string(1,*cur);
				format.push_back(tok);
			}
			else
				format.back().alias += string(1,*cur);
			return plain_or_meta(next(cur),end);
		}
	};

	// EscapeSequence -> Digit+ (':' Modifiers (':' Alias)?)?
	escape_seq = [&](string::const_iterator cur, string::const_iterator end,token &tok)
	{
		assert(cur != end && isdigit(*cur));
		cur = digits(cur,end,tok.width);
	
		tok.is_literal = false;
		tok.alias = "";
		tok.has_sign = false;

		if(cur != end && *cur == ':')
		{
			cur = modifiers(next(cur),end,tok);
			if(cur != end && *cur == ':')
				return alias(next(cur),end,tok);
		}

		return cur;
	};

	// Modifers -> '-'?
	modifiers = [&](string::const_iterator cur, string::const_iterator end,token &tok)
	{
		assert(cur != end);

		if(*cur == '-')
		{
			tok.has_sign = true;
			return next(cur);
		}

		return cur;
	};

	// Alias -> PlainAscii*
	alias = [&](string::const_iterator cur,string::const_iterator end,token &tok)
	{
		assert(cur != end);

		if(*cur != '}' && *cur != ':')
		{
			tok.alias += string(1,*cur);
			return alias(next(cur),end,tok);
		}

		return cur;
	};
	
	// Digit
	digits = [&](string::const_iterator cur,string::const_iterator end,unsigned int &i)
	{
		if(cur != end && isdigit(*cur))
		{
			i = i * 10 + *cur - '0';
			return digits(next(cur),end,i);
		}
		else
			return cur;
	};
	
	plain_or_meta(fmt.cbegin(),fmt.cend());
}

mnemonic::mnemonic(const mnemonic &m)
: area(m.area), 
	opcode(m.opcode), 
	operands(m.operands), 
	instructions(m.instructions), 
	format(m.format)
{}

mnemonic::mnemonic(mnemonic &&m)
: area(move(m.area)),
	opcode(move(m.opcode)),
	operands(move(m.operands)),
	instructions(move(m.instructions)),
	format(move(m.format))
{}

mnemonic &mnemonic::operator=(const mnemonic &m)
{
	if(&m != this)
	{
		area = m.area;
		opcode = m.opcode;
		operands = m.operands;
		instructions = m.instructions;
		format = m.format;
	}

	return *this;
}

mnemonic &mnemonic::operator=(mnemonic &&m)
{
	area = move(m.area);
	opcode = move(m.opcode);
	operands = move(m.operands);
	instructions = move(m.instructions);
	format = move(m.format);

	return *this;
}

ostream &po::operator<<(ostream &os, const mnemonic &m)
{
	os << m.opcode;

	if(m.operands.size())
		os << " ";

	unsigned int idx = 0;
	for(const mnemonic::token &tok: m.format)
	{
		if(tok.alias.empty() && !tok.is_literal)
		{
			assert(idx < m.operands.size());
			if(m.operands[idx].is_constant())
			{
				if(tok.has_sign)
					os << (int)m.operands[idx].constant().value();
				else
					os << m.operands[idx].constant().value();
			}
			else
				os << m.operands[idx];
		}
		else
			os << tok.alias;
		idx += !tok.is_literal;
	}
	
	return os;
}

int64_t po::format_constant(const mnemonic::token &tok, uint64_t v)
{
	assert(tok.width <= 64);
	uint64_t bitmask = 0;
	bitmask = (~bitmask) >> (64 - tok.width);

	if(tok.has_sign)
		return (int64_t)((v & (bitmask >> 1)) & ((v & (1 << (tok.width - 1))) << (64 - tok.width)));
	else
		return v & bitmask;
}
