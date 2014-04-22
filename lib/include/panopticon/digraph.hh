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
#include <boost/iterator/filter_iterator.hpp>
#include <boost/shared_container_iterator.hpp>
#include <boost/mpl/if.hpp>
#include <boost/shared_ptr.hpp>

#pragma once

namespace po
{
	struct digraph_vertex_prop_tag
	{
		using kind = boost::vertex_property_tag;
		static std::size_t const num;
	};

	struct digraph_edge_prop_tag
	{
		using kind = boost::edge_property_tag;
		static std::size_t const num;
	};

	template<typename N, typename E>
	using digraph = boost::adjacency_list<
		boost::vecS,
		boost::vecS,
		boost::directedS,
		boost::property<digraph_vertex_prop_tag,boost::optional<N>>,
		//boost::property<boost::vertex_name_t,boost::optional<N>>,
		boost::property<digraph_edge_prop_tag,boost::optional<E>>
		//boost::property<boost::edge_name_t,boost::optional<E>>
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
		return *boost::get(digraph_vertex_prop_tag(),g,n);
	}

	template<typename N, typename E>
	const E &get_edge(typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor n, const digraph<N,E> &g)
	{
		return *boost::get(digraph_edge_prop_tag(),g,n);
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor insert_node(const N& n, digraph<N,E> &g)
	{
		auto vx = boost::add_vertex(g);
		boost::put(digraph_vertex_prop_tag(),g,vx,n);
		return vx;
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor insert_edge(const E& e, typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor from, typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor to, digraph<N,E> &g)
	{
		auto o = add_edge(from,to,g);

		if(!o.second)
			throw std::runtime_error("edge exists");
		boost::put(digraph_edge_prop_tag(),g,o.first,e);

		return o.first;
	}

	template<typename N, typename E>
	std::pair<typename boost::shared_container_iterator<std::set<typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor>>,typename boost::shared_container_iterator<std::set<typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor>>>
	in_edges(typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor vx, const digraph<N,E> &g)
	{
		using cont = std::set<typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor>;
		using iter = boost::shared_container_iterator<cont>;

		auto p = edges(g);
		boost::shared_ptr<cont> ret(new cont());

		for(auto e: iters(p))
			if(target(e,g) == vx)
				ret->insert(e);

			/*	using pred = std::function<bool(typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor)>;
		using filter_iterator = boost::filter_iterator<pred,decltype(p.first)>;

		pred fn = [g,p,vx](typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor wx) { return target(wx,g) == vx; };
		return std::make_pair(filter_iterator(fn,p.first),filter_iterator(fn,p.second));*/

		return std::make_pair(iter(ret->begin(),ret),iter(ret->end(),ret));
	}
}
