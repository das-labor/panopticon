#ifndef PHASE3_HH
#define PHASE3_HH

#include <iterator>
#include <memory>

#include "dot/dot.hh"
#include "dot/traits.hh"
#include "dot/net_flow.hh"

template<typename T>
dot::net_flow<dot::graph_adaptor<T>> dot::cook_phase3(T graph, const net_flow<T> &ph1, const phase2<T> &ph2, unsigned int nodesep)
{
	typedef node_adaptor<T> node;
	std::unordered_map<std::pair<node,node>,unsigned int> omega, delta;
	graph_adaptor<T> adaptor;

	adaptor.edges = std::shared_ptr<std::unordered_multimap<node,node>>(new std::unordered_multimap<node,node>());
	adaptor.nodes = std::shared_ptr<std::unordered_set<node>>(new std::unordered_set<node>());

	// u -> v
	for(const std::pair<node,node> &edge: ph2.edges_by_tail)
	{
		int Omega = 0;

		adaptor.nodes->insert(edge.first);
		adaptor.nodes->insert(edge.second);

		switch(!!edge.first.is_virtual() + !!edge.second.is_virtual())
		{
			case 0: Omega = 1; break;
			case 1: Omega = 2; break;
			case 2: Omega = 8; break;
			default: ensure(false);
		}

		node tmp = node_adaptor<T>(virtual_node());
		unsigned int o = ph2.weights.at(edge) * Omega;

		adaptor.nodes->insert(tmp);

		// tmp -> u
		adaptor.edges->insert(std::make_pair(tmp,edge.first));
		omega.insert(std::make_pair(std::make_pair(tmp,edge.first),o));
		delta.insert(std::make_pair(std::make_pair(tmp,edge.first),0));

		// tmp -> v
		adaptor.edges->insert(std::make_pair(tmp,edge.second));
		omega.insert(std::make_pair(std::make_pair(tmp,edge.second),o));
		delta.insert(std::make_pair(std::make_pair(tmp,edge.second),0));
	}

	for(const std::pair<int,std::list<node>> &order: ph2.order)
	{
		std::list<node> real;

		std::copy_if(order.second.begin(),order.second.end(),std::inserter(real,real.begin()),
				[&](node n) { return !n.is_virtual(); });
		auto i = next(real.begin());
		while(i != real.end())
		{
			node left = *prev(i), right = *i;
			int sep = nodesep + dimensions(left.node(),graph).first;

			adaptor.edges->insert(std::make_pair(left,right));
			omega.insert(std::make_pair(std::make_pair(left,right),0));
			delta.insert(std::make_pair(std::make_pair(left,right),sep));

			++i;
		}
	}

	ensure(delta.size() == adaptor.edges->size());
	ensure(omega.size() == adaptor.edges->size());

	return preprocess<graph_adaptor<T>>(adaptor,omega,delta);
}
#endif
