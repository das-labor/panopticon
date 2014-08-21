#include <numeric>
#include <iomanip>

#include <boost/tuple/tuple.hpp>
#include <boost/graph/dijkstra_shortest_paths.hpp>

#include <panopticon/region.hh>

using namespace po;
using namespace std;
using namespace boost;

slab::slab(std::unordered_map<offset,tryte> const* m, slab sl)
: _sources(), _cache(boost::none)
{
	for(auto x: sl._sources)
	{
		std::list<std::pair<std::unordered_map<offset,tryte> const*,offset>> lst(std::get<1>(x));

		lst.emplace_back(m,0);
		_sources.push_back(source(std::get<0>(x),lst));
	}
}

slab::slab(iterator i,iterator e) : _sources(), _cache(boost::none)
{
	ensure(i._base == e._base);
	offset o = 0;

	for(auto s: i._base->_sources)
	{
		if(boost::get<std::pair<unsigned char const*,offset>>(&std::get<0>(s)))
		{
			const std::pair<unsigned char const*,offset>& p = boost::get<std::pair<unsigned char const*,offset>>(std::get<0>(s));

			if(i._off < o + p.second && e._off > o)
			{
				offset b = i._off < o ? 0 : std::max<offset>(0,i._off - o);
				offset f = std::min<offset>(p.second,e._off - o);

				if(b)
				{
					std::list<std::pair<std::unordered_map<offset,tryte> const*,offset>> lst;
					std::transform(std::get<1>(s).begin(),std::get<1>(s).end(),std::back_inserter(lst),[&](std::pair<std::unordered_map<offset,tryte> const*,offset>& p)
						{ return std::make_pair(p.first,p.second + b); });

					_sources.push_back(source(std::make_pair(p.first + b,f - b),lst));
				}
				else
					_sources.push_back(source(std::make_pair(p.first + b,f - b),std::get<1>(s)));
			}

			o += p.second;
		}
		else if(boost::get<offset>(&std::get<0>(s)))
		{
			offset u = boost::get<offset>(std::get<0>(s));

			if(i._off < o + u && e._off > o)
			{
				offset b = i._off < o ? 0 : std::max<offset>(0,i._off - o);
				offset f = std::min<offset>(u,e._off - o);

				if(b)
				{
					std::list<std::pair<std::unordered_map<offset,tryte> const*,offset>> lst;
					std::transform(std::get<1>(s).begin(),std::get<1>(s).end(),std::back_inserter(lst),[&](std::pair<std::unordered_map<offset,tryte> const*,offset>& p)
						{ return std::make_pair(p.first,p.second + b); });

					_sources.push_back(source(f - b,lst));
				}
				else
					_sources.push_back(source(f - b,std::get<1>(s)));
			}

			o += u;
		}
		else
			throw std::invalid_argument("slab: unknown variant type");
	}
}

slab po::combine(slab a,slab b)
{
	std::list<slab::source> ret;
	std::copy(a._sources.begin(),a._sources.end(),back_inserter(ret));
	std::copy(b._sources.begin(),b._sources.end(),back_inserter(ret));

	return slab(ret);
}

void slab::debug(void) const
{
	struct vis : public boost::static_visitor<>
	{
		void operator()(offset o) const
			{ std::cout << "undef-" << o; }
		void operator()(std::pair<unsigned char const*,offset>& p) const
			{ std::cout << "plain-" << (ptrdiff_t)p.first << "-" << p.second; }
	};

	std::cout << "[ ";
	for(auto x: _sources)
	{
		boost::apply_visitor(vis(),std::get<0>(x));
		std::cout << " ";
	}
	std::cout << "]" << std::endl;
}

template<>
archive po::marshal(const layer* l, const uuid& u)
{
	archive ret;
	rdf::node root = rdf::iri(u);

	struct visitor : public boost::static_visitor<>
	{
		visitor(archive& a, rdf::node n) : ret(a), root(n) {}

		void operator()(size_t sz)
		{
			ret.triples.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Sparse-Undefined"));
			ret.triples.emplace_back(root,rdf::ns_po("size"),rdf::lit(sz));
		}

		void operator()(const std::unordered_map<offset,tryte>& m)
		{
			ret.triples.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Sparse-Defined"));

			// XXX: should be save in a seperate file
			stringstream ss;
			for(auto p: m)
			{
				ss << p.first << "-";
				if(p.second)
					ss << hex << setw(2) << setfill('0') << static_cast<unsigned int>(*p.second);
				else
					ss << "u";
				ss << " ";
			}

			ret.triples.emplace_back(root,rdf::ns_po("data"),rdf::lit(ss.str()));
		}

		void operator()(const blob& mf)
		{
			ret.triples.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Blob"));
			ret.triples.emplace_back(root,rdf::ns_po("blob"),rdf::iri(mf.tag()));
			ret.blobs.emplace_back(mf);
		}

	private:
		archive& ret;
		rdf::node root;
	};

	visitor v(ret,root);

	boost::apply_visitor(v,l->_data);
	ret.triples.emplace_back(root,rdf::ns_po("name"),rdf::lit(l->name()));

	return ret;
}

template<>
layer* po::unmarshal(const uuid& u, const rdf::storage& st)
{
	rdf::node root = rdf::iri(u);
	rdf::node type = st.first(root,rdf::ns_rdf("type")).object;
	rdf::node name = st.first(root,rdf::ns_po("name")).object;

	if(type == rdf::ns_po("Sparse-Undefined"))
	{
		rdf::node size = st.first(root,rdf::ns_po("size")).object;
		return new layer(name.as_literal(),static_cast<size_t>(stoull(size.as_literal())));
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
			tryte t = (*(k-1) == 'u' ? boost::none : make_optional(static_cast<po::byte>(stoul(string(j+1,k),nullptr,16))));

			kv.emplace(off,t);
			i = k + 1;
		}

		return new layer(name.as_literal(),kv);
	}
	else if(type == rdf::ns_po("Blob"))
	{
		rdf::node b = st.first(root,rdf::ns_po("blob")).object;
		return new layer(name.as_literal(),st.fetch_blob(b.as_iri().as_uuid()));
	}
	else
		throw marshal_exception("unknown layer type \"" + type.as_iri().as_string() + "\"");
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

layer::layer(const std::string &n, std::initializer_list<po::byte> il)
: _name(n), _data(blob(std::move(vector<po::byte>(il))))
{}

layer::layer(const std::string &n, const blob& mf)
: _name(n), _data(mf)
{}

layer::layer(const std::string &n, const std::vector<po::byte> &d)
: _name(n), _data(blob(d))
{}

layer::layer(const std::string &n, const po::byte *d, size_t sz)
: _name(n), _data(blob(std::move(std::vector<po::byte>(d,d + sz))))
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

slab layer::filter_visitor::operator()(const std::unordered_map<offset,tryte>& data) const
{
	return slab(&data,in);
}

slab layer::filter_visitor::operator()(size_t sz) const
{
	return slab(sz);
}

slab layer::filter_visitor::operator()(const blob& mf) const
{
	return slab(mf.data(),mf.size());
}

const string& layer::name(void) const
{
	return _name;
}

po::region_loc po::region::mmap(const std::string& n, const boost::filesystem::path& p)
{
	return region_loc(new region(n,layer_loc(new layer("base",blob(p)))));
}

po::region_loc po::region::undefined(const std::string& n, size_t sz)
{
	return region_loc(new region(n,layer_loc(new layer("base",sz))));
}

po::region_loc po::region::wrap(const std::string& n, const po::byte* p, size_t sz)
{
	return region_loc(new region(n,layer_loc(new layer("base",p,sz))));
}

po::region_loc po::region::wrap(const std::string& n, std::initializer_list<po::byte> il)
{
	return region_loc(new region(n,layer_loc(new layer("base",il))));
}

region::region(const std::string &n, layer_loc r)
:	_base(r),
	_stack(),
	_name(n),
	_size(r->filter(slab()).size()),
	_projection(none)
{}

bool region::operator==(const region& r) const
{
	return _base == r._base &&
				 _stack == r._stack &&
				 _name == r._name &&
				 _size == r._size;
}

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

		proj += make_pair(icl::discrete_interval<offset>::right_open(0,_size),layer_wloc(_base));

		for(auto i: _stack)
		{
			ensure(icl::contains(world,i.first));
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
			n = slab(ret.begin(),ret.begin() + i.first.lower());

		slab src(ret.begin() + i.first.lower(),ret.begin() + i.first.upper());
		slab filtered = i.second->filter(src);

		n = combine(n,filtered);

		if(i.first.upper() < ret.size())
			n = combine(n,slab(ret.begin() + i.first.upper(),ret.end()));

		ret = n;
	}

	ensure(ret.size() == size());
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
			std::vector<boost::default_color_type> color_map;
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
		out.emplace(region_wloc(get_vertex(p.first,regs)),region_wloc(get_vertex(p.second,regs)));

	return out;
}

std::list<std::pair<bound,region_wloc>> po::projection(const regions &regs)
{
	std::list<std::pair<bound,region_wloc>> ret;
	std::function<void(graph_traits<regions>::vertex_descriptor)> step;
	std::set<graph_traits<regions>::vertex_descriptor> visited;

	step = [&](graph_traits<regions>::vertex_descriptor vx)
	{
		region_loc r = get_vertex(vx,regs);
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

	if(num_vertices(regs))
		step(root(regs));

	return ret;
}

template<>
archive po::marshal(const region* r, const uuid& u)
{
	rdf::statements ret;
	std::list<blob> bl;
	rdf::node root = rdf::iri(u);

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Region"));
	ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(r->name()));
	ret.emplace_back(root,rdf::ns_po("base"),rdf::lit(to_string(r->_base.tag())));
	auto m = marshal(r->_base.read(),r->_base.tag());

	std::move(m.triples.begin(),m.triples.end(),back_inserter(ret));
	std::move(m.blobs.begin(),m.blobs.end(),back_inserter(bl));
	rdf::nodes ns;

	for(auto p: r->stack())
	{
		rdf::node n = rdf::node::blank();

		ret.emplace_back(n,rdf::ns_po("bound"),rdf::lit(to_string(p.first.lower()) + ":" + to_string(p.first.upper())));
		ret.emplace_back(n,rdf::ns_po("layer"),rdf::lit(to_string(p.second.tag())));
		ns.emplace_back(n);
		auto m = marshal(p.second.read(),p.second.tag());

		std::move(m.triples.begin(),m.triples.end(),back_inserter(ret));
		std::move(m.blobs.begin(),m.blobs.end(),back_inserter(bl));
	}

	auto p = rdf::write_list(ns.begin(),ns.end(),to_string(u));
	ret.emplace_back(root,rdf::ns_po("layers"),p.first);
	std::move(p.second.begin(),p.second.end(),back_inserter(ret));

	return archive(ret,bl);
}

template<>
region* po::unmarshal(const uuid& u, const rdf::storage& st)
{
	uuids::string_generator sg;
	rdf::node root = rdf::iri(u);
	rdf::node name = st.first(root,rdf::ns_po("name")).object;
	rdf::node base = st.first(root,rdf::ns_po("base")).object;
	rdf::node layers = st.first(root,rdf::ns_po("layers")).object;
	rdf::nodes ns = rdf::read_list(layers,st);

	uuid base_u = sg(base.as_literal());
	layer_loc b(base_u,unmarshal<layer>(base_u,st));
	region *ret = new region(name.as_literal(),b);

	for(auto n: ns)
	{
		rdf::node lay = st.first(n,rdf::ns_po("layer")).object;
		rdf::node b = st.first(n,rdf::ns_po("bound")).object;
		auto i = b.as_literal().find(':');
		uuid lay_u = sg(lay.as_literal());

		ensure(i != string::npos);
		layer_loc l(lay_u,unmarshal<layer>(lay_u,st));
		ret->add(bound(stoll(b.as_literal().substr(0,i)),stoll(b.as_literal().substr(i+1))),l);
	}

	return ret;
}
