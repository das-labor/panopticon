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

use panopticon_core::{Function, Result, Variable, Value, il};
use std::borrow::{Borrow};

pub trait DataFlow: Sized {
    /// Convert `func` into semi-pruned SSA form.
    fn ssa_conversion(&mut self) -> Result<()>;
//        let (globals, usage) = self.global_names();
//        self.phi_functions(&globals, &usage)?;
//        self.rename_variables(&globals)
//    }
//    /// Computes the set of global variables in `func` and their points of usage. Globals are
//    /// variables that are used in multiple basic blocks. Returns (Globals,Usage).
//    fn global_names(&self) -> (ssa::Globals, ssa::Usage) {
//        ssa::global_names(self)
//    }
//
//    /// Does a simple sanity check on all RREIL statements in `func`, returns every variable name
//    /// found and its maximal size in bits.
//    fn type_check(&self) -> Result<HashMap<Cow<'static, str>, usize>> {
//        ssa::type_check(self)
//    }
//
//    /// Inserts SSA Phi functions at junction points in the control flow graph of `func`. The
//    /// algorithm produces the semi-pruned SSA form found in Cooper, Torczon: "Engineering a Compiler".
//    fn phi_functions(&mut self, globals: &ssa::Globals, usage: &ssa::Usage) -> Result<()> {
//        ssa::phi_functions(self, globals, usage)
//    }
//
//    /// Sets the SSA subscripts of all variables in `func`. Follows the algorithm outlined
//    /// Cooper, Torczon: "Engineering a Compiler". The function expects that Phi functions to be
//    /// already inserted.
//    fn rename_variables(&mut self, globals: &ssa::Globals) -> Result<()> {
//        ssa::rename_variables(self, globals)
//    }
//    /// Computes for every control flow guard the dependent RREIL operation via reverse data flow
//    /// analysis.
//    fn flag_operations(&self) -> HashMap<ControlFlowEdge, Operation<Rvalue>> {
//        unimplemented!()
//        //ssa::flag_operations(self)
//    }
//    /// Computes for each basic block in `func` the set of live variables using simple fixed point
//    /// iteration.
//    fn liveness<Function: DataFlow>(&self) -> HashMap<ControlFlowRef, HashSet<Cow<'static, str>>> {
//        liveness::liveness(self)
//    }
}

impl DataFlow for Function<il::noop::Noop> {
    fn ssa_conversion(&mut self) -> Result<()> {
        Ok(())
    }
}

impl DataFlow for Function {
    // @flanfly: can technically implement it for neo by calling its specific functions in `neo::*` here, as it mutates self
    fn ssa_conversion(&mut self) -> Result<()> {
        rewrite_to_ssa(self)
    }
}

impl DataFlow for Function<il::RREIL> {
    fn ssa_conversion(&mut self) -> Result<()> {
        rewrite_to_ssa_rreil(self)
    }
}

pub trait DataFlowOperand {
    fn is_variable(&self) -> bool;
    fn name(&self) -> Option<&str>;
}
//
//pub trait DataFlowLvalue {
//    fn name(&self) -> &str;
//}
//
//pub trait DataFlowRvalue {
//    type Operand: DataFlowOperand;
//    fn operands(&self) -> Vec<&Self::Operand>;
//}
//
//pub trait DataFlowLanguage {
//    type Lvalue: DataFlowLvalue;
//    type Rvalue: DataFlowRvalue;
//    fn is_phi(&self) -> bool;
//    fn rvalue(&self) -> Option<&Self::Rvalue>;
//    fn lvalue(&self) -> Option<&Self::Lvalue>;
//}

impl DataFlowOperand for Value {
    fn is_variable(&self) -> bool {
        match self {
            &Value::Variable(..) => true,
            _ => false
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            &Value::Variable( Variable { ref name, ..}) => Some(name.borrow()),
            _ => None
        }
    }
}
//
//
//pub trait DF {
//    fn liveness_(&self) -> (HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>, HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>);
//}
//
//impl<IL: neo::Language> DF for panopticon_core::neo::Function<IL> where IL::Statement: DataFlowLanguage {
//    fn liveness_(&self) -> (HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>, HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>) {
//        unimplemented!()
//        //liveness_sets_neo(self)
//    }
//}

mod liveness;
pub use liveness::{Liveness,Globals};

mod ssa;
pub use ssa::{rewrite_to_ssa, rewrite_to_ssa_rreil};
