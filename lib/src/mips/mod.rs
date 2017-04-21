/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017 Panopticon Authors
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

//! MIPS disassembler.
//!
//! This disassembler handles the MIPS instruction set.

#![allow(missing_docs)]

use {
    Lvalue, Rvalue,
    Guard,
    Result,
    Region,
    State,
    Match,
    Statement,
    Architecture,
};

#[derive(Clone, Debug)]
pub enum Mos {}

impl Architecture for Mos {
    type Token = u8;
    type Configuration = ();

    fn prepare(reg: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
        unimplemented!()
    }

    fn decode(reg: &Region, addr: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
        unimplemented!()
    }
}
