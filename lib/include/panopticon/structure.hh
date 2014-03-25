#include <unordered_map>
#include <functional>
#include <list>

#include <boost/variant.hpp>

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

	struct format
	{
		static format integer(offset bw, bool sign);
		static format ieee754(offset bw);
		static format bitfield(const std::unordered_map<unsigned int,std::string>& bits);
		static format enumeration(const std::unordered_map<std::list<byte>,std::string>& values);

		boost::variant<
			bound,															  ///< integer
	//		std::unordered_map<std::vector<byte>,std::string>,		  ///< enum
			std::unordered_map<unsigned int,std::string>,			  ///< bitfield
			std::pair<														  ///< generic
				std::function<std::string(const std::list<byte>&)>,
				std::function<std::list<byte>(const std::string&)>
			>
		> type;

		std::string read(slab) const;
		slab write(const std::string&) const;
	};

	using bits = unsigned int;

	struct field
	{
		std::string name;
		bits size;
		std::list<boost::variant<std::string,format>> value;
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
	};

	template<>
	rdf::statements marshal(const structure*, const uuid&);

	template<>
	structure* unmarshal(const uuid&, const rdf::storage&);

	template<typename ParserTag>
	struct_loc parse(region_loc, offset, ParserTag);
}
