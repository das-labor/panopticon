#include <sstream>
#include <algorithm>
#include <functional>

#include <panopticon/mnemonic.hh>

using namespace po;
using namespace std;

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

	return "";
}

string po::symbolic(instr::Function fn)
{
	switch(fn)
	{
		case instr::And: 		return "and";
		case instr::Or: 		return "or";
		case instr::Xor:	 	return "xor";
		case instr::Not: 		return "not";
		case instr::Assign: return "assign";
		case instr::UShr: 	return "u-shift-right";
		case instr::UShl: 	return "i-shift-left";
		case instr::SShr: 	return "s-shift-right";
		case instr::SShl: 	return "s-shift-left";
		case instr::UExt: 	return "u-extend";
		case instr::SExt: 	return "s-extend";
		case instr::Slice:	return "slice";
		//case instr::Concat: return " ∷ ";
		case instr::Add: 		return "add";
		case instr::Sub: 		return "subtract";
		case instr::Mul: 		return "multiply";
		case instr::SDiv: 	return "s-divide";
		case instr::UDiv: 	return "u-divide";
		case instr::SMod: 	return "s-modulo";
		case instr::UMod: 	return "u-modulo";
		case instr::SLeq: 	return "s-less-equal";
		case instr::ULeq: 	return "u-less-equal";
		case instr::Call: 	return "call";
		case instr::Phi: 		return "phi";
		default: assert(false);
	}

	return "";
}

instr::Function po::numeric(const std::string &s)
{
	if(s.substr(0,string(PO).size()) == string(PO))
	{
		string t = s.substr(string(PO).size());

		if(t == "and") return instr::And;
		if(t == "or") return instr::Or;
		if(t == "xor") return instr::Xor;
		if(t == "not") return instr::Not;
		if(t == "assign") return instr::Assign;
		if(t == "u-shift-right") return instr::UShr;
		if(t == "i-shift-left") return instr::UShl;
		if(t == "s-shift-right") return instr::SShr;
		if(t == "s-shift-left") return instr::SShl;
		if(t == "u-extend") return instr::UExt;
		if(t == "s-extend") return instr::SExt;
		if(t == "slice") return instr::Slice;
		//if(t == " ∷ ") return instr::Concat;
		if(t == "add") return instr::Add;
		if(t == "subtract") return instr::Sub;
		if(t == "multiply") return instr::Mul;
		if(t == "s-divide") return instr::SDiv;
		if(t == "u-divide") return instr::UDiv;
		if(t == "s-modulo") return instr::SMod;
		if(t == "u-modulo") return instr::UMod;
		if(t == "s-less-equal") return instr::SLeq;
		if(t == "u-less-equal") return instr::ULeq;
		if(t == "call") return instr::Call;
		if(t == "phi") return instr::Phi;
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
	}

	assert(false);
	return instr::Assign;
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

mnemonic::mnemonic(const mnemonic &m)
: area(m.area),
	opcode(m.opcode),
	operands(m.operands),
	instructions(m.instructions),
	format_seq(m.format_seq),
	format_string(m.format_string)
{}

mnemonic::mnemonic(mnemonic &&m)
: area(move(m.area)),
	opcode(move(m.opcode)),
	operands(move(m.operands)),
	instructions(move(m.instructions)),
	format_seq(move(m.format_seq)),
	format_string(move(m.format_string))
{}

mnemonic &mnemonic::operator=(const mnemonic &m)
{
	if(&m != this)
	{
		area = m.area;
		opcode = m.opcode;
		operands = m.operands;
		instructions = m.instructions;
		format_string = m.format_string;
		format_seq = m.format_seq;
	}

	return *this;
}

mnemonic &mnemonic::operator=(mnemonic &&m)
{
	area = move(m.area);
	opcode = move(m.opcode);
	operands = move(m.operands);
	instructions = move(m.instructions);
	format_string = move(m.format_string);
	format_seq = move(m.format_seq);

	return *this;
}

mnemonic::mnemonic(const bound &a, const string &n, const string &fmt, initializer_list<rvalue> ops, initializer_list<instr> instrs)
: area(a), opcode(n), operands(ops), instructions(instrs), format_seq(), format_string(fmt)
{
	function<string::const_iterator(string::const_iterator,string::const_iterator)> plain_or_meta;
	function<string::const_iterator(string::const_iterator,string::const_iterator,token&)> escape_seq, modifiers, alias;
	function<string::const_iterator(string::const_iterator,string::const_iterator,unsigned int&)> digits;

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
			format_seq.push_back(tok);

			return plain_or_meta(next(cur),end);
		}
		else
		{
			if(format_seq.empty() || !format_seq.back().is_literal)
			{
				token tok;
				tok.is_literal = true;
				tok.alias = string(1,*cur);
				format_seq.push_back(tok);
			}
			else
				format_seq.back().alias += string(1,*cur);
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

string mnemonic::format_operands(void) const
{
	stringstream ss;
	unsigned int idx = 0;

	for(const mnemonic::token &tok: format_seq)
	{
		if(tok.alias.empty() && !tok.is_literal)
		{
			assert(idx < operands.size());
			if(is_constant(operands[idx]))
			{
				if(tok.has_sign)
					ss << (int)to_constant(operands[idx]).content();
				else
					ss << to_constant(operands[idx]).content();
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

bool mnemonic::operator==(const mnemonic& m) const
{
	return opcode == m.opcode &&
				 area == m.area &&
				 operands == m.operands &&
				 instructions == m.instructions &&
				 format_seq == m.format_seq &&
				 format_string == m.format_string;
}

ostream &po::operator<<(ostream &os, const mnemonic &m)
{
	os << m.opcode;

	if(m.operands.size())
		os << " " << m.format_operands();

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

template<>
po::mnemonic* po::unmarshal(const po::uuid& u, const po::rdf::storage& store)
{
	rdf::node node = rdf::ns_local(to_string(u));
	rdf::statement opcode = store.first(node,rdf::ns_po("opcode")),
		format = store.first(node, rdf::ns_po("format")),
		begin = store.first(node, rdf::ns_po("begin")),
		end = store.first(node, rdf::ns_po("end")),
		op_head = store.first(node, rdf::ns_po("operands")),
		exec_head = store.first(node, rdf::ns_po("executes"));

	rdf::nodes ops = rdf::read_list(op_head.object,store);
	rdf::nodes xs = rdf::read_list(exec_head.object,store);
	list<instr> is;
	list<rvalue> as;
	boost::uuids::string_generator sg;

	std::transform(ops.begin(),ops.end(),back_inserter(as),[&](const rdf::node &n)
		{ std::cerr << n.as_iri().substr(n.as_iri().size()-36) << std::endl; return *unmarshal<rvalue>(sg(n.as_iri().substr(n.as_iri().size()-36)),store); });
	std::transform(xs.begin(),xs.end(),back_inserter(is),[&](const rdf::node &i_root)
	{
		rdf::statement func = store.first(i_root,rdf::ns_po("function")),
									 left = store.first(i_root,rdf::ns_po("left")),
									 right_head = store.first(i_root,rdf::ns_po("right"));

		rdf::nodes rights = read_list(right_head.object,store);
		vector<rvalue> rs;

		std::transform(rights.begin(),rights.end(),back_inserter(rs),[&](const rdf::node &n)
			{ return *unmarshal<rvalue>(sg(n.as_iri().substr(n.as_iri().size()-36)),store); });

		instr::Function fn = static_cast<instr::Function>(numeric(func.object.as_iri()));
			lvalue l = to_lvalue(*unmarshal<rvalue>(sg(left.object.as_iri().substr(left.object.as_iri().size()-36)),store));
		instr ret(fn,l,{});
		ret.right = rs;

		return ret;
	});

	return new mnemonic(bound(stoull(begin.object.as_literal()),stoull(end.object.as_literal())),
									opcode.object.as_literal(),
									format.object.as_literal(),
									as.begin(),as.end(),
									is.begin(),is.end());
}

template<>
rdf::statements po::marshal(const mnemonic* mn, const uuid& uu)
{
	size_t rv_cnt = 0;
	boost::uuids::name_generator ng(uu);
	rdf::statements ret;
	std::function<rdf::node(const rvalue&)> map_rvs = [&](const rvalue &rv)
	{
		uuid u = ng(to_string(rv_cnt++));
		rdf::node r(rdf::ns_local(to_string(u)));
		auto st = marshal(&rv,u);

		std::move(st.begin(),st.end(),back_inserter(ret));
		return r;
	};
	rdf::node r = rdf::ns_local(to_string(uu));

	ret.emplace_back(r,rdf::ns_po("opcode"),rdf::lit(mn->opcode));
	ret.emplace_back(r,rdf::ns_po("format"),rdf::lit(mn->format_string));
	ret.emplace_back(r,rdf::ns_po("begin"),rdf::lit(mn->area.lower()));
	ret.emplace_back(r,rdf::ns_po("end"),rdf::lit(mn->area.upper()));

	rdf::nodes n_ops, n_ex;

	std::transform(mn->operands.begin(),mn->operands.end(),back_inserter(n_ops),map_rvs);

	std::transform(mn->instructions.begin(),mn->instructions.end(),back_inserter(n_ex),[&](const instr& i)
	{
		uuid u = ng(to_string(rv_cnt++));
		uuid ul = ng(to_string(rv_cnt++));
		rdf::node r(rdf::ns_local(to_string(u))), rl(rdf::ns_local(to_string(ul))), rr = rdf::node::blank();
		rdf::statements rs, ls;
		rvalue il = i.left;

		ls = marshal(&il,ul);
		std::move(ls.begin(),ls.end(),back_inserter(ret));

		rdf::nodes rn;
		std::transform(i.right.begin(),i.right.end(),back_inserter(rn),map_rvs);
		tie(rr,rs) = write_list(rn.begin(),rn.end(),to_string(u));
		std::move(rs.begin(),rs.end(),back_inserter(ret));

		ret.emplace_back(r,rdf::ns_po("function"),rdf::ns_po(symbolic(i.function)));
		ret.emplace_back(r,rdf::ns_po("left"),rl);
		ret.emplace_back(r,rdf::ns_po("right"),rr);

		return r;
	});

	auto p_ops = write_list(n_ops.begin(),n_ops.end(),to_string(uu) + "-operands");
	auto p_ex = write_list(n_ex.begin(),n_ex.end(),to_string(uu) + "-instrs");

	std::move(p_ops.second.begin(),p_ops.second.end(),back_inserter(ret));
	std::move(p_ex.second.begin(),p_ex.second.end(),back_inserter(ret));

	ret.emplace_back(r,rdf::ns_po("operands"),p_ops.first);
	ret.emplace_back(r,rdf::ns_po("executes"),p_ex.first);

	return ret;
}
