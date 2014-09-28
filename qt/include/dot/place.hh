#include <unordered_map>

#include <boost/graph/depth_first_search.hpp>

#include <panopticon/digraph.hh>

#include "dot/net_flow.hh"

#pragma once

namespace dot
{
	template<typename N,typename E>
	struct place_dag_visitor
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<vx_desc>;
		using virt_graph = typename po::digraph<virt_vx,std::pair<int,int>>; // omega,delta

		place_dag_visitor(void) {}
		place_dag_visitor(virt_graph* h, std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> const* l)
		: _h(h), layout(l), mapping(new typename decltype(mapping)::element_type()) {}
		place_dag_visitor(const place_dag_visitor& v) : _h(v._h), layout(v.layout), mapping(v.mapping) {}

		place_dag_visitor& operator=(const place_dag_visitor& v) { _h = v._h; layout = v.layout; mapping = v.mapping; return *this; }

		void initialize_vertex(vx_desc vx,const po::digraph<N,E>&)
		{
			if(!mapping->count(vx))
				ensure(mapping->emplace(vx,insert_vertex(boost::make_optional(vx),*this->_h)).second);
		}

		void start_vertex(vx_desc,const po::digraph<N,E>&) {}
		void discover_vertex(vx_desc vx,const po::digraph<N,E>&) {}

		void finish_vertex(vx_desc,const po::digraph<N,E>&) {}
		void examine_edge(eg_desc,const po::digraph<N,E>&) {}
		void tree_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			forward_or_cross_edge(e,g);
		}

		void back_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			add_edge(po::target(e,g),po::source(e,g));
		}

		void forward_or_cross_edge(eg_desc e,const po::digraph<N,E>& g)
		{
			add_edge(po::source(e,g),target(e,g));
		}

		void finish_edge(eg_desc,const po::digraph<N,E>&) {}

		void add_edge(vx_desc from, vx_desc to)
		{
			if(from != to)
			{
				int r1 = std::get<1>(layout->at(from));
				int r2 = std::get<0>(layout->at(to));
				ensure(r2 >= r1);

				bool done = false;
				int r = r1 + 1;
				auto prev = mapping->at(from);

				while(r < r2)
				{
					auto ne = insert_vertex(boost::optional<vx_desc>(boost::none),*this->_h);
					auto v = insert_vertex(boost::optional<vx_desc>(boost::none),*this->_h);
					int omega = (prev == mapping->at(from) ? 2 : 8);

					// omega,delta
					insert_edge(std::make_pair(omega,0),ne,prev,*this->_h);
					insert_edge(std::make_pair(omega,0),ne,v,*this->_h);
					prev = ne;
					++r;
				}

				int omega = (prev == mapping->at(from) ? 1 : 2);
				auto ne = insert_vertex(boost::optional<vx_desc>(boost::none),*this->_h);

				insert_edge(std::make_pair(omega,0),ne,prev,*this->_h);
				insert_edge(std::make_pair(omega,0),ne,mapping->at(to),*this->_h);
			}
		}

		virt_graph *_h;
		std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>> const* layout;
		std::shared_ptr<std::unordered_map<vx_desc,typename virt_graph::vertex_descriptor>> mapping;
	};

	/// convert g to DAG w/ a single source and sink
	template<typename N,typename E>
	po::digraph<boost::optional<typename po::digraph<N,E>::vertex_descriptor>,std::pair<int,int>>
	prepare_place_graph(const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,std::tuple<int,int,int>>& layout, const std::unordered_map<typename po::digraph<N,E>::vertex_descriptor,int>& widths, int nodesep, const po::digraph<N,E>& g)
	{
		using vx_desc = typename po::digraph<N,E>::vertex_descriptor;
		using eg_desc = typename po::digraph<N,E>::edge_descriptor;
		using virt_vx = typename boost::optional<vx_desc>;
		using virt_graph = typename po::digraph<virt_vx,std::pair<int,int>>; // omega,delta
		using color_pm_type = boost::associative_property_map<std::unordered_map<vx_desc,boost::default_color_type>>;

		virt_graph h;
		std::unordered_map<vx_desc,boost::default_color_type> color;
		place_dag_visitor<N,E> visitor(&h,&layout);
		bool found_root = false;

		for(auto r: iters(vertices(g)))
		{
			if(in_degree(r,g) == 0)
			{
				boost::depth_first_search(g,visitor,color_pm_type(color),r);
				found_root = true;
			}
		}

		if(!found_root)
			boost::depth_first_search(g,visitor,color_pm_type(color),*vertices(g).first);

		for(auto v: iters(vertices(g)))
		{
			auto p = vertices(g);
			auto n = std::find_if(p.first,p.second,[&](vx_desc w)
				{	return std::get<0>(layout.at(v)) == std::get<0>(layout.at(w)) && std::get<2>(layout.at(v)) >/*+ 1 ==*/ std::get<2>(layout.at(w)); });

			if(n != p.second)
			{
				insert_edge(std::make_pair(0,nodesep),visitor.mapping->at(v),visitor.mapping->at(*n),h);
			}
		}

		std::list<typename virt_graph::vertex_descriptor> sources, sinks;

		for(auto vx: iters(vertices(h)))
		{
			int o = out_degree(vx,h);
			int i = in_degree(vx,h);

			if(o == 0 && i > 0)
				sinks.push_back(vx);
			else if(o > 0 && i == 0)
				sources.push_back(vx);
		}

		if(sources.empty())
			sources.push_back(*vertices(h).first);

		typename virt_graph::vertex_descriptor source, sink;

		// ensure single source node in h
		if(sources.size() == 1)
		{
			source = sources.front();
		}
		else
		{
			source = insert_vertex(virt_vx(boost::none),h);
			for(auto v: sources)
			{
				insert_edge(std::make_pair(dummy_edge_omega,dummy_edge_delta),source,v,h);
			}
		}

		// ensure single sink node in h
		if(sinks.size() == 0)
		{
			sink = *(vertices(h).first + 1);
		}
		else if(sinks.size() == 1)
		{
			sink = sinks.front();
		}
		else
		{
			sink = insert_vertex(virt_vx(boost::none),h);
			for(auto v: sinks)
			{
				insert_edge(std::make_pair(dummy_edge_omega,dummy_edge_delta),v,sink,h);
			}
		}

		return h;
	}
}
