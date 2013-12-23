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

template<>
rvalue *po::unmarshal(const uuid &u, const rdf::storage &store)
{
	rdf::node root = rdf::ns_local(to_string(u));
	rdf::node type = store.first(root,"type"_rdf,boost::none).object();

	if(type == "Undefined"_po)
	{
		return new rvalue(undefined());
	}
	else if(type == "Variable"_po)
	{
		rdf::statement name = store.first(root,"name"_po,boost::none),
									 width = store.first(root,"width"_po,boost::none);

		try
		{
			rdf::statement subscript = store.first(root,"subscript"_po,boost::none);

			return new rvalue(variable(name.object().to_string(),stoull(width.object().to_string()),stoull(subscript.object().to_string())));
		}
		catch(marshal_exception &e)
		{
			return new rvalue(variable(name.object().to_string(),stoull(width.object().to_string())));
		}
	}
	else if(type == "Constant"_po)
	{
		rdf::statement value = store.first(root,"content"_po,boost::none);

		return new rvalue(constant(stoull(value.object().to_string())));
	}
	else if(type == "Memory"_po)
	{
		rdf::statement name = store.first(root,"name"_po,boost::none),
									 offset = store.first(root,"offset"_po,boost::none),
									 bytes = store.first(root,"bytes"_po,boost::none),
									 endianess = store.first(root,"endianess"_po,boost::none);

		uuid ou = boost::uuids::string_generator()(offset.object().to_string());
		std::shared_ptr<rvalue> off(unmarshal<rvalue>(ou,store));
		memory::Endianess e;

		if(endianess.object() == "big-endian"_po)
			e = memory::BigEndian;
		else if(endianess.object() == "little-endian"_po)
			e = memory::LittleEndian;
		else
			throw marshal_exception("unknown endianess");

		return new rvalue(memory(*off,stoull(bytes.object().to_string()),e,name.object().to_string()));
	}
	else
		throw marshal_exception("unknown value type");
}

template<>
rdf::statements po::marshal(const rvalue *rv, const uuid &u)
{
	rdf::statements ret;
	rdf::node root = rdf::ns_local(to_string(u));

	if(!rv)
	{
		return ret;
	}
	else if(is_undefined(*rv))
	{
		ret.emplace_back(root,"type"_rdf,"Undefined"_po);
		return ret;
	}
	else if(is_constant(*rv))
	{
		constant c = to_constant(*rv);

		ret.emplace_back(root,"type"_rdf,"Constant"_po);
		ret.emplace_back(root,"content"_po,rdf::lit(to_string(c.content())));
		return ret;
	}
	else if(is_variable(*rv))
	{
		variable v = to_variable(*rv);

		ret.emplace_back(root,"type"_rdf,"Variable"_po);
		ret.emplace_back(root,"name"_po,rdf::ns_po(v.name()));
		ret.emplace_back(root,"width"_po,rdf::ns_po(to_string(v.width())));
		if(v.subscript() >= 0)
			ret.emplace_back(root,"subscript"_po,rdf::ns_po(to_string(v.subscript())));
		return ret;
	}
	else if(is_memory(*rv))
	{
		memory m = to_memory(*rv);
		uuid ou = boost::uuids::name_generator(u)("offset");

		ret.emplace_back(root,"type"_rdf,"Memory"_po);
		ret.emplace_back(root,"offset"_po,rdf::ns_po(to_string(ou)));
		ret.emplace_back(root,"bytes"_po,rdf::ns_po(to_string(m.bytes())));

		switch(m.endianess())
		{
			case memory::LittleEndian: ret.emplace_back(root,"endianess"_po,"little-endian"_po); break;
			case memory::BigEndian: ret.emplace_back(root,"endianess"_po,"big-endian"_po); break;
			default: throw marshal_exception("unknown endianess");
		}

		ret.emplace_back(root,"name"_po,rdf::ns_po(m.name()));
		auto off_st = marshal(&m.offset(),ou);
		move(off_st.begin(),off_st.end(),back_inserter(ret));

		return ret;
	}
	else
		throw marshal_exception("unknown rvalue type");
}

value_exception::value_exception(const string &w) : runtime_error(w) {}
