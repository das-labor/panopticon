#include <boost/range/any_range.hpp>
#include <boost/range/adaptor/transformed.hpp>
#include <boost/range.hpp>

#include <boost/type_erasure/any.hpp>
#include <boost/type_erasure/any_cast.hpp>
#include <boost/type_erasure/builtin.hpp>
#include <boost/type_erasure/operators.hpp>
#include <boost/type_erasure/member.hpp>
#include <boost/type_erasure/free.hpp>
#include <boost/mpl/vector.hpp>

#include <panopticon/marshal.hh>

#pragma once

BOOST_TYPE_ERASURE_MEMBER((has_filter), filter, 1)
BOOST_TYPE_ERASURE_MEMBER((has_name), name, 0)

namespace po
{
	using slab = boost::any_range<uint8_t,boost::random_access_traversal_tag,uint8_t,std::ptrdiff_t>;
	using layer = boost::type_erasure::any<
									boost::mpl::vector<
										has_filter<slab(const slab&)>,
										has_name<const std::string&(void)>,
										boost::type_erasure::copy_constructible<>,
										boost::type_erasure::assignable<>,
										boost::type_erasure::relaxed,
										boost::type_erasure::typeid_<>
									>,
									boost::type_erasure::_self
								>;

	struct map_layer
	{
		map_layer(const std::string &, std::function<uint8_t(uint8_t)> fn);

		slab filter(const slab&) const;
		const std::string& name(void) const;
	//	void invalidate_cache(void);

	private:
		struct adaptor
		{
			using result_type = uint8_t;

			adaptor(const map_layer *p = nullptr);
			uint8_t operator()(uint8_t) const;

			const map_layer *parent;
		};

		std::string _name;
		std::function<uint8_t(uint8_t)> _operation;
	};

	template<unsigned int N>
	struct block_layer {};

	/*template<>
	rdf::statements marshal(const layer*, const uuid&);

	template<>
	layer* unmarshal(const uuid&, const rdf::storage&);*/
}
