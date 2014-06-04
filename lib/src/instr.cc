#include <panopticon/instr.hh>

using namespace po;

/*
string po::pretty(instr::Function fn)
{
	switch(fn)
	{
		case instr::And:		return " ∨ ";
		case instr::Or:			return " ∧ ";
		case instr::Xor:		return " ⊕ ";
		case instr::Not:		return "¬";
		case instr::Assign: return "";
		case instr::UShr:	return " ≫ ";
		case instr::UShl:	return " ≪ ";
		case instr::SShr:	return " ≫ₛ ";
		case instr::SShl:	return " ≪ₛ ";
		case instr::UExt:	return " ↤ᵤ ";
		case instr::SExt:	return " ↤ₛ ";
		case instr::Slice:	return ":";
		//case instr::Concat: return " ∷ ";
		case instr::Add:		return " + ";
		case instr::Sub:		return " - ";
		case instr::Mul:		return " × ";
		case instr::SDiv:	return " ÷ₛ ";
		case instr::UDiv:	return " ÷ᵤ ";
		case instr::SMod:	return " modₛ ";
		case instr::UMod:	return " modᵤ ";
		case instr::SLeq:	return " ≤ₛ ";
		case instr::ULeq:	return " ≤ᵤ ";
		case instr::Call:	return "call";
		case instr::Phi:		return "ϕ";
		default: assert(false);
	}

	return "";
}

string po::symbolic(instr::Function fn)
{
	switch(fn)
	{
		case instr::And:		return "and";
		case instr::Or:			return "or";
		case instr::Xor:		return "xor";
		case instr::Not:		return "not";
		case instr::Assign: return "assign";
		case instr::UShr:	return "u-shift-right";
		case instr::UShl:	return "i-shift-left";
		case instr::SShr:	return "s-shift-right";
		case instr::SShl:	return "s-shift-left";
		case instr::UExt:	return "u-extend";
		case instr::SExt:	return "s-extend";
		case instr::Slice:	return "slice";
		//case instr::Concat: return " ∷ ";
		case instr::Add:		return "add";
		case instr::Sub:		return "subtract";
		case instr::Mul:		return "multiply";
		case instr::SDiv:	return "s-divide";
		case instr::UDiv:	return "u-divide";
		case instr::SMod:	return "s-modulo";
		case instr::UMod:	return "u-modulo";
		case instr::SLeq:	return "s-less-equal";
		case instr::ULeq:	return "u-less-equal";
		case instr::Call:	return "call";
		case instr::Phi:		return "phi";
		default: assert(false);
	}

	return "";
}

instr::Function po::numeric(const std::string &s)
{
	if(s.substr(0,string(PO).size()) == string(PO))
	{
		string t = s.substr(string(PO).size());

		if(t == "and") return instr::And;
		if(t == "or") return instr::Or;
		if(t == "xor") return instr::Xor;
		if(t == "not") return instr::Not;
		if(t == "assign") return instr::Assign;
		if(t == "u-shift-right") return instr::UShr;
		if(t == "i-shift-left") return instr::UShl;
		if(t == "s-shift-right") return instr::SShr;
		if(t == "s-shift-left") return instr::SShl;
		if(t == "u-extend") return instr::UExt;
		if(t == "s-extend") return instr::SExt;
		if(t == "slice") return instr::Slice;
		//if(t == " ∷ ") return instr::Concat;
		if(t == "add") return instr::Add;
		if(t == "subtract") return instr::Sub;
		if(t == "multiply") return instr::Mul;
		if(t == "s-divide") return instr::SDiv;
		if(t == "u-divide") return instr::UDiv;
		if(t == "s-modulo") return instr::SMod;
		if(t == "u-modulo") return instr::UMod;
		if(t == "s-less-equal") return instr::SLeq;
		if(t == "u-less-equal") return instr::ULeq;
		if(t == "call") return instr::Call;
		if(t == "phi") return instr::Phi;
	}
	else
	{
		if(s == " ∨ ") return instr::And;
		if(s == " ∧ ") return instr::Or;
		if(s == " ⊕ ") return instr::Xor;
		if(s == "¬") return instr::Not;
		if(s == "") return instr::Assign;
		if(s == " ≫ ") return instr::UShr;
		if(s == " ≪ ") return instr::UShl;
		if(s == " ≫ₛ ") return instr::SShr;
		if(s == " ≪ₛ ") return instr::SShl;
		if(s == " ↤ᵤ ") return instr::UExt;
		if(s == " ↤ₛ ") return instr::SExt;
		if(s == ":") return instr::Slice;
		//if(s == " ∷ ") return instr::Concat;
		if(s == " + ") return instr::Add;
		if(s == " - ") return instr::Sub;
		if(s == " × ") return instr::Mul;
		if(s == " ÷ₛ ") return instr::SDiv;
		if(s == " ÷ᵤ ") return instr::UDiv;
		if(s == " modₛ ") return instr::SMod;
		if(s == " modᵤ ") return instr::UMod;
		if(s == " ≤ₛ ") return instr::SLeq;
		if(s == " ≤ᵤ ") return instr::ULeq;
		if(s == "call") return instr::Call;
		if(s == "ϕ") return instr::Phi;
	}

	assert(false);
	return instr::Assign;
}

ostream &po::operator<<(ostream &os, const instr &i)
{
	string fnname = pretty(i.function);

	os << i.left << " ≔ ";
	if(i.right.size() == 0)
		os << fnname;
	else if(i.function == instr::Call)
		os << fnname << "(" << i.right[0] << ")";
	else if(i.right.size() == 1)
		os << fnname << i.right[0];
	else if(i.function == instr::Phi)
		os << fnname << "(" << i.right[0] << ", " << i.right[1] << ")";
	else if(i.function == instr::Slice)
		os << i.right[0] << "[" << i.right[1] << fnname << i.right[2] << "]";
	else if(i.right.size() == 3)
		os << fnname << "(" << i.right[0] << ", " << i.right[1] << ", " << i.right[2] << ")";
	else
		os << i.right[0] << fnname << i.right[1];
	return os;
}
*/
