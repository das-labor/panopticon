#include <boost/iterator/counting_iterator.hpp>
#include <boost/iterator/zip_iterator.hpp>
#include <boost/iterator/transform_iterator.hpp>
#include <boost/tuple/tuple.hpp>
#include <panopticon/layer.hh>

using namespace po;
using namespace std;
using namespace boost;

map_layer::map_layer(const string &n, function<uint8_t(uint8_t)> fn)
: _name(n), _operation(fn)
{}

bool map_layer::operator==(const map_layer &a) const
{
	return a._name == _name;
}

slab map_layer::filter(const slab& in) const
{
	return adaptors::transform(in,adaptor(this));
}

const string& map_layer::name(void) const
{
	return _name;
}

map_layer::adaptor::adaptor(const map_layer *p) : parent(p) {}
uint8_t map_layer::adaptor::operator()(uint8_t i) const { return parent->_operation(i); }

anonymous_layer::anonymous_layer(std::initializer_list<byte> il, const std::string &n) : data(il), _name(n) {}
anonymous_layer::anonymous_layer(offset sz, const std::string &n) : data(sz), _name(n) {}

bool anonymous_layer::operator==(const anonymous_layer &a) const { return a.name() == name() && a.data == data; }

slab anonymous_layer::filter(const slab&) const { return slab(data.cbegin(),data.cend()); }
const std::string& anonymous_layer::name(void) const { return _name; }

mutable_layer::mutable_layer(const std::string &n) : data(), _name(n) {}

slab mutable_layer::filter(const slab& in) const
{
	auto b = make_zip_iterator(boost::make_tuple(counting_iterator<offset>(0),boost::begin(in)));
	auto e = make_zip_iterator(boost::make_tuple(counting_iterator<offset>(size(in)),boost::end(in)));
	auto fn = [this](const boost::tuples::tuple<offset,byte> &p) { return data.count(get<0>(p)) ? data.at(get<0>(p)) : get<1>(p); };
	return slab(make_transform_iterator(b,fn),make_transform_iterator(e,fn));
}

const std::string& mutable_layer::name(void) const { return _name; }
