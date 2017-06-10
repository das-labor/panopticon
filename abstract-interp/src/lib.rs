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

//! Abstract Interpretation Framework.
//!
//! Abstract Interpretation executes an program over sets of concrete values. Each operation is
//! extended to work on some kind of abstract value called abstract domain that approximates a
//! set of concrete values (called the concrete domain). A simple example is the domain of signs.
//! Each value can either be positive, negative, both or none. An abstract interpretation will first
//! replace all constant values with their signs and then execute all basic blocks using the
//! abstract sign domain. For example multiplying two positive values yields a positive value.
//! Adding a positive and a negative sign yields an abstract value representing both signs (called
//! join).

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
extern crate env_logger;

extern crate panopticon_core;
extern crate panopticon_data_flow;
extern crate panopticon_graph_algos;

mod interpreter;
pub use interpreter::{Aoperation, Avalue, Constraint, ProgramPoint, approximate, results, lift, translate};

mod bounded_addr_track;
pub use bounded_addr_track::BoundedAddrTrack;

pub mod kset;
pub use kset::Kset;

mod widening;
pub use widening::Widening;
