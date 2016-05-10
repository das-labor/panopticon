/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015 Marcus Brinkmann
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

use std::convert::Into;
use {
    Rvalue,
    Lvalue,
    CodeGen,
    State,
    Guard,
};
use mos::*;

pub fn nop(_: &mut CodeGen<Mos>) {}

pub fn nop_r(_: &mut CodeGen<Mos>, _: Rvalue) {}

pub fn adc(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        zext/8 carry:8, C:1;
        add res:8, A:8, (r);
        add res:8, res:8, carry:8;

        cmpeq Z:1, [0]:8, res:8;

        cmpleu c1:1, res:8, A:8;
        cmpeq c2:1, res:8, A:8;
        and c2:1, c2:1, C:1;
        and C:1, c1:1, c2:1;

        cmples N:1, res:8, [0]:8;

        cmples v1:1, res:8, A:8;
        cmpeq v2:1, res:8, A:8;
        and v2:1, v2:1, C:1;
        and V:1, v1:1, v2:1;

        mov A:8, res:8;
    }
    /*
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
    cg.equal_i(&*Z, &A.to_rv(), &0);*/
}

pub fn and(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        and A:8, A:8, (r);
        cmpeq Z:1, A:8, [0]:8;
        cmples N:1, A:8, [0]:8;
    }
}

pub fn asl(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    rreil!{cg:
        mov C:1, A:1/7;
        shl A:8, A:8, [1]:8;
        cmpeq Z:1, A:8, [0]:8;
        cmples N:1, A:8, [0]:8;
    }
}

pub fn bit(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        and res:8, A:8, (r);
        cmpeq Z:1, res:8, [0]:8;
        cmples N:1, res:8, [0]:8;
        mov V:1, res:1/7;
    }
}


pub fn brk(_: &mut CodeGen<Mos>) {
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
    rreil!{cg:
        mov C:1, [0]:1;
    }
}

pub fn cli(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        mov I:1, [0]:1;
    }
}

pub fn cld(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        mov D:1, [0]:1;
    }
}

pub fn sec(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        mov C:1, [1]:1;
    }
}

pub fn sei(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        mov I:1, [1]:1;
    }
}

pub fn clv(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        mov V:1, [0]:1;
    }
}

pub fn sed(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        mov D:1, [1]:1;
    }
}

fn cmp(cg: &mut CodeGen<Mos>, r1: Rvalue, r2: Rvalue) {
    rreil!{cg:
        cmpltu C:1, (r1), (r2);
        mov N:1, C:1;
        cmpeq Z:1, (r1), (r2);
    }
}

pub fn cpx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cmp(cg, rreil_rvalue!{ X:8 },r)
}

pub fn cpy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cmp(cg, rreil_rvalue!{ Y:8 },r)
}

pub fn cpa(cg: &mut CodeGen<Mos>, r: Rvalue) {
    cmp(cg, rreil_rvalue!{ A:8 },r)
}

fn dec(cg: &mut CodeGen<Mos>, l: Lvalue, r: Rvalue) {
    rreil!{cg:
        sub (l), (r), [1]:8;
        cmpeq Z:1, (l), [0]:8;
        cmplts N:1, (r), [0]:8;
    }
}

pub fn dea(cg: &mut CodeGen<Mos>, r: Rvalue) {
    dec(cg, rreil_lvalue!{ A:8 }, r)
}

pub fn dex(cg: &mut CodeGen<Mos>) {
    dec(cg, rreil_lvalue!{ X:8 }, rreil_rvalue!{ X:8 })
}

pub fn dey(cg: &mut CodeGen<Mos>) {
    dec(cg, rreil_lvalue!{ Y:8 }, rreil_rvalue!{ Y:8 })
}

pub fn eor(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        xor A:8, (r), A:8;
        cmpeq Z:1, A:8, [0]:8;
        cmplts N:1, A:8, [0]:8;
    }
}

fn inc(cg: &mut CodeGen<Mos>, l: Lvalue, r: Rvalue) {
    rreil!{cg:
        add (l), (r), [1]:8;
        cmpeq Z:1, (l), [0]:8;
        cmplts N:1, (l), [0]:8;
    }
}

pub fn ina(cg: &mut CodeGen<Mos>, r: Rvalue) {
    inc(cg, rreil_lvalue!{ A:8 },r)
}

pub fn inx(cg: &mut CodeGen<Mos>) {
    inc(cg, rreil_lvalue!{ X:8 }, rreil_rvalue!{ X:8 })
}

pub fn iny(cg: &mut CodeGen<Mos>) {
    inc(cg, rreil_lvalue!{ Y:8 }, rreil_rvalue!{ Y:8 })
}

fn ld(cg: &mut CodeGen<Mos>, l: Lvalue, r: Rvalue) {
    rreil!{cg:
        mov (l), (r);
        cmpeq Z:1, (l), [0]:8;
        cmplts N:1, (l), [0]:8;
    }
}

pub fn lda(cg: &mut CodeGen<Mos>, r: Rvalue) {
    ld(cg, rreil_lvalue!{ A:8 }, r)
}

pub fn ldx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    ld(cg, rreil_lvalue!{ X:8 }, r)
}

pub fn ldy(cg: &mut CodeGen<Mos>, r: Rvalue) {
    ld(cg, rreil_lvalue!{ Y:8 }, r)
}

pub fn lsr(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        mov C:1, A:1;
        shl A:8, A:8, (r);
        cmpeq Z:1, A:8, [0]:8;
        mov N:1, [0]:1;
    }
}

pub fn ora(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        or A:8, (r), A:8;
        cmpeq Z:1, A:8, [0]:8;
        cmplts N:1, A:8, [0]:8;
    }
}

pub fn pha(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        zext/9 sp:9, S:8;
        add sp:9, sp:9, [0x100]:9;

        store/ram sp:9, A:8;

        add sp:9, sp:9, [1]:9;
        mov S:8, sp:8;
    }
}

pub fn php(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        zext/9 sp:9, S:8;
        add sp:9, sp:9, [0x100]:9;

        zext/8 flags:8, C:1;
        mov flags:1/1, Z:1;
        mov flags:1/2, I:1;
        mov flags:1/3, D:1;
        mov flags:1/4, B:1;
        mov flags:1/5, ?;
        mov flags:1/6, V:1;
        mov flags:1/7, N:1;

        store/ram sp:9, flags:8;
        add sp:9, sp:9, [1]:9;
        mov S:8, sp:8;
    }
}

pub fn pla(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        zext/9 sp:9, S:8;
        add sp:9, sp:9, [0x100]:9;

        add sp:9, sp:9, [1]:9;
        load/ram A:8, sp:9;

        mov S:8, sp:8;

        cmpeq Z:1, A:8, [0]:8;
        cmplts N:1, A:8, [0]:8;
    }
}

pub fn plp(cg: &mut CodeGen<Mos>) {
    rreil!{cg:
        zext/9 sp:9, S:8;
        add sp:9, sp:9, [0x100]:9;

        add sp:9, sp:9, [1]:9;
        load/ram flags:8, sp:9;

        mov C:1, flags:1;
        mov Z:1, flags:1/1;
        mov I:1, flags:1/2;
        mov D:1, flags:1/3;
        mov V:1, flags:1/6;
        mov N:1, flags:1/7;

        mov S:8, sp:8;

        cmpeq Z:1, A:8, [0]:8;
        cmplts N:1, A:8, [0]:8;
    }
}


pub fn rol(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(_r).unwrap();
    rreil!{cg:
        mov hb:1, (r.extract(1,7).unwrap());
        shl (r), (r), [1]:8;
        mov (r.extract(1,7).unwrap()), C:1;
        mov C:1, hb:1;
        cmpeq Z:1, (r), [0]:8;
        cmples N:1, (r), [0]:8;
    }
}

pub fn ror(cg: &mut CodeGen<Mos>, _r: Rvalue) {
    let r = Lvalue::from_rvalue(_r).unwrap();
    rreil!{cg:
        mov lb:1, (r.extract(1,0).unwrap());
        shr (r), (r), [1]:8;
        mov (r.extract(1,7).unwrap()), C:1;
        mov C:1, lb:1;
        cmpeq Z:1, (r), [0]:8;
        cmples N:1, (r), [0]:8;
    }
}

pub fn rts(_: &mut CodeGen<Mos>) {
    /* FIXME: Pop PC-1 from stack (so that the next instruction is fetched
       from TOS+1 */
}


pub fn sbc(cg: &mut CodeGen<Mos>, r: Rvalue) {
    rreil!{cg:
        zext/8 carry:8, C:1;
        sub res:8, A:8, (r);
        add res:8, res:8, carry:8;

        cmpeq Z:1, [0]:8, res:8;

        cmpleu c1:1, res:8, A:8;
        cmpeq c2:1, res:8, A:8;
        and c2:1, c2:1, C:1;
        and C:1, c1:1, c2:1;

        cmples N:1, res:8, [0]:8;

        cmples v1:1, res:8, A:8;
        cmpeq v2:1, res:8, A:8;
        and v2:1, v2:1, C:1;
        and V:1, v1:1, v2:1;

        mov A:8, res:8;
    }
    /*
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
    cg.equal_i(&*Z, &A.to_rv(), &0);*/
}

fn st(cg: &mut CodeGen<Mos>, reg: Lvalue, ptr: Rvalue) {
    rreil!{cg:
        store/ram (reg), (ptr);
    }
}

pub fn sta(cg: &mut CodeGen<Mos>, r: Rvalue) {
    st(cg, rreil_lvalue!{ A:8 }, r)
}

pub fn stx(cg: &mut CodeGen<Mos>, r: Rvalue) {
    st(cg, rreil_lvalue!{ X:8 }, r)
}

pub fn sty(cg: &mut CodeGen<Mos>, r: Rvalue) {
    st(cg, rreil_lvalue!{ Y:8 }, r)
}

pub fn trr(cg: &mut CodeGen<Mos>, src: &Lvalue, dst: &Lvalue) {
    rreil!{cg:
        mov (dst), (src);
        cmpeq Z:1, (dst), [0]:8;
        cmplts N:1, (dst), [0]:8;
    }
}

pub fn tax(cg: &mut CodeGen<Mos>) {
    trr(cg, &A, &X);
}

pub fn tay(cg: &mut CodeGen<Mos>) {
    trr(cg, &A, &Y);
}

pub fn tsx(cg: &mut CodeGen<Mos>) {
    trr(cg, &SP, &X);
}

pub fn txa(cg: &mut CodeGen<Mos>) {
    trr(cg, &X, &A);
}

pub fn txs(cg: &mut CodeGen<Mos>) {
    trr(cg, &X, &SP);
}

pub fn tya(cg: &mut CodeGen<Mos>) {
    trr(cg, &Y, &A);
}

pub fn jmp_direct(st: &mut State<Mos>) -> bool {
    let next = Rvalue::new_u16(st.get_group("immlo") as u16 | ((st.get_group("immhi") as u16) << 8));

    st.mnemonic(3,"jmp","{c:ram}",vec![next.clone()],&|_: &mut CodeGen<Mos>| {});
    st.jump(next,Guard::always());

    true
}

pub fn jmp_indirect(st: &mut State<Mos>) -> bool {
    let ptr = Rvalue::new_u16(st.get_group("immlo") as u16 | ((st.get_group("immhi") as u16) << 8));

    st.mnemonic(0,"__fetch","",vec![],&|cg: &mut CodeGen<Mos>| {
        rreil!{cg:
            load/ram res:16, (ptr);
        }
    });

    let next = rreil_rvalue!{ res:16 };

    st.mnemonic(3,"jmp","{p:ram}",vec![ptr.clone()],&|_: &mut CodeGen<Mos>| {});
    st.jump(next,Guard::always());

    true
}

pub fn jsr(st: &mut State<Mos>) -> bool {
    let next = Rvalue::new_u16(st.address as u16 + 3);
    let target = Rvalue::new_u16(st.get_group("immlo") as u16 | ((st.get_group("immhi") as u16) << 8));

    st.mnemonic(3,"jsr","{c:ram}",vec![target.clone()],&|cg: &mut CodeGen<Mos>| {
        rreil!{cg:
            call ?, (target);
        }
    });
    st.jump(next,Guard::always());
    true
}
