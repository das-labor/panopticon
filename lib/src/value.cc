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
					default: ensure(false);
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
			case LittleEndian: os << "←"; break;
			case BigEndian: os << "→"; break;
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

bool po::is_constant(const po::rvalue &v) { return boost::get<constant>(&v._variant); }
bool po::is_memory(const po::rvalue &v) { return boost::get<memory>(&v._variant); }
bool po::is_variable(const po::rvalue &v) { return boost::get<variable>(&v._variant); }
bool po::is_undefined(const po::rvalue &v) { return boost::get<undefined>(&v._variant); }
bool po::is_lvalue(const po::rvalue &a) { return is_variable(a) || is_memory(a) || is_undefined(a); }

const po::constant &po::to_constant(const po::rvalue &a)
{
	try
	{
		return get<constant>(a._variant);
	}
	catch(const boost::bad_get&)
	{
		std::stringstream ss;

		ss << a;
		throw value_exception("Invalid cast from " + ss.str() + " to constant.");
	}
}

const po::variable &po::to_variable(const po::rvalue &a)
{
	try
	{
		return get<variable>(a._variant);
	}
	catch(const boost::bad_get&)
	{
		std::stringstream ss;

		ss << a;
		throw value_exception("Invalid cast from " + ss.str() + " to variable.");
	}
}

const po::memory &po::to_memory(const po::rvalue &a)
{
	try
	{
		return get<memory>(a._variant);
	}
	catch(const boost::bad_get&)
	{
		std::stringstream ss;

		ss << a;
		throw value_exception("Invalid cast from " + ss.str() + " to memory reference.");
	}
}

po::lvalue po::to_lvalue(const po::rvalue &a)
{
	if(is_memory(a)) return to_memory(a);
	if(is_variable(a)) return to_variable(a);
	if(is_undefined(a)) return undefined();

	std::stringstream ss;

	ss << a;
	throw value_exception("Invalid cast from " + ss.str() + " to lvalue.");
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
	if(_name == v._name)
	{
		if(_width == v._width)
		{
			return _subscript < v._subscript;
		}
		else
		{
			return _width < v._width;
		}
	}
	else
	{
		return _name < v._name;
	}
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
Endianess memory::endianess(void) const { return _endianess; }
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
std::unique_ptr<rvalue> po::unmarshal(const uuid &u, const rdf::storage &store)
{
	rdf::node root = rdf::iri(u);
	rdf::node type = store.first(root,rdf::ns_rdf("type")).object;

	if(type == rdf::ns_po("Undefined"))
	{
		return std::unique_ptr<rvalue>(new rvalue(undefined()));
	}
	else if (type == rdf::ns_po("Variable"))
	{
		rdf::statement name = store.first(root, rdf::ns_po("name")),
			width = store.first(root, rdf::ns_po("width"));

		try
		{
			rdf::statement subscript = store.first(root, rdf::ns_po("subscript"));

			return std::unique_ptr<rvalue>(new rvalue(variable(name.object.as_literal(),stoull(width.object.as_literal()),stoull(subscript.object.as_literal()))));
		}
		catch(marshal_exception &e)
		{
			return std::unique_ptr<rvalue>(new rvalue(variable(name.object.as_literal(),stoull(width.object.as_literal()))));
		}
	}
	else if(type == rdf::ns_po("Constant"))
	{
		rdf::statement value = store.first(root,rdf::ns_po("content"));

		return std::unique_ptr<rvalue>(new rvalue(constant(stoull(value.object.as_literal()))));
	}
	else if(type == rdf::ns_po("Memory"))
	{
		rdf::statement name = store.first(root,rdf::ns_po("name")),
									 offset = store.first(root,rdf::ns_po("offset")),
									 bytes = store.first(root,rdf::ns_po("bytes")),
									 endianess = store.first(root,rdf::ns_po("endianess"));

		uuid ou = offset.object.as_iri().as_uuid();
		std::unique_ptr<rvalue> off(unmarshal<rvalue>(ou,store));
		Endianess e;

		if(endianess.object == rdf::ns_po("big-endian"))
			e = BigEndian;
		else if(endianess.object == rdf::ns_po("little-endian"))
			e = LittleEndian;
		else
			throw marshal_exception("unknown endianess");

		return std::unique_ptr<rvalue>(new rvalue(memory(*off,stoull(bytes.object.as_literal()),e,name.object.as_literal())));
	}
	else
		throw marshal_exception("unknown value type");
}

template<>
archive po::marshal(rvalue const& rv, const uuid &u)
{
	rdf::statements ret;
	std::list<blob> bl;
	rdf::node root = rdf::iri(u);

	if(is_undefined(rv))
	{
		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Undefined"));
	}
	else if(is_constant(rv))
	{
		constant c = to_constant(rv);

		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Constant"));
		ret.emplace_back(root,rdf::ns_po("content"),rdf::lit(c.content()));
	}
	else if(is_variable(rv))
	{
		variable v = to_variable(rv);

		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Variable"));
		ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(v.name()));
		ret.emplace_back(root,rdf::ns_po("width"),rdf::lit(v.width()));
		if(v.subscript() >= 0)
			ret.emplace_back(root,rdf::ns_po("subscript"),rdf::lit(v.subscript()));
	}
	else if(is_memory(rv))
	{
		memory m = to_memory(rv);
		uuid ou = boost::uuids::name_generator(u)("offset");

		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Memory"));
		ret.emplace_back(root,rdf::ns_po("offset"),rdf::iri(ou));
		ret.emplace_back(root,rdf::ns_po("bytes"),rdf::lit(m.bytes()));

		switch(m.endianess())
		{
			case LittleEndian: ret.emplace_back(root,rdf::ns_po("endianess"),rdf::ns_po("little-endian")); break;
			case BigEndian: ret.emplace_back(root,rdf::ns_po("endianess"),rdf::ns_po("big-endian")); break;
			default: throw marshal_exception("unknown endianess");
		}

		ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(m.name()));
		auto off_st = marshal(m.offset(),ou);
		std::move(off_st.triples.begin(),off_st.triples.end(),back_inserter(ret));
		std::move(off_st.blobs.begin(),off_st.blobs.end(),back_inserter(bl));
	}
	else
		throw marshal_exception("unknown rvalue type");

	ensure(ret.size());
	return archive(ret,bl);
}

value_exception::value_exception(const string &w) : runtime_error(w) {}
