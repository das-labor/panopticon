/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
