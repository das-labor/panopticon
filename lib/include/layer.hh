#include <type_traits>

#include <boost/range/any_range.hpp>
#include <boost/range/adaptor/transformed.hpp>
#include <boost/range.hpp>
#include <boost/type_erasure/any.hpp>
#include <boost/type_erasure/any_cast.hpp>
#include <boost/type_erasure/builtin.hpp>
#include <boost/type_erasure/operators.hpp>
#include <boost/type_erasure/member.hpp>
#include <boost/type_erasure/free.hpp>
#include <boost/type_erasure/call.hpp>
#include <boost/mpl/vector.hpp>
#include <boost/icl/right_open_interval.hpp>
#include <boost/optional.hpp>
#include <boost/icl/split_interval_map.hpp>

#include <panopticon/marshal.hh>
#include <panopticon/loc.hh>
#include <panopticon/digraph.hh>

#pragma once

BOOST_TYPE_ERASURE_MEMBER((has_filter), filter, 1)
BOOST_TYPE_ERASURE_MEMBER((has_name), name, 0)
BOOST_TYPE_ERASURE_MEMBER((has_address_space), address_space, 0)
BOOST_TYPE_ERASURE_MEMBER((has_marshal), marshal, 1)

namespace po
{
/*	template<typename T = boost::type_erasure::_self>
	struct has_marshal { static rdf::statements apply(const T* t, const uuid& u) { return marshal<T>(t,u); } };*/

	template<typename T = boost::type_erasure::_self, typename U = T*>
	struct has_unmarshal { static U apply(const uuid& u, const rdf::storage &s) { return reinterpret_cast<U>(unmarshal<T>(u,s)); } };

	template<typename T = boost::type_erasure::_self>
	struct has_hash { static size_t apply(const T& t) { return std::hash<T>()(t); } };

	using offset = uint64_t;
	using byte = uint8_t;
	using bound = boost::icl::discrete_interval<offset>;
	using slab = boost::any_range<byte,boost::random_access_traversal_tag,byte,std::ptrdiff_t>;
	using layer = boost::type_erasure::any<
									boost::mpl::vector<
										has_filter<slab(const slab&)>,
										has_name<const std::string&(void),const boost::type_erasure::_self>,
										//has_address_space<const bound&(void)>,
										has_marshal<rdf::statements(const uuid&)>,
										has_unmarshal<boost::type_erasure::_self,boost::type_erasure::_self*>,
										has_hash<>,
										boost::type_erasure::copy_constructible<>,
										boost::type_erasure::equality_comparable<>,
										boost::type_erasure::assignable<>,
										boost::type_erasure::relaxed,
										boost::type_erasure::typeid_<>
									>/*,
									boost::type_erasure::_self*/
								>;
	using layer_loc = loc<layer>;
	using layer_wloc = wloc<layer>;

	layer_wloc operator+=(layer_wloc& a, const layer_wloc &b);

	struct map_layer;
	template<> rdf::statements marshal(const map_layer*, const uuid&);

	struct map_layer
	{
		map_layer(const std::string &, std::function<byte(byte)> fn);

		bool operator==(const map_layer&) const;

		slab filter(const slab&) const;
		const std::string& name(void) const;
		rdf::statements marshal(const uuid &u) const { return po::marshal<map_layer>(this,u); }
	//	void invalidate_cache(void);

	private:
		struct adaptor
		{
			using result_type = byte;

			adaptor(const map_layer *p = nullptr);
			byte operator()(byte) const;

			const map_layer *parent;
		};

		std::string _name;
		std::function<byte(byte)> _operation;
	};

	struct anonymous_layer
	{
		anonymous_layer(const anonymous_layer&);
		anonymous_layer(std::initializer_list<byte> il, const std::string &n);
		anonymous_layer(offset sz, const std::string &n);

		anonymous_layer& operator=(const anonymous_layer&);
		bool operator==(const anonymous_layer &a) const;

		slab filter(const slab&) const;
		const std::string& name(void) const;
		rdf::statements marshal(const uuid &u) const { return po::marshal<anonymous_layer>(this,u); }

		std::vector<byte> data;

	private:
		std::string _name;
	};

	struct mutable_layer
	{
		mutable_layer(const std::string &);

		slab filter(const slab&) const;
		const std::string& name(void) const;
		rdf::statements marshal(const uuid &u) const { return po::marshal<mutable_layer>(this,u); }

		std::map<offset,byte> data;

	private:
		std::string _name;
	};

	struct null_layer
	{
		null_layer(void);

		bool operator==(const null_layer &a) const;

		slab filter(const slab&) const;
		const std::string& name(void) const;
		rdf::statements marshal(const uuid &u) const { return po::marshal<null_layer>(this,u); }

	private:
		std::string _name;
	};
}

namespace std
{
	template<>
	struct hash<po::map_layer>
	{
		size_t operator()(const po::map_layer &a) const
		{
			return hash<string>()(a.name());
		}
	};

	template<>
	struct hash<po::anonymous_layer>
	{
		size_t operator()(const po::anonymous_layer &a) const
		{
			return hash<string>()(a.name());// ^ hash<std::vector<po::byte>>()(a.data);
		}
	};

	template<>
	struct hash<po::mutable_layer>
	{
		size_t operator()(const po::mutable_layer &a) const
		{
			return hash<string>()(a.name());// ^ hash<std::map<po::offset,po::byte>>()(a.data);
		}
	};

	template<>
	struct hash<po::null_layer>
	{
		size_t operator()(const po::null_layer &a) const
		{
			return hash<string>()(a.name());// ^ hash<std::map<po::offset,po::byte>>()(a.data);
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

	template<>
	struct hash<po::layer>
	{
		size_t operator()(const po::layer &a) const
		{
			return boost::type_erasure::call(po::has_hash<>(),a);;
		}
	};
}

namespace po
{
	template<>
	rdf::statements marshal(const anonymous_layer*, const uuid&) { return rdf::statements(); }
	template<>
	rdf::statements marshal(const mutable_layer*, const uuid&) { return rdf::statements(); }
	template<>
	rdf::statements marshal(const null_layer*, const uuid&) { return rdf::statements(); }
	template<>
	rdf::statements marshal(const layer *l, const uuid &u) { return boost::type_erasure::call(has_marshal<layer>(),l,u); }

	template<>
	map_layer* unmarshal(const uuid&, const rdf::storage&) { return nullptr; }
	template<>
	mutable_layer* unmarshal(const uuid&, const rdf::storage&) { return nullptr; }
	template<>
	anonymous_layer* unmarshal(const uuid&, const rdf::storage&) { return nullptr; }
	template<>
	null_layer* unmarshal(const uuid&, const rdf::storage&) { return nullptr; }

	struct stack
	{
		using image = boost::icl::split_interval_map<offset,layer_wloc>;
		using layers = digraph<layer_loc,bound>;
		using tree = std::unordered_map<layer_wloc,layer_wloc>;

		stack(void);
		void add(const bound&, layer_loc);

		const image& projection(void) const;
		const layers& graph(void) const;
		//const tree& spanning_tree(void) const;
		//const boost::icl::split_interval_map<offset,std::pair<bound,layer_wloc>> &continuous(void) const;

	private:
		layers _graph;
		boost::graph_traits<digraph<layer_loc,bound>>::vertex_descriptor _root;

		// caches
		mutable boost::optional<image> _projection;
		mutable boost::optional<tree> _spanning_tree;
	};
}
