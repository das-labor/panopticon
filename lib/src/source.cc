#include <source.hh>

#include <map>

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
		boost::icl::split_interval_map<typename rrange::domain_type,int> local;
		std::multimap<typename rrange::domain_type,vertex_descriptor> next;
		auto p = in_edges(v,g);

		local.add(std::make_pair(as.area,1));
		std::for_each(p.first,p.second,[&](edge_descriptor e)
		{
			rrange r = g.get_edge(e);
			local += std::make_pair(r,1);
			next.insert(std::make_pair(first(r),source(e,g)));
		});

		for(const std::pair<rrange,int> &q: local)
		{
			//std::cerr << q.first << " => " << q.second << std::endl;
			if(q.second == 1)
				ret.push_back(std::make_pair(q.first,as));
		}

		visited.insert(v);

		for(const auto &q: next)
			if(!visited.count(q.second))
				step(q.second);
	};

	step(*g.find_node(root));

	assert(visited.size() == g.num_nodes());
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
