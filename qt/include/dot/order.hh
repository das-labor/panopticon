#include <list>
#include <panopticon/digraph.hh>

#pragma once

namespace dot
{
	template<typename N,typename E>
	struct order_dag_visitor
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename std::tuple<int,int,boost::optional<vx_desc>>;
		using virt_graph = typename po::digraph<virt_vx,int>;

		order_dag_visitor(void) {}
		order_dag_visitor(virt_graph* h, std::unordered_map<vx_desc,std::pair<int,int>> const* ranks) : _h(h), _ranks(ranks) {}
		order_dag_visitor(const order_dag_visitor& v) : _h(v._h), _ranks(v._ranks) {}

		order_dag_visitor& operator=(const order_dag_visitor& v) { _h = v._h; _ranks = v._ranks; return *this; }

		void initialize_vertex(vx_desc vx,const po::digraph<N,E>&)
		{
			auto vxs = vertices(*this->_h);
			if(std::none_of(vxs.first,vxs.second,[&](typename virt_graph::vertex_descriptor _w) { auto w = get_vertex(_w,*this->_h); return std::get<2>(w) && *std::get<2>(w) == vx; }))
			{
				insert_vertex(virt_vx(_ranks->at(vx).first,_ranks->at(vx).second,vx),*this->_h);
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
				{ auto v = get_vertex(_v,*this->_h); return std::get<2>(v) && *std::get<2>(v) == source(e,g); });
			auto bi = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return std::get<2>(v) && *std::get<2>(v) == target(e,g); });

			ensure(ai != p.second && bi != p.second);

			auto eds = edges(*this->_h);
			if(std::none_of(eds.first,eds.second,[&](typename virt_graph::edge_descriptor _f) { return source(_f,*this->_h) == *bi && target(_f,*this->_h) == *ai; }))
				insert_edge(0,*bi,*ai,*this->_h);
		}

		void forward_or_cross_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			auto p = vertices(*this->_h);
			auto ai = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return std::get<2>(v) && *std::get<2>(v) == source(e,g); });
			auto bi = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return std::get<2>(v) && *std::get<2>(v) == target(e,g); });

			ensure(ai != p.second && bi != p.second);

			auto eds = edges(*this->_h);
			if(std::none_of(eds.first,eds.second,[&](typename virt_graph::edge_descriptor _f) { return source(_f,*this->_h) == *ai && target(_f,*this->_h) == *bi; }))
				insert_edge(0,*ai,*bi,*this->_h);
		}

		void finish_edge(eg_desc,const po::digraph<N,E>&) {}

		virt_graph *_h;
		std::unordered_map<vx_desc,std::pair<int,int>> const* _ranks;
	};

	/// convert g to DAG w/ two nodes per g-node and a single source and sink
	template<typename N,typename E>
	po::digraph<std::tuple<int,int,boost::optional<typename po::digraph<N,E>::vertex_descriptor>>,int>
	prepare_order_graph(const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>>& ranks, const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename std::tuple<int,int,boost::optional<vx_desc>>;
		using virt_graph = typename po::digraph<virt_vx,int>;
		using color_pm_type = boost::associative_property_map<std::unordered_map<vx_desc,boost::default_color_type>>;

		virt_graph h;
		std::unordered_map<vx_desc,boost::default_color_type> color;
		order_dag_visitor<N,E> visitor(&h,&ranks);

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
				{ auto w = get_vertex(_w,h); return std::get<2>(w) && *std::get<2>(w) == sources.front(); });
			ensure(s != p.second);

			source = *s;
		}
		else
		{
			int r = std::accumulate(sources.begin(),sources.end(),ranks.at(sources.front()).second,[&](int acc, vx_desc v)
					{ return std::min(acc,ranks.at(v).second); });
			source = insert_vertex(virt_vx(r,r,boost::none),h);
			for(auto v: sources)
			{
				auto p = vertices(h);
				auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
					{ auto w = get_vertex(_w,h); return std::get<2>(w) && *std::get<2>(w) == v; });
				ensure(s != p.second);

				insert_edge(0,source,*s,h);
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
				{ auto w = get_vertex(_w,h); return std::get<2>(w) && *std::get<2>(w) == sinks.front(); });
			ensure(s != p.second);

			sink = *s;
		}
		else
		{
			int r = std::accumulate(sinks.begin(),sinks.end(),ranks.at(sinks.front()).second,[&](int acc, vx_desc v)
					{ return std::max(acc,ranks.at(v).second); });
			sink = insert_vertex(virt_vx(r,r,boost::none),h);

			for(auto v: sinks)
			{
				auto p = vertices(h);
				auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
					{ auto w = get_vertex(_w,h); return std::get<2>(w) && *std::get<2>(w) == v; });
				ensure(s != p.second);

				insert_edge(0,*s,sink,h);
			}
		}

		// insert virtual nodes
		bool done = false;
		while(!done)
		{
			done = true;

			for(auto edge: iters(edges(h)))
			{
				auto from = po::source(edge,h), to = target(edge,h);
				int lf = std::get<0>(get_vertex(from,h)), lt = std::get<0>(get_vertex(to,h));

				ensure(lf >= 0 && lt >= 0 && lt - lf >= 0);
				if(lt - lf > 1)
				{
					remove_edge(edge,h);
					done = false;

					int r = lf + 1;
					typename virt_graph::vertex_descriptor prev = from;

					while(r != lt)
					{
						auto n = insert_vertex(virt_vx(r,r,boost::none),h);
						insert_edge(0,prev,n,h);
						prev = n;
						++r;
					}
					insert_edge(0,prev,to,h);
					break;
				}
			}
		}

		return h;
	}

	template<typename N,typename E>
	std::unordered_map<typename po::digraph<std::tuple<int,int,boost::optional<typename po::digraph<N,E>::vertex_descriptor>>,int>::vertex_descriptor,int>
	initial_order(const po::digraph<std::tuple<int,int,boost::optional<typename po::digraph<N,E>::vertex_descriptor>>,int>& graph)
	{
		using node = typename po::digraph<std::tuple<int,int,boost::optional<typename po::digraph<N,E>::vertex_descriptor>>,int>::vertex_descriptor;
		std::unordered_map<node,int> ret;
		std::unordered_multiset<int> rev;
		std::function<void(node)> dfs;

		dfs = [&](node n)
		{
			int rank = get<0>(get_vertex(n,graph));

			ensure(ret.insert(std::make_pair(n,rev.count(rank))).second);
			rev.insert(rank);

			for(auto e: iters(out_edges(n,graph)))
			{
				node m = target(e,graph);
				if(!ret.count(m))
					dfs(m);
			}
		};

		dfs(root(graph));
		ensure(ret.size() == num_vertices(graph));

		return ret;
	}

	template<typename N,typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>
	order(std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>>& lambda, const po::digraph<N,E>& graph)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename std::tuple<int,int,boost::optional<vx_desc>>;
		using virt_graph = typename po::digraph<virt_vx,int>;

		virt_graph h = prepare_order_graph(lambda,graph);
		std::unordered_map<typename virt_graph::vertex_descriptor,int> cur = initial_order<N,E>(h), best;

		/*for(auto vx: iters(vertices(graph)))
			cur.emplace(vx,0);*/

		/*std::cerr << "digraph G {" << std::endl;
			for(auto e: iters(edges(h)))
				std::cerr << po::source(e,h).id << " -> " << target(e,h).id << std::endl;
			for(auto v: iters(vertices(h)))
				std::cerr << v.id << " [label=\"" << layer_nf.lambda[v] << "\"]" << std::endl;
			std::cerr << "}" << std::endl;*/

		// ordering
		int iter = 0;
		int cross = -1;
		best = cur;

		/*while(iter < 24)
		{
			std::unordered_map<node_adaptor<T>,double> median = weighted_median(ph2,iter & 1);
			unsigned int tmp = transpose(ph2);

			if(cross < 0 || static_cast<unsigned int>(cross) > tmp)
			{
				cross = tmp;
				best = ph2;
			}

			++iter;
		}*/

		// map back to g
		std::unordered_map<vx_desc,int> ret;

		for(auto x: best)
		{
			auto v = get_vertex(x.first,h);
			if(std::get<2>(v))
				ret.emplace(*std::get<2>(v),x.second);
		}

		ensure(ret.size() == num_vertices(graph));
		return ret;
	}
}
