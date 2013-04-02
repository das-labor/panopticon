#ifndef MARSHAL_HH
#define MARSHAL_HH

#include <sstream>
#include <string>
#include <iostream>
#include <cassert>
#include <mutex>
#include <unordered_map>

extern "C" {
#include <redland.h>
}

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
	
	class iturtlestream
	{
	public:
		iturtlestream(const std::string &path);
		~iturtlestream(void);

	private:
		static librdf_world *s_rdf_world;
		static raptor_world *s_rap_world;
		static std::mutex s_mutex;
		static unsigned int s_usage;
		static std::unordered_map<std::string,librdf_node *> s_nodes;

		const librdf_node *po(const std::string &s);
		const librdf_node *rdf(const std::string &s);
		const librdf_node *node(const std::string &s);
		
		librdf_storage *m_storage;
		librdf_model *m_model;
	};
}

#endif
