#include <iostream>
#include <list>
#include <tuple>

#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include "rdf.hh"

using namespace std;

node node::blank(void) { return node(boost::uuids::random_generator()()); }

node::node(const iri& n) : _inner(n) {}
node::node(const string& s, const iri& t) : _inner(make_pair(s,t)) {}
node::node(const uuid& u) : _inner(u) {}

bool node::is_iri(void) const { return !!get<iri>(&_inner); }
bool node::is_literal(void) const { return !!get<pair<string,iri>>(&_inner); }
bool node::is_blank(void) const { return !!get<uuid>(&_inner); }

const iri& node::as_iri(void) const { return get<iri>(_inner); }
const iri& node::as_literal(void) const { return get<pair<string,iri>>(_inner).first; }
const iri& node::literal_type(void) const { return get<pair<string,iri>>(_inner).second; }
const uuid& node::as_uuid(void) const { return get<uuid>(_inner); }

bool node::operator==(const node& n) const
{
	return _inner == n._inner;
}

bool node::operator<(const node& n) const
{
	return _inner < n._inner;
}

statement::statement(const node& s, const node& p, const node& o)
: subject(s), predicate(p), object(o) {}

bool statement::operator==(const statement& st) const
{
	return subject == st.subject &&
				 predicate == st.predicate &&
				 object == st.object;
}

bool statement::operator<(const statement& st) const
{
	return subject < st.subject ||
				 (subject == st.subject && predicate < st.predicate) ||
				 (subject == st.subject && predicate == st.predicate && object < st.object);
}

storage::storage(void)
: _meta()
{
	if(!_meta.open("+",PolyDB::OWRITER | PolyDB::OCREATE))
		throw runtime_error("can't open database");
}

storage::storage(const string& base)
: _meta()
{
	if(!_meta.open(base + "meta.kct",PolyDB::OWRITER | PolyDB::OCREATE))
		throw runtime_error("can't open database");
}

storage::~storage(void)
{
	_meta.close();
}

bool storage::has(const node& s, const node& p, const node& o) const
{
	return has(statement(s,p,o));
}

bool storage::has(const statement& st) const
{
	return _meta.check(encode_key(st)) > -1;
}

list<statement> storage::find(const node &sub, const node &pred) const
{
	list<statement> ret;
	vector<string> keys;
	string s = encode_node(sub), p = encode_node(pred);

	_meta.match_prefix(encode_varint(s.size()) + s + encode_varint(p.size()) + p,&keys);
	transform(keys.begin(),keys.end(),inserter(ret,ret.begin()),[&](const string &k) { return decode_key(k.begin(),k.end()).first; });

	return ret;
}

list<statement> storage::find(const node &sub) const
{
	list<statement> ret;
	vector<string> keys;
	string s = encode_node(sub);

	_meta.match_prefix(encode_varint(s.size()) + s,&keys);
	transform(keys.begin(),keys.end(),inserter(ret,ret.begin()),[&](const string &k) { return decode_key(k.begin(),k.end()).first; });

	return ret;
}

int64_t storage::count(void) const
{
	return _meta.count();
}

bool storage::insert(const node& s, const node& p, const node& o)
{
	return insert(statement(s,p,o));
}

bool storage::insert(const statement& st)
{
	if(has(st))
		return false;

	_meta.set(encode_key(st),"");
	return true;
}

bool storage::remove(const node& s, const node& p, const node& o)
{
	return remove(statement(s,p,o));
}

bool storage::remove(const statement& st)
{
	return _meta.remove(encode_key(st));
}

string storage::encode_node(const node& n)
{
	if(n.is_iri())
		return to_string(Named) + n.as_iri();
	else if(n.is_literal())
		return to_string(Literal) + encode_varint(n.as_literal().size()) + n.as_literal() + n.literal_type();
	else if(n.is_blank())
		return to_string(Blank) + to_string(n.as_uuid());
	else
		throw runtime_error("unknown node type");
}

std::pair<node,storage::iter> storage::decode_node(iter b, iter e)
{
	switch(static_cast<uint8_t>(*b))
	{
		case Named:
			return make_pair(node(iri(string(next(b),e))),e);
		case Literal:
		{
			pair<size_t,iter> len = decode_varint(next(b),e);
			return make_pair(node(string(len.second,next(len.second,len.first))),next(len.second,len.first));
		}
		case Blank:
		{
			boost::uuids::string_generator s;
			return make_pair(node(s(string(next(b),e))),e);
		}
		default:
			throw runtime_error("unknown node type");
	}
}

string storage::encode_key(const statement& st)
{
	string s = encode_node(st.subject), p = encode_node(st.predicate), o = encode_node(st.object);
	return encode_varint(s.size()) + s + encode_varint(p.size()) + p + encode_varint(o.size()) + o;
}

pair<statement,storage::iter> storage::decode_key(iter b, iter e)
{
	pair<size_t,iter> s_sz = decode_varint(b,e);
	pair<node,iter> s = decode_node(s_sz.second,next(s_sz.second,s_sz.first));
	pair<size_t,iter> p_sz = decode_varint(s.second,e);
	pair<node,iter> p = decode_node(p_sz.second,next(p_sz.second,p_sz.first));
	pair<size_t,iter> o_sz = decode_varint(p.second,e);
	pair<node,iter> o = decode_node(o_sz.second,next(o_sz.second,o_sz.first));

	return make_pair(statement(s.first,p.first,o.first),o.second);
}

string storage::encode_varint(size_t sz)
{
	string ret;

	while(sz)
	{
		ret.push_back((sz & 0x7f) | (sz > 0x7f ? 0x80 : 0));
		sz >>= 7;
	}

	return ret;
}

std::pair<size_t,storage::iter> storage::decode_varint(iter b, iter e)
{
	size_t ret = 0;
	uint8_t x = 0;

	do
	{
		x = static_cast<uint8_t>(*b++);
		ret = (ret << 7) | (x & 0x7f);
	}
	while(b != e && x & 0x80);

	return make_pair(ret,b);
}
