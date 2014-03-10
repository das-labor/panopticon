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
	boost::optional<std::tuple<std::string,std::string,std::string>> find(const boost::optional<std::string> &s,const boost::optional<std::string> &p,const boost::optional<std::string> &o) const;

private:
	static std::string encode_key(const std::string& a, const std::string& b);
	static std::pair<std::string,std::string> decode_key(const std::string& k);
	static std::string encode_varint(size_t sz);
	static size_t decode_varint(const std::string& a);

	static std::pair<iterator,iterator> find_all(const PolyDB& db);
	static std::pair<iterator,iterator> find_full(const std::string&a, const std::string& b, int a_pos, int b_pos, int val_pos, const PolyDB& db);
	static std::pair<iterator,iterator> find_exact(const std::string&a, const std::string& b, const std::string &val, int a_pos, int b_pos, int val_pos, const PolyDB& db);
	static std::pair<iterator,iterator> find_partial(const std::string&a, int a_pos, int b_pos, int val_pos, const PolyDB& db);

	// varint string varint string
	PolyDB _sp; //subject_predicate;	// targets
	PolyDB _op; //object_predicate; // sources
	PolyDB _so; //subject_object; // arcs
	PolyDB _po; //predicate_object; // nodes
};
