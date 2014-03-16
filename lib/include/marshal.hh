#include <sstream>
#include <string>
#include <cassert>
#include <mutex>
#include <unordered_map>
#include <list>
#include <memory>

#include <kcpolydb.h>

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <boost/functional/hash.hpp>
#include <boost/optional.hpp>
#include <boost/variant.hpp>

#define LOCAL "http://localhost/"
#define PO "http://panopticon.re/rdf/v1/"
#define XSD	"http://www.w3.org/2001/XMLSchema#"
#define RDF	"http://www.w3.org/1999/02/22-rdf-syntax-ns#"
#define TEMPDIR_TEMPLATE std::string("panopXXXXXX")

#pragma once

/**
 * @file
 * @brief Serializing rotines
 */

namespace po
{
	using uuid = boost::uuids::uuid;
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
	class marshal_exception : public std::runtime_error
	{
	public:
		marshal_exception(const std::string &s = "");
	};

	namespace rdf
	{
		using iri = std::string;
		using literal = std::pair<std::string,iri>;
		using kyotocabinet::PolyDB;

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

		using nodes = std::list<node>;

		struct statement
		{
			statement(const node& s, const node& p, const node& o);

			bool operator==(const statement&) const;
			bool operator<(const statement&) const;

			node subject, predicate, object;
		};

		using statements = std::list<statement>;

		struct storage
		{
			using iter = std::string::const_iterator;

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
			int64_t count(void) const;

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
		};

		inline node lit(const std::string& s) { return node(s,XSD"string"); }
		inline node lit(long long n) { return node(std::to_string(n),XSD"integer"); }
		inline node ns_po(const std::string& s) { return node(PO + s); }
		inline node ns_rdf(const std::string& s) { return node(RDF + s); }
		inline node ns_xsd(const std::string& s) { return node(XSD + s); }
		inline node ns_local(const std::string& s) { return node(LOCAL + s); }

		template<typename It>
		std::pair<rdf::node,rdf::statements> write_list(It begin, It end, const std::string &ns);
		nodes read_list(const node &n, const storage &store)
	}

	inline rdf::node operator"" _lit(unsigned long long i) { return rdf::lit(i); }
	inline rdf::node operator"" _lit(const char *str, size_t sz) { return rdf::lit(std::string(str,sz)); }
	inline rdf::node operator"" _po(const char *s, std::size_t l) { return rdf::ns_po(std::string(s,l)); }
	inline rdf::node operator"" _rdf(const char *s, std::size_t l) { return rdf::ns_rdf(std::string(s,l)); }
	inline rdf::node operator"" _xsd(const char *s, std::size_t l) { return rdf::ns_xsd(std::string(s,l)); }
	inline rdf::node operator"" _local(const char *s, std::size_t l) { return rdf::ns_local(std::string(s,l)); }

	template<typename It>
	std::pair<rdf::node,rdf::statements> rdf::write_list(It begin, It end, const std::string &ns)
	{
		rdf::statements ret;
		int counter = 0;
		std::function<node(void)> blank = [&](void) { return ns.empty() ? node::blank() : node(ns + std::to_string(counter++)); };
		rdf::node head = (std::distance(begin,end) ? blank() : "nil"_rdf);

		rdf::node last = head;
		It i = begin;
		while(i != end)
		{
			const rdf::node &n = *i;
			rdf::node next = (std::next(i) == end ? "nil"_rdf : blank());

			ret.emplace_back(last,"first"_rdf,n);
			ret.emplace_back(last,"rest"_rdf,next);

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
	rdf::statements marshal(const T*, const uuid&);
}
