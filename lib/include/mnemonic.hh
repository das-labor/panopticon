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
typedef shared_ptr<struct lvalue> lvalue_ptr;
typedef shared_ptr<struct variable> var_ptr;
typedef shared_ptr<const struct variable> var_cptr;
typedef shared_ptr<struct constant> const_ptr;
typedef shared_ptr<const struct constant> const_cptr;
typedef shared_ptr<struct memory> mem_ptr;
typedef shared_ptr<const struct memory> mem_cptr;
typedef shared_ptr<struct instr> instr_ptr;
typedef shared_ptr<const struct instr> instr_cptr;
typedef shared_ptr<class mnemonic> mne_ptr;
typedef shared_ptr<const class mnemonic> mne_cptr;

template<typename T>
struct range
{
	range(void) { begin = end; };
	range(T b, T e) : begin(b), end(e) { assert(begin <= end); };

	size_t size(void) const { return end - begin; };
	bool includes(const range<T> &a) const { return size() && a.size() && begin <= a.begin && end > a.end; };
	bool includes(T a) const { return size() && begin <= a && end > a; };
	bool overlap(const range<T> &a) const { return size() && a.size() && !(begin >= a.end || end <= a.begin); };
	T last(void) const { return size() ? end - 1 : begin; };
	
	bool operator==(const range<T> &b) const { return size() == b.size() && (!size() || (begin == b.begin && end == b.end)); };
	bool operator!=(const range<T> &b) const { return !(*this == b); };
	bool operator<(const range<T> &b) const { return begin < b.begin; };
	ostream& operator<<(ostream &os) const 
	{ 
		if(size())
		{
			os << begin;
			if(size() > 1)
				os << "," << end-1;
		}
		else
			os << "nil";
		return os; 
	};

	T begin;
	T end;
};

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
struct value 
{
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
	virtual string inspect(void) const;
};

struct lvalue : public value
{
protected:
	lvalue(void);
};

struct variable : public lvalue
{
	variable(name nam, unsigned int w);
	variable(name nam, unsigned int f, unsigned int l);
	unsigned int mask(void) const;
	virtual string inspect(void) const;
	
	name nam;
	range<unsigned int> slice;
};

struct memory : public lvalue
{
	enum Endianess
	{
		None = 0,
		Little = 1,
		Big = 2,
	};

	memory(value_ptr o, unsigned int w, Endianess e, string n);
	virtual string inspect(void) const;

	value_ptr offset;			// from start of the memory space
	unsigned int bytes;		// # bytes to read from offset
	Endianess endianess;	// byte order
	string name;					// descriptive name of the memory space (stack, flash, ...)
};

struct declaration
{
	virtual lvalue_ptr instantiate(void) const = 0;
};

struct lvalproxy
{
	lvalproxy(const declaration &d);
	lvalproxy(var_ptr v);
	lvalproxy(mem_ptr m);
	
	lvalue_ptr value;
};

struct valproxy
{
	valproxy(int i);
	valproxy(nullptr_t i);
	valproxy(value_ptr v);	
	valproxy(const declaration &d);
	valproxy(var_ptr v);
	valproxy(mem_ptr m);
	valproxy(lvalue_ptr v);
	valproxy(lvalproxy v);

	value_ptr value;
};

struct variable_decl : public declaration
{
	variable_decl(string n, unsigned int w);
	var_ptr operator()(unsigned int b = 0, unsigned int e = 0) const;
	virtual lvalue_ptr instantiate(void) const;

	string name;
	unsigned int width;
};

struct memory_decl
{
	memory_decl(string n, memory::Endianess e, unsigned int b);
	mem_ptr operator()(valproxy v, unsigned int b = 0, memory::Endianess e = memory::None) const;
	virtual lvalue_ptr instantiate(void) const;

	string name;
	memory::Endianess endianess;
	unsigned int bytes;
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

	instr(Function fn, string fnname, lvalue_ptr a, vector<value_ptr> args);
	
	string inspect(void) const;
		
	Function function;
	string fnname;
	lvalue_ptr assigns;
	vector<value_ptr> arguments;
};

class mnemonic
{
public:
	typedef vector<instr_ptr>::const_iterator iterator;
	
	mnemonic(range<addr_t> a, string n, list<value_ptr> v = list<value_ptr>());
	virtual string inspect(void) const;

	range<addr_t> area;
	string opcode;
	list<value_ptr> operands;
};
#endif
