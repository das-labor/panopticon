#include <type_traits>
#include <iterator>

#include <boost/range/any_range.hpp>
#include <boost/range/adaptor/transformed.hpp>
#include <boost/range.hpp>
#include <boost/icl/interval.hpp>
#include <boost/icl/split_interval_map.hpp>
#include <boost/optional.hpp>
#include <boost/variant.hpp>
#include <boost/variant/static_visitor.hpp>

#include <panopticon/marshal.hh>
#include <panopticon/loc.hh>
#include <panopticon/digraph.hh>

#pragma once

namespace po
{
	using offset = uint64_t;
	using byte = uint8_t;
	using bound = boost::icl::right_open_interval<offset>;
	using tryte = boost::optional<byte>;
	using slab = boost::any_range<tryte,boost::random_access_traversal_tag,tryte,std::ptrdiff_t>;
}

namespace std
{
	template<> inline po::slab::iterator next(po::slab::iterator i, ptrdiff_t off) { advance(i,off); return i; }
}

namespace po
{
	struct layer
	{
		layer(const std::string&, std::initializer_list<byte>);
		layer(const std::string&, const std::vector<byte>&);
		layer(const std::string&, const byte*, size_t);

		layer(const std::string&, const std::unordered_map<offset,tryte>&);
		layer(const std::string&, offset);

		slab filter(const slab&) const;
		const std::string& name(void) const;
		void write(offset pos, tryte t);

	private:
		struct filter_visitor : public boost::static_visitor<slab>
		{
			filter_visitor(slab);

			slab operator()(const std::vector<byte>& d) const;
			slab operator()(const std::unordered_map<offset,tryte>& m) const;
			slab operator()(size_t sz) const;

			slab in;
		};

		std::string _name;
		boost::variant<
			std::vector<byte>,								///< Constant data. Ignores Input.
			std::unordered_map<offset,tryte>,	///< Sparse constant data.
			size_t														///< Uninitialized (boost::none) data. Ignores input.
		> _data;

		template<typename T>
		friend rdf::statements marshal(const T*, const uuid&);
	};

	using layer_loc = loc<layer>;
	using layer_wloc = wloc<layer>;

	layer_wloc operator+=(layer_wloc& a, const layer_wloc &b);

	template<>
	rdf::statements marshal(const layer*, const uuid&);

	template<>
	layer* unmarshal(const uuid&, const rdf::storage&);
}

namespace std
{
	std::ostream& operator<<(std::ostream&, const po::bound&);

	template<>
	struct hash<po::layer>
	{
		size_t operator()(const po::layer &a) const
		{
			return hash<string>()(a.name());
		}
	};

	template<>
	struct hash<po::bound>
	{
		size_t operator()(const po::bound &a) const
		{
			return hash<po::offset>()(boost::icl::first(a)) + hash<po::offset>()(boost::icl::last(a));
		}
	};
}

namespace po
{
	struct region;

	using region_loc = loc<region>;
	using region_wloc = wloc<region>;

	struct region
	{
		static region_loc mmap(const std::string&, const std::string&);
		static region_loc undefined(const std::string&, size_t);
		static region_loc wrap(const std::string&, const byte*, size_t);
		static region_loc wrap(const std::string&, std::initializer_list<byte>);

		region(const std::string&, layer_loc root);
		void add(const bound&, layer_loc);

		slab read(void) const;

		const std::list<std::pair<bound,layer_wloc>>& flatten(void) const;
		const std::list<std::pair<bound,layer_loc>>& stack(void) const;

		size_t size(void) const;
		const std::string& name(void) const;

	private:
		layer_loc _base;
		std::list<std::pair<bound,layer_loc>> _stack; ///< Stack of layers to apply to this regions data.
		std::string _name;
		size_t _size;

		// caches
		mutable boost::optional<std::list<std::pair<bound,layer_wloc>>> _projection;
	};

	template<>
	rdf::statements marshal(const region*, const uuid&);

	template<>
	region* unmarshal(const uuid&, const rdf::storage&);

	/**
	 * DAG of regions. Models which region covers which.
	 * Edges point from the covered region to the region covering it.
	 */
	using regions = digraph<region_loc,bound>;

	std::unordered_map<region_wloc,region_wloc> spanning_tree(const regions&);
	std::list<std::pair<bound,region_wloc>> projection(const regions&);
}
