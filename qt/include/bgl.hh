#ifndef BGL_HH
#define BGL_HH

#include <boost/graph/graph_traits.hpp>
#include <graph.hh>

namespace boost {
template<>
struct graph_traits<Graph *>
{
	struct traversal : public incidence_graph_tag,
														vertex_list_graph_tag,
														edge_list_graph_tag {};

	typedef Node * vertex_descriptor;
	typedef Arrow * edge_descriptor;
	typedef directed_tag directed_category;
	typedef allow_parallel_edge_tag edge_parallel_category;
	typedef traversal traversal_category;
	
	// Incidence Graph
	typedef QMultiMap<Node *,Arrow *>::iterator out_edge_iterator;
	typedef unsigned int degree_size_type;

	// VertexListGraph
	typedef QList<Node *>::iterator vertex_iterator;
	typedef unsigned int vertices_size_type;

	// EdgeListGraph
	typedef QList<Arrow *>::iterator edge_iterator;
	typedef unsigned int edges_size_type;

	Node *null_vertex(void) { return 0; }
};
};

// Incidence Graph
Node *source(Arrow *, Graph *);
Node *target(Arrow *, Graph *);
std::pair<QMultiMap<Node *,Arrow *>::iterator,QMultiMap<Node *,Arrow *>::iterator> out_edges(Node *, Graph *);
unsigned int out_degree(Node *, Graph *);

// VertexListGraph
std::pair<QList<Node *>::iterator,QList<Node *>::iterator> vertices(Graph *);
unsigned int num_vertices(Graph *);

// EdgeListGraph
std::pair<QList<Arrow *>::iterator,QList<Arrow *>::iterator> edges(Graph *);
unsigned int num_edges(Graph *);

#endif
