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


use {Function, Statement, Operation, Rvalue};
use panopticon_graph_algos::{AdjacencyList, AdjacencyMatrixGraphTrait, GraphTrait, MutableGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::adjacency_list::{AdjacencyListVertexDescriptor, VertexLabelIterator, VertexLabelMutIterator};
use uuid::Uuid;

/// An iterator over every Function in this Program
pub struct FunctionIterator<'a> {
    iter: VertexLabelIterator<'a, CallGraphRef, CallTarget>
}

impl<'a> FunctionIterator<'a> {
    /// Create a new function iterator from the `cfg`
    pub fn new(cfg: &'a CallGraph) -> Self {
        FunctionIterator {
            iter: cfg.vertex_labels(),
        }
    }
}

impl<'a> Iterator for FunctionIterator<'a> {
    type Item = &'a Function;
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
pub struct FunctionMutIterator<'a> {
    iter: VertexLabelMutIterator<'a, CallGraphRef, CallTarget>
}

impl<'a> FunctionMutIterator<'a> {
    /// Create a new function iterator from the `cfg`
    pub fn new(cfg: &'a mut CallGraph) -> Self {
        FunctionMutIterator {
            iter: cfg.vertex_labels_mut(),
        }
    }
}

impl<'a> Iterator for FunctionMutIterator<'a> {
    type Item = &'a mut Function;
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
pub enum CallTarget {
    /// Resolved and disassembled function.
    Concrete(Function),
    /// Reference to an external symbol.
    Symbolic(String, Uuid),
    /// Resolved but not yet disassembled function.
    Todo(Rvalue, Option<String>, Uuid),
}

impl CallTarget {
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
pub type CallGraph = AdjacencyList<CallTarget, ()>;
/// Stable reference to a call graph node
pub type CallGraphRef = AdjacencyListVertexDescriptor;

/// A collection of functions calling each other.
#[derive(Serialize,Deserialize,Debug)]
pub struct Program {
    /// Unique, immutable identifier
    pub uuid: Uuid,
    /// Human-readable name
    pub name: String,
    /// Graph of functions
    pub call_graph: CallGraph,
    /// Symbolic References (Imports)
    pub imports: ::std::collections::HashMap<u64, String>,
}

impl<'a> IntoIterator for &'a Program {
    type Item = &'a Function;
    type IntoIter = FunctionIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        FunctionIterator::new(&self.call_graph)
    }
}

impl Program {
    /// Create a new, empty `Program` named `n`.
    pub fn new(n: &str) -> Program {
        Program {
            uuid: Uuid::new_v4(),
            name: n.to_string(),
            call_graph: CallGraph::new(),
            imports: ::std::collections::HashMap::new(),
        }
    }

    /// Returns a function if it matches the condition in the `filter` closure.
    pub fn find_function_by<'a, F: (Fn(&Function) -> bool)>(&'a self, filter: F) -> Option<&'a Function> {
        for ct in self.call_graph.vertex_labels() {
            match ct {
                &CallTarget::Concrete(ref function) => if filter(function) { return Some(function) },
                _ => (),
            }
        }
        None
    }

    /// Returns a mutable reference to the first function that matches the condition in the `filter` closure.
    pub fn find_function_mut<'a, F: (Fn(&Function) -> bool)>(&'a mut self, filter: F) -> Option<&'a mut Function> {
        for ct in self.call_graph.vertex_labels_mut() {
            match ct {
                &mut CallTarget::Concrete(ref mut function) => if filter(function) { return Some(function) },
                _ => (),
            }
        }
        None
    }

    /// Returns a reference to the function with an entry point starting at `start`.
    pub fn find_function_by_entry(&self, start: u64) -> Option<CallGraphRef> {
        self.call_graph
            .vertices()
            .find(
                |&x| match self.call_graph.vertex_label(x) {
                    Some(&CallTarget::Concrete(ref s)) => {
                        s.start() == start
                    }
                    _ => false,
                }
            )
    }

    /// Returns the function with UUID `a`.
    pub fn find_function_by_uuid<'a>(&'a self, a: &Uuid) -> Option<&'a Function> {
        for ct in self.call_graph.vertex_labels() {
            match ct {
                &CallTarget::Concrete(ref s) => if s.uuid() == a { return Some(s) },
                _ => (),
            }
        }
        None
    }

    /// Returns the function with UUID `a`.
    pub fn find_function_by_uuid_mut<'a>(&'a mut self, a: &Uuid) -> Option<&'a mut Function> {
        for ct in self.call_graph.vertex_labels_mut() {
            match ct {
                &mut CallTarget::Concrete(ref mut s) => if s.uuid() == a { return Some(s) },
                _ => (),
            }
        }
        None
    }

    /// Puts `function` into the call graph, returning the UUIDs of all _new_ `Todo`s
    /// that are called by `function`
    pub fn insert(&mut self, function: Function) -> Vec<Uuid> {
        let maybe_vx = self.call_graph.vertices().find(|ct| self.call_graph.vertex_label(*ct).unwrap().uuid() == function.uuid());

        let calls = function.collect_calls();
        let new_vx = if let Some(vx) = maybe_vx {
            *self.call_graph.vertex_label_mut(vx).unwrap() = CallTarget::Concrete(function);
            vx
        } else {
            self.call_graph.add_vertex(CallTarget::Concrete(function))
        };

        let mut other_funs = Vec::new();
        let mut todos = Vec::new();

        for a in calls {
            let l = other_funs.len();

            for w in self.call_graph.vertices() {
                match self.call_graph.vertex_label(w) {
                    Some(&CallTarget::Concrete(ref function)) => {
                        if let Rvalue::Constant { ref value, .. } = a {
                            if *value == function.start() {
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
                let v = self.call_graph.add_vertex(CallTarget::Todo(a, None, uu));

                self.call_graph.add_edge((), new_vx, v);
                todos.push(uu);
            }
        }

        for other_fun in other_funs {
            if self.call_graph.edge(new_vx, other_fun) == None {
                self.call_graph.add_edge((), new_vx, other_fun);
            }
        }

        todos
    }

    /// Returns the function, todo item or symbolic reference with UUID `uu`.
    pub fn find_call_target_by_uuid<'a>(&'a self, uu: &Uuid) -> Option<CallGraphRef> {
        for vx in self.call_graph.vertices() {
            if let Some(lb) = self.call_graph.vertex_label(vx) {
                if lb.uuid() == uu {
                    return Some(vx);
                }
            } else {
                unreachable!();
            }
        }

        None
    }

    /// Returns an iterator over every Function in this program
    pub fn functions(&self) -> FunctionIterator {
        FunctionIterator::new(&self.call_graph)
    }

    /// Returns a mutable iterator over every Function in this program
    pub fn functions_mut(&mut self) -> FunctionMutIterator {
        FunctionMutIterator::new(&mut self.call_graph)
    }
    /// Calls [Function::set_plt](../function/struct.Function.html#method.set_plt) on all matching functions
    pub fn update_plt(&mut self) {
        for ct in self.call_graph.vertex_labels_mut() {
            match ct {
                &mut CallTarget::Concrete(ref mut function) => {
                    let address = {
                        let mut last = None;
                        let mut count = 0;
                        for statement in function.statements() {
                            count += 1;
                            last = Some(statement);
                        }
                        if count == 2 {
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
