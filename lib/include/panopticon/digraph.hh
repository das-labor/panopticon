#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <algorithm>
#include <type_traits>

#define BOOST_RESULT_OF_USE_DECLTYPE
#include <boost/graph/graph_traits.hpp>
#include <boost/graph/adjacency_list.hpp>
#include <boost/property_map/property_map.hpp>
#include <boost/iterator/transform_iterator.hpp>
#include <boost/mpl/if.hpp>

#pragma once

namespace po
{
	template<typename N, typename E>
	using digraph = boost::adjacency_list<
		boost::vecS,
		boost::vecS,
		boost::directedS,
		boost::property<boost::vertex_name_t,boost::optional<N>>,
		boost::property<boost::edge_name_t,boost::optional<E>>
	>;

	template<typename I>
	struct iter_pair
	{
		iter_pair(const std::pair<I,I> &p) : _iters(p) {}

		I begin(void) const { return _iters.first; }
		I end(void) const { return _iters.second; }

		std::pair<I,I> _iters;
	};

	template<typename I>
	iter_pair<I> iters(const std::pair<I,I> &p)
	{
		return iter_pair<I>(p);
	}

	template<typename X, typename G,typename E>
	struct lambda_visitor
	{
		using event_filter = E;

		lambda_visitor(std::function<void(X)> fn) : m_function(fn) {}
		void operator()(X x, G g) { m_function(x); }

		std::function<void(X)> m_function;
	};

	template<typename X, typename G, typename E>
	lambda_visitor<X,G,E> make_lambda_visitor(std::function<void(X)> fn, G g, E)
	{
		return lambda_visitor<X,G,E>(fn);
	}

	template<typename G>
	typename boost::graph_traits<G>::vertex_descriptor
	root (const G &g)
	{
		std::set<typename boost::graph_traits<G>::vertex_descriptor> seen;

		for(auto i: iters(boost::vertices(g)))
		{
			auto q = out_edges(i,g);

			for(auto j: iters(q))
				seen.insert(target(j,g));
		}

		for(auto i: iters(boost::vertices(g)))
			if(!seen.count(i))
				return i;

		throw std::out_of_range("no root found");
	}

	template<typename N,typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor
	find_node(const N& n, const po::digraph<N,E> &g)
	{
		auto p = boost::vertices(g);
		auto i = std::find_if(p.first,p.second,[&](typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor v) { return get_node(v,g) == n; });
		if(i != p.second)
			return *i;
		else
			throw std::out_of_range("unknown node");
	}

	template<typename N,typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor
	find_edge(const E& e, const po::digraph<N,E> &g)
	{
		auto p = boost::edges(g);
		auto i = std::find_if(p.first,p.second,[&](typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor v) { return get_edge(v,g) == e; });
		if(i != p.second)
			return *i;
		else
			throw std::out_of_range("unknown edge");
	}

	template<typename N, typename E>
	const N &get_node(typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor n, const digraph<N,E> &g)
	{
		return *boost::get(boost::vertex_name_t(),g,n);
	}

	template<typename N, typename E>
	const E &get_edge(typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor n, const digraph<N,E> &g)
	{
		return *boost::get(boost::edge_name_t(),g,n);
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor insert_node(const N& n, digraph<N,E> &g)
	{
		auto vx = boost::add_vertex(g);
		boost::put(boost::vertex_name_t(),g,vx,n);
		return vx;
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor insert_edge(const E& e, typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor from, typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor to, digraph<N,E> &g)
	{
		auto o = add_edge(from,to,g);

		if(!o.second)
			throw std::runtime_error("edge exists");
		boost::put(boost::edge_name_t(),g,o.first,e);

		return o.first;
	}
}
