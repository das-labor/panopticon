/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
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

use disassembler::*;
use {Lvalue,Rvalue};
use std::borrow::Cow;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
/*
pub mod load;
pub mod decode;
pub mod generic;
pub mod semantic;
*/
#[derive(Clone)]
pub enum Mos {}

impl Architecture for Mos {
    // FIXME: There should be a more useful error than crashing thread with shift overflow in libcore
    // when a bit pattern is larger than the Token size.
    type Token = u8;
    type Configuration = Variant;
}

// 8 bit main register
lazy_static! {
    pub static ref A: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("a"), size: 8, offset: 0, subscript: None };
}

// 8 bit index registers
lazy_static! {
    pub static ref X: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("x"), size: 8, offset: 0, subscript: None };
    pub static ref Y: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("y"), size: 8, offset: 0, subscript: None };
    pub static ref SP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("sp"), size: 8, offset: 0, subscript: None };
}

// 16 bit program counter
lazy_static! {
    pub static ref PC: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("pc"), size: 16, offset: 0, subscript: None };
}

// flags
lazy_static! {
    pub static ref N: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("N"), size: 1, offset: 0, subscript: None };
    pub static ref V: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("V"), size: 1, offset: 0, subscript: None };
    pub static ref D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("D"), size: 1, offset: 0, subscript: None };
    pub static ref I: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("I"), size: 1, offset: 0, subscript: None };
    pub static ref Z: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("Z"), size: 1, offset: 0, subscript: None };
    pub static ref C: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("C"), size: 1, offset: 0, subscript: None };
}

pub fn ram<A: Into<Rvalue>>(off: &A, width: u16) -> Lvalue {
    unimplemented!()
    /*Lvalue::Memory{
        offset: Box::new(off.to_rv()),
        name: "ram".to_string(),
        endianess: Endianess::Little,
        bytes: width / 8
    }*/
}

static GLOBAL_MOS_TEMPVAR_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn new_temp(bits: usize) -> Lvalue {
    unimplemented!()
    /*Lvalue::Variable{
        name: format!("__temp{}",GLOBAL_MOS_TEMPVAR_COUNT.fetch_add(1, Ordering::SeqCst)),
        width: bits as u16,
        subscript: None
    }*/
}

#[derive(Clone)]
pub struct Variant {
    pub arg0: Option<Rvalue>,
    pub int_vec: Vec<(&'static str,Rvalue,&'static str)>
}

impl Variant {
    pub fn new() -> Variant {
        Variant {
	    arg0: None,
            int_vec: vec![/*("ENTRY", Rvalue::Constant(0), "MCU Entry")*/],
        }
    }

    pub fn mos6502() -> Variant {
        Variant {
            arg0: None,
            int_vec: vec![
              /*  ("NMI",Rvalue::Memory{ offset: Box::new(Rvalue::Constant(0xfffa)), bytes: 2, endianess: Endianess::Little, name: "ram".to_string() }, "NMI vector"),
                ("RESET",Rvalue::Memory{ offset: Box::new(Rvalue::Constant(0xfffc)), bytes: 2, endianess: Endianess::Little, name: "ram".to_string() }, "Reset routine"),
                ("IRQ/BRK",Rvalue::Memory{ offset: Box::new(Rvalue::Constant(0xfffe)), bytes: 2, endianess: Endianess::Little, name: "ram".to_string() }, "Interrupt routine")*/
            ],
        }
    }

    pub fn wrap(&self, addr: u64) -> Rvalue {
        Rvalue::new_u64(addr % (1u64 << 16))
    }

    pub fn wrap_signed(&self, addr: i64) -> Rvalue {
        let mask = 1i64 << 16;
        Rvalue::new_u64((((addr % mask) + mask) % mask) as u64)
    }
}
