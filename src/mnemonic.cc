#include <sstream>
#include <algorithm>
#include <cstring> // ffs

#include "mnemonic.hh"

bool operator==(const area &a, const area &b) { return a.size() == b.size() && (!a.size() || (a.begin == b.begin && a.end == b.end)); }
bool operator!=(const area &a, const area &b) { return !(a == b); }
bool operator<(const area &a, const area &b) { return a.begin < b.begin; }
ostream& operator<<(ostream &os, const area &a) 
{ 
	if(a.size())
	{
		os << hex << a.begin;
		if(a.size() > 1)
			os << "-" << a.end-1;
		os << dec;
	}
	else
		os << "NIL";
	return os; 
}

name::name(string b) : base(b), subscript(-1) {};
name::name(string b, int i) : base(b), subscript(i) {};
name::name(const char *a) : base(string(a)), subscript(-1) {};
string name::inspect(void) const 
{ 
	stringstream ss;
	
	ss << base;

	if(subscript >= 0)
	{
		string t = to_string(subscript);

		for_each(t.cbegin(),t.cend(),[&ss](const char c)
		{
			switch(c)
			{
				case '0': ss << "₀"; break;
				case '1': ss << "₁"; break;
				case '2': ss << "₂"; break;
				case '3': ss << "₃"; break;
				case '4': ss << "₄"; break;
				case '5': ss << "₅"; break;
				case '6': ss << "₆"; break;
				case '7': ss << "₇"; break;
				case '8': ss << "₈"; break;
				case '9': ss << "₉"; break;
				default: ;
			}
		});
	}

	return ss.str();
}

bool operator<(const name &a, const name &b) 
{ 
	if(a.base != b.base)
		return a.base < b.base;
	else
		return a.subscript < b.subscript;
};

bool operator==(const name &a, const name &b) { return a.base == b.base && a.subscript == b.subscript; }
bool operator!=(const name &a, const name &b) { return !(a == b); }

value::value(unsigned int w) : width(w) {}
value::~value(void) {}

constant::constant(int v, unsigned int w) : value(w), val(v) {}
string constant::inspect(void) const { return to_string(val); }

bool operator<(const constant &a, const constant &b) { return a.val < b.val; }
bool operator==(const constant &a, const constant &b) { return a.val == b.val; }
bool operator!=(const constant &a, const constant &b) { return !(a == b); }

undefined::undefined(unsigned int w) : value(w) {}
string undefined::inspect(void) const { return "⊥"; }

variable::variable(name n, unsigned int w) : value(w), nam(n) {}
variable::variable(string n, unsigned int w) : value(w), nam(n) {}
string variable::inspect(void) const { return nam.inspect(); }
/*
address::address(unsigned int o, unsigned int w, string n) : offset(o), width(w), name(n) {};
address::address(unsigned int w, string n) : offset(0), width(w), name(n) {};
string address::inspect(void) const { return name + "[" + to_string(offset) + "]"; };
*/

instr::instr(Function fn, string fnam, var_ptr var, vector<value_ptr> args)
: function(fn), fnname(fnam), assigns(var), arguments(args) {}
	
string instr::inspect(void) const 
{
	stringstream ss;

	ss << assigns->inspect() << " ≔ ";
	if(arguments.size() == 0)
		ss << fnname;
	else if(function == Call)
		ss << fnname << "(" << arguments[0]->inspect() << ")";
	else if(arguments.size() == 1)
		ss << fnname << arguments[0]->inspect();
	else if(function == Phi)
		ss << fnname << "(" << arguments[0]->inspect() << "," << arguments[1]->inspect() << ")";
	else if(arguments.size() == 3)
		ss << arguments[0]->inspect() << "[" << arguments[1]->inspect() << ":" << arguments[2]->inspect() << "]";
	else
		ss << arguments[0]->inspect() << fnname << arguments[1]->inspect();
	return ss.str();
}
