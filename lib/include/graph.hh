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
	struct graph;

	template<typename X>
	class descriptor
	{
	public:
		static descriptor construct(const X &x) { descriptor ret; ret.m_ptr = &x; return ret; }

		descriptor(void) : m_ptr(0) {}
		descriptor(const descriptor &v) : m_ptr(v.m_ptr) {}
		descriptor &operator=(const descriptor &v)
		{
			if(m_ptr != v.m_ptr)
				m_ptr = v.m_ptr;
			return *this;
		}

		bool operator!=(const descriptor &v) const { return m_ptr != v.m_ptr; }
		bool operator==(const descriptor &v) const { return m_ptr == v.m_ptr; }

	private:
		const X&operator*(void) const
		{
			if(!m_ptr)
				throw std::runtime_error("dereference NULL descriptor");
			else
				return *m_ptr;
		}

		const X *m_ptr;

		template<typename,typename>
		friend struct graph;
		friend struct std::hash<descriptor<X>>;
	};

	template<typename K, typename V>
	struct unordered_pmap
	{
		unordered_pmap(void) : container() {}
		unordered_pmap(std::initializer_list<std::pair<K,V>> il) : container(il) {}
		unordered_pmap(const unordered_pmap &m) : container(m.container) {}

		V& operator[](const K &k)
		{
			return container[k];
		}

		typename std::unordered_map<K,V>::iterator begin(void) { return container.begin(); };
		typename std::unordered_map<K,V>::iterator end(void) { return container.end(); };

		typename std::unordered_map<K,V>::const_iterator begin(void) const { return container.begin(); };
		typename std::unordered_map<K,V>::const_iterator end(void) const { return container.end(); };

		std::unordered_map<K,V> container;
	};

	template<typename V, typename N,typename E>
	void initialize_pmap(unordered_pmap<descriptor<N>,V> &pmap, const graph<N,E> &g, const V &v)
	{
		auto p = g.nodes();
		std::for_each(p.first,p.second,[&](typename boost::graph_traits<po::graph<N,E>>::vertex_descriptor u)
			{ pmap.container.insert(std::make_pair(u,v)); });
	}
}

namespace std
{
	template<typename X>
	struct hash<po::descriptor<X>>
	{
		size_t operator()(const po::descriptor<X> &d) const
		{
			return reinterpret_cast<const size_t>(d.m_ptr);
		}
	};
}

namespace boost
{
	template<typename N, typename E>
	struct graph_traits<po::graph<N,E>>
	{
		// Graph concept
		using vertex_descriptor = po::descriptor<N>;
		using edge_descriptor = po::descriptor<E>;
		using directed_category = directed_tag;
		using edge_parallel_category = allow_parallel_edge_tag;
		struct traversal_category : public vertex_list_graph_tag, edge_list_graph_tag, bidirectional_graph_tag {};

		// VertexListGraph concept
		using vertex_iterator = typename boost::transform_iterator<std::function<po::descriptor<N>(const N&)>, typename std::unordered_set<N>::const_iterator>;
		using vertices_size_type = typename std::unordered_set<N>::size_type;

		// EdgeListGraph concept
		using edge_iterator = typename boost::transform_iterator<std::function<po::descriptor<E>(const E&)>, typename std::unordered_set<E>::const_iterator>;
		using edges_size_type = typename std::unordered_set<E>::size_type;

		// IncidenceGraph concept
		using out_edge_iterator = typename boost::transform_iterator<std::function<po::descriptor<E>(const std::pair<po::descriptor<N>,po::descriptor<E>>&)>, typename std::unordered_multimap<po::descriptor<N>,po::descriptor<E>>::const_iterator>;
		using degree_size_type = typename std::unordered_multimap<po::descriptor<N>,po::descriptor<E>>::size_type;

		// BidirectionalGraph concept
		using in_edge_iterator = typename boost::transform_iterator<std::function<po::descriptor<E>(const std::pair<po::descriptor<N>,po::descriptor<E>>&)>, typename std::unordered_multimap<po::descriptor<N>,po::descriptor<E>>::const_iterator>;
	};

	// ReadablePropertyMap concept
	template<typename K, typename V>
	struct property_traits<po::unordered_pmap<K,V>>
	{
		using value_type = V;
		using reference = const V&;
		using key_type = K;
		struct category : public lvalue_property_map_tag {};
	};

	// PropertyGraph concept for vertex_index_t
	template<typename N,typename E>
	struct property_map<po::graph<N,E>,vertex_index_t>
	{
		using type = po::unordered_pmap<po::descriptor<N>,int64_t>;
		using const_type = po::unordered_pmap<po::descriptor<N>,int64_t>;
	};
}

namespace po
{
	template<typename N, typename E>
	struct graph
	{
		using node_iterator = typename boost::transform_iterator<std::function<po::descriptor<N>(const N&)>, typename std::unordered_set<N>::const_iterator>;
		using edge_iterator = typename boost::transform_iterator<std::function<po::descriptor<E>(const E&)>, typename std::unordered_set<E>::const_iterator>;
		using out_edge_iterator = typename boost::graph_traits<graph<N,E>>::out_edge_iterator;
		using in_edge_iterator = typename boost::graph_traits<graph<N,E>>::in_edge_iterator;
		using size_type = size_t;

		graph(void) : m_nodes(), m_edges(), m_neighbors(), m_forward(), m_backward() {}

		std::pair<edge_iterator, edge_iterator>
		edges(void) const
		{
			return std::make_pair(boost::make_transform_iterator(m_edges.cbegin(),std::function<po::descriptor<E>(const E&)>([](const E &e) { return po::descriptor<E>::construct(e); })),
														boost::make_transform_iterator(m_edges.cend(),std::function<po::descriptor<E>(const E&)>([](const E &e) { return po::descriptor<E>::construct(e); })));
		}

		std::pair<node_iterator, node_iterator>
		nodes(void) const
		{
			return std::make_pair(boost::make_transform_iterator(m_nodes.begin(),std::function<po::descriptor<N>(const N&)>([](const N &n) { return po::descriptor<N>::construct(n); })),
														boost::make_transform_iterator(m_nodes.end(),std::function<po::descriptor<N>(const N&)>([](const N &n) { return po::descriptor<N>::construct(n); })));
		}

		typename boost::graph_traits<graph<N,E>>::edges_size_type
		num_edges(void) const
		{
			return m_edges.size();
		}

		typename boost::graph_traits<graph<N,E>>::vertices_size_type
		num_nodes(void) const
		{
			return m_nodes.size();
		}

		po::descriptor<N>
		source(const po::descriptor<E> &e) const
		{
			auto i = m_neighbors.find(e);

			if(!has_edge(e))
				throw std::out_of_range("unknown edge");

			if(i == m_neighbors.end())
				throw std::out_of_range("unknown edge");
			else
				return i->second.first;
		}

		po::descriptor<N>
		target(const po::descriptor<E> &e) const
		{
			auto i = m_neighbors.find(e);

			if(!has_edge(e))
				throw std::out_of_range("unknown edge");

			if(i == m_neighbors.end())
				throw std::out_of_range("unknown edge");
			else
				return i->second.second;
		}

		std::pair<out_edge_iterator,out_edge_iterator>
		out_edges(const po::descriptor<N> &d) const
		{
			if(!has_node(d))
				throw std::out_of_range("unknown node");

			auto p = m_forward.equal_range(d);
			std::function<po::descriptor<E>(const std::pair<po::descriptor<N>,po::descriptor<E>>&)> fn = [](const std::pair<po::descriptor<N>,po::descriptor<E>> &e) { return e.second; };

			return std::make_pair(boost::make_transform_iterator(p.first,fn),
														boost::make_transform_iterator(p.second,fn));
		}

		std::pair<in_edge_iterator,in_edge_iterator>
		in_edges(const po::descriptor<N> &d) const
		{
			if(!has_node(d))
				throw std::out_of_range("unknown node");

			auto p = m_backward.equal_range(d);
			std::function<po::descriptor<E>(const std::pair<po::descriptor<N>,po::descriptor<E>>&)> fn = [](const std::pair<po::descriptor<N>,po::descriptor<E>> &e) { return e.second; };

			return std::make_pair(boost::make_transform_iterator(p.first,fn),
														boost::make_transform_iterator(p.second,fn));
		}

		po::descriptor<N>
		insert_node(const N &n)
		{
			return po::descriptor<N>::construct(*m_nodes.insert(n).first);
		}

		po::descriptor<E>
		insert_edge(const E &e, const po::descriptor<N> &from, const po::descriptor<N> &to)
		{
			if(!has_node(from) || !has_node(to))
				throw std::out_of_range("unknown node");

			auto i = m_edges.insert(e).first;
			m_forward.insert(std::make_pair(from,po::descriptor<E>::construct(*i)));
			m_backward.insert(std::make_pair(to,po::descriptor<E>::construct(*i)));
			m_neighbors.insert(std::make_pair(po::descriptor<E>::construct(*i),std::make_pair(from,to)));
			return descriptor<E>::construct(*i);
		}

		void remove_edge(const po::descriptor<E> &e)
		{
			if(!has_edge(e))
				throw std::out_of_range("unknown edge");

			auto del = [](const std::pair<po::descriptor<N>,po::descriptor<E>> &p, std::unordered_multimap<po::descriptor<N>,po::descriptor<E>> &m)
			{
				auto is = m.equal_range(p.first);
				auto i = is.first;

				while(i != is.second)
				{
					if(i->second == p.second)
					{
						i = m.erase(i);
						return;
					}
					else
						++i;
				}
				assert(false);
			};
			auto n = m_neighbors.find(e);

			if(n == m_neighbors.end())
				throw std::out_of_range("unknown edge");

			del(std::make_pair(n->second.second,e),m_backward);
			del(std::make_pair(n->second.first,e),m_forward);
			m_neighbors.erase(n);
			m_edges.erase(*e);
		}

		void remove_node(const po::descriptor<N> &n)
		{
			if(!has_node(n))
				throw std::out_of_range("unknown node");

			for(auto i = m_forward.find(n); i != m_forward.end(); i = m_forward.find(n))
				remove_edge(i->second);

			for(auto i = m_backward.find(n); i != m_backward.end(); i = m_backward.find(n))
				remove_edge(i->second);

			m_nodes.erase(*n);
		}

		node_iterator
		find_node(const N& n) const
		{
			return boost::make_transform_iterator(m_nodes.find(n),std::function<po::descriptor<N>(const N&)>([](const N &n) { return po::descriptor<N>::construct(n); }));
		}

		edge_iterator
		find_edge(const E &e) const
		{
			return boost::make_transform_iterator(m_edges.find(e),std::function<po::descriptor<E>(const E&)>([](const E &n) { return po::descriptor<E>::construct(n); }));
		}

		const N &get_node(const po::descriptor<N> &n) const
		{
			if(!has_node(n))
				throw std::out_of_range("unknown node");
			return *n;
		}

		N &get_node(const po::descriptor<N> &n)
		{
			if(!has_node(n))
				throw std::out_of_range("unknown node");
			return const_cast<N&>(*n);
		}

		const E &get_edge(const po::descriptor<E> &e) const
		{
			if(!has_edge(e))
				throw std::out_of_range("unknown edge");
			return *e;
		}

		E &get_edge(const po::descriptor<E> &e)
		{
			if(!has_edge(e))
				throw std::out_of_range("unknown edge");
			return const_cast<E&>(*e);
		}

		bool has_node(const po::descriptor<N> &n) const
		{
			auto i = m_nodes.find(*n);
			return i != m_nodes.end() && &(*i) == &(*n);
		}

		bool has_edge(const po::descriptor<E> &e) const
		{
			auto i = m_edges.find(*e);
			return i != m_edges.end() && &(*i) == &(*e);
		}

	private:
		std::unordered_set<N> m_nodes;
		std::unordered_set<E> m_edges;
		std::unordered_map<po::descriptor<E>,std::pair<po::descriptor<N>,po::descriptor<N>>> m_neighbors;
		std::unordered_multimap<po::descriptor<N>,po::descriptor<E>> m_forward;
		std::unordered_multimap<po::descriptor<N>,po::descriptor<E>> m_backward;
	};

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<graph<N,E>>::vertex_iterator,
						typename boost::graph_traits<graph<N,E>>::vertex_iterator>
	vertices(const graph<N,E> &g)
	{
		return g.nodes();
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::vertices_size_type
	num_vertices(const graph<N,E> &g)
	{
		return g.num_nodes();
	}

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<graph<N,E>>::edge_iterator,
						typename boost::graph_traits<graph<N,E>>::edge_iterator>
	edges(const graph<N,E> &g)
	{
		return g.edges();
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::edges_size_type
	num_edges(const graph<N,E> &g)
	{
		return g.num_edges();
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::vertex_descriptor
	source(const typename boost::graph_traits<graph<N,E>>::edge_descriptor &e, const graph<N,E> &g)
	{
		return g.source(e);
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::vertex_descriptor
	target(const typename boost::graph_traits<graph<N,E>>::edge_descriptor &e, const graph<N,E> &g)
	{
		return g.target(e);
	}

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<graph<N,E>>::out_edge_iterator,
						typename boost::graph_traits<graph<N,E>>::out_edge_iterator>
	out_edges(const typename boost::graph_traits<graph<N,E>>::vertex_descriptor &v, const graph<N,E> &g)
	{
		return g.out_edges(v);
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::degree_size_type
	out_degree(const typename boost::graph_traits<graph<N,E>>::vertex_descriptor &v, const graph<N,E> &g)
	{
		auto p = g.out_edges(v);
		return std::distance(p.first,p.second);
	}

	template<typename N, typename E>
	std::pair<typename boost::graph_traits<graph<N,E>>::in_edge_iterator,
						typename boost::graph_traits<graph<N,E>>::in_edge_iterator>
	in_edges(const typename boost::graph_traits<graph<N,E>>::vertex_descriptor &v, const graph<N,E> &g)
	{
		return g.in_edges(v);
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::degree_size_type
	in_degree(const typename boost::graph_traits<graph<N,E>>::vertex_descriptor &v, const graph<N,E> &g)
	{
		auto p = g.in_edges(v);
		return std::distance(p.first,p.second);
	}

	template<typename N, typename E>
	typename boost::graph_traits<graph<N,E>>::degree_size_type
	degree(const typename boost::graph_traits<graph<N,E>>::vertex_descriptor &v, const graph<N,E> &g)
	{
		return out_degree(v,g) + in_degree(v,g);
	}

	template<typename K,typename V>
	const V& get(const po::unordered_pmap<K,V> &m, const K &k)
	{
		return m.container.at(k);
	}

	template<typename K,typename V>
	void put(po::unordered_pmap<K,V> &m, const K &k, const V &v)
	{
		m.container[k] = v;
	}

	template<typename N, typename E>
	unordered_pmap<typename boost::graph_traits<graph<N,E>>::vertex_descriptor,int64_t>
	get(boost::vertex_index_t, const graph<N,E> &g)
	{
		unordered_pmap<typename boost::graph_traits<graph<N,E>>::vertex_descriptor,int64_t> ret;
		auto p = g.nodes();
		auto i = p.first;

		while(i != p.second)
		{
			put(ret,*i,std::distance(p.first,i));
			++i;
		}

		return ret;
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
	iter_pair<I> iters(std::pair<I,I> &p)
	{
		return iter_pair<I>(p);
	}

	template<typename X, typename G>
	struct lambda_visitor
	{
		lambda_visitor(std::function<void(X,G)> fn) : m_function(fn) {}
		void operator()(X x, G g) { m_function(x,g); }

		std::function<void(X,G)> m_function;
	};
}
