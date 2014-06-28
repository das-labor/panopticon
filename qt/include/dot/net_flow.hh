#ifndef NETFLOW_HH
#define NETFLOW_HH

#include <iostream>
#include <functional>
#include <algorithm>
#include <list>
#include <unordered_set>
#include <map>
#include <numeric>

#include <panopticon/ensure.hh>

#include "dot/dot.hh"

// tail,head components
template<typename T>
std::pair<std::unordered_set<typename dot::graph_traits<T>::node_type>,std::unordered_set<typename dot::graph_traits<T>::node_type>>
dot::partition(const std::pair<typename dot::graph_traits<T>::node_type,typename dot::graph_traits<T>::node_type> &cut, const dot::net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;

	std::function<void(node,std::unordered_set<node> &visited)> dfs;
	dfs = [&](node n, std::unordered_set<node> &visited)
	{
		ensure(visited.insert(n).second);

		for(const std::pair<std::pair<node,node>,int> &e: nf.cut_values)
		{
			const std::pair<node,node> &g = e.first;

			if(g != cut && (g.first == n || g.second == n))
			{
				node other = (g.first == n ? g.second : g.first);
				if(!visited.count(other))
					dfs(other,visited);
			}
		}
	};

	ensure(cut.first != cut.second);
	ensure(nf.cut_values.count(cut));

	std::unordered_set<node> v1,v2;
	dfs(cut.first,v1);
	dfs(cut.second,v2);

	ensure(v1.size() + v2.size() == nf.lambda.size());
	return std::make_pair(v1,v2);
}

template<typename T>
void dot::cut_values(net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;

	for(std::pair<std::pair<node,node> const,int> &p: nf.cut_values)
		p.second = 0;

	for(std::pair<std::pair<node,node> const,int> &g: nf.cut_values)
	{
		const std::pair<node,node> &cut = g.first;
		std::unordered_set<node> head_nodes, tail_nodes;
		int cut_value = nf.omega.at(cut);

		std::tie(tail_nodes,head_nodes) = partition(cut,nf);

		for(const std::pair<node,node> &edge: nf.edges_by_tail)
		{
			if(edge != cut)
			{
				if(head_nodes.count(edge.first) && tail_nodes.count(edge.second))
					cut_value -= nf.omega.at(edge);
				else if(head_nodes.count(edge.second) && tail_nodes.count(edge.first))
					cut_value += nf.omega.at(edge);
			}
		}

		g.second = cut_value;
	}
}

template<typename T>
void dot::normalize(std::unordered_map<typename dot::graph_traits<T>::node_type,int> &lambda)
{
	typedef typename graph_traits<T>::node_type node;

	ensure(lambda.size());
	int min_rank = std::accumulate(lambda.begin(),
																 lambda.end(),
																 lambda.begin()->second,
																 [&](int acc, const std::pair<node,int> &p) { return std::min(acc,p.second); });

	if(min_rank != 0)
		for(std::pair<node const,int> &p: lambda)
			p.second -= min_rank;
}

template<typename T>
void dot::rank(net_flow<T> &nf, const std::unordered_set<typename dot::graph_traits<T>::node_type> &todo)
{
	typedef typename graph_traits<T>::node_type node;
	std::unordered_set<node> unranked(todo);

	ensure(todo.size() && nf.edges_by_head.size());

	// delete old ranking
	for(node n: todo)
		nf.lambda.erase(n);

	while(unranked.size())
	{
		// find a ``root'', node w/o unranked in-edges
		auto i = std::find_if(unranked.begin(),unranked.end(),[&](node n)
		{
			typename std::unordered_multimap<node,node>::const_iterator j,jend;

			std::tie(j,jend) = nf.edges_by_head.equal_range(n);
			return std::none_of(j,jend,[&](const std::pair<node,node> &p) { return unranked.count(p.second); });
		});
		ensure(i != unranked.end());

		// assign rank
		unsigned int rank = 0;
		typename std::unordered_multimap<node,node>::const_iterator j,jend;

		std::tie(j,jend) = nf.edges_by_head.equal_range(*i);

		// in-edges
		if(j != jend)
			rank = std::accumulate(j,jend,
														 (int)nf.lambda.at(j->second) + nf.delta.at(std::make_pair(j->second,j->first)),
														 [&](int acc, const std::pair<node,node> &p)
														 { return std::max(acc,(int)nf.lambda.at(p.second) + (int)nf.delta.at(std::make_pair(p.second,p.first))); });

		ensure(nf.lambda.insert(std::make_pair(*i,rank)).second);
		unranked.erase(i);
	}
}

template<typename T>
void dot::balance(net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;
	std::map<int,unsigned int> rank_count;

	for(std::pair<node const,int> &rank: nf.lambda)
	{
		std::pair<typename std::unordered_multimap<node,node>::const_iterator,typename std::unordered_multimap<node,node>::const_iterator> in_p = nf.edges_by_head.equal_range(rank.first);
		auto out_p = nf.edges_by_tail.equal_range(rank.first);
		int min_rank, max_rank;
		unsigned int in_weight = 0, out_weight = 0;

		if(in_p.first != in_p.second)
		{
			min_rank = accumulate(in_p.first,in_p.second,std::numeric_limits<int>::min(),
														[&](int acc, const std::pair<node,node> &p) -> int
														{
															int d = nf.delta.at(std::make_pair(p.second,p.first));
															return std::max(acc,nf.lambda.at(p.second) + d);
														});
			in_weight = accumulate(in_p.first,in_p.second,0,[&](unsigned int acc, const std::pair<node,node> &p)
					{ return acc + nf.omega.at(std::make_pair(p.second,p.first)); });
		}
		else
		{
			min_rank = 0;
		}

		if(out_p.first != out_p.second)
		{
			max_rank = accumulate(out_p.first,out_p.second,std::numeric_limits<int>::max(),
														[&](int acc, const std::pair<node,node> &p)
														{
															int d = nf.delta.at(p);
															return std::min(acc,nf.lambda.at(p.second) - d);
														});
			out_weight = accumulate(out_p.first,out_p.second,0,[&](unsigned int acc, const std::pair<node,node> &p)
					{ return acc + nf.omega.at(p); });
		}
		else
		{
			max_rank = min_rank;
		}

		if(out_weight == in_weight && max_rank != min_rank)
		{
			if(!(max_rank >= 0 && min_rank >= 0))
			{
				std::cerr << "min_rank: " << min_rank << std::endl
									<< "max_rank: " << max_rank << std::endl;
				ensure(false);
			}

			ensure(max_rank >= min_rank);

			// on-demand computation of a std::map rank -> # of assignments
			if(rank_count.empty())
			{
				int min_possible, max_possible;

				min_possible = max_possible = rank.second;
				for(const std::pair<node,int> &p: nf.lambda)
				{
					if(rank_count.count(p.second))
						rank_count[p.second] += 1;
					else
						rank_count[p.second] = 1;
					min_possible = std::min(min_possible,p.second);
					max_possible = std::max(max_possible,p.second);
				}

				if(max_possible - min_possible + 1 > static_cast<int>(rank_count.size()))
				{
					int i = min_possible;
					while(i <= max_possible)
					{
						if(!rank_count.count(i))
							rank_count[i] = 0;
						++i;
					}
				}
			}

			std::pair<int,unsigned int> best_rank(rank.second,rank_count[rank.second]);
			auto rank_i = rank_count.lower_bound(min_rank);
			while(rank_i != rank_count.end() && rank_i->first <= max_rank)
			{
				if(best_rank.second > rank_i->second)
					best_rank = *rank_i;
				++rank_i;
			}

			if(best_rank.first != rank.second)
			{
				rank_count[rank.second] -= 1;
				rank_count[best_rank.first] += 1;
				rank.second = best_rank.first;
			}
		}
	}
}

template<typename T>
void dot::symmetry(net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;

	for(const std::pair<std::pair<node,node>,int> &cut: nf.cut_values)
	{
		if(cut.second == 0)
		{
			std::unordered_set<node> head_component,tail_component;
			const std::pair<node,node> *min_edge = 0;
			int min_slack = 0;

			std::tie(tail_component,head_component) = partition(cut.first,nf);

			for(const std::pair<node,node> &edge: nf.edges_by_tail)
			{
				if(edge != cut.first &&
					 !nf.cut_values.count(edge) &&
					 (head_component.count(edge.first) &&
					 tail_component.count(edge.second)))
				{
					if(!min_edge || slack(edge,nf) < min_slack)
					{
						min_slack = slack(edge,nf);
						min_edge = &edge;
					}
				}
			}

			if(min_edge)
			{
				if(head_component.count(min_edge->first))
					for(node n: tail_component)
						nf.lambda[n] -= min_slack/2;
				else
					for(node n: head_component)
						nf.lambda[n] += min_slack/2;
			}
		}
	}
}

template<typename T>
void dot::feasible_tree(net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;
	ensure(!nf.nodes.empty());
	ensure(nf.edges_by_head.size());
	ensure(nf.edges_by_head.size() == nf.edges_by_tail.size());

	rank(nf,nf.nodes);

	// tight_tree()
	std::unordered_set<node> tree;
	std::list<std::tuple<node,node,unsigned int,int>> min_slacks; // from, to, slack, delta
	std::function<void(node)> dfs;

	dfs = [&](node n)
	{
		ensure(tree.insert(n).second);

		auto q = nf.edges_by_tail.equal_range(n);
		auto p = nf.edges_by_head.equal_range(n);
		std::map<node,std::pair<node,node>> neight;

		for_each(q.first,q.second,[&](const std::pair<node,node> &m)
			{ if(!tree.count(m.second)) neight.insert(std::make_pair(m.second,m)); });
		for_each(p.first,p.second,[&](const std::pair<node,node> &m)
			{ if(!tree.count(m.second)) neight.insert(std::make_pair(m.second,std::make_pair(m.second,m.first))); });

		for(const std::pair<node,std::pair<node,node>> &g: neight)
		{
			node m = g.first;
			std::pair<node,node> edge = g.second;

			ensure(m != n);
			ensure(slack(edge,nf) >= 0);

			if(slack(edge,nf) == 0)
			{
				if(!tree.count(m))
				{
					nf.cut_values.insert(std::make_pair(edge,0));
					dfs(m);
				}
			}
			else
			{
				min_slacks.push_back(std::make_tuple(edge.first,edge.second,slack(edge,nf),slack(edge,nf) * (edge.second == n ? -1 : 1)));
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

	cut_values(nf);
}

template<typename T>
int dot::length(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e, const net_flow<T> &nf)
{
	return nf.lambda.at(e.second) - nf.lambda.at(e.first);
}

template<typename T>
int dot::slack(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e, const net_flow<T> &nf)
{
	return length(e,nf) - nf.delta.at(e);
}

template<typename T>
void dot::swap(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &cut, const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &min_edge, net_flow<T> &nf)
{
	ensure(nf.cut_values.erase(cut));
	ensure(nf.cut_values.insert(make_pair(min_edge,0)).second);
}

template<typename T>
dot::net_flow<T> dot::preprocess(T graph, const std::unordered_map<typename graph_traits<T>::edge_type,unsigned int> &omega,
																					const std::unordered_map<typename graph_traits<T>::edge_type,unsigned int> &delta)
{
	typedef typename graph_traits<T>::edge_type edge;
	typedef typename graph_traits<T>::node_type node;
	typedef typename graph_traits<T>::edge_iterator edgeIter;
	typedef typename graph_traits<T>::out_edge_iterator outEdgeIter;

	net_flow<T> ret;
	std::unordered_map<node,unsigned int> visited;						// List of already visited nodes in DFS and their depth (distance from the root)
	std::function<void(node, unsigned int)> dfs;
	dfs = [&](node n, unsigned int r)
	{
		outEdgeIter i,iend;

		visited.insert(std::make_pair(n,r));

		std::tie(i,iend) = out_edges(n,graph);
		std::for_each(i,iend,[&](const edge &e)
		{
			node from = source(e,graph), to = sink(e,graph);
			std::pair<node,node> ee = std::make_pair(from,to);

			if(from != to)
			{
				auto j = visited.find(to);

				if(j == visited.end())
				{
					// new edge (forward)
					ret.omega.insert(std::make_pair(ee,omega.at(e)));
					ret.delta.insert(std::make_pair(ee,delta.at(e)));
					ret.edges_by_tail.insert(std::make_pair(n,to));
					ret.edges_by_head.insert(std::make_pair(to,n));
					dfs(to,r+1);
				}
				else if(j->second < r)
				{
					// reverse back-edge
					ret.omega.insert(std::make_pair(std::make_pair(to,n),omega.at(e)));
					ret.delta.insert(std::make_pair(std::make_pair(to,n),delta.at(e)));
					ret.edges_by_tail.insert(std::make_pair(to,n));
					ret.edges_by_head.insert(std::make_pair(n,to));
				}
			}
		});
	};

	edgeIter i,iend;

	std::tie(i,iend) = edges(graph);
	std::for_each(i,iend,[&](const edge &e)
	{
		ret.nodes.insert(source(e,graph));
		ret.nodes.insert(sink(e,graph));
	});

	/*
	 * Breaking cycles by DFS:
	 * 1. User-supplied root
	 */
	if(has_entry(graph))
	{
		node r = entry(graph);
		ensure(ret.nodes.count(r));
		dfs(r,0);
	}

	/*
	 * 2. Look for a root in the graph
	 */
	edgeIter j = std::find_if(i,iend,[&](const edge &e)
	{
		return std::none_of(i,iend,[&](const edge &f)
			{ return source(f,graph) != sink(f,graph) && sink(f,graph) == source(e,graph); });
	});

	if(j != iend)
	{
		edge e = *j;
		node n = source(e,graph);
		dfs(n,0);
	}

	/*
	 * 3. Rest of the nodes
	 */
	if(static_cast<int>(visited.size()) != std::distance(i,iend))
		std::for_each(i,iend,[&](const edge &m)
		{
			if(visited.count(source(m,graph)) == 0)
				dfs(source(m,graph),0);
		});

	// insert non-tree edges that aren't back edges
	std::for_each(i,iend,[&](const edge &p)
	{
		typename std::unordered_multimap<node,node>::const_iterator i,iend,j,jend;
		std::tie(i,iend) = ret.edges_by_tail.equal_range(source(p,graph));
		std::tie(j,jend) = ret.edges_by_tail.equal_range(sink(p,graph));

		if(std::none_of(i,iend,[&](const std::pair<node,node> &q) { return q.second == sink(p,graph); }) &&
			 std::none_of(j,jend,[&](const std::pair<node,node> &q) { return q.second == source(p,graph); }) &&
			 source(p,graph) != sink(p,graph))
		{
			ret.edges_by_tail.insert(std::make_pair(source(p,graph),sink(p,graph)));
			ret.omega.insert(std::make_pair(std::make_pair(source(p,graph),sink(p,graph)),omega.at(p)));
			ret.delta.insert(std::make_pair(std::make_pair(source(p,graph),sink(p,graph)),delta.at(p)));
			ret.edges_by_head.insert(std::make_pair(sink(p,graph),source(p,graph)));
		}
	});

	ensure(ret.omega.size() == ret.edges_by_head.size());
	ensure(ret.omega.size() == ret.edges_by_tail.size());
	ensure(ret.delta.size() == ret.edges_by_head.size());
	ensure(ret.delta.size() == ret.edges_by_tail.size());

	return ret;
}

template<typename T>
void dot::dump(const dot::net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;

	std::cout << "digraph G" << std::endl
						<< "{" << std::endl;


	for(node n: nf.nodes)
	{
		typename std::unordered_multimap<node,node>::const_iterator i,iend;

		if(nf.lambda.count(n))
			std::cout << "\tn" << n << " [label=\"" << n << " " << nf.lambda.at(n) << "\"];" << std::endl;
		else
			std::cout << "\tn" << n << " [label=\"n" << n << "\"];" << std::endl;

		std::tie(i,iend) = nf.edges_by_tail.equal_range(n);

		while(i != iend)
		{
			std::pair<node,node> edge(*i++);

			std::cout << "\tn" << n << " -> n" << edge.second << " [label=\"";
			if(nf.cut_values.count(edge))
				std::cout << "cut:" << nf.cut_values.at(edge) << " ";
			if(nf.omega.count(edge))
				std::cout << "omega:" << nf.omega.at(edge) << " ";
			if(nf.delta.count(edge))
				std::cout << "delta:" << nf.delta.at(edge) << " ";
			std::cout << "\"";

			if(!nf.tight_tree.count(edge) && !nf.cut_values.count(edge))
				std::cout << " style=\"dotted\"";

			std::cout << "];" << std::endl;
		}
	}

	std::cout << "}" << std::endl;
}

template<typename T>
void dot::nf_solve(std::function<void(dot::net_flow<T>&)> bal, dot::net_flow<T> &nf)
{
	typedef typename graph_traits<T>::node_type node;

	feasible_tree(nf);

	auto leave_edge = std::find_if(nf.cut_values.begin(),nf.cut_values.end(),[&](const std::pair<std::pair<node,node>,int> &p)
			{ return p.second < 0; });

	while(leave_edge != nf.cut_values.end())
	{
		const std::pair<node,node> &cut = leave_edge->first;
		std::unordered_set<node> head_nodes, tail_nodes;
		typename std::unordered_multimap<node,node>::const_iterator min_edge = nf.edges_by_tail.end(), i = nf.edges_by_tail.begin();
		int min_slack = 0;

		std::tie(tail_nodes,head_nodes) = partition(cut,nf);

		while(i != nf.edges_by_tail.end())
		{
			const std::pair<node,node> &edge = *i;

			if(edge != cut && !nf.cut_values.count(edge) && head_nodes.count(edge.first) && tail_nodes.count(edge.second))
			{
				if(min_edge == nf.edges_by_tail.end() || slack(edge,nf) < min_slack)
				{
					min_slack = slack(edge,nf);
					min_edge = i;
				}
			}

			++i;
		}

		ensure(min_edge != nf.edges_by_tail.end() && min_slack >= 0);

		// swap edges
		swap(cut,*min_edge,nf);

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
					 (nf.cut_values.count(std::make_pair(edge.second,edge.first)) || nf.cut_values.count(edge)))
				{
					int d = nf.delta.count(edge) ? nf.delta.at(edge) : -1 * nf.delta.at(std::make_pair(edge.second,edge.first));
					nf.lambda[edge.second] = nf.lambda.at(n) + d;

					adjust(edge.second);
				}
			};

			ensure(adjusted.insert(n).second);

			std::for_each(p.first,p.second,op);
			std::for_each(q.first,q.second,op);
		};

		nf.lambda[min_edge->second] = nf.lambda.at(min_edge->first) - nf.delta.at(*min_edge);
		adjust(min_edge->second);
		cut_values(nf);

		ensure(nf.lambda.size() == nf.nodes.size());

		leave_edge = std::find_if(nf.cut_values.begin(),nf.cut_values.end(),[&](const std::pair<std::pair<node,node>,int> &p)
			{ return p.second < 0; });
	}

	bal(nf);
}
#endif
