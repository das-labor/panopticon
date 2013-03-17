#include <sstream>
#include <algorithm>
#include <functional>

#include <mnemonic.hh>

using namespace po;

const addr_t po::naddr = -1;

std::ostream &po::operator<<(std::ostream &os, const instr &i)
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

mnemonic::mnemonic(range<addr_t> a, std::string n, std::string fmt, std::initializer_list<rvalue> ops, std::initializer_list<instr> instrs)
: area(a), opcode(n), operands(ops), instructions(instrs)
{
	std::function<std::string::const_iterator(std::string::const_iterator,std::string::const_iterator)> plain_or_meta;
	std::function<std::string::const_iterator(std::string::const_iterator,std::string::const_iterator,token &)> escape_seq, modifiers, alias;
	std::function<std::string::const_iterator(std::string::const_iterator,std::string::const_iterator,unsigned int &)> digits;

	// FormatString -> ('{' EscapeSequence '}') | PlainAscii
	plain_or_meta = [&](std::string::const_iterator cur, std::string::const_iterator end)
	{
		if(cur == end)
			return cur;
		else if(*cur == '{')
		{
			token tok;
			cur = escape_seq(std::next(cur),end,tok);
			assert(cur != end && *cur == '}');
			format.push_back(tok);

			return plain_or_meta(std::next(cur),end);
		}
		else
		{
			if(format.empty() || !format.back().is_literal)
			{
				token tok;
				tok.is_literal = true;
				tok.alias = std::string(1,*cur);
				format.push_back(tok);
			}
			else
				format.back().alias += std::string(1,*cur);
			return plain_or_meta(std::next(cur),end);
		}
	};

	// EscapeSequence -> Digit+ (':' Modifiers (':' Alias)?)?
	escape_seq = [&](std::string::const_iterator cur, std::string::const_iterator end,token &tok)
	{
		assert(cur != end && isdigit(*cur));
		cur = digits(cur,end,tok.width);
	
		tok.is_literal = false;
		tok.alias = "";
		tok.has_sign = false;

		if(cur != end && *cur == ':')
		{
			cur = modifiers(std::next(cur),end,tok);
			if(cur != end && *cur == ':')
				return alias(std::next(cur),end,tok);
		}

		return cur;
	};

	// Modifers -> '-'?
	modifiers = [&](std::string::const_iterator cur, std::string::const_iterator end,token &tok)
	{
		assert(cur != end);

		if(*cur == '-')
		{
			tok.has_sign = true;
			return std::next(cur);
		}

		return cur;
	};

	// Alias -> PlainAscii*
	alias = [&](std::string::const_iterator cur,std::string::const_iterator end,token &tok)
	{
		assert(cur != end);

		if(*cur != '}' && *cur != ':')
		{
			tok.alias += std::string(1,*cur);
			return alias(std::next(cur),end,tok);
		}

		return cur;
	};
	
	// Digit
	digits = [&](std::string::const_iterator cur,std::string::const_iterator end,unsigned int &i)
	{
		if(cur != end && isdigit(*cur))
		{
			i = i * 10 + *cur - '0';
			return digits(std::next(cur),end,i);
		}
		else
			return cur;
	};
	
	plain_or_meta(fmt.cbegin(),fmt.cend());
}

std::ostream &po::operator<<(std::ostream &os, const mnemonic &m)
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

int64_t po::format_constant(const po::mnemonic::token &tok, uint64_t v)
{
	assert(tok.width <= 64);
	uint64_t bitmask = 0;
	bitmask = (~bitmask) >> (64 - tok.width);

	if(tok.has_sign)
		return (int64_t)((v & (bitmask >> 1)) & ((v & (1 << (tok.width - 1))) << (64 - tok.width)));
	else
		return v & bitmask;
}
