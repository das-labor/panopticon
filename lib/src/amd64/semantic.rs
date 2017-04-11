/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016 Kai Michaelis
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

//! RREIL code generator for Intel x86 and AMD64.
//!
//! This module defines a function for each Intel mnemonic recognized by Panopticon. This function
//! returns RREIL statements implementing the opcode semantics and a `JumpSpec` instance
//! that tells the disassembler where to continue.
//!
//! The RREIL code can be generated using the `rreil!` macro. It returns `Result<Vec<Statement>>`.
//!
//! This modules has some helper functions to make things easier (and results more correct) for the
//! casual contributor. For setting various arithmetic flags use the `set_*_flag` functions. Assign
//! register values using `write_reg`. This makes sure that e.g. EAX, AX, AH and AL are written
//! when RAX is. Also, remember to sign or zero extend input Rvalue instance using `sign_extend`/`zero_extend`. RREIL
//! does not extend values automatically.
//!
//! RREIL has no traps, software interrupts of CPU exceptions, this part of the Intel CPUs can be
//! ignored for now. Also, no paging or segmentation is implemented. Memory addresses are used
//! as-is.
//!
//! When implementing opcodes the instruction set reference in volume 2 of the Intel Software
//! Developer's Manual should be the primary source of inspiration ;-). Aside from that other
//! (RREIL) code generator are worth a look e.g.
//!
//!  * https://github.com/Cr4sh/openreil
//!  * https://github.com/StanfordPL/stoke
//!  * https://github.com/snf/synthir
//!  * https://github.com/c01db33f/reil
//!
//! Simple opcodes that do not require memory access and/or jump/branch can be verified against
//! the CPU directly using a `QuickCheck` harness that's part of the Panopticon test suite. See
//! `tests/amd64.rs` for how to use it.
//!
//! In case you add opcode semantics please update issue #36.

use std::cmp::max;

use {
    Lvalue,
    Rvalue,
    Guard,
    Result,
    Statement,
};
use amd64::*;

/// Sets the adjust flag AF after an addition. Assumes res := a + ?.
fn set_adj_flag(res: &Lvalue, a: &Rvalue) -> Result<Vec<Statement>> {
    rreil!{
        //cmpeq af1:1, (res.extract(4,0).unwrap()), (a.extract(4,0).unwrap());
        cmpltu AF:1, (res.extract(4,0).unwrap()), (a.extract(4,0).unwrap());
        and af1:1, af1:1, AF:1;
        //or AF:1, af1:1, af2:1;
    }
}

/// Sets the adjust flag AF after a subtraction. Assumes res := a - ?.
fn set_sub_adj_flag(res: &Lvalue, a: &Rvalue) -> Result<Vec<Statement>> {
    rreil!{
        //cmpeq af1:1, (res.extract(4,0).unwrap()), (a.extract(4,0).unwrap());
        cmpltu AF:1, (a.extract(4,0).unwrap()), (res.extract(4,0).unwrap());
        //and af1:1, af1:1, AF:1;
        //or AF:1, af1:1, af2:1;
    }
}

/// Sets the parity flag PF.
fn set_parity_flag(res: &Lvalue) -> Result<Vec<Statement>> {
    rreil!{
        mov half_res:8, (res.extract(8,0).unwrap());
        mov PF:1, half_res:1;
        xor PF:1, PF:1, half_res:1/1;
        xor PF:1, PF:1, half_res:1/2;
        xor PF:1, PF:1, half_res:1/3;
        xor PF:1, PF:1, half_res:1/4;
        xor PF:1, PF:1, half_res:1/5;
        xor PF:1, PF:1, half_res:1/6;
        xor PF:1, PF:1, half_res:1/7;
        xor PF:1, PF:1, [1]:1;
    }
}

/// Sets the carry flag CF after an addition. Assumes res := a + ?.
fn set_carry_flag(res: &Lvalue, a: &Rvalue) -> Result<Vec<Statement>> {
    rreil!{
        cmpeq cf1:1, (res), (a);
        cmpltu cf2:1, (res), (a);
        and cf1:1, cf1:1, CF:1;
        or CF:1, cf1:1, cf2:1;
    }
}

/// Sets the carry flag CF after a subtraction. Assumes res := a + ?.
fn set_sub_carry_flag(res: &Lvalue, a: &Rvalue) -> Result<Vec<Statement>> {
    rreil!{
        cmpeq cf1:1, (res), (a);
        cmpltu cf2:1, (a), (res);
        and cf1:1, cf1:1, CF:1;
        or CF:1, cf1:1, cf2:1;
    }
}

/// Sets the overflow flag OF. Assumes res := a ? b.
fn set_overflow_flag(res: &Lvalue, a: &Rvalue, b: &Rvalue, sz: usize) -> Result<Vec<Statement>> {
    /*
     * The rules for turning on the overflow flag in binary/integer math are two:
     *
     * 1. If the sum of two numbers with the sign bits off yields a result number
     *    with the sign bit on, the "overflow" flag is turned on.
     *
     *    0100 + 0100 = 1000 (overflow flag is turned on)
     *
     * 2. If the sum of two numbers with the sign bits on yields a result number
     *    with the sign bit off, the "overflow" flag is turned on.
     *
     *    1000 + 1000 = 0000 (overflow flag is turned on)
     *
     * Otherwise, the overflow flag is turned off.
     */
    let msb = sz - 1;
    rreil!{
        xor of1:sz, (a), (b);
        xor of1:sz, of1:sz, [0xffffffffffffffff]:sz;
        xor of2:sz, (a), (res);
        and OF:1, of1:1/msb, of2:1/msb;
    }
}

/// Assumes res := a ? b
fn set_sub_overflow_flag(res: &Lvalue, a: &Rvalue, b: &Rvalue, sz: usize) -> Result<Vec<Statement>> {
    /*
     * The rules for turning on the overflow flag in binary/integer math are two:
     *
     * 1. If the sum of two numbers with the sign bits off yields a result number
     *    with the sign bit on, the "overflow" flag is turned on.
     *
     *    0100 + 0100 = 1000 (overflow flag is turned on)
     *
     * 2. If the sum of two numbers with the sign bits on yields a result number
     *    with the sign bit off, the "overflow" flag is turned on.
     *
     *    1000 + 1000 = 0000 (overflow flag is turned on)
     *
     * Otherwise, the overflow flag is turned off.
     */
    let msb = sz - 1;
    rreil!{
        xor of1:sz, (a), (b);
        xor of2:sz, (b), (res);
        xor of2:sz, of2:sz, [0xffffffffffffffff]:sz;
        and OF:1, of1:1/msb, of2:1/msb;
    }
}

/// Sign extends `a` and `b` to `max(a.size,b.size)`.
fn sign_extend(a: &Rvalue, b: &Rvalue) -> Result<(Rvalue,Rvalue,usize,Vec<Statement>)> {
    extend(a,b,true)
}

fn zero_extend(a: &Rvalue, b: &Rvalue) -> Result<(Rvalue,Rvalue,usize,Vec<Statement>)> {
    extend(a,b,false)
}

/// Returns (a/sz, b/sz, sz) w/ s = max(a.size,b.size)
fn extend(a: &Rvalue, b: &Rvalue,sign_ext: bool) -> Result<(Rvalue,Rvalue,usize,Vec<Statement>)> {
    let sz = max(a.size().unwrap_or(0),b.size().unwrap_or(0));
    let ext = |x: &Rvalue,s: usize| -> Rvalue {
        match x {
            &Rvalue::Undefined => Rvalue::Undefined,
            &Rvalue::Variable{ ref name, ref offset, ref size, .. } => {
                if *size != s {
                    Rvalue::Variable{
                        name: format!("{}_ext",name).into(),
                        subscript: None,
                        size: s + *offset,
                        offset: *offset
                    }
                } else {
                    x.clone()
                }
            }
            &Rvalue::Constant{ ref value, ref size } => {
                if *size != s {
                    Rvalue::Constant{ value: *value, size: s }
                } else {
                    x.clone()
                }
            }
        }
    };

    let ext_a = ext(a,sz);
    let ext_b = ext(b,sz);
    let mut stmts = vec![];

    assert!(sz > 0);
    assert!(ext_a.size() == None || ext_b.size() == None || ext_a.size() == ext_b.size());

    if a.size() != ext_a.size() {
        if let Some(lv) = Lvalue::from_rvalue(ext_a.clone()) {
            stmts = if sign_ext {
                try!(rreil!{
                    sext/sz (lv), (a);
                })
            } else {
                try!(rreil!{
                    zext/sz (lv), (a);
                })
            };
        }
    }

    if b.size() != ext_b.size() {
        if let Some(lv) = Lvalue::from_rvalue(ext_b.clone()) {
            stmts.append(&mut if sign_ext {
                try!(rreil!{
                    sext/sz (lv), (b);
                })
            } else {
                try!(rreil!{
                    zext/sz (lv), (b);
                })
            });
        }
    }

    Ok((ext_a,ext_b,sz,stmts))
}

/// Returns all sub- and super registers for `name` or None if `name` isn't a register.
fn reg_variants(name: &str) -> Option<(Lvalue,Lvalue,Lvalue,Lvalue,Lvalue)> {
    match name {
        "AL" | "AH" | "AX" | "EAX" | "RAX" => {Some((
                rreil_lvalue!{ AL:8 },
                rreil_lvalue!{ AH:8 },
                rreil_lvalue!{ AX:16 },
                rreil_lvalue!{ EAX:32 },
                rreil_lvalue!{ RAX:64 }))
        },
        "BL" | "BH" | "BX" | "EBX" | "RBX" => {Some((
                rreil_lvalue!{ BL:8 },
                rreil_lvalue!{ BH:8 },
                rreil_lvalue!{ BX:16 },
                rreil_lvalue!{ EBX:32 },
                rreil_lvalue!{ RBX:64 }))
        },
        "CL" | "CH" | "CX" | "ECX" | "RCX" => {Some((
                rreil_lvalue!{ CL:8 },
                rreil_lvalue!{ CH:8 },
                rreil_lvalue!{ CX:16 },
                rreil_lvalue!{ ECX:32 },
                rreil_lvalue!{ RCX:64 }))
        },
        "DL" | "DH" | "DX" | "EDX" | "RDX" => {Some((
                rreil_lvalue!{ DL:8 },
                rreil_lvalue!{ DH:8 },
                rreil_lvalue!{ DX:16 },
                rreil_lvalue!{ EDX:32 },
                rreil_lvalue!{ RDX:64 }))
        },
        "SIL" | "SIH" | "SI" | "ESI" | "RSI" => {Some((
                rreil_lvalue!{ SIL:8 },
                rreil_lvalue!{ SIH:8 },
                rreil_lvalue!{ SI:16 },
                rreil_lvalue!{ ESI:32 },
                rreil_lvalue!{ RSI:64 }))
        },
        "DIL" | "DIH" | "DI" | "EDI" | "RDI" => {Some((
                rreil_lvalue!{ DIL:8 },
                rreil_lvalue!{ DIH:8 },
                rreil_lvalue!{ DI:16 },
                rreil_lvalue!{ EDI:32 },
                rreil_lvalue!{ RDI:64 }))
        },
        "BPL" | "BP" | "EBP" | "RBP" => {Some((
                rreil_lvalue!{ BPL:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ BP:16 },
                rreil_lvalue!{ EBP:32 },
                rreil_lvalue!{ RBP:64 }))
        },
        "SPL" | "SP" | "ESP" | "RSP" => {Some((
                rreil_lvalue!{ SPL:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ SP:16 },
                rreil_lvalue!{ ESP:32 },
                rreil_lvalue!{ RSP:64 }))
        },
        "IP" | "EIP" | "RIP" => {Some((
                rreil_lvalue!{ ? },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ IP:16 },
                rreil_lvalue!{ EIP:32 },
                rreil_lvalue!{ RIP:64 }))
        },
        "R8B" | "R8W" | "R8D" | "R8" => {Some((
                rreil_lvalue!{ R8B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R8W:16 },
                rreil_lvalue!{ R8D:32 },
                rreil_lvalue!{ R8:64 }))
        },
        "R9B" | "R9W" | "R9D" | "R9" => {Some((
                rreil_lvalue!{ R9B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R9W:16 },
                rreil_lvalue!{ R9D:32 },
                rreil_lvalue!{ R9:64 }))
        },
        "R10B" | "R10W" | "R10D" | "R10" => {Some((
                rreil_lvalue!{ R10B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R10W:16 },
                rreil_lvalue!{ R10D:32 },
                rreil_lvalue!{ R10:64 }))
        },
        "R11B" | "R11W" | "R11D" | "R11" => {Some((
                rreil_lvalue!{ R11B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R11W:16 },
                rreil_lvalue!{ R11D:32 },
                rreil_lvalue!{ R11:64 }))
        },
        "R12B" | "R12W" | "R12D" | "R12" => {Some((
                rreil_lvalue!{ R12B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R12W:16 },
                rreil_lvalue!{ R12D:32 },
                rreil_lvalue!{ R12:64 }))
        },
        "R13B" | "R13W" | "R13D" | "R13" => {Some((
                rreil_lvalue!{ R13B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R13W:16 },
                rreil_lvalue!{ R13D:32 },
                rreil_lvalue!{ R13:64 }))
        },
        "R14B" | "R14W" | "R14D" | "R14" => {Some((
                rreil_lvalue!{ R14B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R14W:16 },
                rreil_lvalue!{ R14D:32 },
                rreil_lvalue!{ R14:64 }))
        },
        "R15B" | "R15W" | "R15D" | "R15" => {Some((
                rreil_lvalue!{ R15B:8 },
                rreil_lvalue!{ ? },
                rreil_lvalue!{ R15W:16 },
                rreil_lvalue!{ R15D:32 },
                rreil_lvalue!{ R15:64 }))
        },
        _ => None
    }
}

/// Assigns `val:sz` to `reg`. This function makes sure all that e.g. EAX is written when RAX is.
fn write_reg(reg: &Rvalue, val: &Rvalue, _sz: usize) -> Result<Vec<Statement>> {
    use std::cmp;
    use std::num::Wrapping;

    if let &Rvalue::Variable{ ref name, ref size, ref offset,.. } = reg {
        let mut hi = *offset + *size;
        let mut lo = *offset;
        // this warning seems totally spurious, wtf...
        let mut stmts;

        if let Some((reg8l,reg8h,reg16,reg32,reg64)) = reg_variants(name) {
            if *reg == reg8h.clone().into() {
                hi += 8;
                lo += 8;
            }

            if lo == 0 && hi == 64 && val.size() == Some(64) {
                stmts = try!(rreil!{
                    mov val:64, (val);
                });
            } else {
                stmts = try!(rreil!{
                    zext/64 val:64, (val);
                });
            }

            if lo > 0 {
                let shft = 1 << lo;
                stmts.append(&mut try!(rreil!{
                    mul val:64, val:64, [shft]:64;
                }));
            }

            // *L
            if lo <= 7 {
                let msk = !((0xff << lo) % (1 << cmp::min(8,hi))) & 0xff;

                if msk == 0 {
                    stmts.append(&mut try!(rreil!{
                        mov (reg8l), val:8;
                    }));
                } else if msk < 0xff {
                     stmts.append(&mut try!(rreil!{
                        and (reg8l), (reg8l), [msk]:8;
                        or (reg8l), (reg8l), val:8;
                    }));
                }
            }

            // *H
            if hi >= 9 && lo <= 15 && reg8h != Lvalue::Undefined {
                let msk = (!((0xffff << lo) % (1u64 << cmp::min(16,hi))) & 0xffff) >> 8;
                if msk == 0 {
                    stmts.append(&mut try!(rreil!{
                        mov (reg8h), val:8/8;
                    }));
                } else if msk < 0xff {
                     stmts.append(&mut try!(rreil!{
                        and (reg8h), (reg8h), [msk]:8;
                        or (reg8h), (reg8h), val:8/8;
                    }));
                }
            }

            // *X
            if lo <= 15 {
                let msk = !((0xffff << lo) % (1u64 << cmp::min(16,hi))) & 0xffff;

                if msk == 0 {
                    stmts.append(&mut try!(rreil!{
                        mov (reg16), val:16;
                    }));
                } else if msk < 0xffff {
                     stmts.append(&mut try!(rreil!{
                        and (reg16), (reg16), [msk]:16;
                        or (reg16), (reg16), val:16;
                    }));
                }
            }

            // E*X
            if lo <= 31 {
                let msk = !((0xffffffff << lo) % (1u64 << cmp::min(32,hi))) & 0xffffffff;

                if msk == 0 {
                    stmts.append(&mut try!(rreil!{
                        mov (reg32), val:32;
                    }));
                } else if msk < 0xffffffff {
                     stmts.append(&mut try!(rreil!{
                        and (reg32), (reg32), [msk]:32;
                        or (reg32), (reg32), val:32;
                    }));
                }
            }

            // R*X
            if lo <= 64 {
                let msk = if hi < 64 {
                    !((Wrapping(0xffffffffffffffffu64) << lo).0 % (1 << hi))
                } else {
                    !(Wrapping(0xffffffffffffffffu64) << lo).0
                };

                if msk == 0 || (lo == 0 && hi == 32) {
                    stmts.append(&mut try!(rreil!{
                        mov (reg64), val:64;
                    }));
                } else if msk < 0xffffffffffffffff {
                     stmts.append(&mut try!(rreil!{
                        and (reg64), (reg64), [msk]:64;
                        or (reg64), (reg64), val:64;
                    }));
                }
            }
        } else {
            let lv = Lvalue::Variable{ name: name.clone(), size: *size, subscript: None };
            stmts = try!(rreil!{
                mov (lv),(val);
            });
        }

        Ok(stmts)
    } else {
        Err(format!("Internal error: called write_reg with {:?}",reg).into())
    }
}

pub fn aaa() -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
  /*  rreil!{
        and y:8, AL:8, [0xf]:8;

        // TODO
    }

    let y = new_temp(16);
    let x1 = new_temp(1);
    let x2 = new_temp(1);

    cg.and_b(&y,&*AL,&Rvalue::Constant(0x0f));

    // x1 = !(y <= 9) || AF
    cg.equal_i(&x1,&y.clone().into(),&Rvalue::Constant(9));
    cg.less_i(&x2,&y.clone().into(),&Rvalue::Constant(9));
    cg.or_b(&x1,&x1.clone().into(),&x2.clone().into());
    cg.not_b(&x1,&x1.clone().into());
    cg.or_b(&x1,&x1.clone().into(),&AF.clone().into());

    cg.assign(&*AF,&x1.clone().into());
    cg.assign(&*CF,&x1.clone().into());

    // AX = (AX + x1 * 0x106) % 0x100
    cg.lift_b(&y,&x1.clone().into());
    cg.mul_i(&y,&y.clone().into(),&Rvalue::Constant(0x106));
    cg.add_i(&AX,&AX.clone().into(),&y.clone().into());
    cg.mod_i(&AX,&AX.clone().into(),&Rvalue::Constant(0x100));*/
}

pub fn aam(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
 /*   let temp_al = new_temp(16);

    cg.assign(&temp_al,&AL.clone().into());
    cg.div_i(&*AH,&temp_al,&a);
    cg.mod_i(&*AL,&temp_al,&a);*/
}

pub fn aad(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
 /*   let x = new_temp(16);

    cg.mul_i(&x,&AH.clone().into(),&a);
    cg.add_i(&*AL,&x,&AL.clone().into());
    cg.assign(&*AH,&Rvalue::new_bit(0));*/
}

pub fn aas() -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
 /*   let y1 = new_temp(16);
    let x1 = new_temp(1);
    let x2 = new_temp(1);

    cg.and_b(&y1,&*AL,&Rvalue::Constant(0x0f));

    // x1 = !(y <= 9) || AF
    cg.equal_i(&x1,&y1.clone().into(),&Rvalue::Constant(9));
    cg.less_i(&x2,&y1.clone().into(),&Rvalue::Constant(9));
    cg.or_b(&x1,&x1.clone().into(),&x2.clone().into());
    cg.not_b(&x1,&x1.clone().into());
    cg.or_b(&x1,&x1.clone().into(),&AF.clone().into());

    cg.assign(&*AF,&x1.clone().into());
    cg.assign(&*CF,&x1.clone().into());

    let y2 = new_temp(16);

    // AX = (AX - x1 * 6) % 0x100
    cg.lift_b(&y2,&x1.clone().into());
    cg.mul_i(&y2,&y2.clone().into(),&Rvalue::Constant(6));
    cg.sub_i(&AX,&AX.clone().into(),&y2.clone().into());
    cg.mod_i(&AX,&AX.clone().into(),&Rvalue::Constant(0x100));

    let z = new_temp(16);

    // AH = (AH - x1) % 0x10
    cg.lift_b(&z,&x1.clone().into());
    cg.sub_i(&AH,&AH.clone().into(),&z.clone().into());
    cg.mod_i(&AH,&AH.clone().into(),&Rvalue::Constant(0x10));

    cg.assign(&*AL,&y1.clone().into());*/
}

pub fn adc(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        add res:sz, (a), (b);
        zext/sz cf:sz, CF:1;
        add res:sz, res:sz, cf:sz;
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;

        cmpeq af1:1, (res.extract(4,0).unwrap()), (a.extract(4,0).unwrap());
        cmpltu af2:1, (res.extract(4,0).unwrap()), (a.extract(4,0).unwrap());
        and af1:1, af1:1, CF:1;
        or AF:1, af1:1, af2:1;

    }));
    stmts.append(&mut try!(set_carry_flag(&res,&a)));
    stmts.append(&mut try!(set_overflow_flag(&res,&a,&b,sz)));
    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn add(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    // TODO: use this stmts or below? this is worrisome...
    let (a,b,sz, mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };
    //let mut stmts = vec![];

    stmts.append(&mut try!(rreil!{
        add res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
        cmpltu CF:1, (res), (a);
    }));
    stmts.append(&mut try!(set_adj_flag(&res,&a)));
    stmts.append(&mut try!(set_overflow_flag(&res,&a,&b,sz)));
    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn adcx(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        add res:sz, (a), (b);
        zext/sz cf:sz, CF:1;
        add res:sz, res:sz, cf:sz;
    }));
    stmts.append(&mut try!(set_carry_flag(&res,&a)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn and(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        and res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
        mov CF:1, [0]:1;
        mov OF:1, [0]:1;
        mov AF:1, ?;
    }));
    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn arpl(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn bound(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn bsf(_a: Rvalue, _b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    // let (_,b,sz,_) = try!(sign_extend(&_a,&_b));
    // let res = rreil_lvalue!{ res:sz };
    // let mut stmts = try!(rreil!{
    //     cmpeq ZF:1, (b), [0]:sz;
    //     mov res:sz, ?;
    // });

    // stmts.append(&mut try!(write_reg(&_a,&res.clone().into(),sz)));
    // Ok((stmts,JumpSpec::FallThru))
}

pub fn bsr(_a: Rvalue, _b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    // let (_,b,sz,_) = try!(sign_extend(&_a,&_b));
    // let res = rreil_lvalue!{ res:sz };
    // let mut stmts = try!(rreil!{
    //     cmpeq ZF:1, (b), [0]:sz;
    //     mov res:sz, ?;
    // });

    // stmts.append(&mut try!(write_reg(&_a,&res.clone().into(),sz)));
    // Ok((stmts,JumpSpec::FallThru))
}

pub fn bswap(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    //unimplemented!()
    /*
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

pub fn bt(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    //unimplemented!()
    /*
    using dsl::operator<<;
    rvalue mod = (Rvalue::Constant(1) << (b % constant(bitwidth(a))));

    m.assign(to_lvalue(CF), (a / mod) % 2);
    m.assign(to_lvalue(PF), undefined());
    m.assign(to_lvalue(OF), undefined());
    m.assign(to_lvalue(SF), undefined());
    m.assign(to_lvalue(AF), undefined());*/
}

pub fn btc(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    //unimplemented!()
    /*
    using dsl::operator<<;
    rvalue mod = (Rvalue::Constant(1) << (b % constant(bitwidth(a))));

    m.assign(to_lvalue(CF), (a / mod) % 2);
    m.assign(to_lvalue(PF), undefined());
    m.assign(to_lvalue(OF), undefined());
    m.assign(to_lvalue(SF), undefined());
    m.assign(to_lvalue(AF), undefined());
    m.assign(to_lvalue(a),a ^ mod);*/
}

pub fn btr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    //unimplemented!()
    /*
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

pub fn bts(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
    //unimplemented!()
    /*
    using dsl::operator<<;
    rvalue mod = (Rvalue::Constant(1) << (b % constant(bitwidth(a))));

    m.assign(to_lvalue(CF), (a / mod) % 2);
    m.assign(to_lvalue(PF), undefined());
    m.assign(to_lvalue(OF), undefined());
    m.assign(to_lvalue(SF), undefined());
    m.assign(to_lvalue(AF), undefined());
    m.assign(to_lvalue(a),a & mod);*/
}

pub fn call(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*let stmts = try!(rreil!{
        zext/64 new_rip:64, (a);
        call ?, new_rip:64;
    });*/
    let stmts = try!(rreil!{
        call ?, (a);
    });
    Ok((stmts,JumpSpec::FallThru))
}

pub fn cmov(_: Rvalue, _: Rvalue, _: Condition) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
 /*   let a = Lvalue::from_rvalue(&_a).unwrap();
    let fun = |f: &Lvalue,| {
        let l = new_temp(bitwidth(&a.clone().into()));
        let nl = new_temp(bitwidth(&a.clone().into()));
        let n = new_temp(1);

        cg.lift_b(&l,&f.clone().into());
        cg.not_b(&n,&f.clone().into());
        cg.lift_b(&nl,&n);
        cg.mul_i(&l,&l,&b);
        cg.mul_i(&nl,&nl,&a.clone().into());
        cg.add_i(&a,&l,&nl);
    };

    match c {
        Condition::Overflow => fun(&*OF,cg),
        Condition::NotOverflow =>  {
            let nof = new_temp(1);
            cg.not_b(&nof,&OF.clone().into());
            fun(&nof,cg)
        },
        Condition::Carry => fun(&*CF,cg),
        Condition::AboveEqual => {
            let ncf = new_temp(1);
            cg.not_b(&ncf,&CF.clone().into());
            fun(&ncf,cg)
        },
        Condition::Equal => fun(&*ZF,cg),
        Condition::NotEqual => {
            let nzf = new_temp(1);
            cg.not_b(&nzf,&ZF.clone().into());
            fun(&nzf,cg)
        },
        Condition::BelowEqual => {
            let zc = new_temp(1);
            cg.or_b(&zc,&ZF.clone().into(),&CF.clone().into());
            fun(&zc,cg)
        },
        Condition::Above => {
            let zc = new_temp(1);
            cg.or_b(&zc,&ZF.clone().into(),&CF.clone().into());
            cg.not_b(&zc,&zc);
            fun(&zc,cg)
        },
        Condition::Sign => fun(&*SF,cg),
        Condition::NotSign => {
            let nsf = new_temp(1);
            cg.not_b(&nsf,&SF.clone().into());
            fun(&nsf,cg)
        },
        Condition::Parity => fun(&*PF,cg),
        Condition::NotParity => {
            let npf = new_temp(1);
            cg.not_b(&npf,&PF.clone().into());
            fun(&npf,cg)
        },
        Condition::Less => {
            let b = new_temp(1);
            cg.xor_b(&b,&SF.clone().into(),&OF.clone().into());
            cg.not_b(&b,&b.clone().into());
            fun(&b,cg)
        },
        Condition::GreaterEqual => {
            let b = new_temp(1);
            cg.xor_b(&b,&SF.clone().into(),&OF.clone().into());
            fun(&b,cg)
        },
        Condition::LessEqual => {
            let b = new_temp(1);
            cg.xor_b(&b,&SF.clone().into(),&OF.clone().into());
            cg.not_b(&b,&b.clone().into());
            cg.or_b(&b,&b,&ZF.clone().into());
            fun(&b,cg)
        },
        Condition::Greater => {
            let b = new_temp(1);
            let z = new_temp(1);
            cg.xor_b(&b,&SF.clone().into(),&OF.clone().into());
            cg.not_b(&z,&ZF.clone().into());
            cg.or_b(&b,&b,&z.clone().into());
            fun(&b,cg)
        },
    }*/
}

pub fn cmp(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        sub res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
    }));
    stmts.append(&mut try!(set_sub_carry_flag(&res,&a)));
    stmts.append(&mut try!(set_sub_adj_flag(&res,&a)));
    stmts.append(&mut try!(set_sub_overflow_flag(&res,&a,&b,sz)));
    stmts.append(&mut try!(set_parity_flag(&res)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn cmpsw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmpsb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn cmps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
 /*   let a = Lvalue::Memory{
        offset: Box::new(aoff.clone()),
        bytes: 1,
        endianess: Endianess::Little,
        name: "ram".to_string()
    };
    let b = Lvalue::Memory{
        offset: Box::new(boff.clone()),
        bytes: 1,
        endianess: Endianess::Little,
        name: "ram".to_string()
    };
    let res = new_temp(8);
    let off = new_temp(bitwidth(&aoff));
    let n = new_temp(1);
    let df = new_temp(bitwidth(&aoff));
    let ndf = new_temp(bitwidth(&aoff));

    cg.sub_i(&res,&a.clone().into(),&b.clone().into());
    set_arithm_flags(&res,&res.clone().into(),&a.clone().into(),cg);

    cg.lift_b(&df,&DF.clone().into());
    cg.not_b(&n,&DF.clone().into());
    cg.lift_b(&ndf,&n.clone().into());

    cg.sub_i(&off,&df,&ndf);

    let ao = Lvalue::from_rvalue(&aoff).unwrap();
    let bo = Lvalue::from_rvalue(&boff).unwrap();
    cg.add_i(&ao,&aoff,&off);
    cg.add_i(&bo,&boff,&off);*/
}

pub fn cmpxchg(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    return Ok((vec![],JumpSpec::FallThru));
 /*   cg.equal_i(&*ZF,&a,&EAX.clone().into());

    let n = new_temp(1);
    let zf = new_temp(32);
    let nzf = new_temp(32);
    let la = Lvalue::from_rvalue(&a).unwrap();

    cg.lift_b(&zf,&ZF.clone().into());
    cg.not_b(&n,&ZF.clone().into());
    cg.lift_b(&nzf,&n.clone().into());
    cg.mul_i(&zf,&zf,&b);
    cg.mul_i(&nzf,&nzf,&a);
    cg.add_i(&la,&zf,&nzf);

    cg.lift_b(&zf,&ZF.clone().into());
    cg.lift_b(&nzf,&n.clone().into());
    cg.mul_i(&zf,&zf,&EAX.clone().into());
    cg.add_i(&*EAX,&zf,&nzf);*/
}

pub fn or(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        or res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
        mov CF:1, [0]:1;
        mov OF:1, [0]:1;
        mov AF:1, ?;
    }));
    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn sbb(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        sub res:sz, (a), (b);
        zext/sz cf:sz, CF:1;
        sub res:sz, res:sz, cf:sz;
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;

        cmpeq af1:1, (res.extract(4,0).unwrap()), (a.extract(4,0).unwrap());
        cmpltu af2:1, (a.extract(4,0).unwrap()), (res.extract(4,0).unwrap());
        and af1:1, af1:1, CF:1;
        or AF:1, af1:1, af2:1;
    }));
    stmts.append(&mut try!(set_sub_carry_flag(&res,&a)));
    stmts.append(&mut try!(set_sub_overflow_flag(&res,&a,&b,sz)));
    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn sub(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };

    stmts.append(&mut try!(rreil!{
        sub res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
    }));
    stmts.append(&mut try!(set_sub_carry_flag(&res,&a)));
    stmts.append(&mut try!(set_sub_adj_flag(&res,&a)));
    stmts.append(&mut try!(set_sub_overflow_flag(&res,&a,&b,sz)));
    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn xor(_a: Rvalue, _b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (a,b,sz,mut stmts) = try!(sign_extend(&_a,&_b));
    let res = rreil_lvalue!{ res:sz };

    if a == b {
        stmts.append(&mut try!(rreil!{
            mov res:sz, [0]:sz;
            mov SF:1, [0]:1;
            mov ZF:1, [1]:1;
            mov CF:1, [0]:1;
            mov OF:1, [0]:1;
            mov AF:1, ?;
        }));
    } else {
        stmts.append(&mut try!(rreil!{
            xor res:sz, (a), (b);
            cmplts SF:1, res:sz, [0]:sz;
            cmpeq ZF:1, res:sz, [0]:sz;
            mov CF:1, [0]:1;
            mov OF:1, [0]:1;
            mov AF:1, ?;
        }));
    }

    stmts.append(&mut try!(set_parity_flag(&res)));
    stmts.append(&mut try!(write_reg(&_a,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn cmpxchg8b(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmpxchg16b(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cpuid() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn clc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cld() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cli() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn std() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sti() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn stc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn cbw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cwd() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn clts() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn conv() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"conv","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn conv2() -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"conv2","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
    */
    Ok((vec![],JumpSpec::FallThru))
}

pub fn daa() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn das() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn dec(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn div(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn enter(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn hlt() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let len = st.tokens.len();
    st.mnemonic(len,"hlt","",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    true*/
    Ok((vec![],JumpSpec::DeadEnd))
}

pub fn int3() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn int1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn invd() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn idiv(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn imul1(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn imul2(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    // TODO: use this stmts or below? this seems dangerous...
    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };
    //let mut stmts = vec![];
    // unused
    let _hsz = sz / 2;
    let dsz = sz * 2;
    let max = (1u64 << (sz - 1)) - 1;

    stmts.append(&mut try!(rreil!{
        zext/dsz aa:dsz, (a);
        zext/dsz bb:dsz, (b);
        mul dres:dsz, aa:dsz, bb:dsz;
        mov res:sz, dres:dsz;
        cmplts SF:1, res:sz, [0]:sz;
        mov ZF:1, ?;
        mov AF:1, ?;
        mov PF:1, ?;
        cmpltu CF:1, [max]:dsz, dres:dsz;
        mov OF:1, CF:1;
    }));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))

}

pub fn imul3(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn in_(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn icebp() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn inc(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn insb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn insw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn int(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn into() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn iretw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::DeadEnd)) }

pub fn iret() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let len = st.tokens.len();
    st.mnemonic(len,"iret","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    true*/
    Ok((vec![],JumpSpec::DeadEnd))
}

pub fn setcc(_: Rvalue, _: Condition) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn seto(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Overflow) }
pub fn setno(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::NotOverflow) }
pub fn setb(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Below) }
pub fn setae(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::AboveEqual) }
pub fn setz(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Equal) }
pub fn setnz(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::NotEqual) }
pub fn setbe(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::BelowEqual) }
pub fn seta(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Above) }
pub fn sets(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Sign) }
pub fn setns(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::NotSign) }
pub fn setp(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Parity) }
pub fn setnp(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::NotParity) }
pub fn setl(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Less) }
pub fn setle(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::LessEqual) }
pub fn setg(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::Greater) }
pub fn setge(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { setcc(a,Condition::GreaterEqual) }

pub fn cmovcc(_: Rvalue, _: Condition) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmovo(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Overflow) }
pub fn cmovno(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::NotOverflow) }
pub fn cmovb(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Below) }
pub fn cmovae(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::AboveEqual) }
pub fn cmovz(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Equal) }
pub fn cmovnz(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::NotEqual) }
pub fn cmovbe(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::BelowEqual) }
pub fn cmova(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Above) }
pub fn cmovs(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Sign) }
pub fn cmovns(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::NotSign) }
pub fn cmovp(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Parity) }
pub fn cmovnp(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::NotParity) }
pub fn cmovl(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Less) }
pub fn cmovle(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::LessEqual) }
pub fn cmovg(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::Greater) }
pub fn cmovge(a: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { cmovcc(a,Condition::GreaterEqual) }


pub fn jmp(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    Ok((vec![],JumpSpec::Jump(a)))
}

pub fn jcc(a: Rvalue, _: Condition) -> Result<(Vec<Statement>,JumpSpec)> {
    Ok((vec![],JumpSpec::Branch(a,Guard::always())))
}

pub fn jo(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Overflow) }
pub fn jno(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::NotOverflow) }
pub fn jb(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Below) }
pub fn jae(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::AboveEqual) }
pub fn je(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Equal) }
pub fn jne(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::NotEqual) }
pub fn jbe(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::BelowEqual) }
pub fn ja(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Above) }
pub fn js(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Sign) }
pub fn jns(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::NotSign) }
pub fn jp(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Parity) }
pub fn jnp(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::NotParity) }
pub fn jl(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Less) }
pub fn jle(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::LessEqual) }
pub fn jg(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::Greater) }
pub fn jge(a: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { jcc(a,Condition::GreaterEqual) }

pub fn jcxz(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn jecxz(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn jrcxz(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn lahf() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn sahf() -> Result<(Vec<Statement>,JumpSpec)> {
    let stmts = try!(rreil!{
        mov CF:1, AH:1;
        mov PF:1, AH:1/2;
        mov AF:1, AH:1/4;
        mov ZF:1, AH:1/6;
        mov SF:1, AH:1/7;
    });

    Ok((stmts,JumpSpec::FallThru))
}

pub fn lsl(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lar(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lds(a: Rvalue, b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { lxs(a,b,rreil_rvalue!{ DS:16 }) }
pub fn les(a: Rvalue, b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { lxs(a,b,rreil_rvalue!{ ES:16 }) }
pub fn lss(a: Rvalue, b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { lxs(a,b,rreil_rvalue!{ SS:16 }) }
pub fn lfs(a: Rvalue, b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { lxs(a,b,rreil_rvalue!{ FS:16 }) }
pub fn lgs(a: Rvalue, b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { lxs(a,b,rreil_rvalue!{ GS:16 }) }
pub fn lxs(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lea(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn leave() -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"leave","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn lodsw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn lodsb() -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"lodsb","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn lods() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"lods","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn loop_(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loop","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn loope(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loope","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn loopne(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loopne","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn mov(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (_,b,sz,mut stmts) = try!(zero_extend(&a_,&b_));

    stmts.append(&mut try!(write_reg(&a_,&b,sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn movbe(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn movsb() -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"movsb","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn movsw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn movs() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"movs","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn movsx(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (_,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));

    stmts.append(&mut try!(write_reg(&a_,&b,sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn movzx(a_: Rvalue, b_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (_,b,sz,mut stmts) = try!(zero_extend(&a_,&b_));

    stmts.append(&mut try!(write_reg(&a_,&b,sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn mul(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn neg(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn nop(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lock() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rep() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn repne() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn not(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn out(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn outsb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn outsw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn popfw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pushfw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pop(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"pop","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn popa() -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"popa","",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn popcnt(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn popf(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn push(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"push","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn pusha() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"pusha","",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn pushf(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rcl(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rcr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn ret() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let len = st.tokens.len();
    st.mnemonic(len,"ret","",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    true*/
    Ok((vec![],JumpSpec::DeadEnd))
}

pub fn retn(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*let len = st.tokens.len();
    st.mnemonic(len,"ret","",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    true*/
    Ok((vec![],JumpSpec::DeadEnd))
}

pub fn retf() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let len = st.tokens.len();
    st.mnemonic(len,"retf","",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    true*/
    Ok((vec![],JumpSpec::DeadEnd))
}

pub fn retnf(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    /*let len = st.tokens.len();
    if let Some(d) = decode::decode_imm(st) {
        st.mnemonic(len,"retnf","{u}",vec![d],&|| {} );
        true
    } else {
        false
    }*/
    Ok((vec![],JumpSpec::DeadEnd))
}

pub fn ror(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rol(_a: Rvalue, _b: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
/*    let (a,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    let res = rreil_lvalue!{ res:sz };
    let mut stmts = vec![];
    let msb = sz - 1;

    stmts.append(&mut try!(rreil!{
        mov msb:1, (a.extract(1,sz - 1).unwrap());
        zext/dsz bb:dsz, (b);
        mul dres:dsz, aa:dsz, bb:dsz;
        mov res:sz, dres:dsz;
        cmplts SF:1, res:sz, [0]:sz;
        mov ZF:1, ?;
        mov AF:1, ?;
        mov PF:1, ?;
        cmpltu CF:1, [max]:dsz, dres:dsz;
        mov OF:1, CF:1;
    }));
    stmts.append(&mut try!(write_reg(&a_,&res.clone().into(),sz)));

    Ok((stmts,JumpSpec::FallThru))
*/
   Ok((vec![],JumpSpec::FallThru))
}

pub fn sal(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn salc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sar(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn scasw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn scasb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn scas() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"scas","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn shl(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn shr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn shld(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn shrd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn stosb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn stosw() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn stos() -> Result<(Vec<Statement>,JumpSpec)> {
    /*let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"stos","{u}",Ok((vec![],JumpSpec::FallThru)),&|| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true*/
    Ok((vec![],JumpSpec::FallThru))
}

pub fn test(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ud1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ud2() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xadd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xchg(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdtsc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xgetbv(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xlatb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn wait() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn wbinvd() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetch(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn movsxd(a_: Rvalue, b_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> {
    let (_,b,sz,mut stmts) = try!(sign_extend(&a_,&b_));
    stmts.append(&mut try!(write_reg(&a_,&b,sz)));

    Ok((stmts,JumpSpec::FallThru))
}

pub fn syscall() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sysret() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn movapd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn wrmsr() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdmsr() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdpmc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sysenter() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sysexit() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn getsec() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmread(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmwrite(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmenter() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmexit() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn montmul() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xcryptecb() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rsm() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaddwd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn tzcnt(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lzcnt(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn crc32(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn vmcall() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmlaunch() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmresume() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmxoff() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn clac() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn stac() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn encls() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xsetbv() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmfunc() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xend() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xtest() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn enclu() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn swapgs() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdtscp() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdrand(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdseed(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdpid(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmpxch8b(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmptrld(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmptrst(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmclear(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmxon(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xabort(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xbegin(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdfsbase(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rdgsbase(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn wrfsbase(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn wrgsbase(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fxsave() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fxstor() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xsave() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xrstor() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xsaveopt() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn clflush() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blsr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blsmsk(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blsi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn andn(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bextr(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blendd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blendvb(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bzhi(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn clgi() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtph2ps(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn extracti128(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fist(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldl2g() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fperm() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fperm1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn frndintx() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsubrp(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn gatherdd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn gatherdps(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn gatherqd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn gatherqps(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn inserti128(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn invept(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn invlpg(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn invpcid(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn invvpid(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lgdt(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lidt(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lldt(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lmsw(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ltr(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maskmovpd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maskmovps(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pavgusb(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pblendw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pclmulqdq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpgtq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn perm2f128(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn perm2i128(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn permd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn permilp(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn permilpd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn permilps(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn permpd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn permq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pf2id(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pf2iw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfacc(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfadd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfcmpeq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfcmpge(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfcmpgt(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfmax(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfmin(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfmul(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfnacc(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfpnacc(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfrcp(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfrcpit1(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfrcpit2(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfrsqit1(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfrsqrt(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfsub(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pfsubr(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phminposuw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pi2fd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pi2fw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaddubsw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaskmovd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsxbd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsxbq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsxbw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsxdq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsxwd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsxwq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzxbd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzxbq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzxbw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzxdq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzxwd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzxwq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmulhrsw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmulhrw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psignb(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psignd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psignw(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pswapd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpckldq(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sgdt(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha1msg1(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha1msg2(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha1nexte(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha1rnds4(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha256msg1(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha256msg2(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sha256rnds2(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn shlx(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sidt(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sldt(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn smsw(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn str(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn testpd(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn testps(_: Rvalue, _:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn verr(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn verw(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// MMX
pub fn emms() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn packsswb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn packssdw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn packuswb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddsb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddusb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddusw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pand(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pandn(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpeqb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpeqw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpeqd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpgtb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpgtw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpgtd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmadwd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmulhw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmullw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn por(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psraw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psrad(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psrlw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psrld(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psrlq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psllw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pslld(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psllq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubsb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubusb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubusw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpckhbw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpckhwd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpckhdq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpcklbw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpcklwd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpcklqdq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pxor(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// SSE 1
pub fn addps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn addss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn andnps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn andps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmpps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmpss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn comiss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtpi2ps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtps2pi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtsi2ss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtss2si(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvttps2pi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvttss2si(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn divps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn divss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ldmxcsr() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maskmovq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maxps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maxss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn minps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn minss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movaps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn minhps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movlps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movmskps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movntps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movntq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movups(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mulps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mulss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn orps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pavgb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pavgw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pextrw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pinsrw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaxsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaxub(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pminsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pminub(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovmskb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmulhuw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetchnta(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetcht0(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetcht1(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetcht2(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetchw(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn prefetchwt1(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psadbw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pshufw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pshufb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rcpps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rcpss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rsqrtps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn rsqrtss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sfence() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn shufps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sqrtps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sqrtss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn stmxcsr() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn subps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn subss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ucomiss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn unpckhps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn unpcklps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xorps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// SSE 2
pub fn addpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn addsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn andnpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn andpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cflush(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmppd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cmpsd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn comisd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtdq2pd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtdq2ps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtpd2dq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtpd2pi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtpd2ps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtpi2pd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtps2dq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtps2pd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtsd2si(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtsd2ss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtsi2sd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvtss2sd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvttpd2dq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvttpd2pi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvttps2dq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn cvttsd2si(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn divpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn divsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lfence() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maskmovdqu(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maxpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn maxsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mfence() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn minpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn minsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movdq2q(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movdaq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movdqa(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movdqu(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movhpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movhps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movlpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movmskpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movntdq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movntdqa(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movnti(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movntpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movq2dq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movupd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mulpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mulsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn orpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pabsb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pabsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pabsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn paddq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pause() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmuludq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pshufd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pshufhw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pshuflw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pslldq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psarw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psrldq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn psubq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pusbsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punckhwd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn punpckhqdq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn puncklqdq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn puncklwd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn shufpd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sqrtpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn sqrtsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn subpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn subsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ucomisd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn unpckhpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn unpcklpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn xorpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// SSE 4
pub fn blendpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blendps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blendvpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn blendvps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn dppd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn dpps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn extractps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn insertps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mpsadbw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pblendbw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpestri(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpestrm(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpistri(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpistrm(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pextrb(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pextrd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pextrq(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pinsrb(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pinsrd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pinsrq(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn roundpd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn roundps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn roundsd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn roundss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovsx(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmovzx(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pminsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pminsb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pminud(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pminuw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaxsd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaxsb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaxud(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmaxuw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ptest(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmulld(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pmuldq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phaddw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phaddsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phaddd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phsubw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phsubsw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phsubd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn packusdw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pblendvb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pcmpeqq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn phminpushuw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// SSE 3
pub fn addsubpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn addsubps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn haddpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn haddps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn hsubpd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn hsubps(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn lddqu(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn monitor() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movddup(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movshdup(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn movsldup(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn mwait() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn palignr(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// AVX
pub fn aesdec(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn aesdeclast(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn aesenc(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn aesenclast(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn aesimc(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn aeskeygenassist(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vboradcastss(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vboradcastsd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vboradcastf128(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vzeroupper() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// FPU
pub fn f2xm1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fabs() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fadd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn faddp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fiadd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fbld(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fbstp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fchs() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fclex() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnclex(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmove(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovbe(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovu(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovnb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovne(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovnbe(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcmovnu(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcom(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcomp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcompp() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcomi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcomip(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fucomi(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fucomip(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fcos() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fdecstp() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fdiv(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fdivp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fidiv(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fdivr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fdivrp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fidivr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ffree(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ficom(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ficomp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fild(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fincstp() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn finit() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fninit(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fistp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fisttp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fld(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fld1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldl2t() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldl2e() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldpi() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldlg2() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldln2() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldz() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldcw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmul(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmulp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fimul(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnop() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fpatan() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fprem() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fprem1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fptan() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn frndint() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn frstor(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsave(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnsave(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fscale() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsin() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsincos() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsqrt() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fst1(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fst2(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstp(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstcw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fldenv(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstenv(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnstenv(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstsw1(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstsw2(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnstsw(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsub(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsubp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fisub(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fsubr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fisubr(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn ftst() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fucom(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fucomp(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fucompp() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fxam() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fxch(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fxtract() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fyl2x() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fyl2xp1() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// MPX
pub fn bndcl(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bndcu(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bndcn(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bndmov(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bndmk(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bndldx(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn bndstx(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn noop() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn noop_unary(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn noop_binary(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// FMA
pub fn fmadd132ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmadd132ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmadd213ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmadd213ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmadd231ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmadd231ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmaddsub132ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmaddsub231ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmaddsub232ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmnadd132ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmnsub132ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsub132ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsub132ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsub213ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsub213ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsub231ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsub231ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsubadd132ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsubadd231ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fmsubadd232ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmadd213ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmadd213ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmadd231ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmadd231ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmsub213ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmsub213ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmsub231ps(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fnmsub231ss(_: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

// AVX
pub fn vaddpd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaddps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaddsd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaddss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaddsubpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaddsubps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaesdec(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaesdeclast(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaesenc(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaesenclast(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaesimc(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vaeskeygenassist(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vandpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vandps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vandnpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vandnps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vblendpd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vblendps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vblendvpd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vblendvps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcmppd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcmpps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcmpsd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcmpss(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcomisd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcomiss(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtdq2pd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtdq2ps(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtpd2dq(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtpd2ps(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtps2dq(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtps2pd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtsd2si(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtsd2ss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtsi2sd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtss2sd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtsi2ss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvttpd2dq(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvttps2dq(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvttsd2si(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvttss2si(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vdivps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vdivpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vdivss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vdivsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vdppd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vdpps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vextractps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vhaddpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vhaddps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vhsubpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vhsubps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vinsertps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vlddqu(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vldmxcsr(_:Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmaxpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmaxsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmaxps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmaxss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vminpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vminsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vminps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vminss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovhpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovhps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovlpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovlps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmpsadbw(_:Rvalue, _: Rvalue, _:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vorpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vorps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpabsb(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpabsw(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpabsd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpacksswb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpackssdw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpackusdw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpackuswb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddsb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddusb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpaddusw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpalignr(_: Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpand(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpandn(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpavgb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpavgw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpblendvb(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpblendw(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpclmulqdq(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpeqb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpeqw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpeqd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpeqq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpgtb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpgtw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpgtd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpgtq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphaddw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphaddd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphaddsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphminposuw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphsubw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphsubd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vphsubsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpinsrb(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpinsrd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpinsrw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaddubsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmadwd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaxsb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaxsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaxsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaxub(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaxud(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaxuw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpminsb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpminsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpminsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpminub(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpminud(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpminuw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmuldq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmulhrsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmulhuw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmulhw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmulld(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmullw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmuludq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpor(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsadbw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsignb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsignw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsignd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpslldq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsllw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpslld(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsllq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsrad(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsarw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsrldq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsrlw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsrld(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsrlq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubsb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpusbsw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubusb(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubusw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vptest() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpckhbw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunckhwd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpckhdq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpckhqdq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpcklbw(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpckldq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpuncklqdq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpuncklwd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpxor(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vrcpps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vroundpd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vroundps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vroundsd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vroundss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vrsqrtps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vrsqrtss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsqrtss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsqrtsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vshufps(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vshufpd(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsubps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsubss(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsubpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsubsd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vunpckhps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vunpcklps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vunpckhpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vunpcklpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vbroadcastss(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vbroadcastsd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vbroadcastf128(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vextractf128(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vextracti128(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherdd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherdp(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherpdp(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherqpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vinsertf128(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vinserti128(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmaskmovps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmaskmovpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmulps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmulss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmulpd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmulsd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vblendd(_: Rvalue, _:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpboradcastb(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpboradcastw(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpboradcastd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpboradcastq(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpboradcasti128(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vperm2i128(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermilpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermilps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vperm2f128(_:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaskmovd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaskmovq(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsllvd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsravd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsrlvd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vtestpd(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vtestps(_:Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vzeroall() -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vxorps(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vxorpd(_:Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }

pub fn broadcastf128(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn broadcasti128(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn broadcastsd(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn broadcastss(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fst(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstp1(_: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn fstp2(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pboradcastw(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pbroadcastb(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pbroadcastd(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn pbroadcastq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vandn(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vbextr(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vblendvb(_: Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vbzhi(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vcvtph2ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmadd132ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmadd132ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmadd213ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmadd213ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmadd231ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmadd231ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmaddsub132ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmaddsub231ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmaddsub232ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmnadd132ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmnsub132ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsub132ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsub132ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsub213ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsub213ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsub231ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsub231ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsubadd132ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsubadd231ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfmsubadd232ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmadd213ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmadd213ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmadd231ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmadd231ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmsub213ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmsub213ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmsub231ps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vfnmsub231ss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherdps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherqd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vgatherqps(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vmovq2dq(_: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpestri(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpestrm(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpistri(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpcmpistrm(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpermilp(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpextrw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpmaddwd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpshufb(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpshufd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpshufhw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpshuflw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpshufw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsraw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpsubsw(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpckhwd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpcklqdq(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vpunpcklwd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vrcpss(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vsha1rnds4(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vshld(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vshlx(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
pub fn vshrd(_: Rvalue, _: Rvalue, _: Rvalue) -> Result<(Vec<Statement>,JumpSpec)> { Ok((vec![],JumpSpec::FallThru)) }
