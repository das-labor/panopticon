#ifndef ADAPTOR_HH
#define ADAPTOR_HH

#include <unordered_map>
#include <unordered_set>
#include <cstdint>
#include <memory>
#include <iostream>

#include <panopticon/ensure.hh>

namespace dot
{
	struct virtual_node;
	template<typename T> struct node_adaptor;
}

#include "dot/traits.hh"

namespace dot
{
	struct virtual_node
	{
		virtual_node(void) : index(counter++) {}
		virtual_node(const virtual_node &vn) : index(vn.index) {}

		bool operator==(const virtual_node &vn) const { return index == vn.index; }
		bool operator!=(const virtual_node &vn) const { return !(*this == vn); }
		bool operator<(const virtual_node &vn) const { return index < vn.index; }

		uint64_t index;
		static uint64_t counter;
	};

	template<typename T>
	struct node_adaptor
	{
		node_adaptor(void) : m_tag(Nil) {}
		node_adaptor(typename graph_traits<T>::node_type n) : m_tag(Node), m_node(n) {}
		node_adaptor(virtual_node vn) : m_tag(Virtual), m_virtual_node(vn) {}

		node_adaptor(const node_adaptor &na) : m_tag(na.m_tag)
		{
			if(is_virtual())
				m_virtual_node = na.vnode();
			else if(is_node())
				m_node = na.node();
		}

		node_adaptor(node_adaptor &&na) : m_tag(std::move(na.m_tag))
		{
			if(is_virtual())
				m_virtual_node = std::move(na.m_virtual_node);
			else if(is_node())
				m_node = std::move(na.m_node);
		}

		node_adaptor& operator=(const node_adaptor &na)
		{
			m_tag = na.m_tag;
			if(is_virtual())
				m_virtual_node = na.vnode();
			else if(is_node())
				m_node = na.node();
			return *this;
		}

		node_adaptor& operator=(node_adaptor &&na)
		{
			m_tag = std::move(na.m_tag);
			if(is_virtual())
				m_virtual_node = std::move(na.m_virtual_node);
			else if(is_node())
				m_node = std::move(na.m_node);
			return *this;
		}

		bool operator==(const node_adaptor &na) const
		{
			return m_tag == na.m_tag &&
						 (is_nil() ||
						  (is_node() && m_node == na.node()) ||
						  (is_virtual() && m_virtual_node == na.vnode()));
		}

		bool operator!=(const node_adaptor &na) const { return !(*this == na); }
		bool operator<(const node_adaptor &na) const
		{
			if(m_tag == na.m_tag)
				return (!is_nil() ||
							 (is_node() && m_node < na.node()) ||
							 (is_virtual() && m_virtual_node < na.vnode()));
			else
				return m_tag < na.m_tag;
		}

		bool is_node(void) const { return m_tag == Node; }
		bool is_virtual(void) const { return m_tag == Virtual; }
		bool is_nil(void) const { return m_tag == Nil; }

		typename graph_traits<T>::node_type node(void) const { return m_node; }
		virtual_node vnode(void) const { return m_virtual_node; }

	private:
		enum
		{
			Node,
			Virtual,
			Nil
		} m_tag;

		typename graph_traits<T>::node_type m_node;
		virtual_node m_virtual_node;
	};

	template<typename T>
	std::ostream& operator<<(std::ostream& os, const node_adaptor<T> &n)
	{
		if(n.is_virtual())
			os << "virtual" << n.vnode().index;
		else if(n.is_nil())
			os << "nil";
		else
			os << n.node();

		return os;
	}

	template<typename T>
	struct graph_adaptor
	{
		std::shared_ptr<std::unordered_set<node_adaptor<T>>> nodes;
		std::shared_ptr<std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>> edges;
	};

	template<typename T>
	struct graph_traits<graph_adaptor<T>>
	{
		typedef node_adaptor<T> node_type;
		typedef std::pair<node_adaptor<T>,node_adaptor<T>> edge_type;

		typedef typename std::unordered_set<node_adaptor<T>>::const_iterator node_iterator;
		typedef typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator edge_iterator;
		typedef typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator out_edge_iterator;
	};

	template<typename T>
	std::pair<typename std::unordered_set<node_adaptor<T>>::const_iterator,typename std::unordered_set<node_adaptor<T>>::const_iterator> nodes(graph_adaptor<T> t)
	{
		return std::make_pair(t.nodes->begin(),t.nodes->end());
	}

	template<typename T>
	std::pair<typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator,typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator>
	edges(graph_adaptor<T> t)
	{
		return std::make_pair(t.edges->begin(),t.edges->end());
	}

	template<typename T>
	std::pair<typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator,typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator>
	out_edges(node_adaptor<T> n, graph_adaptor<T> t)
	{
		return t.edges->equal_range(n);
	}

	/*
	template<typename T>
	std::pair<typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator,typename std::unordered_multimap<node_adaptor<T>,node_adaptor<T>>::const_iterator>
	in_edges(node_adaptor<T> n, graph_adaptor<T> t);*/

	template<typename T>
	node_adaptor<T> source(std::pair<node_adaptor<T>,node_adaptor<T>> e, graph_adaptor<T> t)
	{
		return e.first;
	}

	template<typename T>
	node_adaptor<T> sink(std::pair<node_adaptor<T>,node_adaptor<T>> e, graph_adaptor<T> t)
	{
		return e.second;
	}

	template<typename T>
	unsigned int weight(std::pair<node_adaptor<T>,node_adaptor<T>> e, graph_adaptor<T> t);
	template<typename T>
	std::pair<unsigned int,unsigned int> dimensions(node_adaptor<T> n, graph_adaptor<T> t);

	template<typename T>
	bool has_entry(graph_adaptor<T> t)
	{
		return false;
	}

	template<typename T>
	node_adaptor<T> entry(graph_adaptor<T> t)
	{
		ensure(false);
		return *t.nodes->begin();
	}
}

namespace std
{
	template<typename T>
	struct hash<dot::node_adaptor<T>>
	{
		size_t operator()(const dot::node_adaptor<T> &p) const
		{
			hash<typename dot::graph_traits<T>::node_type> h1;
			hash<uint64_t> h2;

			if(p.is_virtual())
				return h2(p.vnode().index);
			else if(p.is_node())
				return h1(p.node());
			else
				return 0;
		}
	};

	template<typename T>
	struct hash<pair<dot::node_adaptor<T>,dot::node_adaptor<T>>>
	{
		size_t operator()(const pair<dot::node_adaptor<T>,dot::node_adaptor<T>> &p) const
		{
			hash<dot::node_adaptor<T>> h;
			return h(p.first) ^ h(p.second);
		}
	};
}

#endif
