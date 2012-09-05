#include <sstream>

#include "mnemonic.hh"

bool operator==(const area &a, const area &b) { return a.isset == b.isset && (!a.isset || (a.begin == b.begin && a.end == b.end)); }
bool operator!=(const area &a, const area &b) { return !(a == b); }
bool operator<(const area &a, const area &b) { return a.begin < b.begin; }

name::name(string b) : base(b), subscript(-1) {};
name::name(const char *a) : base(string(a)), subscript(-1) {};
string name::inspect(void) const { return base + (subscript >=0 ? to_string(subscript) : ""); };
bool operator<(const name &a, const name &b) { return a.base < b.base; };
bool operator==(const name &a, const name &b) { return a.base == b.base; };
bool operator!=(const name &a, const name &b) { return !(a == b); };

value::value(void) {}
value::~value(void) {}

constant::constant(int v) : val(v) {}
string constant::inspect(void) const { return to_string(val); }

undefined::undefined(void) {}
string undefined::inspect(void) const { return "⊥"; }

variable::variable(name n) : nam(n) {}
variable::variable(string n) : nam(n) {}
string variable::inspect(void) const { return nam.inspect(); }

instr::instr(Opcode code, string opname, name var, vector<value_ptr> ops) 
: opcode(code), opname(opname), assigns(new variable(var)), operands(ops) {}
	
string instr::inspect(void) const 
{
	stringstream ss;

	ss << assigns->inspect() << " ≔ ";
	if(operands.size() == 0)
		ss << opname;
	else if(opcode == Call)
		ss << opname << "(" << operands[0]->inspect() << ")";
	else if(operands.size() == 1)
		ss << opname << operands[0]->inspect();
	else if(opcode == Phi)
		ss << opname << "(" << operands[0]->inspect() << "," << operands[1]->inspect() << ")";
	else if(operands.size() == 3)
		ss << operands[0]->inspect() << "[" << operands[1]->inspect() << ":" << operands[2]->inspect() << "]";
	else
		ss << operands[0]->inspect() << opname << operands[1]->inspect();
	return ss.str();
}
