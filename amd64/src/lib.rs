/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016  Panopticon authors
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

//! Intel x86 and AMD64 disassembler.
//!
//! This disassembler handles the Intel x86 instruction set from the 16-bit 8086 over 32-bit x86 to
//! 64-bit AMD64 instructions, including MMX, SSE1-4, AVX, x87 and miscellaneous instruction set
//! extensions.
//!
//! Decoding instructions is done my a hierarchy of tables defined in `tables.rs`. For each
//! mnemonic a function is defined in `semantic.rs` that emits RREIL code implementing it.
//!
//! Instruction decoding is done in `read`. It expects a 15 bytes of code and will decode
//! instruction prefixes, opcode (including escapes) and opcode arguments encoded in the MODR/M
//! byte and/or immediates.
//!
//! All functions in `semantic.rs` follow the same structure. They get the decoded opcode arguments
//! as input and return a vector of RREIL statements and a `JumpSpec` instance that tells the
//! disassembler where to continue.

#![allow(missing_docs)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate panopticon_core;
extern crate byteorder;

#[macro_use]
pub mod tables;
pub mod semantic;

mod disassembler;
pub use disassembler::{AddressingMethod, JumpSpec, MnemonicSpec, Opcode, Operand, OperandSpec, OperandType, read_spec_register};

mod architecture;
pub use architecture::{Amd64, Mode, REGISTERS};
