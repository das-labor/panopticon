#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/symantic.hh>
#include <panopticon/code_generator.hh>

using namespace po;


void po::amd64::adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext)
{
	using dsl::operator%;
	using dsl::operator/;
	using dsl::operator+;
	using dsl::operator-;
	using dsl::operator*;

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

void po::amd64::flagcomp(cg& m, variable const& flag) {}
void po::amd64::flagwr(cg& m, variable const& flag,bool val) {}

void po::amd64::aaa(cg& m) {}
void po::amd64::aam(cg& m, rvalue a, rvalue b) {}
void po::amd64::aad(cg& m, rvalue a, rvalue b) {}
void po::amd64::aas(cg& m) {}
void po::amd64::add(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
void po::amd64::adcx(cg& m, rvalue a, rvalue b) {}
void po::amd64::and_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
void po::amd64::bound(cg& m, rvalue a, rvalue b) {}
void po::amd64::bsf(cg& m, rvalue a, rvalue b) {}
void po::amd64::bsr(cg& m, rvalue a, rvalue b) {}
void po::amd64::bswap(cg& m, rvalue a) {}
void po::amd64::bt(cg& m, rvalue a, rvalue b) {}
void po::amd64::btc(cg& m, rvalue a, rvalue b) {}
void po::amd64::btr(cg& m, rvalue a, rvalue b) {}
void po::amd64::bts(cg& m, rvalue a, rvalue b) {}
void po::amd64::call(cg& m, rvalue a, bool rel) {}
void po::amd64::cbw(cg& m) {}
void po::amd64::cwde(cg& m) {}
void po::amd64::cwqe(cg& m) {}
void po::amd64::cmov(cg& m, rvalue a, rvalue b, condition c) {}
void po::amd64::cmp(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
void po::amd64::cmps(cg& m, int bits) {}
void po::amd64::cmpxchg(cg& m, rvalue a, rvalue b, int bits) {}
void po::amd64::or_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
void po::amd64::sbb(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
void po::amd64::sub(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
void po::amd64::xor_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext) {}
