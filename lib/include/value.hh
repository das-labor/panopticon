#ifndef VALUE_HH
#define VALUE_HH

#include <unordered_map>
#include <iostream>
#include <cstdint>
#include <cassert>

namespace po
{
	class rvalue;
	class constant;
	class undefined;
	class variable;
	class memory;

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

		template<class> friend struct ::std::hash;

	protected:
		union
		{
			uint64_t all;
			struct { uint64_t rest:62, tag:2; } simple;
			struct { uint64_t sub:16, n1:7, n2:7, n3:7, n4:7, n5:7, width:8, rest:3, tag:2; } name; // variable
		} d;
	};

	::std::ostream& operator<<(::std::ostream &os, const po::rvalue &r);
	
	static_assert(alignof(class rvalue) >= 4,"need class alignment of 4 for pointer tagging");
	static_assert(sizeof(class rvalue) == 8,"rvalue should not be larger than one machine word");

	class constant : public rvalue
	{
	public:
		constant(uint32_t v);
		uint64_t value(void) const;
	};

	class lvalue : public rvalue {};
	class undefined : public lvalue {};

	class variable : public lvalue
	{
	public:
		variable(::std::string n, int s = -1, uint8_t w = 0);

		::std::string name(void) const;
		int subscript(void) const;
		uint8_t width(void) const;
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

		memory(rvalue o, unsigned int w, Endianess e, ::std::string n);

		const rvalue &offset(void) const;
		unsigned int bytes(void) const;
		Endianess endianess(void) const;
		const ::std::string &name(void) const;
	};

	// internal
	struct memory_priv
	{
		unsigned int usage;
		rvalue offset;
		unsigned int bytes;
		memory::Endianess endianess;
		::std::string name;
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

inline po::constant operator"" _val(uint64_t n)
{
	return po::constant(n);
}

inline po::variable operator"" _var(const char *n, size_t l)
{
	std::string base(n,l);
	return po::variable(base);
}

#endif
