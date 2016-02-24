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

use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

use graph_algos::{
    GraphTrait,
    IncidenceGraphTrait,
    VertexListGraphTrait,
};
use rustc_serialize::{Encodable,Decodable};
use graph_algos::dominator::{
    immediate_dominator,
};


use value::{Lvalue,Rvalue};
use instr::{Instr,Operation,execute};
use function::{ControlFlowTarget,Function};
use guard::Relation;
use dataflow::liveness;

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

pub fn approximate<A: Avalue>(func: &Function) -> HashMap<Lvalue,A> {
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
                                1 => res::<A>(&ops[0],&ret),
                                _ => ops.iter().map(|x| res::<A>(x,&ret)).fold(A::initial(),|acc,x| A::combine(&acc,&x)),
                            }),
                        &Instr{ op: Operation::Nop(ref a), ref assignee } =>
                            (assignee.clone(),res::<A>(a,&ret)),

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

                    let cur = ret.get(&assignee).cloned();
                    if cur.is_none() || new.more_exact(&cur.clone().unwrap()) {
                        fixpoint = false;
                        ret.insert(assignee,new);
                    }
                });
            }
        }
    }

    ret
}

pub fn results<A: Avalue>(func: &Function,vals: &HashMap<Lvalue,A>) -> HashMap<(String,u16),A> {
    let cfg = &func.cflow_graph;
    let idom = immediate_dominator(func.entry_point.unwrap(),cfg);
    let mut ret = HashMap::<(String,u16),A>::new();
    let mut names = HashSet::<String>::new();

    for vx in cfg.vertices() {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
            bb.execute(|i| {
                if let Lvalue::Variable{ ref name,.. } = i.assignee {
                    names.insert(name.clone());
                }
            });
        }
    }

    for v in cfg.vertices() {
        if cfg.out_degree(v) == 0 {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(v) {
                for lv in names.iter() {
                    let mut bbv = (bb,v);

                    loop {
                        let mut hit = false;
                        bb.execute_backwards(|i| {
                            if let Lvalue::Variable{ ref name, ref width,.. } = i.assignee {
                                if name == lv {
                                    hit = true;
                                    ret.insert((name.clone(),*width),vals.get(&i.assignee).unwrap_or(&A::initial()).clone());
                                }
                            }
                        });

                        if !hit {
                            if let Some(&w) = idom.get(&bbv.1) {
                                if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(w) {
                                    bbv = (bb,w);
                                    continue;
                                }
                            }
                        }

                        break;
                    }
                }
            }
        }
    }

    ret
}

const KSET_MAXIMAL_CARDINALITY: usize = 10;

#[derive(Debug,Eq,Clone,Hash,RustcDecodable,RustcEncodable)]
pub enum Kset {
    Join,
    Set(Vec<u64>),
    Meet,
}

impl PartialEq for Kset {
    fn eq(&self,other: &Kset) -> bool {
        match (self,other) {
            (&Kset::Meet,&Kset::Meet) => true,
            (&Kset::Set(ref a),&Kset::Set(ref b)) =>
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(a,b)| a == b),
                (&Kset::Join,&Kset::Join) => true,
                _ => false
        }
    }
}

impl Avalue for Kset {
    fn abstraction(v: &Rvalue) -> Self {
        if let &Rvalue::Constant(c) = v {
            Kset::Set(vec![c])
        } else {
            Kset::Join
        }
    }

    fn execute(op: &Operation<Self>) -> Self {
        fn permute(_a: &Kset, _b: &Kset, f: &Fn(Rvalue,Rvalue) -> Rvalue) -> Kset {
            match (_a,_b) {
                (&Kset::Join,_) => Kset::Join,
                (_,&Kset::Join) => Kset::Join,
                (&Kset::Set(ref a),&Kset::Set(ref b)) => {
                    let mut ret = HashSet::<u64>::new();
                    for _x in a.iter() {
                        let x = Rvalue::Constant(*_x);
                        for y in b.iter() {
                            if let Rvalue::Constant(z) = f(x.clone(),Rvalue::Constant(*y)) {
                                ret.insert(z);
                                if ret.len() > KSET_MAXIMAL_CARDINALITY {
                                    return Kset::Join;
                                }
                            }
                        }
                    }

                    if ret.is_empty() {
                        Kset::Meet
                    } else {
                        let mut v = ret.drain().collect::<Vec<u64>>();
                        v.sort();
                        Kset::Set(v)
                    }
                },
                _ => Kset::Meet,
            }
        };
        fn map(_a: &Kset, f: &Fn(Rvalue) -> Rvalue) -> Kset {
            if let &Kset::Set(ref a) = _a {
                let mut s = HashSet::<u64>::from_iter(
                    a.iter().filter_map(|a| {
                        if let Rvalue::Constant(ref c) = f(Rvalue::Constant(*a)) {
                            Some(*c)
                        } else {
                            None
                        }
                    }));

                if s.len() > KSET_MAXIMAL_CARDINALITY {
                    Kset::Join
                } else {
                    let mut v = s.drain().collect::<Vec<_>>();
                    v.sort();
                    Kset::Set(v)
                }
            } else {
                _a.clone()
            }
        };

        match op {
            &Operation::LogicAnd(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::LogicAnd(a,b))),
            &Operation::LogicInclusiveOr(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::LogicInclusiveOr(a,b))),
            &Operation::LogicExclusiveOr(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::LogicExclusiveOr(a,b))),
            &Operation::LogicNegation(ref a) =>
                map(a,&|a| execute(&Operation::LogicNegation(a))),
            &Operation::LogicLift(ref a) =>
                map(a,&|a| execute(&Operation::LogicLift(a))),

            &Operation::IntAnd(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntAnd(a,b))),
            &Operation::IntInclusiveOr(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntInclusiveOr(a,b))),
            &Operation::IntExclusiveOr(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntExclusiveOr(a,b))),
            &Operation::IntAdd(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntAdd(a,b))),
            &Operation::IntSubtract(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntSubtract(a,b))),
            &Operation::IntMultiply(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntMultiply(a,b))),
            &Operation::IntDivide(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntDivide(a,b))),
            &Operation::IntModulo(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntModulo(a,b))),
            &Operation::IntLess(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntLess(a,b))),
            &Operation::IntEqual(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntEqual(a,b))),
            &Operation::IntCall(ref a) =>
                map(a,&|a| execute(&Operation::IntCall(a))),
            &Operation::IntRightShift(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntRightShift(a,b))),
            &Operation::IntLeftShift(ref a,ref b) =>
                permute(a,b,&|a,b| execute(&Operation::IntLeftShift(a,b))),

            &Operation::Phi(ref a) => unreachable!(),
            &Operation::Nop(ref a) =>
                map(a,&|a| execute(&Operation::Nop(a))),
        }
    }

    fn narrow(_: &Relation<Self>) -> Self {
        unimplemented!()
    }

    fn combine(&self,a: &Self) -> Self {
        match (self,a) {
            (&Kset::Join,_) => Kset::Join,
            (_,&Kset::Join) => Kset::Join,
            (a,&Kset::Meet) => a.clone(),
            (&Kset::Meet,b) => b.clone(),
            (&Kset::Set(ref a),&Kset::Set(ref b)) => {
                let mut ret = HashSet::<&u64>::from_iter(a.iter().chain(b.iter()))
                    .iter().cloned().cloned().collect::<Vec<u64>>();
                ret.sort();
                if ret.is_empty() {
                    Kset::Meet
                } else if ret.len() > KSET_MAXIMAL_CARDINALITY {
                    Kset::Join
                } else {
                    Kset::Set(ret)
                }
            },
        }
    }

    fn widen(&self,a: &Self) -> Self {
        Kset::Join
    }

    fn initial() -> Self {
        Kset::Meet
    }

    fn more_exact(&self, a: &Self) -> bool {
        if self == a {
            false
        } else {
            match (self,a) {
                (&Kset::Join,_) => true,
                (_,&Kset::Meet) => true,
                (&Kset::Set(ref a),&Kset::Set(ref b)) =>
                    HashSet::<&u64>::from_iter(a.iter())
                    .is_superset(&HashSet::from_iter(b.iter())),
                    _ => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instr::{Instr,Operation};
    use function::{ControlFlowTarget,Function,ControlFlowGraph};
    use guard::{Guard,Relation};
    use value::{Lvalue,Rvalue};
    use mnemonic::{Bound,Mnemonic};
    use dataflow::ssa_convertion;
    use basic_block::BasicBlock;

    use graph_algos::{
        MutableGraphTrait,
        GraphTrait,
    };

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
            self != b && match (self,b) {
                (&Sign::Meet,&Sign::Positive) | (&Sign::Meet,&Sign::Negative) | (&Sign::Meet,&Sign::Join) => false,
                (&Sign::Positive,&Sign::Join) | (&Sign::Negative,&Sign::Join) => false,
                _ => true
            }
        }
    }

    #[test]
    fn signedness_analysis() {
        let x_var = Lvalue::Variable{ name: "x".to_string(), width: 32, subscript: None };
        let n_var = Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let bb0 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(0..1,"assign x".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Instr{ op: Operation::Nop(Rvalue::Constant(0)), assignee: x_var.clone()}].iter()),
                                                     Mnemonic::new(1..2,"assign n".to_string(),"".to_string(),vec![].iter(),vec![
                                                                   Instr{ op: Operation::Nop(Rvalue::Constant(1)), assignee: n_var.clone()}].iter())]);
        let bb1 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(2..3,"add x and n".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Instr{ op: Operation::IntAdd(x_var.to_rv(),n_var.to_rv()), assignee: x_var.clone()}].iter()),
                                                     Mnemonic::new(3..4,"inc n".to_string(),"".to_string(),vec![].iter(),vec![
                                                                   Instr{ op: Operation::IntAdd(n_var.to_rv(),Rvalue::Constant(1)), assignee: n_var.clone()}].iter())]);
        let bb2 = BasicBlock{ area: Bound::new(4,5), mnemonics: vec![] };
        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));

        cfg.add_edge(Guard::sign_leq(&n_var.to_rv(),&Rvalue::Undefined),v0,v1);
        cfg.add_edge(Guard::sign_leq(&n_var.to_rv(),&Rvalue::Undefined),v1,v1);
        cfg.add_edge(Guard::sign_gt(&n_var.to_rv(),&Rvalue::Undefined),v0,v2);
        cfg.add_edge(Guard::sign_gt(&n_var.to_rv(),&Rvalue::Undefined),v1,v2);

        let mut func = Function::new("func".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        ssa_convertion(&mut func);

        let vals = approximate::<Sign>(&func);
        assert_eq!(vals.get(&Lvalue::Variable{ name: "x".to_string(), width: 32, subscript: Some(1) }), Some(&Sign::Zero));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: Some(1) }), Some(&Sign::Positive));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "x".to_string(), width: 32, subscript: Some(2) }), Some(&Sign::Join));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: Some(2) }), Some(&Sign::Positive));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "x".to_string(), width: 32, subscript: Some(3) }), Some(&Sign::Join));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: Some(3) }), Some(&Sign::Positive));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "x".to_string(), width: 32, subscript: Some(4) }), Some(&Sign::Join));
        assert_eq!(vals.get(&Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: Some(4) }), Some(&Sign::Positive));
    }

    /*
     * a = 10
     * b = 0
     * c = undef
     * if (c == 1) {
     *   a += 5;
     *   b = a * c;
     *   c = 2
     * } else {
     *   while(a > 0) {
     *     a -= 1
     *     b += 3
     *     c = 3
     *   }
     * }
     * x = a + b;
     */
    #[test]
    fn kset_test() {
        let a_var = Lvalue::Variable{ name: "a".to_string(), width: 32, subscript: None };
        let b_var = Lvalue::Variable{ name: "b".to_string(), width: 32, subscript: None };
        let c_var = Lvalue::Variable{ name: "c".to_string(), width: 32, subscript: None };
        let x_var = Lvalue::Variable{ name: "x".to_string(), width: 32, subscript: None };
        let bb0 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(0..1,"assign a".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Instr{ op: Operation::Nop(Rvalue::Constant(10)), assignee: a_var.clone()}].iter()),
                                                     Mnemonic::new(1..2,"assign b".to_string(),"".to_string(),vec![].iter(),vec![
                                                                   Instr{ op: Operation::Nop(Rvalue::Constant(0)), assignee: b_var.clone()}].iter()),
                                                                   Mnemonic::new(2..3,"assign c".to_string(),"".to_string(),vec![].iter(),vec![
                                                                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c_var.clone()}].iter())]);
        let bb1 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(3..4,"add a and 5".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Instr{ op: Operation::IntAdd(a_var.to_rv(),Rvalue::Constant(5)), assignee: a_var.clone()}].iter()),
                                                     Mnemonic::new(4..5,"mul a and c".to_string(),"".to_string(),vec![].iter(),vec![
                                                                   Instr{ op: Operation::IntAdd(a_var.to_rv(),c_var.to_rv()), assignee: b_var.clone()}].iter()),
                                                                   Mnemonic::new(4..5,"assign c".to_string(),"".to_string(),vec![].iter(),vec![
                                                                                 Instr{ op: Operation::Nop(Rvalue::Constant(2)), assignee: c_var.clone()}].iter())]);
        let bb2 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(5..6,"dec a".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Instr{ op: Operation::IntSubtract(a_var.to_rv(),Rvalue::Constant(1)), assignee: a_var.clone()}].iter()),
                                                     Mnemonic::new(6..7,"add 3 to b".to_string(),"".to_string(),vec![].iter(),vec![
                                                                   Instr{ op: Operation::IntAdd(b_var.to_rv(),Rvalue::Constant(3)), assignee: b_var.clone()}].iter()),
                                                                   Mnemonic::new(8..9,"assign c".to_string(),"".to_string(),vec![].iter(),vec![
                                                                                 Instr{ op: Operation::Nop(Rvalue::Constant(3)), assignee: c_var.clone()}].iter())]);
        let bb3 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(7..8,"add a and b".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Instr{ op: Operation::IntAdd(a_var.to_rv(),b_var.to_rv()), assignee: x_var.clone()}].iter())]);
        let bb4 = BasicBlock{ area: Bound::new(8,9), mnemonics: vec![] };

        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));

        cfg.add_edge(Guard::eq(&c_var.to_rv(),&Rvalue::Constant(1)),v0,v1);
        cfg.add_edge(Guard::neq(&c_var.to_rv(),&Rvalue::Constant(1)),v0,v4);
        cfg.add_edge(Guard::sign_gt(&a_var.to_rv(),&Rvalue::Constant(0)),v4,v2);
        cfg.add_edge(Guard::sign_leq(&a_var.to_rv(),&Rvalue::Constant(0)),v4,v3);
        cfg.add_edge(Guard::always(),v2,v4);
        cfg.add_edge(Guard::always(),v1,v3);

        let mut func = Function::new("func".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        ssa_convertion(&mut func);

        let vals = approximate::<Kset>(&func);
        let res = results::<Kset>(&func,&vals);

        assert_eq!(res[&("a".to_string(),32)],Kset::Join);
        assert_eq!(res[&("b".to_string(),32)],Kset::Join);
        assert_eq!(res[&("c".to_string(),32)],Kset::Set(vec![2,3]));
        assert_eq!(res[&("x".to_string(),32)],Kset::Join);
    }

}
