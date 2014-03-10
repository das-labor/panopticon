#include <boost/variant.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/optional.hpp>

#include <kcpolydb.h>

#pragma once

using iri = std::string;
using literal = std::pair<std::string,iri>;
using kyotocabinet::PolyDB;

struct node
{
	boost::variant<iri,literal,boost::uuids::uuid> _inner;
};

struct statement
{
	node subject, predicate, object;
};

struct iterator
{
	const statement& operator*(void);
};

struct storage
{
	storage(const std::string& base);
	~storage(void);

	bool insert(const std::string& s, const std::string& p, const std::string& o);

	bool has(const std::string& s, const std::string& p, const std::string& o) const;
	std::list<std::tuple<std::string,std::string,std::string>> find(const std::string &s) const;
	std::list<std::tuple<std::string,std::string,std::string>> find(const std::string &s, const std::string &p) const;

private:
	static std::string encode_key(const std::string& s, const std::string& p, const std::string& o);
	static std::tuple<std::string,std::string,std::string> decode_key(const std::string& k);
	static std::string encode_varint(size_t sz);
	static size_t decode_varint(const std::string& a);

	PolyDB _meta; ///< subject/predicate/object
};
