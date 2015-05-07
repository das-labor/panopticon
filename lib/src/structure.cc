/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <panopticon/structure.hh>
#include <panopticon/marshal.hh>

#include <boost/uuid/uuid_generators.hpp>

using namespace po;
using namespace std;

structure::structure(const std::string& n, const tree<field>& f, const std::string& r)
: name(n), reg(r), fields(f)
{}

po::bound structure::area(void) const
{
	return fields.croot()->area;
}

template<>
archive po::marshal(structure const& s, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::iri(u);

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Structure"));
	ret.emplace_back(root,rdf::ns_po("name"),rdf::lit(s.name));
	ret.emplace_back(root,rdf::ns_po("region-name"),rdf::lit(s.reg));

	boost::uuids::name_generator ng(u);
	unsigned int field_idx = 0;
	std::function<rdf::node(boost::optional<rdf::node>,tree<field>::const_iterator)> fn;

	fn = [&](boost::optional<rdf::node> p, tree<field>::const_iterator me)
	{
		const field& fi = *me;
		uuid uu = ng(to_string(field_idx++));
		rdf::node f = rdf::iri(uu);

		if(p)
			ret.emplace_back(*p,rdf::ns_po("sub-field"),f);

		ret.emplace_back(f,rdf::ns_po("name"),rdf::lit(fi.name));
		ret.emplace_back(f,rdf::ns_po("area"),rdf::lit(to_string(fi.area.lower()) + ":" + to_string(fi.area.upper())));

		struct vis : public boost::static_visitor<>
		{
			vis(const uuid& ns, rdf::statements& r, rdf::node _f) : ng(ns), ret(r), f(_f) {}
			void operator()(const integer& i)
			{
				ret.emplace_back(f,rdf::ns_rdf("type"),rdf::ns_po("Integer"));
				ret.emplace_back(f,rdf::ns_po("mask"),rdf::lit(i.mask));
				ret.emplace_back(f,rdf::ns_po("shift"),rdf::lit(i.shift));
				ret.emplace_back(f,rdf::ns_po("signed"),rdf::boolean(i.has_sign));
				ret.emplace_back(f,rdf::ns_po("base"),rdf::lit(i.base));
				if(i.alternative_base)
					ret.emplace_back(f,rdf::ns_po("alternative-base"),rdf::lit(*i.alternative_base));

				switch(i.endianess)
				{
					case LittleEndian: ret.emplace_back(f,rdf::ns_po("endianess"),rdf::ns_po("little-endian")); break;
					case BigEndian: ret.emplace_back(f,rdf::ns_po("endianess"),rdf::ns_po("big-endian")); break;
					default: ensure(false);
				}

				for(auto p: i.symbolic)
				{
					rdf::node n = rdf::iri(ng("symbolic-" + to_string(p.first)));
					ret.emplace_back(f,rdf::ns_po("symbolic"),n);
					ret.emplace_back(n,rdf::ns_po("numeric"),rdf::lit(p.first));
					ret.emplace_back(n,rdf::ns_po("text"),rdf::lit(p.second));
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
			boost::uuids::name_generator ng;
			rdf::statements& ret;
			rdf::node f;
		};
		vis v(uu,ret,f);
		boost::apply_visitor(v,fi.value);

		auto k = s.fields.cbegin(me);
		while(k != s.fields.cend(me))
			fn(f,k++);

		return f;
	};

	rdf::node rf = fn(boost::none,s.fields.croot());
	ret.emplace_back(root,rdf::ns_po("root-field"),rf);

	return ret;
}

template<>
std::unique_ptr<structure> po::unmarshal(const uuid& u, const rdf::storage& store)
{
	rdf::node node = rdf::iri(u);

	if(!store.has(node,rdf::ns_rdf("type"),rdf::ns_po("Structure")))
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
			rdf::statement mask_st = store.first(n,rdf::ns_po("mask"));
			rdf::statement shift_st = store.first(n,rdf::ns_po("shift"));
			rdf::statement signed_st = store.first(n,rdf::ns_po("signed"));
			rdf::statement base_st = store.first(n,rdf::ns_po("base"));
			rdf::statements altbase_st = store.find(n,rdf::ns_po("alternative-base"));
			rdf::statement endianess_st = store.first(n,rdf::ns_po("endianess"));
			rdf::statements sym_st = store.find(n,rdf::ns_po("symbolic"));
			Endianess ed;
			bool hs;
			std::list<std::pair<unsigned long long,std::string>> syms;

			if(endianess_st.object == rdf::ns_po("little-endian"))
				ed = LittleEndian;
			else if(endianess_st.object == rdf::ns_po("big-endian"))
				ed =  BigEndian;
			else
				throw marshal_exception("invalid endianess");

			if(signed_st.object == rdf::boolean(true))
				hs = true;
			else if(signed_st.object == rdf::boolean(false))
				hs = false;
			else
				throw marshal_exception("invalid signedness");

			for(auto s: sym_st)
			{
				rdf::statement txt_st = store.first(s.object,rdf::ns_po("text"));
				rdf::statement num_st = store.first(s.object,rdf::ns_po("numeric"));
				syms.emplace_back(stoull(num_st.object.as_literal()),txt_st.object.as_literal());
			}

			return field{
				name_st.object.as_literal(),
				bound(stoull(b.substr(0,div)),stoull(b.substr(div + 1))),
				integer{
					stoull(mask_st.object.as_literal()),
					stoi(shift_st.object.as_literal()),
					hs,
					static_cast<unsigned int>(stoull(base_st.object.as_literal())),
					(altbase_st.size() ? boost::make_optional(static_cast<unsigned int>(stoull(altbase_st.front().object.as_literal()))) : boost::none),
					ed,
					syms
				}
			};
		}
		else if(type_st.object == rdf::ns_po("IEEE-754"))
		{
			return field{
				name_st.object.as_literal(),
				bound(stoull(b.substr(0,div)),stoull(b.substr(div + 1))),
				ieee754{}
			};
		}
		else if(type_st.object == rdf::ns_po("Generic"))
		{
			rdf::statement cont_st = store.first(n,rdf::ns_po("contents"));
			return field{
				name_st.object.as_literal(),
				bound(stoull(b.substr(0,div)),stoull(b.substr(div + 1))),
				cont_st.object.as_literal()
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
		for(auto c: store.find(n,rdf::ns_po("sub-field")))
			sn(fields.insert(p,fn(c.object)),c.object);
	};

	sn(fields.root(),root_field.object);

	return std::unique_ptr<structure>(new structure(name.object.as_literal(),fields,region_name.object.as_literal()));
}
