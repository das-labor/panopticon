#ifndef ASTAR_HH
#define ASTAR_HH

#include <unordered_map>
#include <unordered_set>
#include <list>
#include <functional>
#include <algorithm>

#include "dot/dot.hh"
#include "dot/traits.hh"
#include "dot/adaptor.hh"

template<typename T>
std::unordered_set<dot::vis_node<T>> dot::successors(dot::vis_node<T> cur, typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vg, T graph)
{
	auto p = vg.equal_range(cur);
	std::unordered_set<vis_node<T>> ret;

	if(cur.node.is_node() && cur.node.node() != source(e,graph))
		return ret;

	std::for_each(p.first,p.second,[&](const std::pair<dot::vis_node<T>,dot::vis_node<T>> &x)
	{
		if(!x.second.node.is_node() || x.second.node.node() == sink(e,graph))
			ret.insert(x.second);
	});

	return ret;
}

template<typename T>
unsigned int dot::heuristic(dot::vis_node<T> cur, typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vg, T graph)
{
	typename graph_traits<T>::node_type to = sink(e,graph);
	coord pos = position(to,graph);
	std::pair<unsigned int,unsigned int> sz = dimensions(to,graph);

	float xdelta = std::fabs(cur.position.first - (pos.first + sz.first / 2));
	float ydelta = std::fabs(cur.position.second - (pos.second + sz.second / 2));

	return std::sqrt(xdelta * xdelta + ydelta * ydelta);
}

template<typename T>
unsigned int dot::edge_cost(dot::vis_node<T> a, dot::vis_node<T> b, typename dot::graph_traits<T>::edge_type e,T graph)
{
	float xdelta = std::fabs(b.position.first - a.position.first);
	float ydelta = std::fabs(b.position.second - a.position.second);

	return std::sqrt(xdelta * xdelta + ydelta * ydelta);
}

template<typename T>
std::unordered_multimap<dot::vis_node<T>,dot::vis_node<T>> dot::visibility_graph(T graph)
{
	typedef typename graph_traits<T>::node_type node;
	std::unordered_set<vis_node<T>> visnodes;
	std::unordered_multimap<vis_node<T>,vis_node<T>> ret;

	auto p = nodes(graph);

	// collect nodes
	std::for_each(p.first,p.second,[&](const node &n)
	{
		auto pos = position(n,graph);
		auto sz = dimensions(n,graph);
		const int delta = 3;

		visnodes.insert(vis_node<T>(std::make_pair(pos.first + sz.first / 2,pos.second + sz.second / 2),n));
		visnodes.insert(vis_node<T>(std::make_pair(pos.first - delta,pos.second - delta)));
		visnodes.insert(vis_node<T>(std::make_pair(pos.first - delta,pos.second + sz.second + delta)));
		visnodes.insert(vis_node<T>(std::make_pair(pos.first + sz.first + delta,pos.second - delta)));
		visnodes.insert(vis_node<T>(std::make_pair(pos.first + sz.first + delta,pos.second + sz.second + delta)));
	});

	// find edges
	for(const vis_node<T> &from: visnodes)
		for(const vis_node<T> &to: visnodes)
			if(is_free(from,to,graph))
				ret.insert(std::make_pair(from,to));

	return ret;
}

template<typename T>
	void dot::expand(dot::vis_node<T> cur,std::unordered_map<dot::vis_node<T>,unsigned int> &path_cost, std::unordered_map<vis_node<T>,vis_node<T>> &path_ptr, std::map<unsigned int,dot::vis_node<T>> &openlist, const std::unordered_set<dot::vis_node<T>> &closedlist, typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vg, T tag)
{
	for(vis_node<T> succ: successors(cur,e,vg,tag))
	{
		if(closedlist.count(succ))
			continue;

		unsigned int tg = path_cost.at(cur) + edge_cost(cur,succ,e,tag);
		auto i = find_if(openlist.begin(),openlist.end(),[&](const std::pair<unsigned int,vis_node<T>> &s)
			{ return s.second.position == succ.position; });

		if(i != openlist.end() && path_cost.count(succ) && path_cost.at(succ) <= tg)
			continue;

		path_ptr.erase(succ);
		path_cost.erase(succ);
		assert(path_ptr.insert(std::make_pair(succ,cur)).second);
		assert(path_cost.insert(std::make_pair(succ,tg)).second);

		unsigned int f = tg + heuristic(succ,e,vg,tag);

		if(i != openlist.end())
			openlist.erase(i);
		openlist.insert(std::make_pair(f,succ));
	}
}

template<typename T>
std::list<dot::vis_node<T>> dot::route(typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vg, T tag)
{
	std::unordered_map<vis_node<T>,unsigned int> path_cost;
	std::unordered_map<vis_node<T>,vis_node<T>> path_ptr;
	std::map<unsigned int,vis_node<T>> openlist;
	std::unordered_set<vis_node<T>> closedlist;
	std::list<vis_node<T>> ret;
	coord from_pos = position(source(e,tag),tag), to_pos = position(sink(e,tag),tag);
	std::pair<unsigned int,unsigned int> from_sz = dimensions(source(e,tag),tag), to_sz = dimensions(sink(e,tag),tag);
	vis_node<T> start(std::make_pair(from_pos.first + from_sz.first / 2,from_pos.second + from_sz.second / 2),source(e,tag));
	vis_node<T> finish(std::make_pair(to_pos.first + to_sz.first / 2,to_pos.second + to_sz.second / 2),sink(e,tag));

	assert(vg.count(start) && vg.count(finish));
	openlist.insert(std::make_pair(0,start));
	path_cost.insert(std::make_pair(start,0));

	do
	{
		vis_node<T> cur = openlist.begin()->second;

		openlist.erase(openlist.begin());

		if(cur.node == finish.node)
		{
			while(path_ptr.count(cur))
			{
				ret.push_front(cur.position);
				cur = path_ptr.at(cur);
			}
			ret.push_front(cur.position);

			return ret;
		}

		closedlist.insert(cur);
		expand(cur,path_cost,path_ptr,openlist,closedlist,e,vg,tag);
	}
	while(openlist.size());

	return ret;
}

template<typename T>
void dot::astar(T graph)
{
	auto p = edges(graph);
	auto vg = visibility_graph(graph);

	std::for_each(p.first,p.second,[&](const typename graph_traits<T>::edge_type &e)
	{
		std::list<dot::coord> segs;
		std::list<vis_node<T>> path = route(e,vg,graph);

		std::transform(path.begin(),path.end(),std::inserter(segs,segs.end()),[&](const vis_node<T> &vn)
			{ return vn.position; });

		set_segments(e,segs,graph);
	});
}

#endif
