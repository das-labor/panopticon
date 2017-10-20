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

use panopticon_core::{Architecture, Guard, Lvalue, Match, Region, Result, Rvalue, State, Statement};
use std::borrow::Cow;
use syntax;

#[derive(Clone,Debug)]
pub enum Mos {}

impl Architecture for Mos {
    type Token = u8;
    type Configuration = Variant;

    fn prepare(reg: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
        let iv = vec![
            ("NMI", 0xfffa, "NMI vector"),
            ("RESET", 0xfffc, "Reset routine"),
            ("IRQ/BRK", 0xfffe, "Interrupt routine"),
        ];
        let mut ret = vec![];

        for v in iv {
            let mut j = reg.iter(v.1);
            let maybe_lo = j.next();
            let maybe_hi = j.next();
            if let (Some(hi), Some(lo)) = (maybe_hi, maybe_lo) {
                let addr = ((*hi as u64) << 8) | (*lo as u64);

                ret.push((v.0, addr, v.2))
            }
        }

        Ok(ret)
    }

    fn decode(reg: &Region, addr: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
        info!("disass @ {:x}", addr);
        let disass = syntax::disassembler();

        if let Some(st) = disass.next_match(&mut reg.iter(addr), addr, cfg.clone()) {
            info!("    res: {:?}", st);
            Ok(st.into())
        } else {
            Err("Unrecognized instruction".into())
        }
    }
}

// 8 bit main register
lazy_static! {
    pub static ref A: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("A"), size: 8, subscript: None };
}

// 8 bit index registers
lazy_static! {
    pub static ref X: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("X"), size: 8, subscript: None };
    pub static ref Y: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("Y"), size: 8, subscript: None };
    pub static ref SP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("SP"), size: 8, subscript: None };
}

/*
// 16 bit program counter
lazy_static! {
    pub static ref PC: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("PC"), size: 16, subscript: None };
}
*/

// flags
lazy_static! {
    pub static ref N: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("N"), size: 1, subscript: None };
    pub static ref V: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("V"), size: 1, subscript: None };
    //pub static ref D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("D"), size: 1, subscript: None };
    //pub static ref I: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("I"), size: 1, subscript: None };
    pub static ref Z: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("Z"), size: 1, subscript: None };
    pub static ref C: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("C"), size: 1, subscript: None };
}

#[derive(Clone,Debug)]
pub struct Variant {
    pub arg: Option<Rvalue>,
    pub rel: Option<i16>,
}

impl Variant {
    pub fn mos6502() -> Variant {
        Variant { arg: None, rel: None }
    }
}

// No argument
pub fn nonary(opcode: &'static str, sem: fn(&mut Variant) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;

            st.mnemonic_dynargs(
                    len,
                    &opcode,
                    "",
                    &|c| -> Result<(Vec<Rvalue>, Vec<Statement>)> { Ok((vec![], sem(c)?)) },
                )
                .unwrap();
            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}

// RT*
pub fn ret(opcode: &'static str) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            st.mnemonic(
                    len,
                    &opcode,
                    "",
                    vec![],
                    &|_| -> Result<Vec<Statement>> { Ok(vec![]) },
                )
                .unwrap();
            true
        }
    )
}

// Implied register argument
pub fn implied(opcode: &'static str, _arg0: &Lvalue, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    let arg0 = _arg0.clone();
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            st.mnemonic(
                    len,
                    &opcode,
                    "",
                    vec![],
                    &|c| -> Result<Vec<Statement>> { sem(c, arg0.clone().into()) },
                )
                .unwrap();
            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}

// Immediate
pub fn immediate(opcode: &'static str, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let _arg = st.configuration.arg.clone();
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            if let Some(arg) = _arg {
                st.mnemonic_dynargs(
                        len,
                        &opcode,
                        "#{u}",
                        &|c| -> Result<(Vec<Rvalue>, Vec<Statement>)> { Ok((vec![arg.clone()], sem(c, arg.clone())?)) },
                    )
                    .unwrap();
                st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
                true
            } else {
                false
            }
        }
    )
}

// Index into Zero Page
pub fn zpage(opcode: &'static str, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            let base = st.configuration.arg.clone().unwrap();

            st.mnemonic(
                    len,
                    &opcode,
                    "{p:ram}",
                    vec![base.clone()],
                    &|c| -> Result<Vec<Statement>> {
                        let mut stmts = rreil!{
                zext/16 addr:16, (base);
                load/ram/be/8 val:8, addr:16;
            }?;

                        stmts.append(&mut sem(c, rreil_rvalue!{ val:8 })?);

                        Ok(stmts)
                    },
                )
                .unwrap();
            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}

// Index into Zero Page with register offset
pub fn zpage_offset(opcode: &'static str, _arg1: &Lvalue, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    let index = _arg1.clone();
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            let base = st.configuration.arg.clone().unwrap();
            let base_val = if let Rvalue::Constant { ref value, .. } = base {
                *value
            } else {
                unreachable!()
            };
            let index_nam = if let Lvalue::Variable { ref name, .. } = index {
                name.clone()
            } else {
                unreachable!()
            };
            let addr = Lvalue::Variable {
                name: Cow::Owned(format!("${:02X},{}", base_val, index_nam)),
                size: 16,
                subscript: None,
            };

            st.mnemonic(
                    0,
                    "__load",
                    "",
                    vec![],
                    &|_c| -> Result<Vec<Statement>> {
                        rreil!{
                add short_addr:8, (base), (index);
                zext/16 (addr), short_addr:8;
                load/ram/be/8 val:8, (addr);
            }
                    },
                )
                .unwrap();

            st.mnemonic(
                    len,
                    &opcode,
                    "{p:ram}",
                    vec![addr.clone().into()],
                    &|c| -> Result<Vec<Statement>> { sem(c, rreil_rvalue!{ val:8 }) },
                )
                .unwrap();

            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}


pub fn zpage_index(opcode: &'static str, _arg1: Lvalue, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    let index = _arg1.clone();
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            let base = st.configuration.arg.clone().unwrap();
            let base_val = if let Rvalue::Constant { ref value, .. } = base {
                *value
            } else {
                unreachable!()
            };
            let index_nam = if let Lvalue::Variable { ref name, .. } = index {
                name.clone()
            } else {
                unreachable!()
            };
            let addr = if index == rreil_lvalue!{ X:8 } {
                Lvalue::Variable {
                    name: Cow::Owned(format!("(${:02X},{})", base_val, index_nam)),
                    size: 16,
                    subscript: None,
                }
            } else {
                Lvalue::Variable {
                    name: Cow::Owned(format!("(${:02X}),{}", base_val, index_nam)),
                    size: 16,
                    subscript: None,
                }
            };

            st.mnemonic(
                    0,
                    "__load",
                    "",
                    vec![],
                    &|_c| -> Result<Vec<Statement>> {
                        rreil!{
                add short_addr:8, (base), (index);
                zext/16 addr:16, short_addr:8;
                load/ram/be/16 (addr), addr:16;
                load/ram/be/8 val:8, (addr);
            }
                    },
                )
                .unwrap();

            st.mnemonic(
                    len,
                    &opcode,
                    "{p:ram}",
                    vec![addr.clone().into()],
                    &|c| -> Result<Vec<Statement>> { sem(c, rreil_rvalue!{ val:8 }) },
                )
                .unwrap();
            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}

pub fn absolute(opcode: &'static str, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            let base = st.configuration.arg.clone().unwrap();

            st.mnemonic(
                    len,
                    &opcode,
                    "{p:ram}",
                    vec![base.clone()],
                    &|c| -> Result<Vec<Statement>> {
                        let mut stmts = rreil!{
                load/ram/be/8 val:8, (base);
            }?;

                        stmts.append(&mut sem(c, rreil_rvalue!{ val:8 })?);

                        Ok(stmts)
                    },
                )
                .unwrap();
            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}

pub fn absolute_offset(opcode: &'static str, _arg1: &Lvalue, sem: fn(&mut Variant, Rvalue) -> Result<Vec<Statement>>) -> Box<Fn(&mut State<Mos>) -> bool> {
    let index = _arg1.clone();
    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let len = st.tokens.len();
            let next = (st.address + len as u64) as u16;
            let base = st.configuration.arg.clone().unwrap();
            let base_val = if let Rvalue::Constant { ref value, .. } = base {
                *value
            } else {
                unreachable!()
            };
            let index_nam = if let Lvalue::Variable { ref name, .. } = index {
                name.clone()
            } else {
                unreachable!()
            };
            let addr = Lvalue::Variable {
                name: Cow::Owned(format!("${:04X},{}", base_val, index_nam)),
                size: 16,
                subscript: None,
            };

            st.mnemonic(
                    0,
                    "__load",
                    "",
                    vec![],
                    &|_c| {
                        rreil!{
                zext/16 (addr), (index);
                add (addr), (addr), (base);
                load/ram/be/8 val:8, (addr);
            }
                    },
                )
                .unwrap();

            st.mnemonic(
                    len,
                    &opcode,
                    "{p:ram}",
                    vec![addr.clone().into()],
                    &|c| -> Result<Vec<Statement>> { sem(c, rreil_rvalue!{ val:8 }) },
                )
                .unwrap();
            st.jump(Rvalue::new_u16(next), Guard::always()).unwrap();
            true
        }
    )
}

/* Relative branch.  */
pub fn branch(opcode: &'static str, _flag: &Lvalue, _set: bool) -> Box<Fn(&mut State<Mos>) -> bool> {
    let flag = _flag.clone();
    let set = if _set {
        rreil_rvalue!{ [1]:1 }
    } else {
        rreil_rvalue!{ [0]:1 }
    };

    Box::new(
        move |st: &mut State<Mos>| -> bool {
            let rel = st.configuration.rel.unwrap();
            let len = st.tokens.len();
            let fallthru = (st.address + len as u64) as u16;
            let g = Guard::from_flag(&flag.clone().into()).ok().unwrap();
            let k = (st.address as i16).wrapping_add(rel) as u16;

            st.mnemonic(
                    2,
                    opcode,
                    "{c:ram}",
                    vec![rreil_rvalue!{ [k]:16 }],
                    &|_c| -> Result<Vec<Statement>> {
                        rreil!{
                cmpeq flag:1, (set), (flag);
            }
                    },
                )
                .unwrap();

            st.jump(Rvalue::new_u16(fallthru), g.negation()).unwrap();
            st.jump(Rvalue::new_u16(k), g).unwrap();
            true
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::syntax::disassembler;
    use panopticon_core::{Region, Rvalue};
    use std::borrow::Cow;

    #[test]
    fn all() {
        let test_vectors = vec![
            // LDA
            (vec![0xa9, 0x0e], "lda", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xad, 0x0e, 0xab], "lda", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xbd, 0x00, 0x80],
             "lda",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xb9, 0x00, 0x80],
             "lda",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xa5, 0x80], "lda", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xb5, 0x80],
             "lda",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xa1, 0x80],
             "lda",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xb1, 0x80],
             "lda",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // LDX
            (vec![0xa2, 0x0e], "ldx", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xae, 0x0e, 0xab], "ldx", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xbe, 0x00, 0x80],
             "ldx",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xa6, 0x80], "ldx", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xb6, 0x80],
             "ldx",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // LDY
            (vec![0xa0, 0x0e], "ldy", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xac, 0x0e, 0xab], "ldy", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xbc, 0x00, 0x80],
             "ldy",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xa4, 0x80], "ldy", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xb4, 0x80],
             "ldy",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // STA
            (vec![0x8d, 0x0e, 0xab], "sta", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x9d, 0x00, 0x80],
             "sta",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x99, 0x00, 0x80],
             "sta",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x85, 0x80], "sta", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x95, 0x80],
             "sta",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x81, 0x80],
             "sta",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x91, 0x80],
             "sta",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // STX
            (vec![0x8e, 0x0e, 0xab], "stx", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x86, 0x80], "stx", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x96, 0x80],
             "stx",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // STY
            (vec![0x8c, 0x0e, 0xab], "sty", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x84, 0x80], "sty", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x94, 0x80],
             "sty",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // T**
            (vec![0xaa], "tax", vec![]),
            (vec![0xa8], "tay", vec![]),
            (vec![0x8a], "txa", vec![]),
            (vec![0x98], "tya", vec![]),
            (vec![0xba], "tsx", vec![]),
            (vec![0x9a], "txs", vec![]),

            // AND
            (vec![0x29, 0x0e], "and", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0x2d, 0x0e, 0xab], "and", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x3d, 0x00, 0x80],
             "and",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x39, 0x00, 0x80],
             "and",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x25, 0x80], "and", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x35, 0x80],
             "and",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x21, 0x80],
             "and",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x31, 0x80],
             "and",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // ORA
            (vec![0x09, 0x0e], "ora", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0x0d, 0x0e, 0xab], "ora", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x1d, 0x00, 0x80],
             "ora",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x19, 0x00, 0x80],
             "ora",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x05, 0x80], "ora", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x15, 0x80],
             "ora",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x01, 0x80],
             "ora",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x11, 0x80],
             "ora",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // EOR
            (vec![0x49, 0x0e], "eor", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0x4d, 0x0e, 0xab], "eor", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x5d, 0x00, 0x80],
             "eor",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x59, 0x00, 0x80],
             "eor",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x45, 0x80], "eor", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x55, 0x80],
             "eor",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x41, 0x80],
             "eor",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x51, 0x80],
             "eor",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // ADC
            (vec![0x69, 0x0e], "adc", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0x6d, 0x0e, 0xab], "adc", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x7d, 0x00, 0x80],
             "adc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x79, 0x00, 0x80],
             "adc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x65, 0x80], "adc", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x75, 0x80],
             "adc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x61, 0x80],
             "adc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x71, 0x80],
             "adc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // SBC
            (vec![0xe9, 0x0e], "sbc", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xed, 0x0e, 0xab], "sbc", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xfd, 0x00, 0x80],
             "sbc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xf9, 0x00, 0x80],
             "sbc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xe5, 0x80], "sbc", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xf5, 0x80],
             "sbc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xe1, 0x80],
             "sbc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xf1, 0x80],
             "sbc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // INC
            (vec![0xee, 0x0e, 0xab], "inc", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xfe, 0x00, 0x80],
             "inc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xe6, 0x80], "inc", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xf6, 0x80],
             "inc",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // DEC
            (vec![0xce, 0x0e, 0xab], "dec", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xde, 0x00, 0x80],
             "dec",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xc6, 0x80], "dec", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xd6, 0x80],
             "dec",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // IN*
            (vec![0xe8], "inx", vec![]),
            (vec![0xc8], "iny", vec![]),

            // DE*
            (vec![0xca], "dex", vec![]),
            (vec![0x88], "dey", vec![]),

            // ASL
            (vec![0x0a], "asl", vec![]),
            (vec![0x0e, 0x0e, 0xab], "asl", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x1e, 0x00, 0x80],
             "asl",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x06, 0x80], "asl", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x16, 0x80],
             "asl",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // LSR
            (vec![0x4a], "lsr", vec![]),
            (vec![0x4e, 0x0e, 0xab], "lsr", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x5e, 0x00, 0x80],
             "lsr",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x46, 0x80], "lsr", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x56, 0x80],
             "lsr",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // ROL
            (vec![0x2a], "rol", vec![]),
            (vec![0x2e, 0x0e, 0xab], "rol", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x3e, 0x00, 0x80],
             "rol",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x26, 0x80], "rol", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x36, 0x80],
             "rol",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // ROR
            (vec![0x6a], "ror", vec![]),
            (vec![0x6e, 0x0e, 0xab], "ror", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x7e, 0x00, 0x80],
             "ror",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0x66, 0x80], "ror", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0x76, 0x80],
             "ror",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // CMP
            (vec![0xc9, 0x0e], "cmp", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xcd, 0x0e, 0xab], "cmp", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xdd, 0x00, 0x80],
             "cmp",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xd9, 0x00, 0x80],
             "cmp",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$8000,Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xc5, 0x80], "cmp", vec![Rvalue::Constant { value: 0x80, size: 8 }]),
            (vec![0xd5, 0x80],
             "cmp",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("$80,X"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xc1, 0x80],
             "cmp",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80,X)"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),
            (vec![0xd1, 0x80],
             "cmp",
             vec![
                Rvalue::Variable {
                    name: Cow::Borrowed("($80),Y"),
                    subscript: None,
                    size: 16,
                    offset: 0,
                },
            ]),

            // CPX
            (vec![0xe0, 0x0e], "cpx", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xec, 0x0e, 0xab], "cpx", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xe4, 0x80], "cpx", vec![Rvalue::Constant { value: 0x80, size: 8 }]),

            // CPY
            (vec![0xc0, 0x0e], "cpy", vec![rreil_rvalue!{ [0x0e]:8 }]),
            (vec![0xcc, 0x0e, 0xab], "cpy", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0xc4, 0x80], "cpy", vec![Rvalue::Constant { value: 0x80, size: 8 }]),

            // BIT
            (vec![0x2c, 0x0e, 0xab], "bit", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x24, 0x80], "bit", vec![Rvalue::Constant { value: 0x80, size: 8 }]),

            // JMP
            (vec![0x4c, 0x0e, 0xab], "jmp", vec![rreil_rvalue!{ [0xab0e]:16 }]),
            (vec![0x6c, 0x0e, 0xab], "jmp", vec![rreil_rvalue!{ [0xab0e]:16 }]),

            // JSR
            (vec![0x20, 0x0e, 0xab], "jsr", vec![rreil_rvalue!{ [0xab0e]:16 }]),

            // RT*
            (vec![0x60], "rts", vec![]),
            (vec![0x40], "rti", vec![]),

            // B**
            (vec![0x90, 0x0e], "bcc", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0xb0, 0x0e], "bcs", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0xd0, 0x0e], "bne", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0xf0, 0x0e], "beq", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0x10, 0x0e], "bpl", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0x30, 0x0e], "bmi", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0x50, 0x0e], "bvc", vec![rreil_rvalue!{ [0x0e]:16 }]),
            (vec![0x70, 0x0e], "bvs", vec![rreil_rvalue!{ [0x0e]:16 }]),

            // SE*
            (vec![0x38], "sec", vec![]),
            (vec![0xf8], "sed", vec![]),
            (vec![0x78], "sei", vec![]),

            // CL*
            (vec![0x18], "clc", vec![]),
            (vec![0xd8], "cld", vec![]),
            (vec![0x58], "cli", vec![]),
            (vec![0xb8], "clv", vec![]),

            // NOP
            (vec![0xea], "nop", vec![]),

            // P**
            (vec![0x68], "pla", vec![]),
            (vec![0x48], "pha", vec![]),
            (vec![0x28], "plp", vec![]),
            (vec![0x08], "php", vec![]),

            // BRK
            (vec![0x00, 0x00], "brk", vec![]),
        ];
        let main = disassembler();

        for (bytes, opname, args) in test_vectors {
            println!("check '{}'", opname);

            let l = bytes.len();
            let reg = Region::wrap("base".to_string(), bytes);
            let mut i = reg.iter().seek(0);
            let maybe_match = main.next_match(&mut i, 0, Variant::mos6502());

            if let Some(match_st) = maybe_match {
                assert!(match_st.mnemonics.len() >= 1);

                let mne = &match_st.mnemonics.last().unwrap();

                assert_eq!(opname, mne.opcode);
                assert_eq!(mne.area.start, 0);
                assert_eq!(mne.area.end, l as u64);
                assert_eq!(mne.operands, args);
            } else {
                unreachable!()
            }
        }
    }
}
