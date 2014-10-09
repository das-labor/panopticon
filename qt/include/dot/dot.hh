#include <numeric>

#include "dot/rank.hh"
#include "dot/order.hh"

#pragma once

namespace dot
{
	/// min-level, max-level
	template<typename N,typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>> layout(const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;

		if(num_vertices(g) == 0)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>>();
		else if(num_vertices(g) == 1)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>>({std::make_pair(*vertices(g).first,std::make_pair(0,0))});

		std::unordered_map<vx_desc,std::pair<int,int>> ranks = rank(g);
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>> ret;

		for(auto vx: iters(vertices(g)))
			ret.emplace(vx,std::make_pair(ranks.at(vx).first,ranks.at(vx).second));

		return ret;
	}
}
