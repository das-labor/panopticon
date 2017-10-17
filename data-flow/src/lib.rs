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
extern crate bit_set;
extern crate petgraph;
extern crate smallvec;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate env_logger;

use panopticon_core::{Function, BasicBlock, Result, Guard, ControlFlowTarget, ControlFlowEdge, ControlFlowRef, Operation, Rvalue};
use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

use petgraph::Graph;

pub trait DataFlow: Sized {
    /// Convert `func` into semi-pruned SSA form.
    fn ssa_conversion(&mut self) -> Result<()> {
        let (globals, usage) = self.global_names();
        self.phi_functions(&globals, &usage)?;
        self.rename_variables(&globals)
    }
    /// Computes the set of global variables in `func` and their points of usage. Globals are
    /// variables that are used in multiple basic blocks. Returns (Globals,Usage).
    fn global_names(&self) -> (ssa::Globals, ssa::Usage) {
        ssa::global_names(self)
    }

    /// Does a simple sanity check on all RREIL statements in `func`, returns every variable name
    /// found and its maximal size in bits.
    fn type_check(&self) -> Result<HashMap<Cow<'static, str>, usize>> {
        ssa::type_check(self)
    }

    /// Inserts SSA Phi functions at junction points in the control flow graph of `func`. The
    /// algorithm produces the semi-pruned SSA form found in Cooper, Torczon: "Engineering a Compiler".
    fn phi_functions(&mut self, globals: &ssa::Globals, usage: &ssa::Usage) -> Result<()> {
        ssa::phi_functions(self, globals, usage)
    }

    /// Sets the SSA subscripts of all variables in `func`. Follows the algorithm outlined
    /// Cooper, Torczon: "Engineering a Compiler". The function expects that Phi functions to be
    /// already inserted.
    fn rename_variables(&mut self, globals: &ssa::Globals) -> Result<()> {
        ssa::rename_variables(self, globals)
    }
    /// Computes for every control flow guard the dependent RREIL operation via reverse data flow
    /// analysis.
    fn flag_operations(&self) -> HashMap<ControlFlowEdge, Operation<Rvalue>> {
        ssa::flag_operations(self)
    }
    /// Computes for each basic block in `func` the set of live variables using simple fixed point
    /// iteration.
    fn liveness<Function: DataFlow>(&self) -> HashMap<ControlFlowRef, HashSet<Cow<'static, str>>> {
        liveness::liveness(self)
    }

    fn entry_point_mut(&mut self) -> &mut BasicBlock;
    fn entry_point_ref(&self) -> ControlFlowRef;
    fn cfg(&self) -> &Graph<ControlFlowTarget, Guard>;
    fn cfg_mut(&mut self) -> &mut Graph<ControlFlowTarget, Guard>;
}

impl DataFlow for Function {
    fn entry_point_mut(&mut self) -> &mut BasicBlock {
        Function::entry_point_mut(self)
    }
    fn entry_point_ref(&self) -> ControlFlowRef {
        Function::entry_point_ref(self)
    }

    fn cfg(&self) -> &Graph<ControlFlowTarget, Guard> {
        Function::cfg(self)
    }

    fn cfg_mut(&mut self) -> &mut Graph<ControlFlowTarget, Guard> {
        Function::cfg_mut(self)
    }
}

impl DataFlow for panopticon_core::neo::Function {
    // @flanfly: can technically implement it for neo by calling its specific functions in `neo::*` here, as it mutates self
    fn ssa_conversion(&mut self) -> Result<()> {
        neo::rewrite_to_ssa(self).map_err(|e| {
            format!("{}", e).into()
        })
    }
    fn entry_point_mut(&mut self) -> &mut BasicBlock {
        unimplemented!()
    }
    fn entry_point_ref(&self) -> ControlFlowRef {
        unimplemented!()
    }

    fn cfg(&self) -> &Graph<ControlFlowTarget, Guard> {
        unimplemented!()
    }

    fn cfg_mut(&mut self) -> &mut Graph<ControlFlowTarget, Guard> {
        unimplemented!()
    }
}

mod liveness;
mod ssa;

pub mod neo;
