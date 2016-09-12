/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016 Kai Michaelis
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

//! Panopticon is a libre disassembler and binary analysis tool. It started as
//! platform for experiments in static program analysis and grew into a complete
//! library (libpanopticon) and a Qt 5 based UI. The library is written in Rust, the
//! UI is a mix of Rust and QML. Both are licensed as GPLv3.
//!
//! Library Overview
//! ----------------
//!
//! The libpanopticon implements structures to model the in-memory representation of a
//! program including is control flow, call graph and memory maps.
//! The most important types and their interaction are as follows:
//!
//! .. graphviz::
//!
//!   digraph G {
//!       rankdir=LR
//!       graph [bgcolor="transparent"]
//!       node [shape=rect]
//!       Project -> Program [label="contains"]
//!       Program -> Function [label="contains"]
//!       Function -> BasicBlock [label="contains"]
//!       BasicBlock -> Mnemonic [label="contains"]
//!       Mnemonic -> Statement [label="contains"]
//!       Project -> Region [label="contains"]
//!       Region -> Layer [label="contains"]
//!   }
//!
//! The `Program`, `Function`, `BasicBlock` and `Statement` types model the behaviour of code.
//! The `Region` and `Layer` types represent how the program is laid out in memory.
//!
//! Code
//! ~~~~
//!
//! Panopticon models code as a collection of programs contained in a file. Each
//! program consists of functions. A function is a control flow
//! graph, e.g. a graph with nodes representing a sequence of instructions and
//! directed edges for (un)conditional jumps. These instruction sequences are basic
//! blocks and contain a list of mnemonics. Panopticon models the semantic of each
//! mnemonic using the RREIL language. Each mnemonic instance
//! has a sequence of RREIL instructions (`Statement` type) implementing it.
//!
//! Panopticon allows multiple programs per project. An example for that would be a
//! C# application that calls functions of a DLL written in C. Such an application
//! would have two program instances. One for the CIL code of the C# program and one
//! for the AMD64 of Intel 32 object code of the DLL.
//!
//! One of the key features of Panopticon is the ability to "understand" the binary.
//! The disassembler not only knowns about the shape of mnemonics (its syntax) but
//! also what is does (the semantics). Each mnemonic includes a short program in RREIL
//! the implements the mnemonic. This allows sophisticated analysis like symbolic
//! execution, automatic crafting of input to reach certain basic blocks,
//! decompilation and computing bounds on register values without executing the code.
//!
//! Instances of the `Program`, `Function`, `BasicBlock` and `Statement` types are created by
//! the disassembler subsystem (`Disassembler` and `CodeGen` types). A
//! `Disassembler` is given a range of data and an instruction set architecture and
//! creates a model of the code found in the data.
//!
//! Sources
//! ~~~~~~~
//!
//! The in-memory layout of an executable is modeled using the `Region`, `Layer` and
//! `Cell` types. All data is organized into `Region`s. Each `Region` is an array of
//! `Cell`s numbered from 0 to n. Each `Cell` is an is either
//! undefined or has a value between 0 and 255 (both including). `Region`s are read
//! only. Changing their contents is done by applying `Layer` instance to them. A `Layer`
//! reads part of a `Region` or another `Layer` and returns a new `Cell` array. `Layer`s
//! can for example decrypt parts of a `Region` or replace individual `Cell`s with new
//! ones.
//!
//! In normal operation there is one `Region` for each memory address space, one on
//! Von-Neumann machines two on Harvard architectures. Other uses for `Region`s are
//! applying functions to `Cell` array where the result is not equal in size to the
//! input (for example uncompressing parts of the executable image).
//!
//! Graphical UI
//! ------------
//!
//! The qtpanopticon application uses the functionality implemented in the
//! libpanopticon to allow browsing the disassembled code.
//!
//! The UI widgets are mostly implemented in QML ("qml/"), with glue functions written
//! in Rust to connect the QML code to libpanopticon. The UI includes a implementation
//! of DOT for layouting control flow graphs.
//!
//! Moving data to QML is done by JSON RPC.
//! This makes memory management easier and save us from implementing dozens of
//! QObject subclasses.

#![recursion_limit="100"]

#[macro_use]
extern crate log;

extern crate num;
extern crate rustc_serialize;
extern crate flate2;
extern crate graph_algos;
extern crate tempdir;
extern crate uuid;
extern crate rmp_serialize;

#[macro_use]
extern crate lazy_static;

extern crate byteorder;
extern crate goblin;

// core
pub mod disassembler;
pub use disassembler::{
    State,
    Architecture,
    Disassembler,
    Match,
};

#[macro_use]
pub mod il;
pub use il::{
    Rvalue,
    Lvalue,
    Guard,
    Statement,
    Operation,
    execute,
    lift,
};

pub mod mnemonic;
pub use mnemonic::{
    Mnemonic,
    MnemonicFormatToken,
    Bound,
};
pub mod basic_block;
pub use basic_block::{
    BasicBlock,
};

pub mod function;
pub use function::{
    Function,
    ControlFlowTarget,
    ControlFlowRef,
    ControlFlowEdge,
    ControlFlowGraph,
};

pub mod program;
pub use program::{
    Program,
    CallTarget,
    CallGraph,
    CallGraphRef,
    DisassembleEvent,
};

pub mod project;
pub use project::Project;

pub mod region;
pub use region::{
    Region,
    Regions,
};

pub mod layer;
pub use layer::{
    Layer,
    OpaqueLayer,
    LayerIter,
};

pub mod result;
pub use result::{
    Result,
    Error,
};

pub mod dataflow;
pub use dataflow::*;

pub mod abstractinterp;
pub use abstractinterp::{
    Kset,
    approximate,
};

// disassembler
pub mod avr;
pub mod amd64;
pub mod mos;

// file formats
pub mod pe;
pub mod elf;
