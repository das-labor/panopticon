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

//! 8-bit AVR disassembler.
//!
//! This disassembler handles the 8-bit AVR microcontroller instruction set including XMEGA.

#![allow(missing_docs)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate panopticon_core;
extern crate panopticon_graph_algos;
extern crate byteorder;

mod syntax;
mod semantic;

mod disassembler;
pub use crate::disassembler::{Avr, Mcu};
