/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
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

#include <unordered_map>
#include <iostream>
#include <cstdint>
#include <stdexcept>

#include <boost/variant.hpp>
#include <boost/optional.hpp>
#include <boost/operators.hpp>

#include <panopticon/ensure.hh>
#include <panopticon/marshal.hh>
#include <panopticon/hash.hh>

#pragma once

/**
 * @file
 * @brief Basic datatype for IL arguments
 *
 * The value classes model the data IL instructions (instr) work on. The classes
 * are designed to fit into 64 bits and are always call-by-value.
 * Possible data types are a constant, a value in memory, a variable or an undefined value.
 * Variable and memory values are grouped into lvalues. Both can be target or an assignment.
 * Memory, variable and constant values include the width of the value/variable. Variables and constants
 * are sized in bits, memory in bytes. Memory includes the endianess of the data.
 * Converting between classes can be done with to_constant(), to_variable(), to_memory(), etc.
 */

namespace po
{
	class rvalue;
	class constant;
	class undefined;
	class variable;
	class memory;
	class value_exception;

	enum Endianess
	{
		LittleEndian = 1,
		BigEndian = 2
	};

	/**
	 * @brief A constant value
	 *
	 * Models a constant value as a unsigned integer
	 */
	class constant
	{
	public:
		/// Construct a new constant @c v.
		constant(uint64_t v);

		bool operator==(const constant&) const;
		bool operator<(const constant&) const;

		/// @returns integer value of this constant.
		uint64_t content(void) const;

	private:
		uint64_t _content;
	};

	/**
	 * @brief Undefined value
	 */
	class undefined
	{
	public:
		bool operator==(const undefined&) const;
		bool operator<(const undefined&) const;
	};

	/**
	 * @brief A variable with fixed width.
	 *
	 * Variables loosely model registers with a fixed width in bits.
	 * Aside from a name of ASCII characters, it also can have a
	 * subscript integer that describes the version of the variable in the
	 * SSA form of the procedure. User-defined literals _v1, _v8, _v16, _v32, _v64
	 * are shortcuts for constructing 1, 8, 16, 32, and 64 bit wide, unversioned
	 * variables.
	 */
	class variable
	{
	public:
		/**
		 * Construct a variable with name @c n, width of @c w bits and version @c s.
		 */
		variable(const std::string &n, uint16_t w, int s = -1);

		bool operator==(const variable&) const;
		bool operator<(const variable&) const;

		/// @returns width of this varaible in bits
		uint16_t width(void) const;

		/// @return name of the variable
		const std::string& name(void) const;

		/// @returns version of the variable if in SSA form. -1 means no version (not yet in SSA form)
		int subscript(void) const;

	private:
		uint16_t _width;
		std::string _name;
		int _subscript;
	};

	/**
	 * @brief A reference to a memory slot
	 *
	 * A memory reference is modeled by a offset from the beginning of a named
	 * memory region, the number of bytes to read from this offset and the byte ordering
	 * (endianess) to obey when saving it in a register.
	 */
	class memory
	{
	public:
		memory(const memory &);

		/// Construct a new reference to @c b bytes, starting at offset @c o, in memory region @c n, saved in @c e ordering.
		memory(const rvalue &o, uint16_t b, Endianess e, const std::string &n);

		memory& operator=(const memory&);

		bool operator==(const memory&) const;
		bool operator<(const memory&) const;

		/// @returns number of bytes to read from offset().
		uint16_t bytes(void) const;

		/// @returns position in the memory region to read from.
		const rvalue &offset(void) const;

		/// @returns Byte ordering to obey when reading from the reference.
		Endianess endianess(void) const;

		/// @returns name of the memory region this reference points into.
		const std::string &name(void) const;

	private:
		std::unique_ptr<rvalue> _offset;
		uint16_t _bytes;
		Endianess _endianess;
		std::string _name;
	};

	/**
	 * @brief A data type than can be written to.
	 *
	 * All valid targets of a assignment 'memory' and 'variable'.
	 */
	using lvalue = boost::variant<undefined,variable,memory>;

	/**
	 * @brief Base of all data types the IL operates on.
	 */
	class rvalue
	{
	public:

		/**
	 	 * @brief Unmarshal a rvalue from a RDF node
		 */

		/// Constructs a undefined value.
		rvalue(void);

		rvalue(const constant &t) : _variant(t) {}
		rvalue(const undefined &t) : _variant(t) {}
		rvalue(const variable &t) : _variant(t) {}
		rvalue(const memory &t) : _variant(t) {}
		rvalue(const lvalue &t) : _variant(t) {}

	private:
		boost::variant<undefined,constant,variable,memory> _variant;

		friend bool operator==(const po::rvalue&, const po::rvalue&);
		friend bool operator!=(const po::rvalue&, const po::rvalue&);
		friend bool operator<(const po::rvalue&, const po::rvalue&);

		friend bool is_constant(const rvalue &);
		friend bool is_undefined(const rvalue &);
		friend bool is_variable(const rvalue &);
		friend bool is_memory(const rvalue &);

		friend const constant& to_constant(const rvalue&);
		friend const variable& to_variable(const rvalue&);
		friend const memory& to_memory(const rvalue&);
		template<typename> friend struct std::hash;
	};

	/// @returns true if this is a constant value. The to_constant() function will convert to a constant instance.
	bool is_constant(const rvalue&);

	/// @returns true if this is a undefined (default-constructed) value.
	bool is_undefined(const rvalue&);

	/// @returns true if this is a variable. The to_variable() function will convert to a variable instance.
	bool is_variable(const rvalue&);

	/// @returns true if this is a constant value. The to_memory() function will convert to a memory instance.
	bool is_memory(const rvalue&);

	/// @returns true if this is a valid assignment target (left side of a intruction). The toLvalue() function will convert to a lvalue instance.
	bool is_lvalue(const rvalue&);

	/**
	 * Cast this instance to a constant
	 * @throws value_exception if not a constant.
	 */
	const constant& to_constant(const rvalue&);

	/**
	 * Cast this instance to a variable
	 * @throws value_exception if not a variable.
	 */
	const variable& to_variable(const rvalue&);

	/**
	 * Cast this instance to a memory value
	 * @throws value_exception if not a memory.
	 */
	const memory& to_memory(const rvalue&);

	/**
	 * Cast this instance to a lvalue
	 * @throws value_exception if not a lvalue.
	 */
	lvalue to_lvalue(const rvalue&);

	bool operator==(const rvalue&, const rvalue&);
	bool operator!=(const rvalue&, const rvalue&);
	bool operator<(const rvalue&, const rvalue&);
	std::ostream& operator<<(std::ostream&, const rvalue &);

	template<>
	std::unique_ptr<rvalue> unmarshal(const uuid&, const rdf::storage&);

	template<>
	archive marshal(rvalue const&, const uuid&);

	/**
	 * @brief Exception associated with rvalue subclasses
	 *
	 * This exception if thrown if invalid parameters to a data type constructor
	 * are found or a invlaid cast from rvalue is requested.
	 */
	class value_exception : public std::runtime_error
	{
	public:
		/// Conastructs a exception for error message @c w.
		value_exception(const std::string &w);
	};
}

namespace std
{
	template<>
	struct hash<po::Endianess>
	{
		size_t operator()(po::Endianess a) const
		{
			return hash<uint8_t>()(a);
		}
	};

	template<>
	struct hash<po::rvalue>
	{
		size_t operator()(const po::rvalue &a) const
		{
			if(is_memory(a))
			{
				const po::memory &m = to_memory(a);
				return po::hash_struct(m.name(),m.bytes(),m.endianess(),m.offset());
			}
			else if(is_constant(a))
			{
				const po::constant &c = to_constant(a);
				return po::hash_struct(c.content());
			}
			else if(is_variable(a))
			{
				const po::variable &v = to_variable(a);
				return po::hash_struct(v.name(),v.width(),v.subscript());
			}
			else if(is_undefined(a))
			{
				return po::hash_struct(0,1);
			}
			else
			{
				ensure(false);
			}
		}
	};

	template<>
	struct hash<po::variable>
	{
		size_t operator()(const po::variable &a) const { return hash<po::rvalue>()(a); }
	};
}

#ifndef _MSC_VER
/// One bit wide constant
inline po::constant operator"" _i(unsigned long long n)
{
	return po::constant(n);
}

/// One bit wide variable
inline po::variable operator"" _v1(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,1);
}

/// Eight bit wide variable
inline po::variable operator"" _v8(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,8);
}

/// Sixteen bit wide variable
inline po::variable operator"" _v16(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,16);
}

/// Thirtytwo bit wide variable
inline po::variable operator"" _v32(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,32);
}

/// Sixtyfour bit wide variable
inline po::variable operator"" _v64(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,64);
}
#endif
