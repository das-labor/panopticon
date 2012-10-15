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
	value_ptr and_b(name a, valproxy op1, valproxy op2) 	{ return named(instr::And,	 " ∨ ",a,{op1.value,op2.value}); };
	value_ptr or_b(name a, valproxy op1, valproxy op2) 		{ return named(instr::Or,		 " ∧ ",a,{op1.value,op2.value}); };
	value_ptr xor_b(name a, valproxy op1, valproxy op2) 	{ return named(instr::Xor,	 " ⊕ ",a,{op1.value,op2.value}); };
	value_ptr not_b(name a, valproxy op)									{ return named(instr::Not,	 "¬",	 a,{op.value}); };
	value_ptr assign(name a, valproxy op)									{ return named(instr::Assign,"",	 a,{op.value}); };
	value_ptr undef(name a)																{ return named(instr::Assign,"",	 a,{value_ptr(new undefined(0))}); };
	value_ptr shiftr_u(name a, valproxy cnt, valproxy op)	{ return named(instr::UShr,	 " ≫ ",a,{cnt.value,op.value}); };
	value_ptr shiftl_u(name a, valproxy cnt, valproxy op)	{ return named(instr::UShl,	 " ≪ ",a,{cnt.value,op.value}); };
	value_ptr shiftr_s(name a, valproxy cnt, valproxy op)	{ return named(instr::SShr,	 " ≫ₛ ",a,{cnt.value,op.value}); };
	value_ptr shiftl_s(name a, valproxy cnt, valproxy op)	{ return named(instr::SShl,	 " ≪ₛ ",a,{cnt.value,op.value}); };
	value_ptr ext_u(name a, valproxy cnt, valproxy op)		{ return named(instr::UExt,	 " ↤ᵤ ",a,{cnt.value,op.value}); };
	value_ptr ext_s(name a, valproxy cnt, valproxy op)		{ return named(instr::SExt,	 " ↤ₛ ",a,{cnt.value,op.value}); };
	value_ptr slice(name a, valproxy op, valproxy from, valproxy to)		{ return named(instr::Slice,":",a,{op.value,from.value,to.value}); };
	value_ptr concat(name a, valproxy op1, valproxy op2)	{ return named(instr::Concat," ∷ ", a,{op1.value,op2.value}); };
	value_ptr add_i(name a, valproxy op1, valproxy op2)		{ return named(instr::Add,	 " + ",a,{op1.value,op2.value}); };
	value_ptr sub_i(name a, valproxy op1, valproxy op2)		{ return named(instr::Sub,	 " - ",a,{op1.value,op2.value}); };
	value_ptr mul_i(name a, valproxy op1, valproxy op2)		{ return named(instr::Mul,	 " × ",a,{op1.value,op2.value}); };
	value_ptr div_is(name a, valproxy op1, valproxy op2)	{ return named(instr::SDiv,	 " ÷ₛ ",a,{op1.value,op2.value}); };
	value_ptr div_iu(name a, valproxy op1, valproxy op2)	{ return named(instr::UDiv,	 " ÷ᵤ ",a,{op1.value,op2.value}); };
	value_ptr mod_is(name a, valproxy op1, valproxy op2)	{ return named(instr::SMod,	 " modₛ ",a,{op1.value,op2.value}); };
	value_ptr mod_iu(name a, valproxy op1, valproxy op2)	{ return named(instr::UMod,	 " modᵤ ",a,{op1.value,op2.value}); };
	value_ptr leq_is(name a, valproxy op1, valproxy op2)	{ return named(instr::SLeq,	 " ≤ₛ ",a,{op1.value,op2.value}); };
	value_ptr leq_iu(name a, valproxy op1, valproxy op2)	{ return named(instr::ULeq,	 " ≤ᵤ ",a,{op1.value,op2.value}); };
	value_ptr call(name a, valproxy op)										{ return named(instr::Call,	 "call",a,{op.value}); };
	value_ptr store(name a, valproxy addr, valproxy bank, valproxy op)	{ return named(instr::Store,"store",a,{addr.value,bank.value,op.value}); };
	value_ptr load(name a, valproxy addr, valproxy bank)	{ return named(instr::Load,"load",a,{addr.value,bank.value}); };

	// anonymous
	value_ptr and_b(valproxy op1, valproxy op2) 	{ return anonymous(instr::And,	 " ∨ ",{op1.value,op2.value}); };
	value_ptr or_b(valproxy op1, valproxy op2) 		{ return anonymous(instr::Or,		 " ∧ ",{op1.value,op2.value}); };
	value_ptr xor_b(valproxy op1, valproxy op2)	 	{ return anonymous(instr::Xor,	 " ⊕ ",{op1.value,op2.value}); };
	value_ptr not_b(valproxy op)									{ return anonymous(instr::Not,	 "¬",	 {op.value}); };
	value_ptr assign(valproxy op)									{ return anonymous(instr::Assign,"",	 {op.value}); };
	value_ptr shiftr_u(valproxy cnt, valproxy op)	{ return anonymous(instr::UShr,	 " ≫ ",{cnt.value,op.value}); };
	value_ptr shiftl_u(valproxy cnt, valproxy op)	{ return anonymous(instr::UShl,	 " ≪ ",{cnt.value,op.value}); };
	value_ptr shiftr_s(valproxy cnt, valproxy op)	{ return anonymous(instr::SShr,	 " ≫ₛ ",{cnt.value,op.value}); };
	value_ptr shiftl_s(valproxy cnt, valproxy op)	{ return anonymous(instr::SShl,	 " ≪ₛ ",{cnt.value,op.value}); };
	value_ptr ext_u(valproxy cnt, valproxy op)		{ return anonymous(instr::UExt,	 " ↤ᵤ ",{cnt.value,op.value}); };
	value_ptr ext_s(valproxy cnt, valproxy op)		{ return anonymous(instr::SExt,	 " ↤ₛ ",{cnt.value,op.value}); };
	value_ptr slice(valproxy op, valproxy from, valproxy to)		{ return anonymous(instr::Slice,":",{op.value,from.value,to.value}); };
	value_ptr concat(valproxy op1, valproxy op2)	{ return anonymous(instr::Concat," ∷ ",{op1.value,op2.value}); };
	value_ptr add_i(valproxy op1, valproxy op2)		{ return anonymous(instr::Add,	 " + ",{op1.value,op2.value}); };
	value_ptr sub_i(valproxy op1, valproxy op2)		{ return anonymous(instr::Sub,	 " - ",{op1.value,op2.value}); };
	value_ptr mul_i(valproxy op1, valproxy op2)		{ return anonymous(instr::Mul,	 " × ",{op1.value,op2.value}); };
	value_ptr div_is(valproxy op1, valproxy op2)	{ return anonymous(instr::SDiv,	 " ÷ₛ ",{op1.value,op2.value}); };
	value_ptr div_iu(valproxy op1, valproxy op2)	{ return anonymous(instr::UDiv,	 " ÷ᵤ ",{op1.value,op2.value}); };
	value_ptr mod_is(valproxy op1, valproxy op2)	{ return anonymous(instr::SMod,	 " modₛ ",{op1.value,op2.value}); };
	value_ptr mod_iu(valproxy op1, valproxy op2)	{ return anonymous(instr::UMod,	 " modᵤ ",{op1.value,op2.value}); };
	value_ptr leq_is(valproxy op1, valproxy op2)	{ return anonymous(instr::SLeq,	 " ≤ₛ ",{op1.value,op2.value}); };
	value_ptr leq_iu(valproxy op1, valproxy op2)	{ return anonymous(instr::ULeq,	 " ≤ᵤ ",{op1.value,op2.value}); };
	value_ptr call(valproxy op)										{ return anonymous(instr::Call,	 "call",{op.value}); };	
	value_ptr store(valproxy addr, valproxy bank, valproxy op)	{ return anonymous(instr::Store,"store",{addr.value,bank.value,op.value}); };
	value_ptr load(valproxy addr, valproxy bank)	{ return anonymous(instr::Load,"load",{addr.value,bank.value}); };

protected:
	// width infered by architecure::width(name)
	value_ptr named(instr::Function fn, string fnam, name nam, list<value_ptr> proxies)
	{
		vector<value_ptr> arguments;
		unsigned int w = 0;

		transform(proxies.begin(),proxies.end(),std::inserter(arguments,arguments.end()),[&](value_ptr v)
		{ 
			if(v->width || (v->width = width(tag,v)))
				return v;
			else
				assert(false);
				//throw type_exception("Value " + v->inspect() + " has unknown width");
		});

		if(fn == instr::Slice)
		{
			assert(arguments.size() == 3);
			shared_ptr<constant> from, to;

			from = dynamic_pointer_cast<constant>(arguments[1]);
			to = dynamic_pointer_cast<constant>(arguments[2]);

			// slice accepts only constant args (for now)
			assert(from && to && from->val <= to->val && to->val <= arguments[0]->width);
			w = to->val - from->val + 1;
		}
		else if(fn == instr::Concat)
		{
			w = accumulate(arguments.cbegin(),arguments.cend(),(unsigned int)0,[](unsigned int acc, const value_ptr v) { return acc + v->width; });
		}
		else
		{
			assert(all_of(arguments.begin(),arguments.end(),[&](const value_ptr &v) 
			{ 
				if(!w) w = v->width;
				return w == v->width;
			}));
		}

		assert(w);

		var_ptr v(new variable(nam,w));
		if(valid(tag,nam))
			assert(!width(tag,v) || width(tag,v) == w);

		instr_ptr ret(new instr(fn,fnam,v,arguments));
		inserter = ret;
		return ret->assigns;
	};

	value_ptr anonymous(instr::Function fn, string fnam, list<value_ptr> proxies)
	{
		return named(fn,fnam,unused(tag),proxies);
	};

	static unsigned int next;
	std::insert_iterator<std::list<instr_ptr>> inserter;
	T tag;
};

#endif
