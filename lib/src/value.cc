#include <string>
#include <algorithm>
#include <sstream>
#include <cctype>	// isalnum
#include <cstring> // ffs

#include <strings.h>

#include <value.hh>

using namespace po;

rvalue::rvalue(void) 
{
	d.simple.tag = UndefinedValueTag;
	d.simple.rest = 0;
}

rvalue::rvalue(const rvalue &r)
{
	if(r.is_memory())
	{
		memory_priv *p = (memory_priv *)(r.d.simple.rest << 2);
		++p->usage;
		d.simple.rest = (uint64_t)(p) >> 2;
		d.simple.tag = MemoryValueTag;
	}
	else
		d.all = r.d.all;
}

rvalue &rvalue::operator=(const rvalue &r)
{
	if(is_memory())
	{
		memory_priv *p = (memory_priv *)(d.simple.rest << 2);
		if(!--p->usage)
			delete p;
	}
		
	if(r.is_memory())
	{
		memory_priv *p = (memory_priv *)(r.d.simple.rest << 2);
		
		++p->usage;
		d.all = (uint64_t)(p);
		d.simple.tag = MemoryValueTag;
	}
	else
		d.all = r.d.all;

	return *this;
}

rvalue::~rvalue(void) 
{
	if(is_memory())
	{
		memory_priv *p = (memory_priv *)(d.simple.rest << 2);
		if(!--p->usage)
			delete p;
	}
}

std::ostream &po::operator<<(std::ostream &os, const rvalue &r)
{
	switch(r.tag())
	{
	case rvalue::UndefinedValueTag: os << std::string("⊥"); return os;
	case rvalue::ConstantValueTag: 	os << r.constant().value(); return os;
	case rvalue::VariableValueTag:
	{
		const variable &v = r.variable();

		// base name
		os << v.name();
		
		// subscript
		if(v.subscript() >= 0)
		{
			std::string t = std::to_string(v.subscript());

			std::for_each(t.cbegin(),t.cend(),[&os](const char c)
			{
				switch(c)
				{
					case '0': os << "₀"; break;
					case '1': os << "₁"; break;
					case '2': os << "₂"; break;
					case '3': os << "₃"; break;
					case '4': os << "₄"; break;
					case '5': os << "₅"; break;
					case '6': os << "₆"; break;
					case '7': os << "₇"; break;
					case '8': os << "₈"; break;
					case '9': os << "₉"; break;
					default: assert(false);
				}
			});
		}
		return os;
	}
	case rvalue::MemoryValueTag:
	{
		const memory &m = r.memory();
		
		// name and offset
		os << m.name() << "[" << m.offset() << ";" << m.bytes();

		// endianess
		switch(m.endianess())
		{
			case memory::LittleEndian: os << "←"; break;
			case memory::BigEndian: os << "→"; break;
			default: os << "?"; break;
		}

		os << "]";
		return os;
	}
	default: assert(false);
	}
}

bool rvalue::operator<(const rvalue &b) const
{
	if(is_memory() && b.is_memory())
	{
		const po::memory &am = memory();
		const po::memory &bm = b.memory();

		if(am.name() != bm.name())
			return am.name() < bm.name();
		else if(am.offset() != bm.offset())
			return am.offset() < bm.offset();
		else if(am.endianess() != bm.endianess())
			return am.endianess() < bm.endianess();
		else
			return am.bytes() < bm.bytes();
	}
	else
		return d.all < b.d.all;
}

bool rvalue::operator==(const rvalue &b) const
{	
	if(is_memory() && b.is_memory())
	{
		const po::memory &am = memory();
		const po::memory &bm = b.memory();

		return am.name() == bm.name() &&
					 am.offset() == bm.offset() &&
					 am.endianess() == bm.endianess() &&
					 am.bytes() == bm.bytes();
	}
	else
		return d.all == b.d.all;
}

bool rvalue::operator!=(const rvalue &b) const
{
	return !(*this == b);
}
rvalue::Tag rvalue::tag(void) const { return (Tag)d.simple.tag; }
	
bool rvalue::is_constant(void) const { return d.simple.tag == ConstantValueTag; }
bool rvalue::is_undefined(void) const { return d.simple.tag == UndefinedValueTag; }
bool rvalue::is_variable(void) const { return d.simple.tag == VariableValueTag; }
bool rvalue::is_memory(void) const { return d.simple.tag == MemoryValueTag; }
bool rvalue::is_lvalue(void) const { return is_memory() || is_variable(); }

const constant &rvalue::constant(void) const { assert(is_constant()); return *reinterpret_cast<const class constant *>(this); }
const variable &rvalue::variable(void) const { assert(is_variable()); return *reinterpret_cast<const class variable *>(this); }
const memory &rvalue::memory(void) const { assert(is_memory()); return *reinterpret_cast<const class memory *>(this); }

constant::constant(uint32_t n)
{
	d.simple.tag = ConstantValueTag;
	d.simple.rest = n;
}

uint64_t constant::value(void) const
{
	return d.simple.rest;
}

variable::variable(std::string b, int s, uint8_t w)
{
	assert(b.size() <= 6 && all_of(b.begin(),b.end(),[&](const char &c) { return c <= 0x7f; }));
	
	d.name.tag = VariableValueTag;
	d.name.n1 = b.size() >= 1 ? b.data()[0] : 0;
	d.name.n2 = b.size() >= 2 ? b.data()[1] : 0;
	d.name.n3 = b.size() >= 3 ? b.data()[2] : 0;
	d.name.n4 = b.size() >= 4 ? b.data()[3] : 0;
	d.name.n5 = b.size() >= 5 ? b.data()[4] : 0;
	//d.name.n6 = b.size() >= 6 ? b.data()[5] : 0;
	d.name.sub = (s < 0 ? 0xffff : (unsigned int)s);
	d.name.width = w;
}

std::string variable::name(void) const
{
	std::stringstream ss;

	ss << (d.name.n1 ? std::string(1,(char)d.name.n1) : "") 
		 << (d.name.n2 ? std::string(1,(char)d.name.n2) : "") 
		 << (d.name.n3 ? std::string(1,(char)d.name.n3) : "") 
		 << (d.name.n4 ? std::string(1,(char)d.name.n4) : "") 
		 << (d.name.n5 ? std::string(1,(char)d.name.n5) : "");
		// << (d.name.n6 ? std::string(1,(char)d.name.n6) : "");
	return ss.str();
}

int variable::subscript(void) const
{
	return d.name.sub != 0xffff ? d.name.sub : -1;
}

uint8_t variable::width(void) const
{
	return d.name.width;
}

memory::memory(rvalue o, unsigned int w, Endianess e, std::string n)
{
	memory_priv *p = new memory_priv();
	p->offset = o;
	p->bytes = w;
	p->endianess = e;
	p->name = n;
	p->usage = 1;

	d.simple.rest = (uint64_t)(p) >> 2;
	d.simple.tag = MemoryValueTag;
}	

const rvalue &memory::offset(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->offset; 
}

unsigned int memory::bytes(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->bytes; 
}

memory::Endianess memory::endianess(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->endianess; 
}

const std::string &memory::name(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->name; 
}
