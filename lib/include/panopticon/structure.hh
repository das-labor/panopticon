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
		bool operator<(const ref& r) const { return reg == r.reg ? off < r.off : reg < r.reg; }

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
		integer(void) = delete;
		integer(unsigned long long m, int s, bool hs, unsigned int b, boost::optional<unsigned int> ab, Endianess e, const std::list<std::pair<unsigned long long,std::string>> &l)
		: mask(m), shift(s), has_sign(hs), base(b), alternative_base(ab), endianess(e), symbolic(l)
		{}

		bool operator==(const integer& i) const
		{
			if(&i == this)
				return true;
			else
			{
				if(!(mask == i.mask && shift == i.shift &&
						 has_sign == i.has_sign && base == i.base &&
						 alternative_base == i.alternative_base &&
						 endianess == i.endianess))
					return false;

				if(symbolic.size() != i.symbolic.size())
					return false;

				return std::all_of(symbolic.begin(),symbolic.end(),[&](const std::pair<unsigned long long,std::string>& p)
					{ return std::find(i.symbolic.begin(),i.symbolic.end(),p) != i.symbolic.end(); });
			}
		}

		bool operator!=(const integer& i) const { return !(i == *this); }

		unsigned long long mask;
		int shift;
		bool has_sign;
		unsigned int base;
		boost::optional<unsigned int> alternative_base;
		Endianess endianess;
		std::list<std::pair<unsigned long long,std::string>> symbolic;
	};

	struct ieee754
	{
		bool operator==(const ieee754&) const { return true; }
		bool operator!=(const ieee754&) const { return false; }
	};

	using format = boost::variant<integer,ieee754,std::string>;

	std::string read(const format&, slab);
	slab write(const format&, const std::string&);

	struct field
	{
		bool operator==(const field& f) const { return &f == this || (name == f.name && area == f.area && value == f.value); }
		bool operator!=(const field& i) const { return !(i == *this); }

		std::string name;
		bound area;
		format value;
	};

	using struct_loc = loc<struct structure>;
	using struct_wloc = wloc<struct structure>;

	struct structure
	{
		structure(const std::string&, const tree<field>&, const std::string&);

		bool operator==(const structure& s) const
			{ return name == s.name && reg == s.reg && fields == s.fields; }
		bool operator!=(const structure& st) const
			{ return !(st == *this); }

		po::bound area(void) const;

		std::string name;
		std::string reg;
		tree<field> fields;

	private:
		template<typename T>
		friend archive marshal(const T*, const uuid&);
	};

	template<>
	archive marshal(const structure*, const uuid&);

	template<>
	structure* unmarshal(const uuid&, const rdf::storage&);

	template<typename ParserTag>
	struct_loc parse(region_loc, offset, ParserTag);
}
