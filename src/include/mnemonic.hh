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
typedef shared_ptr<const struct variable> var_cptr;
typedef shared_ptr<struct constant> const_ptr;
typedef shared_ptr<const struct constant> const_cptr;
typedef shared_ptr<struct instr> instr_ptr;
typedef shared_ptr<const struct instr> instr_cptr;
typedef shared_ptr<class mnemonic> mne_ptr;
typedef shared_ptr<const class mnemonic> mne_cptr;

struct area
{
	area(void) : begin(0), end(0) {};
	area(addr_t b, addr_t e) : begin(b), end(e) { assert(begin <= end); };

	size_t size(void) const { return end - begin; };
	bool includes(const area &a) const { return size() && a.size() && begin <= a.begin && end > a.end; };
	bool includes(addr_t a) const { return size() && begin <= a && end > a; };
	bool overlap(const area &a) const { return size() && a.size() && !(begin >= a.end || end <= a.begin); };
	addr_t last(void) const { return size() ? end - 1 : begin; };

	addr_t begin;
	addr_t end;
};

bool operator==(const area &a, const area &b);
bool operator!=(const area &a, const area &b);
bool operator<(const area &a, const area &b);
ostream& operator<<(ostream &os, const area &);

struct name
{
	name(string n);
	name(string n, int i);
	name(const char *a);
	string inspect(void) const;	
	
	string base;
	int subscript;
};

bool operator<(const name &a, const name &b);
bool operator==(const name &a, const name &b);
bool operator!=(const name &a, const name &b);

namespace std {
template<>
struct hash<name>
{
	size_t operator()(const name &n) const 
	{
		hash<std::string> hsh1;
		hash<int> hsh2;
		return hsh1(n.base) ^ hsh2(n.subscript); 
	};
};
}
class value 
{
public: 
	value(unsigned int w);
	virtual ~value(void);
	virtual string inspect(void) const = 0;
	
	unsigned int width;

protected: 
	value(void);
};

struct constant : public value
{
	constant(int v, unsigned int w);
	virtual string inspect(void) const;

	unsigned int val;
};

bool operator<(const constant &a, const constant &b);
bool operator==(const constant &a, const constant &b);
bool operator!=(const constant &a, const constant &b);

struct undefined : public value
{
	undefined(unsigned int w);
	virtual string inspect(void) const;
};

struct variable : public value
{
	variable(name nam, unsigned int w);
	variable(string nam, unsigned int w);
	virtual string inspect(void) const;
	
	name nam;
};

/*struct address : public value
{
	address(unsigned int o, unsigned int w, string n);
	address(unsigned int w, string n);
	virtual string inspect(void) const;

	unsigned int offset;	// from start of the memory space
	unsigned int width;		// word size in bytes
	string name;					// descriptive name of the memory space (stack, flash, ...)
};*/

struct valproxy
{
	valproxy(value_ptr v) : value(v) {};
	valproxy(const char *a) : value(value_ptr(new variable(string(a),0))) {};
	valproxy(const name &a) : value(value_ptr(new variable(a,0))) {};
	valproxy(int a) : value(value_ptr(new constant(a,0))) {};
	valproxy(void *a) : value(value_ptr(new undefined(0))) {};

	value_ptr value;
};

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

	instr(Function fn, string fnname, var_ptr var, vector<value_ptr> args);
	
	string inspect(void) const;
		
	Function function;
	string fnname;
	shared_ptr<variable> assigns;
	vector<value_ptr> arguments;
};

class mnemonic
{
public:
	typedef vector<instr_ptr>::const_iterator iterator;

	mnemonic(area a, string n, list<value_ptr> v = list<value_ptr>()) : addresses(a), opcode(n), operands(v) {};

	area addresses;
	string opcode;
	list<value_ptr> operands;

	string inspect(void) const 
	{ 
		auto i = operands.cbegin();
		stringstream ret;
		
		ret << opcode;
		while(i != operands.cend())
		{
			ret << " " << (*i)->inspect();
			if(++i != operands.cend())
				ret << ",";
		}

		return ret.str();
	};
};
#endif
