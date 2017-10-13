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
extern crate smallvec;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate env_logger;

//use panopticon_core::{Function, BasicBlock, Result, ControlFlowGraph, ControlFlowRef};
use panopticon_core::*;
use std::ops::Range;
use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

use petgraph::Graph;

pub trait StandardMnemonic {
    fn opcode(&self) -> &str;
    fn operands(&self) -> Vec<Rvalue>;
    fn area(&self) -> Range<u64>;
}

impl<'a> StandardMnemonic for &'a Mnemonic {
    fn opcode(&self) -> &str {
        self.opcode.as_str()
    }
    fn operands(&self) -> Vec<Rvalue> {
        self.operands.clone()
    }
    fn area(&self) -> Range<u64> {
        self.area.start..self.area.end
    }
}

pub trait StandardBlock<M: StandardMnemonic> {
    type Iter: Iterator<Item = M>;
    fn mnemonics(&self) -> Self::Iter;
}

impl<'a> StandardBlock<&'a Mnemonic> for &'a BasicBlock {
    type Iter = ::std::slice::Iter<'a, Mnemonic>;
    fn mnemonics(&self) -> Self::Iter {
        self.mnemonics.as_slice().iter()
    }
}

pub trait SSAFunction {
    fn ssa_conversion(&mut self) -> Result<()>;
    fn cfg(&self) -> &ControlFlowGraph;
    fn cfg_mut(&mut self) -> &mut ControlFlowGraph;
    fn entry_point_mut(&mut self) -> &mut BasicBlock;
    fn entry_point_ref(&self) -> ControlFlowRef;
    fn postorder(&self) -> Vec<ControlFlowRef>;
}

impl SSAFunction for Function {
    fn ssa_conversion(&mut self) -> Result<()> {
        ssa_convertion(self)
    }
    fn cfg(&self) -> &ControlFlowGraph {
        Function::cfg(self)
    }
    fn cfg_mut(&mut self) -> &mut ControlFlowGraph {
        Function::cfg_mut(self)
    }
    fn entry_point_mut(&mut self) -> &mut BasicBlock {
        Function::entry_point_mut(self)
    }
    fn entry_point_ref(&self) -> ControlFlowRef {
        Function::entry_point_ref(self)
    }
    fn postorder(&self) -> Vec<ControlFlowRef> {
        Function::postorder(self)
    }
}

impl SSAFunction for panopticon_core::neo::Function {
    fn ssa_conversion(&mut self) -> Result<()> {
        Ok(())
    }
    fn entry_point_mut(&mut self) -> &mut BasicBlock {
        unimplemented!()
    }
    fn entry_point_ref(&self) -> ControlFlowRef {
        unimplemented!()
    }
    fn cfg(&self) -> &ControlFlowGraph {
        unimplemented!()
    }
    fn cfg_mut(&mut self) -> &mut ControlFlowGraph {
        unimplemented!()
    }
    fn postorder(&self) -> Vec<ControlFlowRef> {
        unimplemented!()
    }
}

pub trait DataFlow: Sized {
    fn ssa_conversion(&mut self) -> Result<()> {
        Ok(())
    }
    /// Computes the set of global variables in `func` and their points of usage. Globals are
    /// variables that are used in multiple basic blocks. Returns (Globals,Usage).
    fn global_names(&self) -> (HashSet<Cow<'static, str>>, HashMap<Cow<'static, str>, HashSet<u32>>) {
        let (varkill, uevar) = liveness_sets_pet(self);
        let mut usage = HashMap::<Cow<'static, str>, HashSet<u32>>::new();
        let mut globals = HashSet::<Cow<'static, str>>::new();

        for (_, uev) in uevar {
            for v in uev {
                globals.insert(v);
            }
        }

        for (vx, vk) in varkill {
            for v in vk {
                usage.entry(v.clone()).or_insert(HashSet::new()).insert(vx);
            }
        }

        (globals, usage)
    }

    fn type_check(&self) -> Result<HashMap<Cow<'static, str>, usize>> {
        let mut ret = HashMap::<Cow<'static, str>, usize>::new();
        let cfg = self.cfg();
        fn set_len(v: &Rvalue, ret: &mut HashMap<Cow<'static, str>, usize>) {
            match v {
                &Rvalue::Variable { ref name, ref size, .. } => {
                    let val = *::std::cmp::max(ret.get(name).unwrap_or(&0), size);
                    ret.insert(name.clone(), val);
                }
                _ => {}
            }
        }
        for vx in cfg.node_indices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.node_weight(vx) {
                for mne in bb.mnemonics.iter() {
                    for o in mne.operands.iter() {
                        set_len(o, &mut ret);
                    }

                    for instr in mne.instructions.iter() {
                        let ops = instr.op.operands();
                        match ops.len() {
                            0 => return Err("Operation w/o arguments".into()),
                            _ => {
                                for o in ops.iter() {
                                    set_len(o, &mut ret);
                                }
                            }
                        }
                        set_len(&instr.assignee.clone().into(), &mut ret);
                    }
                }
            }
        }

        for ed in cfg.edge_indices() {
            if let Some(&Guard::Predicate { ref flag, .. }) = cfg.edge_weight(ed) {
                set_len(flag, &mut ret);
            }
        }
        Ok(ret)
    }
    fn entry_point_mut(&mut self) -> &mut BasicBlock;
    fn entry_point_ref(&self) -> u32;
    fn cfg(&self) -> &Graph<ControlFlowTarget, Guard>;
    fn cfg_mut(&mut self) -> &mut Graph<ControlFlowTarget, Guard>;
    fn postorder(&self) -> Vec<u32>;
}

mod liveness;
pub use liveness::{liveness, liveness_sets, liveness_sets_pet};

mod ssa;
pub use ssa::{flag_operations, ssa_convertion, type_check};

pub mod neo;
