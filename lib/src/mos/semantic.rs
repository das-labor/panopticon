/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
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

use value::{Rvalue,Lvalue,ToRvalue};
use codegen::CodeGen;
use mos::*;

pub fn _sp(cg: &mut CodeGen<Mos>) -> Lvalue {
    let sp = new_temp(16);
    cg.add_i(&sp, &SP.to_rv(), &0x100);
    ram(&sp, 8)
}

fn _push(cg: &mut CodeGen<Mos>, v: &Rvalue) {
    let sp = _sp(cg);
    cg.assign(&sp, v);
    cg.sub_i(&*SP, &SP.to_rv(), &1);
}

fn _pop(cg: &mut CodeGen<Mos>, dst: &Lvalue) {
    let sp = _sp(cg);
    cg.assign(dst, &sp.to_rv());
    cg.add_i(&*SP, &SP.to_rv(), &1);
}

fn _pushf<A: ToRvalue>(cg: &mut CodeGen<Mos>, b: &A) {
  let flags = new_temp(8);

  cg.add_i(&flags, &flags, &*N);
  cg.lshift_i(&flags, &flags, &1);
  cg.add_i(&flags, &flags, &*V);
  cg.lshift_i(&flags, &flags, &1);

  cg.add_i(&flags, &flags, &1); // Unused bit is always 1
  cg.lshift_i(&flags, &flags, &1);
  cg.add_i(&flags, &flags, &b.to_rv()); // B is always 1 through php
  cg.lshift_i(&flags, &flags, &1);

  cg.add_i(&flags, &flags, &*D);
  cg.lshift_i(&flags, &flags, &1);
  cg.add_i(&flags, &flags, &*I);
  cg.lshift_i(&flags, &flags, &1);
  cg.add_i(&flags, &flags, &*Z);
  cg.lshift_i(&flags, &flags, &1);
  cg.add_i(&flags, &flags, &*C);

  _push(cg, &flags.to_rv());
}

fn _set_nz(cg: &mut CodeGen<Mos>, r: &Rvalue) {
    cg.less_i(&*N, &0x7f, r);
    cg.equal_i(&*Z, r, &0);
}


fn _izx(cg: &mut CodeGen<Mos>, r: &Rvalue) -> Lvalue {
    let lo_addr = new_temp(8);
    let hi_addr = new_temp(8);

    cg.assign(&lo_addr, r);
    cg.add_i(&lo_addr, &lo_addr.to_rv(), &X.to_rv());
    /* Many emulators get this wrong.  Wrap around the zero page, always.
       Confirmed with Visual 6502 (transistor simulator).  */
    cg.add_i(&hi_addr, &lo_addr.to_rv(), &1);

    let lo = ram(&lo_addr, 8);
    let hi = ram(&hi_addr, 8);
    let addr = new_temp(16);

    cg.assign(&addr, &hi.to_rv());
    cg.lshift_i(&addr, &addr, &8);
    cg.add_i(&addr, &addr, &lo.to_rv());

    ram(&addr, 8)
}


fn _izy(cg: &mut CodeGen<Mos>, r: &Rvalue) -> Lvalue {
    let lo_addr = new_temp(8);
    let hi_addr = new_temp(8);

    cg.assign(&lo_addr, r);
    /* Possibly wrap.  */
    cg.add_i(&hi_addr, &lo_addr.to_rv(), &1);

    let lo = ram(&lo_addr, 8);
    let hi = ram(&hi_addr, 8);
    let addr = new_temp(16);

    cg.assign(&addr, &hi.to_rv());
    cg.lshift_i(&addr, &addr, &8);
    cg.add_i(&addr, &addr, &lo.to_rv());

    cg.add_i(&addr, &addr.to_rv(), &Y.to_rv());

    ram(&addr, 8)
}


fn _zpi(cg: &mut CodeGen<Mos>, r: &Rvalue, o: &Rvalue) -> Lvalue {
    let addr = new_temp(16);
    if let &Rvalue::Memory{offset: ref _addr,..} = r {
        cg.add_i(&addr, &**_addr, o);
        cg.and_i(&addr, &addr.to_rv(), &0xff);
    } else {
        panic!("this is a terrible mistake!");
    }
    ram(&addr, 8)
}


fn _idx(cg: &mut CodeGen<Mos>, r: &Rvalue, o: &Rvalue) -> Lvalue {
    let addr = new_temp(16);
    if let &Rvalue::Memory{offset: ref _addr,..} = r {
        cg.add_i(&addr, &**_addr, o);
    } else {
        panic!("this is a terrible mistake!");
    }
    ram(&addr, 8)
}

fn _select(cg: &mut CodeGen<Mos>, a: &Lvalue, v1: &Rvalue, v2: &Rvalue, flag: &Rvalue)
{
    // res = f ? v2 : v1 = f*v1+(1-f)*v2 = f*(v1-v2) + v2
    cg.assign(a, v1);
    cg.sub_i(a, &a.to_rv(), v2);
    cg.mul_i(a, &a.to_rv(), flag);
    cg.add_i(a, &a.to_rv(), v2);
}


pub fn nop(_: &mut CodeGen<Mos>) {}

pub fn nop_r(_: &mut CodeGen<Mos>, _: Rvalue) {}

pub fn nop_rr(_: &mut CodeGen<Mos>, _: Rvalue, _: Rvalue) {}


pub fn adc(cg: &mut CodeGen<Mos>, r: Rvalue) {
    // This will contain our result.  Bit 8 is carry.
    let result = new_temp(16);
    let result_c6 = new_temp(8);
    let result_n = new_temp(8);

    // Decimal mode is a flag.  So we have to calculate both values and _select the right one.
    let normal = new_temp(16);
    let normal_c6 = new_temp(8);
    let normal_n = new_temp(8);
    let decimal = new_temp(16);
    let decimal_c6 = new_temp(8);
    let decimal_n = new_temp(8);

    // These two are used for c6 calculations (overflow).
    // V = C6 xor C7 (carry).  We get C6 by blanking out the top bit.
    let _v1 = new_temp(8);
    let _v2 = new_temp(8);

    // Normal mode.
    cg.assign(&normal, &*A);
    cg.add_i(&normal, &normal.to_rv(), &r);
    cg.add_i(&normal, &normal.to_rv(), &C.to_rv());
    cg.rshift_i(&normal_n, &normal.to_rv(), &7);
    cg.and_i(&normal_n, &normal_n.to_rv(), &1);

    cg.and_i(&_v1, &A.to_rv(), &0x7f);
    cg.and_i(&_v2, &r, &0x7f);
    cg.add_i(&_v1, &_v1.to_rv(), &_v2.to_rv());
    cg.add_i(&_v1, &_v1.to_rv(), &C.to_rv());
    cg.rshift_i(&normal_c6, &_v1.to_rv(), &7);


    // Decimal mode.  It's complicated: http://www.6502.org/tutorials/decimal_mode.html

    // 1a. Decimal
    let al = new_temp(8);
    cg.assign(&al, &*A);
    cg.and_i(&al, &al.to_rv(), &0xf);

    let lo = new_temp(8);
    cg.assign(&lo, &r);
    cg.and_i(&lo, &lo.to_rv(), &0xf);

    cg.add_i(&al, &al.to_rv(), &lo);
    cg.add_i(&al, &al.to_rv(), &C.to_rv());

    // 1b. We have now al = (A & $0F) + (R & $0F) + C <= 0x1f and have to compare to >= 0x0a.
    let adjust = new_temp(8);
    cg.add_i(&adjust, &al.to_rv(), &0xe6);        // -a in 2-complement
    cg.rshift_i(&adjust, &adjust.to_rv(), &7);    // N bit means >= 0x0a

    let adjusted = new_temp(8);
    cg.assign(&adjusted, &al.to_rv());
    cg.add_i(&adjusted, &adjusted.to_rv(), &6);
    cg.and_i(&adjusted, &adjusted.to_rv(), &0xf);
    cg.or_i(&adjusted, &adjusted.to_rv(), &0x10);

    _select(cg, &lo, &al.to_rv(), &adjusted.to_rv(), &adjust.to_rv());
    cg.assign(&al, &lo.to_rv());

    // 1c.
    let _decimal = new_temp(16);
    cg.and_i(&_decimal, &A.to_rv(), &0xf0);
    cg.add_i(&_decimal, &_decimal.to_rv(), &r);
    cg.and_i(&_decimal, &_decimal.to_rv(), &0x1f0);
    cg.add_i(&_decimal, &_decimal.to_rv(), &al.to_rv());

    // In decimal mode, the negative flag is the 8th bit of the previous addition (1c).
    cg.rshift_i(&decimal_n, &_decimal.to_rv(), &7);
    cg.and_i(&decimal_n, &decimal_n.to_rv(), &1);

    // In decimal mode, the overflow flag is the C6 of the previous addition (1c).
    cg.and_i(&_v1, &A.to_rv(), &0x70);
    cg.and_i(&_v2, &r, &0x70);
    cg.add_i(&_v1, &_v1.to_rv(), &_v2.to_rv());
    cg.add_i(&_v1, &_v1.to_rv(), &al.to_rv());
    cg.rshift_i(&decimal_c6, &_v1.to_rv(), &7);

    // 1e. Compare to 0xa0.  Note that _decimal is max. 0x1ff (because al is max. 0x1f)
    let hiadjust = new_temp(16);
    cg.add_i(&hiadjust, &_decimal.to_rv(), &0xfe60);  // -a0 in 2-complement
    cg.rshift_i(&hiadjust, &hiadjust.to_rv(), &15);   // N bit means > 0xa0.  This is also the new carry!

    let hiadjusted = new_temp(16);
    cg.assign(&adjusted, &_decimal.to_rv());
    cg.add_i(&hiadjusted, &hiadjusted.to_rv(), &0x60);
    cg.and_i(&hiadjusted, &hiadjusted.to_rv(), &0xff);
    cg.or_i(&hiadjusted, &hiadjusted.to_rv(), &0x100); // Set new carry.

    _select(cg, &decimal, &_decimal.to_rv(), &hiadjusted.to_rv(), &hiadjust.to_rv());

    // Finally, select the result that is actually used.
    _select(cg, &result, &normal.to_rv(), &decimal.to_rv(), &D.to_rv());
    _select(cg, &result_c6, &normal_c6.to_rv(), &decimal_c6.to_rv(), &D.to_rv());
    _select(cg, &result_n, &normal_n.to_rv(), &decimal_n.to_rv(), &D.to_rv());

    // Output all results.
    cg.assign(&*A, &result.to_rv());
    cg.rshift_i(&*C, &result.to_rv(), &8);
    cg.assign(&*N, &result_n.to_rv());
    cg.xor_i(&*V, &result_c6.to_rv(), &C.to_rv());
    cg.equal_i(&*Z, &A.to_rv(), &0);
}

pub fn adc_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    adc(cg, addr.to_rv());
}

pub fn adc_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    adc(cg, addr.to_rv());
}

pub fn adc_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    adc(cg, addr.to_rv());
}

pub fn adc_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    adc(cg, addr.to_rv());
}


pub fn and(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.and_i(&*A, &A.to_rv(), &r);
    _set_nz(cg, &A.to_rv());
}

pub fn and_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    and(cg, addr.to_rv());
}

pub fn and_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    and(cg, addr.to_rv());
}

pub fn and_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    and(cg, addr.to_rv());
}

pub fn and_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    and(cg, addr.to_rv());
}


pub fn asl(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();

    cg.rshift_i(&*C, &r.to_rv(), &7);
    cg.lshift_i(&*A, &r.to_rv(), &1);
    _set_nz(cg, &A.to_rv());
}

pub fn asl_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    asl(cg, addr.to_rv());
}

pub fn asl_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    asl(cg, addr.to_rv());
}


pub fn bit(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.rshift_i(&*N, &r, &7);
    cg.rshift_i(&*V, &r, &6);

    let _and = new_temp(8);
    cg.and_i(&_and, &A.to_rv(), &r);
    cg.equal_i(&*Z, &_and.to_rv(), &0);
}


pub fn brk(cg: &mut CodeGen<Mos>, r: Rvalue) {
    /* Well.  We could simulate BRK up to the indirect jump at the NMI vector.
       So we add the code to do that here.  But without the ROM, this is useless
       (and with user-provided NMI handlers it would be very dynamic).
       For now, it seems simpler to just ignore the BRK instruction and all its
       side effects.  */
/*
    let reg = new_temp(8);
    cg.assign(&reg, &PC.to_rvalue());
    _push(cg, &reg.to_rv());
    cg.rshift_i(&pc, &PC.to_rvalue(), &8);
    _push(cg, &reg.to_rv());
    _pushf(cg, &0);
*/
}

pub fn clc(cg: &mut CodeGen<Mos>) {
    cg.assign(&*C, &Rvalue::Constant(0));
}

pub fn cli(cg: &mut CodeGen<Mos>) {
    cg.assign(&*I, &Rvalue::Constant(0));
}

pub fn cld(cg: &mut CodeGen<Mos>) {
    cg.assign(&*D, &Rvalue::Constant(0));
}

pub fn sec(cg: &mut CodeGen<Mos>) {
    cg.assign(&*C, &Rvalue::Constant(1));
}

pub fn sei(cg: &mut CodeGen<Mos>) {
    cg.assign(&*I, &Rvalue::Constant(0));
}

pub fn clv(cg: &mut CodeGen<Mos>) {
    cg.assign(&*V, &Rvalue::Constant(0));
}

pub fn sed(cg: &mut CodeGen<Mos>) {
    cg.assign(&*D, &Rvalue::Constant(1));
}


fn _cmp(cg: &mut CodeGen<Mos>, reg: &Rvalue, r: Rvalue) {
    let result = new_temp(16);

    cg.xor_i(&result, &r, &0xff);
    cg.add_i(&result, &result.to_rv(), reg);
    cg.add_i(&result, &result.to_rv(), &1);

    // Common results.
    cg.rshift_i(&*C, &result.to_rv(), &8);
    cg.and_i(&result, &result.to_rv(), &0xff);
    _set_nz(cg, &result.to_rv());
}

pub fn cmp(cg: &mut CodeGen<Mos>, r: Rvalue)
{
    _cmp(cg, &A.to_rv(), r);
}

pub fn cmp_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    cmp(cg, addr.to_rv());
}

pub fn cmp_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    cmp(cg, addr.to_rv());
}

pub fn cmp_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    cmp(cg, addr.to_rv());
}

pub fn cmp_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    cmp(cg, addr.to_rv());
}

pub fn cpx(cg: &mut CodeGen<Mos>, r: Rvalue)
{
    _cmp(cg, &X.to_rv(), r);
}

pub fn cpy(cg: &mut CodeGen<Mos>, r: Rvalue)
{
    _cmp(cg, &Y.to_rv(), r);
}


fn _dec(cg: &mut CodeGen<Mos>, r: &Lvalue)
{
    cg.sub_i(r, &r.to_rv(), &1);
    _set_nz(cg, &r.to_rv());
}

pub fn dec(cg: &mut CodeGen<Mos>, _r: Rvalue)
{
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _dec(cg, &r);
}

pub fn dec_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    dec(cg, addr.to_rv());
}

pub fn dec_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    dec(cg, addr.to_rv());
}

pub fn dex(cg: &mut CodeGen<Mos>) {
    _dec(cg, &X);
}

pub fn dey(cg: &mut CodeGen<Mos>) {
    _dec(cg, &Y);
}


pub fn eor(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.xor_i(&*A, &A.to_rv(), &r);
    _set_nz(cg, &A.to_rv());
}

pub fn eor_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    eor(cg, addr.to_rv());
}

pub fn eor_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    eor(cg, addr.to_rv());
}

pub fn eor_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    eor(cg, addr.to_rv());
}

pub fn eor_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    eor(cg, addr.to_rv());
}


fn _inc(cg: &mut CodeGen<Mos>, r: &Lvalue)
{
    cg.add_i(r, &r.to_rv(), &1);
    _set_nz(cg, &r.to_rv());
}

pub fn inc(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _inc(cg, &r);
}
  
pub fn inc_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    inc(cg, addr.to_rv());
}

pub fn inc_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    inc(cg, addr.to_rv());
}

pub fn inx(cg: &mut CodeGen<Mos>) {
    _inc(cg, &X);
}

pub fn iny(cg: &mut CodeGen<Mos>) {
    _inc(cg, &Y);
}


pub fn jsr(cg: &mut CodeGen<Mos>, r: Rvalue) {
    /* FIXME: Push PC-1 to stack.  */
}


pub fn lda(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.assign(&*A, &r);
    _set_nz(cg, &A.to_rv());
}

pub fn lda_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    lda(cg, addr.to_rv());
}

pub fn lda_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    lda(cg, addr.to_rv());
}

pub fn lda_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    lda(cg, addr.to_rv());
}

pub fn lda_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    lda(cg, addr.to_rv());
}


pub fn ldx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.assign(&*X, &r);
    _set_nz(cg, &X.to_rv());
}

pub fn ldx_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    ldx(cg, addr.to_rv());
}

pub fn ldx_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    ldx(cg, addr.to_rv());
}


pub fn ldy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.assign(&*Y, &r);
    _set_nz(cg, &Y.to_rv());
}

pub fn ldy_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    ldy(cg, addr.to_rv());
}

pub fn ldy_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    ldy(cg, addr.to_rv());
}


pub fn lsr(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();

    cg.assign(&*C, &r.to_rv());
    cg.rshift_i(&*A, &r.to_rv(), &1);
    _set_nz(cg, &A.to_rv());
}

pub fn lsr_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    lsr(cg, addr.to_rv());
}

pub fn lsr_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    lsr(cg, addr.to_rv());
}


pub fn ora(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cg.or_i(&*A, &A.to_rv(), &r);
    _set_nz(cg, &A.to_rv());
}

pub fn ora_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    ora(cg, addr.to_rv());
}

pub fn ora_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    ora(cg, addr.to_rv());
}

pub fn ora_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    ora(cg, addr.to_rv());
}

pub fn ora_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    ora(cg, addr.to_rv());
}


pub fn pha(cg: &mut CodeGen<Mos>) {
  _push(cg, &A.to_rv());
}

pub fn php(cg: &mut CodeGen<Mos>) {
  _pushf(cg, &1);
}

pub fn pla(cg: &mut CodeGen<Mos>) {
  _pop(cg, &*A);
  _set_nz(cg, &A.to_rv());
}

pub fn plp(cg: &mut CodeGen<Mos>) {
  let flags = new_temp(8);
  _pop(cg, &flags);

  cg.and_i(&*C, &flags.to_rv(), &1);
  cg.rshift_i(&flags, &flags, &1);
  cg.and_i(&*Z, &flags.to_rv(), &1);
  cg.rshift_i(&flags, &flags, &1);
  cg.and_i(&*I, &flags.to_rv(), &1);
  cg.rshift_i(&flags, &flags, &1);
  cg.and_i(&*D, &flags.to_rv(), &1);
  cg.rshift_i(&flags, &flags, &3); // B and unused are ignored.

  cg.and_i(&*V, &flags.to_rv(), &1);
  cg.rshift_i(&flags, &flags, &1);
  cg.and_i(&*N, &flags.to_rv(), &1);
}


fn _rol(cg: &mut CodeGen<Mos>, r: &Lvalue)
{
    let c = new_temp(8);
    cg.rshift_i(&c, &r.to_rv(), &7);
    cg.lshift_i(&r, &r.to_rv(), &1);
    cg.or_i(&r, &r.to_rv(), &C.to_rv());
    cg.assign(&*C, &c.to_rv());
    _set_nz(cg, &r.to_rv());
}

pub fn rol(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _rol(cg, &r);
}
  
pub fn rol_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    rol(cg, addr.to_rv());
}

pub fn rol_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    rol(cg, addr.to_rv());
}


fn _ror(cg: &mut CodeGen<Mos>, r: &Lvalue)
{
    let c = new_temp(8);
    cg.lshift_i(&c, &C.to_rv(), &7); /* FIXME: Can PLI do this?  */
    cg.assign(&*C, &r.to_rv());
    cg.rshift_i(&r, &r.to_rv(), &1);
    cg.or_i(&r, &r.to_rv(), &c.to_rv());
    _set_nz(cg, &r.to_rv());
}

pub fn ror(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _ror(cg, &r);
}
  
pub fn ror_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    ror(cg, addr.to_rv());
}

pub fn ror_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    ror(cg, addr.to_rv());
}


pub fn rts(cg: &mut CodeGen<Mos>) {
    /* FIXME: Pop PC-1 from stack (so that the next instruction is fetched
       from TOS+1 */
}


pub fn sbc(cg: &mut CodeGen<Mos>, r: Rvalue) {
    // This will contain our result.  Bit 8 is carry.
    let result = new_temp(16);
    let result_c = new_temp(8);
    let result_v = new_temp(8);
    let result_n = new_temp(8);

    // Decimal mode is a flag.  So we have to calculate both values and _select the right one.
    let normal = new_temp(16);
    let _addend = new_temp(8);
    let decimal = new_temp(16);

    // These two are used for c6 calculations (overflow).
    // V = C6 xor C7 (carry).  We get C6 by blanking out the top bit.
    let _v1 = new_temp(8);
    let _v2 = new_temp(8);

    // Normal mode.  Same as adding 255-r.
    cg.assign(&normal, &*A);
    cg.xor_i(&_addend, &r, &0xff);
    cg.add_i(&normal, &normal.to_rv(), &_addend.to_rv());
    cg.add_i(&normal, &normal.to_rv(), &C.to_rv());
 
    // Common results.
    cg.rshift_i(&result_c, &normal.to_rv(), &8);
    cg.rshift_i(&result_n, &normal.to_rv(), &7);

    cg.and_i(&_v1, &A.to_rv(), &0x7f);
    cg.and_i(&_v2, &_addend.to_rv(), &0x7f);
    cg.add_i(&_v1, &_v1.to_rv(), &_v2.to_rv());
    cg.add_i(&_v1, &_v1.to_rv(), &C.to_rv());
    cg.rshift_i(&result_v, &_v1.to_rv(), &7);
    cg.xor_i(&result_v, &result_v.to_rv(), &result_c.to_rv());

    // Decimal mode.  It's complicated: http://www.6502.org/tutorials/decimal_mode.html

    // FIXME

    // 1a. Decimal
    let al = new_temp(8);
    cg.assign(&al, &*A);
    cg.and_i(&al, &al.to_rv(), &0xf);

    let lo = new_temp(8);
    cg.assign(&lo, &r);
    cg.and_i(&lo, &lo.to_rv(), &0xf);

    cg.sub_i(&al, &al.to_rv(), &lo);
    cg.add_i(&al, &al.to_rv(), &C.to_rv());
    cg.sub_i(&al, &al.to_rv(), &1);

    // 1b. We have now al = (A & $0F) - (R & $0F) - 1 + C and have to compare to < 0.
    let adjust = new_temp(8);
    cg.rshift_i(&adjust, &al.to_rv(), &7);    // N bit means < 0

    let adjusted = new_temp(8);
    cg.assign(&adjusted, &al.to_rv());
    cg.sub_i(&adjusted, &adjusted.to_rv(), &6);
    cg.and_i(&adjusted, &adjusted.to_rv(), &0xf);
    cg.sub_i(&adjusted, &adjusted.to_rv(), &0x10);

    _select(cg, &lo, &al.to_rv(), &adjusted.to_rv(), &adjust.to_rv());
    cg.assign(&al, &lo.to_rv());

    // 1c.
    let _decimal = new_temp(16);
    cg.and_i(&_decimal, &A.to_rv(), &0xf0);
    cg.sub_i(&_decimal, &_decimal.to_rv(), &r);
    cg.add_i(&_decimal, &_decimal.to_rv(), &0x10); // Or sub r&0xf0 instead.
    cg.and_i(&_decimal, &_decimal.to_rv(), &0xfff0);
    cg.add_i(&_decimal, &_decimal.to_rv(), &al.to_rv());

    // 1e. Compare to 0.
    let hiadjust = new_temp(16);
    cg.rshift_i(&hiadjust, &hiadjust.to_rv(), &15); // N bit means > 0xa0.

    let hiadjusted = new_temp(16);
    cg.assign(&adjusted, &_decimal.to_rv());
    cg.sub_i(&hiadjusted, &hiadjusted.to_rv(), &0x60);
    _select(cg, &decimal, &_decimal.to_rv(), &hiadjusted.to_rv(), &hiadjust.to_rv());

    // Finally, select the result that is actually used.
    _select(cg, &result, &normal.to_rv(), &decimal.to_rv(), &D.to_rv());

    // Output all results.
    cg.assign(&*A, &result.to_rv());
    cg.assign(&*C, &result_c.to_rv());
    cg.assign(&*V, &result_v.to_rv());
    cg.assign(&*N, &result_n.to_rv());
    cg.equal_i(&*Z, &A.to_rv(), &0);
}

pub fn sbc_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    sbc(cg, addr.to_rv());
}

pub fn sbc_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    sbc(cg, addr.to_rv());
}

pub fn sbc_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    sbc(cg, addr.to_rv());
}

pub fn sbc_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    sbc(cg, addr.to_rv());
}


pub fn _sta(cg: &mut CodeGen<Mos>, r: Lvalue) {
    cg.assign(&r, &A.to_rv());
}

pub fn sta(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _sta(cg, r);
}

pub fn sta_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    _sta(cg, addr);
}

pub fn sta_idx(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _idx(cg, &r, &o);
    _sta(cg, addr);
}

pub fn sta_izx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izx(cg, &r);
    _sta(cg, addr);
}

pub fn sta_izy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    let addr = _izy(cg, &r);
    _sta(cg, addr);
}

pub fn _stx(cg: &mut CodeGen<Mos>, r: Lvalue) {
    cg.assign(&r, &X.to_rv());
}

pub fn stx(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _stx(cg, r);
}

pub fn stx_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    _stx(cg, addr);
}

pub fn _sty(cg: &mut CodeGen<Mos>, r: Lvalue) {
    cg.assign(&r, &Y.to_rv());
}

pub fn sty(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(&_r).unwrap();
    _sty(cg, r);
}

pub fn sty_zpi(cg: &mut CodeGen<Mos>, r: Rvalue, o: Rvalue) {
    let addr = _zpi(cg, &r, &o);
    _sty(cg, addr);
}


pub fn _trr(cg: &mut CodeGen<Mos>, src: &Lvalue, dst: &Lvalue)
{
    cg.assign(dst, &src.to_rv());
    _set_nz(cg, &dst.to_rv());
}

pub fn tax(cg: &mut CodeGen<Mos>) {
    _trr(cg, &A, &X);
}

pub fn tay(cg: &mut CodeGen<Mos>) {
    _trr(cg, &A, &Y);
}

pub fn tsx(cg: &mut CodeGen<Mos>) {
    _trr(cg, &SP, &X);
}

pub fn txa(cg: &mut CodeGen<Mos>) {
    _trr(cg, &X, &A);
}

pub fn txs(cg: &mut CodeGen<Mos>) {
    _trr(cg, &X, &SP);
}

pub fn tya(cg: &mut CodeGen<Mos>) {
    _trr(cg, &Y, &A);
}
