#ifndef CODE_GENERATOR_HH
#define CODE_GENERATOR_HH

#include <cstring>

#include "mnemonic.hh"
#include "architecture.hh"

template<typename T>
class code_generator
{
public:
	code_generator(std::insert_iterator<std::list<instr_ptr>> i) : inserter(i) {};

	/*pair<bblock_ptr,bblock_ptr> branch(valproxy, guard::Relation, valproxy, std::function<void(cgen_ptr)>, std::function<void(cgen_ptr)> = std::function<void(cgen_ptr)>());
	pair<bblock_ptr,bblock_ptr> branch(guard_ptr g, std::function<void(cgen_ptr)> true_fn, std::function<void(cgen_ptr)> false_fn = std::function<void(cgen_ptr)>());
	bblock_ptr jump(bblock_ptr bb, valproxy a, guard::Relation rel, valproxy b);
	bblock_ptr jump(bblock_ptr bb, guard_ptr = guard_ptr(new guard()));
	void indirect_jump(valproxy tgt, valproxy a, guard::Relation rel, valproxy b);
	void indirect_jump(valproxy tgt, guard_ptr = guard_ptr(new guard()));*/

	// named
	lvalue_ptr and_b(lvalproxy a, valproxy op1, valproxy op2) 		{ return named(instr::And,	 " ∨ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr or_b(lvalproxy a, valproxy op1, valproxy op2) 		{ return named(instr::Or,		 " ∧ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr xor_b(lvalproxy a, valproxy op1, valproxy op2)	 	{ return named(instr::Xor,	 " ⊕ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr not_b(lvalproxy a, valproxy op)										{ return named(instr::Not,	 "¬",	 a.value,{op.value}); };
	lvalue_ptr assign(lvalproxy a, valproxy op)									{ return named(instr::Assign,"",	 a.value,{op.value}); };
	lvalue_ptr undef(lvalproxy a)																{ return named(instr::Assign,"",	 a.value,{lvalue_ptr(new undefined())}); };
	lvalue_ptr shiftr_u(lvalproxy a, valproxy cnt, valproxy op)	{ return named(instr::UShr,	 " ≫ ",a.value,{cnt.value,op.value}); };
	lvalue_ptr shiftl_u(lvalproxy a, valproxy cnt, valproxy op)	{ return named(instr::UShl,	 " ≪ ",a.value,{cnt.value,op.value}); };
	lvalue_ptr shiftr_s(lvalproxy a, valproxy cnt, valproxy op)	{ return named(instr::SShr,	 " ≫ₛ ",a.value,{cnt.value,op.value}); };
	lvalue_ptr shiftl_s(lvalproxy a, valproxy cnt, valproxy op)	{ return named(instr::SShl,	 " ≪ₛ ",a.value,{cnt.value,op.value}); };
	lvalue_ptr ext_u(lvalproxy a, valproxy cnt, valproxy op)			{ return named(instr::UExt,	 " ↤ᵤ ",a.value,{cnt.value,op.value}); };
	lvalue_ptr ext_s(lvalproxy a, valproxy cnt, valproxy op)			{ return named(instr::SExt,	 " ↤ₛ ",a.value,{cnt.value,op.value}); };
	lvalue_ptr slice(lvalproxy a, valproxy op, valproxy from, valproxy to)		{ return named(instr::Slice,":",a.value,{op.value,from.value,to.value}); };
	lvalue_ptr concat(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::Concat," ∷ ", a.value,{op1.value,op2.value}); };
	lvalue_ptr add_i(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::Add,	 " + ",a.value,{op1.value,op2.value}); };
	lvalue_ptr sub_i(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::Sub,	 " - ",a.value,{op1.value,op2.value}); };
	lvalue_ptr mul_i(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::Mul,	 " × ",a.value,{op1.value,op2.value}); };
	lvalue_ptr div_is(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::SDiv,	 " ÷ₛ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr div_iu(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::UDiv,	 " ÷ᵤ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr mod_is(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::SMod,	 " modₛ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr mod_iu(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::UMod,	 " modᵤ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr leq_is(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::SLeq,	 " ≤ₛ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr leq_iu(lvalproxy a, valproxy op1, valproxy op2)		{ return named(instr::ULeq,	 " ≤ᵤ ",a.value,{op1.value,op2.value}); };
	lvalue_ptr call(lvalproxy a, valproxy op)										{ return named(instr::Call,	 "call",a.value,{op.value}); };

	// anonymous
	lvalue_ptr and_b(valproxy op1, valproxy op2) 	{ return anonymous(instr::And,	 " ∨ ",{op1.value,op2.value}); };
	lvalue_ptr or_b(valproxy op1, valproxy op2) 		{ return anonymous(instr::Or,		 " ∧ ",{op1.value,op2.value}); };
	lvalue_ptr xor_b(valproxy op1, valproxy op2)	 	{ return anonymous(instr::Xor,	 " ⊕ ",{op1.value,op2.value}); };
	lvalue_ptr not_b(valproxy op)									{ return anonymous(instr::Not,	 "¬",	 {op.value}); };
	lvalue_ptr assign(valproxy op)									{ return anonymous(instr::Assign,"",	 {op.value}); };
	lvalue_ptr shiftr_u(valproxy cnt, valproxy op)	{ return anonymous(instr::UShr,	 " ≫ ",{cnt.value,op.value}); };
	lvalue_ptr shiftl_u(valproxy cnt, valproxy op)	{ return anonymous(instr::UShl,	 " ≪ ",{cnt.value,op.value}); };
	lvalue_ptr shiftr_s(valproxy cnt, valproxy op)	{ return anonymous(instr::SShr,	 " ≫ₛ ",{cnt.value,op.value}); };
	lvalue_ptr shiftl_s(valproxy cnt, valproxy op)	{ return anonymous(instr::SShl,	 " ≪ₛ ",{cnt.value,op.value}); };
	lvalue_ptr ext_u(valproxy cnt, valproxy op)		{ return anonymous(instr::UExt,	 " ↤ᵤ ",{cnt.value,op.value}); };
	lvalue_ptr ext_s(valproxy cnt, valproxy op)		{ return anonymous(instr::SExt,	 " ↤ₛ ",{cnt.value,op.value}); };
	lvalue_ptr slice(valproxy op, valproxy from, valproxy to)		{ return anonymous(instr::Slice,":",{op.value,from.value,to.value}); };
	lvalue_ptr concat(valproxy op1, valproxy op2)	{ return anonymous(instr::Concat," ∷ ",{op1.value,op2.value}); };
	lvalue_ptr add_i(valproxy op1, valproxy op2)		{ return anonymous(instr::Add,	 " + ",{op1.value,op2.value}); };
	lvalue_ptr sub_i(valproxy op1, valproxy op2)		{ return anonymous(instr::Sub,	 " - ",{op1.value,op2.value}); };
	lvalue_ptr mul_i(valproxy op1, valproxy op2)		{ return anonymous(instr::Mul,	 " × ",{op1.value,op2.value}); };
	lvalue_ptr div_is(valproxy op1, valproxy op2)	{ return anonymous(instr::SDiv,	 " ÷ₛ ",{op1.value,op2.value}); };
	lvalue_ptr div_iu(valproxy op1, valproxy op2)	{ return anonymous(instr::UDiv,	 " ÷ᵤ ",{op1.value,op2.value}); };
	lvalue_ptr mod_is(valproxy op1, valproxy op2)	{ return anonymous(instr::SMod,	 " modₛ ",{op1.value,op2.value}); };
	lvalue_ptr mod_iu(valproxy op1, valproxy op2)	{ return anonymous(instr::UMod,	 " modᵤ ",{op1.value,op2.value}); };
	lvalue_ptr leq_is(valproxy op1, valproxy op2)	{ return anonymous(instr::SLeq,	 " ≤ₛ ",{op1.value,op2.value}); };
	lvalue_ptr leq_iu(valproxy op1, valproxy op2)	{ return anonymous(instr::ULeq,	 " ≤ᵤ ",{op1.value,op2.value}); };
	lvalue_ptr call(valproxy op)										{ return anonymous(instr::Call,	 "call",{op.value}); };	

protected:
	// width infered by architecure::width(name)
	lvalue_ptr named(instr::Function fn, string fnam, lvalue_ptr assign, vector<value_ptr> arguments)
	{
		assert(assign);

		auto sanity_check = [](const value_ptr &v)
		{
			var_cptr w;
			mem_cptr m;
	
			if((w = dynamic_pointer_cast<const variable>(v)))
				return w->nam.base.size() && w->nam.subscript == -1 && w->slice.size();
			else if((m = dynamic_pointer_cast<const memory>(v)))
				return m->name.size() && m->bytes && (m->endianess == memory::Big || m->endianess == memory::Little) && m->offset && m->offset != v;
			else 
				return dynamic_pointer_cast<undefined>(v) || dynamic_pointer_cast<constant>(v);
		};

		assert(all_of(arguments.begin(),arguments.end(),sanity_check) && sanity_check(assign));
				
		instr_ptr ret(new instr(fn,fnam,assign,arguments));
		inserter = ret;
		return assign;
	};

	lvalue_ptr anonymous(instr::Function fn, string fnam, vector<value_ptr> proxies)
	{
		return named(fn,fnam,temporary(tag),proxies);
	};

	static unsigned int next;
	std::insert_iterator<std::list<instr_ptr>> inserter;
	T tag;
};

#endif
