#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <algorithm>
#include <type_traits>
#include <atomic>
#include <functional>
#include <stdexcept>

#define BOOST_RESULT_OF_USE_DECLTYPE
#include <boost/graph/graph_traits.hpp>
#include <boost/property_map/property_map.hpp>
#include <boost/graph/properties.hpp>
#include <boost/shared_container_iterator.hpp>
#include <boost/optional.hpp>
#include <boost/operators.hpp>
#pragma once

namespace po
{
	template<typename F, typename I>
	class map_iterator : public boost::iterator_facade<map_iterator<F,I>,typename F::result_type,boost::forward_traversal_tag,typename F::result_type>
	{
	public:
		map_iterator(void) : _iterator(), _function() {}
		map_iterator(I i, F fn) : _iterator(i), _function(fn) {}

		map_iterator &increment(void) { ++_iterator; return *this; }
		map_iterator &decrement(void) { --_iterator; return *this; }

		typename F::result_type dereference(void) const { return _function(_iterator); }
		bool equal(const map_iterator &a) const { return _iterator == a._iterator; }
		void advance(size_t n) { std::advance(_iterator,n); }

	private:
		I _iterator;
		F _function;
	};

	template<typename I, typename T>
	struct integer_wrapper : public boost::operators<integer_wrapper<I,T>>
	{
		using integer_type = I;

		integer_wrapper() : id() {}
		integer_wrapper(I i) : id(i) {}

		bool operator==(const integer_wrapper& iw) const { return iw.id == id; }
		bool operator<(const integer_wrapper& iw) const { return id < iw.id; }

		I id;
	};
}

namespace std
{
	template<typename I, typename T>
	struct hash<po::integer_wrapper<I,T>>
	{
		size_t operator()(const po::integer_wrapper<I,T>& iw) const
		{
			return hash<I>()(iw.id);
		}
	};
}

namespace po
{
	template<typename N, typename E>
	struct digraph
	{
		struct vertex_descriptor_tag {};
		struct edge_descriptor_tag {};
		using vertex_descriptor = integer_wrapper<uint64_t,vertex_descriptor_tag>;
		using edge_descriptor = integer_wrapper<uint64_t,edge_descriptor_tag>;
		using size_type = size_t;

		digraph(void) : next_vertex(), next_edge(), vertices(), edges(), sources(), destinations(), outgoing(), incoming(), index(boost::none)
		{
			next_vertex.store(1);
			next_edge.store(1);
		}

		digraph(const digraph& d)
		: next_vertex(), next_edge(), vertices(d.vertices), edges(d.edges),
			sources(d.sources), destinations(d.destinations), outgoing(d.outgoing), incoming(d.incoming), index(d.index)
		{
			next_vertex.store(d.next_vertex);
			next_edge.store(d.next_edge);
		}

		digraph& operator=(const digraph& d)
		{
			if(&d != this)
			{
				next_vertex.store(d.next_vertex.load());
				next_edge.store(d.next_edge.load());
				vertices = d.vertices;
				edges = d.edges;
				sources = d.sources;
				destinations = d.destinations;
				outgoing = d.outgoing;
				incoming = d.incoming;
				index = d.index;
			}

			return *this;
		}

		std::atomic<typename vertex_descriptor::integer_type> next_vertex;
		std::atomic<typename edge_descriptor::integer_type> next_edge;
		std::unordered_map<vertex_descriptor,N> vertices;
		std::unordered_map<edge_descriptor,E> edges;
		std::unordered_map<edge_descriptor,vertex_descriptor> sources;
		std::unordered_map<edge_descriptor,vertex_descriptor> destinations;
		std::unordered_multimap<vertex_descriptor,edge_descriptor> outgoing;
		std::unordered_multimap<vertex_descriptor,edge_descriptor> incoming;
		mutable boost::optional<std::unordered_map<vertex_descriptor,size_t>> index;

		using adjacency_iterator = boost::shared_container_iterator<std::vector<vertex_descriptor>>;
		using out_edge_iterator = map_iterator<std::function<edge_descriptor(typename std::unordered_multimap<vertex_descriptor, edge_descriptor>::const_iterator)>, typename std::unordered_multimap<vertex_descriptor, edge_descriptor>::const_iterator>;
		using in_edge_iterator = map_iterator<std::function<edge_descriptor(typename std::unordered_multimap<vertex_descriptor, edge_descriptor>::const_iterator)>, typename std::unordered_multimap<vertex_descriptor, edge_descriptor>::const_iterator>;
		using vertex_iterator = map_iterator<std::function<vertex_descriptor(typename std::unordered_map<vertex_descriptor, N>::const_iterator)>, typename std::unordered_map<vertex_descriptor, N>::const_iterator>;
		using edge_iterator = map_iterator<std::function<edge_descriptor(typename std::unordered_map<edge_descriptor, E>::const_iterator)>, typename std::unordered_map<edge_descriptor, E>::const_iterator>;
	};
}

namespace boost
{
	template<typename N, typename E>
	struct graph_traits<po::digraph<N,E>>
	{
		// Graph concept
		using vertex_descriptor = typename po::digraph<N,E>::vertex_descriptor;
		using edge_descriptor = typename po::digraph<N,E>::edge_descriptor;
		using directed_category = boost::directed_tag;
		using edge_parallel_category = boost::disallow_parallel_edge_tag;
		struct traversal_category : public boost::bidirectional_graph_tag, adjacency_graph_tag, vertex_list_graph_tag, edge_list_graph_tag {};

		static inline vertex_descriptor null_vertex(void) { return 0; }

		// VertexListGraph concept
		using vertex_iterator = typename po::digraph<N,E>::vertex_iterator;
		using vertices_size_type = size_t;

		// EdgeListGraph concept
		using edge_iterator = typename po::digraph<N,E>::edge_iterator;
		using edges_size_type = size_t;

		// IncidenceGraph concept
		using out_edge_iterator = typename po::digraph<N,E>::out_edge_iterator;
		using degree_size_type = size_t;

		// BidirectionalGraph concept
		using in_edge_iterator = typename po::digraph<N,E>::in_edge_iterator;

		// AdjacencyGraph concept
		using adjacency_iterator = typename po::digraph<N,E>::adjacency_iterator;
	};

	template<typename N, typename E>
	struct property_map<po::digraph<N,E>,vertex_index_t>
	{
		using type = associative_property_map<std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,size_t>>;
		using const_type = associative_property_map<std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,size_t>>;
	};
}

namespace po
{
	template<typename N, typename E>
	typename boost::property_map<po::digraph<N,E>, boost::vertex_index_t>::type
	get(boost::vertex_index_t, const po::digraph<N,E>& g)
	{

		if(!g.index)
		{
			g.index = std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,size_t>();

			size_t i = 0;
			for(auto v: g.vertices)
				g.index->emplace(v.first,i++);
		}

		return boost::associative_property_map<std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,size_t>>(*g.index);
	}

	template<typename N, typename E>
	const N &get_vertex(typename po::digraph<N,E>::vertex_descriptor n, const po::digraph<N,E>& g) { return g.vertices.at(n); }

	template<typename N, typename E>
	N &get_vertex(typename po::digraph<N,E>::vertex_descriptor n, po::digraph<N,E>& g) { return g.vertices.at(n); }

	template<typename N, typename E>
	const E &get_edge(typename po::digraph<N,E>::edge_descriptor n, const po::digraph<N,E>& g) { return g.edges.at(n); }

	template<typename N, typename E>
	E &get_edge(typename po::digraph<N,E>::edge_descriptor n, po::digraph<N,E>& g) { return g.edges.at(n); }

	template<typename N, typename E>
	typename po::digraph<N,E>::vertex_descriptor insert_vertex(const N& n, po::digraph<N,E>& g)
	{
		typename po::digraph<N,E>::vertex_descriptor vx(g.next_vertex++);
		g.vertices.emplace(vx,n);
		g.index = boost::none;
		return vx;
	}

	template<typename N, typename E>
	typename po::digraph<N,E>::edge_descriptor insert_edge(const E& e, typename po::digraph<N,E>::vertex_descriptor from, typename po::digraph<N,E>::vertex_descriptor to, po::digraph<N,E>& g)
	{
		assert(g.vertices.count(from) && g.vertices.count(to));

		typename po::digraph<N,E>::edge_descriptor vx = g.next_edge++;
		g.edges.emplace(vx,e);
		g.sources.emplace(vx,from);
		g.destinations.emplace(vx,to);
		g.outgoing.emplace(from,vx);
		g.incoming.emplace(to,vx);
		g.index = boost::none;
		return vx;
	}

	template<typename N, typename E>
	std::pair<typename po::digraph<N,E>::vertex_iterator,
						typename po::digraph<N,E>::vertex_iterator>
	vertices(const po::digraph<N,E> &g)
	{
		auto fn = [](typename std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,N>::const_iterator i) { return i->first; };
		typename po::digraph<N,E>::vertex_iterator b(g.vertices.cbegin(),fn), e(g.vertices.cend(),fn);
		return std::make_pair(b,e);
	}

	template<typename N, typename E>
	std::pair<typename po::digraph<N,E>::edge_iterator,
						typename po::digraph<N,E>::edge_iterator>
	edges(const po::digraph<N,E> &g)
	{
		auto fn = [](typename std::unordered_map<typename po::digraph<N,E>::edge_descriptor,E>::const_iterator i) { return i->first; };
		typename po::digraph<N,E>::edge_iterator b(g.edges.begin(),fn), e(g.edges.end(),fn);
		return std::make_pair(b,e);
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::vertices_size_type
	num_vertices(const po::digraph<N,E> &g)
	{
		return g.vertices.size();
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::edges_size_type
	num_edges(const po::digraph<N,E> &g)
	{
		return g.edges.size();
	}

	template<typename N, typename E>
	typename po::digraph<N,E>::vertex_descriptor
	source(typename po::digraph<N,E>::edge_descriptor e, const po::digraph<N,E> &g)
	{
		assert(g.edges.count(e));
		return g.sources.at(e);
	}

	template<typename N, typename E>
	typename po::digraph<N,E>::vertex_descriptor
	target(typename po::digraph<N,E>::edge_descriptor e, const po::digraph<N,E> &g)
	{
		assert(g.edges.count(e));
		return g.destinations.at(e);
	}

	template<typename N, typename E>
	std::pair<typename po::digraph<N,E>::out_edge_iterator,
						typename po::digraph<N,E>::out_edge_iterator>
	out_edges(typename po::digraph<N,E>::vertex_descriptor v, const po::digraph<N,E> &g)
	{
		assert(g.vertices.count(v));

		auto fn = [](typename std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,typename po::digraph<N,E>::edge_descriptor>::const_iterator i) { return i->second; };
		auto p = g.outgoing.equal_range(v);
		typename po::digraph<N,E>::out_edge_iterator b(p.first,fn), e(p.second,fn);
		return std::make_pair(b,e);
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::degree_size_type
	out_degree(typename po::digraph<N,E>::vertex_descriptor v, const po::digraph<N,E> &g)
	{
		assert(g.vertices.count(v));
		return g.outgoing.count(v);
	}

	template<typename N, typename E>
	std::pair<typename po::digraph<N,E>::in_edge_iterator,
						typename po::digraph<N,E>::in_edge_iterator>
	in_edges(typename po::digraph<N,E>::vertex_descriptor v, const po::digraph<N,E> &g)
	{
		assert(g.vertices.count(v));

		auto fn = [](typename std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,typename po::digraph<N,E>::edge_descriptor>::const_iterator i) { return i->second; };
		auto p = g.incoming.equal_range(v);
		typename po::digraph<N,E>::in_edge_iterator b(p.first,fn), e(p.second,fn);
		return std::make_pair(b,e);
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::degree_size_type
	in_degree(typename po::digraph<N,E>::vertex_descriptor v, const po::digraph<N,E> &g)
	{
		assert(g.vertices.count(v));
		return g.incoming.count(v);
	}

	template<typename N, typename E>
	typename boost::graph_traits<po::digraph<N,E>>::degree_size_type
	degree(typename po::digraph<N,E>::vertex_descriptor v, const po::digraph<N,E> &g)
	{
		return in_degree(v,g) + out_degree(v,g);
	}

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

	template<typename N, typename E>
	void remove_vertex(typename po::digraph<N,E>::vertex_descriptor v, po::digraph<N,E>& g)
	{
		assert(g.vertices.count(v));

		while(g.outgoing.count(v))
			remove_edge(g.outgoing.find(v)->second,g);
		while(g.incoming.count(v))
			remove_edge(g.incoming.find(v)->second,g);

		g.vertices.erase(v);
		g.index = boost::none;
	}

	template<typename N, typename E>
	void remove_edge(typename po::digraph<N,E>::edge_descriptor e, po::digraph<N,E>& g)
	{
		assert(g.edges.count(e) && g.sources.count(e) && g.destinations.count(e) && g.outgoing.count(g.sources.at(e)) && g.incoming.count(g.destinations.at(e)));

		g.edges.erase(e);
		g.outgoing.erase(g.sources.at(e));
		g.incoming.erase(g.destinations.at(e));
		g.sources.erase(e);
		g.destinations.erase(e);
		g.index = boost::none;
	}

	template<typename N, typename E>
	std::pair<typename po::digraph<N,E>::adjacency_iterator,
						typename po::digraph<N,E>::adjacency_iterator>
	adjacent_vertices(typename po::digraph<N,E>::vertex_descriptor v, const po::digraph<N,E> &g)
	{
		boost::shared_ptr<std::vector<typename po::digraph<N,E>::vertex_descriptor>> cont(new std::vector<typename po::digraph<N,E>::vertex_descriptor>());

		for(auto p: iters(g.incoming.equal_range(v)))
		{
			auto w = g.sources.at(p.second);
			if(find(cont->begin(),cont->end(),w) == cont->end())
				cont->emplace_back(w);
		}

		for(auto p: iters(g.outgoing.equal_range(v)))
		{
			auto w = g.destinations.at(p.second);
			if(find(cont->begin(),cont->end(),w) == cont->end())
				cont->emplace_back(w);
		}

		typename po::digraph<N,E>::adjacency_iterator b(cont->begin(),cont), e(cont->end(),cont);
		return std::make_pair(b,e);
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
		auto p = vertices(g);
		auto i = p.first;

		while(i != p.second)
		{
			if(!in_degree(*i,g))
				return *i;
			else
				++i;
		}

		throw std::runtime_error("no root found");
	}

	template<typename N,typename E>
	typename po::digraph<N,E>::vertex_descriptor
	find_node(const N& n, const po::digraph<N,E> &g)
	{
		auto p = vertices(g);
		auto ret = std::find_if(p.first,p.second,[&](typename boost::graph_traits<po::digraph<N,E>>::vertex_descriptor v) { return get_vertex(v,g) == n; });

		if(ret == p.second)
			throw std::out_of_range("not found");
		else
			return *ret;
	}

	template<typename N,typename E>
	typename po::digraph<N,E>::vertex_descriptor
	find_edge(const E& e, const po::digraph<N,E> &g)
	{
		auto p = edges(g);
		auto ret = std::find_if(p.first,p.second,[&](typename boost::graph_traits<po::digraph<N,E>>::edge_descriptor v) { return get_edge(v,g) == e; });

		if(ret == p.second)
			throw std::out_of_range("not found");
		else
			return *ret;
	}
}
