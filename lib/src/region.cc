#include <numeric>

#include <boost/iterator/counting_iterator.hpp>
#include <boost/iterator/zip_iterator.hpp>
#include <boost/iterator/transform_iterator.hpp>
#include <boost/tuple/tuple.hpp>
#include <boost/graph/dijkstra_shortest_paths.hpp>
#include <boost/range/join.hpp>

#include <panopticon/region.hh>

using namespace po;
using namespace std;
using namespace boost;

template<>
rdf::statements po::marshal(const layer*, const uuid&) { return rdf::statements(); }

template<>
layer* po::unmarshal(const uuid&, const rdf::storage&) { return nullptr; }

std::ostream& std::operator<<(std::ostream& os, const po::bound& b)
{
	os << "[" << boost::icl::first(b) << ", " << boost::icl::last(b) << ")";
	return os;
}

po::layer_wloc po::operator+=(po::layer_wloc& a, const po::layer_wloc &b)
{
	return a = b;
}

layer::layer(const string &n, function<po::tryte(po::tryte)> fn)
: _name(n), _data(fn)
{}

layer::layer(const std::string &n, std::initializer_list<byte> il)
: _name(n), _data(std::move(vector<byte>(il)))
{}

layer::layer(const std::string &n, const std::vector<byte> &d)
: _name(n), _data(d)
{}

layer::layer(const std::string &n, const byte *d, size_t sz)
: _name(n), _data(std::move(std::vector<byte>(d,d + sz)))
{}

layer::layer(const std::string &n, const std::unordered_map<offset,tryte> &d)
: _name(n), _data(d)
{}

layer::layer(const std::string &n, offset sz)
: _name(n), _data(sz)
{}

void layer::write(offset pos, tryte t)
{
	try
	{
		boost::get<std::unordered_map<offset,tryte>>(_data)[pos] = t;
	}
	catch(const boost::bad_visit&)
	{
		throw std::invalid_argument("no mutable layer");
	}
}

slab layer::filter(const slab& in) const
{
	return boost::apply_visitor(filter_visitor(in),_data);
}

layer::filter_visitor::filter_visitor(slab s) : static_visitor(), in(s) {}

slab layer::filter_visitor::operator()(std::function<tryte(tryte)> fn) const
{
	return adaptors::transform(in,fn);
}

slab layer::filter_visitor::operator()(const std::vector<byte>& d) const
{
	return slab(d.cbegin(),d.cend());
}

slab layer::filter_visitor::operator()(const std::unordered_map<offset,tryte>& data) const
{
	using func = std::function<po::tryte(const boost::tuples::tuple<offset,po::tryte> &)>;
	slab::const_iterator sb = boost::begin(in), se = boost::end(in);
	func fn = [data](const boost::tuples::tuple<offset,po::tryte> &p) { return data.count(boost::get<0>(p)) ? data.at(boost::get<0>(p)) : boost::get<1>(p); };
	auto b = make_zip_iterator(boost::make_tuple(counting_iterator<offset,boost::random_access_traversal_tag>(0),sb));
	auto e = make_zip_iterator(boost::make_tuple(counting_iterator<offset,boost::random_access_traversal_tag>(size(in)),se));
	using transform_iter = boost::transform_iterator<func,decltype(b)>;

	return slab(transform_iter(b,fn),transform_iter(e,fn));
}

slab layer::filter_visitor::operator()(size_t sz) const
{
	using func = std::function<po::tryte(int)>;
	func fn = [](int) { return boost::none; };
	counting_iterator<offset,boost::random_access_traversal_tag> a(0);
	counting_iterator<offset,boost::random_access_traversal_tag> b(sz);
	using transform_iter = boost::transform_iterator<func,decltype(b)>;

	return slab(transform_iter(a,fn),transform_iter(b,fn));
}

const string& layer::name(void) const
{
	return _name;
}

po::region_loc po::region::undefined(const std::string& n, size_t sz)
{
	return region_loc(new region(n,layer_loc(new layer("base",sz))));
}

po::region_loc po::region::wrap(const std::string& n, const byte* p, size_t sz)
{
	return region_loc(new region(n,layer_loc(new layer("base",p,sz))));
}

po::region_loc po::region::wrap(const std::string& n, std::initializer_list<byte> il)
{
	return region_loc(new region(n,layer_loc(new layer("base",il))));
}

region::region(const std::string &n, layer_loc r)
: _graph(),
	_root(insert_node(r,_graph)),
	_name(n),
	_size(boost::size(r->filter(slab()))),
	_projection(none)
{}

void region::add(const bound &_b, layer_loc l)
{
	bound b = bound(0,_size) & _b;
	auto proj = projection();
	auto i = proj.find(b);
	auto vx = insert_node(l,_graph);
	boost::optional<offset> last = none;

	if(i == proj.end())
	{
		insert_edge(b,_root,vx,_graph);
	}
	else
	{
		while(i != proj.end() && icl::size(i->first & b))
		{
			bound n = i->first & b;
			auto j = find_node(i->second.lock(),_graph);
			insert_edge(n,j,vx,_graph);

			if(last && *last + 1 != icl::first(n))
			{
				bound m(*last + 1,icl::first(n));
				insert_edge(m,_root,vx,_graph);
			}
			last = icl::last(n);

			++i;
		}

		if(*last != icl::last(b))
			insert_edge(bound(*last + 1,icl::last(b) + 1),_root,vx,_graph);
	}

	_projection = none;
}

const region::image& region::projection(void) const
{
	if(!_projection)
	{
		using vertex_descriptor = boost::graph_traits<digraph<layer_loc,bound>>::vertex_descriptor;
		using edge_descriptor = boost::graph_traits<digraph<layer_loc,bound>>::edge_descriptor;
		std::function<void(vertex_descriptor)> step;
		std::unordered_set<vertex_descriptor> visited;

		_projection = icl::interval_map<offset,layer_wloc>();
		*_projection += make_pair(bound(0,_size),layer_wloc(get_node(_root,_graph)));

		step = [&](vertex_descriptor v)
		{
			layer_loc as = get_node(v,_graph);
			auto p = out_edges(v,_graph);

			assert(visited.insert(v).second);

			std::for_each(p.first,p.second,[&](edge_descriptor e)
			{
				bound b = get_edge(e,_graph);
				layer_loc other = get_node(target(e,_graph),_graph);

				*_projection += make_pair(b,layer_wloc(other));
			});

			std::for_each(p.first,p.second,[&](edge_descriptor e)
			{
				auto u = target(e,_graph);

				if(u != *vertices(_graph).second && !visited.count(u))
					step(u);
			});
		};

		step(_root);
		assert(visited.size() == num_vertices(_graph));
	}

	return *_projection;
}

const region::layers& region::graph(void) const { return _graph; }
const std::string& region::name(void) const { return _name; }
size_t region::size(void) const { return _size; }

po::slab region::read(boost::optional<po::layer_loc> l) const
{
	std::list<std::pair<bound,layer_wloc>> src;
	slab ret;

	if(l)
	{
		auto vx = find_node(*l,_graph);
		auto p = out_edges(vx,_graph);
		auto i = p.first;

		if(i == p.second)
		{
			return get_node(vx,_graph)->filter(slab());
		}
		else
		{
			while(i != p.second)
			{
				src.emplace_back(get_edge(*i,_graph),layer_wloc(get_node(target(*i,_graph),_graph)));
				++i;
			}
		}
	}
	else
	{
		for(auto r: projection())
			src.emplace_back(r);
	}

	src.sort([&](const std::pair<bound,layer_wloc> &a, const std::pair<bound,layer_wloc> &b) { return icl::first(a.first) < icl::first(b.first); });

	return std::accumulate(src.begin(),src.end(),slab(),[&](slab acc, const pair<bound,layer_wloc>& s)
	{
		slab all = read(s.second.lock());
		return boost::range::join(acc,slab(std::next(boost::begin(all),icl::first(s.first)),
																			 std::next(boost::begin(all),icl::upper(s.first))));
	});
}

std::unordered_map<region_wloc,region_wloc> po::spanning_tree(const regions &regs)
{
	using vertex_descriptor = boost::graph_traits<digraph<po::region_loc,po::bound>>::vertex_descriptor;
	using edge_descriptor = boost::graph_traits<digraph<po::region_loc,po::bound>>::edge_descriptor;

	auto r = root(regs);
	std::map<edge_descriptor,int> w_map;
	boost::associative_property_map<std::map<edge_descriptor,int>> weight_adaptor(w_map);
	auto common_parent = [&](vertex_descriptor v, vertex_descriptor u)
	{
		auto find_path = [&](vertex_descriptor x)
		{
			std::map<vertex_descriptor,vertex_descriptor> p_map;
			boost::associative_property_map<std::map<vertex_descriptor,vertex_descriptor>> pred_adaptor(p_map);

			boost::dijkstra_shortest_paths(regs,x,boost::weight_map(weight_adaptor).predecessor_map(pred_adaptor));

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
	map<vertex_descriptor,vertex_descriptor> ret;

	for(auto v: iters(edges(regs)))
		put(weight_adaptor,v,1);

	/*
	 * for(n: nodes(G))
	 * 	 for(e: out_edges(n))
	 *     c = target(e)
	 *     if(!in_tree(c))
	 *       add_to_tree(n,c)
	 *     else
	 *       del_from_tree(c)
	 *       add_to_tree(common_parent(n,c),c)
	 */
	boost::breadth_first_search(regs,r,boost::visitor(boost::make_bfs_visitor(make_lambda_visitor(
		std::function<void(vertex_descriptor v)>([&](vertex_descriptor v)
		{
			for(auto e: iters(out_edges(v,regs)))
			{
				auto c = target(e,regs);
				if(ret.count(c) == 0)
					ret[c] = v;
				else
					ret[c] = common_parent(ret.at(c),v);
			}
		}),regs,boost::on_discover_vertex()))));

	std::unordered_map<region_wloc,region_wloc> out;

	for(const pair<vertex_descriptor,vertex_descriptor> &p: ret)
		out.emplace(region_wloc(get_node(p.first,regs)),region_wloc(get_node(p.second,regs)));

	return out;
}

std::list<std::pair<bound,region_wloc>> po::projection(const regions &regs)
{
	std::list<std::pair<bound,region_wloc>> ret;
	std::function<void(graph_traits<regions>::vertex_descriptor)> step;
	std::set<graph_traits<regions>::vertex_descriptor> visited;

	step = [&](graph_traits<regions>::vertex_descriptor vx)
	{
		region_loc r = get_node(vx,regs);
		auto p = out_edges(vx,regs);
		offset last = 0;

		std::for_each(p.first,p.second,[&](const graph_traits<regions>::edge_descriptor e)
		{
			bound b = get_edge(e,regs);
			auto nx = target(e,regs);
			bound free(last,b.lower());

			if(last < b.lower())
				ret.emplace_back(free,r);
			last = b.upper();

			if(visited.insert(nx).second)
				step(nx);
		});

		if(last < r->size())
		{
			bound free(last,r->size());
			ret.emplace_back(free,r);
		}
	};

	step(root(regs));
	return ret;
}

template<>
rdf::statements po::marshal(const region*, const uuid&) { return rdf::statements(); }

template<>
region* po::unmarshal(const uuid&, const rdf::storage&) { return nullptr; }
