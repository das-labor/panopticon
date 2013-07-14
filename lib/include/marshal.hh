#ifndef MARSHAL_HH
#define MARSHAL_HH

#include <sstream>
#include <string>
#include <iostream>
#include <cassert>
#include <mutex>
#include <unordered_map>
#include <stack>
#include <list>
#include <memory>

extern "C" {
#include <redland.h>
}

#define LOCAL "http://localhost/"
#define PO "http://panopticum.io/"
#define XSD	"http://www.w3.org/2001/XMLSchema#"
#define RDF	"http://www.w3.org/1999/02/22-rdf-syntax-ns#"
#define TEMPDIR_TEMPLATE std::string("panopXXXXXX")

/**
 * @file
 * @brief Serializing rotines
 *
 * Serializing flowgraphs is done with custom streams. Each class is responsible for
 * implementing its own stream out operators.
 */

namespace po
{
	/**
	 * @brief Output as graph in DOT language.
	 *
	 * This stream converts objects into a graph that can be drawn with 
	 * graphviz. Stream manipulators exists to enable/disable generation of
	 * call edges, procedure bodies and IL dumps.
	 *
	 * The class derives from stringstream. The graph is read with str().
	 * @note Output is large and only useful for debugging
	 */
	class odotstream : public std::ostringstream
	{
	public:
		odotstream(void);

		/// Draw calls as edges between procedures
		bool calls;

		/// Draw procedure bodies as control flow graphs
		bool body;

		/// If body is true draw the prodecures as DOT cluster
		bool subgraph;

		/// Dump IL code after mnemonic
		bool instrs;
	};
	
	odotstream &operator<<(odotstream &os, odotstream &(*func)(odotstream &os));

	odotstream &calls(odotstream &os);
	odotstream &nocalls(odotstream &os);
	odotstream &body(odotstream &os);
	odotstream &nobody(odotstream &os);
	odotstream &subgraph(odotstream &os);
	odotstream &nosubgraph(odotstream &os);
	odotstream &instrs(odotstream &os);
	odotstream &noinstrs(odotstream &os);

	/// @returns unique identifier for @c t that can be used in the DOT and Turtle output.
	template<typename T>
	std::string unique_name(const T &t)
	{
		return std::string("generic_") + std::to_string((uintptr_t)&t);
	}

	/// Makes all stream out operators defined for ostringstream usable for odotstream.
	template<typename T>
	odotstream &operator<<(odotstream &os, const T &t)
	{
		static_cast<std::ostringstream &>(os) << t;
		return os;
	}

	/**
	 * @brief Output as RDF graph in Turtle syntax
	 *
	 * Serializes to a collection of triples that describe a RDF graph.
	 * This is the main serialization format of Panopticum and the only one
	 * that can be deserialized.
	 *
	 * Access the Turtle syntax with str().
	 */
	class oturtlestream : public std::ostringstream
	{
	public:
		oturtlestream(void);

		/// @returns a new blank node to be used in the output. Guaranteed to be unique.
		std::string blank(void);

		/// Supresses double quotes around rvalue's
		bool embed;
	
	private:
		unsigned long long m_blank;
	};
	
	oturtlestream &embed(oturtlestream &os);
	oturtlestream &noembed(oturtlestream &os);

	oturtlestream &operator<<(oturtlestream &os, oturtlestream &(*func)(oturtlestream &os));
	oturtlestream &operator<<(oturtlestream &os, std::ostream& (*func)(std::ostream&));

	/// Makes all stream out operators defined for ostringstream usable for oturtlestream.
	template<typename T>
	oturtlestream &operator<<(oturtlestream &os, const T &t)
	{
		static_cast<std::ostringstream &>(os) << t;
		return os;
	}

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

		typedef world* world_ptr;

		class world : std::enable_shared_from_this<world>
		{
		public:
			static world_ptr instance(void);

			librdf_world *rdf(void) const;
			raptor_world *raptor(void) const;

			struct proxy
			{
				proxy(std::nullptr_t);
				proxy(const std::string &s);
				proxy(const char *s);
				proxy(const node &n);
				~proxy(void);

				proxy(const proxy &p);
				proxy &operator=(const proxy &p);

				bool is_literal;
				bool is_node;
				std::string literal;
				librdf_node *node;
			};

		private:
			world(void);
			~world(void);

			librdf_world *m_rdf_world;
			raptor_world *m_rap_world;
			static world *s_instance;
		};

		class storage
		{
		public:
			static storage from_archive(const std::string &panopPath);
			static storage from_stream(const oturtlestream &os);
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

			void snapshot(const std::string &path);
			
		private:
		
			storage(bool openStore);

			librdf_storage *m_storage;
			librdf_model *m_model;
			std::string m_tempdir;
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
			librdf_node *m_node;
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
			librdf_statement *m_statement;
		};

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
			librdf_stream *m_stream;
		};

		class list
		{
		public:
			list(void);
			
			void insert(const rdf::node &s);
			rdf::stream to_stream(const rdf::storage &store);

		private:
			std::list<rdf::node> m_list;
		};
	}

	class ordfstream
	{
	public:
		ordfstream(rdf::storage &store);

		std::stack<rdf::node> &context(void);
		rdf::storage &store(void) const;
	
	private:
		rdf::storage &m_storage;
		std::stack<rdf::node> m_context;
	};

	ordfstream& operator<<(ordfstream &os, const rdf::statement &st);
	
	inline rdf::node operator"" _lit(const char *s)
	{
		rdf::world_ptr w = rdf::world::instance();
		librdf_uri *type = librdf_new_uri(w->rdf(),reinterpret_cast<const unsigned char *>(XSD"string"));
		rdf::node ret(librdf_new_node_from_typed_literal(w->rdf(),reinterpret_cast<const unsigned char *>(s),NULL,type));

		librdf_free_uri(type);
		return ret;
	}
	
	inline rdf::node operator"" _lit(unsigned long long n)
	{
		return rdf::lit(n);
	}

	inline rdf::node operator"" _po(const char *s, unsigned long l)
	{			
		return rdf::ns_po(std::string(s,l));
	}
	
	inline rdf::node operator"" _rdf(const char *s, unsigned long l)
	{			
		return rdf::ns_rdf(std::string(s,l));
	}
	
	inline rdf::node operator"" _xsd(const char *s, unsigned long l)
	{			
		return rdf::ns_xsd(std::string(s,l));
	}
}

#endif
