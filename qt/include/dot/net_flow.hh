#include <unordered_map>
#include <list>
#include <boost/optional.hpp>
#include <panopticon/digraph.hh>

#pragma once

template<typename N>
struct net_flow
{
	net_flow(const po::digraph<N,int>& g) : graph(g)
	{
		for(auto e: iters(edges(graph)))
			delta.insert(std::make_pair(e,1));
	}

	void solve(std::function<void()> bal)
	{
		using node = typename po::digraph<N,int>::vertex_descriptor;
		using edge_desc = typename po::digraph<N,int>::edge_descriptor;

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
					if(/*tail_nodes.count(n) &&*/ !adjusted.count(target(edge,graph)) && cut_values.count(edge))
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

		/*std::cerr << "digraph G {" << std::endl;
		for(auto e: iters(edges(graph)))
			std::cerr << po::source(e,graph).id << " -> " << target(e,graph).id << std::endl;
		for(auto v: iters(vertices(graph)))
			std::cerr << v.id << " [label=\"" << v.id << " (" << lambda[v] << ")\"]" << std::endl;
		std::cerr << "}" << std::endl;*/

		bal();
	}

	void swap_edges(typename po::digraph<N,int>::edge_descriptor cut, typename po::digraph<N,int>::edge_descriptor min_edge)
	{
		ensure(cut_values.erase(cut));
		ensure(cut_values.insert(std::make_pair(min_edge,0)).second);
	}

	void feasible_tree(void)
	{
		using node = typename po::digraph<N,int>::vertex_descriptor;

		ensure(num_vertices(graph));
		ensure(num_edges(graph));

		{
			auto p = vertices(graph);
			std::unordered_set<typename po::digraph<N,int>::vertex_descriptor> all;
			std::copy(p.first,p.second,std::inserter(all,all.end()));
			rank(all);
		}

		// tight_tree()
		std::unordered_set<typename po::digraph<N,int>::vertex_descriptor> tree;
		std::list<std::tuple<typename po::digraph<N,int>::vertex_descriptor,typename po::digraph<N,int>::vertex_descriptor,unsigned int,int>> min_slacks; // from, to, slack, delta
		std::function<void(typename po::digraph<N,int>::vertex_descriptor)> tight_tree;

		tight_tree = [&](typename po::digraph<N,int>::vertex_descriptor n)
		{
			ensure(tree.insert(n).second);

			std::unordered_set<typename po::digraph<N,int>::edge_descriptor> eds;
			auto p = in_edges(n,graph);
			auto q = out_edges(n,graph);

			std::copy_if(p.first,p.second,std::inserter(eds,eds.end()),[&](typename po::digraph<N,int>::edge_descriptor e) { return !tree.count(source(e,graph)); });
			std::copy_if(q.first,q.second,std::inserter(eds,eds.end()),[&](typename po::digraph<N,int>::edge_descriptor e) { return !tree.count(target(e,graph)); });

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

	void rank(const std::unordered_set<typename po::digraph<N,int>::vertex_descriptor>& todo)
	{
		std::unordered_set<typename po::digraph<N,int>::vertex_descriptor> unranked(todo);

		ensure(todo.size() && num_edges(graph));

		// delete old ranking
		for(auto n: todo)
			lambda.erase(n);

		while(unranked.size())
		{
			// find a ``root'', node w/o unranked in-edges
			auto i = std::find_if(unranked.begin(),unranked.end(),[&](typename po::digraph<N,int>::vertex_descriptor n)
			{
				auto p = in_edges(n,graph);
				return std::none_of(p.first,p.second,[&](typename po::digraph<N,int>::edge_descriptor e)
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
															 [&](int acc, typename po::digraph<N,int>::edge_descriptor e)
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
	std::pair<std::unordered_set<typename po::digraph<N,int>::vertex_descriptor>,std::unordered_set<typename po::digraph<N,int>::vertex_descriptor>>
	partition(const typename po::digraph<N,int>::edge_descriptor& cut)
	{
		using node = typename po::digraph<N,int>::vertex_descriptor;
		using edge_desc = typename po::digraph<N,int>::edge_descriptor;

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
		using node = typename po::digraph<N,int>::vertex_descriptor;
		using edge_desc = typename po::digraph<N,int>::edge_descriptor;

		for(std::pair<edge_desc const,int> &p: cut_values)
			p.second = 0;

		for(std::pair<edge_desc const,int> &g: cut_values)
		{
			const edge_desc &cut = g.first;
			std::unordered_set<node> head_nodes, tail_nodes;
			int cut_value = get_edge(cut,graph);

			std::tie(tail_nodes,head_nodes) = partition(cut);

			for(const edge_desc& edge: iters(edges(graph)))
			{
				if(edge != cut)
				{
					if(head_nodes.count(source(edge,graph)) && tail_nodes.count(target(edge,graph)))
						cut_value -= get_edge(edge,graph);
					else if(head_nodes.count(target(edge,graph)) && tail_nodes.count(source(edge,graph)))
						cut_value += get_edge(edge,graph);
				}
			}

			g.second = cut_value;
		}
	}

	int length(typename po::digraph<N,int>::edge_descriptor e) const
	{
		return lambda.at(target(e,graph)) - lambda.at(source(e,graph));
	}

	int slack(typename po::digraph<N,int>::edge_descriptor e) const
	{
		return length(e) - delta.at(e);
	}

	// in
	const po::digraph<N,int>& graph;
	std::unordered_map<typename po::digraph<N,int>::edge_descriptor,int> delta;
	// TODO lim-low tree

	// out
	std::unordered_map<typename po::digraph<N,int>::vertex_descriptor,int> lambda;
	std::unordered_set<typename po::digraph<N,int>::edge_descriptor> tight_tree;
	std::unordered_map<typename po::digraph<N,int>::edge_descriptor,int> cut_values;
};
