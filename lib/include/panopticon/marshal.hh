#include <sstream>
#include <string>
#include <unordered_map>
#include <list>
#include <memory>
#include <random>
#include <atomic>

#include <kcpolydb.h>

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <boost/variant.hpp>
#include <boost/operators.hpp>
#include <boost/functional/hash.hpp>
#include <boost/filesystem.hpp>
#include <boost/optional.hpp>

#include <panopticon/hash.hh>
#include <panopticon/ensure.hh>

#define PO "http://panopticon.re/rdf/v1/"
#define XSD "http://www.w3.org/2001/XMLSchema#"
#define RDF "http://www.w3.org/1999/02/22-rdf-syntax-ns#"

#pragma once

/**
 * @file
 * @brief Serializing rotines
 */

namespace po
{
	struct uuid : boost::uuids::uuid
	{
		uuid(void) : boost::uuids::uuid(generator()) {}
		uuid(const boost::uuids::uuid& u) : boost::uuids::uuid(u) {}
		uuid(const std::string& s) : boost::uuids::uuid(boost::uuids::string_generator()(s)) {}

		static std::mt19937 prng;
		static boost::uuids::basic_random_generator<std::mt19937> generator;
	};
}

namespace std
{
	template<>
	struct hash<po::uuid>
	{
		size_t operator()(const po::uuid &u) const { return boost::hash<boost::uuids::uuid>()(u); }
	};
}

namespace po
{
	struct blob
	{
		blob(const boost::filesystem::path&, const uuid& t = uuid());
		blob(const std::vector<uint8_t>&, const uuid& t = uuid());
		blob(const char*, size_t);
		blob(const blob&);
		~blob(void);

		bool operator==(const blob& f) const { return _reference == f._reference; }

		char* data(void) { return _data; }

		size_t size(void) const { return _size; }
		const char* data(void) const { return _data; }
		const uuid& tag(void) const { return _tag; }
		boost::optional<boost::filesystem::path> path(void) const
			{ return _source ? boost::make_optional(_source->second) : boost::none; }

	private:
		size_t _size;
		boost::optional<std::pair<int,boost::filesystem::path>> _source;
		char* _data;
		uuid _tag;
		std::atomic<unsigned long long>* _reference;
	};

	class marshal_exception : public std::runtime_error
	{
	public:
		marshal_exception(const std::string &s = "");
	};

	namespace rdf
	{
		struct iri
		{
			iri(const std::string&);
			iri(const uuid&);

			bool operator==(const iri&) const;
			bool operator!=(const iri&) const;
			bool operator<(const iri&) const;

			bool is_uuid(void) const;

			uuid as_uuid(void) const;
			const std::string& as_string(void) const;
			const std::string& raw(void) const;

		private:
			std::string _iri;
		};

		using literal = std::pair<std::string,iri>;
		using kyotocabinet::PolyDB;

		struct node : public boost::operators<node>
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
			std::string as_literal(void) const;
			const iri& literal_type(void) const;
			const uuid& blank_id(void) const;

		private:
			boost::variant<iri,literal,uuid> _inner;
		};

		std::ostream& operator<<(std::ostream&, const node&);

		using nodes = std::list<node>;

		struct statement : public boost::operators<statement>
		{
			statement(const node& s, const node& p, const node& o);

			bool operator==(const statement&) const;
			bool operator<(const statement&) const;

			node subject, predicate, object;
		};

		std::ostream& operator<<(std::ostream&, const statement&);

		using statements = std::list<statement>;
	}

	struct archive
	{
		archive(const rdf::statements& st = rdf::statements(), const std::list<blob>& b = std::list<blob>()) : triples(st), blobs(b) {}

		bool operator==(const archive& a) const { return a.triples == triples && a.blobs == blobs; }

		rdf::statements triples;
		std::list<blob> blobs;
	};

	namespace rdf
	{
		struct storage
		{
			using iter = std::string::const_iterator;

			storage(void);
			storage(const storage&);
			storage(const boost::filesystem::path&);
			~storage(void);

			storage& operator=(const storage&);

			bool insert(const statement& st);
			bool insert(const node&, const node&, const node&);
			bool remove(const statement& st);
			bool remove(const node&, const node&, const node&);
			bool register_blob(const blob&);
			bool unregister_blob(const blob&);

			bool has(const statement& st) const;
			bool has(const node&, const node&, const node&) const;
			std::list<statement> all(void) const;
			std::list<statement> find(const node &s) const;
			std::list<statement> find(const node &s, const node &p) const;
			statement first(const node &s, const node &p) const;
			int64_t count(void) const;
			void snapshot(const boost::filesystem::path&) const;
			blob fetch_blob(const uuid&) const;

			static std::string encode_node(const node& n);
			static std::pair<node,iter> decode_node(iter, iter);
			static std::string encode_key(const statement& st);
			static std::pair<statement,iter> decode_key(iter, iter);
			static std::string encode_varint(size_t sz);
			static std::pair<size_t,iter> decode_varint(iter, iter);

		private:
			enum node_type : char
			{
				Blank = 'B',
				Literal = 'L',
				Named = 'N'
			};

			mutable PolyDB _meta; ///< subject/predicate/object
			boost::filesystem::path _tempdir;
			mutable std::list<blob> _blobs;
		};

		inline node lit(const std::string& s) { return node(s,iri(XSD"string")); }
		inline node lit(long long n) { return node(std::to_string(n),iri(XSD"integer")); }
		inline node boolean(bool b) { return node(b ? "true" : "false",iri(XSD"boolean")); }
		inline node ns_po(const std::string& s) { return node(iri(PO + s)); }
		inline node ns_rdf(const std::string& s) { return node(iri(RDF + s)); }
		inline node ns_xsd(const std::string& s) { return node(iri(XSD + s)); }

		template<typename It>
		std::pair<rdf::node,rdf::statements> write_list(It begin, It end, const uuid&);
		nodes read_list(const node &n, const storage &store);
	}

#ifndef _MSC_VER
	inline rdf::node operator"" _lit(unsigned long long i) { return rdf::lit(i); }
	inline rdf::node operator"" _lit(const char *str, size_t sz) { return rdf::lit(std::string(str,sz)); }
	inline rdf::node operator"" _po(const char *s, std::size_t l) { return rdf::ns_po(std::string(s,l)); }
	inline rdf::node operator"" _rdf(const char *s, std::size_t l) { return rdf::ns_rdf(std::string(s,l)); }
	inline rdf::node operator"" _xsd(const char *s, std::size_t l) { return rdf::ns_xsd(std::string(s,l)); }
#endif

	template<typename It>
	std::pair<rdf::node,rdf::statements> rdf::write_list(It begin, It end, const uuid& ns)
	{
		boost::uuids::name_generator ng(ns);
		rdf::statements ret;
		unsigned int counter = 0;
		rdf::node head = std::distance(begin,end) ? rdf::node(iri(ng(std::to_string(counter++)))) : rdf::ns_rdf("nil");

		rdf::node last = head;
		It i = begin;
		while(i != end)
		{
			const rdf::node &n = *i;
			rdf::node next = std::next(i) == end ? rdf::ns_rdf("nil") : rdf::node(iri(ng(std::to_string(counter++))));

			ret.emplace_back(last,rdf::ns_rdf("first"),n);
			ret.emplace_back(last,rdf::ns_rdf("rest"),next);

			last = next;
			++i;
		}

		return std::make_pair(head,ret);
	}

	/*
	 * SerializableConcept
	 *
	 * std::unordered_set<rdf::statement> marshal(const T*, const uuid&);
	 * T* unmarshal(const uuid&, const rdf::storage&);
	 */

	template<typename T>
	T* unmarshal(const uuid&,const rdf::storage&);

	template<typename T>
	archive marshal(const T*, const uuid&);
}

namespace std
{
	template<>
	struct hash<po::rdf::iri>
	{
		size_t operator()(const po::rdf::iri& i) const { return std::hash<std::string>()(i.as_string()); }
	};

	template<>
	struct hash<po::rdf::node>
	{
		size_t operator()(const po::rdf::node& n) const
		{
			if(n.is_iri())
				return hash<po::rdf::iri>{}(n.as_iri());
			else if(n.is_literal())
				return po::hash_struct(n.as_literal(),n.literal_type());
			else if(n.is_blank())
				return hash<po::uuid>()(n.blank_id());
			throw std::runtime_error("unknown node type");
		}
	};
}
