#ifndef PHASE1_HH
#define PHASE1_HH

#include <algorithm>

#include "dot.hh"
#include "net_flow.hh"

template<typename T>
dot::net_flow<T> dot::cook_phase1(T graph)
{
	typedef typename graph_traits<T>::edge_type edge;

	net_flow<T> ret;
	std::unordered_map<typename graph_traits<T>::edge_type,unsigned int> omega;
	std::unordered_map<typename graph_traits<T>::edge_type,unsigned int> delta;
	auto p = edges(graph);

	std::for_each(p.first,p.second,[&](const edge &e)
	{
		omega.insert(std::make_pair(e,1));
		delta.insert(std::make_pair(e,1));
	});

	return preprocess(graph,omega,delta);
}

#endif
