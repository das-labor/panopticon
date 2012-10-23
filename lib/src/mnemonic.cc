#include <sstream>
#include <algorithm>
#include <cstring> // ffs

#include "mnemonic.hh"

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

value::value(void) {}

constant::constant(int v) : val(v) {}
string constant::inspect(void) const { return to_string(val); }

bool operator<(const constant &a, const constant &b) { return a.val < b.val; }
bool operator==(const constant &a, const constant &b) { return a.val == b.val; }
bool operator!=(const constant &a, const constant &b) { return !(a == b); }

string undefined::inspect(void) const { return "⊥"; }

lvalue::lvalue(void) {}

variable::variable(name n, unsigned int w) : nam(n), slice(0,w) {}
variable::variable(name n, unsigned int b, unsigned int e) : nam(n), slice(b,e) {}
unsigned int variable::mask(void) const { return (~((1 << slice.begin) - 1)) & ((1 << slice.end) - 1); }
string variable::inspect(void) const 
{ 
	string ret = nam.inspect() + "\\{";
	
	if(slice.size() == 1)
		ret += to_string(slice.begin);
	else if(slice.size() > 1)
		ret += to_string(slice.begin) + "," + to_string(slice.end-1);
	return ret + "\\}";
}

memory::memory(value_ptr o, unsigned int w, Endianess e, string n)
: offset(o), bytes(w), endianess(e), name(n) {}
string memory::inspect(void) const 
{ 
	string ret = name + "[" + offset->inspect() + ";" + to_string(bytes);

	switch(endianess)
	{
		case Little: ret += "←"; break;
		case Big: ret += "→"; break;
		default: ret += "?"; break;
	}

	return ret + "]";
}

variable_decl::variable_decl(string n, unsigned int w)
: name(n), width(w) { assert(w); };
lvalue_ptr variable_decl::instantiate(void) const { return lvalue_ptr(new variable(name,width)); }
var_ptr variable_decl::operator()(unsigned int b, unsigned int e) const
{
	var_ptr v = dynamic_pointer_cast<variable>(instantiate());
	
	assert(v);
	v->slice = range<unsigned int>(b,e+1);
	
	return v;
}

memory_decl::memory_decl(string n, memory::Endianess e, unsigned int b)
: name(n), endianess(e), bytes(b) {}
lvalue_ptr memory_decl::instantiate(void) const { return lvalue_ptr(new memory(value_ptr(new constant(0)),bytes,endianess,name)); }
mem_ptr memory_decl::operator()(valproxy v, unsigned int b, memory::Endianess e) const
{
	mem_ptr m = dynamic_pointer_cast<memory>(instantiate());

	assert(m);
	m->offset = v.value;
	if(b) m->bytes = b;
	if(e != memory::None) m->endianess = e;
	
	return m;
}
	
lvalproxy::lvalproxy(const declaration &d) : value(d.instantiate()) {}
lvalproxy::lvalproxy(var_ptr v) : value(v) {}
lvalproxy::lvalproxy(mem_ptr m) : value(m) {}

valproxy::valproxy(int i)	: value(value_ptr(new constant(i))) {}
valproxy::valproxy(nullptr_t i) : value(value_ptr(new undefined())) {}
valproxy::valproxy(value_ptr v) : value(v) {}
valproxy::valproxy(const declaration &d) : value(d.instantiate()) {}
valproxy::valproxy(var_ptr v) : value(v) {}
valproxy::valproxy(mem_ptr m) : value(m) {}
valproxy::valproxy(lvalue_ptr v) : value(v) {}
valproxy::valproxy(lvalproxy v) : value(v.value) {}

instr::instr(Function fn, string fnam, lvalue_ptr var, vector<value_ptr> args)
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
		ss << fnname << "(" << arguments[0]->inspect() << ", " << arguments[1]->inspect() << ")";
	else if(function == Slice)
		ss << arguments[0]->inspect() << "[" << arguments[1]->inspect() << ":" << arguments[2]->inspect() << "]";
	else if(arguments.size() == 3)
		ss << fnname << "(" << arguments[0]->inspect() << ", " << arguments[1]->inspect() << ", " << arguments[2]->inspect() << ")";
	else
		ss << arguments[0]->inspect() << fnname << arguments[1]->inspect();
	return ss.str();
}

mnemonic::mnemonic(range<addr_t> a, string n, list<value_ptr> v)
: area(a), opcode(n), operands(v) {}

string mnemonic::inspect(void) const 
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
}
