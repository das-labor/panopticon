/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
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

use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashSet,HashMap};

use graph_algos::{GraphTrait};

use value::Rvalue;
use instr::{Instr,Operation};
use function::{ControlFlowTarget,Function};

/// Models both under- and overapproximation
trait AbstractDomain {
    type Value: Clone + PartialEq + Hash + Debug;

    fn abstraction(&Rvalue) -> Self::Value;
    fn execute(&Operation,&HashMap<Rvalue,Self::Value>) -> Self::Value;
    fn combine(&Self::Value,&Self::Value) -> Self::Value;
    fn widen(&Self::Value,&Self::Value) -> Self::Value;
    fn more_exact(&Self::Value,&Self::Value) -> bool;
    fn initial() -> Self::Value;
}

fn approximate<A: AbstractDomain>(func: &Function) -> HashMap<Rvalue,A::Value> {
    let rpo = {
        let mut ret = func.postorder();
        ret.reverse();
        ret
    };
    let mut fixpoint = false;
    let mut ret = HashMap::<Rvalue,A::Value>::new();

    while !fixpoint {
        fixpoint = true;

        for v in rpo.iter() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(*v) {
                bb.execute(|i| {
                    let (assignee,new) = match i {
                        &Instr{ op: Operation::Phi(ref ops), ref assignee } =>
                            (assignee.to_rv(),match ops.len() {
                                0 => panic!("Phi function w/o arguments"),
                                1 => A::abstraction(&ops[0]),
                                _ => ops.iter().map(A::abstraction).fold(A::initial(),|acc,x| A::combine(&acc,&x)),
                            }),
                        &Instr{ ref op, ref assignee } => (assignee.to_rv(),A::execute(op,&ret)),
                    };
                    let cur = ret.entry(assignee.clone()).or_insert(A::initial()).clone();

                    if A::more_exact(&new,&cur) {
                        fixpoint = false;
                        ret.insert(assignee,new);
                    }
                });
            }
        }
    }

    ret
}

struct KSet;

impl AbstractDomain for KSet {
    // None -> Top, Some(vec![]) -> Bot
    type Value = Option<Vec<Rvalue>>;

    fn abstraction(v: &Rvalue) -> Self::Value {
        Some(vec![v.clone()])
    }

    fn execute(op: &Operation, env: &HashMap<Rvalue,Self::Value>) -> Self::Value {
        Some(vec![])
    }

    fn combine(a: &Self::Value, b: &Self::Value) -> Self::Value {
        unimplemented!()
    }

    fn widen(a: &Self::Value, b: &Self::Value) -> Self::Value {
        unimplemented!()
    }

    fn initial() -> Self::Value {
        Some(vec![])
    }

    fn more_exact(_: &Self::Value, _: &Self::Value) -> bool {
        unimplemented!()
    }
}
