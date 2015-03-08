/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <panopticon/mnemonic.hh>
#include <panopticon/architecture.hh>
#include <panopticon/ensure.hh>

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
		lvalue and_b(lvalue a, rvalue op1, rvalue op2)		{ logic_and<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 ∨ op2</tt>
		lvalue or_b(lvalue a, rvalue op1, rvalue op2)			{ logic_or<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := (int)op</tt>
		lvalue lift_b(lvalue a, rvalue op)								{ logic_lift<rvalue> i{op}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := ¬op</tt>
		lvalue not_b(lvalue a, rvalue op)									{ logic_neg<rvalue> i{op}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op</tt>
		lvalue assign(lvalue a, rvalue op)								{ univ_nop<rvalue> i{op}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 ∧ op2</tt>
		lvalue and_i(lvalue a, rvalue op1, rvalue op2)		{ int_and<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 ∨ op2</tt>
		lvalue or_i(lvalue a, rvalue op1, rvalue op2)			{ int_or<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 ⊕ op2</tt>
		lvalue xor_i(lvalue a, rvalue op1, rvalue op2)		{ int_xor<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 + op2</tt>
		lvalue add_i(lvalue a, rvalue op1, rvalue op2)		{ int_add<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 - op2</tt>
		lvalue sub_i(lvalue a, rvalue op1, rvalue op2)		{ int_sub<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 * op2</tt>
		lvalue mul_i(lvalue a, rvalue op1, rvalue op2)		{ int_mul<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 div op2</tt>
		lvalue div_i(lvalue a, rvalue op1, rvalue op2)		{ int_div<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 % op2</tt>
		lvalue mod_i(lvalue a, rvalue op1, rvalue op2)		{ int_mod<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 == op2</tt>
		lvalue equal_i(lvalue a, rvalue op1, rvalue op2)	{ int_equal<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 < op2</tt>
		lvalue less_i(lvalue a, rvalue op1, rvalue op2)		{ int_less<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op()</tt>
		lvalue call_i(lvalue a, rvalue op)								{ int_call<rvalue> i{op}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 >> op2</tt>
		lvalue rshift_i(lvalue a, rvalue op1, rvalue op2)	{ int_rshift<rvalue> i{op1,op2}; return named(i,a); };
		/// @returns \c a and emits an IL instruction for <tt>a := op1 << op2</tt>
		lvalue lshift_i(lvalue a, rvalue op1, rvalue op2)	{ int_lshift<rvalue> i{op1,op2}; return named(i,a); };

		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∧ op2</tt>
		lvalue and_b(rvalue op1, rvalue op2)		{ logic_and<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∨ op2</tt>
		lvalue or_b(rvalue op1, rvalue op2)			{ logic_or<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := ¬op</tt>
		lvalue not_b(rvalue op)									{ logic_neg<rvalue> i{op}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := (int)op</tt>
		lvalue lift_b(rvalue op)								{ logic_lift<rvalue> i{op}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op</tt>
		lvalue assign(rvalue op)								{ univ_nop<rvalue> i{op}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∧ op2</tt>
		lvalue and_i(rvalue op1, rvalue op2)		{ int_and<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ∨ op2</tt>
		lvalue or_i(rvalue op1, rvalue op2)			{ int_or<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 ⊕ op2</tt>
		lvalue xor_i(rvalue op1, rvalue op2)		{ int_xor<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 + op2</tt>
		lvalue add_i(rvalue op1, rvalue op2)		{ int_add<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 - op2</tt>
		lvalue sub_i(rvalue op1, rvalue op2)		{ int_sub<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 * op2</tt>
		lvalue mul_i(rvalue op1, rvalue op2)		{ int_mul<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 div op2</tt>
		lvalue div_i(rvalue op1, rvalue op2)		{ int_div<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 % op2</tt>
		lvalue mod_i(rvalue op1, rvalue op2)		{ int_mod<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 == op2</tt>
		lvalue equal_i(rvalue op1, rvalue op2)	{ int_equal<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 < op2</tt>
		lvalue less_i(rvalue op1, rvalue op2)		{ int_less<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op()</tt>
		lvalue call_i(rvalue op)								{ int_call<rvalue> i{op}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 >> op2</tt>
		lvalue rshift_i(rvalue op1, rvalue op2)	{ int_rshift<rvalue> i{op1,op2}; return anonymous(i); };
		/// @returns a new temporary \c tmp and emits an IL instruction for <tt>tmp := op1 << op2</tt>
		lvalue lshift_i(rvalue op1, rvalue op2)	{ int_lshift<rvalue> i{op1,op2}; return anonymous(i); };

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

			ensure(all_of(arguments.begin(),arguments.end(),sanity_check) && sanity_check(assign));
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

	namespace dsl
	{
		struct callback_list
		{
			template<typename T>
			callback_list(code_generator<T>& cg)
			: add_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::add_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				sub_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::sub_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				mul_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::mul_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				div_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::div_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				mod_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::mod_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				and_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::and_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				or_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::or_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				xor_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::xor_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				less_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::less_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				equal_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::equal_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				rshift_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::rshift_i,&cg,std::placeholders::_1,std::placeholders::_2)),
				lshift_i(std::bind((lvalue(code_generator<T>::*)(rvalue,rvalue))&code_generator<T>::lshift_i,&cg,std::placeholders::_1,std::placeholders::_2))
			{}

			std::function<rvalue(const rvalue&,const rvalue&)> add_i;
			std::function<rvalue(const rvalue&,const rvalue&)> sub_i;
			std::function<rvalue(const rvalue&,const rvalue&)> mul_i;
			std::function<rvalue(const rvalue&,const rvalue&)> div_i;
			std::function<rvalue(const rvalue&,const rvalue&)> mod_i;
			std::function<rvalue(const rvalue&,const rvalue&)> and_i;
			std::function<rvalue(const rvalue&,const rvalue&)> or_i;
			std::function<rvalue(const rvalue&,const rvalue&)> xor_i;
			std::function<rvalue(const rvalue&,const rvalue&)> less_i;
			std::function<rvalue(const rvalue&,const rvalue&)> equal_i;
			std::function<rvalue(const rvalue&,const rvalue&)> rshift_i;
			std::function<rvalue(const rvalue&,const rvalue&)> lshift_i;
		};

#ifdef _MSC_VER
		extern __declspec(thread) callback_list* current_code_generator;
#else
		extern __thread callback_list* current_code_generator;
#endif

		inline rvalue operator+(const rvalue& a, const rvalue& b) { return current_code_generator->add_i(a,b); }
		inline rvalue operator+(unsigned long long a, const rvalue& b) { return constant(a) + b; }
		inline rvalue operator+(const rvalue& a, unsigned long long b) { return a + constant(b); }
		inline rvalue operator-(const rvalue& a, const rvalue& b) { return current_code_generator->sub_i(a,b); }
		inline rvalue operator-(unsigned long long a, const rvalue& b) { return constant(a) - b; }
		inline rvalue operator-(const rvalue& a, unsigned long long b) { return a - constant(b); }
		inline rvalue operator*(const rvalue& a, const rvalue& b) { return current_code_generator->mul_i(a,b); }
		inline rvalue operator*(unsigned long long a, const rvalue& b) { return constant(a) * b; }
		inline rvalue operator*(const rvalue& a, unsigned long long b) { return a * constant(b); }
		inline rvalue operator/(const rvalue& a, const rvalue& b) { return current_code_generator->div_i(a,b); }
		inline rvalue operator/(unsigned long long a, const rvalue& b) { return constant(a) / b; }
		inline rvalue operator/(const rvalue& a, unsigned long long b) { return a / constant(b); }
		inline rvalue operator%(const rvalue& a, const rvalue& b) { return current_code_generator->mod_i(a,b); }
		inline rvalue operator%(unsigned long long a, const rvalue& b) { return constant(a) % b; }
		inline rvalue operator%(const rvalue& a, unsigned long long b) { return a % constant(b); }
		inline rvalue operator&(const rvalue& a, const rvalue& b) { return current_code_generator->and_i(a,b); }
		inline rvalue operator&(unsigned long long a, const rvalue& b) { return constant(a) & b; }
		inline rvalue operator&(const rvalue& a, unsigned long long b) { return a & constant(b); }
		inline rvalue operator|(const rvalue& a, const rvalue& b) { return current_code_generator->or_i(a,b); }
		inline rvalue operator|(unsigned long long a, const rvalue& b) { return constant(a) | b; }
		inline rvalue operator|(const rvalue& a, unsigned long long b) { return a | constant(b); }
		inline rvalue operator^(const rvalue& a, const rvalue& b) { return current_code_generator->xor_i(a,b); }
		inline rvalue operator^(unsigned long long a, const rvalue& b) { return constant(a) ^ b; }
		inline rvalue operator^(const rvalue& a, unsigned long long b) { return a ^ constant(b); }
		inline rvalue operator>>(const rvalue& a, const rvalue& b) { return current_code_generator->rshift_i(a,b); }
		inline rvalue operator>>(unsigned long long a, const rvalue& b) { return constant(a) >> b; }
		inline rvalue operator>>(const rvalue& a, unsigned long long b) { return a >> constant(b); }
		inline rvalue operator<<(const rvalue& a, const rvalue& b) { return current_code_generator->lshift_i(a,b); }
		inline rvalue operator<<(unsigned long long a, const rvalue& b) { return constant(a) << b; }
		inline rvalue operator<<(const rvalue& a, unsigned long long b) { return a << constant(b); }

		inline rvalue less(const rvalue& a, const rvalue& b) { return current_code_generator->less_i(a,b); }
		inline rvalue less(const rvalue& a, unsigned long long b) { return less(a,constant(b)); }
		inline rvalue less(unsigned long long a, const rvalue& b) { return less(constant(a),b); }
		inline rvalue equal(const rvalue& a, const rvalue& b) { return current_code_generator->equal_i(a,b); }
		inline rvalue equal(const rvalue& a, unsigned long long b) { return equal(a,constant(b)); }
		inline rvalue equal(unsigned long long a, const rvalue& b) { return equal(constant(a),b); }
	}
}
