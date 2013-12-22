#include <sstream>
#include <string>
#include <cassert>
#include <mutex>
#include <unordered_map>
#include <list>
#include <memory>

extern "C" {
#include <redland.h>
}

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <boost/functional/hash.hpp>
#include <boost/optional.hpp>

#define LOCAL "http://localhost/"
#define PO "http://panopticum.io/"
#define XSD	"http://www.w3.org/2001/XMLSchema#"
#define RDF	"http://www.w3.org/1999/02/22-rdf-syntax-ns#"
#define TEMPDIR_TEMPLATE std::string("panopXXXXXX")

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
		class world;
		class node;
		class statement;
		class stream;
		class list;

		using world_ptr = world*;

		class world : std::enable_shared_from_this<world>
		{
		public:
			static world& instance(void);

			world(const world&) = delete;
			world &operator=(const world&) = delete;

			virtual ~world(void);

			librdf_world *rdf(void) const;
			raptor_world *raptor(void) const;

		private:
			world(void);

			librdf_world *_rdf_world;
			raptor_world *_rap_world;
			static std::unique_ptr<world> _instance;
		};

		class storage
		{
		public:
			static storage from_archive(const std::string &panopPath);
			static storage from_turtle(const std::string &turtlePath);

			storage(void);
			storage(storage &&);
			storage(const storage &) = delete;

			~storage(void);

			storage &operator=(const storage &) = delete;

			rdf::stream select(boost::optional<const rdf::node&> s, boost::optional<const rdf::node&> p, boost::optional<const rdf::node&> o) const;
			rdf::statement first(boost::optional<const rdf::node&> s,boost::optional<const rdf::node&> p,boost::optional<const rdf::node&> o) const;
			bool has(boost::optional<const rdf::node&> s,boost::optional<const rdf::node&> p,boost::optional<const rdf::node&> o) const;

			void insert(const rdf::statement &c);
			void insert(const rdf::node &s, const rdf::node &p, const rdf::node &o);

			void remove(const rdf::statement &);

			void snapshot(const std::string &path);

			std::string dump(const std::string &format) const;

		private:

			storage(bool openStore);

			librdf_storage *_storage;
			librdf_model *_model;
			std::string _tempdir;
		};

		class node
		{
		public:
			node(void);
			explicit node(const std::string &blank_id);
			node(librdf_node *n);
			node(const node &n);
			node(node &&n);

			~node(void);

			node &operator=(const node &n);
			node &operator=(node &&n);

			bool operator==(const node &n) const;
			bool operator!=(const node &n) const;

			std::string to_string(void) const;
			librdf_node *inner(void) const;

		private:
			librdf_node *_node;
		};

		std::ostream& operator<<(std::ostream&, const node&);

		node lit(const std::string &s);
		node lit(unsigned long long n);
		node ns_po(const std::string &s);
		node ns_rdf(const std::string &s);
		node ns_xsd(const std::string &s);
		node ns_local(const std::string &s);

		class statement
		{
		public:
			statement(librdf_statement *st = 0);
			statement(const rdf::node &s, const rdf::node &p, const rdf::node &o);
			statement(const statement &st);
			statement(statement &&st);

			~statement(void);

			statement &operator=(const statement &st);
			statement &operator=(statement &&st);

			node subject(void) const;
			node predicate(void) const;
			node object(void) const;
			librdf_statement *inner(void) const;

		private:
			librdf_statement *_statement;
		};

		std::ostream& operator<<(std::ostream&, const statement&);

		using statements = std::list<statement>;
		using nodes = std::list<node>;

		nodes read_list(const node &n, const storage&);

		template<typename It>
		std::pair<node,statements> write_list(It begin, It end, const std::string &ns = "");

		class stream
		{
		public:
			stream(librdf_stream *s);
			stream(librdf_iterator *i, boost::optional<const rdf::node&> s, boost::optional<const rdf::node&> p, boost::optional<const rdf::node&> o);
			stream(const stream &s) = delete;
			stream(stream &&s);

			~stream(void);

			stream &operator=(const stream &) = delete;
			stream &operator=(stream &&st);

			bool eof(void) const;
			stream &operator>>(statement &st);

		private:
			librdf_stream *_stream;
		};
	}

	inline rdf::node operator"" _lit(const char *_s, std::size_t l)
	{
		std::string s(_s,l);
		rdf::world &w = rdf::world::instance();
		librdf_uri *type = librdf_new_uri(w.rdf(),reinterpret_cast<const unsigned char *>(XSD"string"));
		rdf::node ret(librdf_new_node_from_typed_literal(w.rdf(),reinterpret_cast<const unsigned char *>(s.c_str()),NULL,type));

		librdf_free_uri(type);
		return ret;
	}

	inline rdf::node operator"" _lit(unsigned long long n)
	{
		return rdf::lit(n);
	}

	inline rdf::node operator"" _po(const char *s, std::size_t l)
	{
		return rdf::ns_po(std::string(s,l));
	}

	inline rdf::node operator"" _rdf(const char *s, std::size_t l)
	{
		return rdf::ns_rdf(std::string(s,l));
	}

	inline rdf::node operator"" _xsd(const char *s, std::size_t l)
	{
		return rdf::ns_xsd(std::string(s,l));
	}

	inline rdf::node operator"" _local(const char *s, std::size_t l)
	{
		return rdf::ns_local(std::string(s,l));
	}

	template<typename It>
	std::pair<rdf::node,rdf::statements> rdf::write_list(It begin, It end, const std::string &ns)
	{
		rdf::statements ret;
		int counter = 0;
		std::function<node(void)> blank = [&](void) { return ns.empty() ? node() : node(ns + std::to_string(counter++)); };
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
	 * T* unmarshall(const uuid&, const rdf::storage&);
	 */

	template<typename T>
	T* unmarshal(const uuid&,const rdf::storage&);

	template<typename T>
	rdf::statements marshal(const T*, const uuid&);
}
