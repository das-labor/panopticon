#include <iostream>
#include <list>
#include <string>
#include <vector>
#include <cassert>
#include <initializer_list>

#include <panopticon/value.hh>
#include <panopticon/marshal.hh>
#include <panopticon/region.hh>

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
	class instr;
	class mnemonic;

	/**
	 * @brief Single IL statement
	 *
	 * In order to allow code analysis algorithms to
	 * be implemented in a instruction set-agnostic manner,
	 * all opcodes are translated into a intermediate
	 * language first. Analysis is done on the IL and the
	 * results are mapped back to the original code.
	 *
	 * Every instance of the instr class models on IL statement.
	 * Each statement has the form a := f(b,...,z) where @c f is
	 * a @ref Function defined in the IL, @c b to @z its
	 * arguments (currently up to 3) and @c a is the variable
	 * receiving the result for @c f.
	 */
	class instr
	{
	public:
		/**
		 * IL functions
		 */
		enum Function
		{
			Phi,			///< phi-Function
			Not,			///< Bitwise Not
			And,			///< Bitwise And
			Or,			///< Bitwise Or
			Xor,			///< Bitwise Xor
			Assign,		///< Assign Intermediate
			ULeq,			///< Unsigned less-or-equal
			SLeq,			///< Signed less-or-equal
			UShr,			///< Unsigned right shift *
			UShl,			///< Unsigned left shift *
			SShr,			///< Signed right shift
			SShl,			///< Signed left shift
			SExt,			///< Signed extension *
			UExt,			///< Unsigned extension *
			Slice,		///< Slice
			//Concat,		// Concatenation
			Add,			///< Addition
			Sub,			///< Subtraction
			Mul,			///< Multiplication
			SDiv,			///< Signed Division
			UDiv,			///< Unsigned Division
			SMod,			///< Unsigned Modulo reduction
			UMod,			///< Signed Modulo reduction
			Call,			///< Procedure call
			/// @todo Floating point
		};

		/// Construct a statement applying function @arg fn to @arg args. Saves the result in @arg a
		template<class... Values>
		instr(Function fn, lvalue a, Values&&... args) : function(fn), left(a), right({args...}) {}

		/// Construct a statement applying function @arg fn to @arg r. Saves the result in @arg a
		instr(Function fn, lvalue a, std::initializer_list<rvalue> r) : function(fn), left(a), right(r) {}

		Function function;
		lvalue left;
		std::vector<rvalue> right;
	};

	std::string pretty(instr::Function fn); 				///< Pretty print the function
	std::string symbolic(instr::Function fn);				///< Returns a string suitable for describing the function in RDF
	instr::Function numeric(const std::string &s);	///< Maps a string returned from @ref symbolic back the enum value

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
	class mnemonic
	{
	public:
		typedef std::vector<instr>::const_iterator iterator;

		/**
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
			bool has_sign;	///< True if the variable content has a sign
			unsigned int width; ///< Width of the bit vector encoded in the IL variable
			std::string alias;	///< String alias or literal value (is @ref is_literal is true) of the token
			bool is_literal;	///< True whenever this is a string literal not connected to a IL variable
		};

		static mnemonic unmarshal(const rdf::node &n, const rdf::storage &store);

		/**
		 * Construct a new mnemonic for opcode @arg n spanning @arg a, formatted using format string @arg fmt.
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

		mnemonic(const mnemonic &m);
		mnemonic(mnemonic &&m);

		mnemonic &operator=(const mnemonic &m);
		mnemonic &operator=(mnemonic &&m);

		/// Render the operands using the format string
		std::string format_operands(void) const;

		offset length;										///< Size of this mnemonic
		std::string opcode;								///< Mnemonic of the opcode
		std::vector<rvalue> operands;			///< Operands of the mnemonic left to right
		std::vector<instr> instructions;	///< Instructions encoding the mnemonic semantics
		std::vector<token> format;				///< Parsed format string
	};

	std::ostream& operator<<(std::ostream &os, const instr &i);
	std::ostream& operator<<(std::ostream &os, const mnemonic &m);
	/*odotstream& operator<<(odotstream &os, const mnemonic &m);
	oturtlestream& operator<<(oturtlestream &os, const mnemonic &m);*/
	std::string unique_name(const mnemonic &mne);

	/// Format a concrete value as specified in a escape sequence
	int64_t format_constant(const mnemonic::token &tok, uint64_t v);
}
