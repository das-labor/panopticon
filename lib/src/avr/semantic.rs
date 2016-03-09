use std::borrow::Cow;

use {
    Guard,
    Rvalue,
    Lvalue,
    CodeGen,
    State,
};
use super::*;

pub fn cpse(st: &mut State<Avr>) -> bool {
    let rd = reg(st,"d");
    let rr = reg(st,"r");
    let fallthru = st.configuration.wrap(st.address + 2);
    let skip = st.configuration.wrap(st.address + 4);
    let g = Guard::from_flag(&rreil_rvalue!{ skip_flag:1 }).ok().unwrap();

    st.mnemonic(2,"cpse","{u}, {u}",vec!(rd.clone().into(),rr.clone().into()),&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            cmpeq skip_flag:1, (rr.clone()), (rd.clone());
        }
    });

    if st.tokens.len() == 1 {
        st.jump(skip,g.clone());
    } else {
        st.configuration.skip = Some((g.clone(),st.address));
    }

    st.jump(fallthru,g.negation());
    true
}

pub fn adc(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    let half_rd = if let &Lvalue::Variable{ ref name, size: 8, ref subscript, offset: 0 } = &rd {
        Lvalue::Variable{
            name: name.clone(),
            size: 4,
            subscript: None,
            offset: 0,
        }
    } else {
        unreachable!()
    };

    rreil!{cg:
        zext/8 carry:8, C:1;
        add res:8, (rd), (rr);
        add res:8, res:8, carry:8;

        // zero flag
        cmpeq Z:1, res:8, [0]:8;

        // negative flag
        cmples N:1, res:8, [0]:8;

        // carry
        cmpeq cf1:1, res:8, (rd);
        cmpltu cf2:1, res:8, (rd);
        and cf1:1, cf1:1, C:1;
        or C:1, cf1:1, cf2:1;

        // half carry
        cmpeq h1:1, res:4, (half_rd);
        cmpltu h2:1, res:4, (half_rd);
        and h1:1, h1:1, H:1;
        or H:1, h1:1, h2:1;

        // overflow flag
        cmples s1:1, [0]:8, (rd);
        cmples s2:1, [0]:8, (rr);
        cmplts s3:1, res:8, [0]:8;

        cmplts t1:1, (rd), [0]:8;
        cmplts t2:1, (rr), [0]:8;
        cmples t3:1, [0]:8, res:8;

        and v1:1, s1:1, s2:1;
        and v1:1, v1:1, s3:1;

        and v2:1, t1:1, t2:1;
        and v2:1, v2:1, t3:1;

        or V:1, v1:1, v2:1;

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd), res:8;
    }
}

pub fn add(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    let half_rd = rd.extract(4,0).ok().unwrap();

    rreil!{cg:
        add res:8, (rd), (rr);

        // zero flag
        cmpeq Z:1, res:8, [0]:8;

        // negative flag
        cmples N:1, res:8, [0]:8;

        // carry
        cmpltu C:1, res:8, (rd);

        // half carry
        cmpltu H:1, res:4, (half_rd);

        // overflow flag
        cmples s1:1, [0]:8, (rd);
        cmples s2:1, [0]:8, (rr);
        cmplts s3:1, res:8, [0]:8;

        cmplts t1:1, (rd), [0]:8;
        cmplts t2:1, (rr), [0]:8;
        cmples t3:1, [0]:8, res:8;

        and v1:1, s1:1, s2:1;
        and v1:1, v1:1, s3:1;

        and v2:1, t1:1, t2:1;
        and v2:1, v2:1, t3:1;

        or V:1, v1:1, v2:1;

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd), res:8;
    }
}

pub fn adiw(st: &mut State<Avr>) -> bool {
    let rd1 = reg(st,"d");
    let rd2 = resolv(st.get_group("d") + 1);
    let rd = rreil_lvalue!{ reg:16 };
    let k = Rvalue::new_u8(st.get_group("k") as u8);

    st.mnemonic(0,"__wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/16 reg:16, (rd1);
            mov reg:8/8, (rd2);
        }
    });

    st.mnemonic(2,"adiw","{u:16}, {u:8}",vec!(rd.clone().into(),k.clone()),&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/16 imm:16, (k);
            add res:16, reg:16, imm:16;

            // zero flag
            cmpeq Z:1, res:16, [0]:16;

            // negative flag
            cmples N:1, res:16, [0]:16;

            // caky
            cmpltu C:1, res:8, reg:16;

            // overflow flag
            cmples s1:1, [0]:16, reg:16;
            cmples s2:1, [0]:16, (k);
            cmplts s3:1, res:16, [0]:16;

            cmplts t1:1, reg:16, [0]:16;
            cmplts t2:1, (k), [0]:16;
            cmples t3:1, [0]:16, res:16;

            and v1:1, s1:1, s2:1;
            and v1:1, v1:1, s3:1;

            and v2:1, t1:1, t2:1;
            and v2:1, v2:1, t3:1;

            or V:1, v1:1, v2:1;

            mov (rd1), res:8;
            mov (rd2), res:8/8;
        }
    });

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn and(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        and res:8, (rd), (rr);

        mov V:1, [0]:1;
        cmpeq Z:1, res:8, [0]:8;
        cmples N:1, res:8, [0]:8;
        cmples S:1, res:8, [0]:8;
    }
}

pub fn asr(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    let lsb_rd = rd.extract(1,0).ok().unwrap();

    rreil!{cg:
        mov lsb:1, C:1;
        cmpltu C:1, [0x7f]:8, (rd);
        shl (rd), (rd), [1]:8;
        mov (lsb_rd), lsb:1;

        cmpeq Z:1, res:8, [0]:8;
        cmples N:1, res:8, [0]:8;
        xor V:1, N:1, C:1;
        mov S:1, C:1;
    }
}

pub fn _break(cg: &mut CodeGen<Avr>) {}

pub fn bld(rd: Lvalue, b: u64, cg: &mut CodeGen<Avr>) {
    let r = rd.extract(1,b as usize).ok().unwrap();

    rreil!{cg:
        mov (r), T:1;
    }
}

pub fn bst(rd: Lvalue, b: u64, cg: &mut CodeGen<Avr>) {
    let r = rd.extract(1,b as usize).ok().unwrap();

    rreil!{cg:
        mov T:1, (r);
    }
}

pub fn call(st: &mut State<Avr>) -> bool {
    let k = st.configuration.wrap(st.get_group("k") * 2);
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    st.mnemonic(4,"call","{c:flash}",vec![k.clone()],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            call ?, (k);
        }
    });

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn cbx(rd: Lvalue, b: u64, cg: &mut CodeGen<Avr>) {
    let r = rd.extract(1,b as usize).ok().unwrap();

    rreil!{cg:
        mov (r), [0]:1;
    }
}

pub fn com(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sub res:8, [0xff]:8, (rd);
        mov C:1, [0]:1;
        cmpeq Z:1, [0]:8, res:8;
        cmplts N:1, res:8, [0]:8;
        mov V:1, [0]:1;
        mov S:1, N:1;
    }
}

pub fn cp(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    let half_rd = rd.extract(4,0).ok().unwrap();
    let half_rr = rr.extract(4,0).ok().unwrap();

    rreil!{cg:
        sub res:8, (rd), (rr);

        // carry
        cmpltu C:1, (rd), res:8;

        // half carry
        cmpltu H:1, (half_rd), res:4;

        // overflow flag
        cmples s1:1, (rd), [0]:8;
        cmples s2:1, (rr), [0]:8;
        cmplts s3:1, [0]:8, res:8;

        cmplts t1:1, [0]:8, (rd);
        cmplts t2:1, [0]:8, (rr);
        cmples t3:1, res:8, [0]:8;

        and v1:1, s1:1, s2:1;
        and v1:1, v1:1, s3:1;

        and v2:1, t1:1, t2:1;
        and v2:1, v2:1, t3:1;

        or V:1, v1:1, v2:1;

        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn cpc(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    let half_rd = rd.extract(4,0).ok().unwrap();
    let half_rr = rr.extract(4,0).ok().unwrap();

    rreil!{cg:
        zext/8 carry:8, C:1;
        sub res:8, (rd), (rr);
        sub res:8, res:8, carry:8;

        // carry
        cmpeq cf1:1, res:8, (rd);
        cmpltu cf2:1, res:8, (rd);
        and cf1:1, cf1:1, C:1;
        or C:1, cf1:1, cf2:1;

        // half carry
        cmpeq h1:1, res:4, (half_rd);
        cmpltu h2:1, res:4, (half_rd);
        and h1:1, h1:1, H:1;
        or H:1, h1:1, h2:1;

        // overflow flag
        cmples s1:1, (rd), [0]:8;
        cmples s2:1, (rr), [0]:8;
        cmplts s3:1, [0]:8, res:8;

        cmplts t1:1, [0]:8, (rd);
        cmplts t2:1, [0]:8, (rr);
        cmples t3:1, res:8, [0]:8;

        and v1:1, s1:1, s2:1;
        and v1:1, v1:1, s3:1;

        and v2:1, t1:1, t2:1;
        and v2:1, v2:1, t3:1;

        or V:1, v1:1, v2:1;

        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn dec(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        cmpeq V:1, (rd), [0x80]:8;
        sub (rd), (rd), [1]:8;
        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn des(st: &mut State<Avr>) -> bool {
    let k = Rvalue::new_u8(st.get_group("K") as u8);
    st.mnemonic(2,"des","{u}",vec![k],&|cg: &mut CodeGen<Avr>| {
    rreil!{cg:
        mov R0:8, ?;
        mov R1:8, ?;
        mov R2:8, ?;
        mov R3:8, ?;
        mov R4:8, ?;
        mov R5:8, ?;
        mov R6:8, ?;
        mov R7:8, ?;
        mov R8:8, ?;
        mov R9:8, ?;
        mov R10:8, ?;
        mov R11:8, ?;
        mov R12:8, ?;
        mov R13:8, ?;
        mov R14:8, ?;
        mov R15:8, ?;
    }
    });
let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn eicall(cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        zext/22 p:22, R30:8;
        mov p:8/8, R31:8;
        mov p:6/16, EIND:6;
        load/sram q:22, p:22;
        call ?, q:22;
    }
}

pub fn eijmp(st: &mut State<Avr>) -> bool {
    st.mnemonic(2,"eijmp","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/22 p:22, R30:8;
            mov p:8/8, R31:8;
            mov p:6/16, EIND:6;
            load/sram q:22, p:22;
        }
    });

    let next = Rvalue::Variable{
        name: Cow::Borrowed("q"),
        size: 22,
        subscript: None,
        offset: 0,
    };

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn elpm(rd: Lvalue, off: usize, st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable{
        name: Cow::Borrowed("Z"),
        size: 24,
        subscript: None,
        offset: 0,
    };

    st.mnemonic(0,"__wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/24 (zreg), R30:8;
            mov (zreg.extract(8,8).ok().unwrap()), R31:8;
            mov (zreg.extract(8,16).ok().unwrap()), RAMPZ:8;
        }
    });

    st.mnemonic(0,"elpm","{p:sram}",vec![zreg.clone().into()],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            load/sram ptr:24, (zreg);
            load/flash (rd), ptr:24;
        }
    });

    if off <= 1 {
        st.mnemonic(0,"__inc_wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
            rreil!{cg:
                add (zreg), (zreg), [1]:24;
                mov R30:8, (zreg.extract(8,0).ok().unwrap());
                mov R31:8, (zreg.extract(8,8).ok().unwrap());
                mov RAMPZ:8, (zreg.extract(8,16).ok().unwrap());
            }
        });
    }

    true
}

pub fn elpm1(st: &mut State<Avr>) -> bool {
    elpm(rreil_lvalue!{ R0:8 },0,st)
}

pub fn elpm2(st: &mut State<Avr>) -> bool {
    elpm(reg(st,"d"),0,st)
}

pub fn elpm3(st: &mut State<Avr>) -> bool {
    elpm(reg(st,"d"),1,st)
}

pub fn eor(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        xor res:8, (rd), (rr);

        mov V:1, [0]:1;
        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn fmul(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        zext/16 rd:16, (rd);
        zext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;
        shl res:16, res:16, [1]:16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn fmuls(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sext/16 rd:16, (rd);
        sext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;
        shl res:16, res:16, [1]:16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn fmulsu(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sext/16 rd:16, (rd);
        zext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;
        shl res:16, res:16, [1]:16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn icall(st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable{
        name: Cow::Borrowed("Z"),
        size: 16,
        subscript: None,
        offset: 0,
    };

    st.mnemonic(0,"__wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/16 (zreg), R30:8;
            mov (zreg.extract(8,8).ok().unwrap()), R31:8;
        }
    });

    st.mnemonic(0,"icall","{p:sram}",vec![zreg.clone().into()],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            load/sram ptr:24, (zreg);
            call ?, ptr:24;
        }
    });

    true
}

pub fn ijmp(st: &mut State<Avr>) -> bool {
    st.mnemonic(2,"ijmp","",vec![],&|cg: &mut CodeGen<Avr>| {
    rreil!{cg:
        zext/22 p:22, R30:8;
        mov p:8/8, R31:8;
        mov p:6/16, EIND:6;
        load/sram q:22, p:22;
    }
    });

    let next = Rvalue::Variable{
        name: Cow::Borrowed("q"),
        size: 22,
        subscript: None,
        offset: 0
    };

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn _in(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov (rd), (rr);
    }
}

pub fn inc(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        cmpeq V:1, (rd), [0x80]:8;
        add (rd), (rd), [1]:8;
        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn jmp(st: &mut State<Avr>) -> bool {
    let pc_mod = 1 << st.configuration.pc_bits;
    let _k = (st.get_group("k") % pc_mod) * 2;
    let k = Rvalue::Constant{ value: _k, size: 24 };

    st.mnemonic(4,"jmp","{c:flash}",vec!(k.clone()),&|_: &mut CodeGen<Avr>| {});
    optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
    st.jump(k,Guard::always());
    true
}

pub fn lac(ptr: Lvalue, reg: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram zcont:8, (ptr);
        xor nreg:8, (reg), [0xff]:8;
        and (reg), zcont:8, nreg:8;
        store/sram (ptr), (reg);
    }
}

pub fn las(ptr: Lvalue, reg: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram zcont:8, (ptr);
        or (reg), (reg), zcont:8;
        store/sram (ptr), (reg);
    }
}

pub fn lat(ptr: Lvalue, reg: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram zcont:8, (ptr);
        xor (reg), (reg), zcont:8;
        store/sram (ptr), (reg);
    }
}

pub fn ld(ptr: Lvalue, reg: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram (reg), (ptr);
    }
}

pub fn ldi(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov (rd), [k]:8;
    }
}

pub fn lds1(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram (rd), [k]:16;
    }
}

pub fn lds2(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    let j = if k <= 0x1F { k + 0x20 } else { k };
    rreil!{cg:
        load/sram (rd), [j]:16;
    }
}

pub fn lpm(rd: Lvalue, off: usize, st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable{
        name: Cow::Borrowed("Z"),
        size: 16,
        subscript: None,
        offset: 0,
    };

    st.mnemonic(0,"__wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/16 (zreg), R30:8;
            mov (zreg.extract(8,8).ok().unwrap()), R31:8;
        }
    });

    st.mnemonic(0,"lpm","{p:sram}",vec![zreg.clone().into()],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            load/sram ptr:16, (zreg);
            load/flash (rd), ptr:16;
        }
    });

    if off <= 1 {
        st.mnemonic(0,"__inc_wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
            rreil!{cg:
                add (zreg), (zreg), [1]:16;
                mov R30:8, (zreg.extract(8,0).ok().unwrap());
                mov R31:8, (zreg.extract(8,8).ok().unwrap());
            }
        });
    }

    true
}

pub fn lpm1(st: &mut State<Avr>) -> bool {
    lpm(rreil_lvalue!{ R0:8 },0,st)
}

pub fn lpm2(st: &mut State<Avr>) -> bool {
    lpm(reg(st,"d"),0,st)
}

pub fn lpm3(st: &mut State<Avr>) -> bool {
    lpm(reg(st,"d"),1,st)
}

pub fn lsr(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov C:1, (rd.extract(1,0).ok().unwrap());
        shr (rd), (rd), [1]:8;
        mov N:1, [0]:1;
        cmpeq Z:1, (rd), [0]:8;
        xor V:1, C:1, N:1;
        xor S:1, V:1, N:1;
    }
}

pub fn mov(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov (rd), (rr);
    }
}

pub fn movw(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov (rd), (rr);
    }
}

pub fn mul(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        zext/16 rd:16, (rd);
        zext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;
        cmpeq Z:1, res:16, [0]:16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;
    }
}

pub fn muls(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sext/16 rd:16, (rd);
        sext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn mulsu(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sext/16 rd:16, (rd);
        zext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn neg(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sub res:8, [0]:8, (rd);

        cmplts N:1, res:8, [0]:8;
        cmpeq Z:1, res:8, [0]:8;
        cmpeq V:1, res:8, [0x80]:8;
        or H:1, res:1/3, (rd.extract(1,3).ok().unwrap());
        xor S:1, V:1, N:1;

        mov (rd), res:8;
    }
}

pub fn nop(cg: &mut CodeGen<Avr>) {}

pub fn or(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        or res:8, (rd), (rr);

        cmplts N:1, res:8, [0]:8;
        cmpeq Z:1, res:8, [0]:8;
        mov V:1, [0]:1;
        xor S:1, V:1, N:1;

        mov (rd), res:8;
    }
}

pub fn out(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov (rd), (rr);
    }
}


pub fn pop(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        zext/16 stack:16, spl:8;
        mov stack:16/8, sph:8;
        add stack:16, stack:16, [1]:16;
        load/ram (rd), stack:16;
        mov spl:8, stack:8;
        mov sph:8, stack:16/8;
    }
}

pub fn push(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        zext/16 stack:16, spl:8;
        mov stack:16/8, sph:8;
        load/ram (rd), stack:16;
        sub stack:16, stack:16, [1]:16;
        mov spl:8, stack:8;
        mov sph:8, stack:16/8;
    }
}

pub fn rcall(st: &mut State<Avr>) -> bool {
    let k = st.configuration.wrap(st.get_group("k") * 2 + st.address);
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    st.mnemonic(2,"rcall","{c:flash}",vec![k.clone()],&|cg: &mut CodeGen<Avr>| {
    rreil!{cg:
        call ?, (k);
    }
    });

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn ret(cg: &mut CodeGen<Avr>) {}

pub fn rjmp(st: &mut State<Avr>) -> bool {
    let pc_mod = 1 << st.configuration.pc_bits;
    let _k = (st.get_group("k") % pc_mod) * 2;
    let k = Rvalue::Constant{ value: _k, size: 24 };

    st.mnemonic(2,"rjmp","{c:flash}",vec!(k.clone()),&|_: &mut CodeGen<Avr>| {});
    optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
    st.jump(k,Guard::always());
    true
}

pub fn ror(rd: Lvalue, cg: &mut CodeGen<Avr>) {
 rreil!{cg:
        mov nc:1, (rd.extract(1,7).ok().unwrap());
        shr (rd), (rd), [1]:8;
        mov (rd.extract(1,0).ok().unwrap()), C:1;
        mov C:1, nc:1;
        mov N:1, [0]:1;
        cmpeq Z:1, (rd), [0]:8;
        xor V:1, C:1, N:1;
        xor S:1, V:1, N:1;
    }
}

pub fn sbc(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        zext/8 carry:8, C:1;
        sub res:8, (rd), (rr);
        sub res:8, res:8, carry:8;

        // zero flag
        cmpeq maybe_z:1, res:8, [0]:8;
        and Z:1, Z:1, maybe_z:1;

        // negative flag
        cmples N:1, res:8, [0]:8;

        // carry
        cmpltu C:1, (rd), (rr);

        // half carry
        cmpltu H:1, (rd.extract(4,0).ok().unwrap()), (rr.extract(4,0).ok().unwrap());

        // overflow flag
        cmplts V:1, (rd), (rr);

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd), res:8;
    }
}

pub fn sbci(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov k:8, [k]:8;
        zext/8 carry:8, C:1;
        sub res:8, (rd), k:8;
        sub res:8, res:8, carry:8;

        // zero flag
        cmpeq maybe_z:1, res:8, [0]:8;
        and Z:1, Z:1, maybe_z:1;

        // negative flag
        cmples N:1, res:8, [0]:8;

        // carry
        cmpltu C:1, (rd), k:8;

        // half carry
        cmpltu H:1, (rd.extract(4,0).ok().unwrap()), k:4;

        // overflow flag
        cmplts V:1, (rd), k:8;

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd), res:8;
    }
}

pub fn sbi(rd: Lvalue, b: u64, cg: &mut CodeGen<Avr>) {
    let r = rd.extract(1,b as usize).ok().unwrap();

    rreil!{cg:
        mov (r), [1]:1;
    }
}

pub fn sbiw(st: &mut State<Avr>) -> bool {
        let rd1 = reg(st,"d");
    let rd2 = resolv(st.get_group("d") + 1);
    let rd = rreil_lvalue!{ reg:16 };
    let k = Rvalue::new_u8(st.get_group("k") as u8);

    st.mnemonic(0,"__wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/16 reg:16, (rd1);
            mov reg:8/8, (rd2);
        }
    });

    st.mnemonic(2,"sbiw","{u:16}, {u:8}",vec!(rd.clone().into(),k.clone()),&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
        zext/16 reg:16, (rd1);
        mov reg:8/8, (rd2);

        zext/16 carry:16, C:1;
        sub res:16, reg:16, reg:16;
        sub res:16, res:16, carry:16;

        // zero flag
        cmpeq Z:1, res:16, [0]:16;

        // negative flag
        cmples N:1, res:16, [0]:8;

        // carry
        cmpltu C:1, reg:16, k:8;

        // overflow flag
        cmplts V:1, res:16, k:8;

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd1), res:8;
        mov (rd2), res:8/8;
    }
    });

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(),st);
    st.jump(next,Guard::always());
    true
}

pub fn sleep(cg: &mut CodeGen<Avr>) {}

pub fn spm(rd: Lvalue, off: usize, st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable{
        name: Cow::Borrowed("Z"),
        size: 16,
        subscript: None,
        offset: 0,
    };

    st.mnemonic(0,"__wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            zext/16 (zreg), R30:8;
            mov (zreg.extract(8,8).ok().unwrap()), R31:8;
        }
    });

    st.mnemonic(0,"spm","{p:sram}",vec![zreg.clone().into()],&|cg: &mut CodeGen<Avr>| {
        rreil!{cg:
            load/sram ptr:16, (zreg);
            load/flash ptr:16, (rd);
        }
    });

    if off <= 1 {
        st.mnemonic(0,"__inc_wide_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
            rreil!{cg:
                add (zreg), (zreg), [1]:16;
                mov R30:8, (zreg.extract(8,0).ok().unwrap());
                mov R31:8, (zreg.extract(8,8).ok().unwrap());
            }
        });
    }

    true
}

pub fn spm1(st: &mut State<Avr>) -> bool {
    spm(rreil_lvalue!{ R0:8 },0,st)
}

pub fn spm2(st: &mut State<Avr>) -> bool {
    spm(reg(st,"d"),0,st)
}

pub fn spm3(st: &mut State<Avr>) -> bool {
    spm(reg(st,"d"),1,st)
}

pub fn st(ptr: Lvalue, reg: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram (ptr), (reg);
    }
}

pub fn sts1(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram (rd), [k]:16;
    }
}

pub fn sts2(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    let j = if k <= 0x1F { k + 0x20 } else { k };
    rreil!{cg:
        load/sram (rd), [j]:16;
    }
}

pub fn sub(rd: Lvalue, rr: Rvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sub res:8, (rd), (rr);

        // zero flag
        cmpeq Z:1, res:8, [0]:8;

        // negative flag
        cmples N:1, res:8, [0]:8;

        // (half) carry
        cmpltu C:1, (rd), (rr);
        cmpltu H:1, (rd.extract(4,0).ok().unwrap()), (rr.extract(4,0).ok().unwrap());

        // overflow flag
        cmplts V:1, (rd), (rr);

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd), res:8;
    }
}

pub fn subi(rd: Lvalue, k: u64, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        sub res:8, (rd), [k]:8;

        // zero flag
        cmpeq Z:1, res:8, [0]:8;

        // negative flag
        cmples N:1, res:8, [0]:8;

        // (half) carry
        cmpltu C:1, (rd), [k]:8;
        cmpltu H:1, (rd.extract(4,0).ok().unwrap()), [k]:4;

        // overflow flag
        cmplts V:1, (rd), [k]:8;

        // sign test flag
        xor S:1, N:1, V:1;

        mov (rd), res:8;
    }
}

pub fn swap(rd: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        mov tmp:4, (rd.extract(4,0).ok().unwrap());
        mov (rd.extract(4,0).ok().unwrap()), (rd.extract(4,4).ok().unwrap());
        mov (rd.extract(4,4).ok().unwrap()), tmp:4;
    }
}

pub fn wdr(cg: &mut CodeGen<Avr>) {}

pub fn xch(ptr: Lvalue, reg: Lvalue, cg: &mut CodeGen<Avr>) {
    rreil!{cg:
        load/sram zcont:8, (ptr);
        store/sram (ptr), (reg);
        mov (reg), zcont:8;
    }
}
