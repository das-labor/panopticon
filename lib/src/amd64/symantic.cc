#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/symantic.hh>
#include <panopticon/code_generator.hh>

using namespace po;
using namespace dsl;

rvalue po::amd64::sign_ext(rvalue v, unsigned from, unsigned to, cg& m)
{
	using dsl::operator*;

	rvalue sign = v / (1 << (from - 1));
	rvalue rest = v % (1 << (from - 1));

	return (sign * (1 << (to - 1))) + rest;
}

void po::amd64::set_arithm_flags(rvalue res, rvalue res_half, rvalue a, rvalue b, cg& m)
{
	size_t const a_w = to_variable(a).width();
	rvalue const msb_res = less(res / (1 << (a_w - 1)),1);

	m.assign(CF,res / constant(1 << a_w));
	m.assign(AF,res_half / constant(0x100));
	m.assign(SF,msb_res);
	m.assign(ZF,equal(a,constant(0)));
	m.assign(OF,CF ^ SF);

	rvalue b0 = res % 2;
	rvalue b1 = (res % 4) / 2;
	rvalue b2 = (res % 8) / 4;
	rvalue b3 = (res % 16) / 8;
	rvalue b4 = (res % 32) / 16;
	rvalue b5 = (res % 64) / 32;
	rvalue b6 = (res % 128) / 64;
	rvalue b7 = (res % 256) / 128;
	m.assign(PF,b0 ^ b1 ^ b2 ^ b3 ^ b4 ^ b5 ^ b6 ^ b7);
}

void po::amd64::adc(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	size_t const a_w = to_variable(a).width();
	rvalue const res = a + (se ? sign_ext(b,se->first,se->second,m) : b) + CF;
	rvalue const res_half = (a % constant(0x100)) + (b % constant(0x100)) + CF;

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::flagcomp(cg& m, variable const& flag)
{
	m.assign(flag,m.not_b(flag));
}

void po::amd64::flagwr(cg& m, variable const& flag,bool val)
{
	m.assign(flag,constant(!!val));
}

void po::amd64::aaa(cg& m)
{
	using dsl::operator*;

	rvalue y = al & constant(0x0f);
	rvalue x = m.or_b(m.not_b(m.or_b(less(y,constant(9)),equal(y,constant(9)))),AF);

	m.assign(AF,m.lift_b(x));
	m.assign(CF,m.lift_b(x));
	m.assign(ax,(ax + m.lift_b(x) * constant(0x106)) % constant(0x100));
}

void po::amd64::aam(cg& m, rvalue a)
{
	rvalue temp_al = m.assign(al);

	m.assign(ah,temp_al / a);
	m.assign(al,temp_al % a);
}

void po::amd64::aad(cg& m, rvalue a)
{
	using dsl::operator*;

	rvalue temp_al = m.assign(al);
	rvalue temp_ah = m.assign(ah);

	m.assign(al,temp_al + temp_ah * a);
	m.assign(ah,constant(0));
}

void po::amd64::aas(cg& m)
{
	using dsl::operator*;

	rvalue y = al & constant(0x0f);
	rvalue x = m.or_b(m.not_b(m.or_b(less(y,constant(9)),equal(y,constant(9)))),AF);

	m.assign(AF,m.lift_b(x));
	m.assign(CF,m.lift_b(x));
	m.assign(ax,(ax - m.lift_b(x) * constant(6)) % constant(0x100));
	m.assign(ah,(ah - m.lift_b(x)) % constant(0x10));
	m.assign(al,y);
}

void po::amd64::add(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	size_t const a_w = to_variable(a).width();
	rvalue const res = a + (se ? sign_ext(b,se->first,se->second,m) : b);
	rvalue const res_half = (a % constant(0x100)) + (b % constant(0x100));

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::adcx(cg& m, rvalue a, rvalue b)
{
	size_t const a_w = to_variable(a).width();
	rvalue const res = a + b + CF;

	m.assign(CF,res / constant(1 << a_w));
	m.assign(to_lvalue(a),res % constant(1 << a_w));
}

void po::amd64::and_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	rvalue const res = a & (se ? sign_ext(b,se->first,se->second,m) : b);
	rvalue const res_half = (a % constant(0x100)) & (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::bound(cg& m, rvalue a, rvalue b) {}

void po::amd64::bsf(cg& m, rvalue a, rvalue b)
{
	using dsl::operator*;

	size_t const a_w = to_variable(a).width();
	size_t bit = 0;
	boost::optional<rvalue> prev;

	m.assign(ZF,equal(constant(0),b));

	while(bit < a_w)
	{
		rvalue val = (b % (1 << (bit + 1)) / (1 << bit));

		m.assign(to_lvalue(a),constant(bit + 1) * val);
		if(prev)
			prev = *prev | val;
		else
			prev = val;

		++bit;
	}
}

void po::amd64::bsr(cg& m, rvalue a, rvalue b)
{
	using dsl::operator*;

	size_t const a_w = to_variable(a).width();
	size_t bit = a_w - 1;
	boost::optional<rvalue> prev;

	m.assign(ZF,equal(constant(0),b));

	do
	{
		rvalue val = (b % (1 << (bit + 1)) / (1 << bit));

		m.assign(to_lvalue(a),constant(bit + 1) * val);
		if(prev)
			prev = *prev | val;
		else
			prev = val;
	}
	while(bit--);
}

void po::amd64::bswap(cg& m, rvalue a)
{
	using dsl::operator*;

	size_t const a_w = to_variable(a).width();
	size_t byte = 0;

	rvalue tmp = undefined();

	while(byte < a_w / 8)
	{
		unsigned int lsb = byte * 8;
		unsigned int div = (1 << lsb), mul = (1 << (a_w - byte * 8));

		tmp = tmp + (((a / div) % constant(0x100)) * mul);
		++byte;
	}

	m.assign(to_lvalue(a),tmp);
}

void po::amd64::bt(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	rvalue mod = (constant(1) << (b % constant(to_variable(a).width())));

	m.assign(CF,(a / mod) % 2);
	m.assign(PF,undefined());
	m.assign(OF,undefined());
	m.assign(SF,undefined());
	m.assign(AF,undefined());
}

void po::amd64::btc(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	rvalue mod = (constant(1) << (b % constant(to_variable(a).width())));

	m.assign(CF,(a / mod) % 2);
	m.assign(PF,undefined());
	m.assign(OF,undefined());
	m.assign(SF,undefined());
	m.assign(AF,undefined());
	m.assign(to_lvalue(a),a ^ mod);
}

void po::amd64::btr(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	size_t const a_w = to_variable(a).width();
	rvalue mod =  ((constant(1) << (b % constant(to_variable(a).width()))));

	m.assign(CF,(a / mod) % 2);
	m.assign(PF,undefined());
	m.assign(OF,undefined());
	m.assign(SF,undefined());
	m.assign(AF,undefined());
	m.assign(to_lvalue(a),(a & (constant(0xffffffffffffffff) ^ mod)) % constant(1 << a_w));
}

void po::amd64::bts(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	rvalue mod = (constant(1) << (b % constant(to_variable(a).width())));

	m.assign(CF,(a / mod) % 2);
	m.assign(PF,undefined());
	m.assign(OF,undefined());
	m.assign(SF,undefined());
	m.assign(AF,undefined());
	m.assign(to_lvalue(a),a & mod);
}

void po::amd64::call(cg& m, rvalue a, bool rel, bool near)
{
}

void po::amd64::cbw(cg& m) {}
void po::amd64::cwde(cg& m) {}
void po::amd64::cwqe(cg& m) {}
void po::amd64::cmov(cg& m, rvalue a, rvalue b, condition c) {}

void po::amd64::cmp(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	rvalue const res = a - (se ? sign_ext(b,se->first,se->second,m) : b);
	rvalue const res_half = (a % constant(0x100)) - (b % constant(0x100));

	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::cmps(cg& m, int bits) {}
void po::amd64::cmpxchg(cg& m, rvalue a, rvalue b, int bits) {}

void po::amd64::or_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	rvalue const res = a | (se ? sign_ext(b,se->first,se->second,m) : b);
	rvalue const res_half = (a % constant(0x100)) | (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::sbb(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	size_t const a_w = to_variable(a).width();
	rvalue const res = a - (se ? sign_ext(b,se->first,se->second,m) : b) - CF;
	rvalue const res_half = (a % constant(0x100)) - (b % constant(0x100)) - CF;

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::sub(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	size_t const a_w = to_variable(a).width();
	rvalue const res = a - (se ? sign_ext(b,se->first,se->second,m) : b);
	rvalue const res_half = (a % constant(0x100)) - (b % constant(0x100));

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::xor_(cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> se)
{
	rvalue const res = a ^ (se ? sign_ext(b,se->first,se->second,m) : b);
	rvalue const res_half = (a % constant(0x100)) ^ (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);
}
