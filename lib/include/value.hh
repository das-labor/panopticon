#ifndef VALUE_HH
#define VALUE_HH

#include <unordered_map>
#include <iostream>
#include <cstdint>
#include <cassert>
#include <stdexcept>

namespace po
{
	class rvalue;
	class constant;
	class undefined;
	class lvalue;
	class variable;
	class memory;
	class value_exception;

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

		rvalue(void);
		rvalue(const rvalue &r);
		~rvalue(void);

		rvalue &operator=(const rvalue &r);
		
		bool operator<(const rvalue &b) const;
		bool operator==(const rvalue &b) const;
		bool operator!=(const rvalue &b) const;

		Tag tag(void) const;
		
		bool is_constant(void) const;
		bool is_undefined(void) const;
		bool is_variable(void) const;
		bool is_memory(void) const;
		bool is_lvalue(void) const;

		const class constant &constant(void) const;
		const class variable &variable(void) const;
		const class memory &memory(void) const;

		template<class> friend struct std::hash;

	protected:
		union
		{
			uint64_t all;
			struct { uint64_t rest:62, tag:2; } simple;
			struct { uint64_t sub:16, n1:7, n2:7, n3:7, n4:7, n5:7, width:8, rest:3, tag:2; } name; // variable
		} d;
	};

	std::ostream& operator<<(std::ostream &os, const po::rvalue &r);
	
	static_assert(alignof(class rvalue) >= 4,"need class alignment of 4 for pointer tagging");
	static_assert(sizeof(class rvalue) == 8,"rvalue should not be larger than one register");
	static_assert(sizeof(uintptr_t) <= sizeof(class rvalue),"rvalue must be able to hold a pointer value");

	class constant : public rvalue
	{
	public:
		constant(uint64_t v, uint16_t w);

		uint16_t width(void) const;
		uint64_t content(void) const;
	};

	// internal
	struct constant_priv
	{
		unsigned int usage;
		uint64_t content;
		uint16_t width;
	};

	uint64_t flsll(uint64_t);

	class lvalue : public rvalue {};
	class undefined : public lvalue {};

	class variable : public lvalue
	{
	public:
		variable(std::string n, uint16_t w, int s = -1);

		uint16_t width(void) const;
		std::string name(void) const;
		int subscript(void) const;
	};

	class memory : public lvalue
	{	
	public:
		enum Endianess
		{
			NoEndian = 0,
			LittleEndian = 1,
			BigEndian = 2
		};

		memory(rvalue o, uint16_t b, Endianess e, std::string n);

		uint16_t bytes(void) const;
		const rvalue &offset(void) const;
		Endianess endianess(void) const;
		const std::string &name(void) const;
	};

	// internal
	struct memory_priv
	{
		unsigned int usage;
		rvalue offset;
		uint16_t bytes;
		memory::Endianess endianess;
		std::string name;
	};

	class value_exception : public std::runtime_error
	{
	public:
		value_exception(const std::string &w);
	};
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
				const po::memory &m = a.memory();
				
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

inline po::constant operator"" _i1(unsigned long long n)
{
	return po::constant(n,std::max(1,8));
}

inline po::constant operator"" _i8(unsigned long long n)
{
	return po::constant(n,std::max(1,8));
}

inline po::constant operator"" _i16(unsigned long long n)
{
	return po::constant(n,std::max(1,16));
}

inline po::constant operator"" _i32(unsigned long long n)
{
	return po::constant(n,std::max(1,32));
}

inline po::constant operator"" _i64(unsigned long long n)
{
	return po::constant(n,std::max(1,64));
}

inline po::variable operator"" _v1(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,1);
}

inline po::variable operator"" _v8(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,8);
}

inline po::variable operator"" _v16(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,16);
}

inline po::variable operator"" _v32(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,32);
}

inline po::variable operator"" _v64(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base,64);
}

#endif
