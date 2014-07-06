#include <sstream>
#include <algorithm>
#include <functional>

#include <panopticon/mnemonic.hh>

using namespace po;
using namespace std;

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
			ensure(cur != end && *cur == '}');
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
		ensure(cur != end && isdigit(*cur));
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
		ensure(cur != end);

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
		ensure(cur != end);

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
			ensure(idx < operands.size());
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
	ensure(tok.width <= 64);
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
	rdf::node node = rdf::iri(u);
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

	std::transform(ops.begin(),ops.end(),back_inserter(as),[&](const rdf::node &n)
	{
		return *unmarshal<rvalue>(n.as_iri().as_uuid(),store);
	});

	std::transform(xs.begin(),xs.end(),back_inserter(is),[&](const rdf::node &i_root)
	{
		rdf::statement func = store.first(i_root,rdf::ns_po("function")),
							left = store.first(i_root,rdf::ns_po("left")),
							right_head = store.first(i_root,rdf::ns_po("right"));

		rdf::nodes rights = read_list(right_head.object,store);
		vector<rvalue> rs;

		std::transform(rights.begin(),rights.end(),back_inserter(rs),[&](const rdf::node &n)
			{ return *unmarshal<rvalue>(n.as_iri().as_uuid(),store); });

		instr::operation fn = from_symbolic(func.object.as_iri().as_string(),rs);
			lvalue l = to_lvalue(*unmarshal<rvalue>(left.object.as_iri().as_uuid(),store));
		instr ret(fn,l);

		return ret;
	});

	return new mnemonic(bound(stoull(begin.object.as_literal()),stoull(end.object.as_literal())),
									opcode.object.as_literal(),
									format.object.as_literal(),
									as.begin(),as.end(),
									is.begin(),is.end());
}

template<>
archive po::marshal(const mnemonic* mn, const uuid& uu)
{
	size_t rv_cnt = 0;
	boost::uuids::name_generator ng(uu);
	rdf::statements ret;
	std::list<blob> bl;
	std::function<rdf::node(const rvalue&)> map_rvs = [&](const rvalue &rv)
	{
		uuid u = ng(to_string(rv_cnt++));
		rdf::node r = rdf::iri(u);
		auto st = marshal(&rv,u);

		ensure(st.triples.size());
		std::move(st.triples.begin(),st.triples.end(),back_inserter(ret));
		std::move(st.blobs.begin(),st.blobs.end(),back_inserter(bl));
		return r;
	};
	rdf::node r = rdf::iri(uu);

	ret.emplace_back(r,rdf::ns_po("opcode"),rdf::lit(mn->opcode));
	ret.emplace_back(r,rdf::ns_po("format"),rdf::lit(mn->format_string));
	ret.emplace_back(r,rdf::ns_po("begin"),rdf::lit(mn->area.lower()));
	ret.emplace_back(r,rdf::ns_po("end"),rdf::lit(mn->area.upper()));

	rdf::nodes n_ops, n_ex;

	std::transform(mn->operands.begin(),mn->operands.end(),back_inserter(n_ops),map_rvs);

	std::transform(mn->instructions.begin(),mn->instructions.end(),back_inserter(n_ex),[&](const instr& i)
	{
		uuid u = ng(to_string(rv_cnt++));
		rdf::node r = rdf::iri(u), rl = rdf::node::blank(), rr = rdf::node::blank();
		rdf::statements rs;

		rl = map_rvs(i.assignee);

		rdf::nodes rn;
		std::vector<rvalue> right = operands(i);
		std::transform(right.begin(),right.end(),back_inserter(rn),map_rvs);
		tie(rr,rs) = write_list(rn.begin(),rn.end(),u);
		std::move(rs.begin(),rs.end(),back_inserter(ret));

		ret.emplace_back(r,rdf::ns_po("function"),rdf::iri(symbolic(i.function)));
		ret.emplace_back(r,rdf::ns_po("left"),rl);
		ret.emplace_back(r,rdf::ns_po("right"),rr);

		return r;
	});

	auto p_ops = write_list(n_ops.begin(),n_ops.end(),ng(to_string(uu) + "-operands"));
	auto p_ex = write_list(n_ex.begin(),n_ex.end(),ng(to_string(uu) + "-instrs"));

	std::move(p_ops.second.begin(),p_ops.second.end(),back_inserter(ret));
	std::move(p_ex.second.begin(),p_ex.second.end(),back_inserter(ret));

	ret.emplace_back(r,rdf::ns_po("operands"),p_ops.first);
	ret.emplace_back(r,rdf::ns_po("executes"),p_ex.first);

	return archive(ret,bl);
}
