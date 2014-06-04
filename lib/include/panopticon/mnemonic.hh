#include <iostream>
#include <list>
#include <string>
#include <vector>
#include <cassert>
#include <initializer_list>

#include <panopticon/value.hh>
#include <panopticon/marshal.hh>
#include <panopticon/region.hh>
#include <panopticon/instr.hh>

#pragma once

/**
 * @file
 * @brief Opcode mnemonics and IL
 *
 * All code analysis done by Panopticon is done
 * on an intermediate language (IL). Disassemblers
 * defined for any supported instruction set
 * architecture translate native Opcode into IL
 * statements (@ref instr instances). These are
 * grouped into @ref mnemonic objects which in turn
 * are grouped into procedures.
 */

namespace po
{
	/**
	 * @brief Mnemonic from a Instruction Set
	 *
	 * IL statements are grouped into mnemonics. Format and
	 * a mapping from mnemonic operands to IL variables help
	 * Panopticon do display analysis results to the user without
	 * showing IL statements.
	 *
	 * The textual representation of a mnemonic is given as an
	 * format string. The string is displayed verbatim except
	 * for everything wrapped in curly braces. These mark
	 * mnemonic operands that can be replaced with concrete
	 * values.
	 *
	 * The format string syntax is '{' Width ( ':' Modifiers ( ':' Alias )? )? '}'
	 * Where @c Width is the size of the register or operand in bits, Modifiers is
	 * '-' if the value has a sign and Alias the symbolic name of operand.
	 * Example: "add {32::eax}, {32:-:5}" is displayed as "add eax, 5" or "add 10, 5"
	 * if the value of the eax register is known to be 10 before execution of the
	 * opcode.
	 */
	struct mnemonic
	{
		typedef std::vector<instr>::const_iterator iterator;

		/**
		 * Construct a new mnemonic for opcode @arg n spanning @arg a, formatted using format string @arg fmt.
		 * Mnemonics are formatted as a sequence of tokens. Each token
		 * is either a literal string or a placeholder o be filled
		 * with the contents of a IL variable.
		 *
		 * In the latter case, @ref has_sign, @ref width and @ref alias
		 * are used to format the value.
		 */
		struct token
		{
			token(void) : has_sign(false), width(0), alias(""), is_literal(false) {}
			bool operator==(const token& t) const { return has_sign == t.has_sign && width == t.width && alias == t.alias && is_literal == t.is_literal; }
			bool has_sign;		  ///< True if the variable content has a sign
			unsigned int width; ///< Width of the bit vector encoded in the IL variable
			std::string alias;	///< String alias or literal value (is @ref is_literal is true) of the token
			bool is_literal;		///< True whenever this is a string literal not connected to a IL variable
		};

		mnemonic(const mnemonic &m);
		mnemonic(mnemonic &&m);

		mnemonic &operator=(const mnemonic &m);
		mnemonic &operator=(mnemonic &&m);

		/**
		 * Operands between @arg ops_begin and @arg ops_end and IL statements between @arg instr_begin and
		 * instr_end are copied into the new instance.
		 *
		 * @note The values pointed to by F1 must be convertible to @ref rvalue.
		 * @note The values pointed to by F2 must be convertible to @ref instr.
		 */
		template <typename F1, typename F2>
		mnemonic(const bound &a, const std::string &n, const std::string &fmt, F1 ops_begin, F1 ops_end, F2 instr_begin, F2 instr_end)
		: mnemonic(a,n,fmt,{},{})
		{
			std::copy(ops_begin,ops_end,inserter(operands,operands.begin()));
			std::copy(instr_begin,instr_end,inserter(instructions,instructions.begin()));
		}

		/**
		 * Construct a new mnemonic for opcode @arg n spanning @arg a, formatted using format string @arg fmt.
		 * Operands in @arg ops and IL statements in instrs are copied into the new instance.
		 */
		mnemonic(const bound &a, const std::string &n, const std::string &fmt, std::initializer_list<rvalue> ops, std::initializer_list<instr> instrs);

		bool operator==(const mnemonic&) const;

		/// Render the operands using the format string
		std::string format_operands(void) const;

		bound area;												///< Size of this mnemonic
		std::string opcode;								///< Mnemonic of the opcode
		std::vector<rvalue> operands;			///< Operands of the mnemonic left to right
		std::vector<instr> instructions;	///< Instructions encoding the mnemonic semantics
		std::list<token> format_seq;			///< Parsed format string
		std::string format_string;				///< Format string
	};

	template<>
	mnemonic* unmarshal(const uuid&, const rdf::storage&);

	template<>
	rdf::statements marshal(const mnemonic*, const uuid&);

	std::ostream& operator<<(std::ostream &os, const mnemonic &m);

	int64_t format_constant(const mnemonic::token &tok, uint64_t v);
}
