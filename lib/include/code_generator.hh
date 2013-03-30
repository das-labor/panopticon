#ifndef CODE_GENERATOR_HH
#define CODE_GENERATOR_HH

#include <cstring>

#include <mnemonic.hh>
#include <architecture.hh>

namespace po
{
	template<typename T>
	class code_generator
	{
	public:
		code_generator(std::insert_iterator< std::list<instr>> i) : inserter(i) {};

		// named
		lvalue and_b(lvalue a, rvalue op1, rvalue op2)	 	{ return named(instr::And,a,op1,op2); };
		lvalue or_b(lvalue a, rvalue op1, rvalue op2) 		{ return named(instr::Or,a,op1,op2); };
		lvalue xor_b(lvalue a, rvalue op1, rvalue op2)	 	{ return named(instr::Xor,a,op1,op2); };
		lvalue not_b(lvalue a, rvalue op)									{ return named(instr::Not,a,op); };
		lvalue assign(lvalue a, rvalue op)								{ return named(instr::Assign,a,op); };
		lvalue shiftr_u(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::UShr,a,cnt,op); };
		lvalue shiftl_u(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::UShl,a,cnt,op); };
		lvalue shiftr_s(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::SShr,a,cnt,op); };
		lvalue shiftl_s(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::SShl,a,cnt,op); };
		lvalue ext_u(lvalue a, rvalue cnt, rvalue op)			{ return named(instr::UExt,a,cnt,op); };
		lvalue ext_s(lvalue a, rvalue cnt, rvalue op)			{ return named(instr::SExt,a,cnt,op); };
		lvalue slice(lvalue a, rvalue op, rvalue from, rvalue to)		{ return named(instr::Slice,a,op,from,to); };
		lvalue add_i(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::Add,a,op1,op2); };
		lvalue sub_i(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::Sub,a,op1,op2); };
		lvalue mul_i(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::Mul,a,op1,op2); };
		lvalue div_is(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::SDiv,a,op1,op2); };
		lvalue div_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::UDiv,a,op1,op2); };
		lvalue mod_is(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::SMod,a,op1,op2); };
		lvalue mod_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::UMod,a,op1,op2); };
		lvalue leq_is(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::SLeq,a,op1,op2); };
		lvalue leq_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::ULeq,a,op1,op2); };
		lvalue call(lvalue a, rvalue op)									{ return named(instr::Call,a,op); };

		// anonymous
		lvalue and_b(rvalue op1, rvalue op2) 		{ return anonymous(instr::And,op1,op2); };
		lvalue or_b(rvalue op1, rvalue op2) 		{ return anonymous(instr::Or,op1,op2); };
		lvalue xor_b(rvalue op1, rvalue op2)	 	{ return anonymous(instr::Xor,op1,op2); };
		lvalue not_b(rvalue op)									{ return anonymous(instr::Not,op); };
		lvalue assign(rvalue op)								{ return anonymous(instr::Assign,op); };
		lvalue shiftr_u(rvalue cnt, rvalue op)	{ return anonymous(instr::UShr,cnt,op); };
		lvalue shiftl_u(rvalue cnt, rvalue op)	{ return anonymous(instr::UShl,cnt,op); };
		lvalue shiftr_s(rvalue cnt, rvalue op)	{ return anonymous(instr::SShr,cnt,op); };
		lvalue shiftl_s(rvalue cnt, rvalue op)	{ return anonymous(instr::SShl,cnt,op); };
		lvalue ext_u(rvalue cnt, rvalue op)			{ return anonymous(instr::UExt,cnt,op); };
		lvalue ext_s(rvalue cnt, rvalue op)			{ return anonymous(instr::SExt,cnt,op); };
		lvalue slice(rvalue op, rvalue from, rvalue to)		{ return anonymous(instr::Slice,op,from,to); };
		lvalue add_i(rvalue op1, rvalue op2)		{ return anonymous(instr::Add,op1,op2); };
		lvalue sub_i(rvalue op1, rvalue op2)		{ return anonymous(instr::Sub,op1,op2); };
		lvalue mul_i(rvalue op1, rvalue op2)		{ return anonymous(instr::Mul,op1,op2); };
		lvalue div_is(rvalue op1, rvalue op2)		{ return anonymous(instr::SDiv,op1,op2); };
		lvalue div_iu(rvalue op1, rvalue op2)		{ return anonymous(instr::UDiv,op1,op2); };
		lvalue mod_is(rvalue op1, rvalue op2)		{ return anonymous(instr::SMod,op1,op2); };
		lvalue mod_iu(rvalue op1, rvalue op2)		{ return anonymous(instr::UMod,op1,op2); };
		lvalue leq_is(rvalue op1, rvalue op2)		{ return anonymous(instr::SLeq,op1,op2); };
		lvalue leq_iu(rvalue op1, rvalue op2)		{ return anonymous(instr::ULeq,op1,op2); };
		lvalue call(rvalue op)									{ return anonymous(instr::Call,op); };	

	protected:
		template<class... Values>
		lvalue named(instr::Function fn, lvalue assign, Values&&... args)
		{
			std::vector<rvalue> arguments({args...});

			auto sanity_check = [](const rvalue &v)
			{
				if(v.is_variable())
					return v.to_variable().name().size() && v.to_variable().subscript() == -1 && v.to_variable().width();
				else if(v.is_memory())
					return v.to_memory().name().size() && v.to_memory().bytes() && 
								 (v.to_memory().endianess() == memory::BigEndian || v.to_memory().endianess() == memory::LittleEndian) && 
								 v.to_memory().offset() != v;
				else if(v.is_constant())
					return v.to_constant().width() > 0;
				else
					return v.is_undefined();
			};

			assert(all_of(arguments.begin(),arguments.end(),sanity_check) && sanity_check(assign));
					
			instr ret(fn,assign,arguments);
			inserter = ret;

			return assign;
		}

		template<class... Values>
		lvalue anonymous(instr::Function fn, Values... args)
		{
			return named(fn,temporary(tag),args...);
		}

		static unsigned int next;
		std::insert_iterator< std::list<instr>> inserter;
		T tag;
	};
}

#endif
