/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
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

//! Collection of data flow algorithms.
//!
//! This module contains algorithms to convert RREIL code into SSA form. Aside from SSA form this
//! module implements functions to compute liveness sets and basic reverse data flow information.

extern crate panopticon_core;
extern crate panopticon_graph_algos;
extern crate bit_set;
extern crate petgraph;

mod liveness;
pub use liveness::{liveness, liveness_sets};

mod ssa;
pub use ssa::{flag_operations, ssa_convertion, type_check};

pub mod neo;
