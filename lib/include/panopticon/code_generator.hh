#include <panopticon/mnemonic.hh>
#include <panopticon/architecture.hh>

#pragma once

/**
 * @file
 */

namespace po
{
	/**
	 * @brief Interface for constructing IL statements
	 *
	 * This class is used to construct a list of IL statements emitting them to a output iterator.
	 * Aside from simplifying the syntax this class adds basic error checks to prevent creation of
	 * some invalid instructions.
	 *
	 * @note T must be a model of the Architecture concept.
	 */
	template<typename T>
	class code_generator
	{
	public:
		/**
		 * Construct a new generator.
		 * @param i Insert iterator used to output the IL statements.
		 */
		code_generator(std::insert_iterator<std::list<instr>> i) : inserter(i), tag() {};

		/// @returns \c a and emits an IL instruction for <tt>a := op1 ∧ op2</tt>
		lvalue and_b(lvalue a, rvalue op1, rvalue op2)		{ return named(logic_and<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 ∨ op2</tt>
		lvalue or_b(lvalue a, rvalue op1, rvalue op2)		{ return named(logic_or<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 ⊕ op2</tt>
		//lvalue xor_b(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::Xor,a,op1,op2); };
		/// @returns \c a and emits an IL instruction for <tt>a := ¬op</tt>
		lvalue not_b(lvalue a, rvalue op)					{ return named(logic_neg<rvalue>{op},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op</tt>
		lvalue assign(lvalue a, rvalue op)								{ return named(univ_nop<rvalue>{op},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 <<ᵤ op2</tt>
		//lvalue shiftr_u(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::UShr,a,cnt,op); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 >>ᵤ op2</tt>
		//lvalue shiftl_u(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::UShl,a,cnt,op); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 <<ₛ op2</tt>
		//lvalue shiftr_s(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::SShr,a,cnt,op); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 >>ₛ op2</tt>
		//lvalue shiftl_s(lvalue a, rvalue cnt, rvalue op)	{ return named(instr::SShl,a,cnt,op); };
		/// @returns \c a and emits an IL instruction sign-extending \c op1 to \c op2 bits and assigning the result to \c a
		//lvalue ext_u(lvalue a, rvalue cnt, rvalue op)			{ return named(instr::UExt,a,cnt,op); };
		/// @returns \c a and emits an IL instruction extending \c op1 to \c op2 bits and assigning the result to \c a
		//lvalue ext_s(lvalue a, rvalue cnt, rvalue op)			{ return named(instr::SExt,a,cnt,op); };
		/// @returns \c a and emits an IL instruction extracting bits \c to to \c from from \c op and assigning the result to \c a
		//lvalue slice(lvalue a, rvalue op, rvalue from, rvalue to)		{ return named(instr::Slice,a,op,from,to); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 + op2</tt>
		lvalue add_i(lvalue a, rvalue op1, rvalue op2)		{ return named(int_add<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 - op2</tt>
		lvalue sub_i(lvalue a, rvalue op1, rvalue op2)		{ return named(int_sub<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 * op2</tt>
		lvalue mul_i(lvalue a, rvalue op1, rvalue op2)		{ return named(int_mul<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 / op2</tt>
		//lvalue div_is(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::SDiv,a,op1,op2); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 div op2</tt>
		lvalue div_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(int_div<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 % op2</tt>
		//lvalue mod_is(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::SMod,a,op1,op2); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 % op2</tt>
		lvalue mod_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(int_mod<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 <ᵤ op2</tt>
		//lvalue leq_is(lvalue a, rvalue op1, rvalue op2)		{ return named(instr::SLeq,a,op1,op2); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 <ₛ op2</tt>
		lvalue less_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(int_less<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op()</tt>
		lvalue call(lvalue a, rvalue op)									{ return named(int_call<rvalue>{op},a); };

		/*
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∧ op2</tt>
		lvalue and_b(rvalue op1, rvalue op2)		{ return anonymous(instr::And,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∨ op2</tt>
		lvalue or_b(rvalue op1, rvalue op2)			{ return anonymous(instr::Or,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ⊕ op2</tt>
		lvalue xor_b(rvalue op1, rvalue op2)		{ return anonymous(instr::Xor,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := ¬op</tt>
		lvalue not_b(rvalue op)									{ return anonymous(instr::Not,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op</tt>
		lvalue assign(rvalue op)								{ return anonymous(instr::Assign,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 <<ᵤ op2</tt>
		lvalue shiftr_u(rvalue cnt, rvalue op)	{ return anonymous(instr::UShr,cnt,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 >>ᵤ op2</tt>
		lvalue shiftl_u(rvalue cnt, rvalue op)	{ return anonymous(instr::UShl,cnt,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 <<ₛ op2</tt>
		lvalue shiftr_s(rvalue cnt, rvalue op)	{ return anonymous(instr::SShr,cnt,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 >>ₛ op2</tt>
		lvalue shiftl_s(rvalue cnt, rvalue op)	{ return anonymous(instr::SShl,cnt,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction sign-extending \c op1 to \c op2 bits and assigning the result to \c tmp
		lvalue ext_u(rvalue cnt, rvalue op)			{ return anonymous(instr::UExt,cnt,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction extending \c op1 to \c op2 bits and assigning the result to \c tmp
		lvalue ext_s(rvalue cnt, rvalue op)			{ return anonymous(instr::SExt,cnt,op); };
		/// @returns a new temporary \c tmp and emits an IL instruction extracting bits \c to to \c from from \c op and assigning the result to \c tmp
		lvalue slice(rvalue op, rvalue from, rvalue to)		{ return anonymous(instr::Slice,op,from,to); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 + op2</tt>
		lvalue add_i(rvalue op1, rvalue op2)		{ return anonymous(instr::Add,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 - op2</tt>
		lvalue sub_i(rvalue op1, rvalue op2)		{ return anonymous(instr::Sub,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 * op2</tt>
		lvalue mul_i(rvalue op1, rvalue op2)		{ return anonymous(instr::Mul,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 / op2</tt>
		lvalue div_is(rvalue op1, rvalue op2)		{ return anonymous(instr::SDiv,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 div op2</tt>
		lvalue div_iu(rvalue op1, rvalue op2)		{ return anonymous(instr::UDiv,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 % op2</tt>
		lvalue mod_is(rvalue op1, rvalue op2)		{ return anonymous(instr::SMod,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 % op2</tt>
		lvalue mod_iu(rvalue op1, rvalue op2)		{ return anonymous(instr::UMod,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 <ᵤ op2</tt>
		lvalue leq_is(rvalue op1, rvalue op2)		{ return anonymous(instr::SLeq,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 <ₛ op2</tt>
		lvalue leq_iu(rvalue op1, rvalue op2)		{ return anonymous(instr::ULeq,op1,op2); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op()</tt>
		lvalue call(rvalue op)									{ return anonymous(instr::Call,op); };*/

	protected:
		/**
		 * Construct a new instr instance and emit it to \ref inserter.
		 * Values must be subclasses of rvalue.
		 * @returns assign
		 */
		template<class... Values>
		lvalue named(instr::operation fn, lvalue assign)
		{
			instr ret(fn,assign);
			std::vector<rvalue> arguments = operands(ret);

			auto sanity_check = [](const rvalue &v)
			{
				if(is_variable(v))
					return to_variable(v).name().size() && to_variable(v).subscript() == -1 && to_variable(v).width();
				else if(is_memory(v))
					return to_memory(v).name().size() && to_memory(v).bytes() &&
								 (to_memory(v).endianess() == memory::BigEndian || to_memory(v).endianess() == memory::LittleEndian) &&
								 to_memory(v).offset() != v;
				else if(is_constant(v))
					return true;
				else
					return is_undefined(v);
			};

			assert(all_of(arguments.begin(),arguments.end(),sanity_check) && sanity_check(assign));
			inserter = ret;

			return assign;
		}

		/**
		 * Construct a new instr instance and emit it to \ref inserter.
		 * Values must be subclasses of rvalue.
		 * @returns A new temporary that holds the value of the expression.
		 */
		template<class... Values>
		lvalue anonymous(instr::operation fn)
		{
			return named(fn,temporary(tag));
		}

		std::insert_iterator<std::list<instr>> inserter;
		T tag;
	};
}
