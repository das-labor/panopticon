#include <panopticon/layer.hh>

using namespace po;
using namespace std;

map_layer::map_layer(const string &n, function<uint8_t(uint8_t)> fn)
: _name(n), _operation(fn)
{}

slab map_layer::filter(const slab& in) const
{
	return boost::adaptors::transform(in,adaptor(this));
}

const string& map_layer::name(void) const
{
	return _name;
}

map_layer::adaptor::adaptor(const map_layer *p) : parent(p) {}
uint8_t map_layer::adaptor::operator()(uint8_t i) const { return parent->_operation(i); }
