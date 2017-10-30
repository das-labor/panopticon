/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015  Panopticon authors
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

//! A graph of functions and symbolic references.
//!
//! The edges of the function graph (call graph) signify that one function calls another. Aside
//! from functions symbolic references are also part of the call graph. These are placeholders for
//! external functions imported from dynamic libraries.
//!
//! Program instances also have a human-readable name and a unique ID.
//!
//! Unlike the basic block graph of a function, a call graph has no error nodes. If disassembling a
//! function fails, it will still be added to the call graph. The function will only have a single
//! error node.

use {Statement, Operation, Rvalue};
use petgraph::visit::{IntoNodeReferences};
// use stable when API is at parity with Graph
//use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::graph::{NodeIndex, Graph};
use uuid::Uuid;

use std::collections::HashMap;

pub use neo::Function as Function;

/// An iterator over every Function in this Program
pub struct FunctionIterator<'a, IL: 'a> {
    iter: Box<Iterator<Item = &'a CallTarget<IL>> + 'a>
}

impl<'a, IL> FunctionIterator<'a, IL> {
    /// Create a new function iterator from the `cfg`
    pub fn new(cfg: &'a CallGraph<IL>) -> Self {
        let iter = Box::new(cfg.node_indices().filter_map(move |idx| cfg.node_weight(idx)));
        FunctionIterator {
            iter
        }
    }
}

impl<'a, IL> Iterator for FunctionIterator<'a, IL> {
    type Item = &'a Function<IL>;
    fn next(&mut self) ->  Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(&CallTarget::Concrete(ref function)) => return Some(function),
                _ => ()
            }
        }
    }
}

/// An iterator over every Function in this Program
pub struct FunctionMutIterator<'a, IL: 'a> {
    iter: Box<Iterator<Item = &'a mut CallTarget<IL>> + 'a>
}

impl<'a, IL> FunctionMutIterator<'a, IL> {
    /// Create a new function iterator from the `cfg`
    pub fn new(cfg: &'a mut CallGraph<IL>) -> Self {
        let iter = Box::new(cfg.node_weights_mut());
        FunctionMutIterator {
            iter
        }
    }
}

impl<'a, IL> Iterator for FunctionMutIterator<'a, IL> {
    type Item = &'a mut Function<IL>;
    fn next(&mut self) ->  Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(&mut CallTarget::Concrete(ref mut function)) => return Some(function),
                _ => ()
            }
        }
    }
}

/// Node of the program call graph.
#[derive(Serialize,Deserialize,Debug)]
pub enum CallTarget<IL> {
    /// Resolved and disassembled function.
    Concrete(Function<IL>),
    /// Reference to an external symbol.
    Symbolic(String, Uuid),
    /// Resolved but not yet disassembled function.
    Todo(Rvalue, Option<String>, Uuid),
}

impl<IL> CallTarget<IL> {
    /// Returns the UUID of the call graph node.
    pub fn uuid(&self) -> &Uuid {
        match self {
            &CallTarget::Concrete(ref f) => f.uuid(),
            &CallTarget::Symbolic(_, ref uuid) => uuid,
            &CallTarget::Todo(_, _, ref uuid) => uuid,
        }
    }
}

/// Graph of functions/symbolic references
pub type CallGraph<IL> = Graph<CallTarget<IL>, ()>;
/// Stable reference to a call graph node
pub type CallGraphRef = NodeIndex<u32>;

/// A collection of functions calling each other.
#[derive(Serialize,Deserialize,Debug)]
pub struct Program<IL> {
    /// Unique, immutable identifier
    pub uuid: Uuid,
    /// Human-readable name
    pub name: String,
    /// Graph of functions
    pub call_graph: CallGraph<IL>,
    /// Symbolic References (Imports)
    pub imports: HashMap<u64, String>,
}

impl<'a, IL> IntoIterator for &'a Program<IL> {
    type Item = &'a Function<IL>;
    type IntoIter = FunctionIterator<'a, IL>;
    fn into_iter(self) -> Self::IntoIter {
        FunctionIterator::new(&self.call_graph)
    }
}

impl<IL> Program<IL> {
    /// Create a new, empty `Program` named `n`.
    pub fn new(n: &str) -> Self {
        Program {
            uuid: Uuid::new_v4(),
            name: n.to_string(),
            call_graph: CallGraph::new(),
            imports: HashMap::new(),
        }
    }

    /// Returns a function if it matches the condition in the `filter` closure.
    pub fn find_function_by<'a, Filter: (Fn(&Function<IL>) -> bool)>(&'a self, filter: Filter) -> Option<&'a Function<IL>> {
        for (_, node) in self.call_graph.node_references() {
            match node {
                &CallTarget::Concrete(ref function) => if filter(function) { return Some(function) },
                _ => (),
            }
        }
        None
    }

    /// Returns a mutable reference to the first function that matches the condition in the `filter` closure.
    pub fn find_function_mut<'a, Filter: (Fn(&Function<IL>) -> bool)>(&'a mut self, filter: Filter) -> Option<&'a mut Function<IL>> {
        let mut idx = None;
        for (nidx, node) in self.call_graph.node_references() {
            match node {
                &CallTarget::Concrete(ref function) => if filter(function) { idx = Some(nidx); break },
                _ => (),
            }
        }
        match idx {
            Some(idx) => {
                let ct = self.call_graph.node_weight_mut(idx).unwrap();
                match ct {
                    &mut CallTarget::Concrete(ref mut function) => Some(function),
                    _ => unreachable!()
                }
            },
            None => None
        }
    }

    /// Returns a reference to the function with an entry point starting at `start`.
    pub fn find_function_by_entry(&self, start: u64) -> Option<CallGraphRef> {
        self.call_graph
            .node_indices()
            .find(
                |&x| match self.call_graph.node_weight(x) {
                    Some(&CallTarget::Concrete(ref s)) => {
                        s.entry_address() == start
                    }
                    _ => false,
                }
            )
    }

    /// Returns the function with UUID `a`.
    pub fn find_function_by_uuid<'a>(&'a self, a: &Uuid) -> Option<&'a Function<IL>> {
        for (_, node) in self.call_graph.node_references() {
            match node {
                &CallTarget::Concrete(ref function) => if function.uuid() == a { return Some(function) },
                _ => (),
            }
        }
        None
    }

    /// Returns the function with UUID `a`.
    pub fn find_function_by_uuid_mut<'a>(&'a mut self, a: &Uuid) -> Option<&'a mut Function<IL>> {
        for ct in self.call_graph.node_weights_mut() {
            match ct {
                &mut CallTarget::Concrete(ref mut s) => if s.uuid() == a { return Some(s) },
                _ => (),
            }
        }
        None
    }

    /// Puts `function` into the call graph, returning the UUIDs of all _new_ `Todo`s
    /// that are called by `function`
    pub fn insert(&mut self, function: Function<IL>) -> Vec<Uuid> {
        let maybe_vx = self.call_graph.node_indices().find(|ct| self.call_graph.node_weight(*ct).unwrap().uuid() == function.uuid());

        // FIXME: add collect calls
        //let calls = function.collect_calls();
        let calls = vec![];
        let new_vx = if let Some(vx) = maybe_vx {
            *self.call_graph.node_weight_mut(vx).unwrap() = CallTarget::Concrete(function);
            vx
        } else {
            self.call_graph.add_node(CallTarget::Concrete(function))
        };

        let mut other_funs = Vec::new();
        let mut todos = Vec::new();

        for a in calls {
            let l = other_funs.len();

            for w in self.call_graph.node_indices() {
                match self.call_graph.node_weight(w) {
                    Some(&CallTarget::Concrete(ref function)) => {
                        if let Rvalue::Constant { ref value, .. } = a {
                            if *value == function.entry_address() {
                                other_funs.push(w);
                                break;
                            }
                        }
                    }
                    Some(&CallTarget::Todo(ref _a, _, _)) => {
                        if *_a == a {
                            other_funs.push(w);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if l == other_funs.len() {
                let uu = Uuid::new_v4();
                let v = self.call_graph.add_node(CallTarget::Todo(a, None, uu));

                self.call_graph.add_edge(new_vx, v, ());
                todos.push(uu);
            }
        }

        for other_fun in other_funs {
            if self.call_graph.find_edge(new_vx, other_fun) == None {
                self.call_graph.add_edge(new_vx, other_fun, ());
            }
        }

        todos
    }

    /// Returns the function, todo item or symbolic reference with UUID `uu`.
    pub fn find_call_target_by_uuid(&self, uu: &Uuid) -> Option<CallGraphRef> {
        for (id, node) in self.call_graph.node_references() {
            if node.uuid() == uu {
                return Some(id);
            }
        }
        None
    }

    /// Returns an iterator over every Function in this program
    pub fn functions(&self) -> FunctionIterator<IL> {
        FunctionIterator::new(&self.call_graph)
    }

    /// Returns a mutable iterator over every Function in this program
    pub fn functions_mut(&mut self) -> FunctionMutIterator<IL> {
        FunctionMutIterator::new(&mut self.call_graph)
    }
    /// Calls [Function::set_plt](../function/struct.Function.html#method.set_plt) on all matching functions
    pub fn update_plt(&mut self) {
        for ct in self.call_graph.node_indices() {
            match self.call_graph.node_weight_mut(ct).unwrap() {
                &mut CallTarget::Concrete(ref mut function) => {
                    let address = {
                        let mut last = None;
                        let mut count = 0;
                        for bb in function.basic_blocks() {
                            // FIXME: add statements back
//                            for statement in function.statements(bb) {
//                                count += 1;
//                                last = Some(statement);
//                            }
                        }
                        if count == 2 {
                            // FIXME: needs language Load trait :/
                            if let Some( &Statement { op: Operation::Load(_, _, _, Rvalue::Constant { value, .. }), .. }) = last {
                                Some(value)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };
                    if let Some(address) = address {
                        match self.imports.get(&address) {
                            Some(import) => {
                                function.set_plt(import, address);
                            },
                            None => (),
                        }
                    }
                },
                _ => (),
            }
        }
    }

    pub fn iter_callgraph<'a>(&'a self) -> Box<Iterator<Item = &'a CallTarget<IL>> + 'a> {
        Box::new(self.call_graph.node_references().map(|(_, node)| node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {BasicBlock, ControlFlowTarget, Function, Lvalue, Mnemonic, Operation, Region, Rvalue, Statement};
    use panopticon_graph_algos::{AdjacencyMatrixGraphTrait, EdgeListGraphTrait, GraphTrait, MutableGraphTrait, VertexListGraphTrait};
    use uuid::Uuid;

    #[test]
    fn find_by_entry() {
        let mut prog = Program::new("prog_test");
        let mut func = Function::undefined(0, None, &Region::undefined("ram".to_owned(), 100), Some("test".to_owned()));

        let bb0 = BasicBlock::from_vec(vec![Mnemonic::dummy(0..10)]);
        let vx = func.cfg_mut().add_vertex(ControlFlowTarget::Resolved(bb0));
        func.set_entry_point_ref(vx);

        let func2_start = 0xdeadbeef;
        // technically passing func2_start is useless here since we overwrite it with bb below
        let mut func2 = Function::undefined(func2_start, None, &Region::undefined("ram".to_owned(), 100), Some("test2".to_owned()));
        let bb1 = BasicBlock::from_vec(vec![Mnemonic::dummy(func2_start..5)]);
        let vx = func2.cfg_mut().add_vertex(ControlFlowTarget::Resolved(bb1));
        func2.set_entry_point_ref(vx);

        let vx1 = prog.call_graph.add_vertex(CallTarget::Concrete(func));
        let vx2 = prog.call_graph.add_vertex(CallTarget::Concrete(func2));

        assert_eq!(prog.find_function_by_entry(0), Some(vx1));
        assert_eq!(prog.find_function_by_entry(func2_start), Some(vx2));
        assert_eq!(prog.find_function_by_entry(2), None);
    }

    #[test]
    fn insert_replaces_todo() {
        let uu = Uuid::new_v4();
        let mut prog = Program::new("prog_test");

        let tvx = prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(12), None, uu));
        let vx0 = prog.call_graph.add_vertex(CallTarget::Concrete(Function::undefined(0, None, &Region::undefined("ram".to_owned(), 100), Some("test".to_owned()))));
        let vx1 = prog.call_graph.add_vertex(CallTarget::Concrete(Function::undefined(0, None, &Region::undefined("ram".to_owned(), 100), Some("test2".to_owned()))));

        let e1 = prog.call_graph.add_edge((), tvx, vx0);
        let e2 = prog.call_graph.add_edge((), vx1, tvx);

        let mut func = Function::undefined(0, Some(uu.clone()), &Region::undefined("ram".to_owned(), 100), Some("test3".to_owned()));
        let bb0 = BasicBlock::from_vec(vec![Mnemonic::dummy(12..20)]);
        let vx = func.cfg_mut().add_vertex(ControlFlowTarget::Resolved(bb0));
        func.set_entry_point_ref(vx);
        let uuf = func.uuid().clone();

        let new = prog.insert(func);

        assert_eq!(new, vec![]);

        if let Some(&CallTarget::Concrete(ref f)) = prog.call_graph.vertex_label(tvx) {
            assert_eq!(f.uuid(), &uuf);
        } else {
            unreachable!();
        }
        assert!(prog.call_graph.vertex_label(vx0).is_some());
        assert!(prog.call_graph.vertex_label(vx1).is_some());
        assert_eq!(prog.call_graph.edge(tvx, vx0), e1);
        assert_eq!(prog.call_graph.edge(vx1, tvx), e2);
        assert_eq!(prog.call_graph.num_edges(), 2);
        assert_eq!(prog.call_graph.num_vertices(), 3);
    }

    #[test]
    fn insert_ignores_new_todo() {
        let uu1 = Uuid::new_v4();
        let uu2 = Uuid::new_v4();
        let mut prog = Program::new("prog_test");

        let tvx = prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(12), None, uu1));

        let mut func = Function::undefined(0, Some(uu2.clone()), &Region::undefined("ram".to_owned(), 100), Some("test3".to_owned()));
        let ops1 = vec![];
        let i1 = vec![
            Statement {
                op: Operation::Call(Rvalue::new_u64(12)),
                assignee: Lvalue::Undefined,
            },
        ];
        let mne1 = Mnemonic::new(
            0..10,
            "call".to_string(),
            "12".to_string(),
            ops1.iter(),
            i1.iter(),
        )
                .ok()
                .unwrap();
        let bb0 = BasicBlock::from_vec(vec![mne1]);
        let vx = func.cfg_mut().add_vertex(ControlFlowTarget::Resolved(bb0));
        func.set_entry_point_ref(vx);
        let uuf = func.uuid().clone();

        let new = prog.insert(func);

        assert_eq!(new, vec![]);

        if let Some(&CallTarget::Concrete(ref f)) = prog.call_graph.vertex_label(tvx) {
            assert_eq!(f.uuid(), &uuf);
        }
        assert!(prog.call_graph.vertex_label(tvx).is_some());
        assert_eq!(prog.call_graph.num_edges(), 1);
        assert_eq!(prog.call_graph.num_vertices(), 2);
    }
}
