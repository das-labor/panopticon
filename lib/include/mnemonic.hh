#ifndef MNEMONIC_HH
#define MNEMONIC_HH

#include <iostream>
#include <list>
#include <string>
#include <vector>
#include <cassert>
#include <initializer_list>

#include <value.hh>
#include <marshal.hh>

namespace po
{
	class instr;
	class mnemonic;

	typedef uint32_t addr_t;
	extern const addr_t naddr;

	template<typename T>
	struct range
	{
		range(void) : begin(0), end(0) {}
		range(T b) : begin(b), end(b) {}
		range(T b, T e) : begin(b), end(e) { assert(begin <= end); }

		size_t size(void) const { return end - begin; }
		bool includes(const range<T> &a) const { return size() && a.size() && begin <= a.begin && end > a.end; }
		bool includes(T a) const { return size() && begin <= a && end > a; }
		bool overlap(const range<T> &a) const { return size() && a.size() && !(begin >= a.end || end <= a.begin); }
		T last(void) const { return size() ? end - 1 : begin; }
		
		bool operator==(const range<T> &b) const { return size() == b.size() && (!size() || (begin == b.begin && end == b.end)); }
		bool operator!=(const range<T> &b) const { return !(*this == b); }
		bool operator<(const range<T> &b) const { return begin < b.begin; }

		T begin;
		T end;
	};

	template<typename T>
	std::ostream& operator<<(std::ostream &os, const range<T> &r)
	{ 
		if(r.size())
		{
			os << r.begin;
			if(r.size() > 1)
				os << "-" << r.end-1;
		}
		else
			os << "nil";
		return os; 
	}

	class instr
	{
	public:
		enum Function
		{
			Phi,			// phi-Function
			Not,			// Bitwise Not
			And,			// Bitwise And
			Or,				// Bitwise Or
			Xor,			// Bitwize Xor
			Assign,		// Assign Intermediate
			ULeq,			// Unsigned less-or-equal
			SLeq,			// Signed less-or-equal
			UShr,			// Unsigned right shift	*
			UShl,			// Unsigned left shift *
			SShr,			// Signed right shift
			SShl,			// Signed left shift
			SExt,			// Signed extension *
			UExt,			// Unsigned extension *
			Slice,		// Slice
			//Concat,		// Concatenation
			Add,			// Addition
			Sub,			// Subtraction
			Mul,			// Multiplication
			SDiv,			// Signed Division
			UDiv,			// Unsigned Division
			SMod,			// Unsigned Modulo reduction
			UMod,			// Signed Modulo reduction
			Call,			// Procedure call
			// Floating point
		};

		template<class... Values>
		instr(Function fn, lvalue a, Values&&... args) : function(fn), left(a), right({args...}) {}
		
		Function function;
		lvalue left;
		std::vector<rvalue> right;
	};

	odotstream& operator<<(odotstream &os, const instr &i);
	oturtlestream& operator<<(oturtlestream &os, const instr &i);
	std::string pretty(instr::Function fn);
	std::string symbolic(instr::Function fn);
	instr::Function numeric(const std::string &s);

	class mnemonic
	{
	public:
		typedef std::vector<instr>::const_iterator iterator;
		
		struct token
		{
			token(void) : has_sign(false), width(0), alias(""), is_literal(false) {}
			bool has_sign;
			unsigned int width;
			std::string alias;
			bool is_literal;
		};
		
		static mnemonic unmarshal(const rdf::node &n, const rdf::storage &store);

		template <typename F1, typename F2>
		mnemonic(const range<addr_t> &a, const std::string &n, const std::string &fmt, F1 ops_begin, F1 ops_end, F2 instr_begin, F2 instr_end)
		: mnemonic(a,n,fmt,{},{})
		{
			std::copy(ops_begin,ops_end,inserter(operands,operands.begin()));
			std::copy(instr_begin,instr_end,inserter(instructions,instructions.begin()));
		}

		mnemonic(const range<addr_t> &a, const std::string &n, const std::string &fmt, std::initializer_list<rvalue> ops, std::initializer_list<instr> instrs);

		mnemonic(const mnemonic &m);
		mnemonic(mnemonic &&m);

		mnemonic &operator=(const mnemonic &m);
		mnemonic &operator=(mnemonic &&m);

		std::string format_operands(void) const;

		range<addr_t> area;
		std::string opcode;
		std::vector<rvalue> operands;
		std::vector<instr> instructions;
		std::vector<token> format;
	};
	
	std::ostream& operator<<(std::ostream &os, const instr &i);
	std::ostream& operator<<(std::ostream &os, const mnemonic &m);
	odotstream& operator<<(odotstream &os, const mnemonic &m);
	oturtlestream& operator<<(oturtlestream &os, const mnemonic &m);
	std::string unique_name(const mnemonic &mne);

	int64_t format_constant(const mnemonic::token &tok, uint64_t v);
}

#endif
