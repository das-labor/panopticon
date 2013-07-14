#ifndef TYPES_HH
#define TYPES_HH

#include "dot/adaptor.hh"

namespace dot
{
	typedef std::pair<float,float> coord;

	template<typename T>
	struct vis_node
	{
		vis_node(const coord &c, const node_adaptor<T> n = node_adaptor<T>()) : position(c), node(n) {}
		bool operator==(const vis_node<T> &a) const { return a.position == position && a.node == node; }

		coord position;
		node_adaptor<T> node;
	};

	//template<typename T>
	//using vis_graph = std::unordered_multimap<vis_node<T>,vis_node<T>>;
}

namespace std
{
	template<>
	struct hash<dot::coord>
	{
		size_t operator()(const dot::coord &p) const
		{
			hash<float> h;
			return h(p.first) ^ h(p.second);
		}
	};

	template<typename T>
	struct hash<dot::vis_node<T>>
	{
		size_t operator()(const dot::vis_node<T> &p) const
		{
			hash<dot::node_adaptor<T>> h1;
			hash<dot::coord> h2;
			return h2(p.position) ^ h1(p.node);
		}
	};
}

#endif
