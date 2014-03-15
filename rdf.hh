#include <boost/variant.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/optional.hpp>

#include <kcpolydb.h>

#pragma once

using iri = std::string;
using literal = std::pair<std::string,iri>;
using kyotocabinet::PolyDB;
using boost::uuids::uuid;

struct node
{
	static node blank(void);

	node(const iri& n);
	node(const std::string& s, const iri& t);
	node(const uuid& u);

	bool operator==(const node&) const;
	bool operator<(const node&) const;

	bool is_iri(void) const;
	bool is_literal(void) const;
	bool is_blank(void) const;

	const iri& as_iri(void) const;
	const iri& as_literal(void) const;
	const iri& literal_type(void) const;
	const uuid& as_uuid(void) const;

private:
	boost::variant<iri,literal,uuid> _inner;
};

inline node operator"" _lit(unsigned long long i)
{
	return node(std::to_string(i),"xs:integer");
}

inline node operator"" _lit(const char *str, size_t sz)
{
	return node(std::string(str,sz),"xs:string");
}

inline node operator"" _po(const char *str, size_t sz)
{
	return node("http://panopticon.re/rdf/v1/" + std::string(str,sz));
}

inline node operator"" _rdf(const char *str, size_t sz)
{
	return node("http://www.w3.org/1999/02/22-rdf-syntax-ns#" + std::string(str,sz));
}

struct statement
{
	statement(const node& s, const node& p, const node& o);

	bool operator==(const statement&) const;
	bool operator<(const statement&) const;

	node subject, predicate, object;
};

struct iterator
{
	const statement& operator*(void);
};

struct storage
{
	storage(void);
	storage(const std::string& base);
	~storage(void);

	bool insert(const statement& st);
	bool insert(const node&, const node&, const node&);
	bool remove(const statement& st);
	bool remove(const node&, const node&, const node&);

	bool has(const statement& st) const;
	bool has(const node&, const node&, const node&) const;
	std::list<statement> find(const node &s) const;
	std::list<statement> find(const node &s, const node &p) const;
	int64_t count(void);

private:
	using iter = std::string::const_iterator;
	enum node_type : uint8_t
	{
		Blank = 0,
		Literal = 1,
		Named = 2
	};

	static std::string encode_node(const node& n);
	static std::pair<node,iter> decode_node(iter, iter);
	static std::string encode_key(const statement& st);
	static std::pair<statement,iter> decode_key(iter, iter);
	static std::string encode_varint(size_t sz);
	static std::pair<size_t,iter> decode_varint(iter, iter);

	PolyDB _meta; ///< subject/predicate/object
};
