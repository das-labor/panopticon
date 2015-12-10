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
use value::{Lvalue,Rvalue,Endianess,ToRvalue};
use codegen::CodeGen;
use guard::Guard;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub mod syntax;

#[derive(Clone)]
pub enum Mos {}

impl Architecture for Mos {
    type Token = u8;
    type Configuration = Variant;
}

#[derive(Clone)]
pub struct Variant {
    pub pc_bits: u16,                                   ///< width of the program counter in bits
    pub int_vec: Vec<(&'static str,u64,&'static str)>   ////< interrupt vector: (name, offset, comment)
}

impl Variant {
    pub fn new() -> Variant {
        Variant {
            pc_bits: 16,
            int_vec: vec![("RESET", 0, "MCU Reset Interrupt")],
        }
    }

    pub fn mos6502() -> Variant {
        Variant {
	    pc_bits: 16,
	    int_vec: vec![("RESET", 0, "MOS 6502 Reset Interrupt")],
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
