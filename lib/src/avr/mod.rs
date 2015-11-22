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

use disassembler::*;
use program::{Program,DisassembleEvent};
use layer::LayerIter;
use value::{Lvalue,Rvalue,Endianess,ToRvalue};
use codegen::CodeGen;
use guard::Guard;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub mod syntax;

#[derive(Clone)]
pub enum Avr {}

impl Architecture for Avr {
    type Token = u16;
    type Configuration = Mcu;
}

#[derive(Clone)]
pub struct Mcu {
    pub pc_bits: u16,                                   ///< width of the program counter in bits (FLASHEND)
    pub int_vec: Vec<(&'static str,u64,&'static str)>,  ///< interrupt vector: (name, offset, comment)
    pub skip: Option<(Guard,u64)>,
}

impl Mcu {
    pub fn new() -> Mcu {
        Mcu {
            pc_bits: 13,
            int_vec: vec![("RESET",0,"MCU Reset Interrupt")],
            skip: None,
        }
    }

    pub fn atmega88() -> Mcu {
        Mcu {
            pc_bits: 13,
            int_vec: vec![
                ("RESET",0,"MCU Reset Interrupt"),
                ("INT0",2,"External Interrupt Request 0"),
                ("INT1",4,"External Interrupt Request 1"),
                ("PCI0",6,"Pin Change Interrupt Request 0"),
                ("PCI1",8,"Pin Change Interrupt Request 1"),
                ("PCI2",10,"Pin Change Interrupt Request 2"),
                ("WDT",12,"Watchdog Time-out Interrupt"),
                ("OC2A",14,"Timer/Counter2 Compare Match A"),
                ("OC2B",16,"Timer/Counter2 Compare Match B"),
                ("OVF2",18,"Timer/Counter2 Overflow"),
                ("ICP1",20,"Timer/Counter1 Capture Event"),
                ("OC1A",22,"Timer/Counter1 Compare Match A"),
                ("OC1B",24,"Timer/Counter1 Compare Match B"),
                ("OVF1",26,"Timer/Counter1 Overflow"),
                ("OC0A",28,"TimerCounter0 Compare Match A"),
                ("OC0B",30,"TimerCounter0 Compare Match B"),// XXX: m88def.inc says 0x1f (words)
                ("OVF0",32,"Timer/Couner0 Overflow"),
                ("SPI",34,"SPI Serial Transfer Complete"),
                ("URXC",36,"USART Rx Complete"),
                ("UDRE",38,"USART, Data Register Empty"),
                ("UTXC",40,"USART Tx Complete"),
                ("ADCC",42,"ADC Conversion Complete"),
                ("ERDY",44,"EEPROM Ready"),
                ("ACI",46,"Analog Comparator"),
                ("TWI",48,"Two-wire Serial Interface"),
                ("SPMR",50,"Store Program Memory Read")
            ],
            skip: None,
        }
    }

    pub fn wrap(&self, addr: u64) -> Rvalue {
        Rvalue::Constant(addr % (1u64 << self.pc_bits))
    }

    pub fn wrap_signed(&self, addr: i64) -> Rvalue {
        let mask = 1i64 << self.pc_bits;
        Rvalue::Constant((((addr % mask) + mask) % mask) as u64)
    }
}

pub fn reg(st: &State<Avr>, cap: &str) -> Lvalue {
    resolv(st.get_group(cap))
}

pub fn ioreg(st: &State<Avr>, cap: &str) -> Lvalue {
    let r = st.get_group(cap);
    let name = match r {
	    0x00 => "ubrr1",
		0x01 => "ucsr1b",
		0x02 => "ucsr1a",
		0x03 => "udr1",
		0x05 => "pine",
		0x06 => "ddre",
		0x07 => "porte",
		0x08 => "acsr",
		0x09 => "ubrr0",
		0x0A => "ucsr0b",
		0x0B => "ucsr0a",
		0x0C => "udr0",
		0x0D => "spcr",
		0x0E => "spsr",
		0x0F => "spdr",
		0x10 => "pind",
		0x11 => "ddrd",
		0x12 => "portd",
		0x13 => "pinc",
		0x14 => "ddrc",
		0x15 => "portc",
		0x16 => "pinb",
		0x17 => "ddrb",
		0x18 => "portb",
		0x19 => "pina",
		0x1A => "ddra",
		0x1B => "porta",
		0x1C => "eecr",
		0x1D => "eedr",
		0x1E => "eearl",
		0x1F => "eearh",
		0x20 => "ubrrh",
		0x21 => "wdtcr",
		0x22 => "ocr2",
		0x23 => "tcnt2",
		0x24 => "icr1l",
		0x25 => "icr1h",
		0x26 => "assr",
		0x27 => "tccr2",
		0x28 => "ocr1bl",
		0x29 => "ocr1bh",
		0x2A => "ocr1al",
		0x2B => "ocr1ah",
		0x2C => "tcnt1l",
		0x2D => "tcnt1h",
		0x2E => "tccr1b",
		0x2F => "tccr1a",
		0x30 => "sfior",
		0x31 => "ocr0",
		0x32 => "tcnt0",
		0x33 => "tccr0",
		0x34 => "mcusr",
		0x35 => "mcucr",
		0x36 => "emcucr",
		0x37 => "spmcr",
		0x38 => "tifr",
		0x39 => "timsk",
		0x3A => "gifr",
		0x3B => "gimsk",
		0x3D => "spl",
		0x3E => "sph",
		0x3F => "sreg",
        _ => "unknown_ioreg",
    };

    Lvalue::Variable{
        name: name.to_string(),
        width: 8,
        subscript: None
    }
}

pub fn sram<A: ToRvalue>(off: &A) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.to_rv()),
        name: "sram".to_string(),
        endianess: Endianess::Big,
        bytes: 1
    }
}

pub fn flash<A: ToRvalue>(off: &A) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.to_rv()),
        name: "flash".to_string(),
        endianess: Endianess::Big,
        bytes: 2
    }
}

pub fn get_sp(cg: &mut CodeGen<Avr>) -> Lvalue {
    let sp = new_temp(16);
    let spl = Lvalue::Variable{
        name: "spl".to_string(),
        width: 8,
        subscript: None
    };
    let sph = Lvalue::Variable{
        name: "sph".to_string(),
        width: 8,
        subscript: None
    };

    cg.mul_i(&sp,&sph,&0x100);
    cg.add_i(&sp,&spl,&sp);
    sram(&sp)
}

pub fn set_sp<A: ToRvalue>(v: &A, cg: &mut CodeGen<Avr>) {
    let sp = new_temp(16);
    let spl = Lvalue::Variable{
        name: "spl".to_string(),
        width: 8,
        subscript: None
    };
    let sph = Lvalue::Variable{
        name: "sph".to_string(),
        width: 8,
        subscript: None
    };

    cg.mul_i(&sp,&sph,&0x100);
    cg.add_i(&sp,&spl,&sp);
    cg.assign(&sram(&sp),v);
}

pub fn resolv(r: u64) -> Lvalue {
    if r > 31 {
        panic!("can't decode register {}",r);
    } else {
        Lvalue::Variable{
            name: format!("r{}",r),
            width: 8,
            subscript: None
        }
    }
}

pub fn optional_skip(next: Rvalue, st: &mut State<Avr>) {
    if st.configuration.skip.is_some() {
        let (g,o) = st.configuration.skip.as_ref().unwrap().clone();
        st.jump_from(o,next,g);
    }
}

static GLOBAL_AVR_TEMPVAR_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn new_temp(bits: usize) -> Lvalue {
    Lvalue::Variable{
        name: format!("__temp{}",GLOBAL_AVR_TEMPVAR_COUNT.fetch_add(1, Ordering::SeqCst)),
        width: bits as u16,
        subscript: None
    }
}

lazy_static! {
    pub static ref R0: Lvalue = Lvalue::Variable{ name: "r0".to_string(), width: 8, subscript: None };
    pub static ref R1: Lvalue = Lvalue::Variable{ name: "r1".to_string(), width: 8, subscript: None };
    pub static ref R2: Lvalue = Lvalue::Variable{ name: "r2".to_string(), width: 8, subscript: None };
    pub static ref R3: Lvalue = Lvalue::Variable{ name: "r3".to_string(), width: 8, subscript: None };
    pub static ref R4: Lvalue = Lvalue::Variable{ name: "r4".to_string(), width: 8, subscript: None };
    pub static ref R5: Lvalue = Lvalue::Variable{ name: "r5".to_string(), width: 8, subscript: None };
    pub static ref R6: Lvalue = Lvalue::Variable{ name: "r6".to_string(), width: 8, subscript: None };
    pub static ref R7: Lvalue = Lvalue::Variable{ name: "r7".to_string(), width: 8, subscript: None };
    pub static ref R8: Lvalue = Lvalue::Variable{ name: "r8".to_string(), width: 8, subscript: None };
    pub static ref R9: Lvalue = Lvalue::Variable{ name: "r9".to_string(), width: 8, subscript: None };
}

lazy_static! {
    pub static ref R10: Lvalue = Lvalue::Variable{ name: "r10".to_string(), width: 8, subscript: None };
    pub static ref R11: Lvalue = Lvalue::Variable{ name: "r11".to_string(), width: 8, subscript: None };
    pub static ref R12: Lvalue = Lvalue::Variable{ name: "r12".to_string(), width: 8, subscript: None };
    pub static ref R13: Lvalue = Lvalue::Variable{ name: "r13".to_string(), width: 8, subscript: None };
    pub static ref R14: Lvalue = Lvalue::Variable{ name: "r14".to_string(), width: 8, subscript: None };
    pub static ref R15: Lvalue = Lvalue::Variable{ name: "r15".to_string(), width: 8, subscript: None };
    pub static ref R26: Lvalue = Lvalue::Variable{ name: "r26".to_string(), width: 8, subscript: None };
    pub static ref R27: Lvalue = Lvalue::Variable{ name: "r27".to_string(), width: 8, subscript: None };
    pub static ref R28: Lvalue = Lvalue::Variable{ name: "r28".to_string(), width: 8, subscript: None };
    pub static ref R29: Lvalue = Lvalue::Variable{ name: "r29".to_string(), width: 8, subscript: None };
}

lazy_static! {
    pub static ref R30: Lvalue = Lvalue::Variable{ name: "r30".to_string(), width: 8, subscript: None };
    pub static ref R31: Lvalue = Lvalue::Variable{ name: "r31".to_string(), width: 8, subscript: None };

    pub static ref C: Lvalue = Lvalue::Variable{ name: "C".to_string(), width: 1, subscript: None };
    pub static ref V: Lvalue = Lvalue::Variable{ name: "V".to_string(), width: 1, subscript: None };
    pub static ref I: Lvalue = Lvalue::Variable{ name: "I".to_string(), width: 1, subscript: None };
    pub static ref H: Lvalue = Lvalue::Variable{ name: "I".to_string(), width: 1, subscript: None };
    pub static ref T: Lvalue = Lvalue::Variable{ name: "T".to_string(), width: 1, subscript: None };
    pub static ref N: Lvalue = Lvalue::Variable{ name: "N".to_string(), width: 1, subscript: None };
    pub static ref S: Lvalue = Lvalue::Variable{ name: "S".to_string(), width: 1, subscript: None };
    pub static ref Z: Lvalue = Lvalue::Variable{ name: "Z".to_string(), width: 1, subscript: None };

    pub static ref EIND: Lvalue = Lvalue::Variable{ name: "EIND".to_string(), width: 8, subscript: None };
    pub static ref RAMPZ: Lvalue = Lvalue::Variable{ name: "RAMPZ".to_string(), width: 8, subscript: None };
}

pub fn disassemble<F: Fn(DisassembleEvent)>(_: Mcu, data: LayerIter, progress: Option<F>) -> Program {
    Program::disassemble(None,syntax::disassembler(),Mcu::new(),data,0,"flash".to_string(),progress)
}

#[cfg(test)]
mod tests {
    use super::*;
    use region::Region;
    use super::syntax::disassembler;
    use function::{ControlFlowTarget,Function};
    use value::Rvalue;

    use std::hash::{Hash,Hasher,SipHasher};

    use graph_algos::traits::{VertexListGraph,Graph,EdgeListGraph};

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
            Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))) => Some(format!("\"v{}\"",c)),
            Some(&ControlFlowTarget::Unresolved(ref c)) => {
                let ref mut h = SipHasher::new();
                c.hash::<SipHasher>(h);
                Some(format!("\"c{}\"",h.finish()))
            },
            _ => None,
        }
    }
}
