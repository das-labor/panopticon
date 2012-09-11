#include <sstream>
#include <algorithm>

#include "mnemonic.hh"

unsigned int instr_builder::next = 0;

bool operator==(const area &a, const area &b) { return a.isset == b.isset && (!a.isset || (a.begin == b.begin && a.end == b.end)); }
bool operator!=(const area &a, const area &b) { return !(a == b); }
bool operator<(const area &a, const area &b) { return a.begin < b.begin; }

name::name(string b) : base(b), subscript(-1) {};
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
value::~value(void) {}

constant::constant(int v) : val(v) {}
string constant::inspect(void) const { return to_string(val); }

bool operator<(const constant &a, const constant &b) { return a.val < b.val; }
bool operator==(const constant &a, const constant &b) { return a.val == b.val; }
bool operator!=(const constant &a, const constant &b) { return !(a == b); }

undefined::undefined(void) {}
string undefined::inspect(void) const { return "⊥"; }

variable::variable(name n) : nam(n) {}
variable::variable(string n) : nam(n) {}
string variable::inspect(void) const { return nam.inspect(); }

// named
value_ptr instr_builder::and_b(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::And,		" ∨ ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::or_b(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Or,		" ∧ ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::xor_b(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Xor,		" ⊕ ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::phi(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Phi,		"ϕ",			a,{op1.value,op2.value}))); };
value_ptr instr_builder::not_b(name a, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::Not,		"¬",			a,{op.value}))); };
value_ptr instr_builder::assign(name a, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::Assign,"",				a,{op.value}))); };
value_ptr instr_builder::undef(name a)
	{ return accept_instr(instr_ptr(new instr(instr::Assign,"",				a,{value_ptr(new undefined)}))); };
value_ptr instr_builder::shiftr_u(name a, valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::UShr,	" ≫ ",		a,{cnt.value,op.value}))); };
value_ptr instr_builder::shiftl_u(name a, valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::UShl,	" ≪ ",		a,{cnt.value,op.value}))); };
value_ptr instr_builder::shiftr_s(name a, valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::SShr,	" ≫ₛ ",		a,{cnt.value,op.value}))); };
value_ptr instr_builder::shiftl_s(name a, valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::SShl,	" ≪ₛ ",		a,{cnt.value,op.value}))); };
value_ptr instr_builder::ext_u(name a, valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::UExt,	" ↤ᵤ ",		a,{cnt.value,op.value}))); };
value_ptr instr_builder::ext_s(name a, valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::SExt,	" ↤ₛ ",		a,{cnt.value,op.value}))); };
value_ptr instr_builder::slice(name a, valproxy op, valproxy from, valproxy to)
	{ return accept_instr(instr_ptr(new instr(instr::Slice,	":",a,{op.value,from.value,to.value}))); };
value_ptr instr_builder::concat(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Concat," ∷ ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::add_i(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Add,		" + ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::sub_i(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Sub,		" - ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::mul_i(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Mul,		" × ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::div_is(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::SDiv,	" ÷ₛ ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::div_iu(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::UDiv,	" ÷ᵤ ",		a,{op1.value,op2.value}))); };
value_ptr instr_builder::mod_is(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::SMod,	" modₛ ",	a,{op1.value,op2.value}))); };
value_ptr instr_builder::mod_iu(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::UMod,	" modᵤ ",	a,{op1.value,op2.value}))); };
value_ptr instr_builder::leq_is(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::SLeq,	" ≤ₛ ",	a,{op1.value,op2.value}))); };
value_ptr instr_builder::leq_iu(name a, valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::ULeq,	" ≤ᵤ ",	a,{op1.value,op2.value}))); };
value_ptr instr_builder::call(name a, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::Call,	"call",	a,{op.value}))); };

// anonymous
value_ptr instr_builder::and_b(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::And,		" ∨ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::or_b(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Or,		" ∧ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::xor_b(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Xor,		" ⊕ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::phi(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Phi,		"ϕ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::not_b(valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::Not,		"¬",name("t"+to_string(next++)),{op.value}))); };
value_ptr instr_builder::assign(valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::Assign,"",name("t"+to_string(next++)),{op.value}))); };
value_ptr instr_builder::undef(void)
	{ return accept_instr(instr_ptr(new instr(instr::Assign,"",name("t"+to_string(next++)),{value_ptr(new undefined)}))); };
value_ptr instr_builder::shiftr_u(valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::UShr,	" ≫ ",name("t"+to_string(next++)),{cnt.value,op.value}))); };
value_ptr instr_builder::shiftl_u(valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::UShl,	" ≪ ",name("t"+to_string(next++)),{cnt.value,op.value}))); };
value_ptr instr_builder::shiftr_s(valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::SShr,	" ≫ₛ ",name("t"+to_string(next++)),{cnt.value,op.value}))); };
value_ptr instr_builder::shiftl_s(valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::SShl,	" ≪ₛ ",name("t"+to_string(next++)),{cnt.value,op.value}))); };
value_ptr instr_builder::ext_u(valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::UExt,	" ↤ᵤ ",name("t"+to_string(next++)),{cnt.value,op.value}))); };
value_ptr instr_builder::ext_s(valproxy cnt, valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::SExt,	" ↤ₛ ",name("t"+to_string(next++)),{cnt.value,op.value}))); };
value_ptr instr_builder::slice(valproxy op, valproxy from, valproxy to)
	{ return accept_instr(instr_ptr(new instr(instr::Slice,	":",name("t"+to_string(next++)),{op.value,from.value,to.value}))); };
value_ptr instr_builder::concat(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Concat," ∷ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::add_i(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Add,		" + ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::sub_i(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Sub,		" - ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::mul_i(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::Mul,		" × ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::div_is(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::SDiv,	" ÷ₛ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::div_iu(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::UDiv,	" ÷ᵤ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::mod_is(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::SMod,	" modₛ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::mod_iu(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::UMod,	" modᵤ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::leq_is(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::SLeq,	" ≤ₛ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::leq_iu(valproxy op1, valproxy op2)
	{ return accept_instr(instr_ptr(new instr(instr::ULeq,	" ≤ᵤ ",name("t"+to_string(next++)),{op1.value,op2.value}))); };
value_ptr instr_builder::call(valproxy op)
	{ return accept_instr(instr_ptr(new instr(instr::Call,	"call",name("t"+to_string(next++)),{op.value}))); };

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
