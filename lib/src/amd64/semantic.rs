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

use {
    Lvalue,
    Rvalue,
    CodeGen,
    State,
    Guard,
};
use amd64::*;
/*
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
*/
pub fn aaa(_: &mut CodeGen<Amd64>) {
  /*  rreil!{cg:
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

pub fn aam(_: &mut CodeGen<Amd64>, _: Rvalue) {
 /*   let temp_al = new_temp(16);

    cg.assign(&temp_al,&AL.clone().into());
    cg.div_i(&*AH,&temp_al,&a);
    cg.mod_i(&*AL,&temp_al,&a);*/
}

pub fn aad(_: &mut CodeGen<Amd64>, _: Rvalue) {
 /*   let x = new_temp(16);

    cg.mul_i(&x,&AH.clone().into(),&a);
    cg.add_i(&*AL,&x,&AL.clone().into());
    cg.assign(&*AH,&Rvalue::new_bit(0));*/
}

pub fn aas(_: &mut CodeGen<Amd64>) {
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

/// res := a ? ?
fn set_aux_flag(cg: &mut CodeGen<Amd64>, res: &Lvalue, a: &Rvalue) {
    rreil!{cg:
        mov half_res:4, (res);
        mov half_a:4, (a);
        cmpeq af1:1, half_res:4, half_a:4;
        cmpltu af2:1, half_res:4, half_a:4;
        and af1:1, af1:1, CF:1;
        or AF:1, af1:1, af2:1;
    }
}

fn set_parity_flag(cg: &mut CodeGen<Amd64>, res: &Lvalue) {
    rreil!{cg:
        mov half_res:8, (res);
        xor PF:1, res:1, res:1/1;
        xor PF:1, PF:1, half_res:1/2;
        xor PF:1, PF:1, half_res:1/3;
        xor PF:1, PF:1, half_res:1/4;
        xor PF:1, PF:1, half_res:1/5;
        xor PF:1, PF:1, half_res:1/6;
        xor PF:1, PF:1, half_res:1/7;
    }
}

/// res := a ? ?
fn set_carry_flag(cg: &mut CodeGen<Amd64>, res: &Lvalue, a: &Rvalue) {
    rreil!{cg:
        cmpeq cf1:1, (res), (a);
        cmpltu cf2:1, (res), (a);
        and cf1:1, cf1:1, CF:1;
        or CF:1, cf1:1, cf2:1;
    }
}

/// Assumes res := a ? b
fn set_overflow_flag(cg: &mut CodeGen<Amd64>, res: &Lvalue, a: &Rvalue, b: &Rvalue, sz: usize) {
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
    rreil!{cg:
        cmples s1:1, [0]:sz, (a);
        cmples s2:1, [0]:sz, (b);
        cmplts s3:1, (res), [0]:sz;

        cmplts t1:1, (a), [0]:sz;
        cmplts t2:1, (b), [0]:sz;
        cmples t3:1, [0]:sz, (res);

        and ov1:1, s1:1, s2:1;
        and ov1:1, ov1:1, s3:1;

        and ov2:1, t1:1, t2:1;
        and ov2:1, ov2:1, t3:1;

        or OV:1, ov1:1, ov2:1;
    };
}

/// Returns (a/sz, b/sz, sz) w/ s = max(a.size,b.size)
fn sign_extend(cg: &mut CodeGen<Amd64>, a: &Rvalue, b: &Rvalue) -> (Rvalue,Rvalue,usize) {
    let sz = max(a.size().unwrap_or(0),b.size().unwrap_or(0));
    let ext = |x: &Rvalue,s: usize| -> Rvalue {
        match x {
            &Rvalue::Undefined => Rvalue::Undefined,
            &Rvalue::Variable{ ref name, ref subscript, ref offset,.. } =>
                Rvalue::Variable{
                    name: name.clone(),
                    subscript: subscript.clone(),
                    size: s + *offset,
                    offset: *offset
                },
            &Rvalue::Constant{ ref value,.. } =>
                Rvalue::Constant{ value: *value, size: s },
        }
    };

    let ext_a = ext(a,sz);
    let ext_b = ext(b,sz);

    assert!(sz > 0);
    assert!(ext_a.size() == None || ext_b.size() == None || ext_a.size() == ext_b.size());

    if a.size() != ext_a.size() {
        if let Some(lv) = Lvalue::from_rvalue(ext_a.clone()) {
            rreil!{cg:
                sext/sz (lv), (a);
            }
        }
    }

    if b.size() != ext_b.size() {
        if let Some(lv) = Lvalue::from_rvalue(ext_b.clone()) {
            rreil!{cg:
                sext/sz (lv), (b);
            }
        }
    }

    (ext_a,ext_b,sz)
}

fn write_reg(cg: &mut CodeGen<Amd64>, _reg: &Rvalue, _: &Rvalue, sz: usize) {
    if let Some(ref reg) = Lvalue::from_rvalue(_reg.clone()) {
        if sz < 64 {
            if let &Lvalue::Variable{ ref name,.. } = reg {
                if name == "RAX" || name == "RBX" || name == "RCX" ||
                   name == "RDX" || name == "RDI" || name == "RSI" ||
                   name == "RBP" || name == "RSP" || name == "R8" ||
                   name == "R9" || name == "R10" || name == "R11" ||
                   name == "R12" || name == "R13" || name == "R14" ||
                   name == "R15" {
                    rreil!{cg:
                        zext/64 reg:64, res:sz;
                    }
                    return
                }
            }
        }
        rreil!{cg:
            mov reg:sz, res:sz;
        };
    } else {
        unreachable!()
    }
}

pub fn adc(cg: &mut CodeGen<Amd64>, _a: Rvalue, _b: Rvalue) {
    let (a,b,sz) = sign_extend(cg,&_a,&_b);
    let res = rreil_lvalue!{ res:sz };

    rreil!{cg:
        add res:sz, (a), (b);
        zext/sz cf:sz, CF:1;
        add res:sz, res:sz, cf:sz;
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
    }

    set_carry_flag(cg,&res,&a);
    set_aux_flag(cg,&res,&a);
    set_overflow_flag(cg,&res,&a,&b,sz);
    set_parity_flag(cg,&res);
    write_reg(cg,&_a,&res.clone().into(),sz);
}

pub fn add(cg: &mut CodeGen<Amd64>, _a: Rvalue, _b: Rvalue) {
    let (a,b,sz) = sign_extend(cg,&_a,&_b);
    let res = rreil_lvalue!{ res:sz };

    rreil!{cg:
        add res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
    }

    set_carry_flag(cg,&res,&a);
    set_aux_flag(cg,&res,&a);
    set_overflow_flag(cg,&res,&a,&b,sz);
    set_parity_flag(cg,&res);
    write_reg(cg,&_a,&res.clone().into(),sz);
}

pub fn adcx(cg: &mut CodeGen<Amd64>, _a: Rvalue, _b: Rvalue) {
    let (a,b,sz) = sign_extend(cg,&_a,&_b);
    let res = rreil_lvalue!{ res:sz };

    rreil!{cg:
        add res:sz, (a), (b);
        zext/sz cf:sz, CF:1;
        add res:sz, res:sz, cf:sz;
    }

    set_carry_flag(cg,&res,&a);
    write_reg(cg,&_a,&res.clone().into(),sz);
}

pub fn and(cg: &mut CodeGen<Amd64>, _a: Rvalue, _b: Rvalue) {
    let (a,b,sz) = sign_extend(cg,&_a,&_b);
    let res = rreil_lvalue!{ res:sz };

    rreil!{cg:
        and res:sz, (a), (b);
        cmplts SF:1, res:sz, [0]:sz;
        cmpeq ZF:1, res:sz, [0]:sz;
        mov CF:1, [0]:1;
        mov OF:1, [0]:1;
    }

    set_parity_flag(cg,&res);
    write_reg(cg,&_a,&res.clone().into(),sz);
}

pub fn arpl(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn bound(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn bsf(cg: &mut CodeGen<Amd64>, _a: Rvalue, _b: Rvalue) {
    let (_,b,sz) = sign_extend(cg,&_a,&_b);
    let res = rreil_lvalue!{ res:sz };

    rreil!{cg:
        cmpeq ZF:1, (b), [0]:sz;
        mov res:sz, ?;
    }

    write_reg(cg,&_a,&res.clone().into(),sz);
}

pub fn bsr(cg: &mut CodeGen<Amd64>, _a: Rvalue, _b: Rvalue) {
    let (_,b,sz) = sign_extend(cg,&_a,&_b);
    let res = rreil_lvalue!{ res:sz };

    rreil!{cg:
        cmpeq ZF:1, (b), [0]:sz;
        mov res:sz, ?;
    }

    write_reg(cg,&_a,&res.clone().into(),sz);
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
       rreil!{cg:
        zext/64 new_rip:64, (a);
        call ?, new_rip:64;
    }
}

pub fn near_rcall(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    let bits = cg.configuration.operand_size.num_bits();
    rreil!{cg:
        call ?, (a.extract(bits,0).ok().unwrap());
    }

}

pub fn far_call(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    far_xcall(cg,a,false)
}

pub fn far_rcall(cg: &mut CodeGen<Amd64>, a: Rvalue) {
    far_xcall(cg,a,true)
}

pub fn far_xcall(cg: &mut CodeGen<Amd64>, a: Rvalue, _: bool) {
    let sz = a.size().unwrap();

    rreil!{cg:
        //mov seg:16, (a.extract(16,sz - 16).ok().unwrap());
        //mov CS:16, seg:16;
        mov new_ip:sz, (a);
    }

    near_call(cg,rreil_rvalue!{ new_ip:sz });
}

pub fn cmov(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Condition) {
 /*   let a = Lvalue::from_rvalue(&_a).unwrap();
    let fun = |f: &Lvalue,cg: &mut CodeGen<Amd64>| {
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

pub fn cmp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
 /*   let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.sub_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.clone().into(),&Rvalue::Constant(0x100));

    set_arithm_flags(&res,&res_half.clone().into(),&a.clone().into(),cg);*/
}

pub fn cmps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
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

pub fn cmpxchg(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
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

pub fn or(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
 /*   let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.or_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.clone().into(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.clone().into());
    set_arithm_flags(&res,&res_half.clone().into(),&a.clone().into(),cg);*/
}

pub fn sbb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
 /*   let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.sub_i(&res,&a,&b_ext);
    cg.sub_i(&res,&res.clone().into(),&CF.clone().into());
    cg.mod_i(&res_half,&res.clone().into(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.clone().into());
    set_arithm_flags(&res,&res_half.clone().into(),&a.clone().into(),cg);*/
}

pub fn sub(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
 /*   let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.sub_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.clone().into(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.clone().into());
    set_arithm_flags(&res,&res_half.clone().into(),&a.clone().into(),cg);*/
}

pub fn xor(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {
 /*   let aw = bitwidth(&_a);
    let bw = if let Rvalue::Constant(_) = b { aw } else { bitwidth(&b) };
    let res = new_temp(aw);
    let res_half = new_temp(8);
    let a = Lvalue::from_rvalue(&_a).unwrap();
    let b_ext = if aw == bw { b.clone() } else { sign_ext(&b,bw,aw,cg) };

    cg.xor_i(&res,&a,&b_ext);
    cg.mod_i(&res_half,&res.clone().into(),&Rvalue::Constant(0x100));

    cg.assign(&a,&res.clone().into());
    set_arithm_flags(&res,&res_half.clone().into(),&a.clone().into(),cg);*/
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

    st.mnemonic(len,"conv","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn conv2(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"conv2","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn daa(_: &mut CodeGen<Amd64>) {}
pub fn das(_: &mut CodeGen<Amd64>) {}
pub fn dec(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn div(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn enter(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn hlt(st: &mut State<Amd64>) -> bool {
    let len = st.tokens.len();
    st.mnemonic(len,"hlt","",vec![],&|_: &mut CodeGen<Amd64>| {} );
    true
}

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
    let len = st.tokens.len();
    st.mnemonic(len,"iret","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    true
}

pub fn jmp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn jcxz(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn jecxz(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn jrcxz(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn lahf(_: &mut CodeGen<Amd64>) {}
pub fn lar(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn lds(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,DS.clone().into()) }
pub fn les(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,ES.clone().into()) }
pub fn lss(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,SS.clone().into()) }
pub fn lfs(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,FS.clone().into()) }
pub fn lgs(cg: &mut CodeGen<Amd64>, a: Rvalue, b: Rvalue) { lxs(cg,a,b,GS.clone().into()) }
pub fn lxs(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn lea(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn leave(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"leave","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn lodsb(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"lodsb","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn lods(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"lods","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn loop_(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loop","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn loope(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loope","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn loopne(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"loopne","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn mov(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movbe(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn movsb(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"movsb","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn movs(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"movs","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
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

    st.mnemonic(len,"pop","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn popa(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"popa","",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn popcnt(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn popf(_: &mut CodeGen<Amd64>, _: Rvalue) {}

pub fn push(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"push","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn pusha(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"pusha","",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn pushf(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn rcl(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rcr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn ret(st: &mut State<Amd64>) -> bool {
    let len = st.tokens.len();
    st.mnemonic(len,"ret","",vec![],&|_: &mut CodeGen<Amd64>| {} );
    true
}

pub fn retf(st: &mut State<Amd64>) -> bool {
    let len = st.tokens.len();
    st.mnemonic(len,"retf","",vec![],&|_: &mut CodeGen<Amd64>| {} );
    true
}

pub fn retn(st: &mut State<Amd64>) -> bool {
    let len = st.tokens.len();
    if let Some(d) = decode::decode_imm(st) {
        st.mnemonic(len,"retn","{u}",vec![d],&|_: &mut CodeGen<Amd64>| {} );
        true
    } else {
        false
    }
}

pub fn retnf(st: &mut State<Amd64>) -> bool {
    let len = st.tokens.len();
    if let Some(d) = decode::decode_imm(st) {
        st.mnemonic(len,"retnf","{u}",vec![d],&|_: &mut CodeGen<Amd64>| {} );
        true
    } else {
        false
    }
}

pub fn ror(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rol(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn sahf(_: &mut CodeGen<Amd64>) {}
pub fn sal(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn salc(_: &mut CodeGen<Amd64>) {}
pub fn sar(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

pub fn scas(st: &mut State<Amd64>) -> bool {
    let next = st.address + (st.tokens.len() as u64);
    let len = st.tokens.len();

    st.mnemonic(len,"scas","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
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

    st.mnemonic(len,"stos","{u}",vec![],&|_: &mut CodeGen<Amd64>| {} );
    st.jump(Rvalue::new_u64(next),Guard::always());
    true
}

pub fn test(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ud1(_: &mut CodeGen<Amd64>) {}
pub fn ud2(_: &mut CodeGen<Amd64>) {}
pub fn xadd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn xchg(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn rdtsc(_: &mut CodeGen<Amd64>) {}
pub fn xgetbv(_: &mut CodeGen<Amd64>) {}

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
pub fn pshufb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
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
pub fn movhps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movlpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movmskpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movntdq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn movntdqa(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
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
pub fn blendvpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn blendvps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
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
pub fn pmovsx(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmovzx(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pminsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pminsb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pminud(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pminuw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmaxsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmaxsb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmaxud(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmaxuw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn ptest(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmulld(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pmuldq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn phaddw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn phaddd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn packusdw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pblendvb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn pcmpeqq(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn phminpushuw(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}

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
pub fn vaddsubpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaddsubps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaesdec(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaesdeclast(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaesenc(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaesenclast(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaesimc(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vaeskeygenassist(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vandpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vandps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vandnpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vandnps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vblendpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vblendps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vblendvpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vblendvps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcmppd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcmpps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcmpsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcmpss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcomisd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcomiss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtdq2pd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtdq2ps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtpd2dq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtpd2ps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtps2dq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtps2pd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtsd2si(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvtsd2ss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcvtsi2sd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcvtss2sd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcvtsi2ss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vcvttpd2dq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvttps2dq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvttsd2si(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vcvttss2si(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vdivps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vdivpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vdivss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vdivsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vdppd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vdpps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vextractps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vhaddpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vhaddps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vhsubpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vhsubps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vinsertps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vlddqu(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vldmxcsr(_: &mut CodeGen<Amd64>, _:Rvalue) {}
pub fn vmaxpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmaxsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmaxps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmaxss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vminpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vminsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vminps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vminss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmovhpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmovhps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmovlpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmovlps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmovsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmovss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmpsadbw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _:Rvalue, _: Rvalue) {}
pub fn vorpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vorps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpabsb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpabsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpabsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpacksswb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpackssdw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpackusdw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpackuswb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddsb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddusb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpaddusw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpalignr(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpand(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpandn(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpavgb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpavgw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpblendvb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpblendw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpclmulqdq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpeqb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpeqw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpeqd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpeqq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpgtb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpgtw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpgtd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpcmpgtq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphaddw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphaddd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphaddsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphminposuw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphsubw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphsubd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vphsubsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpinsrb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpinsrd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpinsrw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaddubsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmadwd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaxsb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaxsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaxsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaxub(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaxud(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaxuw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpminsb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpminsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpminsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpminub(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpminud(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpminuw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmuldq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmulhrsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmulhuw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmulhw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmulld(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmullw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmuludq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpor(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsadbw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsignb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsignw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsignd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpslldq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsllw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpslld(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsllq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsrad(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsarw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsrldq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsrlw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsrld(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsrlq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubsb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpusbsw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubusb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsubusw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vptest(_: &mut CodeGen<Amd64>) {}
pub fn vpunpckhbw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpunckhwd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpunpckhdq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpunpckhqdq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpunpcklbw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpunpckldq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpuncklqdq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpuncklwd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpxor(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vrcpps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vroundpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vroundps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vroundsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vroundss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vrsqrtps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vrsqrtss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vsqrtss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vsqrtsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vshufps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vshufpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vsubps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vsubss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vsubpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vsubsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vunpckhps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vunpcklps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vunpckhpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vunpcklpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vbroadcastss(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vbroadcastsd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vbroadcastf128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vextractf128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vextracti128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vgatherdd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vgatherdp(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vgatherpdp(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vgatherqpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vinsertf128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vinserti128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmaskmovps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmaskmovpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmulps(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmulss(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmulpd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vmulsd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vblendd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpboradcastb(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpboradcastw(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpboradcastd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpboradcastq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpboradcasti128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vpermd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpermpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpermps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpermq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vperm2i128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpermilpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpermilps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vperm2f128(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaskmovd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpmaskmovq(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsllvd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsravd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vpsrlvd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vtestpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vtestps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue) {}
pub fn vzeroall(_: &mut CodeGen<Amd64>) {}
pub fn vxorps(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}
pub fn vxorpd(_: &mut CodeGen<Amd64>, _:Rvalue, _: Rvalue, _: Rvalue) {}

// FPU
pub fn f2xm1(_: &mut CodeGen<Amd64>) {}
pub fn fabs(_: &mut CodeGen<Amd64>) {}
pub fn fadd(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn faddp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fiadd(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fbld(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fbstp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fchs(_: &mut CodeGen<Amd64>) {}
pub fn fclex(_: &mut CodeGen<Amd64>) {}
pub fn fnclex(_: &mut CodeGen<Amd64>) {}
pub fn fcmovb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmove(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmovbe(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmovu(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmovnb(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmovne(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmovnbe(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcmovnu(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcom(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcomp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fcompp(_: &mut CodeGen<Amd64>) {}
pub fn fcomi(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcomip(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fucomi(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fucomip(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fcos(_: &mut CodeGen<Amd64>) {}
pub fn fdecstp(_: &mut CodeGen<Amd64>) {}
pub fn fdiv(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fdivp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fidiv(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fdivr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fdivrp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fidivr(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn ffree(_: &mut CodeGen<Amd64>) {}
pub fn ficom(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn ficomp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fild(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fincstp(_: &mut CodeGen<Amd64>) {}
pub fn finit(_: &mut CodeGen<Amd64>) {}
pub fn fninit(_: &mut CodeGen<Amd64>) {}
pub fn fistp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fisttp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fld(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fld1(_: &mut CodeGen<Amd64>) {}
pub fn fldl2t(_: &mut CodeGen<Amd64>) {}
pub fn fldl2e(_: &mut CodeGen<Amd64>) {}
pub fn fldpi(_: &mut CodeGen<Amd64>) {}
pub fn fldlg2(_: &mut CodeGen<Amd64>) {}
pub fn fldln2(_: &mut CodeGen<Amd64>) {}
pub fn fldz(_: &mut CodeGen<Amd64>) {}
pub fn fldcw(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fmul(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fmulp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fimul(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fnop(_: &mut CodeGen<Amd64>) {}
pub fn fpatan(_: &mut CodeGen<Amd64>) {}
pub fn fprem(_: &mut CodeGen<Amd64>) {}
pub fn fprem1(_: &mut CodeGen<Amd64>) {}
pub fn fptan(_: &mut CodeGen<Amd64>) {}
pub fn frndint(_: &mut CodeGen<Amd64>) {}
pub fn frstor(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fsave(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fnsave(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fscale(_: &mut CodeGen<Amd64>) {}
pub fn fsin(_: &mut CodeGen<Amd64>) {}
pub fn fsincos(_: &mut CodeGen<Amd64>) {}
pub fn fsqrt(_: &mut CodeGen<Amd64>) {}
pub fn fst(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fstp(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fstcw(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fldenv(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fstenv(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fnstenv(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fstsw(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fnstsw(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fsub(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fsubp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fisub(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn fsubr(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fisubr(_: &mut CodeGen<Amd64>, _: Rvalue) {}
pub fn ftst(_: &mut CodeGen<Amd64>) {}
pub fn fucom(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fucomp(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fucompp(_: &mut CodeGen<Amd64>) {}
pub fn fxam(_: &mut CodeGen<Amd64>) {}
pub fn fxch(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn fxtract(_: &mut CodeGen<Amd64>) {}
pub fn fyl2x(_: &mut CodeGen<Amd64>) {}
pub fn fyl2xp1(_: &mut CodeGen<Amd64>) {}

// MPX
pub fn bndcl(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn bndcu(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn bndcn(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn bndmov(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn bndmk(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn bndldx(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
pub fn bndstx(_: &mut CodeGen<Amd64>, _: Rvalue, _: Rvalue) {}
