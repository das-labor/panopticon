#include <limits>

#include "dot/dot.hh"

#pragma once

template<typename T>
void dot::layout(T graph, unsigned int ranksep, unsigned int nodesep)
{
	typedef dot::graph_traits<T> traits;

	/*
	static_ensure(std::is_copy_constructible<T>::value,"The graph type needs to be copy-constructible");

	static_ensure(std::is_copy_constructible<traits::node_type>::value,"The node type needs to be copy-constructible");
	static_ensure(std::is_copy_assignable<traits::node_type>::value,"The node type needs to be copy-assignable");
	static_ensure(std::is_default_constructible<traits::node_type>::value,"The node type needs a default constructor");

	static_ensure(std::is_copy_constructible<traits::edge_type>::value,"The edge type needs to be copy-constructible");
	static_ensure(std::is_copy_assignable<traits::edge_type>::value,"The edge type needs to be copy-assignable");
	static_ensure(std::is_default_constructible<traits::edge_type>::value,"The edge type needs a default constructor");
	*/

	auto nd = nodes(graph);
	if(nd.first == nd.second)
		return;

	// rank
	net_flow<T> ph1 = cook_phase1(graph);
	nf_solve<T>(balance<T>,ph1);

	// ordering
	int iter = 0;
	int cross = -1;
	phase2<T> best, ph2 = cook_phase2(graph,ph1);

	order(ph2);

	best = ph2;
	while(iter < 24)
	{
		std::unordered_map<node_adaptor<T>,double> median = weighted_median(ph2,iter & 1);
		unsigned int tmp = transpose(ph2);

		if(cross < 0 || static_cast<unsigned int>(cross) > tmp)
		{
			cross = tmp;
			best = ph2;
		}

		++iter;
	}
	ph2 = best;

	// x coordinate
	net_flow<graph_adaptor<T>> ph3 = cook_phase3(graph,ph1,ph2,nodesep);
	nf_solve<graph_adaptor<T>>(symmetry<graph_adaptor<T>>,ph3);

	int x_correction = std::numeric_limits<int>::max();
	std::map<unsigned int,unsigned int> maxh;
	typename traits::node_iterator i,iend;

	std::tie(i,iend) = nodes(graph);

	std::for_each(i,iend,[&](const typename traits::node_type &n)
	{
		int r = ph1.lambda.at(n);

		x_correction = std::min(x_correction,ph3.lambda.at(n));
		if(maxh.count(r))
			maxh[r] = std::max(maxh.at(r),dimensions(n,graph).second);
		else
			maxh.insert(std::make_pair(r,dimensions(n,graph).second));
	});

	// position nodes
	int t = 0;
	for(std::pair<unsigned int const,unsigned int> &x: maxh)
		t = x.second += t + ranksep;

	std::unordered_map<typename traits::node_type,std::pair<int,int>> pos;
	for(typename graph_traits<graph_adaptor<T>>::node_type m: ph2.nodes)
		if(m.is_node())
		{
			typename traits::node_type n = m.node();
			set_position(n,std::make_pair(ph3.lambda.at(n) - x_correction,maxh.at(ph1.lambda.at(n))),graph);
		}
}
