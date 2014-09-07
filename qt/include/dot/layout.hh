#include <limits>
#include <list>
#include <unordered_map>

#include <boost/graph/depth_first_search.hpp>

#include <panopticon/digraph.hh>

#pragma once

namespace dot
{
	const unsigned int dummy_edge_omega = 1;
	const unsigned int graph_edge_omega = 10;

	template<typename N, typename E>
	struct net_flow
	{
		net_flow(const po::digraph<N,E>& g, boost::optional<std::unordered_map<typename po::digraph<N,E>::edge_descriptor,unsigned int>> o) : graph(g)
		{
			if(o)
			{
				omega = *o;
				ensure(omega.size() == num_edges(g));
			}

			for(auto e: iters(edges(graph)))
			{
				if(!o)
					omega.insert(std::make_pair(e,graph_edge_omega));
				else
					ensure(omega.count(e));
				delta.insert(std::make_pair(e,1));
			}
		}

		void solve(std::function<void()> bal)
		{
			using node = typename po::digraph<N,E>::vertex_descriptor;
			using edge_desc = typename po::digraph<N,E>::edge_descriptor;

			feasible_tree();

			// finds a tree edge w/ negative cut value if one exists
			auto leave_edge = std::find_if(cut_values.begin(),cut_values.end(),[&](const std::pair<edge_desc,int> &p)
					{ return p.second < 0; });

			// finds non tree edge to replace leave_edge
			while(leave_edge != cut_values.end())
			{
				const edge_desc& cut = leave_edge->first;
				std::unordered_set<node> head_nodes, tail_nodes;
				boost::optional<std::pair<edge_desc,int>> min_edge = boost::none;

				std::cout << "have leave edge" << std::endl;

				std::tie(tail_nodes,head_nodes) = partition(cut);

				for(auto edge: iters(edges(graph)))
					if(edge != cut && !cut_values.count(edge) && head_nodes.count(source(edge,graph)) && tail_nodes.count(target(edge,graph)) && (!min_edge || slack(edge) < min_edge->second))
						min_edge = std::make_pair(edge,slack(edge));

				ensure(min_edge && min_edge->second >= 0);

				// swap edges
				swap_edges(cut,min_edge->first);

				// tail node of the new edge determines the rank of its head. Nodes of the tail component are adjusted after that
				std::unordered_set<node> adjusted;
				std::function<void(node n)> adjust;
				adjust = [&](node n)
				{
					auto p = out_edges(n,graph);
					auto q = in_edges(n,graph);
					std::function<void(const edge_desc&)> op = [&](const edge_desc &edge)
					{
						if(tail_nodes.count(n) && !adjusted.count(target(edge,graph)) && cut_values.count(edge))
						{
							lambda[target(edge,graph)] = lambda.at(n) + delta.at(edge);
							adjust(target(edge,graph));
						}
					};

					ensure(adjusted.insert(n).second);

					std::for_each(p.first,p.second,op);
					std::for_each(q.first,q.second,op);
				};

				if(tail_nodes.count(target(min_edge->first,graph)))
				{
					lambda[source(min_edge->first,graph)] = lambda.at(target(min_edge->first,graph)) - delta.at(min_edge->first);
					adjust(source(min_edge->first,graph));
				}
				else
				{
					lambda[target(min_edge->first,graph)] = lambda.at(source(min_edge->first,graph)) - delta.at(min_edge->first);
					adjust(target(min_edge->first,graph));
				}
				compute_cut_values();

				ensure(lambda.size() == num_vertices(graph));

				// finds a tree edge w/ negative cut value if one exists
				leave_edge = std::find_if(cut_values.begin(),cut_values.end(),[&](const std::pair<edge_desc,int> &p)
					{ return p.second < 0; });
			}

			bal();
		}

		void swap_edges(typename po::digraph<N,E>::edge_descriptor cut, typename po::digraph<N,E>::edge_descriptor min_edge)
		{
			ensure(cut_values.erase(cut));
			ensure(cut_values.insert(std::make_pair(min_edge,0)).second);
		}

		void feasible_tree(void)
		{
			using node = typename po::digraph<N,E>::vertex_descriptor;

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

				std::unordered_set<typename po::digraph<N,E>::edge_descriptor> eds;
				auto p = in_edges(n,graph);
				auto q = out_edges(n,graph);

				std::copy_if(p.first,p.second,std::inserter(eds,eds.end()),[&](typename po::digraph<N,E>::edge_descriptor e) { return !tree.count(source(e,graph)); });
				std::copy_if(q.first,q.second,std::inserter(eds,eds.end()),[&](typename po::digraph<N,E>::edge_descriptor e) { return !tree.count(target(e,graph)); });

				for(auto g: eds)
				{
					node m = (n != source(g,graph) ? source(g,graph) : target(g,graph));
					int dir = (n != source(g,graph) ? 1 : -1);

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
						min_slacks.push_back(std::make_tuple(source(g,graph),target(g,graph),slack(g),slack(g) * dir));
					}
				}
			};

			while(true)
			{
				tree.clear();
				cut_values.clear();
				tight_tree(lambda.begin()->first);
				ensure(tree.size() <= num_vertices(graph));
				if(tree.size() == num_vertices(graph))
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
					lambda[*i] = lambda.at(*i) + delta;
					++i;
				}
			}

			compute_cut_values();
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
					ensure(delta.count(*p.first));
					ensure(lambda.count(source(*p.first,graph)));
					unsigned int rank = std::accumulate(p.first,p.second,
																 (int)lambda.at(source(*p.first,graph)) + delta.at(*p.first),
																 [&](int acc, typename po::digraph<N,E>::edge_descriptor e)
																 { return std::max(acc,(int)lambda.at(source(e,graph)) + (int)delta.at(e)); });

					ensure(lambda.insert(std::make_pair(*i,rank)).second);
				}
				else
				{
					ensure(lambda.insert(std::make_pair(*i,0)).second);
				}

				unranked.erase(i);
			}
		}

		// tail,head components
		std::pair<std::unordered_set<typename po::digraph<N,E>::vertex_descriptor>,std::unordered_set<typename po::digraph<N,E>::vertex_descriptor>>
		partition(const typename po::digraph<N,E>::edge_descriptor& cut)
		{
			using node = typename po::digraph<N,E>::vertex_descriptor;
			using edge_desc = typename po::digraph<N,E>::edge_descriptor;

			std::function<void(node,std::unordered_set<node> &visited)> dfs;
			dfs = [&](node n, std::unordered_set<node> &visited)
			{
				ensure(visited.insert(n).second);

				for(const std::pair<edge_desc,int> &e: cut_values)
				{
					const edge_desc &g = e.first;

					if(g != cut && (source(g,graph) == n || target(g,graph) == n))
					{
						node other = (source(g,graph) == n ? target(g,graph) : source(g,graph));
						if(!visited.count(other))
							dfs(other,visited);
					}
				}
			};

			ensure(source(cut,graph) != target(cut,graph));
			ensure(cut_values.count(cut));

			std::unordered_set<node> v1,v2;
			dfs(source(cut,graph),v1);
			dfs(target(cut,graph),v2);

			ensure(v1.size() + v2.size() == lambda.size());
			return std::make_pair(v1,v2);
		}

		void compute_cut_values(void)
		{
			using node = typename po::digraph<N,E>::vertex_descriptor;
			using edge_desc = typename po::digraph<N,E>::edge_descriptor;

			for(std::pair<edge_desc const,int> &p: cut_values)
				p.second = 0;

			for(std::pair<edge_desc const,int> &g: cut_values)
			{
				const edge_desc &cut = g.first;
				std::unordered_set<node> head_nodes, tail_nodes;
				int cut_value = omega.at(cut);

				std::tie(tail_nodes,head_nodes) = partition(cut);

				for(const edge_desc& edge: iters(edges(graph)))
				{
					if(edge != cut)
					{
						if(head_nodes.count(source(edge,graph)) && tail_nodes.count(target(edge,graph)))
							cut_value -= omega.at(edge);
						else if(head_nodes.count(target(edge,graph)) && tail_nodes.count(source(edge,graph)))
							cut_value += omega.at(edge);
					}
				}

				g.second = cut_value;
			}

			for(std::pair<edge_desc const,int> &g: cut_values)
				std::cout << "cut: " << g.second << std::endl;

		}

		int length(typename po::digraph<N,E>::edge_descriptor e) const
		{
			return lambda.at(target(e,graph)) - lambda.at(source(e,graph));
		}

		int slack(typename po::digraph<N,E>::edge_descriptor e) const
		{
			return length(e) - delta.at(e);
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
		dag_visitor(virt_graph* h, std::unordered_map<typename virt_graph::edge_descriptor,unsigned int> *o) : _h(h), _omega(o) {}
		dag_visitor(const dag_visitor& v) : _h(v._h), _omega(v._omega) {}

		dag_visitor& operator=(const dag_visitor& v) { _h = v._h; _omega = v._omega; return *this; }

		void initialize_vertex(vx_desc vx,const po::digraph<N,E>&)
		{
			auto vxs = vertices(*this->_h);
			if(std::none_of(vxs.first,vxs.second,[&](typename virt_graph::vertex_descriptor _w) { auto w = get_vertex(_w,*this->_h); return w && w->second == vx; }))
			{
				auto a = insert_vertex(boost::make_optional(std::make_pair(true,vx)),*this->_h);
				auto b = insert_vertex(boost::make_optional(std::make_pair(false,vx)),*this->_h);
				_omega->insert(std::make_pair(insert_edge(0,a,b,*this->_h),dummy_edge_omega));
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
				_omega->insert(std::make_pair(insert_edge(0,*bi,*ai,*this->_h),graph_edge_omega));
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
				_omega->insert(std::make_pair(insert_edge(0,*ai,*bi,*this->_h),graph_edge_omega));
		}

		void finish_edge(eg_desc,const po::digraph<N,E>&) {}

		virt_graph *_h;
		std::unordered_map<typename virt_graph::edge_descriptor,unsigned int> *_omega;
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
		std::unordered_map<typename virt_graph::edge_descriptor,unsigned int> omega;
		dag_visitor<N,E> visitor(&h,&omega);

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

		for(auto r: sources)
			boost::depth_first_search(g,visitor,color_pm_type(color),r);

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

		// insert virtual nodes
		/*for(auto edge: iters(edges(graph)))
		{
			auto from = source(e,graph), to = target(e,graph);
			int rank_first = layer.lambda.at(from);
			int rank_sec = ph1.lambda.at(to);
			node_adaptor<T> from_a(from), to_a(to);
			std::pair<node_adaptor<T>,node_adaptor<T>> edge_a(from_a,to_a);
			unsigned int omega = ph1.omega.at(e);

			ph2.nodes.insert(from_a);
			ph2.nodes.insert(to_a);
			ph2.widths.insert(std::make_pair(from_a,dimensions(from,graph).first));
			ph2.widths.insert(std::make_pair(to_a,dimensions(to,graph).first));

			ph2.lambda.insert(std::make_pair(from_a,rank_first));
			ph2.lambda.insert(std::make_pair(to_a,rank_sec));

			if(abs(rank_first - rank_sec) > 1)
			{
				node_adaptor<T> i = from_a;
				int rank = rank_first + dir;
				node_adaptor<T> tmp;

				while(rank != rank_sec)
				{
					tmp = node_adaptor<T>(virtual_node());

					ph2.widths.insert(std::make_pair(tmp,dimensions(from,graph).first));
					ph2.edges_by_tail.insert(std::make_pair(i,tmp));
					ph2.edges_by_head.insert(std::make_pair(tmp,i));
					ph2.rank_assignments.insert(std::make_pair(rank,tmp));
					ph2.lambda.insert(std::make_pair(tmp,rank));
					ph2.nodes.insert(tmp);
					ph2.weights.insert(std::make_pair(std::make_pair(i,tmp),omega));

					rank += dir;
					i = tmp;
				}

				ensure(!tmp.is_nil());
				ph2.edges_by_tail.insert(std::make_pair(tmp,to_a));
				ph2.edges_by_head.insert(std::make_pair(to_a,tmp));
				ph2.rank_assignments.insert(std::make_pair(rank,from_a));
				ph2.rank_assignments.insert(std::make_pair(rank,to_a));
				ph2.weights.insert(std::make_pair(std::make_pair(tmp,to_a),omega));
			}
			else
			{
				ph2.edges_by_tail.insert(edge_a);
				ph2.edges_by_head.insert(std::make_pair(to_a,from_a));
				ph2.rank_assignments.insert(std::make_pair(rank_first,from_a));
				ph2.rank_assignments.insert(std::make_pair(rank_sec,to_a));
				ph2.weights.insert(std::make_pair(edge_a,omega));
			}*/

			// insert dummy nodes
			// ordering
			// map back to g

			std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> ret;
			for(auto vx: iters(vertices(g)))
			{
				auto p = vertices(h);
				auto upper = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w) { auto w = get_vertex(_w,h); return w && w->first && w->second == vx; });
				auto lower = find_if(p.first,p.second,[&](typename virt_graph::vertex_descriptor _w) { auto w = get_vertex(_w,h); return w && !w->first && w->second == vx; });

				ensure(upper != p.second && lower != p.second);
				ret.emplace(vx,std::make_tuple(layer_nf.lambda.at(*upper),layer_nf.lambda.at(*lower),-1));
			}

			ensure(ret.size() == num_vertices(g));

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
