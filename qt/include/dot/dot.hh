#include "dot/rank.hh"
#include "dot/order.hh"

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

/*	/// convert g to DAG w/ two nodes per g-node and a single source and sink
	template<typename N,typename E>
	digraph<optional<N>,E> make_acyclic(const digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<N>;
		using virt_graph = typename po::digraph<virt_vx,E>;
		using color_pm_type = boost::associative_property_map<std::unordered_map<vx_desc,boost::default_color_type>>;

		virt_graph ret;
		std::unordered_map<vx_desc,boost::default_color_type> color;
		std::list<vx_desc> sources, sinks;

		for(auto vx: iters(vertices(g)))
		{
			int o = out_degree(vx,g);
			int i = in_degree(vx,g);

			if(o == 0 && i > 0)
				sinks.push_back(vx);
			else if(o > 0 && i == 0)
				sources.push_back(vx);
		}

		if(sources.empty())
			sources.push_back(*vertices(g).first);

		for(auto r: sources)
			boost::depth_first_search(g,visitor,color_pm_type(color),r);

		typename virt_graph::vertex_descriptor source, sink;

		// ensure single source node in ret
		if(sources.size() == 1)
		{
			auto p = vertices(ret);
			source = root(ret);
		}
		else
		{
			source = insert_vertex(virt_vx(boost::none),h);
			for(auto v: sources)
			{
				auto p = vertices(h);
				auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
					{ auto w = get_vertex(_w,h); return w && w->first && w->second == v; });
				ensure(s != p.second);

				omega.insert(std::make_pair(insert_edge(0,source,*s,h),dummy_edge_omega));
			}
		}

		// ensure single sink node in h
		if(sinks.size() == 0)
		{
			sink = *(vertices(h).first + 1);
		}
		else if(sinks.size() == 1)
		{
			auto p = vertices(h);
			auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
				{ auto w = get_vertex(_w,h); return w && !w->first && w->second == sinks.front(); });
			ensure(s != p.second);

			sink = *s;
		}
		else
		{
			sink = insert_vertex(virt_vx(boost::none),h);
			for(auto v: sinks)
			{
				auto p = vertices(h);
				auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
					{ auto w = get_vertex(_w,h); return w && !w->first && w->second == v; });
				ensure(s != p.second);

				omega.insert(std::make_pair(insert_edge(0,*s,sink,h),dummy_edge_omega));
			}
		}

		// layer assign
		net_flow<virt_vx,int> layer_nf(h,omega);
		layer_nf.solve(std::function<void(void)>([](void) {}));

/	std::cerr << "digraph G {" << std::endl;
			for(auto e: iters(edges(h)))
				std::cerr << po::source(e,h).id << " -> " << target(e,h).id << std::endl;
			for(auto v: iters(vertices(h)))
				std::cerr << v.id << " [label=\"" << layer_nf.lambda[v] << "\"]" << std::endl;
			std::cerr << "}" << std::endl;/

		// insert virtual nodes
		bool done = false;
		while(!done)
		{
			done = true;

			for(auto edge: iters(edges(h)))
			{
				auto from = po::source(edge,h), to = target(edge,h);
				int lf = layer_nf.lambda.at(from), lt = layer_nf.lambda.at(to);

				ensure(lf >= 0 && lt >= 0 && lt - lf >= 0);
				if(lt - lf > 1)
				{
					remove_edge(edge,h);
					done = false;

					int r = lf + 1;
					typename virt_graph::vertex_descriptor prev = from;

					while(r != lt)
					{
						auto n = insert_vertex(virt_vx(boost::none),h);
						layer_nf.lambda.emplace(n,r);
						insert_edge(0,prev,n,h);
						prev = n;
						++r;
					}
					insert_edge(0,prev,to,h);
					break;
				}
			}
		}


		// ordering
		int iter = 0;
		int cross = -1;
		std::unordered_map<typename virt_graph::vertex_descriptor,unsigned int> ordering = order(layer_nf.lambda,h);
/
		order(layer_nf);

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
		ph2 = best;/

		// map back to g

		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> ret;
		}*/
}
/*
#include <unordered_map>
#include <unordered_set>
#include <map>
#include <list>
#include <functional>
#include <algorithm>

#include <panopticon/digraph.hh>

#pragma once

//#include "dot/traits.hh"
//#include "dot/adaptor.hh"
//#include "dot/types.hh"

namespace dot
{
	template<typename N,typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> layout(const po::digraph<N,E>& g);

	template<typename N, typename E>
	struct net_flow
	{
		net_flow(const po::digraph<N,E>& g) : graph(g)
		{
			for(auto e: iters(edges(graph)))
			{
				omega.insert(std::make_pair(e,1));
				delta.insert(std::make_pair(e,1));
			}
		}

		// in
		const po::digraph<N,E>& graph;
		std::unordered_map<typename po::digraph<N,E>::edge_descriptor,unsigned int> omega;
		std::unordered_map<typename po::digraph<N,E>::edge_descriptor,unsigned int> delta;
		// TODO lim-low tree

		// out
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int> lambda;
		std::unordered_set<typename po::digraph<N,E>::edge_descriptor> tight_tree;
		std::unordered_map<typename po::digraph<N,E>::edge_descriptor,int> cut_values;
	};

	template<typename T>
	void dump(const net_flow<N,E> &nf);

	template<typename N,typename E>
	net_flow<N,E> preprocess(const po::digraph<N,E>& graph,
												 const std::unordered_map<typename po::digraph<N,E>::edge_descriptor,unsigned int> &omega,
												 const std::unordered_map<typename po::digraph<N,E>::edge_descriptor,unsigned int> &delta);

	template<typename T>
	void nf_solve(std::function<void(net_flow<N,E>&)> balance, net_flow<N,E> &nf);
	template<typename T>
	void feasible_tree(net_flow<N,E> &nf);
	template<typename T>
	std::pair<std::unordered_set<typename graph_traits<T>::node_type>,std::unordered_set<typename graph_traits<T>::node_type>> partition(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &c, const net_flow<N,E> &nf);
	template<typename T>
	void rank(net_flow<N,E> &nf, const std::unordered_set<typename graph_traits<T>::node_type> &todo);
	template<typename T>
	void cut_values(net_flow<N,E> &nf);
	template<typename T>
	void swap(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &cut, const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &replace, net_flow<N,E> &nf);
	template<typename T>
	void normalize(std::unordered_map<typename graph_traits<T>::node_type,int> &lambda);
	template<typename T>
	void balance(net_flow<N,E> &nf);
	template<typename T>
	int length(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e, const net_flow<N,E> &nf);
	template<typename T>
	int slack(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e, const net_flow<N,E> &nf);
	template<typename T>
	void symmetry(net_flow<N,E> &nf);

	template<typename T>
	net_flow<N,E> cook_phase1(T graph);

	template<typename T>
	struct phase2
	{
		// in
		std::unordered_set<node_adaptor<T>> nodes;
		std::unordered_multimap<node_adaptor<T>,node_adaptor<T>> edges_by_head;
		std::unordered_multimap<node_adaptor<T>,node_adaptor<T>> edges_by_tail;
		std::unordered_map<std::pair<node_adaptor<T>,node_adaptor<T>>,unsigned int> weights;
		std::multimap<int,node_adaptor<T>> rank_assignments;
		std::unordered_map<node_adaptor<T>,int> lambda;
		std::unordered_map<node_adaptor<T>,unsigned int> widths;

		// out
		std::unordered_map<node_adaptor<T>,int> x_coord;
		std::map<int,std::list<node_adaptor<T>>> order;
	};

	template<typename T>
	phase2<T> cook_phase2(T t, const net_flow<N,E> &ph1);
	template<typename T>
	void order(phase2<T> &ph2);
	template<typename T>
	std::unordered_map<node_adaptor<T>,double> weighted_median(phase2<T> &ph2, bool down);
	template<typename T>
	double median_value(node_adaptor<T> node, int adj_rank, const phase2<T> &ph2);
	template<typename T>
	unsigned int crossing(node_adaptor<T> a, node_adaptor<T> b, const phase2<T> &ph2);
	template<typename T>
	void swap(node_adaptor<T> a, node_adaptor<T> b, phase2<T> &ph2);
	template<typename T>
	unsigned int transpose(phase2<T> &ph2);

	// phase3
	template<typename T>
	net_flow<graph_adaptor<T>> cook_phase3(T t, const net_flow<N,E> &ph1, const phase2<T> &ph2, unsigned int node_sep);

	// A*
	template<typename T>
	void astar(T graph);
	template<typename T>
	std::list<vis_node<T>> route(typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vg, T tag);
	template<typename T>
	void expand(vis_node<T> cur,std::unordered_map<vis_node<T>,unsigned int> &path_cost, std::unordered_map<vis_node<T>,vis_node<T>> &path_ptr, std::map<unsigned int,vis_node<T>> &openlist, const std::unordered_set<vis_node<T>> &closedlist, typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vg, T tag);

	template<typename T>
	std::unordered_set<vis_node<T>> successors(vis_node<T> cur, typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vis_graph, T graph);
	template<typename T>
	unsigned int heuristic(vis_node<T> cur, typename graph_traits<T>::edge_type e, const std::unordered_multimap<vis_node<T>,vis_node<T>> &vis_graph, T graph);
	template<typename T>
	unsigned int edge_cost(vis_node<T> from, vis_node<T> to, typename graph_traits<T>::edge_type e, T graph);
	//template<typename T>
	//std::list<std::pair<unsigned int, unsigned int>> polylines(const std::list<std::pair<unsigned int, unsigned int>> &coords, typename graph_traits<T>::edge_type e, const std::unordered_multimap<dot::coord,dot::coord> &vis_graph, T graph);
	template<typename T>
	std::unordered_multimap<vis_node<T>,vis_node<T>> visibility_graph(T graph);
}

#include "layout.hh"
#include "net_flow.hh"
#include "phase1.hh"
#include "phase2.hh"
#include "phase3.hh"
#include "astar.hh"*/
