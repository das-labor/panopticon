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

use std::cmp::max;

use value::{Lvalue,Rvalue,Endianess};
use codegen::CodeGen;
use disassembler::State;
use amd64::*;
use guard::Guard;

fn do_push(v: &Rvalue, mode: Mode, cg: &mut CodeGen<Amd64>) {
    if let &Rvalue::Variable{ width: w, ..} = v {
        cg.assign(&Lvalue::Memory{
            offset: Box::new(RIP.to_rv()),
            bytes: w / 8,
            endianess: Endianess::Little,
            name: "ram".to_string()
        },v);

        match mode {
            Mode::Real => {
                cg.add_i(&*SP,&SP.to_rv(),&Rvalue::Constant(w as u64));
                cg.mod_i(&*SP,&SP.to_rv(),&Rvalue::Constant(0x10000));
            }
            Mode::Protected => {
                cg.add_i(&*ESP,&ESP.to_rv(),&Rvalue::Constant(w as u64));
                cg.mod_i(&*ESP,&ESP.to_rv(),&Rvalue::Constant(0x100000000));
            }

            Mode::Long => {
                cg.add_i(&*RSP,&RSP.to_rv(),&Rvalue::Constant(w as u64));
            }
        }
    } else {
        unreachable!()
    }
}

fn bitwidth(a: &Rvalue) -> usize {
    match a {
        &Rvalue::Variable{ width: w, .. } => w as usize,
        &Rvalue::Memory{ bytes: b, .. } => (b as usize) * 8,
        _ => unreachable!()
    }
}

fn sign_ext(v: &Rvalue, from: usize, to: usize, cg: &mut CodeGen<Amd64>) -> Rvalue {
    assert!(from < to  && from > 0);

    let sign = new_temp(to);
    let rest = new_temp(to);
    let mask = Rvalue::Constant(1 << (from - 1));

    cg.div_i(&sign,v,&mask);
    cg.mod_i(&rest,v,&mask);

    cg.mod_i(&sign,&sign.to_rv(),&Rvalue::Constant(1 << (to - 1)));
    cg.add_i(&rest,&sign.to_rv(),&rest.to_rv());

    rest.to_rv()
}

fn set_arithm_flags(res: &Lvalue, res_half: &Rvalue, a: &Rvalue, cg: &mut CodeGen<Amd64>) {
    let aw = bitwidth(a);

    if aw < 64 {
        cg.div_i(&*CF,&res.to_rv(),&Rvalue::Constant(1 << aw));
    } else {
        cg.assign(&*CF,&Rvalue::Undefined);
    }

    cg.div_i(&*AF,res_half,&Rvalue::Constant(0x100));
    cg.div_i(&*SF,&res.to_rv(),&Rvalue::Constant(1 << (aw - 1)));
    cg.equal_i(&*ZF,a, &Rvalue::Constant(0));
    cg.xor_i(&*OF,&CF.to_rv(),&SF.to_rv());

    let tmp = new_temp(aw);

    cg.mod_i(&*PF,&res.to_rv(),&Rvalue::Constant(2));

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(4));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(2));
    cg.xor_i(&*PF,&*PF,&tmp.to_rv());

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(8));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(4));
    cg.xor_i(&*PF,&*PF,&tmp.to_rv());

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(16));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(8));
    cg.xor_i(&*PF,&*PF,&tmp.to_rv());

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(32));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(16));
    cg.xor_i(&*PF,&*PF,&tmp.to_rv());

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(64));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(32));
    cg.xor_i(&*PF,&*PF,&tmp.to_rv());

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(128));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(64));
    cg.xor_i(&*PF,&*PF,&tmp);

    cg.mod_i(&tmp,&res.to_rv(),&Rvalue::Constant(256));
    cg.div_i(&tmp,&res.to_rv(),&Rvalue::Constant(128));
    cg.xor_i(&*PF,&*PF,&tmp.to_rv());
}

pub fn flagwr(flag: &Lvalue, val: bool) -> Box<Fn(&mut CodeGen<Amd64>)> {
    let f = flag.clone();
    Box::new(move |cg: &mut CodeGen<Amd64>| {
        cg.assign(&f,&Rvalue::Constant(if val { 1 } else { 0 }));
    })
}

pub fn flagcomp(flag: &Lvalue) -> Box<Fn(&mut CodeGen<Amd64>)> {
    let f = flag.clone();
    Box::new(move |cg: &mut CodeGen<Amd64>| {
        cg.not_b(&f,&f);
    })
}

pub fn aaa(cg: &mut CodeGen<Amd64>) {
    let y = new_temp(16);
    let x1 = new_temp(1);
    let x2 = new_temp(1);

    cg.and_b(&y,&*AL,&Rvalue::Constant(0x0f));

    // x1 = !(y <= 9) || AF
    cg.equal_i(&x1,&y.to_rv(),&Rvalue::Constant(9));
    cg.less_i(&x2,&y.to_rv(),&Rvalue::Constant(9));
    cg.or_b(&x1,&x1.to_rv(),&x2.to_rv());
    cg.not_b(&x1,&x1.to_rv());
    cg.or_b(&x1,&x1.to_rv(),&AF.to_rv());

    cg.assign(&*AF,&x1.to_rv());
    cg.assign(&*CF,&x1.to_rv());

    // AX = (AX + x1 * 0x106) % 0x100
    cg.lift_b(&y,&x1.to_rv());
    cg.mul_i(&y,&y.to_rv(),&Rvalue::Constant(0x106));
    cg.add_i(&AX,&AX.to_rv(),&y.to_rv());
    cg.mod_i(&AX,&AX.to_rv(),&Rvalue::Constant(0x100));
}

pub fn aam(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    let temp_al = new_temp(16);

    cg.assign(&temp_al,&AL.to_rv());
    cg.div_i(&*AH,&temp_al,&a);
    cg.mod_i(&*AL,&temp_al,&a);
}

pub fn aad(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    let x = new_temp(16);

    cg.mul_i(&x,&AH.to_rv(),&a);
    cg.add_i(&*AL,&x,&AL.to_rv());
    cg.assign(&*AH,&Rvalue::Constant(0));
}

pub fn aas(cg: &mut CodeGen<Amd64>) {
    let y1 = new_temp(16);
    let x1 = new_temp(1);
    let x2 = new_temp(1);

    cg.and_b(&y1,&*AL,&Rvalue::Constant(0x0f));

    // x1 = !(y <= 9) || AF
    cg.equal_i(&x1,&y1.to_rv(),&Rvalue::Constant(9));
    cg.less_i(&x2,&y1.to_rv(),&Rvalue::Constant(9));
    cg.or_b(&x1,&x1.to_rv(),&x2.to_rv());
    cg.not_b(&x1,&x1.to_rv());
    cg.or_b(&x1,&x1.to_rv(),&AF.to_rv());

    cg.assign(&*AF,&x1.to_rv());
    cg.assign(&*CF,&x1.to_rv());

    let y2 = new_temp(16);

    // AX = (AX - x1 * 6) % 0x100
    cg.lift_b(&y2,&x1.to_rv());
    cg.mul_i(&y2,&y2.to_rv(),&Rvalue::Constant(6));
    cg.sub_i(&AX,&AX.to_rv(),&y2.to_rv());
    cg.mod_i(&AX,&AX.to_rv(),&Rvalue::Constant(0x100));

    let z = new_temp(16);

    // AH = (AH - x1) % 0x10
    cg.lift_b(&z,&x1.to_rv());
    cg.sub_i(&AH,&AH.to_rv(),&z.to_rv());
    cg.mod_i(&AH,&AH.to_rv(),&Rvalue::Constant(0x10));

    cg.assign(&*AL,&y1.to_rv());
}

pub fn adc(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(max(aw,bw) + 1);
    let res_half = new_temp(8);
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.add_i(&res,&a.to_rv(),&b_ext);
    cg.add_i(&res,&res.to_rv(),&*CF);
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    if aw < 64 {
        cg.mod_i(&a,&res.to_rv(),&Rvalue::Constant(1 << aw));
    } else {
        cg.assign(&a,&res);
    }
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn add(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(max(aw,bw) + 1);
    let res_half = new_temp(8);
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.add_i(&res,&a.to_rv(),&b_ext);
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    if aw < 64 {
        cg.mod_i(&a,&res.to_rv(),&Rvalue::Constant(1 << aw));
    } else {
        cg.assign(&a,&res);
    }
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn adcx(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let res = new_temp(aw + 1);

    cg.add_i(&res,&a,&b);
    cg.add_i(&res,&res,&*CF);
    if aw < 64 {
        cg.mod_i(&a,&res,&Rvalue::Constant(1 << aw));
        cg.div_i(&res,&res,&Rvalue::Constant(1 << aw));
    } else {
        cg.assign(&a,&res);
    }
    cg.less_i(&*CF,&Rvalue::Constant(0xffffffffffffffff),&res);
}

pub fn and(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(max(aw,bw) + 1);
    let res_half = new_temp(8);
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.and_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res,&Rvalue::Constant(0x100));

    if aw < 64 {
        cg.mod_i(&a,&res.to_rv(),&Rvalue::Constant(1 << aw));
    } else {
        cg.assign(&a,&res);
    }
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn arpl(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn bound(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn bsf(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let a = Lvalue::from_rvalue(&_a).unwrap();

    cg.equal_i(&*ZF,&b,&Rvalue::Constant(0));

    for bit in (0..aw) {
        let val = new_temp(aw);

        if bit < 63 {
            cg.mod_i(&val,&b,&Rvalue::Constant(1 << (bit as u64 + 1)));
            cg.div_i(&val,&val.to_rv(),&Rvalue::Constant(1u64 << bit));
        } else {
            cg.assign(&val,&b);
        }
        cg.mul_i(&a,&val.to_rv(),&Rvalue::Constant(bit as u64 + 1));
    }
}

pub fn bsr(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let a = Lvalue::from_rvalue(&_a).unwrap();

    cg.equal_i(&*ZF,&b,&Rvalue::Constant(0));

    for bit in (0..aw).rev() {
        let val = new_temp(aw);

        if bit < 63 {
            cg.mod_i(&val,&b,&Rvalue::Constant(1u64 << (bit + 1)));
            cg.div_i(&val,&val.to_rv(),&Rvalue::Constant(1u64 << bit));
        } else {
            cg.assign(&val,&b);
        }
        cg.mul_i(&a,&val.to_rv(),&Rvalue::Constant(bit as u64 + 1));
    }
}

pub fn bswap(_: &mut CodeGen<Amd64>, _: Rvalue) {
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

pub fn bt(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
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

pub fn btc(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
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

pub fn btr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
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

pub fn bts(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
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

pub fn near_call(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    near_xcall(cg,a,false)
}

pub fn near_rcall(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    near_xcall(cg,a,true)
}

pub fn near_xcall(cg: &mut CodeGen<Amd64>, a: Rvalue, rel: bool) {
    match cg.configuration.operand_size {
        OperandSize::Sixteen => {
            let new_ip = if rel {
                let x = Lvalue::from_rvalue(&sign_ext(&a,32,64,cg)).unwrap();
                cg.add_i(&x,&x.to_rv(),&RIP.to_rv());
                x
            } else {
                Lvalue::from_rvalue(&sign_ext(&a,32,64,cg)).unwrap()
            };

            do_push(&RIP.to_rv(),Mode::Long,cg);
            cg.assign(&*RIP, &new_ip);
            cg.call_i(&Lvalue::Undefined,&new_ip);
        },
        OperandSize::ThirtyTwo => {
            let new_ip = if rel {
                let x = new_temp(32);
                cg.add_i(&x,&a,&EIP.to_rv());
                cg.mod_i(&x,&x,&Rvalue::Constant(0x100000000));
                x
            } else {
                Lvalue::from_rvalue(&a).unwrap()
            };

            do_push(&EIP.to_rv(),Mode::Protected,cg);
            cg.assign(&*EIP, &new_ip);
            cg.call_i(&Lvalue::Undefined,&new_ip);
        },
        OperandSize::SixtyFour => {
            let new_ip = if rel {
                let x = new_temp(16);
                cg.add_i(&x,&a,&EIP.to_rv());
                cg.mod_i(&x,&x.to_rv(),&Rvalue::Constant(0x10000));
                x
            } else {
                let x = new_temp(16);
                cg.mod_i(&x,&a,&Rvalue::Constant(0x10000));
                x
            };

            do_push(&RIP.to_rv(),Mode::Real,cg);
            cg.assign(&*RIP, &new_ip);
            cg.call_i(&Lvalue::Undefined,&new_ip);
        }
        OperandSize::HundredTwentyEight => unreachable!(),
        OperandSize::Eight => unreachable!(),
    }
}

pub fn far_call(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    far_xcall(cg,a,false)
}

pub fn far_rcall(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    far_xcall(cg,a,true)
}

pub fn far_xcall(cg: &mut CodeGen<Amd64>, a: Rvalue, _: bool) {
    match cg.configuration.operand_size {
        OperandSize::Sixteen => {
            do_push(&CS.to_rv(),Mode::Real,cg);
            do_push(&IP.to_rv(),Mode::Real,cg);
        },
        OperandSize::ThirtyTwo => {
            do_push(&CS.to_rv(),Mode::Protected,cg);
            do_push(&EIP.to_rv(),Mode::Protected,cg);
        },
        OperandSize::SixtyFour => {
            do_push(&CS.to_rv(),Mode::Long,cg);
            do_push(&RIP.to_rv(),Mode::Long,cg);
        },
        OperandSize::HundredTwentyEight => unreachable!(),
        OperandSize::Eight => unreachable!(),
    }
    cg.call_i(&Lvalue::Undefined,&a);
}

pub fn cmov(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue, c: Condition) {
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let fun = |f: &Lvalue,cg: &mut CodeGen<Amd64>| {
        let l = new_temp(bitwidth(&a.to_rv()));
        let nl = new_temp(bitwidth(&a.to_rv()));
        let n = new_temp(1);

        cg.lift_b(&l,&f.to_rv());
        cg.not_b(&n,&f.to_rv());
        cg.lift_b(&nl,&n);
        cg.mul_i(&l,&l,&b);
        cg.mul_i(&nl,&nl,&a.to_rv());
        cg.add_i(&a,&l,&nl);
    };

    match c {
        Condition::Overflow => fun(&*OF,cg),
        Condition::NotOverflow =>  {
            let nof = new_temp(1);
            cg.not_b(&nof,&OF.to_rv());
            fun(&nof,cg)
        },
        Condition::Carry => fun(&*CF,cg),
        Condition::AboveEqual => {
            let ncf = new_temp(1);
            cg.not_b(&ncf,&CF.to_rv());
            fun(&ncf,cg)
        },
        Condition::Equal => fun(&*ZF,cg),
        Condition::NotEqual => {
            let nzf = new_temp(1);
            cg.not_b(&nzf,&ZF.to_rv());
            fun(&nzf,cg)
        },
        Condition::BelowEqual => {
            let zc = new_temp(1);
            cg.or_b(&zc,&ZF.to_rv(),&CF.to_rv());
            fun(&zc,cg)
        },
        Condition::Above => {
            let zc = new_temp(1);
            cg.or_b(&zc,&ZF.to_rv(),&CF.to_rv());
            cg.not_b(&zc,&zc);
            fun(&zc,cg)
        },
        Condition::Sign => fun(&*SF,cg),
        Condition::NotSign => {
            let nsf = new_temp(1);
            cg.not_b(&nsf,&SF.to_rv());
            fun(&nsf,cg)
        },
        Condition::Parity => fun(&*PF,cg),
        Condition::NotParity => {
            let npf = new_temp(1);
            cg.not_b(&npf,&PF.to_rv());
            fun(&npf,cg)
        },
        Condition::Less => {
            let b = new_temp(1);
            cg.xor_b(&b,&SF.to_rv(),&OF.to_rv());
            cg.not_b(&b,&b.to_rv());
            fun(&b,cg)
        },
        Condition::GreaterEqual => {
            let b = new_temp(1);
            cg.xor_b(&b,&SF.to_rv(),&OF.to_rv());
            fun(&b,cg)
        },
        Condition::LessEqual => {
            let b = new_temp(1);
            cg.xor_b(&b,&SF.to_rv(),&OF.to_rv());
            cg.not_b(&b,&b.to_rv());
            cg.or_b(&b,&b,&ZF.to_rv());
            fun(&b,cg)
        },
        Condition::Greater => {
            let b = new_temp(1);
            let z = new_temp(1);
            cg.xor_b(&b,&SF.to_rv(),&OF.to_rv());
            cg.not_b(&z,&ZF.to_rv());
            cg.or_b(&b,&b,&z.to_rv());
            fun(&b,cg)
        },
    }
}

pub fn cmp(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.sub_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn cmps(cg: &mut CodeGen<Amd64>, aoff: Rvalue, boff: Rvalue) {
    let a = Lvalue::Memory{
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

    cg.sub_i(&res,&a.to_rv(),&b.to_rv());
    set_arithm_flags(&res,&res.to_rv(),&a.to_rv(),cg);

    cg.lift_b(&df,&DF.to_rv());
    cg.not_b(&n,&DF.to_rv());
    cg.lift_b(&ndf,&n.to_rv());

    cg.sub_i(&off,&df,&ndf);

    let ao = Lvalue::from_rvalue(&aoff).unwrap();
    let bo = Lvalue::from_rvalue(&boff).unwrap();
    cg.add_i(&ao,&aoff,&off);
    cg.add_i(&bo,&boff,&off);
}

pub fn cmpxchg(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) {
    cg.equal_i(&*ZF,&a,&EAX.to_rv());

    let n = new_temp(1);
    let zf = new_temp(32);
    let nzf = new_temp(32);
    let la = Lvalue::from_rvalue(&a).unwrap();

    cg.lift_b(&zf,&ZF.to_rv());
    cg.not_b(&n,&ZF.to_rv());
    cg.lift_b(&nzf,&n.to_rv());
    cg.mul_i(&zf,&zf,&b);
    cg.mul_i(&nzf,&nzf,&a);
    cg.add_i(&la,&zf,&nzf);

    cg.lift_b(&zf,&ZF.to_rv());
    cg.lift_b(&nzf,&n.to_rv());
    cg.mul_i(&zf,&zf,&EAX.to_rv());
    cg.add_i(&*EAX,&zf,&nzf);
}

pub fn or(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.or_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.to_rv());
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn sbb(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.sub_i(&res,&a,&b_ext);
    cg.sub_i(&res,&res.to_rv(),&CF.to_rv());
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.to_rv());
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn sub(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.sub_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.to_rv());
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn xor(cg: &mut CodeGen<Amd64>, _a: Rvalue, b: Rvalue) {
    let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.xor_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.to_rv(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.to_rv());
    set_arithm_flags(&res,&res_half.to_rv(),&a.to_rv(),cg);
}

pub fn cmpxchg8b(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn cmpxchg16b(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn cpuid(_: &mut CodeGen<Amd64>) {}
pub fn clc(_: &mut CodeGen<Amd64>) {}
pub fn cld(_: &mut CodeGen<Amd64>) {}
pub fn cli(_: &mut CodeGen<Amd64>) {}
pub fn cmc(_: &mut CodeGen<Amd64>) {}
pub fn std(_: &mut CodeGen<Amd64>) {}
pub fn sti(_: &mut CodeGen<Amd64>) {}
pub fn stc(_: &mut CodeGen<Amd64>) {}

pub fn conv(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"conv","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn conv2(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"conv2","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn daa(_: &mut CodeGen<Amd64>) {}
pub fn das(_: &mut CodeGen<Amd64>) {}
pub fn dec(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn div(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn enter(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn hlt(_: &mut CodeGen<Amd64>) {}
pub fn idiv(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn imul1(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn imul2(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn imul3(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn in_(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn icebp(_: &mut CodeGen<Amd64>) {}
pub fn inc(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn ins(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn int(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn into(_: &mut CodeGen<Amd64>) {}

pub fn iret(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"iret","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn jcc(_: &mut CodeGen<Amd64>, _: Rvalue, _: Condition) {}
pub fn jmp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn jcxz(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn jecxz(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn jrcxz(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn lahf(_: &mut CodeGen<Amd64>) {}
pub fn lar(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn lds(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,DS.to_rv()) }
pub fn les(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,ES.to_rv()) }
pub fn lss(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,SS.to_rv()) }
pub fn lfs(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,FS.to_rv()) }
pub fn lgs(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,GS.to_rv()) }
pub fn lxs(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn lea(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn leave(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"leave","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn lodsb(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"lodsb","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn lods(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"lods","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn loop_(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loop","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn loope(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loope","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn loopne(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loopne","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn mov(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movbe(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn movsb(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"movsb","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn movs(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"movs","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn movsx(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movzx(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mul(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn neg(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn nop(_: &mut CodeGen<Amd64>) {}
pub fn not(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn out(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn outs(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn pop(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"pop","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn popa(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"popa","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn popcnt(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn popf(_: &mut CodeGen<Amd64>, _: Rvalue) {}

pub fn push(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"push","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn pusha(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"pusha","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn pushf(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn rcl(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rcr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ret(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn retf(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn ror(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rol(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn sahf(_: &mut CodeGen<Amd64>) {}
pub fn sal(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn salc(_: &mut CodeGen<Amd64>) {}
pub fn sar(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn scas(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"scas","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn setcc(_: &mut CodeGen<Amd64>, _: Rvalue, _: Condition) {}
pub fn shl(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn shr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn shld(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn shrd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}

pub fn stos(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"stos","{}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::Constant(next),Guard::always());
    true
}

pub fn test(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ud1(_: &mut CodeGen<Amd64>) {}
pub fn ud2(_: &mut CodeGen<Amd64>) {}
pub fn xadd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn xchg(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn syscall(_: &mut CodeGen<Amd64>) {}
pub fn sysret(_: &mut CodeGen<Amd64>) {}
pub fn movapd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn vzeroupper(_: &mut CodeGen<Amd64>) {}

// MMX
pub fn emms(_: &mut CodeGen<Amd64>) {}
pub fn packsswb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn packssdw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn packuswb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddsb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddsw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddusb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddusw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pand(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pandn(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpeqb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpeqw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpeqd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpgtb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpgtw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpgtd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmadwd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmulhw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmullw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn por(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psraw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psrad(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psrlw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psrld(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psrlq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psllw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pslld(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psllq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubsb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubsw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubusb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubusw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpckhbw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpckhwd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpckhdq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpcklbw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpcklwd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpckldq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pxor(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

// SSE 1
pub fn addps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn addss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn andnps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn andps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cmpps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn cmpss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn comiss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtpi2ps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtps2pi(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtsi2ss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtss2si(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvttps2pi(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvttss2si(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn divps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn divss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ldmxcsr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn maskmovq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn maxps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn maxss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn minps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn minss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movaps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn minhps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movlps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movmskps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movntps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movntq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movups(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mulps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mulss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn orps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pavgb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pavgw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pextrw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pinsrw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pmaxsw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmaxub(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pminsw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pminub(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmovmskb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmulhuw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn prefetchnta(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn prefetcht0(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn prefetcht1(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn prefetcht2(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn prefetchw(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn prefetchwt1(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn psadbw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pshufw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn rcpps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rcpss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rsqrtps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rsqrtss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn sfence(_: &mut CodeGen<Amd64>) {}
pub fn shufps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn sqrtps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn sqrtss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn stmxcsr(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn subps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn subss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ucomiss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn unpckhps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn unpcklps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn xorps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

// SSE 2
pub fn addpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn addsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn andnpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn andpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cflush(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn cmppd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn cmpsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn comisd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtdq2pd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtdq2ps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtpd2dq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtpd2pi(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtpd2ps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtpi2pd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtps2dq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtps2pd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtsd2si(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtsd2ss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtsi2sd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvtss2sd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvttpd2dq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvttpd2pi(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvttps2dq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn cvttsd2si(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn divpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn divsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn lfence(_: &mut CodeGen<Amd64>) {}
pub fn maskmovdqu(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn maxpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn maxsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mfence(_: &mut CodeGen<Amd64>) {}
pub fn minpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn minsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movdq2q(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movdaq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movdqa(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movdqu(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movhpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movlpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movmskpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movntdq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movnti(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movntpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movq2dq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movupd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mulpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mulsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn orpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pabsb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pabsw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pabsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn paddq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pause(_: &mut CodeGen<Amd64>) {}
pub fn pmuludq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pshufd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pshufhw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pshuflw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pslldq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psarw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psrldq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn psubq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pusbsw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punckhwd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn punpckhqdq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn puncklqdq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn puncklwd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn shufpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn sqrtpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn sqrtsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn subpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn subsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ucomisd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn unpckhpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn unpcklpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn xorpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

// SSE 4
pub fn blendpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn blendps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn dppd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn dpps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn extractps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn insertps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn mpsadbw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pblendbw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pcmpestri(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pcmpestrm(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pcmpistri(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pcmpistrm(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pextrb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pextrd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pextrq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pinsrb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pinsrd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn pinsrq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn roundpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn roundps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn roundsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn roundss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}

// SSE 3
pub fn addsubpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn addsubps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn haddpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn haddps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn hsubpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn hsubps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn lddqu(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn monitor(_: &mut CodeGen<Amd64>) {}
pub fn movddup(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movshdup(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movsldup(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn mwait(_: &mut CodeGen<Amd64>) {}
pub fn palignr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}

// AVX
pub fn vaddpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaddps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaddsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaddss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
