#include <unordered_map>
#include <functional>
#include <list>

#include <boost/variant.hpp>

#include <panopticon/value.hh>
#include <panopticon/region.hh>
#include <panopticon/marshal.hh>
#include <panopticon/loc.hh>
#include <panopticon/tree.hh>

#pragma once

namespace po
{
	/// Position of a single tryte in the region graph
	struct ref
	{
		std::string reg;
		offset off;
	};

	/// Possibly empty area in the region graph
	struct area
	{
		std::string reg;
		bound bnd;
	};

	struct integer
	{
		unsigned long long mask;
		int shift;
		bool has_sign;
		unsigned int base;
		boost::optional<unsigned int> alternative_base;
		Endianess endianess;
		std::shared_ptr<std::unordered_map<unsigned long long,std::string>> symbolic;
	};

	struct ieee754 {};

	using format = boost::variant<integer,ieee754,std::string>;

	std::string read(const format&, slab);
	slab write(const format&, const std::string&);

	struct field
	{
		std::string name;
		bound area;
		format value;
	};

	using struct_loc = loc<struct structure>;
	using struct_wloc = wloc<struct structure>;

	struct structure
	{
		std::string name;
		tree<field> fields;

	private:
		boost::optional<area> _area;
		std::string _region;

		friend area extends(struct_loc);

		template<typename T>
		friend rdf::statements marshal(const T*, const uuid&);
	};

	template<>
	rdf::statements marshal(const structure*, const uuid&);

	template<>
	structure* unmarshal(const uuid&, const rdf::storage&);

	template<typename ParserTag>
	struct_loc parse(region_loc, offset, ParserTag);
}
