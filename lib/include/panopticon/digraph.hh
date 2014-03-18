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
	using digraph = boost::adjacency_list<boost::vecS,boost::vecS,boost::directedS,boost::property<boost::vertex_name_t,boost::optional<N>>,boost::property<boost::edge_name_t,boost::optional<E>>>;
}
/*
namespace boost
{
	template<typename N, typename E>
	struct graph_traits<po::digraph<N,E>>
	{
		using base = adjacency_list<vecS,vecS,directedS,property<vertex_name_t,N>,property<edge_name_t,E>>;

		// Graph concept
		using vertex_descriptor = typename graph_traits<base>::vertex_descriptor;
		using edge_descriptor = typename graph_traits<base>::edge_descriptor;
		using directed_category = typename graph_traits<base>::directed_category;
		using edge_parallel_category = typename graph_traits<base>::edge_parallel_category;
		using traversal_category = typename graph_traits<base>::traversal_category;

		// VertexListGraph concept
		using vertex_iterator = typename graph_traits<base>::vertex_iterator;
		using vertices_size_type = typename graph_traits<base>::vertices_size_type;

		// EdgeListGraph concept
		using edge_iterator = typename graph_traits<base>::edge_iterator;
		using edges_size_type = typename graph_traits<base>::edges_size_type;

		// IncidenceGraph concept
		using out_edge_iterator = typename graph_traits<base>::out_edge_iterator;
		using degree_size_type = typename graph_traits<base>::degree_size_type;

		// BidirectionalGraph concept
		using in_edge_iterator = typename graph_traits<base>::in_edge_iterator;
	};
}*/

namespace po
{
	/*template<typename N, typename E>
	struct digraph
	{
		using vertex_descriptor = typename boost::graph_traits<digraph<N,E>>::vertex_descriptor;
		using edge_descriptor = typename boost::graph_traits<digraph<N,E>>::edge_descriptor;
		using vertex_iterator = typename boost::graph_traits<digraph<N,E>>::vertex_iterator;
		using edge_iterator = typename boost::graph_traits<digraph<N,E>>::edge_iterator;
		using out_edge_iterator = typename boost::graph_traits<digraph<N,E>>::out_edge_iterator;
		using in_edge_iterator = typename boost::graph_traits<digraph<N,E>>::in_edge_iterator;
		using size_type = size_t;

		digraph(void) : _adjacency_list() {}

		const N &get_node(const vertex_descriptor& n) const
		{
			return get(boost::vertex_name_t(),n,_adjacency_list);
		}

		N &get_node(const vertex_descriptor &n)
		{
			return get(boost::vertex_name_t(),n,_adjacency_list);
		}

		const E &get_edge(const edge_descriptor& e) const
		{
			return (_edges.count(e) ? _edges.at(e) : throw std::out_of_range("no such edge"));
		}

		E &get_edge(const edge_descriptor& e)
		{
			return (_edges.count(e) ? _edges[e] : throw std::out_of_range("no such edge"));
		}

		vertex_descriptor insert_node(const N& n)
		{
			vertex_descriptor vx = add_vertex(_adjacency_list);
			_nodes.emplace(vx,n);

			return vx;
		}

		edge_descriptor insert_edge(const E& e, vertex_descriptor from, vertex_descriptor to)
		{
			auto o = add_edge(from,to,_adjacency_list);

			if(!o.second)
				throw std::runtime_error("edge exists");

			_edges.emplace(o.first,e);

			return o.first;
		}

	private:
		boost::adjacency_list<boost::vecS,boost::vecS,boost::directedS,boost::property<boost::vertex_name_t,N>,boost::property<boost::edge_name_t,E>> _adjacency_list;

		friend std::pair<vertex_iterator,vertex_iterator> vertices(const digraph &);
	};

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<digraph<N,E>>::vertex_iterator,
						typename boost::graph_traits<digraph<N,E>>::vertex_iterator>
	vertices(const digraph<N,E> &g)
	{
		return boost::vertices(g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::vertices_size_type
	num_vertices(const digraph<N,E> &g)
	{
		return boost::num_vertices(g._adjacency_list);
	}

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<digraph<N,E>>::edge_iterator,
						typename boost::graph_traits<digraph<N,E>>::edge_iterator>
	edges(const digraph<N,E> &g)
	{
		return boost::edges(g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::edges_size_type
	num_edges(const digraph<N,E> &g)
	{
		return boost::num_edges(g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::vertex_descriptor
	source(const typename boost::graph_traits<digraph<N,E>>::edge_descriptor &e, const digraph<N,E> &g)
	{
		return boost::source(e,g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::vertex_descriptor
	target(const typename boost::graph_traits<digraph<N,E>>::edge_descriptor &e, const digraph<N,E> &g)
	{
		return boost::target(e,g._adjacency_list);
	}

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<digraph<N,E>>::out_edge_iterator,
						typename boost::graph_traits<digraph<N,E>>::out_edge_iterator>
	out_edges(const typename boost::graph_traits<digraph<N,E>>::vertex_descriptor &v, const digraph<N,E> &g)
	{
		return boost::out_edges(v,g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::degree_size_type
	out_degree(const typename boost::graph_traits<digraph<N,E>>::vertex_descriptor &v, const digraph<N,E> &g)
	{
		return boost::out_degree(v,g._adjacency_list);
	}

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<digraph<N,E>>::in_edge_iterator,
						typename boost::graph_traits<digraph<N,E>>::in_edge_iterator>
	in_edges(const typename boost::graph_traits<digraph<N,E>>::vertex_descriptor &v, const digraph<N,E> &g)
	{
		return boost::in_edges(v,g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::degree_size_type
	in_degree(const typename boost::graph_traits<digraph<N,E>>::vertex_descriptor &v, const digraph<N,E> &g)
	{
		return boost::in_degree(v,g._adjacency_list);
	}

	template<typename N, typename E>
	typename boost::graph_traits<digraph<N,E>>::degree_size_type
	degree(const typename boost::graph_traits<digraph<N,E>>::vertex_descriptor &v, const digraph<N,E> &g)
	{
		return boost::degree(v,g._adjacency_list);
	}*/

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
		auto p = boost::vertices(g);
		auto i = p.first;

		while(i != p.second)
		{
			//TODO
			auto q = out_edges(*i,g);

			if(!std::distance(q.first,q.second))
				return *i;
			else
				++i;
		}

		throw std::runtime_error("no root found");
	}

	template<typename N,typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor
	find_node(const N& n, const po::digraph<N,E> &g)
	{
		auto p = boost::vertices(g);
		return *std::find_if(p.first,p.second,[&](typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor v) { return get_node(v,g) == n; });
	}

	template<typename N,typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor
	find_edge(const E& e, const po::digraph<N,E> &g)
	{
		auto p = boost::edges(g);
		return *std::find_if(p.first,p.second,[&](typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor v) { return get_edge(v,g) == e; });
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
