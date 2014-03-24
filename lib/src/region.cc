#include <numeric>
#include <iomanip>

#include <boost/iterator/counting_iterator.hpp>
#include <boost/iterator/zip_iterator.hpp>
#include <boost/iterator/transform_iterator.hpp>
#include <boost/tuple/tuple.hpp>
#include <boost/graph/dijkstra_shortest_paths.hpp>
#include <boost/range/join.hpp>
#include <boost/range/adaptors.hpp>

#include <panopticon/region.hh>

using namespace po;
using namespace std;
using namespace boost;

template<>
rdf::statements po::marshal(const layer* l, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::ns_local(to_string(u));

	if(get<size_t>(&l->_data))
	{
		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Sparse-Undefined"));
		ret.emplace_back(root,rdf::ns_po("size"),rdf::lit(get<size_t>(l->_data)));
	}
	else if(get<vector<byte>>(&l->_data))
	{
		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Dense-Defined"));

		// XXX: should be save in a seperate file
		stringstream ss;
		for(auto i: get<vector<byte>>(l->_data))
			ss << hex << setw(2) << setfill('0') << static_cast<unsigned int>(i);

		cout << "write: " << ss.str() << endl;
		ret.emplace_back(root,rdf::ns_po("data"),rdf::lit(ss.str()));
	}
	else if(get<std::unordered_map<offset,tryte>>(&l->_data))
	{
		ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Sparse-Defined"));

		// XXX: should be save in a seperate file
		stringstream ss;
		for(auto p: get<std::unordered_map<offset,tryte>>(l->_data))
		{
			ss << p.first << "-";
			if(p.second)
				ss << hex << setw(2) << setfill('0') << static_cast<unsigned int>(*p.second);
			else
				ss << "u";
			ss << " ";
		}

		cout << "write: " << ss.str() << endl;
		ret.emplace_back(root,rdf::ns_po("data"),rdf::lit(ss.str()));
	}
	else
		throw marshal_exception("unknown layer type");

	ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(l->name()));

	return ret;
}

template<>
layer* po::unmarshal(const uuid& u, const rdf::storage& st)
{
	rdf::node root = rdf::ns_local(to_string(u));
	rdf::node type = st.first(root,rdf::ns_rdf("type")).object;
	rdf::node name = st.first(root,rdf::ns_po("name")).object;

	if(type == rdf::ns_po("Sparse-Undefined"))
	{
		rdf::node size = st.first(root,rdf::ns_po("size")).object;
		return new layer(name.as_literal(),static_cast<size_t>(stoull(size.as_literal())));
	}
	else if(type == rdf::ns_po("Dense-Defined"))
	{
		string data = st.first(root,rdf::ns_po("data")).object.as_literal();
		vector<byte> vec;

		auto i = data.begin();

		while(i != data.end() && std::next(i) != data.end())
		{
			vec.push_back(static_cast<byte>(stoul(string(i,i+2))));
			advance(i,2);
		}

		return new layer(name.as_literal(),vec);
	}
	else if(type == rdf::ns_po("Sparse-Defined"))
	{
		string data = st.first(root,rdf::ns_po("data")).object.as_literal();
		std::unordered_map<offset,tryte> kv;
		auto i = data.begin();

		while(i != data.end())
		{
			auto j = find(i,data.end(),'-');
			if(j == data.end())
				break;

			auto k = find(j,data.end(),' ');
			if(k == data.end())
				break;

			offset off = stoul(string(i,j));
			tryte t = (*(k-1) == 'u' ? boost::none : make_optional(static_cast<byte>(stoul(string(j+1,k),nullptr,16))));

			kv.emplace(off,t);
			i = k + 1;
		}

		return new layer(name.as_literal(),kv);
	}
	else
		throw marshal_exception("unknown layer type \"" + type.as_literal() + "\"");
}

std::ostream& std::operator<<(std::ostream& os, const po::bound& b)
{
	if(icl::size(b))
		os << "[" << b.lower() << ", " << b.upper() << ")";
	else
		os << "[]";

	return os;
}

po::layer_wloc po::operator+=(po::layer_wloc& a, const po::layer_wloc &b)
{
	return a = b;
}

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

bool layer::operator==(const layer& l) const
{
	return name() == l.name() &&
				 _data == l._data;
}

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
:	_base(r),
	_stack(),
	_name(n),
	_size(boost::size(r->filter(slab()))),
	_projection(none)
{}

void region::add(const bound &_b, layer_loc l)
{
	bound b = _b & bound(0,_size);

	_stack.emplace_back(b,l);
	_projection = none;
}

const std::list<std::pair<bound,layer_wloc>>& region::flatten(void) const
{
	if(!_projection)
	{
		icl::interval_map<offset,layer_wloc> proj;
		bound world(0,_size);

		for(auto i: _stack)
		{
			assert(icl::contains(world,i.first));
			auto iv = icl::discrete_interval<offset>::right_open(i.first.lower(),i.first.upper());

			proj += make_pair(iv,layer_wloc(i.second));
		}

		_projection = list<pair<bound,layer_wloc>>();
		for(auto i: proj)
			_projection->emplace_back(bound(i.first.lower(),i.first.upper()),i.second);
	}

	return *_projection;
}

const list<pair<po::bound,layer_loc>>& region::stack(void) const { return _stack; }
const std::string& region::name(void) const { return _name; }
size_t region::size(void) const { return _size; }

po::slab region::read(void) const
{
	slab ret = _base->filter(slab());

	for(auto i: _stack)
	{
		slab n;

		if(i.first.lower())
			n = slab(boost::begin(ret),next(boost::begin(ret),i.first.lower()));

		slab src(next(boost::begin(ret),i.first.lower()),next(boost::begin(ret),i.first.upper()));
		slab filtered = i.second->filter(src);

		n = join(n,filtered);

		if(i.first.upper() < boost::size(ret))
			n = join(n,slab(next(boost::begin(ret),i.first.upper()),boost::end(ret)));

		ret = n;
	}

	return ret;
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
			std::list<vertex_descriptor> path;

			path.push_back(i);
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
		std::list<graph_traits<regions>::edge_descriptor> es;

		for(graph_traits<regions>::edge_descriptor a: iters(p))
			es.push_back(a);

		es.sort([&](const graph_traits<regions>::edge_descriptor a, const graph_traits<regions>::edge_descriptor b)
			{ return get_edge(a,regs).lower() < get_edge(b,regs).lower(); });

		for(auto e: es)
		{
			bound b = get_edge(e,regs);
			auto nx = target(e,regs);
			bound free(last,b.lower());

			if(last < b.lower())
				ret.emplace_back(free,r);
			last = b.upper();

			if(visited.insert(nx).second)
				step(nx);
		}

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
