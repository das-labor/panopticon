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

extern crate panopticon_core;
extern crate panopticon_graph_algos;
extern crate futures;
extern crate uuid;

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Panopticon(::panopticon_core::Error);
            Time(::std::time::SystemTimeError);
            Io(::std::io::Error);
            NulError(::std::ffi::NulError);
            UuidParseError(::uuid::ParseError);
        }
    }
}
pub use crate::errors::{Error, ErrorKind, Result};

mod ffi;

mod glue;
pub use crate::glue::Glue;

mod types;
pub use crate::types::{CBasicBlockLine, CBasicBlockOperand};
