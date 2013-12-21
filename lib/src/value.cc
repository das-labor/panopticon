#include <string>
#include <algorithm>

#include <panopticon/value.hh>

using namespace po;
using namespace std;

rvalue::rvalue(void) : _variant(undefined()) {}

ostream &po::operator<<(ostream &os, const rvalue &r)
{
	if(is_undefined(r))
		os << string("⊥");
	else if(is_constant(r))
		os << to_constant(r).content();
	else if(is_variable(r))
	{
		const variable &v = to_variable(r);

		// base name
		os << v.name();

		// subscript
		if(v.subscript() >= 0)
		{
			string t = to_string(v.subscript());

			for_each(t.cbegin(),t.cend(),[&os](const char c)
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
	}
	else if(is_memory(r))
	{
		const memory &m = to_memory(r);

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
	}
	else
		throw value_exception("Unknown value type");

	return os;
}

bool po::operator==(const po::rvalue &a, const po::rvalue &b) { return a._variant == b._variant; }
bool po::operator!=(const po::rvalue &a, const po::rvalue &b) { return !(a._variant == b._variant); }
bool po::operator<(const po::rvalue &a, const po::rvalue &b) { return a._variant < b._variant; }

bool po::is_constant(const po::rvalue &v) { try { boost::get<constant>(v._variant); return true; } catch(const boost::bad_get&) { return false; } }
bool po::is_memory(const po::rvalue &v) { try { boost::get<memory>(v._variant); return true; } catch(const boost::bad_get&) { return false; } }
bool po::is_variable(const po::rvalue &v) { try { boost::get<variable>(v._variant); return true; } catch(const boost::bad_get&) { return false; } }
bool po::is_undefined(const po::rvalue &v) { try { boost::get<undefined>(v._variant); return true; } catch(const boost::bad_get&) { return false; } }
bool po::is_lvalue(const po::rvalue &a) { return is_variable(a) || is_memory(a) || is_undefined(a); }

const po::constant &po::to_constant(const po::rvalue &a) { try { return get<constant>(a._variant); } catch(const boost::bad_get&) { throw value_exception("Cast to constant from invalid type"); } }
const po::variable &po::to_variable(const po::rvalue &a) { try { return get<variable>(a._variant); } catch(const boost::bad_get&) { throw value_exception("Cast to variable from invalid type"); } }
const po::memory &po::to_memory(const po::rvalue &a) { try { return get<memory>(a._variant); } catch(const boost::bad_get&) { throw value_exception("Cast to memory from invalid type"); } }
po::lvalue po::to_lvalue(const po::rvalue &a)
{
	if(is_memory(a)) return to_memory(a);
	if(is_variable(a)) return to_variable(a);
	if(is_undefined(a)) return undefined();
	throw value_exception("Cast to lvalue from invalid type");
}

bool undefined::operator<(const undefined&) const { return false; }
bool undefined::operator==(const undefined&) const { return true; }

constant::constant(uint64_t n) : _content(n) {}
uint64_t constant::content(void) const { return _content; }
bool constant::operator==(const constant &c) const { return _content == c._content; }
bool constant::operator<(const constant &c) const { return _content < c._content; }

variable::variable(const string &b, uint16_t w, int s)
: _width(w), _name(b), _subscript(s)
{
	if(b.empty())
		throw value_exception("anonymous variable");
	if(!w)
		throw value_exception("variable w/ zero width");
}

const string& variable::name(void) const { return _name; }
int variable::subscript(void) const { return _subscript; }
uint16_t variable::width(void) const { return _width; }
bool variable::operator==(const variable &v) const { return _name == v._name && _subscript == v._subscript && _width == v._width; }
bool variable::operator<(const variable &v) const
{
	return _width == v._width ? (_name == v._name ? _subscript < v._subscript : _name < v._name) : _width < v._width;
}

memory::memory(const rvalue &o, uint16_t w, Endianess e, const string &n)
: _offset(new rvalue(o)), _bytes(w), _endianess(e), _name(n)
{
	if(n.empty())
		throw value_exception("Memory bank name must not be empty");
	if(!w)
		throw value_exception("Memory bytes read must be non-zero");
}

memory::memory(const memory &m) : _offset(new rvalue(*m._offset)), _bytes(m._bytes), _endianess(m._endianess), _name(m._name) {}
memory &memory::operator=(const memory &m)
{
	_offset.reset(new rvalue(*m._offset));
	_bytes = m._bytes;
	_endianess = m._endianess;
	_name = m._name;

	return *this;
}

const rvalue &memory::offset(void) const { return *_offset; }
uint16_t memory::bytes(void) const { return _bytes; }
memory::Endianess memory::endianess(void) const { return _endianess; }
const string &memory::name(void) const { return _name; }
bool memory::operator==(const memory &m) const
{
	return *_offset == *m._offset && _bytes == m._bytes &&
				 _endianess == m._endianess && _name == m._name;
}

bool memory::operator<(const memory &m) const
{
	return _name == m._name ? (*_offset == *m._offset ? (_bytes == m._bytes ? _endianess < m._endianess : _bytes < m._bytes) : *_offset < *m._offset) : _name < m._name;
}
/*
rvalue rvalue::unmarshal(const rdf::node &node, const rdf::storage &store)
{
	if(node == "undefined"_po)
	{
		return undefined();
	}
	else
	{
		rdf::statement type = store.first(node,"type"_rdf,nullptr);

		if(type.object() == "Variable"_po)
		{
			rdf::statement name = store.first(node,"name"_po,nullptr),
										 width = store.first(node,"width"_po,nullptr);

			try
			{
				rdf::statement subscript = store.first(node,"subscript"_po,nullptr);

				return variable(name.object().to_string(),stoull(width.object().to_string()),stoull(subscript.object().to_string()));
			}
			catch(marshal_exception &e)
			{
				return variable(name.object().to_string(),stoull(width.object().to_string()));
			}
		}
		else if(type.object() == "Constant"_po)
		{
			rdf::statement value = store.first(node,"value"_po,nullptr),
										 width = store.first(node,"width"_po,nullptr);

			return constant(stoull(value.object().to_string()),stoull(width.object().to_string()));
		}
		else if(type.object() == "Memory"_po)
		{
			rdf::statement name = store.first(node,"name"_po,nullptr),
										 offset = store.first(node,"offset"_po,nullptr),
										 bytes = store.first(node,"bytes"_po,nullptr),
										 endianess = store.first(node,"endianess"_po,nullptr);

			rvalue off = rvalue::unmarshal(offset.object(),store);
			memory::Endianess e;

			if(endianess.object() == "big-endian"_po)
				e = memory::BigEndian;
			else if(endianess.object() == "little-endian"_po)
				e = memory::LittleEndian;
			else
				e = memory::NoEndian;

			return memory(off,stoull(bytes.object().to_string()),e,name.object().to_string());
		}
		else
			throw marshal_exception("unknown value type");
	}
}

lvalue lvalue::unmarshal(const rdf::node &node, const rdf::storage &store)
{
	rvalue ret = rvalue::unmarshal(node,store);

	if(ret.is_lvalue())
		return ret.to_lvalue();
	else
		throw marshal_exception("not a lvalue");
}*/

value_exception::value_exception(const string &w) : runtime_error(w) {}
