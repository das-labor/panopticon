use value::{Lvalue,Rvalue};
use codegen::CodeGen;
use disassembler::State;
use amd64::{Amd64,Mode,Condition,ss,ds,fs,gs};
use guard::Guard;

fn do_push(v: Rvalue, mode: Mode, _: &mut CodeGen) {
    unimplemented!();
    /*
	variable v = to_variable(_v);
	int const w = v.width() / 8;

	m.assign(memory(rip,w,LittleEndian,"ram"),v);

	switch(mode)
	{
		case amd64_state::RealMode:
			m.assign(to_lvalue(sp),sp + w % 0x10000);
			return;
		case amd64_state::ProtectedMode:
			m.assign(to_lvalue(esp), esp + w % 0x100000000);
			return;
		case amd64_state::LongMode:
			m.assign(to_lvalue(rsp), rsp + w);
			return;
		default:
			throw std::invalid_argument("invalid mode in do_push");
	}
    */
}

fn bitwidth(a: Rvalue) -> usize {
    unimplemented!();/*
	if(is_variable(a))
		return to_variable(a).width();
	else if(is_memory(a))
		return to_memory(a).bytes() * 8;
	else
		throw std::invalid_argument("bitwidth() called with argument that is not a memory ref or variable.");*/
}

/*rvalue po::amd64::sign_ext(rvalue v, unsigned from, unsigned to, _: &mut CodeGen)
{
	using dsl::operator*;

	rvalue sign = v / (1 << (from - 1));
	rvalue rest = v % (1 << (from - 1));

	return (sign * (1 << (to - 1))) + rest;
}

fn set_arithm_flags(rvalue res, rvalue res_half, a: Rvalue, b: Rvalue, _: &mut CodeGen)
{
	size_t const a_w = bitwidth(a);
	rvalue const msb_res = less(res / (1 << (a_w - 1)),1);

	m.assign(to_lvalue(CF),res / Rvalue::Constant(1 << a_w));
	m.assign(to_lvalue(AF), res_half / Rvalue::Constant(0x100));
	m.assign(to_lvalue(SF), msb_res);
	m.assign(to_lvalue(ZF), equal(a, Rvalue::Constant(0)));
	m.assign(to_lvalue(OF), CF ^ SF);

	b: Rvalue0 = res % 2;
	b: Rvalue1 = (res % 4) / 2;
	b: Rvalue2 = (res % 8) / 4;
	b: Rvalue3 = (res % 16) / 8;
	b: Rvalue4 = (res % 32) / 16;
	b: Rvalue5 = (res % 64) / 32;
	b: Rvalue6 = (res % 128) / 64;
	b: Rvalue7 = (res % 256) / 128;
	m.assign(to_lvalue(PF), b0 ^ b1 ^ b2 ^ b3 ^ b4 ^ b5 ^ b6 ^ b7);
}*/

pub fn adc(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	size_t const a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a + (a_w == b_w ? b : sign_ext(b,b_w,a_w,m)) + CF;
	rvalue const res_half = (a % Rvalue::Constant(0x100)) + (b % constant(0x100)) + CF;

	m.assign(to_lvalue(a),res % Rvalue::Constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);*/
}

/*fn flagcomp(_: &mut CodeGen, variable const& flag)
{
}*/

pub fn flagwr(flag: &Lvalue, val: bool) -> Box<Fn(&mut CodeGen)> {
    let f = flag.clone();
    Box::new(move |cg: &mut CodeGen| {
        cg.assign(&f,&Rvalue::Constant(if val { 1 } else { 0 }));
    })
}

pub fn flagcomp(flag: &Lvalue) -> Box<Fn(&mut CodeGen)> {
    let f = flag.clone();
    Box::new(move |cg: &mut CodeGen| {
        cg.not_b(&f,&f);
    })
}

pub fn aaa(_: &mut CodeGen) {
    unimplemented!()/*
	using dsl::operator*;

	rvalue y = al & Rvalue::Constant(0x0f);
	rvalue x = m.or_b(m.not_b(m.or_b(less(y,Rvalue::Constant(9)),equal(y,constant(9)))),AF);

	m.assign(to_lvalue(AF), m.lift_b(x));
	m.assign(to_lvalue(CF), m.lift_b(x));
	m.assign(to_lvalue(ax), (ax + m.lift_b(x) * Rvalue::Constant(0x106)) % constant(0x100));*/
}

pub fn aam(_: &mut CodeGen, a: Rvalue) {
    unimplemented!()/*
	rvalue temp_al = m.assign(to_lvalue(al));

	m.assign(to_lvalue(ah), temp_al / a);
	m.assign(to_lvalue(al), temp_al % a);*/
}

pub fn aad(_: &mut CodeGen, a: Rvalue) {
    unimplemented!()/*
	using dsl::operator*;

	rvalue temp_al = m.assign(to_lvalue(al));
	rvalue temp_ah = m.assign(to_lvalue(ah));

	m.assign(to_lvalue(al), temp_al + temp_ah * a);
	m.assign(to_lvalue(ah), Rvalue::Constant(0));*/
}

pub fn aas(_: &mut CodeGen) {
    unimplemented!()/*
	using dsl::operator*;

	rvalue y = al & Rvalue::Constant(0x0f);
	rvalue x = m.or_b(m.not_b(m.or_b(less(y,Rvalue::Constant(9)),equal(y,constant(9)))),AF);

	m.assign(to_lvalue(AF), m.lift_b(x));
	m.assign(to_lvalue(CF), m.lift_b(x));
	m.assign(to_lvalue(ax), (ax - m.lift_b(x) * Rvalue::Constant(6)) % constant(0x100));
	m.assign(to_lvalue(ah), (ah - m.lift_b(x)) % Rvalue::Constant(0x10));
	m.assign(to_lvalue(al), y);*/
}

pub fn add(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	size_t const a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a + (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) + (b % constant(0x100));

	m.assign(to_lvalue(a),res % Rvalue::Constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn adcx(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	size_t const a_w = bitwidth(a);
	rvalue const res = a + b + CF;

	m.assign(to_lvalue(CF), res / Rvalue::Constant(1 << a_w));
	m.assign(to_lvalue(a),res % Rvalue::Constant(1 << a_w));*/
}

pub fn and(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	unsigned int a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a & (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) & (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn arpl(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}

pub fn bound(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}

pub fn bsf(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator*;

	size_t const a_w = bitwidth(a);
	size_t bit = 0;
	boost::optional<rvalue> prev;

	m.assign(to_lvalue(ZF), equal(Rvalue::Constant(0), b));

	while(bit < a_w)
	{
		rvalue val = (b % (1 << (bit + 1)) / (1 << bit));

		m.assign(to_lvalue(a),Rvalue::Constant(bit + 1) * val);
		if(prev)
			prev = *prev | val;
		else
			prev = val;

		++bit;
	}*/
}

pub fn bsr(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator*;

	size_t const a_w = bitwidth(a);
	size_t bit = a_w - 1;
	boost::optional<rvalue> prev;

	m.assign(to_lvalue(ZF), equal(Rvalue::Constant(0), b));

	do
	{
		rvalue val = (b % (1 << (bit + 1)) / (1 << bit));

		m.assign(to_lvalue(a),Rvalue::Constant(bit + 1) * val);
		if(prev)
			prev = *prev | val;
		else
			prev = val;
	}
	while(bit--);*/
}

pub fn bswap(_: &mut CodeGen, a: Rvalue) {
    unimplemented!()/*
	using dsl::operator*;

	size_t const a_w = bitwidth(a);
	size_t byte = 0;

	rvalue tmp = undefined();

	while(byte < a_w / 8)
	{
		unsigned int lsb = byte * 8;
		unsigned int div = (1 << lsb), mul = (1 << (a_w - byte * 8));

		tmp = tmp + (((a / div) % Rvalue::Constant(0x100)) * mul);
		++byte;
	}

	m.assign(to_lvalue(a),tmp);*/
}

pub fn bt(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator<<;
	rvalue mod = (Rvalue::Constant(1) << (b % constant(bitwidth(a))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());*/
}

pub fn btc(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator<<;
	rvalue mod = (Rvalue::Constant(1) << (b % constant(bitwidth(a))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
	m.assign(to_lvalue(a),a ^ mod);*/
}

pub fn btr(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator<<;
	size_t const a_w = bitwidth(a);
	rvalue mod =  ((Rvalue::Constant(1) << (b % constant(bitwidth(a)))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
	m.assign(to_lvalue(a),(a & (Rvalue::Constant(0xffffffffffffffff) ^ mod)) % constant(1 << a_w));*/
}

pub fn bts(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator<<;
	rvalue mod = (Rvalue::Constant(1) << (b % constant(bitwidth(a))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
	m.assign(to_lvalue(a),a & mod);*/
}

pub fn near_call(cg: &mut CodeGen, a: Rvalue) {
    near_xcall(cg,a,false)
}

pub fn near_rcall(cg: &mut CodeGen, a: Rvalue) {
    near_xcall(cg,a,true)
}

pub fn near_xcall(_: &mut CodeGen, a: Rvalue, rel: bool) {
    unimplemented!()/*
	rvalue new_ip;
	amd64_state::OperandSize op = amd64_state::OpSz_16;

	switch(op)
	{
		case amd64_state::OpSz_64:
		{
			if(rel)
				new_ip = (sign_ext(a,32,64,m) + rip);
			else
				new_ip = sign_ext(a,32,64,m);

			do_push(rip,amd64_state::LongMode,m);
			m.assign(to_lvalue(rip), new_ip);
			m.call_i(new_ip);

			return;
		}
		case amd64_state::OpSz_32:
		{
			if(rel)
				new_ip = (a + eip) % 0x100000000;
			else
				new_ip = a;

			do_push(eip,amd64_state::ProtectedMode,m);
			m.assign(to_lvalue(eip), new_ip);
			m.call_i(new_ip);

			return;
		}
		case amd64_state::OpSz_16:
		{
			if(rel)
				new_ip = (a + eip) % 0x10000;
			else
				new_ip = a % 0x10000;

			do_push(ip,amd64_state::RealMode,m);
			m.assign(to_lvalue(ip), new_ip);
			m.call_i(new_ip);

			return;
		}
		default:
			throw std::invalid_argument("near_call with wrong mode");
	}*/
}

pub fn far_call(cg: &mut CodeGen, a: Rvalue) {
    far_xcall(cg,a,false)
}

pub fn far_rcall(cg: &mut CodeGen, a: Rvalue) {
    far_xcall(cg,a,true)
}

pub fn far_xcall(_: &mut CodeGen, a: Rvalue, rel: bool) {
    unimplemented!()/*
	amd64_state::OperandSize op = amd64_state::OpSz_16;

	switch(op)
	{
		case amd64_state::OpSz_16:
		{
			do_push(cs,amd64_state::RealMode,m);
			do_push(ip,amd64_state::RealMode,m);

			return;
		}
		case amd64_state::OpSz_32:
		{
			do_push(cs,amd64_state::ProtectedMode,m);
			do_push(eip,amd64_state::ProtectedMode,m);

			return;
		}
		case amd64_state::OpSz_64:
		{
			do_push(cs,amd64_state::LongMode,m);
			do_push(rip,amd64_state::LongMode,m);

			return;
		}
		default:
			throw std::invalid_argument("far_call invalid op size");
	}*/
}

pub fn cmov(_: &mut CodeGen, a: Rvalue, b: Rvalue, c: Condition) {
    unimplemented!()/*
	using dsl::operator*;

	auto fun = [&](rvalue f)
	{
		m.assign(to_lvalue(a),b + (m.lift_b(f) * b) + (m.lift_b(m.not_b(f)) * a));
	};

	switch(c)
	{
		case Overflow:    fun(OF); break;
		case NotOverflow: fun(m.not_b(OF)); break;
		case Carry:       fun(CF); break;
		case AboveEqual:  fun(m.not_b(CF)); break;
		case Equal:       fun(ZF); break;
		case NotEqual:    fun(m.not_b(ZF)); break;
		case BelowEqual:  fun(m.or_b(ZF,CF)); break;
		case Above:       fun(m.not_b(m.or_b(ZF,CF))); break;
		case Sign:        fun(SF); break;
		case NotSign:     fun(m.not_b(SF)); break;
		case Parity:      fun(PF); break;
		case NotParity:   fun(m.not_b(PF)); break;
		case Less:        fun(m.or_b(m.and_b(SF,OF),m.and_b(m.not_b(SF),m.not_b(OF)))); break;
		case GreaterEqual:fun(m.or_b(m.and_b(m.not_b(SF),OF),m.and_b(SF,m.not_b(OF)))); break;
		case LessEqual:   fun(m.or_b(ZF,m.or_b(m.and_b(SF,OF),m.and_b(m.not_b(SF),m.not_b(OF))))); break;
		case Greater:     fun(m.or_b(m.not_b(ZF),m.or_b(m.and_b(m.not_b(SF),OF),m.and_b(SF,m.not_b(OF))))); break;
		default:
			throw std::invalid_argument("invalid condition in cmov");
	}*/
}

pub fn cmp(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	unsigned int a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a - (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) - (b % constant(0x100));

	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn cmps(_: &mut CodeGen, aoff: Rvalue, boff: Rvalue) {
    unimplemented!()/*
	using dsl::operator*;
	int bits = 8;

	rvalue const a = memory(aoff,bits / 8,LittleEndian,"ram"), b = memory(boff,bits / 8,LittleEndian,"ram");
	rvalue const res = a - b;
	rvalue const res_half = (a % 0x100) - (b % 0x100);

	set_arithm_flags(res,res_half,a,b,m);

	rvalue off = (bits / 8) * m.lift_b(DF) - (bits / 8) * m.lift_b(m.not_b(DF));

	m.assign(to_lvalue(aoff),aoff + off);
	m.assign(to_lvalue(boff),boff + off);*/
}

pub fn cmpxchg(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	using dsl::operator*;
	a: Rvaluecc = eax;

	rvalue t = equal(a,acc);

	m.assign(to_lvalue(ZF), t);
	m.assign(to_lvalue(a),m.lift_b(t) * b + m.lift_b(m.not_b(t)) * a);
	m.assign(to_lvalue(acc),m.lift_b(t) * acc + m.lift_b(m.not_b(ZF)) * a);*/
}

pub fn or(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	unsigned int a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a | (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) | (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn sbb(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	unsigned int a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a - (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) - (b % constant(0x100)) - CF;

	m.assign(to_lvalue(a),res % Rvalue::Constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn sub(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	unsigned int a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a - (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) - (b % constant(0x100));

	m.assign(to_lvalue(a),res % Rvalue::Constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn xor(_: &mut CodeGen, a: Rvalue, b: Rvalue) {
    unimplemented!()/*
	unsigned int a_w = bitwidth(a), b_w = (is_Rvalue::Constant(b) ? a_w : bitwidth(b));
	rvalue const res = a ^ (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % Rvalue::Constant(0x100)) ^ (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);*/
}

pub fn cmpxchg8b(_: &mut CodeGen, a: Rvalue) {}
pub fn cmpxchg16b(_: &mut CodeGen, a: Rvalue) {}
pub fn cpuid(_: &mut CodeGen) {}

pub fn conv(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn conv2(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn daa(_: &mut CodeGen) {}
pub fn das(_: &mut CodeGen) {}
pub fn dec(_: &mut CodeGen, a: Rvalue) {}
pub fn div(_: &mut CodeGen, a: Rvalue) {}
pub fn enter(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn hlt(_: &mut CodeGen) {}
pub fn idiv(_: &mut CodeGen, a: Rvalue) {}
pub fn imul1(_: &mut CodeGen, a: Rvalue) {}
pub fn imul2(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn imul3(_: &mut CodeGen, a: Rvalue, b: Rvalue, c: Rvalue) {}
pub fn in_(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn icebp(_: &mut CodeGen) {}
pub fn inc(_: &mut CodeGen, a: Rvalue) {}
pub fn ins(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn int(_: &mut CodeGen, a: Rvalue) {}
pub fn into(_: &mut CodeGen) {}

pub fn iret(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn jcc(_: &mut CodeGen,a: Rvalue, c: Condition) {}
pub fn jmp(_: &mut CodeGen,a: Rvalue) {}
pub fn jxz(_: &mut CodeGen,a: Rvalue, b: Rvalue) {}
pub fn lahf(_: &mut CodeGen) {}
pub fn lar(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn lds(cg: &mut CodeGen,a: Rvalue, b: Rvalue) { lxs(cg,a,b,ds.to_rv()) }
pub fn lss(cg: &mut CodeGen,a: Rvalue, b: Rvalue) { lxs(cg,a,b,ss.to_rv()) }
pub fn lfs(cg: &mut CodeGen,a: Rvalue, b: Rvalue) { lxs(cg,a,b,fs.to_rv()) }
pub fn lgs(cg: &mut CodeGen,a: Rvalue, b: Rvalue) { lxs(cg,a,b,gs.to_rv()) }
pub fn lxs(_: &mut CodeGen,a: Rvalue, b: Rvalue, seg: Rvalue) {}
pub fn lea(_: &mut CodeGen,a: Rvalue, b: Rvalue) {}

pub fn leave(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn lodsb(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn lods(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn loop_(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn loope(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn loopne(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn mov(_: &mut CodeGen,a: Rvalue,b: Rvalue) {}
pub fn movbe(_: &mut CodeGen,a: Rvalue,b: Rvalue) {}

pub fn movsb(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn movs(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn movsx(_: &mut CodeGen,a: Rvalue,b: Rvalue) {}
pub fn movzx(_: &mut CodeGen,a: Rvalue,b: Rvalue) {}
pub fn mul(_: &mut CodeGen, a: Rvalue) {}
pub fn neg(_: &mut CodeGen, a: Rvalue) {}
pub fn nop(_: &mut CodeGen) {}
pub fn not(_: &mut CodeGen,a: Rvalue) {}
pub fn out(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}

pub fn outs(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn pop(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn popa(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn popcnt(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn popf(_: &mut CodeGen,a: Rvalue) {}

pub fn push(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn pusha(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn pushf(_: &mut CodeGen,a: Rvalue) {}
pub fn rcl(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn rcr(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn ret(_: &mut CodeGen, a: Rvalue) {}
pub fn retf(_: &mut CodeGen, a: Rvalue) {}
pub fn ror(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn rol(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn sahf(_: &mut CodeGen) {}
pub fn sal(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn salc(_: &mut CodeGen) {}
pub fn sar(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}

pub fn scas(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn setcc(_: &mut CodeGen, a: Rvalue, c: Condition) {}
pub fn shl(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn shr(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn shld(_: &mut CodeGen, a: Rvalue, b: Rvalue, c: Rvalue) {}
pub fn shrd(_: &mut CodeGen, a: Rvalue, b: Rvalue, c: Rvalue) {}

pub fn stos(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);

    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn test(_: &mut CodeGen,a: Rvalue, b: Rvalue) {}
pub fn ud1(_: &mut CodeGen) {}
pub fn ud2(_: &mut CodeGen) {}
pub fn xadd(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
pub fn xchg(_: &mut CodeGen, a: Rvalue, b: Rvalue) {}
