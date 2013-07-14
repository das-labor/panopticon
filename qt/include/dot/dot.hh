#ifndef DOT_HH
#define DOT_HH

#include <unordered_map>
#include <unordered_set>
#include <map>
#include <list>
#include <functional>
#include <algorithm>

#include "traits.hh"
#include "adaptor.hh"
#include "types.hh"

namespace dot
{
	template<typename T>
	void layout(T t, unsigned int ranksep, unsigned int nodesep);

	template<typename T>
	struct net_flow
	{
		typedef typename graph_traits<T>::node_type node;
		typedef std::pair<node,node> edge;

		// in
		std::unordered_set<node> nodes;
		std::unordered_multimap<node,node> edges_by_head;
		std::unordered_multimap<node,node> edges_by_tail;
		std::unordered_map<edge,unsigned int> omega;
		std::unordered_map<edge,unsigned int> delta;
		// TODO lim-low tree

		// out
		std::unordered_map<node,int> lambda;
		std::unordered_set<edge> tight_tree;
		std::unordered_map<edge,int> cut_values;
	};

	template<typename T>
	void dump(const net_flow<T> &nf);

	template<typename T>
	net_flow<T> preprocess(T graph,
												 const std::unordered_map<typename graph_traits<T>::edge_type,unsigned int> &omega,
												 const std::unordered_map<typename graph_traits<T>::edge_type,unsigned int> &delta);

	template<typename T>
	void nf_solve(std::function<void(net_flow<T>&)> balance, net_flow<T> &nf);
	template<typename T>
	void feasible_tree(net_flow<T> &nf);
	template<typename T>
	std::pair<std::unordered_set<typename graph_traits<T>::node_type>,std::unordered_set<typename graph_traits<T>::node_type>> partition(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &c, const net_flow<T> &nf);
	template<typename T>
	void rank(net_flow<T> &nf, const std::unordered_set<typename graph_traits<T>::node_type> &todo);
	template<typename T>
	void cut_values(net_flow<T> &nf);
	template<typename T>
	void swap(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &cut, const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &replace, net_flow<T> &nf);
	template<typename T>
	void normalize(std::unordered_map<typename graph_traits<T>::node_type,int> &lambda);
	template<typename T>
	void balance(net_flow<T> &nf);
	template<typename T>
	int length(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e, const net_flow<T> &nf);
	template<typename T>
	int slack(const std::pair<typename graph_traits<T>::node_type,typename graph_traits<T>::node_type> &e, const net_flow<T> &nf);
	template<typename T>
	void symmetry(net_flow<T> &nf);

	template<typename T>
	net_flow<T> cook_phase1(T graph);

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
	phase2<T> cook_phase2(T t, const net_flow<T> &ph1);
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
	net_flow<graph_adaptor<T>> cook_phase3(T t, const net_flow<T> &ph1, const phase2<T> &ph2, unsigned int node_sep);

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
#include "astar.hh"

#endif
