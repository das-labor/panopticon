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


use {ControlFlowTarget, Function, Rvalue};
use panopticon_graph_algos::{AdjacencyList, AdjacencyMatrixGraphTrait, GraphTrait, MutableGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor;
use uuid::Uuid;

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
    pub fn uuid(&self) -> Uuid {
        match self {
            &CallTarget::Concrete(Function { uuid, .. }) => uuid,
            &CallTarget::Symbolic(_, uuid) => uuid,
            &CallTarget::Todo(_, _, uuid) => uuid,
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
}

impl Program {
    /// Create a new, empty `Program` named `n`.
    pub fn new(n: &str) -> Program {
        Program {
            uuid: Uuid::new_v4(),
            name: n.to_string(),
            call_graph: CallGraph::new(),
        }
    }

    /// Returns a reference to the function with an entry point starting at `a`.
    pub fn find_function_by_entry(&self, a: u64) -> Option<CallGraphRef> {
        self.call_graph
            .vertices()
            .find(
                |&x| match self.call_graph.vertex_label(x) {
                    Some(&CallTarget::Concrete(ref s)) => {
                        if let Some(e) = s.entry_point {
                            if let Some(&ControlFlowTarget::Resolved(ref ee)) = s.cflow_graph.vertex_label(e) {
                                ee.area.start == a
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            )
    }

    /// Returns the function with UUID `a`.
    pub fn find_function_by_uuid<'a>(&'a self, a: &Uuid) -> Option<&'a Function> {
        self.call_graph
            .vertices()
            .find(
                |&x| match self.call_graph.vertex_label(x) {
                    Some(&CallTarget::Concrete(ref s)) => s.uuid == *a,
                    _ => false,
                }
            )
            .and_then(
                |r| match self.call_graph.vertex_label(r) {
                    Some(&CallTarget::Concrete(ref s)) => Some(s),
                    _ => None,
                }
            )
    }

    /// Returns the function with UUID `a`.
    pub fn find_function_by_uuid_mut<'a>(&'a mut self, a: &Uuid) -> Option<&'a mut Function> {
        let ct = self.call_graph
            .vertices()
            .find(
                |&x| match self.call_graph.vertex_label(x) {
                    Some(&CallTarget::Concrete(ref s)) => s.uuid == *a,
                    _ => false,
                }
            );

        if ct.is_none() {
            return None;
        }

        match self.call_graph.vertex_label_mut(ct.unwrap()) {
            Some(&mut CallTarget::Concrete(ref mut s)) => Some(s),
            _ => None,
        }
    }

    /// Puts function/reference `new_ct` into the call graph, returning the UUIDs of all functions
    /// that are called by `new_ct` and call `new_ct`.
    pub fn insert(&mut self, new_ct: CallTarget) -> Vec<Uuid> {
        let new_uu = new_ct.uuid();
        let maybe_vx = self.call_graph.vertices().find(|ct| self.call_graph.vertex_label(*ct).unwrap().uuid() == new_uu);

        let new_vx = if let Some(vx) = maybe_vx {
            *self.call_graph.vertex_label_mut(vx).unwrap() = new_ct;
            vx
        } else {
            self.call_graph.add_vertex(new_ct)
        };

        let mut other_funs = Vec::new();
        let mut ret = Vec::new();
        let calls = if let Some(&CallTarget::Concrete(ref fun)) = self.call_graph.vertex_label(new_vx) {
            fun.collect_calls()
        } else {
            vec![]
        };

        for a in calls {
            let l = other_funs.len();

            for w in self.call_graph.vertices() {
                match self.call_graph.vertex_label(w) {
                    Some(&CallTarget::Concrete(Function { cflow_graph: ref cg, entry_point: Some(ent), .. })) => {
                        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cg.vertex_label(ent) {
                            if let Rvalue::Constant { ref value, .. } = a {
                                if *value == bb.area.start {
                                    other_funs.push(w);
                                    break;
                                }
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
                ret.push(uu);
            }
        }

        for other_fun in other_funs {
            if self.call_graph.edge(new_vx, other_fun) == None {
                self.call_graph.add_edge((), new_vx, other_fun);
            }
        }

        ret
    }

    /// Returns the function, todo item or symbolic reference with UUID `uu`.
    pub fn find_call_target_by_uuid<'a>(&'a self, uu: &Uuid) -> Option<CallGraphRef> {
        for vx in self.call_graph.vertices() {
            if let Some(lb) = self.call_graph.vertex_label(vx) {
                if lb.uuid() == *uu {
                    return Some(vx);
                }
            } else {
                unreachable!();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {BasicBlock, ControlFlowTarget, Function, Lvalue, Mnemonic, Operation, Rvalue, Statement};
    use panopticon_graph_algos::{AdjacencyMatrixGraphTrait, EdgeListGraphTrait, GraphTrait, MutableGraphTrait, VertexListGraphTrait};
    use uuid::Uuid;

    #[test]
    fn find_by_entry() {
        let mut prog = Program::new("prog_test");
        let mut func = Function::new("test2".to_string(), "ram".to_string());

        let bb0 = BasicBlock::from_vec(vec![Mnemonic::dummy(0..10)]);
        func.entry_point = Some(func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb0)));

        prog.call_graph.add_vertex(CallTarget::Concrete(Function::new("test".to_string(), "ram".to_string())));
        let vx1 = prog.call_graph.add_vertex(CallTarget::Concrete(func));

        assert_eq!(prog.find_function_by_entry(0), Some(vx1));
        assert_eq!(prog.find_function_by_entry(1), None);
    }

    #[test]
    fn insert_replaces_todo() {
        let uu = Uuid::new_v4();
        let mut prog = Program::new("prog_test");

        let tvx = prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(12), None, uu));
        let vx0 = prog.call_graph.add_vertex(CallTarget::Concrete(Function::new("test".to_string(), "ram".to_string())));
        let vx1 = prog.call_graph.add_vertex(CallTarget::Concrete(Function::new("test2".to_string(), "ram".to_string())));

        let e1 = prog.call_graph.add_edge((), tvx, vx0);
        let e2 = prog.call_graph.add_edge((), vx1, tvx);

        let mut func = Function::with_uuid("test3".to_string(), uu.clone(), "ram".to_string());
        let bb0 = BasicBlock::from_vec(vec![Mnemonic::dummy(12..20)]);
        func.entry_point = Some(func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb0)));
        let uuf = func.uuid.clone();

        let new = prog.insert(CallTarget::Concrete(func));

        assert_eq!(new, vec![]);

        if let Some(&CallTarget::Concrete(ref f)) = prog.call_graph.vertex_label(tvx) {
            assert_eq!(f.uuid, uuf);
            assert!(f.entry_point.is_some());
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

        let mut func = Function::with_uuid("test3".to_string(), uu2.clone(), "ram".to_string());
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
        func.entry_point = Some(func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb0)));
        let uuf = func.uuid.clone();

        let new = prog.insert(CallTarget::Concrete(func));

        assert_eq!(new, vec![]);

        if let Some(&CallTarget::Concrete(ref f)) = prog.call_graph.vertex_label(tvx) {
            assert_eq!(f.uuid, uuf);
            assert!(f.entry_point.is_some());
        }
        assert!(prog.call_graph.vertex_label(tvx).is_some());
        assert_eq!(prog.call_graph.num_edges(), 1);
        assert_eq!(prog.call_graph.num_vertices(), 2);
    }
}
