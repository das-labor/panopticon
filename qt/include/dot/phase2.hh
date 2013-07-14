#ifndef PHASE2_HH
#define PHASE2_HH

#include <algorithm>

#include "dot.hh"

namespace std
{
	template<>
	struct hash<std::pair<unsigned int, bool>>
	{
		size_t operator()(const std::pair<unsigned int, bool> &p) const
		{
			hash<unsigned int> ui;
			hash<bool> b;

			return ui(p.first) ^ b(p.second);
		}
	};
}

template<typename T>
unsigned int dot::transpose(dot::phase2<T> &ph2)
{
	typedef node_adaptor<T> node;
	bool improved;
	unsigned int ret;

	do
	{
		improved = false;
		ret = 0;

		for(std::pair<const int,std::list<node>> &r: ph2.order)
		{
			std::list<node> &order = r.second;

			if(order.size() > 1)
			{
				auto i = order.begin();

				while(i != prev(order.end()))
				{
					node a = *i;
					node b = *next(i);
					unsigned int x = crossing(a,b,ph2);
					unsigned int y = crossing(b,a,ph2);

					if(x > y)
					{
						improved = true;

						swap(a,b,ph2);

						i = std::find(order.begin(),order.end(),a);
						ret += y;
					}
					else
					{
						ret += x;
						++i;
					}
				}
			}
		}
	}
	while(improved);

	return ret;
}

template<typename T>
unsigned int dot::crossing(node_adaptor<T> a, node_adaptor<T> b, const dot::phase2<T> &ph2)
{
	typedef node_adaptor<T> node;
	std::unordered_multiset<std::pair<unsigned int,bool>> adj_a, adj_b;
	int rank_ab = ph2.lambda.at(a);
	std::function<void(const std::pair<node,node>&,std::unordered_multiset<std::pair<unsigned int,bool>>*)> op;

	op = [&](const std::pair<node,node> &edge, std::unordered_multiset<std::pair<unsigned int,bool>> *s)
	{
		node adj = edge.second;
		int rank = ph2.lambda.at(adj);
		const std::list<node> &order = ph2.order.at(rank);

		if(rank != rank_ab)
			s->insert(std::make_pair(distance(order.begin(),find(order.begin(),order.end(),adj)),rank_ab < rank));
	};

	auto p = ph2.edges_by_head.equal_range(a);
	std::for_each(p.first,p.second,bind(op,std::placeholders::_1,&adj_a));
	p = ph2.edges_by_tail.equal_range(a);
	std::for_each(p.first,p.second,bind(op,std::placeholders::_1,&adj_a));

	p = ph2.edges_by_head.equal_range(b);
	std::for_each(p.first,p.second,bind(op,std::placeholders::_1,&adj_b));
	p = ph2.edges_by_tail.equal_range(b);
	std::for_each(p.first,p.second,bind(op,std::placeholders::_1,&adj_b));

	assert(ph2.lambda.at(a) == ph2.lambda.at(b));
	unsigned int ret = 0;

	for(const std::pair<unsigned int,bool> &x: adj_b)
		for(const std::pair<unsigned int,bool> &y: adj_a)
			if(x.second == y.second && x.first < y.first)
				++ret;

	return ret;
}

template<typename T>
void dot::swap(node_adaptor<T> a, node_adaptor<T> b, dot::phase2<T> &ph2)
{
	typedef node_adaptor<T> node;
	assert(ph2.lambda.at(a) == ph2.lambda.at(b));

	int rank = ph2.lambda.at(a);
	std::list<node> &order = ph2.order[rank];

	auto i = std::find(order.begin(),order.end(),a);
	auto j = std::find(order.begin(),order.end(),b);

	assert(i != order.end() && j != order.end() && i != j);
	std::swap(*i,*j);
}

template<typename T>
dot::phase2<T> dot::cook_phase2(T graph, const net_flow<T> &ph1)
{
	typedef node_adaptor<T> node;
	typedef typename graph_traits<T>::edge_type edge;
	phase2<T> ph2;

	// insert virtual nodes
	for(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e: ph1.edges_by_tail)
	{
		typename graph_traits<T>::node_type from = e.first, to = e.second;
		int rank_first = ph1.lambda.at(from);
		int rank_sec = ph1.lambda.at(to);
		int dir = rank_first < rank_sec ? 1 : -1;
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

			assert(!tmp.is_nil());
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
		}
	}

	return ph2;
}

template<typename T>
void dot::order(phase2<T> &ph2)
{
	typedef node_adaptor<T> node;
	// initial dfs order
	std::function<void(node)> dfs;
	std::unordered_set<node> ordered;

	dfs = [&](node n)
	{
		if(ph2.lambda.count(n))
		{
			int rank = ph2.lambda.at(n);

			assert(ordered.insert(n).second);

			if(ph2.order.count(rank))
				ph2.order.at(rank).push_back(n);
			else
				ph2.order.insert(std::make_pair(rank,std::list<node>({n})));
		}

		auto p = ph2.edges_by_tail.equal_range(n);
		std::for_each(p.first,p.second,[&](const std::pair<node,node> &e)
		{
			if(!ordered.count(e.second))
				dfs(e.second);
		});
	};

	std::unordered_multimap<int,node> lambda;
	auto i = ph2.rank_assignments.begin();
	while(i != ph2.rank_assignments.end())
	{
		if(!ordered.count(i->second))
			dfs(i->second);
		++i;
	}
}

template<typename T>
std::unordered_map<dot::node_adaptor<T>,double> dot::weighted_median(phase2<T> &ph2, bool down)
{
	typedef node_adaptor<T> node;
	int prev_rank, end_rank, rank;
	std::unordered_map<node,double> median;


	if(down)
	{
		prev_rank = ph2.rank_assignments.begin()->first;
		end_rank = ph2.rank_assignments.rbegin()->first;
	}
	else
	{
		prev_rank = ph2.rank_assignments.rbegin()->first;
		end_rank = ph2.rank_assignments.begin()->first;
	}

	do
	{
		if(down)
			rank = ph2.rank_assignments.upper_bound(prev_rank)->first;
		else
			rank = ph2.rank_assignments.lower_bound(prev_rank - 1)->first;

		auto p = ph2.rank_assignments.equal_range(rank);
		while(p.first != p.second)
		{
			median.insert(std::make_pair(p.first->second,median_value(p.first->second,prev_rank,ph2)));
			++p.first;
		}

		ph2.order.at(rank).sort([&](node a, node b)
				{ return median.at(a) < median.at(b); });

		prev_rank = rank;
	}
	while(rank != end_rank);

	return median;
}

template<typename T>
double dot::median_value(node_adaptor<T> n, int adj_rank, const phase2<T> &ph2)
{
	typedef node_adaptor<T> node;
	std::function<std::vector<int>(const std::vector<int>&,const std::pair<node,node>&)> binop = [&](const std::vector<int> &acc, const std::pair<node,node> &edge) -> std::vector<int>
	{
		int rank = ph2.lambda.at(edge.second);

		if(rank == adj_rank)
		{
			const std::list<node> &o = ph2.order.at(rank);
			std::vector<int> ret(acc);

			ret.push_back(distance(o.begin(),find(o.begin(),o.end(),edge.second)));
			return ret;
		}
		else
			return acc;
	};

	auto p = ph2.edges_by_head.equal_range(n);
	auto q = ph2.edges_by_tail.equal_range(n);
	std::vector<int> adj_nodes = accumulate(p.first,p.second,accumulate(q.first,q.second,std::vector<int>(),binop),binop);

	sort(adj_nodes.begin(),adj_nodes.end());

	if(adj_nodes.size() == 0)
		return -1.0;
	else if(adj_nodes.size() & 1)
		return adj_nodes.at(adj_nodes.size() / 2);
	else if(adj_nodes.size() == 2)
		return ((double)(adj_nodes.at(0) + adj_nodes.at(1))) / 2.0;
	else
	{
		double norm_l = adj_nodes.at(adj_nodes.size() / 2 - 1) - adj_nodes.at(0);
		double norm_r = adj_nodes.at(adj_nodes.size() - 1) - adj_nodes.at(adj_nodes.size() / 2);

		return (adj_nodes.at(adj_nodes.size() / 2 - 1) * norm_r + adj_nodes.at(adj_nodes.size() / 2) * norm_l) / (norm_l + norm_r);
	}
}

#endif
