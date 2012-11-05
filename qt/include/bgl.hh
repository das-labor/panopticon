#ifndef BGL_HH
#define BGL_HH

#include <boost/graph/graph_traits.hpp>
#include <boost/property_map/property_map.hpp>
#include <graph.hh>

template<typename K, typename V>
struct PropertyMap
{
	PropertyMap(std::function<V(const K&)> g, std::function<void(const K&, const V&)> p) : get(g), put(p) {};
	
	const std::function<V(const K&)> get;
	const std::function<void(const K&, const V&)> put;
};

namespace boost {
template<>
struct graph_traits<Scene *>
{
	struct traversal : public incidence_graph_tag,
														vertex_list_graph_tag,
														edge_list_graph_tag {};

	typedef Node * vertex_descriptor;
	typedef Arrow * edge_descriptor;
	typedef directed_tag directed_category;
	typedef allow_parallel_edge_tag edge_parallel_category;
	typedef traversal traversal_category;
	
	// Incidence Scene
	typedef QMultiMap<Node *,Arrow *>::iterator out_edge_iterator;
	typedef unsigned int degree_size_type;

	// VertexListScene
	typedef QList<Node *>::iterator vertex_iterator;
	typedef unsigned int vertices_size_type;

	// EdgeListScene
	typedef QList<Arrow *>::iterator edge_iterator;
	typedef unsigned int edges_size_type;

	Node *null_vertex(void) { return 0; }

};

template<typename K, typename V>
struct property_traits<PropertyMap<K,V> *>
{
	struct cat : public readable_property_map_tag,
											writable_property_map_tag {};

	typedef V value_type;
	typedef V& reference_type;
	typedef K key_type;
	typedef cat category;
};
};

// Incidence Scene
Node *source(Arrow *, Scene *);
Node *target(Arrow *, Scene *);
std::pair<QMultiMap<Node *,Arrow *>::iterator,QMultiMap<Node *,Arrow *>::iterator> out_edges(Node *, Scene *);
unsigned int out_degree(Node *, Scene *);

// VertexListScene
std::pair<QList<Node *>::iterator,QList<Node *>::iterator> vertices(Scene *);
unsigned int num_vertices(Scene *);

// EdgeListScene
std::pair<QList<Arrow *>::iterator,QList<Arrow *>::iterator> edges(Scene *);
unsigned int num_edges(Scene *);

// ReadablePropertyMap
template<typename K, typename V>
V get(const PropertyMap<K,V> *pmap, const K &key)
{
	assert(pmap);
	return pmap->get(key);
}

// WriteablePropetyMap
template<typename K, typename V>
void put(PropertyMap<K,V> *pmap, const K &key, const V &val)
{
	assert(pmap);
	pmap->put(key,val);
}

template<typename K, typename V>
void set(PropertyMap<K,V> *pmap, K &key, const V &val)
{
	put(pmap,key,val);
}

#endif
