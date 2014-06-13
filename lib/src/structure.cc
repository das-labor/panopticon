#include <panopticon/structure.hh>
#include <panopticon/marshal.hh>

#include <boost/uuid/uuid_generators.hpp>

using namespace po;
using namespace std;

structure::structure(const std::string& n, const tree<field>& f, const std::string& r)
: name(n), fields(f), _area(boost::none), _region(r)
{}

template<>
rdf::statements po::marshal(const structure* s, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::ns_local(to_string(u));

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Structure"));
	ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(s->name));
	ret.emplace_back(root,rdf::ns_po("region-name"),rdf::lit(s->_region));

	boost::uuids::string_generator sg;
	std::function<void(tree<field>::const_iterator)> fn;

	fn = [&](tree<field>::const_iterator j)
	{
		const field& fi = *j;
		rdf::node f = rdf::ns_local(to_string(sg("root-field")));
		ret.emplace_back(root,rdf::ns_po("sub-field"),f);
		ret.emplace_back(f,rdf::ns_po("name"),fi.name);
		ret.emplace_back(f,rdf::ns_po("area"),to_string(fi.area.lower()) + ":" + to_string(fi.area.upper()));

		struct vis : public boost::static_visitor<>
		{
			vis(rdf::statements& r, rdf::node _f) : ret(r), f(_f) {}
			void operator()(const integer& i) const
			{
				ret.emplace_back(f,rdf::ns_rdf("type"),rdf::ns_po("Integer"));
				ret.emplace_back(f,rdf::ns_po("mask"),rdf::lit(i.mask));
				ret.emplace_back(f,rdf::ns_po("shift"),rdf::lit(i.shift));
				ret.emplace_back(f,rdf::ns_po("signed"),rdf::lit(i.has_sign));
				ret.emplace_back(f,rdf::ns_po("base"),rdf::lit(i.base));
				if(i.alternative_base)
					ret.emplace_back(f,rdf::ns_po("alternative-base"),rdf::lit(*i.alternative_base));

				switch(i.endianess)
				{
					case LittleEndian: ret.emplace_back(f,rdf::ns_po("endianess"),rdf::lit("little-endian")); break;
					case BigEndian: ret.emplace_back(f,rdf::ns_po("endianess"),rdf::lit("big-endian")); break;
					default: assert(false);
				}

				if(i.symbolic)
				{
					for(auto p: *i.symbolic)
					{
						rdf::node n = rdf::ns_local(to_string(sg("symbolic-" + to_string(p.first))));
						ret.emplace_back(n,rdf::ns_rdf("type"),rdf::ns_po("Symbolic-Value"));
						ret.emplace_back(n,rdf::ns_po("numeric"),rdf::lit(p.first));
						ret.emplace_back(n,rdf::ns_po("symbolic"),rdf::lit(p.second));
					}
				}
			}

			void operator()(const ieee754&) const
				{ ret.emplace_back(f,rdf::ns_rdf("type"),rdf::ns_po("IEEE-754")); }

			void operator()(const std::string& s) const
			{
				ret.emplace_back(f,rdf::ns_rdf("type"),rdf::ns_po("Generic"));
				ret.emplace_back(f,rdf::ns_po("contents"),rdf::lit(s));
			}

		private:
			boost::uuids::string_generator sg;
			rdf::statements& ret;
			rdf::node f;
		};

		boost::apply_visitor(vis(ret,f), fi.value);
	};

	fn(s->fields.croot());

	return ret;
}

template<>
structure* po::unmarshal(const uuid& u, const rdf::storage& store)
{
	rdf::node node(rdf::ns_local(to_string(u)));

	if(store.has(node,rdf::ns_rdf("type"),rdf::ns_po("Structure")))
		throw marshal_exception("invalid type");

	rdf::statement name = store.first(node,rdf::ns_po("name")),
						region_name = store.first(node,rdf::ns_po("region-name")),
						root_field = store.first(node,rdf::ns_po("root-field"));

	std::function<field(rdf::node)> fn;
	fn = [&](rdf::node n)
	{
		rdf::statement name_st = store.first(n,rdf::ns_po("name"));
		rdf::statement area_st = store.first(n,rdf::ns_po("area"));
		rdf::statement type_st = store.first(n,rdf::ns_rdf("type"));
		std::string b = area_st.object.as_literal();
		std::string::size_type div = b.find(":");

		if(div == std::string::npos)
			throw marshal_exception("ill-formed bound");

		if(type_st.object == rdf::ns_po("Integer"))
		{
			return field{
				name_st.object.as_literal(),
				bound(stoull(b.substr(0,div - 1)),stoull(b.substr(div))),
				integer{}
			};
		}
		else if(type_st.object == rdf::ns_po("IEEE-754"))
		{
			return field{
				name_st.object.as_literal(),
				bound(stoull(b.substr(0,div - 1)),stoull(b.substr(div))),
				integer{}
			};
		}
		else if(type_st.object == rdf::ns_po("Generic"))
		{
			return field{
				name_st.object.as_literal(),
				bound(stoull(b.substr(0,div - 1)),stoull(b.substr(div))),
				integer{}
			};
		}
		else
		{
			throw marshal_exception("unknown field type");
		}
	};

	tree<field> fields(fn(root_field.object));

	std::function<void(tree<field>::iterator,rdf::node)> sn;
	sn = [&](tree<field>::iterator p, rdf::node n)
	{
		auto q = fields.insert(p,fn(n));

		for(auto c: store.find(n,rdf::ns_po("sub-field")))
			sn(q,c.object);
	};

	sn(fields.root(),root_field.object);

	return new structure(name.object.as_literal(),fields,region_name.object.as_literal());
}
