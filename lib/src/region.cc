#include <boost/iterator/counting_iterator.hpp>
#include <boost/iterator/zip_iterator.hpp>
#include <boost/iterator/transform_iterator.hpp>
#include <boost/tuple/tuple.hpp>
#include <boost/graph/dijkstra_shortest_paths.hpp>
#include <boost/graph/reverse_graph.hpp>
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

map_layer::map_layer(const string &n, function<po::tryte(po::tryte)> fn)
: _name(n), _operation(fn)
{}

bool map_layer::operator==(const map_layer &a) const
{
	return a._name == _name;
}

slab map_layer::filter(const slab& in) const
{
	return adaptors::transform(in,adaptor(this));
}

const string& map_layer::name(void) const
{
	return _name;
}

map_layer::adaptor::adaptor(const map_layer *p) : parent(p) {}
po::tryte map_layer::adaptor::operator()(po::tryte i) const { return parent->_operation(i); }

anonymous_layer::anonymous_layer(std::initializer_list<po::tryte> il, const std::string &n) : data(il), _name(n) {}
anonymous_layer::anonymous_layer(offset sz, const std::string &n) : data(sz), _name(n) {}

bool anonymous_layer::operator==(const anonymous_layer &a) const { return a.name() == name() && a.data == data; }

slab anonymous_layer::filter(const slab&) const { return slab(data.cbegin(),data.cend()); }
const std::string& anonymous_layer::name(void) const { return _name; }

mutable_layer::mutable_layer(const std::string &n) : data(), _name(n) {}

slab mutable_layer::filter(const slab& in) const
{
	using func = std::function<po::tryte(const boost::tuples::tuple<offset,po::tryte> &)>;
	slab::const_iterator sb = boost::begin(in), se = boost::end(in);
	func fn = [this](const boost::tuples::tuple<offset,po::tryte> &p) { return data.count(boost::get<0>(p)) ? data.at(boost::get<0>(p)) : boost::get<1>(p); };
	auto b = make_zip_iterator(boost::make_tuple(counting_iterator<offset,boost::random_access_traversal_tag>(0),sb));
	auto e = make_zip_iterator(boost::make_tuple(counting_iterator<offset,boost::random_access_traversal_tag>(size(in)),se));
	using transform_iter = boost::transform_iterator<func,decltype(b)>;

	return slab(transform_iter(b,fn),transform_iter(e,fn));
}

const std::string& mutable_layer::name(void) const { return _name; }

po::slab po::filter(const po::layer &a, const po::slab &s)
{
	if(boost::get<po::map_layer>(&a))
		return boost::get<po::map_layer>(a).filter(s);
	if(boost::get<po::mutable_layer>(&a))
		return boost::get<po::mutable_layer>(a).filter(s);
	if(boost::get<po::anonymous_layer>(&a))
		return boost::get<po::anonymous_layer>(a).filter(s);
	else
		throw invalid_argument("unknown layer type");
}

std::string po::name(const po::layer &a)
{
	if(boost::get<po::map_layer>(&a))
		return boost::get<po::map_layer>(a).name();
	if(boost::get<po::mutable_layer>(&a))
		return boost::get<po::mutable_layer>(a).name();
	if(boost::get<po::anonymous_layer>(&a))
		return boost::get<po::anonymous_layer>(a).name();
	else
		throw invalid_argument("unknown layer type");
}

region::region(const std::string &n, size_t sz)
: _graph(),
	_root(_graph.insert_node(layer_loc(uuids::random_generator()(),new layer(anonymous_layer({},"root"))))),
	_name(n),
	_size(sz),
	_projection(none)
{}

void region::add(const bound &_b, layer_loc l)
{
	bound b = bound(0,_size) & _b;
	auto proj = projection();
	auto i = proj.find(b);
	auto vx = _graph.insert_node(l);
	boost::optional<offset> last = none;

	if(i == proj.end())
	{
		_graph.insert_edge(b,vx,_root);
	}
	else
	{
		while(i != proj.end() && icl::size(i->first & b))
		{
			bound n = i->first & b;
			_graph.insert_edge(n,vx,*_graph.find_node(i->second.lock()));

			if(last && *last + 1 != icl::first(n))
			{
				bound m(*last + 1,icl::first(n));
				_graph.insert_edge(m,vx,_root);
			}
			last = icl::last(n);

			++i;
		}

		if(*last != icl::last(b))
			_graph.insert_edge(bound(*last + 1,icl::last(b) + 1),vx,_root);
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
		*_projection += make_pair(bound(0,_size),layer_wloc(_graph.get_node(_root)));

		step = [&](vertex_descriptor v)
		{
			layer_loc as = _graph.get_node(v);
			auto p = in_edges(v,_graph);

			assert(visited.insert(v).second);

			std::for_each(p.first,p.second,[&](edge_descriptor e)
			{
				bound b = _graph.get_edge(e);
				layer_loc other = _graph.get_node(source(e,_graph));

				*_projection += make_pair(b,layer_wloc(other));
			});

			std::for_each(p.first,p.second,[&](edge_descriptor e)
			{
				auto u = source(e,_graph);

				if(u != *_graph.nodes().second && !visited.count(u))
					step(u);
			});
		};

		step(_root);
		assert(visited.size() == _graph.num_nodes());
	}

	return *_projection;
}

const region::layers& region::graph(void) const { return _graph; }
const std::string& region::name(void) const { return _name; }
size_t region::size(void) const { return _size; }

po::slab region::read(void) const
{
	const image &img = projection();
	po::slab ret;

	for(auto r: img)
		ret = boost::range::join(ret,read(r.second.lock()));

	return ret;
}

po::slab region::read(po::layer_loc l) const
{
	auto vx = _graph.find_node(l);
	auto p = _graph.out_edges(*vx);
	auto i = p.first;
	std::list<std::pair<bound,layer_wloc>> src;
	slab ret;

	while(i != p.second)
	{
		src.emplace_back(_graph.get_edge(*i),layer_wloc(_graph.get_node(_graph.target(*i))));
		++i;
	}

	src.sort([&](const std::pair<bound,layer_wloc> &a, const std::pair<bound,layer_wloc> &b) { return icl::first(a.first) < icl::first(b.first); });

	for(auto s: src)
	{
		slab all = read(s.second.lock());
		ret = boost::range::join(ret,slab(std::next(boost::begin(all),icl::first(s.first)),
																		std::next(boost::begin(all),icl::upper(s.first))));
	}

	return ret;
}

std::unordered_map<region_wloc,region_wloc> po::spanning_tree(const regions &regs)
{
	using vertex_descriptor = typename boost::graph_traits<digraph<po::region_loc,po::bound>>::vertex_descriptor;
	using edge_descriptor = typename boost::graph_traits<digraph<po::region_loc,po::bound>>::edge_descriptor;

	auto r = root(make_reverse_graph(regs));
	std::unordered_map<edge_descriptor,int> w_map;
	boost::associative_property_map<std::unordered_map<edge_descriptor,int>> weight_adaptor(w_map);
	auto common_parent = [&](vertex_descriptor v, vertex_descriptor u)
	{
		auto find_path = [&](vertex_descriptor x)
		{
			std::unordered_map<vertex_descriptor,vertex_descriptor> p_map;
			boost::associative_property_map<std::unordered_map<vertex_descriptor,vertex_descriptor>> pred_adaptor(p_map);

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
	unordered_pmap<vertex_descriptor,vertex_descriptor> ret;

	for(auto v: iters(regs.edges()))
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
	auto revgraph = boost::make_reverse_graph(regs);
	boost::breadth_first_search(revgraph,r,boost::visitor(boost::make_bfs_visitor(make_lambda_visitor(
		std::function<void(vertex_descriptor v)>([&](vertex_descriptor v)
		{
			for(auto e: iters(regs.in_edges(v)))
			{
				auto c = source(e,regs);
				if(ret.count(c) == 0)
					ret[c] = v;
				else
					ret[c] = common_parent(ret.at(c),v);
			}
		}),revgraph,boost::on_discover_vertex()))));

	std::unordered_map<region_wloc,region_wloc> out;

	for(const pair<vertex_descriptor,vertex_descriptor> &p: ret)
		out.emplace(region_wloc(regs.get_node(p.first)),region_wloc(regs.get_node(p.second)));

	return out;
}

std::list<std::pair<bound,region_wloc>> po::projection(const regions &regs)
{
	std::list<std::pair<bound,region_wloc>> ret;
	std::function<void(graph_traits<regions>::vertex_descriptor)> step;
	std::unordered_set<graph_traits<regions>::vertex_descriptor> visited;

	step = [&](graph_traits<regions>::vertex_descriptor vx)
	{
		region_loc r = regs.get_node(vx);
		auto p = regs.in_edges(vx);
		offset last = 0;

		std::for_each(p.first,p.second,[&](const graph_traits<regions>::edge_descriptor e)
		{
			bound b = regs.get_edge(e);
			auto nx = regs.source(e);
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

	step(root(make_reverse_graph(regs)));
	return ret;
}

template<>
rdf::statements po::marshal(const region*, const uuid&) { return rdf::statements(); }

template<>
region* po::unmarshal(const uuid&, const rdf::storage&) { return nullptr; }
