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

bool operator<(const constant &a, const constant &b);
bool operator==(const constant &a, const constant &b);
bool operator!=(const constant &a, const constant &b);

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
	valproxy(const name &a) : value(new variable(name(a))) {};
	valproxy(int a) : value(new constant(a)) {};
	valproxy(void *a) : value(new undefined()) {};

	value_ptr value;
};

class instr_builder
{
public:
	// named
	value_ptr and_b(name a, valproxy op1, valproxy op2);
	value_ptr or_b(name a, valproxy op1, valproxy op2);
	value_ptr xor_b(name a, valproxy op1, valproxy op2);
	value_ptr phi(name a, valproxy op1, valproxy op2);
	value_ptr not_b(name a, valproxy op);
	value_ptr assign(name a, valproxy op);
	value_ptr undef(name a);
	value_ptr shiftr_u(name a, valproxy cnt, valproxy op);
	value_ptr shiftl_u(name a, valproxy cnt, valproxy op);
	value_ptr shiftr_s(name a, valproxy cnt, valproxy op);
	value_ptr shiftl_s(name a, valproxy cnt, valproxy op);
	value_ptr ext_u(name a, valproxy cnt, valproxy op);
	value_ptr ext_s(name a, valproxy cnt, valproxy op);
	value_ptr slice(name a, valproxy op, valproxy from, valproxy to);
	value_ptr concat(name a, valproxy op1, valproxy op2);
	value_ptr add_i(name a, valproxy op1, valproxy op2);
	value_ptr sub_i(name a, valproxy op1, valproxy op2);
	value_ptr mul_i(name a, valproxy op1, valproxy op2);
	value_ptr div_is(name a, valproxy op1, valproxy op2);
	value_ptr div_iu(name a, valproxy op1, valproxy op2);
	value_ptr mod_is(name a, valproxy op1, valproxy op2);
	value_ptr mod_iu(name a, valproxy op1, valproxy op2);
	value_ptr leq_is(name a, valproxy op1, valproxy op2);
	value_ptr leq_iu(name a, valproxy op1, valproxy op2);
	value_ptr call(name a, valproxy op);
	
	// anonymous
	value_ptr and_b(valproxy op1, valproxy op2);
	value_ptr or_b(valproxy op1, valproxy op2);
	value_ptr xor_b(valproxy op1, valproxy op2);
	value_ptr phi(valproxy op1, valproxy op2);
	value_ptr not_b(valproxy op);
	value_ptr assign(valproxy op);
	value_ptr undef(void);
	value_ptr shiftr_u(valproxy cnt, valproxy op);
	value_ptr shiftl_u(valproxy cnt, valproxy op);
	value_ptr shiftr_s(valproxy cnt, valproxy op);
	value_ptr shiftl_s(valproxy cnt, valproxy op);
	value_ptr ext_u(valproxy cnt, valproxy op);
	value_ptr ext_s(valproxy cnt, valproxy op);
	value_ptr slice(valproxy op, valproxy from, valproxy to);
	value_ptr concat(valproxy op1, valproxy op2);
	value_ptr add_i(valproxy op1, valproxy op2);
	value_ptr sub_i(valproxy op1, valproxy op2);
	value_ptr mul_i(valproxy op1, valproxy op2);
	value_ptr div_is(valproxy op1, valproxy op2);
	value_ptr div_iu(valproxy op1, valproxy op2);
	value_ptr mod_is(valproxy op1, valproxy op2);
	value_ptr mod_iu(valproxy op1, valproxy op2);
	value_ptr leq_is(valproxy op1, valproxy op2);
	value_ptr leq_iu(valproxy op1, valproxy op2);
	value_ptr call(valproxy op);

protected:
	virtual value_ptr accept_instr(instr_ptr i) = 0;
	static unsigned int next;
};

class mnemonic : public instr_builder
{
public:
	typedef list<instr_cptr>::const_iterator iterator;
	mnemonic(area a, string n, list<value_ptr> v = list<value_ptr>()) : addresses(a), name(n), arguments(v) {};

	area addresses;
	string name;
	list<instr_cptr> instructions;
	list<value_ptr> arguments;

	value_ptr accept_instr(instr_ptr i)	{ instructions.push_back(i); return i->assigns; };
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
