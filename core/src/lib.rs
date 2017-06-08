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

//! A library for disassembling and analysing binary code.
//!
//! The panopticon crate implements structures to model the in-memory representation of a
//! program including is control flow, call graph and memory maps.
//! The most important types and their interaction are as follows:
//!
//! ```text
//! Project
//! ├── Region
//! │   └── Layer
//! └── Program
//!     └── Function
//!         └── BasicBlock
//!             └── Mnemonic
//!                 └── Statement
//! ```
//!
//! The [`Program`](program/index.html), [`Function`](function/index.html),
//! [`BasicBlock`](basic_block/index.html) and [`Statement`](il/struct.Statement.html)
//! types model the behaviour of code.
//! The [`Region`](region/index.html) and [`Layer`](layer/index.html) types
//! represent how the program is laid out in memory.
//!
//! # Code
//!
//! Panopticon models code as a collection of programs. Each
//! [`Program`](program/index.html) consists of functions. A [`Function`](function/index.html) a graph with nodes representing a
//! sequence of instructions and edges representing jumps. These instruction sequences are [`BasicBlock`s](basic_block/index.html)
//! and contain a list of [`Mnemonic`](mnemonic/index.html)s. The meaning of each
//! `Mnemonic` is described in the [RREIL][1] language. Each mnemonic includes a sequence of
//! [`Statement`s](il/struct.Statement.html) implementing it.
//!
//! Panopticon allows multiple programs per project. For example, imagine a C# application that calls into a
//! native DLL written in C. Such an application would have two program instances. One for the CIL
//! code of the C# part of the application and one for the AMD64 object code inside the DLL.
//!
//! The [`Disassembler`](disassembler/index.html) and [`CodeGen`](codegen/index.html) are used to fill `Function`
//! structures with `Mnemonic`s.
//!
//! # Data
//!
//! The in-memory layout of an executable is modeled using the [`Region`](region/index.html), [`Layer`](layer/index.html) and
//! [`Cell`](layer/type.Cell.html) types. All data is organized into `Region`s. Each `Region` is an array of
//! `Cell`s numbered from 0 to n. Each `Cell` is an is either
//! undefined or has a value between 0 and 255 (both including). `Region`s are read
//! only. Changing their contents is done by applying `Layer` instance to them. A `Layer`
//! reads part of a `Region` or another `Layer` and returns a new `Cell` array. For example, `Layer`
//! can decrypt parts of a `Region` or replace individual `Cell`s with new
//! ones.
//!

//! In normal operation there is one `Region` for each memory address space, one on
//! Von-Neumann machines two on Harvard architectures. Other uses for `Region`s are
//! applying functions to `Cell` array where the result is not equal in size to the
//! input (for example uncompressing parts of the executable image).

#![recursion_limit="100"]
#![warn(missing_docs)]

#[macro_use]
extern crate log;

extern crate num;
extern crate flate2;
extern crate panopticon_graph_algos;
extern crate uuid;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate byteorder;
extern crate goblin;
extern crate quickcheck;

#[cfg(test)]
extern crate env_logger;


// core
pub mod disassembler;
pub use disassembler::{Architecture, Disassembler, Match, State};

#[macro_use]
pub mod il;
pub use il::{Guard, Lvalue, Operation, Rvalue, Statement, execute, lift};

pub mod mnemonic;
pub use mnemonic::{Bound, Mnemonic, MnemonicFormatToken};
pub mod basic_block;
pub use basic_block::BasicBlock;

pub mod function;
pub use function::{ControlFlowEdge, ControlFlowGraph, ControlFlowRef,
                   ControlFlowTarget, Function};

pub mod program;
pub use program::{CallGraph, CallGraphRef, CallTarget, Program};

pub mod project;
pub use project::Project;

pub mod region;
pub use region::{Region, World};

pub mod layer;
pub use layer::{Layer, LayerIter, OpaqueLayer};

pub mod result;
pub use result::{Error, Result};

// file formats
pub mod loader;
pub use loader::{Machine, load};
