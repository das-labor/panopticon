#include <sstream>
#include <algorithm>
#include <functional>

#include <mnemonic.hh>

using namespace po;

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
	// shared state of the parser
	unsigned int cur_bitwidth = 0;
	unsigned int cur_validx = 0;

	std::function<void(std::string::const_iterator,std::string::const_iterator)> plain_or_meta, escape_seq, data_type;

	// FormatString -> ('%' EscapeSequence) | PlainAscii
	plain_or_meta = [&](std::string::const_iterator cur, std::string::const_iterator end)
	{
		if(cur == end)
			return;
		else if(*cur == '%')
			return escape_seq(next(cur),end);
		else
		{
			if(format.empty() || format.back().type != token::Literal)
				format.emplace_back(token(std::string(1,*cur)));
			else
				format.back().literal += std::string(1,*cur);
			return plain_or_meta(next(cur),end);
		}
	};

	// EscapeSequence -> Digit+ DataType
	escape_seq = [&](std::string::const_iterator cur, std::string::const_iterator end)
	{
		assert(cur != end);
		
		if(isdigit(*cur))
		{
			cur_bitwidth = cur_bitwidth * 10 + (*cur - '0');
			return escape_seq(next(cur),end);
		}
		else
			return data_type(cur,end);
	};
		
	// DataType -> 's' | 'u'
	data_type = [&](std::string::const_iterator cur, std::string::const_iterator end)
	{
		assert(cur != end);
		assert(cur_bitwidth > 0);
		switch(*cur)
		{
		case 's':
			format.emplace_back(token(cur_validx,cur_bitwidth,token::Signed));
			break;
		case 'u':
			format.emplace_back(token(cur_validx,cur_bitwidth,token::Unsigned));
			break;
		default:
			assert(false);
		}

		cur_bitwidth = 0;
		++cur_validx;
		return plain_or_meta(next(cur),end);
	};
	
	plain_or_meta(fmt.cbegin(),fmt.cend());
}

std::ostream &po::operator<<(std::ostream &os, const mnemonic &m)
{
	os << m.opcode;

	if(m.operands.size())
		os << " ";

	for(const mnemonic::token &tok: m.format)
		switch(tok.type)
		{
		case mnemonic::token::Literal:	os << tok.literal; break;
		case mnemonic::token::Signed:	
			assert(m.operands.size() > tok.index);
			if(m.operands[tok.index].is_constant())
				os << (int)m.operands[tok.index].constant().value();
			else
				os << m.operands[tok.index];
			break;
		case mnemonic::token::Unsigned:
			assert(m.operands.size() > tok.index);
			os << m.operands[tok.index];
			break;
		default:
			assert(false);
		}
	
	return os;
}

mnemonic::token::token(std::string l) : type(Literal), literal(l) {}
mnemonic::token::token(unsigned int idx, unsigned int w, Type t) : type(t), index(idx), width(w) {}
