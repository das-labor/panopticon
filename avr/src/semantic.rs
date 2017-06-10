/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

use disassembler::{Avr, Mcu, optional_skip, reg, resolv};

use panopticon_core::{Guard, Lvalue, Result, Rvalue, State, Statement};
use std::borrow::Cow;

pub fn cpse(st: &mut State<Avr>) -> bool {
    let rd = reg(st, "cd");
    let rr = reg(st, "cr");
    let fallthru = st.configuration.wrap(st.address + 2);
    let skip = st.configuration.wrap(st.address + 4);
    let g = Guard::from_flag(&rreil_rvalue!{ skip_flag:1 }).ok().unwrap();

    st.mnemonic(
            2,
            "cpse",
            "{u}, {u}",
            vec![rd.clone().into(), rr.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            cmpeq skip_flag:1, (rr.clone()), (rd.clone());
        }
            },
        )
        .unwrap();

    optional_skip(fallthru.clone(), st);

    if st.tokens.len() == 1 {
        st.jump(skip, g.clone()).unwrap();
    } else {
        st.configuration.skip = Some((g.clone(), st.address));
    }

    st.jump(fallthru, g.negation()).unwrap();
    true
}

pub fn adc(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    let half_rd = if let &Lvalue::Variable { ref name, size: 8, .. } = &rd {
        Lvalue::Variable { name: name.clone(), size: 4, subscript: None }
    } else {
        unreachable!()
    };

    rreil!{
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

pub fn add(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    let half_rd = rd.extract(4, 0).ok().unwrap();

    rreil!{
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
    let rd1 = resolv(st.get_group("d") * 2 + 24);
    let rd2 = resolv(st.get_group("d") * 2 + 25);
    let k = Rvalue::new_u8(st.get_group("K") as u8);

    st.mnemonic(
            0,
            "__wide_reg",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 reg:16, (rd1);
            sel/8 reg:16, (rd2);
        }
            },
        )
        .unwrap();

    st.mnemonic(
            2,
            "adiw",
            "{u:8}, {u:8}",
            vec![rd1.clone().into(), k.clone()],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 imm:16, (k);
            add res:16, reg:16, imm:16;

            // zero flag
            cmpeq Z:1, res:16, [0]:16;

            // negative flag
            cmples N:1, res:16, [0]:16;

            // carry
            cmpltu C:1, res:16, reg:16;

            // overflow flag
            cmples s1:1, [0]:16, reg:16;
            cmples s2:1, [0]:16, imm:16;
            cmplts s3:1, res:16, [0]:16;

            cmplts t1:1, reg:16, [0]:16;
            cmplts t2:1, imm:16, [0]:16;
            cmples t3:1, [0]:16, res:16;

            and v1:1, s1:1, s2:1;
            and v1:1, v1:1, s3:1;

            and v2:1, t1:1, t2:1;
            and v2:1, v2:1, t3:1;

            or V:1, v1:1, v2:1;

            mov (rd1), res:8;
            mov (rd2), res:8/8;
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn and(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        and res:8, (rd), (rr);

        mov V:1, [0]:1;
        cmpeq Z:1, res:8, [0]:8;
        cmples N:1, res:8, [0]:8;
        cmples S:1, res:8, [0]:8;
    }
}

pub fn asr(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        mov lsb:1, C:1;
        cmpltu C:1, [0x7f]:8, (rd);
        shl (rd), (rd), [1]:8;
        sel/0 (rd), lsb:1;

        cmpeq Z:1, res:8, [0]:8;
        cmples N:1, res:8, [0]:8;
        xor V:1, N:1, C:1;
        mov S:1, C:1;
    }
}

pub fn _break(_: &mut Mcu) -> Result<Vec<Statement>> {
    Ok(vec![])
}

pub fn bld(rd: Lvalue, b: u64, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sel/b (rd), T:1;
    }
}

pub fn bst(rd: Lvalue, b: u64, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    let r: Rvalue = rd.extract(1, b as usize).ok().unwrap();

    rreil!{
        mov T:1, (r);
    }
}

pub fn call(st: &mut State<Avr>) -> bool {
    let k = st.configuration.wrap(st.get_group("k") * 2);
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    st.mnemonic(
            4,
            "call",
            "{c:flash}",
            vec![k.clone()],
            &|_cg: &mut Mcu| {
                rreil!{
            call ?, (k);
        }
            },
        )
        .unwrap();

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn cbx(rd: Lvalue, b: u64, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sel/b (rd), [0]:1;
    }
}

pub fn com(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sub res:8, [0xff]:8, (rd);
        mov C:1, [0]:1;
        cmpeq Z:1, [0]:8, res:8;
        cmplts N:1, res:8, [0]:8;
        mov V:1, [0]:1;
        mov S:1, N:1;
    }
}

pub fn cp(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    let half_rd: Rvalue = rd.extract(4, 0).ok().unwrap();

    rreil!{
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

pub fn cpc(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    let half_rd: Rvalue = rd.extract(4, 0).ok().unwrap();

    rreil!{
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

pub fn dec(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        cmpeq V:1, (rd), [0x80]:8;
        sub (rd), (rd), [1]:8;
        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn des(st: &mut State<Avr>) -> bool {
    let k = Rvalue::new_u8(st.get_group("K") as u8);
    st.mnemonic(
            2,
            "des",
            "{u}",
            vec![k],
            &|_cg: &mut Mcu| {
                rreil!{
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
            },
        )
        .unwrap();
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn eicall(_cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        zext/22 p:22, R30:8;
        sel/8 p:22, R31:8;
        sel/16 p:22, EIND:6;
        load/sram q:22, p:22;
        call ?, q:22;
    }
}

pub fn eijmp(st: &mut State<Avr>) -> bool {
    st.mnemonic(
            2,
            "eijmp",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/22 p:22, R30:8;
            sel/8 p:22, R31:8;
            sel/16 p:22, EIND:6;
            load/sram q:22, p:22;
        }
            },
        )
        .unwrap();

    let next = Rvalue::Variable {
        name: Cow::Borrowed("q"),
        size: 22,
        subscript: None,
        offset: 0,
    };

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn elpm(rd: Lvalue, off: usize, st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable { name: Cow::Borrowed("Z"), size: 24, subscript: None };

    st.mnemonic(
            0,
            "__wide_reg",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/24 (zreg), R30:8;
            sel/8 (zreg), R31:8;
            sel/16 (zreg), RAMPZ:8;
        }
            },
        )
        .unwrap();

    let arg = if rd == rreil_lvalue!{ R0:8 } {
        vec![]
    } else {
        vec![zreg.clone().into()]
    };
    st.mnemonic(
            2,
            "elpm",
            "{p:sram}",
            arg,
            &|_cg: &mut Mcu| {
                let mut stmts = rreil!{
            load/sram ptr:24, (zreg);
            load/flash (rd), ptr:24;
        }?;

                if off <= 1 {
                    stmts.append(
                        &mut rreil!{
                add (zreg), (zreg), [1]:24;
                mov R30:8, (zreg.extract(8,0).ok().unwrap());
                mov R31:8, (zreg.extract(8,8).ok().unwrap());
                mov RAMPZ:8, (zreg.extract(8,16).ok().unwrap());
            }?
                    );
                }

                Ok(stmts)
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn elpm1(st: &mut State<Avr>) -> bool {
    elpm(rreil_lvalue!{ R0:8 }, 0, st)
}

pub fn elpm2(st: &mut State<Avr>) -> bool {
    elpm(reg(st, "D"), 0, st)
}

pub fn elpm3(st: &mut State<Avr>) -> bool {
    elpm(reg(st, "D"), 1, st)
}

pub fn eor(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        xor res:8, (rd), (rr);

        mov V:1, [0]:1;
        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn fmul(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
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

pub fn fmuls(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
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

pub fn fmulsu(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
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
    let zreg = Lvalue::Variable { name: Cow::Borrowed("Z"), size: 16, subscript: None };

    st.mnemonic(
            0,
            "__wide_reg",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 (zreg), R30:8;
            sel/8 (zreg), R31:8;
        }
            },
        )
        .unwrap();

    st.mnemonic(
            2,
            "icall",
            "{p:sram}",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            load/sram ptr:24, (zreg);
            call ?, ptr:24;
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn ijmp(st: &mut State<Avr>) -> bool {
    let next = Lvalue::Variable { name: Cow::Borrowed("R30:R31"), size: 22, subscript: None };
    st.mnemonic(
            2,
            "ijmp",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/22 p:22, R30:8;
            sel/8 p:22, R31:8;
            sel/16 p:22, [0]:6;
            mov (next), p:22;
        }
            },
        )
        .unwrap();

    optional_skip(next.clone().into(), st);
    st.jump(next.into(), Guard::always()).unwrap();
    true
}

pub fn _in(st: &mut State<Avr>) -> bool {
    let rd = reg(st, "D");
    let rr = Rvalue::Constant { value: st.get_group("A"), size: 6 };

    st.mnemonic(
            2,
            "in",
            "{u}, {u}",
            vec![rd.clone().into(), rr.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            load/io (rd), (rr);
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn inc(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        cmpeq V:1, (rd), [0x80]:8;
        add (rd), (rd), [1]:8;
        cmpeq Z:1, res:8, [0]:8;
        cmpltu N:1, res:8, [0]:8;
        xor S:1, V:1, N:1;
    }
}

pub fn jmp(st: &mut State<Avr>) -> bool {
    let pc_mod = ((st.configuration.flashend + 1) * 2) as u64;
    let _k = (st.get_group("k") * 2) % pc_mod;
    let k = Rvalue::Constant { value: _k, size: st.configuration.pc_bits as usize };

    st.mnemonic(
            4,
            "jmp",
            "{c:flash}",
            vec![k.clone()],
            &|_: &mut Mcu| Ok(vec![]),
        )
        .unwrap();
    optional_skip(
        st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),
        st,
    );
    st.jump(k, Guard::always()).unwrap();
    true
}

pub fn lac(ptr: Lvalue, reg: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        load/sram zcont:8, (ptr);
        xor nreg:8, (reg), [0xff]:8;
        and (reg), zcont:8, nreg:8;
        store/sram (ptr), (reg);
    }
}

pub fn las(ptr: Lvalue, reg: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        load/sram zcont:8, (ptr);
        or (reg), (reg), zcont:8;
        store/sram (ptr), (reg);
    }
}

pub fn lat(ptr: Lvalue, reg: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        load/sram zcont:8, (ptr);
        xor (reg), (reg), zcont:8;
        store/sram (ptr), (reg);
    }
}

pub fn ld(ptr: Lvalue, reg: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        load/sram (reg), (ptr);
    }
}

pub fn ldi(rd: Lvalue, k: u64, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        mov (rd), [k]:8;
    }
}

pub fn lds1(st: &mut State<Avr>) -> bool {
    let rd = reg(st, "D");
    let k = Rvalue::new_u16(st.get_group("k") as u16);

    st.mnemonic(
            4,
            "lds",
            "{p:sram}, {u}",
            vec![rd.clone().into(), k.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            load/sram (rd), (k);
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn lds2(st: &mut State<Avr>) -> bool {
    let rd = resolv(st.get_group("d") + 16);
    let _k = st.get_group("k") as u16;
    let k = Rvalue::new_u16(if _k <= 0x1F { _k + 0x20 } else { _k });

    st.mnemonic(
            2,
            "lds",
            "{u}, {p:sram}",
            vec![rd.clone().into(), k.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            load/sram (rd), (k);
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn lpm(rd: Lvalue, off: usize, st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable { name: Cow::Borrowed("Z"), size: 16, subscript: None };

    st.mnemonic(
            0,
            "__wide_reg",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 (zreg), R30:8;
            sel/8 (zreg), R31:8;
        }
            },
        )
        .unwrap();

    let arg = if rd == rreil_lvalue!{ R0:8 } {
        vec![]
    } else {
        vec![zreg.clone().into()]
    };
    st.mnemonic(
            2,
            "lpm",
            "{p:sram}",
            arg,
            &|_cg: &mut Mcu| {
                let mut stmts = rreil!{
            load/sram ptr:16, (zreg);
            load/flash (rd), ptr:16;
        }?;

                if off <= 1 {
                    stmts.append(
                        &mut rreil!{
                add (zreg), (zreg), [1]:16;
                mov R30:8, (zreg.extract(8,0).ok().unwrap());
                mov R31:8, (zreg.extract(8,8).ok().unwrap());
            }?
                    );
                }

                Ok(stmts)
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn lpm1(st: &mut State<Avr>) -> bool {
    lpm(rreil_lvalue!{ R0:8 }, 0, st)
}

pub fn lpm2(st: &mut State<Avr>) -> bool {
    lpm(reg(st, "D"), 0, st)
}

pub fn lpm3(st: &mut State<Avr>) -> bool {
    lpm(reg(st, "D"), 1, st)
}

pub fn lsr(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        mov C:1, (rd.extract(1,0).ok().unwrap());
        shr (rd), (rd), [1]:8;
        mov N:1, [0]:1;
        cmpeq Z:1, (rd), [0]:8;
        xor V:1, C:1, N:1;
        xor S:1, V:1, N:1;
    }
}

pub fn mov(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        mov (rd), (rr);
    }
}

pub fn movw(st: &mut State<Avr>) -> bool {
    let rd1 = resolv(st.get_group("d") * 2);
    let rd2 = resolv(st.get_group("d") * 2 + 1);
    let rr1 = resolv(st.get_group("r") * 2);
    let rr2 = resolv(st.get_group("r") * 2 + 1);
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    st.mnemonic(
            2,
            "movw",
            "{u}, {u}",
            vec![rd1.clone().into(), rr1.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            mov (rd1), (rr1);
            mov (rd2), (rr2);
        }
            },
        )
        .unwrap();

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn mul(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        zext/16 rd:16, (rd);
        zext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;
        cmpeq Z:1, res:16, [0]:16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;
    }
}

pub fn muls(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sext/16 rd:16, (rd);
        sext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn mulsu(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sext/16 rd:16, (rd);
        zext/16 rr:16, (rr);

        mul res:16, rd:16, rr:16;

        mov C:1, res:1/16;

        mov R0:8, res:8;
        mov R1:8, res:8/8;

        cmpeq Z:1, res:16, [0]:16;
    }
}

pub fn neg(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sub res:8, [0]:8, (rd);

        cmplts N:1, res:8, [0]:8;
        cmpeq Z:1, res:8, [0]:8;
        cmpeq V:1, res:8, [0x80]:8;
        or H:1, res:1/3, (rd.extract(1,3).ok().unwrap());
        xor S:1, V:1, N:1;

        mov (rd), res:8;
    }
}

pub fn nop(_: &mut Mcu) -> Result<Vec<Statement>> {
    Ok(vec![])
}

pub fn or(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        or res:8, (rd), (rr);

        cmplts N:1, res:8, [0]:8;
        cmpeq Z:1, res:8, [0]:8;
        mov V:1, [0]:1;
        xor S:1, V:1, N:1;

        mov (rd), res:8;
    }
}


pub fn out(st: &mut State<Avr>) -> bool {
    let rd = Rvalue::Constant { value: st.get_group("A"), size: 6 };
    let rr = reg(st, "R");
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    st.mnemonic(
            2,
            "out",
            "{u}, {u}",
            vec![rd.clone().into(), rr.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            store/io (rr), (rd);
        }
            },
        )
        .unwrap();
    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn pop(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        zext/16 stack:16, spl:8;
        sel/8 stack:16, sph:8;
        add stack:16, stack:16, [1]:16;
        load/ram (rd), stack:16;
        mov spl:8, stack:8;
        mov sph:8, stack:8/8;
    }
}

pub fn push(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        zext/16 stack:16, spl:8;
        sel/8 stack:16, sph:8;
        load/ram (rd), stack:16;
        sub stack:16, stack:16, [1]:16;
        mov spl:8, stack:8;
        mov sph:8, stack:8/8;
    }
}

pub fn rcall(st: &mut State<Avr>) -> bool {
    let pc_mod = ((st.configuration.flashend + 1) * 2) as u64;
    let _k = (st.address + st.get_group("k") * 2 + 2) % pc_mod;
    let k = Rvalue::Constant { value: _k, size: st.configuration.pc_bits };
    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    st.mnemonic(
            2,
            "rcall",
            "{c:flash}",
            vec![k.clone()],
            &|_cg: &mut Mcu| {
                rreil!{
        call ?, (k);
    }
            },
        )
        .unwrap();

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn ret(_: &mut Mcu) -> Result<Vec<Statement>> {
    Ok(vec![])
}

pub fn rjmp(st: &mut State<Avr>) -> bool {
    let pc_mod = ((st.configuration.flashend + 1) * 2) as u64;
    let _k = (st.address + st.get_group("k") * 2 + 2) % pc_mod;
    let k = Rvalue::Constant { value: _k, size: st.configuration.pc_bits };

    st.mnemonic(
            2,
            "rjmp",
            "{c:flash}",
            vec![k.clone()],
            &|_: &mut Mcu| Ok(vec![]),
        )
        .unwrap();
    optional_skip(
        st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),
        st,
    );
    st.jump(k, Guard::always()).unwrap();
    true
}

pub fn ror(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        mov nc:1, (rd.extract(1,7).ok().unwrap());
        shr (rd), (rd), [1]:8;
        sel/1 (rd), C:1;
        mov C:1, nc:1;
        mov N:1, [0]:1;
        cmpeq Z:1, (rd), [0]:8;
        xor V:1, C:1, N:1;
        xor S:1, V:1, N:1;
    }
}

pub fn sbc(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
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

pub fn sbi(rd: Lvalue, b: u64, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        sel/b (rd), [1]:1;
    }
}

pub fn sbiw(st: &mut State<Avr>) -> bool {
    let rd1 = resolv(st.get_group("d") * 2 + 24);
    let rd2 = resolv(st.get_group("d") * 2 + 25);
    let k = Rvalue::new_u8(st.get_group("K") as u8);

    st.mnemonic(
            0,
            "__wide_reg",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 reg:16, (rd1);
            sel/8 reg:16, (rd2);
        }
            },
        )
        .unwrap();

    st.mnemonic(
            2,
            "sbiw",
            "{u:8}, {u:8}",
            vec![rd1.clone().into(), k.clone()],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 reg:16, (rd1);
            sel/8 reg:16, (rd2);
            zext/16 imm:16, k:8;

            sub res:16, reg:16, imm:16;

            // zero flag
            cmpeq Z:1, res:16, [0]:16;

            // negative flag
            cmples N:1, res:16, [0]:16;

            // carry
            cmpltu C:1, reg:16, imm:16;

            // overflow flag
            cmplts V:1, res:16, imm:16;

            // sign test flag
            xor S:1, N:1, V:1;

            mov (rd1), res:8;
            mov (rd2), res:8/8;
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn sleep(_: &mut Mcu) -> Result<Vec<Statement>> {
    Ok(vec![])
}

pub fn spm(rd: Lvalue, off: usize, st: &mut State<Avr>) -> bool {
    let zreg = Lvalue::Variable {
        name: if off == 0 {
            Cow::Borrowed("Z")
        } else {
            Cow::Borrowed("Z+")
        },
        size: 16,
        subscript: None,
    };
    let len = st.tokens.len() * 2;

    st.mnemonic(
            0,
            "__wide_reg",
            "",
            vec![],
            &|_cg: &mut Mcu| {
                rreil!{
            zext/16 (zreg), R30:8;
            sel/8 (zreg), R31:8;
        }
            },
        )
        .unwrap();

    let arg = if off == 0 {
        vec![]
    } else {
        vec![zreg.clone().into()]
    };
    st.mnemonic(
            len,
            "spm",
            "{p:sram}",
            arg,
            &|_cg: &mut Mcu| {
                let mut stmts = rreil!{
            load/sram ptr:16, (zreg);
            load/flash ptr:16, (rd);
        }?;

                if off <= 1 {
                    stmts.append(
                        &mut rreil!{
                add (zreg), (zreg), [1]:16;
                mov R30:8, (zreg.extract(8,0).ok().unwrap());
                mov R31:8, (zreg.extract(8,8).ok().unwrap());
            }?
                    );
                }

                Ok(stmts)
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn spm1(st: &mut State<Avr>) -> bool {
    spm(rreil_lvalue!{ R0:8 }, 0, st)
}

pub fn spm2(st: &mut State<Avr>) -> bool {
    spm(reg(st, "D"), 0, st)
}

pub fn st(ptr: Lvalue, reg: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        load/sram (ptr), (reg);
    }
}

pub fn sts1(st: &mut State<Avr>) -> bool {
    let rd = reg(st, "R");
    let k = Rvalue::new_u16(st.get_group("k") as u16);

    st.mnemonic(
            4,
            "sts",
            "{p:sram}, {u}",
            vec![k.clone().into(), rd.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            store/sram (rd), (k);
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn sts2(st: &mut State<Avr>) -> bool {
    let rd = resolv(st.get_group("r") + 16);
    let _k = st.get_group("k") as u16;
    let k = Rvalue::new_u16(if _k <= 0x1F { _k + 0x20 } else { _k });

    st.mnemonic(
            2,
            "sts",
            "{p:sram}, {u}",
            vec![k.clone().into(), rd.clone().into()],
            &|_cg: &mut Mcu| {
                rreil!{
            store/sram (rd), (k);
        }
            },
        )
        .unwrap();

    let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

    optional_skip(next.clone(), st);
    st.jump(next, Guard::always()).unwrap();
    true
}

pub fn sub(rd: Lvalue, rr: Rvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
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

pub fn swap(rd: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        mov tmp:4, (rd.extract(4,0).ok().unwrap());
        sel/0 (rd), (rd.extract(4,4).ok().unwrap());
        sel/4 (rd), tmp:4;
    }
}

pub fn wdr(_: &mut Mcu) -> Result<Vec<Statement>> {
    Ok(vec![])
}

pub fn xch(ptr: Lvalue, reg: Lvalue, _cg: &mut Mcu) -> Result<Vec<Statement>> {
    rreil!{
        load/sram zcont:8, (ptr);
        store/sram (ptr), (reg);
        mov (reg), zcont:8;
    }
}
