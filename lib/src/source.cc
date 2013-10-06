#include <source.hh>

#include <boost/graph/visitors.hpp>
#include <boost/graph/adjacency_list.hpp>
#include <boost/graph/breadth_first_search.hpp>
#include <boost/property_map/property_map.hpp>
#include <boost/graph/graph_utility.hpp>
#include <boost/graph/reverse_graph.hpp>
#include <boost/graph/dijkstra_shortest_paths.hpp>

#include <map>

namespace po
{
	address_space &operator+=(address_space &me, const address_space &as)
	{
		return me = as;
	}
}

std::list<std::pair<po::rrange,po::address_space>> po::projection(const po::address_space &root, const po::graph<po::address_space,po::rrange> &g)
{
	using vertex_descriptor = boost::graph_traits<graph<address_space,rrange>>::vertex_descriptor;
	using edge_descriptor = boost::graph_traits<graph<address_space,rrange>>::edge_descriptor;
	std::list<std::pair<rrange,address_space>> ret;
	std::function<void(vertex_descriptor)> step;
	std::unordered_set<vertex_descriptor> visited;

	step = [&](vertex_descriptor v)
	{
		const address_space &as = g.get_node(v);
		boost::icl::split_interval_map<typename rrange::domain_type,address_space> local;
		auto p = in_edges(v,g);

		local.add(std::make_pair(as.area,as));
		assert(visited.insert(v).second);

		std::for_each(p.first,p.second,[&](edge_descriptor e)
		{
			rrange r = g.get_edge(e);
			const address_space &other = g.get_node(source(e,g));

			local += std::make_pair(r,other);
		});

		for(const std::pair<rrange,address_space> &q: local)
		{
			if(q.second == as)
			{
				ret.push_back(std::make_pair(q.first,as));
			}
			else
			{
				auto u = g.find_node(q.second);
				if(u != g.nodes().second && !visited.count(*u))
					step(*u);
			}
		}
	};

	step(*g.find_node(root));

	assert(visited.size() == g.num_nodes());
	return ret;
}

boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor po::root(const po::graph<po::address_space,po::rrange> &g)
{
	auto p = g.nodes();
	auto i = p.first;

	while(i != p.second)
		if(!out_degree(*i,g))
			return *i;
		else
			++i;

	throw std::runtime_error("no root node");
}

po::unordered_pmap<boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor,boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor>
po::tree(const po::graph<po::address_space,po::rrange> &g)
{
	using vertex_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor;
	using edge_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::edge_descriptor;

	auto r = root(g);
	std::unordered_map<edge_descriptor,int> w_map;
	boost::associative_property_map<std::unordered_map<edge_descriptor,int>> weight_adaptor(w_map);
	auto common_parent = [&](vertex_descriptor v, vertex_descriptor u)
	{
		auto find_path = [&](vertex_descriptor x)
		{
			std::unordered_map<vertex_descriptor,vertex_descriptor> p_map;
			boost::associative_property_map<std::unordered_map<vertex_descriptor,vertex_descriptor>> pred_adaptor(p_map);

			boost::dijkstra_shortest_paths(g,x,boost::weight_map(weight_adaptor).predecessor_map(pred_adaptor));

			auto i = r;
			std::list<vertex_descriptor> path({i});
			while(i != p_map[i])
			{
				i = p_map[i];
				path.push_back(i);
			}
			return path;
		};

		auto l1 =	find_path(v);
		auto l2 = find_path(u);

		return *std::find_first_of(l1.begin(),l1.end(),l2.begin(),l2.end());
	};
	unordered_pmap<vertex_descriptor,vertex_descriptor> ret;

	for(auto v: iters(g.edges()))
		put(weight_adaptor,v,1);

	/*
	 * for(n: nodes(G))
	 * 	 for(e: in_edges(n))
	 *     c = source(e)
	 *     if(!in_tree(c))
	 *       add_to_tree(n,c)
	 *     else
	 *       del_from_tree(c)
	 *       add_to_tree(common_parent(n,c),c)
	 */
	auto revgraph = boost::make_reverse_graph(g);
	boost::breadth_first_search(revgraph,r,boost::visitor(boost::make_bfs_visitor(make_lambda_visitor(
		std::function<void(vertex_descriptor v)>([&](vertex_descriptor v)
		{
			for(auto e: iters(g.in_edges(v)))
			{
				auto c = source(e,g);
				if(ret.count(c) == 0)
					ret[c] = v;
				else
					ret[c] = common_parent(ret.at(c),v);
			}
		}),revgraph,boost::on_discover_vertex()))));

	return ret;
}

/*
bytes po::read(const address_space &as, const range<addr_t> &a, const graph<address_space,range<addr_t>> &g)
{
	bytes ret;
	auto out = out_edges(*g.find_node(as),g);
	auto is = std::back_inserter(ret);

	std::for_each(out.first,out.second,[&](typename graph<address_space,range<addr_t>>::edge_descriptor ei)
	{
		const address_space &tgt = g.get_node(target(ei,g));
		const range<addr_t> &sel = g.get_edge(ei);
		bytes b = read(tgt,sel,g);

		std::copy(b.begin(),b.end(),is);
	});

	return as.map(ret,a);
}

int main(int argc, char *argv[])
{
	address_space a1("source",[](const bytes&) { return bytes({0,1,2,3,4,5,6,7,8,9}); });
	address_space a2("xor 0x55",[](const bytes &bs)
	{
		bytes ret;
		std::transform(bs.begin(),bs.end(),std::inserter(ret,ret.begin()),std::bind(std::bit_xor<uint8_t>(),std::placeholders::_1,0x55));

		return ret;
	});

	graph<address_space,range<addr_t>> filters;
	auto proj = projection(filters);

	for(const std::pair<address_space const,range<addr_t> const> &p: proj)
	{
		std::cout << p.first.name << std::endl;
		for(const uint8_t &b: read(p.first,p.second,filters))
			std::cout << b << " ";
		std::cout << std::endl;
	}

	return 0;
}*/
