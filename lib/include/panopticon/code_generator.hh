#include <panopticon/mnemonic.hh>

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
		lvalue or_b(lvalue a, rvalue op1, rvalue op2)			{ return named(logic_or<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := ¬op</tt>
		lvalue not_b(lvalue a, rvalue op)									{ return named(logic_neg<rvalue>{op},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op</tt>
		lvalue assign(lvalue a, rvalue op)								{ return named(univ_nop<rvalue>{op},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 + op2</tt>
		lvalue add_i(lvalue a, rvalue op1, rvalue op2)		{ return named(int_add<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 - op2</tt>
		lvalue sub_i(lvalue a, rvalue op1, rvalue op2)		{ return named(int_sub<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 * op2</tt>
		lvalue mul_i(lvalue a, rvalue op1, rvalue op2)		{ return named(int_mul<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 div op2</tt>
		lvalue div_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(int_div<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 % op2</tt>
		lvalue mod_iu(lvalue a, rvalue op1, rvalue op2)		{ return named(int_mod<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 <ₛ op2</tt>
		lvalue less_iu(lvalue a, rvalue op1, rvalue op2)	{ return named(int_less<rvalue>{op1,op2},a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op()</tt>
		lvalue call(lvalue a, rvalue op)									{ return named(int_call<rvalue>{op},a); };

		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∧ op2</tt>
		lvalue and_b(rvalue op1, rvalue op2)		{ return anonymous(logic_and<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∨ op2</tt>
		lvalue or_b(rvalue op1, rvalue op2)			{ return anonymous(logic_or<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := ¬op</tt>
		lvalue not_b(rvalue op)									{ return anonymous(logic_neg<rvalue>{op}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op</tt>
		lvalue assign(rvalue op)								{ return anonymous(univ_nop<rvalue>{op}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 + op2</tt>
		lvalue add_i(rvalue op1, rvalue op2)		{ return anonymous(int_add<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 - op2</tt>
		lvalue sub_i(rvalue op1, rvalue op2)		{ return anonymous(int_sub<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 * op2</tt>
		lvalue mul_i(rvalue op1, rvalue op2)		{ return anonymous(int_mul<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 div op2</tt>
		lvalue div_iu(rvalue op1, rvalue op2)		{ return anonymous(int_div<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 % op2</tt>
		lvalue mod_iu(rvalue op1, rvalue op2)		{ return anonymous(int_mod<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 <ᵤ op2</tt>
		lvalue less_iu(rvalue op1, rvalue op2)		{ return anonymous(int_less<rvalue>{op1,op2}); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 <ₛ op2</tt>
		lvalue call(rvalue op)									{ return anonymous(int_call<rvalue>{op}); };

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
								 (to_memory(v).endianess() == BigEndian || to_memory(v).endianess() == LittleEndian) &&
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
