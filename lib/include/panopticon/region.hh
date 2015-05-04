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

#include <type_traits>
#include <iterator>
#include <numeric>

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

	std::ostream& operator<<(std::ostream& os, const po::tryte& b);

	struct slab
	{
		using source = std::tuple<
			boost::variant<
				offset,
				std::pair<unsigned char const*,offset>
			>,
			std::list<std::pair<std::unordered_map<offset,tryte> const*,offset>>
		>;

		struct iterator : public boost::iterator_facade<
			iterator,
			po::tryte,
			boost::random_access_traversal_tag,
			po::tryte>
		{
			iterator(slab const * s,offset o) : _base(s), _off(o) {}

			inline iterator &increment(void) { _off++; return *this; }
			inline iterator &decrement(void) { _off--; return *this; }

			inline po::tryte dereference(void) const { return _base->read(_off); }
			inline bool equal(const iterator &a) const { return _base == a._base && _off == a._off; }

			inline void advance(size_t sz) { _off += sz; }

		private:
			slab const* _base;
			offset _off;

			friend struct slab;
		};

		inline slab(void) : _sources(), _cache(boost::none) {}
		inline slab(unsigned char const* r,offset o) : _sources({source(std::make_pair(r,o),{})}), _cache(boost::none) {}
		inline slab(char const* r,offset o) : slab(reinterpret_cast<unsigned char const*>(r),o) {}
		inline slab(offset o) : _sources({source(o,{})}), _cache(boost::none) {}

		slab(std::unordered_map<offset,tryte> const* m, slab sl);
		slab(iterator i,iterator e);

		inline offset size(void) const
		{
			if(_sources.size() == 1 && _cache)
				return _cache->first.upper() - _cache->first.lower();
			else
				return std::accumulate<std::list<source>::const_iterator,offset>(_sources.begin(),_sources.end(),0,
					[](offset acc, const source& s)
					{
						offset const *o = boost::get<offset>(&std::get<0>(s));
						if(o)
							return acc + *o;
						else
							return acc + boost::get<std::pair<unsigned char const*,offset>>(std::get<0>(s)).second;
					});
		}

		inline tryte read(offset i) const
		{
			struct vis : public boost::static_visitor<tryte>
			{
				vis(offset _i) : i(_i) {}
				tryte operator()(const std::pair<unsigned char const*,offset>& p) const { return p.first[i]; }
				tryte operator()(offset) const { return boost::none; }

				po::offset i;
			};

			if(!_cache || !boost::icl::contains(_cache->first,i))
			{
				offset j = 0;
				_cache = boost::none;

				for(const source& x: _sources)
				{
					offset const *o = boost::get<offset>(&std::get<0>(x));
					bound bnd(j,j + (o ? *o : boost::get<std::pair<unsigned char const*,offset>>(std::get<0>(x)).second));

					if(boost::icl::contains(bnd,i))
					{
						_cache = std::make_pair(bnd,&x);
						break;
					}

					j += (bnd.upper() - bnd.lower());
				}
			}

			if(!_cache)
					throw std::out_of_range("oor in slab");

			auto p = make_pair(std::get<1>(*_cache->second).rbegin(),std::get<1>(*_cache->second).rend());
			for(auto l: iters(p))
				if(l.first->count(l.second + i))
					return l.first->at(l.second + i);

			vis v(i - _cache->first.lower());
			return boost::apply_visitor(v,std::get<0>(*_cache->second));
		}

		inline iterator begin(void) const { return iterator(this,0); }
		inline iterator end(void) const { return iterator(this,size()); }

	private:
		inline slab(const std::list<source>& l) : _sources(l), _cache(boost::none) {}

		std::list<source> _sources;
		mutable boost::optional<std::pair<bound,source const*>> _cache;

		friend slab combine(slab,slab);
		friend std::ostream& operator<<(std::ostream& os, const po::slab& b);
	};

	std::ostream& operator<<(std::ostream& os, const po::slab& b);
	slab combine(slab,slab);

	struct layer
	{
		layer(const std::string&, std::initializer_list<byte>);
		layer(const std::string&, const std::vector<byte>&);
		layer(const std::string&, const byte*, offset);

		layer(const std::string&, const std::unordered_map<offset,tryte>&);
		layer(const std::string&, offset);
		layer(const std::string&, const blob&);

		bool operator==(const layer&) const;

		slab filter(const slab&) const;
		const std::string& name(void) const;
		void write(offset pos, tryte t);
		bool is_undefined(void) const;

	private:
		struct filter_visitor : public boost::static_visitor<slab>
		{
			filter_visitor(slab);

			slab operator()(const std::unordered_map<offset,tryte>& m) const;
			slab operator()(offset sz) const;
			slab operator()(const blob&) const;

			slab in;
		};

		std::string _name;
		boost::variant<
			blob,															///< Constant data. Ignores Input.
			std::unordered_map<offset,tryte>,	///< Sparse constant data.
			offset														///< Uninitialized (boost::none) data. Ignores input.
		> _data;

		template<typename T>
		friend archive marshal(T const&, const uuid&);
	};

	using layer_loc = loc<layer>;
	using layer_wloc = wloc<layer>;

	layer_wloc operator+=(layer_wloc& a, const layer_wloc &b);

	template<>
	archive marshal(layer const&, const uuid&);

	template<>
	std::unique_ptr<layer> unmarshal(const uuid&, const rdf::storage&);
}

namespace std
{
	std::ostream& operator<<(std::ostream&, const po::bound&);

	template<>
	struct hash<po::layer>
	{
		po::offset operator()(const po::layer &a) const
		{
			return hash<string>()(a.name());
		}
	};

	template<>
	struct hash<po::bound>
	{
		po::offset operator()(const po::bound &a) const
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
		static region_loc mmap(const std::string&, const boost::filesystem::path&);
		static region_loc undefined(const std::string&, offset);
		static region_loc wrap(const std::string&, const byte*, offset);
		static region_loc wrap(const std::string&, std::initializer_list<byte>);

		region(const std::string&, layer_loc root);

		bool operator==(const region& r) const;

		void add(const bound&, layer_loc);

		slab read(void) const;
		const std::list<std::pair<bound,layer_wloc>>& flatten(void) const;
		const std::list<std::pair<bound,layer_loc>>& stack(void) const;

		offset size(void) const;
		const std::string& name(void) const;

	private:
		layer_loc _base;
		std::list<std::pair<bound,layer_loc>> _stack; ///< Stack of layers to apply to this regions data.
		std::string _name;
		offset _size;

		// caches
		mutable boost::optional<std::list<std::pair<bound,layer_wloc>>> _projection;

		template<typename T>
		friend archive marshal(T const&, const uuid&);
	};

	template<>
	archive marshal(region const&, const uuid&);

	template<>
	std::unique_ptr<region> unmarshal(const uuid&, const rdf::storage&);

	/**
	 * DAG of regions. Models which region covers which.
	 * Edges point from the covered region to the region covering it.
	 */
	using regions = digraph<region_loc,bound>;

	std::unordered_map<region_wloc,region_wloc> spanning_tree(const regions&);
	std::list<std::pair<bound,region_wloc>> projection(const regions&);
}
