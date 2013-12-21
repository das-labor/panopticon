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

			rdf::stream select(const rdf::node &s, const rdf::node &p, const rdf::node &o) const;
			rdf::statement first(const rdf::node &s, const rdf::node &p, const rdf::node &o) const;

			void insert(const rdf::statement &c);
			void insert(const rdf::node &s, const rdf::node &p, const rdf::node &o);

			void remove(const rdf::statement &);

			void snapshot(const std::string &path);

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

		node lit(const std::string &s);
		node lit(unsigned long long n);
		node ns_po(const std::string &s);
		node ns_rdf(const std::string &s);
		node ns_xsd(const std::string &s);

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

		using statements = std::list<statement>;

		class stream
		{
		public:
			stream(librdf_stream *s);
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

		class list
		{
		public:
			list(void);

			void insert(const rdf::node &s);
			rdf::stream to_stream(const rdf::storage &store);

		private:
			std::list<rdf::node> _list;
		};
	}

	inline rdf::node operator"" _lit(const char *s)
	{
		rdf::world &w = rdf::world::instance();
		librdf_uri *type = librdf_new_uri(w.rdf(),reinterpret_cast<const unsigned char *>(XSD"string"));
		rdf::node ret(librdf_new_node_from_typed_literal(w.rdf(),reinterpret_cast<const unsigned char *>(s),NULL,type));

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
