#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/symatic.hh>

using namespace po;
using dsl::operator%;
using dsl::operator/;
using dsl::operator+;
using dsl::operator-;
using dsl::operator*;

void po::amd64::adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext)
{
	if(sign_ext)
	{
		rvalue sign = b / (1 << (sign_ext->first - 1));
		rvalue rest = b % (1 << (sign_ext->first - 1));
		rvalue ex = (sign * (1 << (sign_ext->second - 1))) + rest;

		m.assign(to_lvalue(a),a + ex + CF);
	}
	else
	{
		m.assign(to_lvalue(a),a + b + CF);
	}
	// set OF, SF, ZF, AF, CF, and PF
}
