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

use std::convert::Into;
use std::borrow::Cow;

use disassembler::*;
use {Lvalue,Rvalue};
use CodeGen;
use Guard;

pub mod syntax;
pub mod semantic;

#[derive(Clone)]
pub enum Avr {}

impl Architecture for Avr {
    type Token = u16;
    type Configuration = Mcu;
}

#[derive(Clone)]
pub struct Mcu {
    pub pc_bits: u16,                                   ///< width of the program counter in bits (FLASHEND)
    pub int_vec: Vec<(&'static str,Rvalue,&'static str)>,  ///< interrupt vector: (name, offset, comment)
    pub skip: Option<(Guard,u64)>,
}

impl Mcu {
    pub fn new() -> Mcu {
        Mcu {
            pc_bits: 22,
            int_vec: vec![/*("RESET",Rvalue::Constant(0),"MCU Reset Interrupt")*/],
            skip: None,
        }
    }

    pub fn atmega103() -> Mcu {
        Mcu {
            pc_bits: 16,
            int_vec: vec![
                /*("RESET",Rvalue::Constant(0),"MCU Reset Interrupt"),
                ("INT0",Rvalue::Constant(0x02),"External Interrupt 0"),
                ("INT1",Rvalue::Constant(0x04),"External Interrupt 1"),
                ("INT2",Rvalue::Constant(0x06),"External Interrupt 2"),
                ("INT3",Rvalue::Constant(0x08),"External Interrupt 3"),
                ("INT4",Rvalue::Constant(0x0a),"External Interrupt 4"),
                ("INT5",Rvalue::Constant(0x0c),"External Interrupt 5"),
                ("INT6",Rvalue::Constant(0x0e),"External Interrupt 6"),
                ("INT7",Rvalue::Constant(0x10),"External Interrupt 7"),
                ("OC2",Rvalue::Constant(0x12),"Timer/Counter2 Compare Match"),
                ("OVF2",Rvalue::Constant(0x14),"Timer/Counter2 Overflow"),
                ("ICP1",Rvalue::Constant(0x16),"Timer/Counter1 Capture Event"),
                ("OC1A",Rvalue::Constant(0x18),"Timer/Counter1 Compare Match A"),
                ("OC1B",Rvalue::Constant(0x1a),"Timer/Counter1 Compare Match B"),
                ("OVF1",Rvalue::Constant(0x1c),"Timer/Counter1 Overflow"),
                ("OC0",Rvalue::Constant(0x1e),"Timer/Counter0 Compare Match"),
                ("OVF0",Rvalue::Constant(0x20),"Timer/Counter0 Overflow"),
                ("SPI",Rvalue::Constant(0x22),"SPI Serial Transfer Complete"),
                ("URXC",Rvalue::Constant(0x24),"UART, Rx Complete"),
                ("UDRE",Rvalue::Constant(0x26),"UART Data Register Empty"),
                ("UTXC",Rvalue::Constant(0x28),"UART, Tx Complete"),
                ("ADCC",Rvalue::Constant(0x2a),"ADC Conversion Complete"),
                ("ERDY",Rvalue::Constant(0x2c),"EEPROM Ready"),
                ("ACI",Rvalue::Constant(0x2e),"Analog Comparator"),*/
            ],
            skip: None,
        }
    }

    pub fn atmega8() -> Mcu {
        Mcu {
            pc_bits: 13,
            int_vec: vec![
               /* ("RESET",Rvalue::Constant(0),"MCU Reset Interrupt"),
                ("INT0",Rvalue::Constant(0x01),"External Interrupt Request 0"),
                ("INT1",Rvalue::Constant(0x02),"External Interrupt Request 1"),
                ("OC2",Rvalue::Constant(0x03),"Timer/Counter2 Compare Match"),
                ("OVF2",Rvalue::Constant(0x04),"Timer/Counter2 Overflow"),
                ("ICP1",Rvalue::Constant(0x05),"Timer/Counter1 Capture Event"),
                ("OC1A",Rvalue::Constant(0x06),"Timer/Counter1 Compare Match A"),
                ("OC1B",Rvalue::Constant(0x07),"Timer/Counter1 Compare Match B"),
                ("OVF1",Rvalue::Constant(0x08),"Timer/Counter1 Overflow"),
                ("OVF0",Rvalue::Constant(0x09),"Timer/Counter0 Overflow"),
                ("SPI",Rvalue::Constant(0x0a),"Serial Transfer Complete"),
                ("URXC",Rvalue::Constant(0x0b),"USART, Rx Complete"),
                ("UDRE",Rvalue::Constant(0x0c),"USART Data Register Empty"),
                ("UTXC",Rvalue::Constant(0x0d),"USART, Tx Complete"),
                ("ADCC",Rvalue::Constant(0x0e),"ADC Conversion Complete"),
                ("ERDY",Rvalue::Constant(0x0f),"EEPROM Ready"),
                ("ACI",Rvalue::Constant(0x10),"Analog Comparator"),
                ("TWI",Rvalue::Constant(0x11),"2-wire Serial Interface"),
                ("SPMR",Rvalue::Constant(0x12),"Store Program Memory Ready"),*/
            ],
            skip: None,
        }
    }

    pub fn atmega88() -> Mcu {
        Mcu {
            pc_bits: 13,
            int_vec: vec![
              /*  ("RESET",Rvalue::Constant(0),"MCU Reset Interrupt"),
                ("INT0",Rvalue::Constant(2),"External Interrupt Request 0"),
                ("INT1",Rvalue::Constant(4),"External Interrupt Request 1"),
                ("PCI0",Rvalue::Constant(6),"Pin Change Interrupt Request 0"),
                ("PCI1",Rvalue::Constant(8),"Pin Change Interrupt Request 1"),
                ("PCI2",Rvalue::Constant(10),"Pin Change Interrupt Request 2"),
                ("WDT",Rvalue::Constant(12),"Watchdog Time-out Interrupt"),
                ("OC2A",Rvalue::Constant(14),"Timer/Counter2 Compare Match A"),
                ("OC2B",Rvalue::Constant(16),"Timer/Counter2 Compare Match B"),
                ("OVF2",Rvalue::Constant(18),"Timer/Counter2 Overflow"),
                ("ICP1",Rvalue::Constant(20),"Timer/Counter1 Capture Event"),
                ("OC1A",Rvalue::Constant(22),"Timer/Counter1 Compare Match A"),
                ("OC1B",Rvalue::Constant(24),"Timer/Counter1 Compare Match B"),
                ("OVF1",Rvalue::Constant(26),"Timer/Counter1 Overflow"),
                ("OC0A",Rvalue::Constant(28),"TimerCounter0 Compare Match A"),
                ("OC0B",Rvalue::Constant(30),"TimerCounter0 Compare Match B"),// XXX: m88def.inc says 0x1f (words)
                ("OVF0",Rvalue::Constant(32),"Timer/Couner0 Overflow"),
                ("SPI",Rvalue::Constant(34),"SPI Serial Transfer Complete"),
                ("URXC",Rvalue::Constant(36),"USART Rx Complete"),
                ("UDRE",Rvalue::Constant(38),"USART, Data Register Empty"),
                ("UTXC",Rvalue::Constant(40),"USART Tx Complete"),
                ("ADCC",Rvalue::Constant(42),"ADC Conversion Complete"),
                ("ERDY",Rvalue::Constant(44),"EEPROM Ready"),
                ("ACI",Rvalue::Constant(46),"Analog Comparator"),
                ("TWI",Rvalue::Constant(48),"Two-wire Serial Interface"),
                ("SPMR",Rvalue::Constant(50),"Store Program Memory Read")*/
            ],
            skip: None,
        }
    }

    pub fn wrap(&self, addr: u64) -> Rvalue {
        Rvalue::Constant{ value: addr % (1u64 << self.pc_bits), size: self.pc_bits as usize }
    }

    pub fn wrap_signed(&self, addr: i64) -> Rvalue {
        let mask = 1i64 << self.pc_bits;
        Rvalue::Constant{ value: (((addr % mask) + mask) % mask) as u64, size: self.pc_bits as usize }
    }
}

#[derive(PartialEq)]
pub enum AddressRegister {
    X,
    Y,
    Z,
}

#[derive(PartialEq)]
pub enum AddressOffset {
    None,
    Predecrement,
    Postincrement,
    Displacement,
}

pub fn reg(st: &State<Avr>, cap: &str) -> Lvalue {
    resolv(st.get_group(cap))
}

pub fn resolv(r: u64) -> Lvalue {
    match r {
        0 => rreil_lvalue!{ R0:8 },
        1 => rreil_lvalue!{ R1:8 },
        2 => rreil_lvalue!{ R2:8 },
        3 => rreil_lvalue!{ R3:8 },
        4 => rreil_lvalue!{ R4:8 },
        5 => rreil_lvalue!{ R5:8 },
        6 => rreil_lvalue!{ R6:8 },
        7 => rreil_lvalue!{ R7:8 },
        8 => rreil_lvalue!{ R8:8 },
        9 => rreil_lvalue!{ R9:8 },
        10 => rreil_lvalue!{ R10:8 },
        11 => rreil_lvalue!{ R11:8 },
        12 => rreil_lvalue!{ R12:8 },
        13 => rreil_lvalue!{ R13:8 },
        14 => rreil_lvalue!{ R14:8 },
        15 => rreil_lvalue!{ R15:8 },
        16 => rreil_lvalue!{ R16:8 },
        17 => rreil_lvalue!{ R17:8 },
        18 => rreil_lvalue!{ R18:8 },
        19 => rreil_lvalue!{ R19:8 },
        20 => rreil_lvalue!{ R20:8 },
        21 => rreil_lvalue!{ R21:8 },
        22 => rreil_lvalue!{ R22:8 },
        23 => rreil_lvalue!{ R23:8 },
        24 => rreil_lvalue!{ R24:8 },
        25 => rreil_lvalue!{ R25:8 },
        26 => rreil_lvalue!{ R26:8 },
        27 => rreil_lvalue!{ R27:8 },
        28 => rreil_lvalue!{ R28:8 },
        29 => rreil_lvalue!{ R29:8 },
        30 => rreil_lvalue!{ R30:8 },
        31 => rreil_lvalue!{ R31:8 },
        _ => unreachable!("can't decode register {}",r),
    }
}

pub fn optional_skip(next: Rvalue, st: &mut State<Avr>) {
    if st.configuration.skip.is_some() {
        let (g,o) = st.configuration.skip.as_ref().unwrap().clone();
        st.jump_from(o,next,g);
    }
}

pub fn skip(n: &'static str, expect: bool) -> Box<Fn(&mut State<Avr>) -> bool> {
    Box::new(move |st: &mut State<Avr>| {
        let bit = st.get_group("sb") as u8;
        let b = Rvalue::new_u8(bit);
        let (rr,_rr) = if st.has_group("sr") {
            let reg = reg(st,"sr");
            if let Lvalue::Variable{ ref name,.. } = reg {
                (Lvalue::Variable{ name: name.clone(), size: 1, subscript: None, offset: bit as usize },reg.clone().into())
            } else {
                unreachable!()
            }
        } else {
            let A = Rvalue::Constant{ value: st.get_group("sA"), size: 6 };

            st.mnemonic(0,"__io_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
                rreil!{cg:
                    load/io ioreg:8, (A);
                }
            });

            (Lvalue::Variable{ name: Cow::Borrowed("ioreg"), size: 1, offset: bit as usize, subscript: None },A)
        };

        st.mnemonic(2,n,"{u}, {u}",vec![_rr.clone().into(),b.clone()],&|cg: &mut CodeGen<Avr>| {
            let rr = rr.clone();
            rreil!{cg:
                mov skip_flag:1, (rr);
            }
        });

        let fallthru = st.configuration.wrap(st.address + 2);
        let skip = st.configuration.wrap(st.address + 4);
        let g = {
            let tmp = Guard::from_flag(&rreil_rvalue!{ skip_flag:1 }).ok().unwrap();
            if !expect { tmp.negation() } else { tmp }
        };

        if st.tokens.len() == 1 {
            st.jump(skip,g.clone());
        } else {
            st.configuration.skip = Some((g.clone(),st.address));
        }

        st.jump(fallthru,g.negation());
        true
    })
}

pub fn binary(n: &'static str,sem: fn(Lvalue,Rvalue,&mut CodeGen<Avr>)) -> Box<Fn(&mut State<Avr>) -> bool>{
    Box::new(move |st: &mut State<Avr>| {
        let rd = if st.has_group("D") {
            reg(st,"D")
        } else {
            resolv(st.get_group("d") + 16).into()
        };
        let rr = if st.has_group("R") {
            reg(st,"R").into()
        } else if st.has_group("r") {
            resolv(st.get_group("r") + 16).into()
        } else {
            Rvalue::new_u8(st.get_group("K") as u8)
        };
        let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

        st.mnemonic(2,n,"{u}, {u}",vec!(rd.clone().into(),rr.clone()),&|cg: &mut CodeGen<Avr>| {
            sem(rd.clone(),rr.clone(),cg)
        });
        optional_skip(next.clone(),st);
        st.jump(next,Guard::always());
        true
    })
}

pub fn binary_imm(n: &'static str,sem: fn(Lvalue,u64,&mut CodeGen<Avr>)) -> Box<Fn(&mut State<Avr>) -> bool>{
    Box::new(move |st: &mut State<Avr>| {
        let (rd,rd_rv) = if st.has_group("D") {
            (reg(st,"D"),None)
        } else if st.has_group("d") {
            (resolv(st.get_group("d") + 16),None)
        } else {
            let A = Rvalue::Constant{ value: st.get_group("A"), size: 6 };

            st.mnemonic(0,"__io_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
                rreil!{cg:
                    load/io ioreg:8, (A);
                }
            });

            (Lvalue::Variable{ name: Cow::Borrowed("ioreg"), size: 8, offset: 0, subscript: None },Some(A))
        };
        let (k,kc) = if st.has_group("k") {
            (st.get_group("k"),Rvalue::new_u8(st.get_group("k") as u8))
        } else {
            (st.get_group("b"),Rvalue::Constant{ value: st.get_group("b"), size: 3 })
        };
        let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
        let len = st.tokens.len() * 2;

        st.mnemonic(len,n,"{u}, {u}",vec![rd_rv.unwrap_or(rd.clone().into()),kc.clone()],&|cg: &mut CodeGen<Avr>| {
            sem(rd.clone(),k,cg)
        });
        optional_skip(next.clone(),st);
        st.jump(next,Guard::always());
        true
    })
}

pub fn binary_ptr(n: &'static str,sem: fn(Lvalue,Lvalue,&mut CodeGen<Avr>),ar: AddressRegister,off: AddressOffset,ptr_first: bool) -> Box<Fn(&mut State<Avr>) -> bool> {
    Box::new(move |st: &mut State<Avr>| {
        let maybe_q = st.groups.iter().find(|x| x.0 == "q").cloned();
        let reg_str = match (&ar,&off) {
            (&AddressRegister::X,&AddressOffset::None) => "X".to_string(),
            (&AddressRegister::X,&AddressOffset::Predecrement) => "-X".to_string(),
            (&AddressRegister::X,&AddressOffset::Postincrement) => "X+".to_string(),
            (&AddressRegister::Y,&AddressOffset::None) => "Y".to_string(),
            (&AddressRegister::Y,&AddressOffset::Predecrement) => "-Y".to_string(),
            (&AddressRegister::Y,&AddressOffset::Postincrement) => "Y+".to_string(),
            (&AddressRegister::Y,&AddressOffset::Displacement) => format!("Y+{}",maybe_q.clone().unwrap().1),
            (&AddressRegister::Z,&AddressOffset::None) => "Z".to_string(),
            (&AddressRegister::Z,&AddressOffset::Predecrement) => "-Z".to_string(),
            (&AddressRegister::Z,&AddressOffset::Postincrement) => "Z+".to_string(),
            (&AddressRegister::Z,&AddressOffset::Displacement) => format!("Z+{}",maybe_q.clone().unwrap().1),
            _ => unreachable!(),
        };
        let addr_reg = Lvalue::Variable{
            name: Cow::Owned(reg_str),
            size: 16,
            subscript: None,
            offset: 0,
        };
        let reg = if st.has_group("D") {
            reg(st,"D")
        } else if st.has_group("d") {
            resolv(st.get_group("d") + 16)
        } else if st.has_group("R") {
            reg(st,"R")
        } else {
            resolv(st.get_group("r") + 16)
        };
        let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

        st.mnemonic(0,"__addr_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
            let (r1,r2) = match ar {
                AddressRegister::X => (rreil_lvalue!{ R26:8 },rreil_lvalue!{ R27:8 }),
                AddressRegister::Y => (rreil_lvalue!{ R28:8 },rreil_lvalue!{ R29:8 }),
                AddressRegister::Z => (rreil_lvalue!{ R30:8 },rreil_lvalue!{ R31:8 }),
            };

            rreil!{cg:
                zext/16 (addr_reg), (r1);
                mov (addr_reg.extract(8,8).ok().unwrap()), (r2);
            }
        });

        let (fmt,rd,rr) = if ptr_first {
            ("{p}, {u}",addr_reg.clone().into(),reg.clone().into())
        } else {
            ("{u}, {p}",reg.clone().into(),addr_reg.clone().into())
        };

        st.mnemonic(2,n,fmt,vec!(rd,rr),&|cg: &mut CodeGen<Avr>| {
            if off == AddressOffset::Predecrement {
                rreil!{cg:
                    sub (addr_reg), (addr_reg), [1]:16;
                }
            }

            sem(addr_reg.clone(),reg.clone(),cg);

            if off == AddressOffset::Postincrement {
                rreil!{cg:
                    add (addr_reg), (addr_reg), [1]:16;
                }
            } else if off == AddressOffset::Displacement {
                rreil!{cg:
                    add (addr_reg), (addr_reg), (Rvalue::new_u16(maybe_q.clone().unwrap().1 as u16));
                }
            }
        });

        st.mnemonic(0,"__addr_reg","",vec![],&|cg: &mut CodeGen<Avr>| {
            let (r1,r2) = match ar {
                AddressRegister::X => (rreil_lvalue!{ R26:8 },rreil_lvalue!{ R27:8 }),
                AddressRegister::Y => (rreil_lvalue!{ R28:8 },rreil_lvalue!{ R29:8 }),
                AddressRegister::Z => (rreil_lvalue!{ R30:8 },rreil_lvalue!{ R31:8 }),
            };

            rreil!{cg:
                mov (r1), (addr_reg.extract(8,0).ok().unwrap());
                mov (r2), (addr_reg.extract(8,8).ok().unwrap());
            }
        });

        optional_skip(next.clone(),st);
        st.jump(next,Guard::always());
        true
    })
}

pub fn nonary(n: &'static str,sem: fn(&mut CodeGen<Avr>)) -> Box<Fn(&mut State<Avr>) -> bool>{
    Box::new(move |st: &mut State<Avr>| {
        let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

        st.mnemonic(2,n,"",vec![],&sem);
        optional_skip(next.clone(),st);
        st.jump(next,Guard::always());
        true
    })
}

pub fn unary(n: &'static str,sem: fn(Lvalue,&mut CodeGen<Avr>)) -> Box<Fn(&mut State<Avr>) -> bool>{
    Box::new(move |st: &mut State<Avr>| {
        let rd = if st.has_group("D") { reg(st,"D") } else { resolv(st.get_group("d") + 16) };
        let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

        st.mnemonic(2,n,"{u}",vec!(rd.clone().into()),&|cg: &mut CodeGen<Avr>| {
            sem(rd.clone(),cg)
        });
        optional_skip(next.clone(),st);
        st.jump(next,Guard::always());
        true
    })
}

pub fn flag(n: &'static str,_f: &Lvalue,val: bool) -> Box<Fn(&mut State<Avr>) -> bool>{
    let f = _f.clone();
    Box::new(move |st: &mut State<Avr>| -> bool {
        let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

        st.mnemonic(2,n,"",vec!(),&|cg: &mut CodeGen<Avr>| {
            let bit = if val { 1 } else { 0 };

            rreil!{cg:
                mov (f.clone()), [bit]:1;
            }
        });
        optional_skip(next.clone(),st);
        st.jump(next,Guard::always());
        true
    })
}

pub fn branch(n: &'static str,_f: &Lvalue,val: bool) -> Box<Fn(&mut State<Avr>) -> bool>{
    let f = _f.clone();
    Box::new(move |st: &mut State<Avr>| -> bool {
        let _k = st.get_group("k") as u8; // 6 bits, signed
        let k = (if _k >= 0x20 { (0xE0 | _k) as i8 } else { _k as i8 } * 2) as i64;
        let jump = st.configuration.wrap_signed(st.address as i64 + st.tokens.len() as i64 + k);
        let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

        st.mnemonic(2,n,"{c:flash}",vec!(jump.clone().into()),&|cg: &mut CodeGen<Avr>| {
            let bit = if val { 1 } else { 0 };

            rreil!{cg:
                mov (f), [bit]:1;
            }
        });

        optional_skip(fallthru.clone(),st);
        let g = Guard::from_flag(&f.clone().into()).ok().unwrap();

        if val {
            st.jump(fallthru,g.negation());
            st.jump(jump,g);
        } else {
            st.jump(jump,g.negation());
            st.jump(fallthru,g);
        }
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use region::Region;
    use super::syntax::disassembler;
    use function::{ControlFlowTarget,Function};
    use Rvalue;
    use std::borrow::Cow;

    use std::hash::{Hash,Hasher,SipHasher};

    use graph_algos::{
        VertexListGraphTrait,
        GraphTrait,
        EdgeListGraphTrait
    };

    #[test]
    fn avr_single_skip() {
        let reg = Region::wrap("flash".to_string(),
        vec!(
            0x12,0x2c, // 0:2 mov
            0x12,0x10, // 2:4 cpse
            0x23,0x0c, // 4:6 add
            0x21,0x2c, // 6:8 mov
        ));
        let main = disassembler();
        let fun = Function::disassemble::<Avr>(None,main,Mcu::new(),reg.iter(),0,reg.name().to_string());
        let cg = &fun.cflow_graph;

        for x in cg.vertices() {
            match cg.vertex_label(x) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    for mne in bb.mnemonics.iter() {
                        println!("{:?}: {}",mne.area,mne.opcode);
                    }
                },
                Some(&ControlFlowTarget::Unresolved(ref v)) => {
                    println!("{:?}",v);
                },
                None => {}
            }
        }

        for x in cg.edges() {
            let from = cg.source(x);
            let to = cg.target(x);
            let from_ident = to_ident(cg.vertex_label(from));
            let to_ident = to_ident(cg.vertex_label(to));

            if from_ident.is_some() && to_ident.is_some() {
                println!("{} -> {}",from_ident.unwrap(),to_ident.unwrap());
            }
        }

        assert_eq!(cg.num_edges(),4);
        assert_eq!(cg.num_vertices(),4);
    }

    #[test]
    fn avr_double_skip() {
        let reg = Region::wrap("flash".to_string(),
        vec!(
            0x12,0x2c, // 0:2 mov
            0x12,0x10, // 2:4 cpse
            0x12,0x10, // 2:4 cpse
            0x23,0x0c, // 4:6 add
            0x21,0x2c, // 6:8 mov
        ));
        let main = disassembler();
        let fun = Function::disassemble::<Avr>(None,main,Mcu::new(),reg.iter(),0,reg.name().to_string());
        let cg = &fun.cflow_graph;

        for x in cg.vertices() {
            match cg.vertex_label(x) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    for mne in bb.mnemonics.iter() {
                        println!("{:?}: {}",mne.area,mne.opcode);
                    }
                },
                Some(&ControlFlowTarget::Unresolved(ref v)) => {
                    println!("{:?}",v);
                },
                None => {}
            }
        }


        for x in cg.edges() {
            let from = cg.source(x);
            let to = cg.target(x);
            let from_ident = to_ident(cg.vertex_label(from));
            let to_ident = to_ident(cg.vertex_label(to));

            if from_ident.is_some() && to_ident.is_some() {
                println!("{} -> {}",from_ident.unwrap(),to_ident.unwrap());
            }
        }

        assert_eq!(cg.num_edges(),6);
        assert_eq!(cg.num_vertices(),5);
    }

    #[test]
    fn avr_triple_skip() {
        let reg = Region::wrap("flash".to_string(),
        vec!(
            0x12,0x2c, // 0:2 mov
            0x12,0x10, // 2:4 cpse
            0x12,0x10, // 2:4 cpse
            0x12,0x10, // 2:4 cpse
            0x23,0x0c, // 4:6 add
            0x21,0x2c, // 6:8 mov
        ));
        let main = disassembler();
        let fun = Function::disassemble::<Avr>(None,main,Mcu::new(),reg.iter(),0,reg.name().to_string());
        let cg = &fun.cflow_graph;

        for x in cg.vertices() {
            match cg.vertex_label(x) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    for mne in bb.mnemonics.iter() {
                        println!("{:?}: {}",mne.area,mne.opcode);
                    }
                },
                Some(&ControlFlowTarget::Unresolved(ref v)) => {
                    println!("{:?}",v);
                },
                None => {}
            }
        }


        for x in cg.edges() {
            let from = cg.source(x);
            let to = cg.target(x);
            let from_ident = to_ident(cg.vertex_label(from));
            let to_ident = to_ident(cg.vertex_label(to));

            if from_ident.is_some() && to_ident.is_some() {
                println!("{} -> {}",from_ident.unwrap(),to_ident.unwrap());
            }
        }

        assert_eq!(cg.num_edges(),8);
        assert_eq!(cg.num_vertices(),6);
    }

    #[test]
    fn avr_brne() {
        let reg = Region::wrap("flash".to_string(),
            vec!(
                0xde,0x01, //  0: movw
                0x11,0x96, //  2: adiw
                0x88,0xe0, //  4: ldi
                0x0d,0x90, //  6: ld
                0x01,0x92, //  8: st
                0x81,0x50, // 10: subi
                0xe1,0xf7, // 12: brne
                0x81,0xe0, // 14: ldi
                0x01,0xc0, // 16: rjmp
                0x80,0xe0, // 18: ldi
                0x68,0x96, // 20: adiw
                0xe4,0xe0  // 22: ldi
            ));
        let main = disassembler();
        let fun = Function::disassemble::<Avr>(None,main,Mcu::new(),reg.iter(),0,reg.name().to_string());

        for x in fun.cflow_graph.edges() {
            let cg = &fun.cflow_graph;
            let from = cg.source(x);
            let to = cg.target(x);
            let from_ident = to_ident(cg.vertex_label(from));
            let to_ident = to_ident(cg.vertex_label(to));

            if from_ident.is_some() && to_ident.is_some() {
                println!("{} -> {}",from_ident.unwrap(),to_ident.unwrap());
            }
        }
    }

    fn to_ident(t: Option<&ControlFlowTarget>) -> Option<String> {
        match t {
            Some(&ControlFlowTarget::Resolved(ref bb)) => Some(format!("\"bb{}\"",bb.area.start)),
            Some(&ControlFlowTarget::Unresolved(Rvalue::Constant{ ref value,.. })) => Some(format!("\"v{}\"",*value)),
            Some(&ControlFlowTarget::Unresolved(ref c)) => {
                let ref mut h = SipHasher::new();
                c.hash::<SipHasher>(h);
                Some(format!("\"c{}\"",h.finish()))
            },
            _ => None,
        }
    }

    #[test]
    fn all() {
        let test_vectors = vec![
            (vec![0xec,0x0e],"add",vec![rreil_rvalue!{ R14:8 },rreil_rvalue!{ R28:8 }]),
            (vec![0xe1,0x1c],"adc",vec![rreil_rvalue!{ R14:8 }, rreil_rvalue!{ R1:8 }]),
            (vec![0x49,0x5f],"subi",vec![rreil_rvalue!{ R20:8 },Rvalue::new_u8(0xF9)]),
            (vec![0x48,0x1b],"sub",vec![rreil_rvalue!{ R20:8 }, rreil_rvalue!{ R24:8 }]),
            (vec![0x59,0x0b],"sbc",vec![rreil_rvalue!{ R21:8 }, rreil_rvalue!{ R25:8 }]),
            (vec![0x50,0x40],"sbci",vec![rreil_rvalue!{ R21:8 }, Rvalue::new_u8(0x00)]),
            (vec![0x88,0x23],"and",vec![rreil_rvalue!{ R24:8 }, rreil_rvalue!{ R24:8 }]),
            (vec![0x8c,0x7f],"andi",vec![rreil_rvalue!{ R24:8 }, Rvalue::new_u8(0xFC)]),
            (vec![0x35,0x2b],"or",vec![rreil_rvalue!{ R19:8 }, rreil_rvalue!{ R21:8 }]),
            (vec![0x9c,0x66],"ori",vec![rreil_rvalue!{ R25:8 }, Rvalue::new_u8(0x6c)]),
            (vec![0xff,0x24],"eor",vec![rreil_rvalue!{ R15:8 }, rreil_rvalue!{ R15:8 }]),
            (vec![0x80,0x95],"com",vec![rreil_rvalue!{ R24:8 }]),
            (vec![0x41,0x94],"neg",vec![rreil_rvalue!{ R4:8 }]),
            (vec![0xf3,0x94],"inc",vec![rreil_rvalue!{ R15:8 }]),
            (vec![0xfa,0x94],"dec",vec![rreil_rvalue!{ R15:8 }]),
            (vec![0x29,0xc0],"rjmp",vec![rreil_rvalue!{ [82]:24 }]),
            (vec![0x00,0xd0],"rcall",vec![rreil_rvalue!{ [0]:24 }]),
            (vec![0x08,0x95],"ret",vec![]),
            (vec![0x18,0x95],"reti",vec![]),
            (vec![0xff,0x13],"cpse",vec![rreil_rvalue!{ R31:8 },rreil_rvalue!{ R31:8 }]),
            (vec![0x28,0x17],"cp",vec![rreil_rvalue!{ R18:8 }, rreil_rvalue!{ R24:8 }]),
            (vec![0x39,0x07],"cpc",vec![rreil_rvalue!{ R19:8 }, rreil_rvalue!{ R25:8 }]),
            (vec![0x61,0x31],"cpi",vec![rreil_rvalue!{ R22:8 }, Rvalue::new_u8(0x11)]),
            (vec![0x80,0xfd],"sbrc",vec![rreil_rvalue!{ R24:8 },rreil_rvalue!{ [0]:8 }]),
            (vec![0x65,0xfe],"sbrs",vec![rreil_rvalue!{ R6:8 },Rvalue::new_u8(5)]),
            (vec![0xb0,0x99],"sbic",vec![rreil_rvalue!{ [0x16]:6 },rreil_rvalue!{ [0]:8 }]),
            (vec![0xce,0x9b],"sbis",vec![rreil_rvalue!{ [0x19]:6 },rreil_rvalue!{ [6]:8 }]),
            (vec![0xf1,0xf3],"breq",vec![Rvalue::Constant{ value: (0b1111111111111111111111-2), size: 22 }]),
            (vec![0xb1,0xf7],"brne",vec![Rvalue::Constant{ value: (0b1111111111111111111111-18), size: 22 }]),
            (vec![0xf8,0xf3],"brlo",vec![Rvalue::Constant{ value: (0b1111111111111111111111-0), size: 22 }]),
            (vec![0xc8,0xf7],"brsh",vec![Rvalue::Constant{ value: (0b1111111111111111111111-12), size: 22 }]),
            (vec![0xea,0xf3],"brmi",vec![Rvalue::Constant{ value: (0b1111111111111111111111-4), size: 22 }]),
            (vec![0xaa,0xf7],"brpl",vec![Rvalue::Constant{ value: (0b1111111111111111111111-20), size: 22 }]),
            (vec![0x54,0xf4],"brge",vec![Rvalue::Constant{ value: 21, size: 22 }]),
            (vec![0xdc,0xf3],"brlt",vec![Rvalue::Constant{ value: (0b1111111111111111111111-8), size: 22 }]),
            (vec![0xd5,0xf3],"brhs",vec![Rvalue::Constant{ value: (0b1111111111111111111111-10), size: 22 }]),
            (vec![0x95,0xf7],"brhc",vec![Rvalue::Constant{ value: (0b1111111111111111111111-26), size: 22 }]),
            (vec![0xce,0xf3],"brts",vec![Rvalue::Constant{ value: (0b1111111111111111111111-12), size: 22 }]),
            (vec![0x8e,0xf7],"brtc",vec![Rvalue::Constant{ value: (0b1111111111111111111111-28), size: 22 }]),
            (vec![0xe3,0xf3],"brvs",vec![Rvalue::Constant{ value: (0b1111111111111111111111-6), size: 22 }]),
            (vec![0xa3,0xf7],"brvc",vec![Rvalue::Constant{ value: (0b1111111111111111111111-22), size: 22 }]),
            (vec![0xc7,0xf3],"brie",vec![Rvalue::Constant{ value: (0b1111111111111111111111-14), size: 22 }]),
            (vec![0x87,0xf7],"brid",vec![Rvalue::Constant{ value: (0b1111111111111111111111-30), size: 22 }]),
            (vec![0x9c,0x91],"ld",vec![rreil_rvalue!{ R25:8 }, Rvalue::Variable{ name: Cow::Borrowed("X"), size: 16, offset: 0, subscript: None }]),
            (vec![0x8d,0x91],"ld",vec![rreil_rvalue!{ R24:8 }, Rvalue::Variable{ name: Cow::Borrowed("X+"), size: 16, offset: 0, subscript: None }]),
            (vec![0x88,0x81],"ld",vec![rreil_rvalue!{ R24:8 }, Rvalue::Variable{ name: Cow::Borrowed("Y"), size: 16, offset: 0, subscript: None }]),
            (vec![0xb0,0x81],"ld",vec![rreil_rvalue!{ R27:8 }, Rvalue::Variable{ name: Cow::Borrowed("Z"), size: 16, offset: 0, subscript: None }]),
            (vec![0x01,0x90],"ld",vec![rreil_rvalue!{ R0:8 }, Rvalue::Variable{ name: Cow::Borrowed("Z+"), size: 16, offset: 0, subscript: None }]),
            (vec![0x0d,0x92],"st",vec![Rvalue::Variable{ name: Cow::Borrowed("X+"), size: 16, offset: 0, subscript: None }, rreil_rvalue!{ R0:8 }]),
            (vec![0x88,0x83],"st",vec![Rvalue::Variable{ name: Cow::Borrowed("Y"), size: 16, offset: 0, subscript: None }, rreil_rvalue!{ R24:8 }]),
            (vec![0x80,0x83],"st",vec![Rvalue::Variable{ name: Cow::Borrowed("Z"), size: 16, offset: 0, subscript: None }, rreil_rvalue!{ R24:8 }]),
            (vec![0x81,0x93],"st",vec![Rvalue::Variable{ name: Cow::Borrowed("Z+"), size: 16, offset: 0, subscript: None }, rreil_rvalue!{ R24:8 }]),
            (vec![0x03,0x2e],"mov",vec![rreil_rvalue!{ R0:8 },rreil_rvalue!{ R19:8 }]),
            (vec![0x10,0xe0],"ldi",vec![rreil_rvalue!{ R17:8 }, Rvalue::new_u8(0x00)]),
            (vec![0xcd,0xb7],"in",vec![rreil_rvalue!{ R28:8 }, rreil_rvalue!{ [0x3d]:6 }]),
            (vec![0xde,0xbf],"out",vec![rreil_rvalue!{ [0x3e]:6 }, rreil_rvalue!{ R29:8 }]),
            (vec![0xc8,0x95],"lpm",vec![]),
            (vec![0xc0,0x9a],"sbi",vec![rreil_rvalue!{ [0x18]:6 }, rreil_rvalue!{ [0]:3}]),
            (vec![0xc0,0x98],"cbi",vec![rreil_rvalue!{ [0x18]:6 }, rreil_rvalue!{ [0]:3}]),
            (vec![0x76,0x95],"lsr",vec![rreil_rvalue!{ R23:8 }]),
            (vec![0x87,0x95],"ror",vec![rreil_rvalue!{ R24:8 }]),
            (vec![0x82,0x95],"swap",vec![rreil_rvalue!{ R24:8 }]),
            (vec![0x70,0xfa],"bst",vec![rreil_rvalue!{ R7:8 },rreil_rvalue!{ [0]:3}]),
            (vec![0xf7,0xf9],"bld",vec![rreil_rvalue!{ R31:8 }, rreil_rvalue!{ [7]:3}]),
            (vec![0x08,0x94],"sec",vec![]),
            (vec![0x88,0x94],"clc",vec![]),
            (vec![0x28,0x94],"sen",vec![]),
            (vec![0xa8,0x94],"cln",vec![]),
            (vec![0x18,0x94],"sez",vec![]),
            (vec![0x98,0x94],"clz",vec![]),
            (vec![0x78,0x94],"sei",vec![]),
            (vec![0xf8,0x94],"cli",vec![]),
            (vec![0x48,0x94],"ses",vec![]),
            (vec![0xc8,0x94],"cls",vec![]),
            (vec![0x38,0x94],"sev",vec![]),
            (vec![0xb8,0x94],"clv",vec![]),
            (vec![0x68,0x94],"set",vec![]),
            (vec![0xe8,0x94],"clt",vec![]),
            (vec![0x58,0x94],"seh",vec![]),
            (vec![0xd8,0x94],"clh",vec![]),
            (vec![0x00,0x00],"nop",vec![]),
            (vec![0x88,0x95],"sleep",vec![]),
            (vec![0xa8,0x95],"wdr",vec![]),
            (vec![0x01,0x96],"adiw",vec![rreil_rvalue!{ R24:8 }, Rvalue::new_u8(0x01)]),
            (vec![0x13,0x97],"sbiw",vec![rreil_rvalue!{ R26:8 }, Rvalue::new_u8(0x03)]),
            (vec![0x09,0x94],"ijmp",vec![]),
            (vec![0x09,0x95],"icall",vec![]),
            (vec![0x2a,0x88],"ldd",vec![rreil_rvalue!{ R2:8 }, Rvalue::Variable{ name: Cow::Borrowed("Y+18"), size: 16, offset: 0, subscript: None }]),
            (vec![0x87,0x81],"ldd",vec![rreil_rvalue!{ R24:8 }, Rvalue::Variable{ name: Cow::Borrowed("Z+7"), size: 16, offset: 0, subscript: None }]),
            (vec![0x80,0x91,0x62,0x00],"lds",vec![rreil_rvalue!{ R24:8 }, Rvalue::new_u16(0x0062)]),
            (vec![0x99,0x83],"std",vec![Rvalue::Variable{ name: Cow::Borrowed("Y+1"), size: 16, offset: 0, subscript: None }, rreil_rvalue!{ R25:8 }]),
            (vec![0x84,0x83],"std",vec![Rvalue::Variable{ name: Cow::Borrowed("Z+4"), size: 16, offset: 0, subscript: None }, rreil_rvalue!{ R24:8 }]),
            (vec![0x90,0x93,0x7c,0x00],"sts",vec![Rvalue::new_u16(0x007C),rreil_rvalue!{ R25:8 }]),
            (vec![0xcf,0x93],"push",vec![rreil_rvalue!{ R28:8 }]),
            (vec![0xcf,0x91],"pop",vec![rreil_rvalue!{ R28:8 }]),
            (vec![0x0c,0x94,0xe9,0x0e],"jmp", vec![rreil_rvalue!{ [0x1dd2]:22 }]),
            (vec![0x0e,0x94,0xa4,0x0a],"call", vec![rreil_rvalue!{ [0x1548]:22 }]),
            (vec![0xd8,0x95],"elpm",vec![]),
            (vec![0x31,0x9c],"mul",vec![rreil_rvalue!{ R3:8 },rreil_rvalue!{ R1:8 }]),
            (vec![0xd5,0x02],"muls",vec![rreil_rvalue!{ R29:8 },rreil_rvalue!{ R21:8 }]),
            (vec![0x55,0x03],"mulsu",vec![rreil_rvalue!{ R21:8 },rreil_rvalue!{ R21:8 }]),
            (vec![0x5c,0x03],"fmul",vec![rreil_rvalue!{ R21:8 },rreil_rvalue!{ R20:8 }]),
            (vec![0x90,0x03],"fmuls",vec![rreil_rvalue!{ R17:8 },rreil_rvalue!{ R16:8 }]),
            (vec![0xff,0x03],"fmulsu",vec![rreil_rvalue!{ R23:8 },rreil_rvalue!{ R23:8 }]),
            (vec![0x4a,0x01],"movw",vec![rreil_rvalue!{ R8:8 }, rreil_rvalue!{ R20:8 }]),
            (vec![0xc8,0x95],"lpm",vec![]),
            (vec![0xe8,0x95],"spm",vec![]),
            (vec![0x98,0x95],"break",vec![]),
            // EIJMP
            // EICALL
            // XCH
            // LAS
            // LAC
            // LAT
            // DES
        ];
        let main = disassembler();

        for (bytes,opname,args) in test_vectors {
            println!("check '{}'",opname);

            let l = bytes.len();
            let reg = Region::wrap("base".to_string(),bytes);
            let mut i = reg.iter().seek(0);
            let maybe_match = main.next_match(&mut i,0,Mcu::new());

            if let Some(match_st) = maybe_match {
                assert!(match_st.mnemonics.len() >= 1);

                for m in match_st.mnemonics.iter() {
                    if m.area.start != m.area.end {
                        assert_eq!(opname,m.opcode);
                        assert_eq!(m.area.start,0);
                        assert_eq!(m.area.end,l as u64);
                        assert_eq!(m.operands,args);
                    }
                }
            } else {
                unreachable!()
            }
        }
    }
}
