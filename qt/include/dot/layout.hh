#include <limits>
#include <list>
#include <unordered_map>

#include <boost/graph/depth_first_search.hpp>

#include <panopticon/digraph.hh>

#pragma once

namespace dot
{
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

		void solve(std::function<void()> bal)
		{
			feasible_tree();
/*
			auto leave_edge = std::find_if(cut_values.begin(),cut_values.end(),[&](const std::pair<std::pair<node,node>,int> &p)
					{ return p.second < 0; });

			while(leave_edge != cut_values.end())
			{
				const std::pair<node,node> &cut = leave_edge->first;
				std::unordered_set<node> head_nodes, tail_nodes;
				typename std::unordered_multimap<node,node>::const_iterator min_edge = nf.edges_by_tail.end(), i = nf.edges_by_tail.begin();
				int min_slack = 0;

				std::tie(tail_nodes,head_nodes) = partition(cut);

				while(i != nf.edges_by_tail.end())
				{
					const std::pair<node,node> &edge = *i;

					if(edge != cut && !cut_values.count(edge) && head_nodes.count(edge.first) && tail_nodes.count(edge.second))
					{
						if(min_edge == nf.edges_by_tail.end() || slack(edge) < min_slack)
						{
							min_slack = slack(edge);
							min_edge = i;
						}
					}

					++i;
				}

				ensure(min_edge != nf.edges_by_tail.end() && min_slack >= 0);

				// swap edges
				swap(cut,*min_edge);

				// tail node of the new edge deterstd::mins the rank of its head. Nodes of the tail component are adjusted after that
				std::unordered_set<node> adjusted;
				std::function<void(node n)> adjust;
				adjust = [&](node n)
				{
					auto p = nf.edges_by_head.equal_range(n);
					auto q = nf.edges_by_tail.equal_range(n);
					std::function<void(const std::pair<node,node>&)> op = [&](const std::pair<node,node> &edge)
					{
						if(tail_nodes.count(n) &&
							 !adjusted.count(edge.second) &&
							 (cut_values.count(std::make_pair(edge.second,edge.first)) || cut_values.count(edge)))
						{
							int d = delta.count(edge) ? delta.at(edge) : -1 * delta.at(std::make_pair(edge.second,edge.first));
							lambda[edge.second] = lambda.at(n) + d;

							adjust(edge.second);
						}
					};

					ensure(adjusted.insert(n).second);

					std::for_each(p.first,p.second,op);
					std::for_each(q.first,q.second,op);
				};

				lambda[min_edge->second] = lambda.at(min_edge->first) - delta.at(*min_edge);
				adjust(min_edge->second);
				cut_values();

				ensure(lambda.size() == nodes.size());

				leave_edge = std::find_if(cut_values.begin(),cut_values.end(),[&](const std::pair<std::pair<node,node>,int> &p)
					{ return p.second < 0; });
			}

			bal(*this);*/
		}

	private:
		void feasible_tree(void)
		{
			ensure(num_vertices(graph));
			ensure(num_edges(graph));

			{
				auto p = vertices(graph);
				std::unordered_set<typename po::digraph<N,E>::vertex_descriptor> all;
				std::copy(p.first,p.second,std::inserter(all,all.end()));
				rank(all);
			}

			// tight_tree()
			std::unordered_set<typename po::digraph<N,E>::vertex_descriptor> tree;
			std::list<std::tuple<typename po::digraph<N,E>::vertex_descriptor,typename po::digraph<N,E>::vertex_descriptor,unsigned int,int>> min_slacks; // from, to, slack, delta
			std::function<void(typename po::digraph<N,E>::vertex_descriptor)> tight_tree;

			tight_tree = [&](typename po::digraph<N,E>::vertex_descriptor n)
			{
				ensure(tree.insert(n).second);

				std::unordered_set<po::digraph<N,E>::edge_descriptor> eds;
				auto p = in_edges(n,graph);
				auto q = out_edges(n,graph);

				std::copy_if(p.first,p.second,std::inserter(eds,eds.end()),[&](typename po::digraph<N,E>::edge_descriptor e) { return tree.count(source(e,graph)); });
				std::copy_if(q.first,q.second,std::inserter(eds,eds.end()),[&](typename po::digraph<N,E>::edge_descriptor e) { return tree.count(target(e,graph)); });

				for(auto g: eds)
				{
					node m = g.first;
					std::pair<node,node> edge = g.second;

					ensure(m != n);
					ensure(slack(g) >= 0);

					if(slack(g) == 0)
					{
						if(!tree.count(m))
						{
							cut_values.insert(std::make_pair(g,0));
							tight_tree(m);
						}
					}
					else
					{
						min_slacks.push_back(std::make_tuple(g.first,edge.second,slack(edge),slack(edge) * (edge.second == n ? -1 : 1)));
					}
				}
			};

			while(true)
			{
				tree.clear();
				nf.cut_values.clear();
				dfs(nf.lambda.begin()->first);

				ensure(tree.size() <= nf.nodes.size());
				if(tree.size() == nf.nodes.size())
					break;

				node n, m;
				unsigned int slack;
				int delta;

				ensure(min_slacks.size());
				min_slacks.sort([&](const std::tuple<node,node,unsigned int,int> &a, const std::tuple<node,node,unsigned int,int> &b)
						{ return std::get<2>(a) < std::get<2>(b); });

				std::tie(n,m,slack,delta) = *std::find_if(min_slacks.begin(),min_slacks.end(),[&](const std::tuple<node,node,unsigned int,int> &a)
						{ return tree.count(std::get<0>(a)) + tree.count(std::get<1>(a)) == 1; });

				auto i = tree.begin();
				while(i != tree.end())
				{
					nf.lambda[*i] = nf.lambda.at(*i) + delta;
					++i;
				}
			}

			//cut_values(nf);
		}

		void rank(const std::unordered_set<typename po::digraph<N,E>::vertex_descriptor>& todo)
		{
			std::unordered_set<typename po::digraph<N,E>::vertex_descriptor> unranked(todo);

			ensure(todo.size() && num_edges(graph));

			// delete old ranking
			for(auto n: todo)
				lambda.erase(n);

			while(unranked.size())
			{
				// find a ``root'', node w/o unranked in-edges
				auto i = std::find_if(unranked.begin(),unranked.end(),[&](typename po::digraph<N,E>::vertex_descriptor n)
				{
					auto p = in_edges(n,graph);
					return std::none_of(p.first,p.second,[&](typename po::digraph<N,E>::edge_descriptor e)
						{ return unranked.count(source(e,graph)); });
				});
				ensure(i != unranked.end());

				// assign rank
				auto p = in_edges(*i,graph);


				if(p.first != p.second)
				{
					unsigned int rank = std::accumulate(p.first,p.second,
																 (int)lambda.at(source(*p.first,graph)) + delta.at(*p.first),
																 [&](int acc, typename po::digraph<N,E>::edge_descriptor e)
																 { return std::max(acc,(int)lambda.at(source(e,graph)) + (int)delta.at(e)); });

					ensure(lambda.insert(std::make_pair(*i,rank)).second);
				}
				unranked.erase(i);
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

	template<typename N,typename E>
	struct dag_visitor
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<std::pair<bool,vx_desc>>; // true <=> upper node
		using virt_graph = typename po::digraph<virt_vx,int>;

		dag_visitor(void) : _h(0) {}
		dag_visitor(virt_graph& h) : _h(&h) {}
		dag_visitor(const dag_visitor& v) : _h(v._h) {}

		dag_visitor& operator=(const dag_visitor& v) { _h = v._h; return *this; }

		void initialize_vertex(vx_desc,const po::digraph<N,E>&) {}
		void start_vertex(vx_desc,const po::digraph<N,E>&) {}
		void discover_vertex(vx_desc vx,const po::digraph<N,E>&)
		{
			auto a = insert_vertex(boost::make_optional(std::make_pair(true,vx)),*this->_h);
			auto b = insert_vertex(boost::make_optional(std::make_pair(false,vx)),*this->_h);
			insert_edge(0,a,b,*this->_h);
		}

		void finish_vertex(vx_desc,const po::digraph<N,E>&) {}
		void examine_edge(eg_desc,const po::digraph<N,E>&) {}
		void tree_edge(eg_desc,const po::digraph<N,E>&) {}
		void back_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			auto p = vertices(*this->_h);
			auto ai = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == source(e,g) && !v->first; });
			auto bi = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == target(e,g) && v->first; });

			ensure(ai != p.second && bi != p.second);
			insert_edge(0,*bi,*ai,*this->_h);
		}

		void forward_or_cross_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			auto p = vertices(*this->_h);
			auto ai = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == source(e,g) && !v->first; });
			auto bi = std::find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _v)
				{ auto v = get_vertex(_v,*this->_h); return v && v->second == target(e,g) && v->first; });

			ensure(ai != p.second && bi != p.second);
			insert_edge(0,*ai,*bi,*this->_h);
		}

		void finish_edge(eg_desc,const po::digraph<N,E>&) {}

		virt_graph *_h;
	};

	// min-level, max-level, x order
	template<typename N,typename E>
	std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> layout(const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<std::pair<bool,vx_desc>>; // true <=> upper node
		using virt_graph = typename po::digraph<virt_vx,int>;

		// dfs
		// convert g to DAG w/ two nodes per g-node and a single source and sink
		using color_pm_type = boost::associative_property_map<std::unordered_map<vx_desc,boost::default_color_type>>;

		if(num_vertices(g) == 0)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>();
		else if(num_vertices(g) == 1)
			return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>({std::make_pair(*vertices(g).first,std::make_tuple(0,0,0))});

		virt_graph h;
		std::unordered_map<vx_desc,boost::default_color_type> color;
		dag_visitor<N,E> v(h);

		std::list<vx_desc> sources, sinks;

		for(auto vx: iters(vertices(g)))
		{
			int o = out_degree(vx,g);
			int i = in_degree(vx,g);

			if(o == 0 && i > 0)
				sinks.push_back(i);
			else if(o > 0 && i == 0)
				sources.push_back(i);
		}

		for(auto r: sources)
			boost::depth_first_search(g,v,color_pm_type(color),r);

		typename virt_graph::vertex_descriptor source, sink;

		// ensure single source node in h
		if(sources.size() == 0)
		{
			source = *vertices(h).first;
		}
		else if(sources.size() == 1)
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

				insert_edge(0,source,*s,h);
			}
		}

		// ensure single sink node in h
		if(sinks.size() == 0)
		{
			sink = *vertices(h).first;
		}
		else if(sinks.size() == 1)
		{
			auto p = vertices(h);
			auto s = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w)
				{ auto w = get_vertex(_w,h); return w && !w->first && w->second == sources.front(); });
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

				insert_edge(0,*s,sink,h);
			}
		}

		// layer assign
		net_flow<virt_vx,int> layer_nf(h);
		layer_nf.solve(std::function<void(void)>([](void) {}));


		// insert dummy nodes
		// ordering
		// map back to g

		return std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>();
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
