#include <unordered_map>

#include <boost/graph/depth_first_search.hpp>

#include <panopticon/digraph.hh>

#include "dot/net_flow.hh"

#pragma once

namespace dot
{
	const int dummy_edge_omega = 1;
	const int graph_edge_omega = 10;
	const int dummy_edge_delta = 0;
	const int graph_edge_delta = 1;

	template<typename N,typename E>
	struct rank_dag_visitor
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<std::pair<bool,vx_desc>>; // true <=> upper node
		using virt_graph = typename po::digraph<virt_vx,std::pair<int,int>>; // omega,delta

		rank_dag_visitor(void) {}
		rank_dag_visitor(virt_graph* h) : _h(h) {}
		rank_dag_visitor(const rank_dag_visitor& v) : _h(v._h) {}

		rank_dag_visitor& operator=(const rank_dag_visitor& v) { _h = v._h; return *this; }

		void initialize_vertex(vx_desc vx,const po::digraph<N,E>&)
		{
			auto vxs = vertices(*this->_h);
			if(std::none_of(vxs.first,vxs.second,[&](typename virt_graph::vertex_descriptor _w) { auto w = get_vertex(_w,*this->_h); return w && w->second == vx; }))
			{
				auto a = insert_vertex(boost::make_optional(std::make_pair(true,vx)),*this->_h);
				auto b = insert_vertex(boost::make_optional(std::make_pair(false,vx)),*this->_h);
				insert_edge(std::make_pair(dummy_edge_omega,dummy_edge_delta),a,b,*this->_h);
			}
		}

		void start_vertex(vx_desc,const po::digraph<N,E>&) {}
		void discover_vertex(vx_desc vx,const po::digraph<N,E>&) {}

		void finish_vertex(vx_desc,const po::digraph<N,E>&) {}
		void examine_edge(eg_desc,const po::digraph<N,E>&) {}
		void tree_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			forward_or_cross_edge(e,g);
		}

		void back_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			auto p = vertices(*this->_h);
			auto ai = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == source(e,g) && !v->first; });
			auto bi = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == target(e,g) && v->first; });

			ensure(ai != p.second && bi != p.second);

			auto eds = edges(*this->_h);
			if(std::none_of(eds.first,eds.second,[&](typename virt_graph::edge_descriptor _f) { return source(_f,*this->_h) == *bi && target(_f,*this->_h) == *ai; }))
				insert_edge(std::make_pair(graph_edge_omega,graph_edge_delta),*bi,*ai,*this->_h);
		}

		void forward_or_cross_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			auto p = vertices(*this->_h);
			auto ai = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == source(e,g) && !v->first; });
			auto bi = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == target(e,g) && v->first; });

			ensure(ai != p.second && bi != p.second);

			auto eds = edges(*this->_h);
			if(std::none_of(eds.first,eds.second,[&](typename virt_graph::edge_descriptor _f) { return source(_f,*this->_h) == *ai && target(_f,*this->_h) == *bi; }))
				insert_edge(std::make_pair(graph_edge_omega,graph_edge_delta),*ai,*bi,*this->_h);
		}

		void finish_edge(eg_desc,const po::digraph<N,E>&) {}

		virt_graph *_h;
	};

	/// convert g to DAG w/ two nodes per g-node and a single source and sink
	template<typename N,typename E>
	po::digraph<boost::optional<std::pair<bool,typename po::digraph<N,E>::vertex_descriptor>>,std::pair<int,int>> prepare_rank_graph(const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<std::pair<bool,vx_desc>>; // true <=> upper node
		using virt_graph = typename po::digraph<virt_vx,std::pair<int,int>>; // omega,delta
		using color_pm_type = boost::associative_property_map<std::unordered_map<vx_desc,boost::default_color_type>>;

		virt_graph h;
		std::unordered_map<vx_desc,boost::default_color_type> color;
		rank_dag_visitor<N,E> visitor(&h);

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

		// ensure single source node in h
		if(sources.size() == 1)
		{
			auto p = vertices(h);
			auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
				{ auto w = get_vertex(_w,h); return w && w->first && w->second == sources.front(); });
			ensure(s != p.second);

			source = *s;
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

				insert_edge(std::make_pair(dummy_edge_omega,dummy_edge_delta),source,*s,h);
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

				insert_edge(std::make_pair(dummy_edge_omega,dummy_edge_delta),*s,sink,h);
			}
		}

		return h;
	}

	template<typename N, typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>> rank(const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<std::pair<bool,vx_desc>>; // true <=> upper node
		using virt_graph = typename po::digraph<virt_vx,std::pair<int,int>>; // omega,delta

		virt_graph h = prepare_rank_graph(g);

		// layer assign
		net_flow<virt_vx> layer_nf(h);
		layer_nf.solve(std::function<void(void)>([](void) {}));
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>> ret;

		// map back to g
		for(auto vx: iters(vertices(g)))
		{
			auto p = vertices(h);
			auto f = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _wx)
					{ auto wx = get_vertex(_wx,h); return wx && wx->first && wx->second == vx; });
			auto l = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _wx)
					{ auto wx = get_vertex(_wx,h); return wx && !wx->first && wx->second == vx; });

			ensure(l != p.second && f != p.second);
			ret.emplace(vx,std::make_pair(layer_nf.lambda.at(*f),layer_nf.lambda.at(*l)));
		}

		return ret;
	}
}
/*po::digraph<
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
}*/
