#ifndef MNEMONIC_HH
#define MNEMONIC_HH

#include <list>
#include <string>
#include <vector>
#include <memory>
#include <cassert>
#include <sstream>

using namespace std;

typedef uint32_t addr_t;

typedef shared_ptr<struct value> value_ptr;
typedef shared_ptr<struct variable> var_ptr;
typedef shared_ptr<struct instr> instr_ptr;
typedef shared_ptr<const struct instr> instr_cptr;
typedef shared_ptr<class mnemonic> mne_ptr;
typedef shared_ptr<const class mnemonic> mne_cptr;

struct area
{
	area(void) : isset(false) {};
	area(addr_t b, addr_t e) : isset(true), begin(b), end(e) { assert(begin <= end); };

	size_t size(void) const { return end - begin; };
	bool includes(const area &a) const { return isset && a.isset && begin <= a.begin && end > a.end; };
	bool includes(addr_t a) const { return isset && begin <= a && end > a; };
	bool overlap(const area &a) const { return isset && a.isset && !(begin >= a.end || end <= a.begin); };

	bool isset;
	addr_t begin;
	addr_t end;
};

bool operator==(const area &a, const area &b);
bool operator!=(const area &a, const area &b);
bool operator<(const area &a, const area &b);

struct name
{
	name(string n);
	name(const char *a);
	string inspect(void) const;	
	
	string base;
	int subscript;
};

bool operator<(const name &a, const name &b);
bool operator==(const name &a, const name &b);
bool operator!=(const name &a, const name &b);

class value 
{
public: 
	virtual ~value(void);
	virtual string inspect(void) const = 0;

protected: 
	value(void);
};

struct constant : public value
{
	constant(int v);

	virtual string inspect(void) const;

	int val;
};

struct undefined : public value
{
	undefined(void);
	virtual string inspect(void) const;
};

struct variable : public value
{
	variable(name nam);
	variable(string nam);
	virtual string inspect(void) const;
	
	name nam;
};

class instr
{
public:
	enum Opcode
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
		Concat,		// Concatenation
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

	//instr(Opcode code, string opname, char var, vector<char> ops);
	instr(Opcode code, string opname, name var, vector<value_ptr> ops);
	
	string inspect(void) const;
		
	Opcode opcode;
	string opname;
	shared_ptr<variable> assigns;
	vector<value_ptr> operands;
};

struct valproxy
{
	valproxy(value_ptr v) : value(v) {};
	valproxy(const char *a) : value(new variable(name(a))) {};
	valproxy(int a) : value(new constant(a)) {};
	valproxy(void *a) : value(new undefined()) {};

	value_ptr value;
};

template<typename T>
struct instr_builder
{
	T and_b(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::And,		" ∨ ",		a,{op1.value,op2.value}))); };
	T or_b(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::Or,		" ∧ ",		a,{op1.value,op2.value}))); };
	T xor_b(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::Xor,		" ⊕ ",		a,{op1.value,op2.value}))); };
	T phi(name a, valproxy op1, valproxy op2)		{ return accept_instr(instr_ptr(new instr(instr::Phi,		"ϕ",			a,{op1.value,op2.value}))); };
	T not_b(name a, valproxy op) 								{ return accept_instr(instr_ptr(new instr(instr::Not,		"¬",			a,{op.value}))); };
	T assign(name a, valproxy op)								{ return accept_instr(instr_ptr(new instr(instr::Assign,"",				a,{op.value}))); };
	T undef(name a)															{ return accept_instr(instr_ptr(new instr(instr::Assign,"",				a,{value_ptr(new undefined)}))); };
	T shiftr_u(name a, valproxy cnt, valproxy op)	{ return accept_instr(instr_ptr(new instr(instr::UShr,	" ≫ ",		a,{cnt.value,op.value}))); };
	T shiftl_u(name a, valproxy cnt, valproxy op)	{ return accept_instr(instr_ptr(new instr(instr::UShl,	" ≪ ",		a,{cnt.value,op.value}))); };
	T shiftr_s(name a, valproxy cnt, valproxy op)	{ return accept_instr(instr_ptr(new instr(instr::SShr,	" ≫ₛ ",		a,{cnt.value,op.value}))); };
	T shiftl_s(name a, valproxy cnt, valproxy op)	{ return accept_instr(instr_ptr(new instr(instr::SShl,	" ≪ₛ ",		a,{cnt.value,op.value}))); };
	T ext_u(name a, valproxy cnt, valproxy op)		{ return accept_instr(instr_ptr(new instr(instr::UExt,	" ↤ᵤ ",		a,{cnt.value,op.value}))); };
	T ext_s(name a, valproxy cnt, valproxy op)		{ return accept_instr(instr_ptr(new instr(instr::SExt,	" ↤ₛ ",		a,{cnt.value,op.value}))); };
	T slice(name a, valproxy op, valproxy from, valproxy to)	{ return accept_instr(instr_ptr(new instr(instr::Slice,	":",a,{op.value,from.value,to.value}))); };
	T concat(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::Concat," ∷ ",		a,{op1.value,op2.value}))); };
	T add_i(name a, valproxy op1, valproxy op2)		{ return accept_instr(instr_ptr(new instr(instr::Add,		" + ",		a,{op1.value,op2.value}))); };
	T sub_i(name a, valproxy op1, valproxy op2)		{ return accept_instr(instr_ptr(new instr(instr::Sub,		" - ",		a,{op1.value,op2.value}))); };
	T mul_i(name a, valproxy op1, valproxy op2)		{ return accept_instr(instr_ptr(new instr(instr::Mul,		" × ",		a,{op1.value,op2.value}))); };
	T div_is(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::SDiv,	" ÷ₛ ",		a,{op1.value,op2.value}))); };
	T div_iu(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::UDiv,	" ÷ᵤ ",		a,{op1.value,op2.value}))); };
	T mod_is(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::SMod,	" modₛ ",	a,{op1.value,op2.value}))); };
	T mod_iu(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::UMod,	" modᵤ ",	a,{op1.value,op2.value}))); };
	T leq_is(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::SLeq,	" ≤ₛ ",	a,{op1.value,op2.value}))); };
	T leq_iu(name a, valproxy op1, valproxy op2)	{ return accept_instr(instr_ptr(new instr(instr::ULeq,	" ≤ᵤ ",	a,{op1.value,op2.value}))); };
	T call(name a, valproxy op)										{ return accept_instr(instr_ptr(new instr(instr::Call,	"call",	a,{op.value}))); };

protected:
	virtual T accept_instr(instr_ptr i) = 0;
};

class mnemonic : public instr_builder<instr_ptr>
{
public:
	mnemonic(area a, string n, list<value_ptr> v = list<value_ptr>()) : addresses(a), name(n), arguments(v) {};

	area addresses;
	string name;
	list<instr_cptr> instructions;
	list<value_ptr> arguments;

	instr_ptr accept_instr(instr_ptr i)	{ instructions.push_back(i); return i; };
	string inspect(void) const 
	{ 
		auto i = arguments.cbegin();
		stringstream ret;
		
		ret << name;
		while(i != arguments.cend())
		{
			ret << " " << (*i)->inspect();
			if(++i != arguments.cend())
				ret << ",";
		}

		return ret.str();
	};
};

#endif
