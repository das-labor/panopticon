#ifndef VALUE_HH
#define VALUE_HH

#include <unordered_map>
#include <iostream>
#include <cstdint>
#include <cassert>
#include <stdexcept>

#include <inflate.hh>

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
 * Converting between classes can be done with constant(), variable(), memory(), ... in rvalue.
 *
 * Internally rvalue is a tagged union where the tag value is stored by tagging a pointers last 3 bits.
 * The sub classes of rvalue add functions to query one type of the union. Memory and constant use a 
 * heap allocated structure to save its members. Variable fits into the 61 bits before the tag.
 */

namespace po
{
	class rvalue;
	class constant;
	class undefined;
	class lvalue;
	class variable;
	class memory;
	class value_exception;

	/**
	 * @brief Base of all data types the IL operates on.
	 *
	 * Aside from various support routines, rvalue implements
	 * secure conversion to its sub classes.
	 */
	class rvalue
	{
	public:
		enum Tag
		{
			UndefinedValueTag = 0,
			ConstantValueTag = 1,
			VariableValueTag = 2,
			MemoryValueTag = 3
		};

		/// Constructs a undefined value.
		rvalue(void);
		rvalue(const rvalue &r);
		~rvalue(void);

		rvalue &operator=(const rvalue &r);
	
		bool operator<(const rvalue &b) const;
		bool operator==(const rvalue &b) const;
		bool operator!=(const rvalue &b) const;

		/// @returns Tag of the union. Use is_* to query for specific values.
		Tag tag(void) const;
		
		/// @returns true if this is a constant value. The to_constant() function will convert to a constant instance.
		bool is_constant(void) const;
		
		/// @returns true if this is a undefined (default-constructed) value. 
		bool is_undefined(void) const;
		
		/// @returns true if this is a variable. The to_variable() function will convert to a variable instance.
		bool is_variable(void) const;
		
		/// @returns true if this is a constant value. The to_memory() function will convert to a memory instance.
		bool is_memory(void) const;
		
		/// @returns true if this is a valid assignment target (left side of a intruction). The toLvalue() function will convert to a lvalue instance.
		bool is_lvalue(void) const;

		/**
		 * Cast this instance to a constant
		 * @returns Cast of 'this' to 'constant' type
		 * @throws value_exception if not a constant.
		 */
		const class constant &to_constant(void) const;

		/**
		 * Cast this instance to a variable
		 * @returns Cast of 'this' to 'variable' type
		 * @throws value_exception if not a variable.
		 */
		const class variable &to_variable(void) const;
		
		/**
		 * Cast this instance to a memory value
		 * @returns Cast of 'this' to 'memory' type
		 * @throws value_exception if not a memory.
		 */
		const class memory &to_memory(void) const;

		template<class> friend struct std::hash;

	protected:
		union
		{
			uint64_t all;
			struct { uint64_t rest:62, tag:2; } simple;
			struct { uint64_t sub:16, n1:7, n2:7, n3:7, n4:7, n5:7, width:8, rest:3, tag:2; } name; // variable
		} d;

		/// Releases its reference to a memory_priv instance if a memory value
		void destruct_memory(void);
		
		/// Releases its reference to a constant_priv instance if a constant
		void destruct_constant(void);
		
		/// Replaces this instance with @c r, getting ahold of the memory_priv instance of @c r.
		void assign_memory(const class memory &r);
		
		/// Replaces this instance with @c r, getting ahold of the constant_priv instance of @c r.
		void assign_constant(const class constant &r);
	};

	std::ostream& operator<<(std::ostream &os, const po::rvalue &r);
	
	// Make sure we don't introduce bug accedatially.
	static_assert(alignof(class rvalue) >= 4,"need class alignment of 4 for pointer tagging");
	static_assert(sizeof(class rvalue) == 8,"rvalue should not be larger than one register");
	static_assert(sizeof(uintptr_t) <= sizeof(class rvalue),"rvalue must be able to hold a pointer value");

	/**
	 * @brief A constant value with fixed width
	 *
	 * This rvalue subclass models a constant value as a unsigned integer
	 * of known width in bits. The user-defined literals _i8, _i16, _i32, _i64
	 * are shortcuts for constucting constant of 8, 16, 32 or 64 bits.
	 */
	class constant : public rvalue
	{
	public:
		/// Construct a new constant @c v with width of @c w. The @c v argument is trucated to @c w bits.
		constant(uint64_t v, uint16_t w);

		/// @returns width of this constant in bits
		uint16_t width(void) const;

		/// @returns integer value of this constant. Never larger than 1 << width()
		uint64_t content(void) const;
	};

	/**
	 * @brief Internal. Do not use.
	 * @ingroup internal
	 */
	struct constant_priv
	{
		unsigned int usage;
		uint64_t content;
		uint16_t width;
	};

	/// @returns floor(log2(x)), by looking for the last set bit.
	uint64_t flsll(uint64_t);

	/**
	 * @brief A data type than can be written to.
	 *
	 * This is the parent of all valid targets of a assignment 'memory' and 'variable'.
	 */
	class lvalue : public rvalue {};

	/**
	 * @brief Undefined value
	 */
	class undefined : public lvalue {};

	/**
	 * @brief A variable with fixed width.
	 *
	 * Variables loosely model registers with a fixed width in bits. 
	 * Aside from a name of up to 5 ASCII characters, it also can have a
	 * subscript integer that describes the version of the variable in the
	 * SSA form of the procedure. User-defined literals _v1, _v8, _v16, _v32, _v64
	 * are shortcuts for constructing 1, 8, 16, 32, and 64 bit wide, unversioned 
	 * variables.
	 */
	class variable : public lvalue
	{
	public:
		/**
		 * Construct a variable with name @c n, width of @c w bits and version @c s.
		 * @note @c n can only include ASCII characters (<= 0x7f) and can not be longer than 5 characters.
		 */
		variable(std::string n, uint16_t w, int s = -1);

		/// @returns width of this varaible in bits
		uint16_t width(void) const;

		/// @return name of the variable
		std::string name(void) const;

		/// @returns version of the variable if in SSA form. -1 means no version (not yet in SSA form)
		int subscript(void) const;
	};

	/**
	 * @brief A reference to a memory slot
	 *
	 * A memory reference is modeled by a offset from the beginning of a named
	 * memory region, the number of bytes to read from this offset and the byte ordering
	 * (endianess) to obey when saving it in a register.
	 */
	class memory : public lvalue
	{	
	public:
		enum Endianess
		{
			NoEndian = 0,
			LittleEndian = 1,
			BigEndian = 2
		};

		/// Construct a new reference to @c b bytes, starting at offset @c o, in memory region @c n, saved in @c e ordering.
		memory(rvalue o, uint16_t b, Endianess e, std::string n);

		/// @returns number of bytes to read from offset().
		uint16_t bytes(void) const;

		/// @returns position in the memory region to read from.
		const rvalue &offset(void) const;

		/// @returns Byte ordering to obey when reading from the reference.
		Endianess endianess(void) const;

		/// @returns name of the memory region this reference points into.
		const std::string &name(void) const;
	};

	/**
	 * @brief Internal. Do not use.
	 * @ingroup internal
	 */
	struct memory_priv
	{
		unsigned int usage;
		rvalue offset;
		uint16_t bytes;
		memory::Endianess endianess;
		std::string name;
	};

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

	/// @returns a rvalue as typed literal
	oturtlestream &operator<<(oturtlestream &os, rvalue r);
	}

namespace std 
{
	template<>
	struct hash<po::rvalue>
	{
		size_t operator()(const po::rvalue &a) const 
		{
			if(a.is_memory())
			{
				hash<unsigned int> ui;
				hash<po::rvalue> v;
				hash<uint8_t> e;
				hash<string> n;
				const po::memory &m = a.to_memory();
				
				return v(m.offset()) ^ ui(m.bytes()) ^ e(m.endianess()) ^ n(m.name());
			}
			else
			{
				hash<uint64_t> ui64;

				return ui64(a.d.all);
			}
		};
	};
}

/// One bit wide constant
inline po::constant operator"" _i1(unsigned long long n)
{
	return po::constant(n,std::max(1,8));
}

/// Eight bit wide constant
inline po::constant operator"" _i8(unsigned long long n)
{
	return po::constant(n,std::max(1,8));
}

/// Sixteen bit wide constant
inline po::constant operator"" _i16(unsigned long long n)
{
	return po::constant(n,std::max(1,16));
}

/// Thirtytwo bit wide constant
inline po::constant operator"" _i32(unsigned long long n)
{
	return po::constant(n,std::max(1,32));
}

/// Sixtyfour bit wide constant
inline po::constant operator"" _i64(unsigned long long n)
{
	return po::constant(n,std::max(1,64));
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
