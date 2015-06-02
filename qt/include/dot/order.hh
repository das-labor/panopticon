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

#include <list>

#include <panopticon/region.hh>
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
		order_dag_visitor(virt_graph* h, std::unordered_map<vx_desc,std::pair<int,int>> const* ranks)
		: _h(h), _ranks(ranks), _mapping(new std::unordered_map<vx_desc,typename virt_graph::vertex_descriptor>()) {}
		order_dag_visitor(const order_dag_visitor& v) : _h(v._h), _ranks(v._ranks), _mapping(v._mapping) {}

		order_dag_visitor& operator=(const order_dag_visitor& v)
			{ _h = v._h; _ranks = v._ranks; _mapping = v._mapping; return *this; }

		void initialize_vertex(vx_desc vx,const po::digraph<N,E>&)
		{
			if(!_mapping->count(vx))
				_mapping->emplace(vx,insert_vertex(virt_vx(_ranks->at(vx).first,_ranks->at(vx).second,vx),*this->_h));
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
			insert_edge(0,_mapping->at(po::target(e,g)),_mapping->at(po::source(e,g)),*this->_h);
		}

		void forward_or_cross_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			insert_edge(0,_mapping->at(po::source(e,g)),_mapping->at(po::target(e,g)),*this->_h);
		}

		void finish_edge(eg_desc,const po::digraph<N,E>&) {}

		virt_graph *_h;
		std::unordered_map<vx_desc,std::pair<int,int>> const* _ranks;
		std::shared_ptr<std::unordered_map<vx_desc,typename virt_graph::vertex_descriptor>> _mapping;
	};

	/// convert g to DAG w/ two nodes per g-node and a single source and sink
	template<typename N,typename E>
	po::digraph<std::tuple<int,int,boost::optional<typename po::digraph<N,E>::vertex_descriptor>>,int>
	prepare_order_graph(const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>>& ranks, const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using virt_vx = typename std::tuple<int,int,boost::optional<vx_desc>>;
		using virt_graph = typename po::digraph<virt_vx,int>;
		using virt_desc = typename po::digraph<virt_vx,int>::vertex_descriptor;
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

		virt_desc source, sink;

		// ensure single source node in h
		if(sources.size() == 1)
		{
			auto p = vertices(h);
			auto s = find_if(p.first,p.second,[&](virt_desc _w)
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
				auto s = find_if(p.first,p.second,[&](virt_desc _w)
					{ auto w = get_vertex(_w,h); return std::get<2>(w) && *std::get<2>(w) == v; });
				ensure(s != p.second);

				insert_edge(0,source,*s,h);
			}
		}

		ensure(source == root(h));

		// ensure single sink node in h
		if(sinks.size() == 0)
		{
			auto p = vertices(h);
			auto s = find_if(p.first,p.second,[&](virt_desc _w)
				{ return _w != source; });

			if(s != p.second)
				sink = *s;
			else
				sink = source;
		}
		else if(sinks.size() == 1)
		{
			auto p = vertices(h);
			auto s = find_if(p.first,p.second,[&](virt_desc _w)
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
				auto s = find_if(p.first,p.second,[&](virt_desc _w)
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

				ensure(lf >= 0);
				ensure(lt >= 0);
				ensure(lt >= lf);

				if(lt - lf > 1)
				{
					remove_edge(edge,h);
					done = false;

					int r = lf + 1;
					virt_desc prev = from;

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
		using node = typename std::remove_reference<decltype(graph)>::type::vertex_descriptor;

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
	order(const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::pair<int,int>>& lambda, const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>& widths, int nodesep, const po::digraph<N,E>& graph)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using virt_vx = typename std::tuple<int,int,boost::optional<vx_desc>>;
		using virt_graph = typename po::digraph<virt_vx,int>;
		using virt_graph2 = typename po::digraph<virt_vx,std::pair<int,int>>;

		if(num_vertices(graph) == 0)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>();
		else if(num_vertices(graph) == 1)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>{ { std::make_pair(*vertices(graph).first,0) } };
		if(num_vertices(graph) == 2)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>{ {
				std::make_pair(*vertices(graph).first,0), std::make_pair(*(vertices(graph).first + 1),1) } };

		virt_graph h = prepare_order_graph(lambda,graph);

		try
		{
			root(h);
		}
		catch(std::runtime_error const&)
		{
			ensure(false);
		}

		std::unordered_map<typename virt_graph::vertex_descriptor,int> cur = initial_order<N,E>(h), best;

		/*for(auto vx: iters(vertices(graph)))
			cur.emplace(vx,0);*/

		// ordering
		//int iter = 0;
		//int cross = -1;
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

		// convert for x-pos placement
		virt_graph2 h2;
		std::unordered_map<typename virt_graph::vertex_descriptor,typename virt_graph2::vertex_descriptor> mapping;

		for(auto vx: iters(vertices(h)))
			mapping.emplace(vx,insert_vertex(get_vertex(vx,h),h2));

		for(auto _v: iters(vertices(h)))
		{
			int ord = best.at(_v);
			auto v = get_vertex(_v,h);
			auto i = std::find_if(best.begin(),best.end(),[&](std::pair<typename virt_graph::vertex_descriptor,int> p)
			{
				auto w = get_vertex(p.first,h);
				int wa = std::get<0>(w);
				int wb = std::get<1>(w);
				int va = std::get<0>(v);
				int vb = std::get<1>(v);

				ensure(wa <= wb && va <= vb);

				po::bound ww(wa,wb + 1);
				po::bound vv(va,vb + 1);

				return !(boost::icl::disjoint(ww,vv)) && p.second == ord + 1;
			});

			if(i != best.end())
			{
				int v_width = (std::get<2>(v) ? widths.at(*std::get<2>(v)) : 0);
				int w_width = (std::get<2>(get_vertex(i->second,h)) ? widths.at(*std::get<2>(get_vertex(i->second,h))) : 0);
				insert_edge(std::make_pair(0,nodesep + (v_width + w_width) / 2),mapping.at(_v),mapping.at(i->first),h2);
			}
		}

		for(auto edge: iters(edges(h)))
		{
			auto from = mapping.at(po::source(edge,h));
			auto to = mapping.at(po::target(edge,h));
			auto ne = insert_vertex(virt_vx(-1,-1,boost::none),h2);
			int omega;

			switch(!get<2>(get_vertex(from,h2)) + !std::get<2>(get_vertex(to,h2)))
			{
				case 2: omega = 8; break;
				case 1: omega = 2; break;
				case 0: omega = 1; break;
				default: throw std::out_of_range("1 + 1 > 2");
			}

			insert_edge(std::make_pair(omega,0),ne,from,h2);
			insert_edge(std::make_pair(omega,0),ne,to,h2);
		}

		single_source_sink(virt_vx(-1,-1,boost::none),std::make_pair(0,0),h2);

		net_flow<virt_vx> layer_nf(h2);
		layer_nf.solve(std::function<void(void)>([](void) {}));
		layer_nf.make_symmetric();

		// move the nodes so that all x coordinates are >= 0
		int x_correction = std::accumulate(layer_nf.lambda.begin(),layer_nf.lambda.end(),std::numeric_limits<int>::max(),[&](int a, std::pair<typename decltype(h2)::vertex_descriptor,int> b)
		{
			auto v = get_vertex(b.first,h2);
			return std::min<int>(a,b.second - (std::get<2>(v) ? widths.at(*std::get<2>(v)) / 2 : 0));
		});

		//x_correction = 0;

		// map back to graph
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int> ret;
		for(auto _v: iters(vertices(h2)))
		{
			auto v = get_vertex(_v,h2);
			if(get<2>(v))
				ret.emplace(*get<2>(v),layer_nf.lambda.at(_v) - x_correction);
		}

		ensure(ret.size() == num_vertices(graph));
		return ret;
	}
}
