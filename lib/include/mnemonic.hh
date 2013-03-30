#ifndef MNEMONIC_HH
#define MNEMONIC_HH

#include <iostream>
#include <list>
#include <string>
#include <vector>
#include <cassert>
#include <initializer_list>

#include <value.hh>
#include <inflate.hh>

namespace po
{
	class instr;
	class mnemonic;

	typedef uint32_t addr_t;
	extern const addr_t naddr;

	template<typename T>
	struct range
	{
		range(void) { begin = end = 0; }
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
		instr(Function fn, lvalue a, Values&&... args)
		: function(fn), left(a), right({args...})
		{
			switch(fn)
			{
			case And: 	fnname = " ∨ "; break;
			case Or: 		fnname = " ∧ "; break;
			case Xor: 	fnname = " ⊕ "; break;
			case Not: 	fnname = "¬"; break;
			case Assign: fnname = ""; break;
			case UShr: 	fnname = " ≫ "; break;
			case UShl: 	fnname = " ≪ "; break;
			case SShr: 	fnname = " ≫ₛ "; break;
			case SShl: 	fnname = " ≪ₛ "; break;
			case UExt: 	fnname = " ↤ᵤ "; break;
			case SExt: 	fnname = " ↤ₛ "; break;
			case Slice: fnname = ":"; break;
			//case Concat: fnname = " ∷ "; break;
			case Add: 	fnname = " + "; break;
			case Sub: 	fnname = " - "; break;
			case Mul: 	fnname = " × "; break;
			case SDiv: 	fnname = " ÷ₛ "; break;
			case UDiv: 	fnname = " ÷ᵤ "; break;
			case SMod: 	fnname = " modₛ "; break;
			case UMod: 	fnname = " modᵤ "; break;
			case SLeq: 	fnname = " ≤ₛ "; break;
			case ULeq: 	fnname = " ≤ᵤ "; break;
			case Call: 	fnname = "call"; break;
			case Phi: 	fnname = "ϕ"; break;
			default: assert(false);
			}
		}
			

		Function function;
		std::string fnname;
		lvalue left;
		std::vector<rvalue> right;
	};

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

		range<addr_t> area;
		std::string opcode;
		std::vector<rvalue> operands;
		std::vector<instr> instructions;
		std::vector<token> format;
	};
	
	std::ostream& operator<<(std::ostream &os, const instr &i);
	std::ostream& operator<<(std::ostream &os, const mnemonic &m);
	odotstream& operator<<(odotstream &os, const mnemonic &m);

	int64_t format_constant(const mnemonic::token &tok, uint64_t v);
}

#endif
