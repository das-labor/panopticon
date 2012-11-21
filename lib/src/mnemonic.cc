#include <sstream>
#include <algorithm>

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

mnemonic::mnemonic(range<addr_t> a, std::string n, std::initializer_list<rvalue> ops, std::initializer_list<instr> instrs)
: area(a), opcode(n), operands(ops), instructions(instrs) {}

std::ostream &po::operator<<(std::ostream &os, const mnemonic &m)
{ 
	auto i = m.operands.cbegin();
	
	os << m.opcode;
	while(i != m.operands.cend())
	{
		os << " " << (*i);
		if(++i != m.operands.cend())
			os << ",";
	}

	return os;
}
