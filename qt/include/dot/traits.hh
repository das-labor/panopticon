#ifndef TRAITS_HH
#define TRAITS_HH

#include <unordered_map>
#include <unordered_set>
#include <list>
#include <cstdint>

namespace dot
{
	template<typename T>
	struct graph_traits
	{
		typedef void node_type;
		typedef void edge_type;

		typedef void node_iterator;
		typedef void edge_iterator;
		typedef void out_edge_iterator;
	};
}

#include "types.hh"

namespace dot
{
	// graph attributes
	template<typename T>
	std::pair<typename graph_traits<T>::node_iterator,typename graph_traits<T>::node_iterator> nodes(T t);
	template<typename T>
	std::pair<typename graph_traits<T>::edge_iterator,typename graph_traits<T>::edge_iterator> edges(T t);
	template<typename T>
	bool has_entry(T t);
	template<typename T>
	typename graph_traits<T>::node_type entry(T t);

	// node attributes
	template<typename T>
	std::pair<typename graph_traits<T>::out_edge_iterator,typename graph_traits<T>::out_edge_iterator> out_edges(typename graph_traits<T>::node_type n, T t);
	template<typename T>
	std::pair<typename graph_traits<T>::edge_iterator,typename graph_traits<T>::edge_iterator> in_edges(typename graph_traits<T>::node_type n, T t);
	template<typename T>
	std::pair<unsigned int,unsigned int> dimensions(typename graph_traits<T>::node_type n, T t);
	template<typename T>
	coord position(typename graph_traits<T>::node_type n, T t);

	// edge attributes
	template<typename T>
	typename graph_traits<T>::node_type source(typename graph_traits<T>::edge_type e, T t);
	template<typename T>
	typename graph_traits<T>::node_type sink(typename graph_traits<T>::edge_type e, T t);
	template<typename T>
	unsigned int weight(typename graph_traits<T>::edge_type e, T t);

	// grid queries (A*)
	/*template<typename T>
	bool is_free(float x, float y, unsigned int w, unsigned int h, typename graph_traits<T>::edge_type e, T graph);*/
	template<typename T>
	bool is_free(const vis_node<T> &a, const vis_node<T> &b, T graph);

	// node position (dot)
	template<typename T>
	void set_position(typename graph_traits<T>::node_type n, const coord &pos, T t);

	// edge segments (A*)
	template<typename T>
	void set_segments(typename graph_traits<T>::edge_type e, const std::list<coord> &segs, T t);
}

namespace std
{
	template<typename T>
	struct hash<pair<T,T>>
	{
		size_t operator()(const pair<T,T> &p) const
		{
			hash<T> h;
			return h(p.first) ^ h(p.second);
		}
	};
}

#endif
