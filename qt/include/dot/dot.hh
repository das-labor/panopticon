#include <numeric>

#include "dot/rank.hh"
#include "dot/order.hh"
#include "dot/place.hh"

#pragma once

namespace dot
{
	/// min-level, max-level, x order
	template<typename N,typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> layout(const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;

		if(num_vertices(g) == 0)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>();
		else if(num_vertices(g) == 1)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>({std::make_pair(*vertices(g).first,std::make_tuple(0,0,0))});

		std::unordered_map<vx_desc,std::pair<int,int>> ranks = rank(g);
		std::unordered_map<vx_desc,int> ordering = order(ranks,g);
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> ret;

		for(auto vx: iters(vertices(g)))
			ret.emplace(vx,std::make_tuple(ranks.at(vx).first,ranks.at(vx).second,ordering.at(vx)));

		return ret;
	}

	/// x pos
	template<typename N,typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>
	place(const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>& layout, const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>& widths, int nodesep, const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;

		if(num_vertices(g) == 0)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>();
		else if(num_vertices(g) == 1)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>({std::make_pair(*vertices(g).first,0)});

		// solve x-coord
		po::digraph<boost::optional<vx_desc>,std::pair<int,int>> aux = prepare_place_graph(layout,widths,nodesep,g);

		net_flow<boost::optional<vx_desc>> layer_nf(aux);
		layer_nf.solve(std::function<void(void)>([](void) {}));
		layer_nf.make_symmetric();

		// move the nodes so that all x coordinates are >= 0
		int x_correction = std::accumulate(layer_nf.lambda.begin(),layer_nf.lambda.end(),std::numeric_limits<int>::max(),[](int a, std::pair<typename decltype(aux)::vertex_descriptor,int> b)
				{ return std::min<int>(a,b.second); });

		// map back to graph
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int> ret;
		for(auto _v: iters(vertices(aux)))
		{
			auto v = get_vertex(_v,aux);
			if(v)
				ret.emplace(*v,layer_nf.lambda.at(_v) - x_correction);
		}

		/*int x_correction = std::numeric_limits<int>::max();
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
		}*/

		return ret;
	}
}
