#include <panopticon/structure.hh>
#include <panopticon/marshal.hh>

#include <boost/uuid/uuid_generators.hpp>

using namespace po;
using namespace std;

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
					ret.emplace_back(f,rdf::ns_po("alternative_base"),rdf::lit(*i.alternative_base));

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
structure* po::unmarshal(const uuid&, const rdf::storage&)
{
	return nullptr;
}
