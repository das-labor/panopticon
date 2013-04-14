#include <sstream>
#include <algorithm>
#include <functional>

#include <mnemonic.hh>

using namespace po;
using namespace std;

const addr_t po::naddr = -1;

string po::pretty(instr::Function fn)
{
	switch(fn)
	{
		case instr::And: 		return " ∨ ";
		case instr::Or: 		return " ∧ ";
		case instr::Xor: 		return " ⊕ ";
		case instr::Not: 		return "¬";
		case instr::Assign:	return "";
		case instr::UShr: 	return " ≫ ";
		case instr::UShl: 	return " ≪ ";
		case instr::SShr: 	return " ≫ₛ ";
		case instr::SShl: 	return " ≪ₛ ";
		case instr::UExt: 	return " ↤ᵤ ";
		case instr::SExt: 	return " ↤ₛ ";
		case instr::Slice: 	return ":";
		//case instr::Concat: return " ∷ ";
		case instr::Add: 		return " + ";
		case instr::Sub: 		return " - ";
		case instr::Mul: 		return " × ";
		case instr::SDiv: 	return " ÷ₛ ";
		case instr::UDiv: 	return " ÷ᵤ ";
		case instr::SMod: 	return " modₛ ";
		case instr::UMod: 	return " modᵤ ";
		case instr::SLeq: 	return " ≤ₛ ";
		case instr::ULeq: 	return " ≤ᵤ ";
		case instr::Call: 	return "call";
		case instr::Phi: 		return "ϕ";
		default: assert(false);
	}
}

string po::symbolic(instr::Function fn)
{
	switch(fn)
	{
		case instr::And: 		return "po:and";
		case instr::Or: 		return "po:or";
		case instr::Xor:	 	return "po:xor";
		case instr::Not: 		return "po:not";
		case instr::Assign: return "po:assign";
		case instr::UShr: 	return "po:u-shift-right";
		case instr::UShl: 	return "po:i-shift-left";
		case instr::SShr: 	return "po:s-shift-right";
		case instr::SShl: 	return "po:s-shift-left";
		case instr::UExt: 	return "po:u-extend";
		case instr::SExt: 	return "po:s-extend";
		case instr::Slice:	return "po:slice";
		//case instr::Concat: return " ∷ ";
		case instr::Add: 		return "po:add";
		case instr::Sub: 		return "po:subtract";
		case instr::Mul: 		return "po:multiply";
		case instr::SDiv: 	return "po:s-divide";
		case instr::UDiv: 	return "po:u-divide";
		case instr::SMod: 	return "po:s-modulo";
		case instr::UMod: 	return "po:u-modulo";
		case instr::SLeq: 	return "po:s-less-equal";
		case instr::ULeq: 	return "po:u-less-equal";
		case instr::Call: 	return "po:call";
		case instr::Phi: 		return "po:phi";
		default: assert(false);
	}
}

instr::Function po::numeric(const std::string &s)
{
	if(s.substr(0,3) == "po:")
	{
		string t = s.substr(3);
		if(t == "po:and") return instr::And;
		if(t == "po:or") return instr::Or;
		if(t == "po:xor") return instr::Xor;
		if(t == "po:not") return instr::Not;
		if(t == "po:assign") return instr::Assign;
		if(t == "po:u-shift-right") return instr::UShr;
		if(t == "po:i-shift-left") return instr::UShl;
		if(t == "po:s-shift-right") return instr::SShr;
		if(t == "po:s-shift-left") return instr::SShl;
		if(t == "po:u-extend") return instr::UExt;
		if(t == "po:s-extend") return instr::SExt;
		if(t == "po:slice") return instr::Slice;
		//if(t == " ∷ ") return instr::Concat;
		if(t == "po:add") return instr::Add;
		if(t == "po:subtract") return instr::Sub;
		if(t == "po:multiply") return instr::Mul;
		if(t == "po:s-divide") return instr::SDiv;
		if(t == "po:u-divide") return instr::UDiv;
		if(t == "po:s-modulo") return instr::SMod;
		if(t == "po:u-modulo") return instr::UMod;
		if(t == "po:s-less-equal") return instr::SLeq;
		if(t == "po:u-less-equal") return instr::ULeq;
		if(t == "po:call") return instr::Call;
		if(t == "po:phi") return instr::Phi;
		assert(false);
	}
	else
	{
		if(s == " ∨ ") return instr::And;
		if(s == " ∧ ") return instr::Or;
		if(s == " ⊕ ") return instr::Xor;
		if(s == "¬") return instr::Not;
		if(s == "") return instr::Assign;
		if(s == " ≫ ") return instr::UShr;
		if(s == " ≪ ") return instr::UShl;
		if(s == " ≫ₛ ") return instr::SShr;
		if(s == " ≪ₛ ") return instr::SShl;
		if(s == " ↤ᵤ ") return instr::UExt;
		if(s == " ↤ₛ ") return instr::SExt;
		if(s == ":") return instr::Slice;
		//if(s == " ∷ ") return instr::Concat;
		if(s == " + ") return instr::Add;
		if(s == " - ") return instr::Sub;
		if(s == " × ") return instr::Mul;
		if(s == " ÷ₛ ") return instr::SDiv;
		if(s == " ÷ᵤ ") return instr::UDiv;
		if(s == " modₛ ") return instr::SMod;
		if(s == " modᵤ ") return instr::UMod;
		if(s == " ≤ₛ ") return instr::SLeq;
		if(s == " ≤ᵤ ") return instr::ULeq;
		if(s == "call") return instr::Call;
		if(s == "ϕ") return instr::Phi;
		assert(false);
	}
}
			
ostream &po::operator<<(ostream &os, const instr &i)
{
	string fnname = pretty(i.function);

	os << i.left << " ≔ ";
	if(i.right.size() == 0)
		os << fnname;
	else if(i.function == instr::Call)
		os << fnname << "(" << i.right[0] << ")";
	else if(i.right.size() == 1)
		os << fnname << i.right[0];
	else if(i.function == instr::Phi)
		os << fnname << "(" << i.right[0] << ", " << i.right[1] << ")";
	else if(i.function == instr::Slice)
		os << i.right[0] << "[" << i.right[1] << fnname << i.right[2] << "]";
	else if(i.right.size() == 3)
		os << fnname << "(" << i.right[0] << ", " << i.right[1] << ", " << i.right[2] << ")";
	else
		os << i.right[0] << fnname << i.right[1];
	return os;
}

odotstream& po::operator<<(odotstream &os, const instr &i)
{
	static_cast<ostringstream &>(os) << "<tr><td> </td><td ALIGN=\"LEFT\" COLSPAN=\"2\">" << i << "</td></tr>";
	return os;
}

oturtlestream& po::operator<<(oturtlestream &os, const instr &i)
{
	os << "[ po:function " << symbolic(i.function) << "; "
		 << " po:left " << static_cast<rvalue>(i.left) << "; "
		 << " po:right (";
		for(rvalue v: i.right)
			os << " " << v;
		os << " )]";
	return os;
}

oturtlestream& po::operator<<(oturtlestream &os, const mnemonic &m)
{
	os << "[ po:opcode \"" << m.opcode << "\"^^xsd:string;"
								 << " po:format \"" << accumulate(m.format.begin(),m.format.end(),string(),[&](const string &a, const mnemonic::token &t)
																									{
																										string ret = a;

																										if(t.is_literal)
																											ret += t.alias;
																										else
																										{
																											ret += "{" + to_string(t.width) + ":";
																											if(t.has_sign)
																												ret += "-";
																											ret += ":" + t.alias + "}";
																										}
																										return ret;
																									}) << "\"^^po:Format;"
		 						 << " po:begin " << m.area.begin << ";"
								 << " po:end " << m.area.end << ";" << endl
								 << " po:operands (";
	for(rvalue v: m.operands)
		os << " " << v;
	os << " );" << endl
								 << " po:executes (";

	for(const instr &i: m.instructions)
		os << " " << i;
	os << ")" << endl;
	

	os << "]" << endl;
	return os;
}

string po::unique_name(const mnemonic &mne)
{
	return "mne_" + to_string(mne.area.begin);
}

mnemonic mnemonic::unmarshal(const rdf::node &node, const rdf::storage &store)
{
	rdf::node nil = store.single("rdf:nil");
	rdf::statement opcode = store.first(node,"po:opcode",nullptr),
								 format = store.first(node,"po:format",nullptr),
								 begin = store.first(node,"po:begin",nullptr),
								 end = store.first(node,"po:end",nullptr),
								 op_head = store.first(node,"po:operands",nullptr),
								 exec_head = store.first(node,"po:executes",nullptr);
	list<instr> is;

	while(exec_head.object() != nil)
	{
		rdf::statement i_root = store.first(exec_head.object(),"rdf:first",nullptr),
									 func = store.first(i_root.object(),"po:function",nullptr),
									 left = store.first(i_root.object(),"po:left",nullptr),
									 right_head = store.first(i_root.object(),"po:right",nullptr);

		exec_head = store.first(exec_head.object(),"rdf:rest",nullptr);
	//	is.emplace_back(instr(numeric(func.object().to_string()),lvalue::unmarshal(left.object()),{}));
	}

	return mnemonic(range<addr_t>(stoull(begin.object().to_string()),stoull(end.object().to_string())),
									opcode.object().to_string(),
									format.object().to_string(),
									{},{});
//									initializer_list<instr>(is.begin(),is.end()));
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

string mnemonic::format_operands(void) const
{
	stringstream ss;
	unsigned int idx = 0;

	for(const mnemonic::token &tok: format)
	{
		if(tok.alias.empty() && !tok.is_literal)
		{
			assert(idx < operands.size());
			if(operands[idx].is_constant())
			{
				if(tok.has_sign)
					ss << (int)operands[idx].to_constant().content();
				else
					ss << operands[idx].to_constant().content();
			}
			else
				ss << operands[idx];
		}
		else
			ss << tok.alias;
		idx += !tok.is_literal;
	}

	return ss.str();
}

ostream &po::operator<<(ostream &os, const mnemonic &m)
{
	os << m.opcode;

	if(m.operands.size())
		os << " " << m.format_operands();
	
	return os;
}

odotstream& po::operator<<(odotstream &os, const mnemonic &m)
{
	os << "\t" << (os.subgraph ? "\t" : "")
		 << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x"
		 << std::hex << m.area.begin << std::dec 
		 << "</td><td ALIGN=\"LEFT\">" << m.opcode
		 << "</td><td ALIGN=\"LEFT\">"
		 << m.format_operands()
		 << "</td></tr>";

	if(os.instrs)
		 for(const instr &i: m.instructions)
			 os << i;

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
