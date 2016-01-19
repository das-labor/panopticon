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

use value::{Lvalue,Rvalue};
use instr::{Instr,Operation};
use function::{ControlFlowTarget,Function};
use guard::Relation;

use rustc_serialize::{Encodable,Decodable};

/// Models both under- and overapproximation
pub trait Avalue: Clone + PartialEq + Eq + Hash + Debug + Encodable + Decodable {
    fn abstraction(&Rvalue) -> Self;
    fn execute(&Operation<Self>) -> Self;
    fn narrow(&Relation<Self>) -> Self;
    fn combine(&self,&Self) -> Self;
    fn widen(&self,&Self) -> Self;
    fn more_exact(&self,&Self) -> bool;
    fn initial() -> Self;
}

fn approximate<A: Avalue>(func: &Function) -> HashMap<Lvalue,A> {
    let rpo = {
        let mut ret = func.postorder();
        ret.reverse();
        ret
    };
    let mut fixpoint = false;
    let mut ret = HashMap::<Lvalue,A>::new();
    fn res<A: Avalue>(v: &Rvalue, env: &HashMap<Lvalue,A>) -> A {
        if let Some(ref lv) = Lvalue::from_rvalue(v) {
            env.get(lv).unwrap_or(&A::initial()).clone()
        } else {
            A::abstraction(v)
        }
    };

    while !fixpoint {
        fixpoint = true;

        for v in rpo.iter() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(*v) {
                bb.execute(|i| {
                    let (assignee,new) = match i {
                        &Instr{ op: Operation::Phi(ref ops), ref assignee } =>
                            (assignee.clone(),match ops.len() {
                                0 => panic!("Phi function w/o arguments"),
                                1 => A::abstraction(&ops[0]),
                                _ => ops.iter().map(A::abstraction).fold(A::initial(),|acc,x| A::combine(&acc,&x)),
                            }),
                        &Instr{ op: Operation::Nop(ref a), ref assignee } =>
                            (assignee.clone(),A::abstraction(a)),

                        &Instr{ op: Operation::LogicAnd(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::LogicAnd(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::LogicInclusiveOr(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::LogicInclusiveOr(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::LogicExclusiveOr(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::LogicExclusiveOr(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::LogicNegation(ref a), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::LogicNegation(res::<A>(a,&ret)))),
                        &Instr{ op: Operation::LogicLift(ref a), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::LogicLift(res::<A>(a,&ret)))),

                        &Instr{ op: Operation::IntAnd(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntAnd(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntInclusiveOr(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntInclusiveOr(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntExclusiveOr(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntExclusiveOr(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntAdd(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntAdd(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntSubtract(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntSubtract(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntMultiply(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntMultiply(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntDivide(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntDivide(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntModulo(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntModulo(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntLess(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntLess(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntEqual(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntEqual(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntCall(ref a), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntCall(res::<A>(a,&ret)))),
                        &Instr{ op: Operation::IntRightShift(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntRightShift(res::<A>(a,&ret),res::<A>(b,&ret)))),
                        &Instr{ op: Operation::IntLeftShift(ref a,ref b), ref assignee } =>
                            (assignee.clone(),A::execute(&Operation::IntLeftShift(res::<A>(a,&ret),res::<A>(b,&ret)))),
                    };
                    let cur = ret.entry(assignee.clone()).or_insert(A::initial()).clone();

                    if new.more_exact(&cur) {
                        fixpoint = false;
                        ret.insert(assignee,new);
                    }
                });
            }
        }
    }

    ret
}

/*
struct KSet {
    const MAXIMAL_SIZE: usize = 10;
    // None -> Top, Some(vec![]) -> Bot
    val: Option<Vec<Rvalue>>;
}

impl Avalue for KSet {
    fn abstraction(v: &Rvalue) -> Self {
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
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use instr::{Instr,Operation};
    use function::{ControlFlowTarget,Function};
    use guard::Relation;
    use value::{Lvalue,Rvalue};

    use rustc_serialize::{Encodable,Decodable};

    #[derive(Debug,Clone,PartialEq,Eq,Hash,RustcDecodable,RustcEncodable)]
    enum Sign {
        Join,
        Positive,
        Negative,
        Zero,
        Meet
    }

    impl Avalue for Sign {
        fn abstraction(v: &Rvalue) -> Self {
            match v {
                &Rvalue::Constant(c) if c > 0 => Sign::Positive,
                &Rvalue::Constant(c) if c < 0 => Sign::Negative,
                &Rvalue::Constant(0) => Sign::Zero,
                _ => Sign::Join,
            }
        }

        fn execute(op: &Operation<Self>) -> Self {
            match op {
                &Operation::IntAdd(Sign::Positive,Sign::Positive) => Sign::Positive,
                &Operation::IntAdd(Sign::Positive,Sign::Zero) => Sign::Positive,
                &Operation::IntAdd(Sign::Zero,Sign::Positive) => Sign::Positive,
                &Operation::IntAdd(Sign::Negative,Sign::Negative) => Sign::Negative,
                &Operation::IntAdd(Sign::Negative,Sign::Zero) => Sign::Negative,
                &Operation::IntAdd(Sign::Zero,Sign::Negative) => Sign::Negative,
                &Operation::IntAdd(Sign::Positive,Sign::Negative) => Sign::Join,
                &Operation::IntAdd(Sign::Negative,Sign::Positive) => Sign::Join,
                &Operation::IntAdd(_,Sign::Join) => Sign::Join,
                &Operation::IntAdd(Sign::Join,_) => Sign::Join,
                &Operation::IntAdd(ref a,Sign::Meet) => a.clone(),
                &Operation::IntAdd(Sign::Meet,ref b) => b.clone(),

                &Operation::IntSubtract(Sign::Positive,Sign::Positive) => Sign::Join,
                &Operation::IntSubtract(Sign::Positive,Sign::Zero) => Sign::Positive,
                &Operation::IntSubtract(Sign::Zero,Sign::Positive) => Sign::Negative,
                &Operation::IntSubtract(Sign::Negative,Sign::Negative) => Sign::Join,
                &Operation::IntSubtract(Sign::Negative,Sign::Zero) => Sign::Negative,
                &Operation::IntSubtract(Sign::Zero,Sign::Negative) => Sign::Positive,
                &Operation::IntSubtract(Sign::Positive,Sign::Negative) => Sign::Positive,
                &Operation::IntSubtract(Sign::Negative,Sign::Positive) => Sign::Negative,
                &Operation::IntSubtract(_,Sign::Join) => Sign::Join,
                &Operation::IntSubtract(Sign::Join,_) => Sign::Join,
                &Operation::IntSubtract(ref a,Sign::Meet) => a.clone(),
                &Operation::IntSubtract(Sign::Meet,ref b) => b.clone(),

                &Operation::IntMultiply(Sign::Positive,Sign::Positive) => Sign::Positive,
                &Operation::IntMultiply(Sign::Negative,Sign::Negative) => Sign::Positive,
                &Operation::IntMultiply(Sign::Positive,Sign::Negative) => Sign::Negative,
                &Operation::IntMultiply(Sign::Negative,Sign::Positive) => Sign::Negative,
                &Operation::IntMultiply(_,Sign::Zero) => Sign::Zero,
                &Operation::IntMultiply(Sign::Zero,_) => Sign::Zero,
                &Operation::IntMultiply(_,Sign::Join) => Sign::Join,
                &Operation::IntMultiply(Sign::Join,_) => Sign::Join,
                &Operation::IntMultiply(ref a,Sign::Meet) => a.clone(),
                &Operation::IntMultiply(Sign::Meet,ref b) => b.clone(),

                &Operation::IntDivide(Sign::Positive,Sign::Positive) => Sign::Positive,
                &Operation::IntDivide(Sign::Negative,Sign::Negative) => Sign::Positive,
                &Operation::IntDivide(Sign::Positive,Sign::Negative) => Sign::Negative,
                &Operation::IntDivide(Sign::Negative,Sign::Positive) => Sign::Negative,
                &Operation::IntDivide(_,Sign::Zero) => Sign::Zero,
                &Operation::IntDivide(Sign::Zero,_) => Sign::Zero,
                &Operation::IntDivide(_,Sign::Join) => Sign::Join,
                &Operation::IntDivide(Sign::Join,_) => Sign::Join,
                &Operation::IntDivide(ref a,Sign::Meet) => a.clone(),
                &Operation::IntDivide(Sign::Meet,ref b) => b.clone(),

                &Operation::IntModulo(Sign::Positive,Sign::Positive) => Sign::Positive,
                &Operation::IntModulo(Sign::Negative,Sign::Negative) => Sign::Positive,
                &Operation::IntModulo(Sign::Positive,Sign::Negative) => Sign::Negative,
                &Operation::IntModulo(Sign::Negative,Sign::Positive) => Sign::Negative,
                &Operation::IntModulo(_,Sign::Zero) => Sign::Zero,
                &Operation::IntModulo(Sign::Zero,_) => Sign::Zero,
                &Operation::IntModulo(_,Sign::Join) => Sign::Join,
                &Operation::IntModulo(Sign::Join,_) => Sign::Join,
                &Operation::IntModulo(ref a,Sign::Meet) => a.clone(),
                &Operation::IntModulo(Sign::Meet,ref b) => b.clone(),

                _ => Sign::Join,
            }
        }

        fn narrow(_: &Relation<Self>) -> Self {
            Sign::Join
        }

        fn combine(&self, b: &Self) -> Self {
            match (self,b) {
                (x,y) if x == y => x.clone(),
                (&Sign::Meet,x) => x.clone(),
                (x,&Sign::Meet) => x.clone(),
                _ => Sign::Join
            }
        }

        fn widen(&self, b: &Self) -> Self {
            Sign::Join
        }


        fn initial() -> Self {
            Sign::Meet
        }

        fn more_exact(&self, b: &Self) -> bool {
            match (self,b) {
                (&Sign::Meet,&Sign::Positive) | (&Sign::Meet,&Sign::Negative) | (&Sign::Meet,&Sign::Join) => true,
                (&Sign::Positive,&Sign::Join) | (&Sign::Negative,&Sign::Join) => true,
                _ => false
            }
        }
    }

    #[test]
    fn signed() {


    }
}
