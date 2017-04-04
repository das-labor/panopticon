// Panopticon - A libre disassembler
// Copyright (C) 2014,2015,2016 Kai Michaelis
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

//! Abstract Interpretation Framework.
//!
//! Abstract Interpretation executes an program over sets of concrete values. Each operation is
//! extended to work on some kind of abstract value called abstract domain that approximates a
//! set of concrete values (called the concrete domain). A simple example is the domain of signs.
//! Each value can either be positive, negative, both or none. An abstract interpretation will first
//! replace all constant values with their signs and then execute all basic blocks using the
//! abstract sign domain. For example multiplying two positive values yields a positive value.
//! Adding a positive and a negative sign yields an abstract value representing both signs (called
//! join).

use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::borrow::Cow;
use std::cmp::max;

use graph_algos::{GraphTrait, IncidenceGraphTrait, VertexListGraphTrait, BidirectionalGraphTrait};
use rustc_serialize::{Encodable, Decodable};
use graph_algos::dominator::immediate_dominator;
use graph_algos::order::{weak_topo_order, HierarchicalOrdering};

use {Lvalue, Rvalue, Statement, Operation, execute, ControlFlowTarget, ControlFlowRef, ControlFlowGraph, Function, Guard, lift, Result, flag_operations};

/// Linear constraint.
pub enum Constraint {
    /// True if equal to.
    Equal(Rvalue),
    /// True if less than (unsigned).
    LessUnsigned(Rvalue),
    /// True if less than or equal to (unsigned).
    LessOrEqualUnsigned(Rvalue),
    /// True if less than (signed).
    LessSigned(Rvalue),
    /// True if less than or equal to (signed).
    LessOrEqualSigned(Rvalue),
}

/// A program point is a unique RREIL instruction inside a function.
#[derive(Debug,PartialEq,Eq,Clone,RustcDecodable,RustcEncodable,PartialOrd,Ord,Hash)]
pub struct ProgramPoint {
    address: u64,
    position: usize,
}

/// Abstract Domain. Models both under- and over-approximation.
pub trait Avalue: Clone + PartialEq + Eq + Hash + Debug + Encodable + Decodable {
    /// Alpha function. Returns domain element that approximates the concrete value the best
    fn abstract_value(&Rvalue) -> Self;
    /// Alpha function. Returns domain element that approximates the concrete value that fullfil
    /// the constraint the best.
    fn abstract_constraint(&Constraint) -> Self;
    /// Execute the abstract version of the operation, yielding the result.
    fn execute(&ProgramPoint, &Operation<Self>) -> Self;
    /// Narrows `self` with the argument.
    fn narrow(&self, &Self) -> Self;
    /// Widens `self` with the argument.
    fn widen(&self, other: &Self) -> Self;
    /// Computes the lowest upper bound of self and the argument.
    fn combine(&self, &Self) -> Self;
    /// Returns true if `other` <= `self`.
    fn more_exact(&self, other: &Self) -> bool;
    /// Returns the meet of the domain
    fn initial() -> Self;
    /// Mimics the Select operation.
    fn extract(&self, size: usize, offset: usize) -> Self;
}

/// Does an abstract interpretation of `func` using the abstract domain `A`. The function uses a
/// fixed point iteration and the widening strategy outlined in
/// Bourdoncle: "Efficient chaotic iteration strategies with widenings".
pub fn approximate<A: Avalue>(func: &Function) -> Result<HashMap<Lvalue, A>> {
    if func.entry_point.is_none() {
        return Err("function has no entry point".into());
    }

    let wto = weak_topo_order(func.entry_point.unwrap(), &func.cflow_graph);
    let edge_ops = flag_operations(func);
    fn stabilize<A: Avalue>(h: &Vec<Box<HierarchicalOrdering<ControlFlowRef>>>, graph: &ControlFlowGraph, constr: &HashMap<Lvalue, A>, sizes: &HashMap<Cow<'static, str>, usize>, ret: &mut HashMap<(Cow<'static, str>, usize), A>) -> Result<()> {
        let mut stable = true;
        let mut iter_cnt = 0;
        let head = if let Some(h) = h.first() {
            match &**h {
                &HierarchicalOrdering::Element(ref vx) => vx.clone(),
                &HierarchicalOrdering::Component(ref vec) => return stabilize(vec, graph, constr, sizes, ret),
            }
        } else {
            return Ok(());
        };

        loop {
            for x in h.iter() {
                match &**x {
                    &HierarchicalOrdering::Element(ref vx) => stable &= !try!(execute(*vx, iter_cnt >= 2 && *vx == head, graph, constr, sizes, ret)),
                    &HierarchicalOrdering::Component(ref vec) => {
                        try!(stabilize(&*vec, graph, constr, sizes, ret));
                        stable = true;
                    }
                }
            }

            if stable {
                for (lv, a) in constr.iter() {
                    if let &Lvalue::Variable { ref name, subscript: Some(ref subscript), .. } = lv {
                        if let Some(ref mut x) = ret.get_mut(&(name.clone(), *subscript)) {
                            let n = x.narrow(&a);
                            **x = n;
                        }
                    }
                }

                // execute(*vx,do_widen && vx == head,graph,ret),
                return Ok(());
            }

            stable = true;
            iter_cnt += 1;
        }
    }
    fn execute<A: Avalue>(t: ControlFlowRef, do_widen: bool, graph: &ControlFlowGraph, _: &HashMap<Lvalue, A>, sizes: &HashMap<Cow<'static, str>, usize>, ret: &mut HashMap<(Cow<'static, str>, usize), A>) -> Result<bool> {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = graph.vertex_label(t) {
            let mut change = false;
            let mut pos = 0usize;
            bb.execute(|i| {
                if let Statement { ref op, assignee: Lvalue::Variable { ref name, subscript: Some(ref subscript), .. } } = *i {
                    let pp = ProgramPoint {
                        address: bb.area.start,
                        position: pos,
                    };
                    let new = A::execute(&pp, &lift(op, &|x| res::<A>(x, sizes, &ret)));
                    let assignee = (name.clone(), *subscript);
                    let cur = ret.get(&assignee).cloned();

                    if cur.is_none() {
                        change = true;
                        ret.insert(assignee, new);
                    } else {
                        if do_widen {
                            let c = cur.unwrap();
                            let w = c.widen(&new);

                            if w != c {
                                change = true;
                                ret.insert(assignee, w);
                            }
                        } else if new.more_exact(&cur.clone().unwrap()) {
                            change = true;
                            ret.insert(assignee, new);
                        }
                    }
                }

                pos += 1;
            });

            Ok(change)
        } else {
            Ok(false)
        }
    }
    fn res<A: Avalue>(v: &Rvalue, sizes: &HashMap<Cow<'static, str>, usize>, env: &HashMap<(Cow<'static, str>, usize), A>) -> A {
        if let &Rvalue::Variable { ref name, subscript: Some(ref subscript), ref size, ref offset } = v {
            let nam = (name.clone(), *subscript);
            let t = env.get(&nam).unwrap_or(&A::initial()).clone();

            if *offset > 0 || *size != *sizes.get(&nam.0).unwrap_or(&0) {
                t.extract(*size, *offset)
            } else {
                t
            }
        } else {
            A::abstract_value(v)
        }
    };
    let mut ret = HashMap::<(Cow<'static, str>, usize), A>::new();
    let mut sizes = HashMap::<Cow<'static, str>, usize>::new();
    let mut constr = HashMap::<Lvalue, A>::new();

    for vx in func.cflow_graph.vertices() {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
            bb.execute(|i| {
                if let Lvalue::Variable { ref name, ref size, .. } = i.assignee {
                    let t = *size;
                    let s = *sizes.get(name).unwrap_or(&t);
                    sizes.insert(name.clone(), max(s, t));
                }
            });
        }
    }

    for vx in func.cflow_graph.vertices() {
        for e in func.cflow_graph.in_edges(vx) {
            if let Some(&Guard::Predicate { .. }) = func.cflow_graph.edge_label(e) {
                match edge_ops.get(&e).cloned() {
                    Some(Operation::Equal(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(Lvalue::from_rvalue(right).unwrap(),
                                      A::abstract_constraint(&Constraint::Equal(left.clone())));
                    }
                    Some(Operation::Equal(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(Lvalue::from_rvalue(left).unwrap(),
                                      A::abstract_constraint(&Constraint::Equal(right.clone())));
                    }
                    Some(Operation::LessUnsigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(Lvalue::from_rvalue(right).unwrap(),
                                      A::abstract_constraint(&Constraint::LessUnsigned(left.clone())));
                    }
                    Some(Operation::LessUnsigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(Lvalue::from_rvalue(left).unwrap(),
                                      A::abstract_constraint(&Constraint::LessUnsigned(right.clone())));
                    }
                    Some(Operation::LessSigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(Lvalue::from_rvalue(right).unwrap(),
                                      A::abstract_constraint(&Constraint::LessSigned(left.clone())));
                    }
                    Some(Operation::LessSigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(Lvalue::from_rvalue(left).unwrap(),
                                      A::abstract_constraint(&Constraint::LessSigned(right.clone())));
                    }
                    Some(Operation::LessOrEqualUnsigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(Lvalue::from_rvalue(right).unwrap(),
                                      A::abstract_constraint(&Constraint::LessOrEqualUnsigned(left.clone())));
                    }
                    Some(Operation::LessOrEqualUnsigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(Lvalue::from_rvalue(left).unwrap(),
                                      A::abstract_constraint(&Constraint::LessOrEqualUnsigned(right.clone())));
                    }
                    Some(Operation::LessOrEqualSigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(Lvalue::from_rvalue(right).unwrap(),
                                      A::abstract_constraint(&Constraint::LessOrEqualSigned(left.clone())));
                    }
                    Some(Operation::LessOrEqualSigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(Lvalue::from_rvalue(left).unwrap(),
                                      A::abstract_constraint(&Constraint::LessOrEqualSigned(right.clone())));
                    }
                    _ => {}
                }
            }
        }
    }

    match wto {
        HierarchicalOrdering::Component(ref v) => {
            try!(stabilize(v, &func.cflow_graph, &constr, &sizes, &mut ret));
        }
        HierarchicalOrdering::Element(ref v) => {
            try!(execute(*v, false, &func.cflow_graph, &constr, &sizes, &mut ret));
        }
    }

    Ok(HashMap::from_iter(ret.iter().filter_map(|(&(ref name, ref subscript), val)| {
        if let Some(sz) = sizes.get(name) {
            Some((Lvalue::Variable {
                      name: name.clone(),
                      subscript: Some(*subscript),
                      size: *sz,
                  },
                  val.clone()))
        } else {
            None
        }
    })))
}

/// Given a function and an abstract interpretation result this functions returns that variable
/// names and abstract values that live after the function returns.
pub fn results<A: Avalue>(func: &Function, vals: &HashMap<Lvalue, A>) -> HashMap<(Cow<'static, str>, usize), A> {
    let cfg = &func.cflow_graph;
    let idom = immediate_dominator(func.entry_point.unwrap(), cfg);
    let mut ret = HashMap::<(Cow<'static, str>, usize), A>::new();
    let mut names = HashSet::<Cow<'static, str>>::new();

    for vx in cfg.vertices() {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
            bb.execute(|i| {
                if let Lvalue::Variable { ref name, .. } = i.assignee {
                    names.insert(name.clone());
                }
            });
        }
    }

    for v in cfg.vertices() {
        if cfg.out_degree(v) == 0 {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(v) {
                for lv in names.iter() {
                    let mut bbv = (bb, v);

                    loop {
                        let mut hit = false;
                        bb.execute_backwards(|i| {
                            if let Lvalue::Variable { ref name, ref size, .. } = i.assignee {
                                if name == lv {
                                    hit = true;
                                    ret.insert((name.clone(), *size),
                                               vals.get(&i.assignee)
                                                   .unwrap_or(&A::initial())
                                                   .clone());
                                }
                            }
                        });

                        if !hit {
                            let next_bb = idom.get(&bbv.1).cloned();
                            let fixpoint = {
                                next_bb == Some(bbv.1)
                            };

                            if !fixpoint {
                                if let Some(w) = next_bb {
                                    if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(w) {
                                        bbv = (bb, w);
                                        continue;
                                    }
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

/// Largest Kset cardinality before Join.
const KSET_MAXIMAL_CARDINALITY: usize = 10;

/// Kindler et.al style Kset domain. Domain elements are sets of concrete values. Sets have a
/// maximum cardinality. Every set larger than that is equal the lattice join. The partial order is
/// set inclusion.
#[derive(Debug,Eq,Clone,Hash,RustcDecodable,RustcEncodable)]
pub enum Kset {
    /// Lattice join. Sets larger than `KSET_MAXIMAL_CARDINALITY`.
    Join,
    /// Set of concrete values and their size in bits. The set is never empty and never larger than
    /// `KSET_MAXIMAL_CARDINALITY`.
    Set(Vec<(u64, usize)>),
    /// Lattice meet, equal to the empty set.
    Meet,
}

impl PartialEq for Kset {
    fn eq(&self, other: &Kset) -> bool {
        match (self, other) {
            (&Kset::Meet, &Kset::Meet) => true,
            (&Kset::Set(ref a), &Kset::Set(ref b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b),
            (&Kset::Join, &Kset::Join) => true,
            _ => false,
        }
    }
}

impl Avalue for Kset {
    fn abstract_value(v: &Rvalue) -> Self {
        if let &Rvalue::Constant { ref value, ref size } = v {
            Kset::Set(vec![(if *size < 64 {
                                *value % (1u64 << *size)
                            } else {
                                *value
                            },
                            *size)])
        } else {
            Kset::Join
        }
    }

    fn abstract_constraint(constr: &Constraint) -> Self {
        if let &Constraint::Equal(Rvalue::Constant { ref value, ref size }) = constr {
            Kset::Set(vec![(if *size < 64 {
                                *value % (1u64 << *size)
                            } else {
                                *value
                            },
                            *size)])
        } else {
            Kset::Join
        }
    }

    fn execute(_: &ProgramPoint, op: &Operation<Self>) -> Self {
        fn permute(_a: &Kset, _b: &Kset, f: &Fn(Rvalue, Rvalue) -> Rvalue) -> Kset {
            match (_a, _b) {
                (&Kset::Join, _) => Kset::Join,
                (_, &Kset::Join) => Kset::Join,
                (&Kset::Set(ref a), &Kset::Set(ref b)) => {
                    let mut ret = HashSet::<(u64, usize)>::new();
                    for &(_x, _xs) in a.iter() {
                        let x = Rvalue::Constant {
                            value: _x,
                            size: _xs,
                        };
                        for &(_y, _ys) in b.iter() {
                            let y = Rvalue::Constant {
                                value: _y,
                                size: _ys,
                            };
                            if let Rvalue::Constant { value, size } = f(x.clone(), y) {
                                ret.insert((value, size));
                                if ret.len() > KSET_MAXIMAL_CARDINALITY {
                                    return Kset::Join;
                                }
                            }
                        }
                    }

                    if ret.is_empty() {
                        Kset::Meet
                    } else {
                        let mut v = ret.drain().collect::<Vec<(u64, usize)>>();
                        v.sort();
                        Kset::Set(v)
                    }
                }
                _ => Kset::Meet,
            }
        };
        fn map(_a: &Kset, f: &Fn(Rvalue) -> Rvalue) -> Kset {
            if let &Kset::Set(ref a) = _a {
                let mut s = HashSet::<(u64, usize)>::from_iter(a.iter()
                    .filter_map(|&(a, _as)| {
                        if let Rvalue::Constant { value, size } =
                            f(Rvalue::Constant {
                                value: a,
                                size: _as,
                            }) {
                            Some((value, size))
                        } else {
                            None
                        }
                    }));

                if s.len() > KSET_MAXIMAL_CARDINALITY {
                    Kset::Join
                } else if s.is_empty() {
                    Kset::Meet
                } else {
                    let mut v = s.drain().collect::<Vec<(_, _)>>();
                    v.sort();
                    Kset::Set(v)
                }
            } else {
                _a.clone()
            }
        };

        match *op {
            Operation::And(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::And(a, b))),
            Operation::InclusiveOr(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::InclusiveOr(a, b))),
            Operation::ExclusiveOr(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::ExclusiveOr(a, b))),
            Operation::Add(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::Add(a, b))),
            Operation::Subtract(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::Subtract(a, b))),
            Operation::Multiply(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::Multiply(a, b))),
            Operation::DivideSigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::DivideSigned(a, b))),
            Operation::DivideUnsigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::DivideUnsigned(a, b))),
            Operation::Modulo(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::Modulo(a, b))),
            Operation::ShiftRightSigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::ShiftRightSigned(a, b))),
            Operation::ShiftRightUnsigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::ShiftRightUnsigned(a, b))),
            Operation::ShiftLeft(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::ShiftLeft(a, b))),

            Operation::LessOrEqualSigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::LessOrEqualSigned(a, b))),
            Operation::LessOrEqualUnsigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::LessOrEqualUnsigned(a, b))),
            Operation::LessSigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::LessSigned(a, b))),
            Operation::LessUnsigned(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::LessUnsigned(a, b))),
            Operation::Equal(ref a, ref b) => permute(a, b, &|a, b| execute(Operation::Equal(a, b))),

            Operation::Move(ref a) => map(a, &|a| execute(Operation::Move(a))),
            Operation::Call(ref a) => map(a, &|a| execute(Operation::Call(a))),
            Operation::ZeroExtend(ref sz, ref a) => map(a, &|a| execute(Operation::ZeroExtend(*sz, a))),
            Operation::SignExtend(ref sz, ref a) => map(a, &|a| execute(Operation::SignExtend(*sz, a))),
            Operation::Select(ref off, ref a, ref b) => permute(a, b, &|a, b| execute(Operation::Select(*off, a, b))),

            Operation::Load(ref r, ref a) => map(a, &|a| execute(Operation::Load(r.clone(), a))),
            Operation::Store(ref r, ref a) => map(a, &|a| execute(Operation::Store(r.clone(), a))),

            Operation::Phi(ref ops) => {
                match ops.len() {
                    0 => unreachable!("Phi function w/o arguments"),
                    1 => ops[0].clone(),
                    _ => ops.iter().fold(Kset::Meet, |acc, x| acc.combine(&x)),
                }
            }
        }
    }

    fn narrow(&self, a: &Self) -> Self {
        match a {
            &Kset::Meet => Kset::Meet,
            &Kset::Join => self.clone(),
            &Kset::Set(ref v) => {
                match self {
                    &Kset::Meet => Kset::Meet,
                    &Kset::Join => Kset::Set(v.clone()),
                    &Kset::Set(ref w) => {
                        let set = HashSet::<&(u64, usize)>::from_iter(v.iter());
                        Kset::Set(w.iter().filter(|x| set.contains(x)).cloned().collect::<Vec<_>>())
                    }
                }
            }
        }
    }

    fn combine(&self, a: &Self) -> Self {
        match (self, a) {
            (&Kset::Join, _) => Kset::Join,
            (_, &Kset::Join) => Kset::Join,
            (a, &Kset::Meet) => a.clone(),
            (&Kset::Meet, b) => b.clone(),
            (&Kset::Set(ref a), &Kset::Set(ref b)) => {
                let mut ret = HashSet::<&(u64, usize)>::from_iter(a.iter().chain(b.iter()))
                    .iter()
                    .cloned()
                    .cloned()
                    .collect::<Vec<(u64, usize)>>();
                ret.sort();
                if ret.is_empty() {
                    Kset::Meet
                } else if ret.len() > KSET_MAXIMAL_CARDINALITY {
                    Kset::Join
                } else {
                    Kset::Set(ret)
                }
            }
        }
    }

    fn widen(&self, s: &Self) -> Self {
        s.clone()
    }

    fn initial() -> Self {
        Kset::Meet
    }

    fn more_exact(&self, a: &Self) -> bool {
        if self == a {
            false
        } else {
            match (self, a) {
                (&Kset::Join, _) => true,
                (_, &Kset::Meet) => true,
                (&Kset::Set(ref a), &Kset::Set(ref b)) => HashSet::<&(u64, usize)>::from_iter(a.iter()).is_superset(&HashSet::from_iter(b.iter())),
                _ => false,
            }
        }
    }

    fn extract(&self, size: usize, offset: usize) -> Self {
        match self {
            &Kset::Join => Kset::Join,
            &Kset::Meet => Kset::Meet,
            &Kset::Set(ref v) => {
                Kset::Set(v.iter()
                    .map(|&(v, _)| ((v >> offset) % (1 << (size - 1)), size))
                    .collect::<Vec<_>>())
            }
        }
    }
}

/// Mihaila et.al. Widening Point inferring cofibered domain. This domain is parameterized with a
/// child domain.
#[derive(Debug,PartialEq,Eq,Clone,Hash,RustcDecodable,RustcEncodable)]
pub struct Widening<A: Avalue> {
    value: A,
    point: Option<ProgramPoint>,
}

impl<A: Avalue> Avalue for Widening<A> {
    fn abstract_value(v: &Rvalue) -> Self {
        Widening {
            value: A::abstract_value(v),
            point: None,
        }
    }

    fn abstract_constraint(c: &Constraint) -> Self {
        Widening {
            value: A::abstract_constraint(c),
            point: None,
        }
    }

    fn execute(pp: &ProgramPoint, op: &Operation<Self>) -> Self {
        match op {
            &Operation::Phi(ref ops) => {
                let widen = ops.iter().map(|x| x.point.clone().unwrap_or(pp.clone())).max() > Some(pp.clone());

                Widening {
                    value: match ops.len() {
                        0 => unreachable!("Phi function w/o arguments"),
                        1 => ops[0].value.clone(),
                        _ => {
                            ops.iter().map(|x| x.value.clone()).fold(A::initial(), |acc, x| {
                                if widen {
                                    acc.widen(&x)
                                } else {
                                    acc.combine(&x)
                                }
                            })
                        }
                    },
                    point: Some(pp.clone()),
                }
            }
            _ => {
                Widening {
                    value: A::execute(pp, &lift(op, &|x| x.value.clone())),
                    point: Some(pp.clone()),
                }
            }
        }
    }

    fn widen(&self, s: &Self) -> Self {
        Widening {
            value: self.value.widen(&s.value),
            point: self.point.clone(),
        }
    }

    fn combine(&self, s: &Self) -> Self {
        Widening {
            value: self.value.combine(&s.value),
            point: self.point.clone(),
        }
    }

    fn narrow(&self, _: &Self) -> Self {
        self.clone()
    }

    fn initial() -> Self {
        Widening {
            value: A::initial(),
            point: None,
        }
    }

    fn more_exact(&self, a: &Self) -> bool {
        self.value.more_exact(&a.value)
    }

    fn extract(&self, size: usize, offset: usize) -> Self {
        Widening {
            value: self.value.extract(size, offset),
            point: self.point.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Statement, Operation, ControlFlowTarget, Function, ControlFlowGraph, Guard, Lvalue, Rvalue, Bound, Mnemonic, ssa_convertion, BasicBlock};

    use graph_algos::MutableGraphTrait;
    use std::borrow::Cow;

    #[derive(Debug,Clone,PartialEq,Eq,Hash,RustcDecodable,RustcEncodable)]
    enum Sign {
        Join,
        Positive,
        Negative,
        Zero,
        Meet,
    }

    impl Avalue for Sign {
        fn abstract_value(v: &Rvalue) -> Self {
            match v {
                &Rvalue::Constant { value: c, .. } if c > 0 => Sign::Positive,
                &Rvalue::Constant { value: 0, .. } => Sign::Zero,
                _ => Sign::Join,
            }
        }

        fn abstract_constraint(c: &Constraint) -> Self {
            match c {
                &Constraint::Equal(Rvalue::Constant { value: 0, .. }) => Sign::Zero,
                &Constraint::LessUnsigned(Rvalue::Constant { value: 1, .. }) => Sign::Zero,
                &Constraint::LessOrEqualUnsigned(Rvalue::Constant { value: 0, .. }) => Sign::Zero,
                &Constraint::LessSigned(Rvalue::Constant { value: 0, .. }) => Sign::Negative,
                &Constraint::LessOrEqualSigned(Rvalue::Constant { value: v, size: s }) if s <= 64 && v & (1 << (s - 1)) != 0 => Sign::Negative,
                &Constraint::LessSigned(Rvalue::Constant { value: v, size: s }) if s <= 64 && v & (1 << (s - 1)) != 0 => Sign::Negative,
                _ => Sign::Join,
            }
        }

        fn execute(_: &ProgramPoint, op: &Operation<Self>) -> Self {
            match op {
                &Operation::Add(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Operation::Add(Sign::Positive, Sign::Zero) => Sign::Positive,
                &Operation::Add(Sign::Zero, Sign::Positive) => Sign::Positive,
                &Operation::Add(Sign::Negative, Sign::Negative) => Sign::Negative,
                &Operation::Add(Sign::Negative, Sign::Zero) => Sign::Negative,
                &Operation::Add(Sign::Zero, Sign::Negative) => Sign::Negative,
                &Operation::Add(Sign::Positive, Sign::Negative) => Sign::Join,
                &Operation::Add(Sign::Negative, Sign::Positive) => Sign::Join,
                &Operation::Add(_, Sign::Join) => Sign::Join,
                &Operation::Add(Sign::Join, _) => Sign::Join,
                &Operation::Add(ref a, Sign::Meet) => a.clone(),
                &Operation::Add(Sign::Meet, ref b) => b.clone(),

                &Operation::Subtract(Sign::Positive, Sign::Positive) => Sign::Join,
                &Operation::Subtract(Sign::Positive, Sign::Zero) => Sign::Positive,
                &Operation::Subtract(Sign::Zero, Sign::Positive) => Sign::Negative,
                &Operation::Subtract(Sign::Negative, Sign::Negative) => Sign::Join,
                &Operation::Subtract(Sign::Negative, Sign::Zero) => Sign::Negative,
                &Operation::Subtract(Sign::Zero, Sign::Negative) => Sign::Positive,
                &Operation::Subtract(Sign::Positive, Sign::Negative) => Sign::Positive,
                &Operation::Subtract(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Operation::Subtract(_, Sign::Join) => Sign::Join,
                &Operation::Subtract(Sign::Join, _) => Sign::Join,
                &Operation::Subtract(ref a, Sign::Meet) => a.clone(),
                &Operation::Subtract(Sign::Meet, ref b) => b.clone(),

                &Operation::Multiply(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Operation::Multiply(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Operation::Multiply(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Operation::Multiply(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Operation::Multiply(_, Sign::Zero) => Sign::Zero,
                &Operation::Multiply(Sign::Zero, _) => Sign::Zero,
                &Operation::Multiply(_, Sign::Join) => Sign::Join,
                &Operation::Multiply(Sign::Join, _) => Sign::Join,
                &Operation::Multiply(ref a, Sign::Meet) => a.clone(),
                &Operation::Multiply(Sign::Meet, ref b) => b.clone(),

                &Operation::DivideSigned(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Operation::DivideSigned(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Operation::DivideSigned(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Operation::DivideSigned(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Operation::DivideSigned(_, Sign::Zero) => Sign::Zero,
                &Operation::DivideSigned(Sign::Zero, _) => Sign::Zero,
                &Operation::DivideSigned(_, Sign::Join) => Sign::Join,
                &Operation::DivideSigned(Sign::Join, _) => Sign::Join,
                &Operation::DivideSigned(ref a, Sign::Meet) => a.clone(),
                &Operation::DivideSigned(Sign::Meet, ref b) => b.clone(),

                &Operation::DivideUnsigned(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Operation::DivideUnsigned(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Operation::DivideUnsigned(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Operation::DivideUnsigned(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Operation::DivideUnsigned(_, Sign::Zero) => Sign::Zero,
                &Operation::DivideUnsigned(Sign::Zero, _) => Sign::Zero,
                &Operation::DivideUnsigned(_, Sign::Join) => Sign::Join,
                &Operation::DivideUnsigned(Sign::Join, _) => Sign::Join,
                &Operation::DivideUnsigned(ref a, Sign::Meet) => a.clone(),
                &Operation::DivideUnsigned(Sign::Meet, ref b) => b.clone(),

                &Operation::Modulo(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Operation::Modulo(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Operation::Modulo(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Operation::Modulo(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Operation::Modulo(_, Sign::Zero) => Sign::Zero,
                &Operation::Modulo(Sign::Zero, _) => Sign::Zero,
                &Operation::Modulo(_, Sign::Join) => Sign::Join,
                &Operation::Modulo(Sign::Join, _) => Sign::Join,
                &Operation::Modulo(ref a, Sign::Meet) => a.clone(),
                &Operation::Modulo(Sign::Meet, ref b) => b.clone(),

                &Operation::Move(ref a) => a.clone(),
                &Operation::ZeroExtend(_, Sign::Negative) => Sign::Join,
                &Operation::ZeroExtend(_, ref a) => a.clone(),
                &Operation::SignExtend(_, ref a) => a.clone(),

                &Operation::Phi(ref ops) => {
                    match ops.len() {
                        0 => unreachable!("Phi function w/o arguments"),
                        1 => ops[0].clone(),
                        _ => ops.iter().fold(Sign::Meet, |acc, x| acc.combine(&x)),
                    }
                }

                _ => Sign::Join,
            }
        }

        fn narrow(&self, a: &Self) -> Self {
            match a {
                &Sign::Meet => Sign::Meet,
                &Sign::Join => self.clone(),
                &Sign::Positive | &Sign::Negative | &Sign::Zero => {
                    match self {
                        &Sign::Meet => Sign::Meet,
                        &Sign::Join => a.clone(),
                        a => if *a == *self { a.clone() } else { Sign::Meet },
                    }
                }
            }
        }

        fn combine(&self, b: &Self) -> Self {
            match (self, b) {
                (x, y) if x == y => x.clone(),
                (&Sign::Meet, x) => x.clone(),
                (x, &Sign::Meet) => x.clone(),
                _ => Sign::Join,
            }
        }

        fn widen(&self, b: &Self) -> Self {
            if *b == *self {
                self.clone()
            } else {
                Sign::Join
            }
        }


        fn initial() -> Self {
            Sign::Meet
        }

        fn more_exact(&self, b: &Self) -> bool {
            self != b &&
            match (self, b) {
                (&Sign::Meet, &Sign::Positive) |
                (&Sign::Meet, &Sign::Negative) |
                (&Sign::Meet, &Sign::Join) => false,
                (&Sign::Positive, &Sign::Join) |
                (&Sign::Negative, &Sign::Join) => false,
                _ => true,
            }
        }

        fn extract(&self, size: usize, offset: usize) -> Self {
            match self {
                &Sign::Join => Sign::Join,
                &Sign::Meet => Sign::Meet,
                &Sign::Positive => Sign::Positive,
                &Sign::Negative => Sign::Negative,
                &Sign::Zero => Sign::Zero,
            }
        }
    }

    // x = 0;
    // n = 1;
    // while (n <= undef) {
    //  x = x + n;
    //  n = n + 1;
    // }
    //
    #[test]
    fn signedness_analysis() {
        let x_var = Lvalue::Variable {
            name: Cow::Borrowed("x"),
            size: 32,
            subscript: None,
        };
        let n_var = Lvalue::Variable {
            name: Cow::Borrowed("n"),
            size: 32,
            subscript: None,
        };
        let flag = Lvalue::Variable {
            name: Cow::Borrowed("flag"),
            size: 1,
            subscript: None,
        };
        let bb0 = BasicBlock::from_vec(vec![Mnemonic::new(0..1,
                                                          "assign x".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u64(0)),
                                                                   assignee: x_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(1..2,
                                                          "assign n".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u64(1)),
                                                                   assignee: n_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(2..3,
                                                          "cmp n".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::LessOrEqualSigned(n_var.clone().into(), Rvalue::Undefined),
                                                                   assignee: flag.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb1 = BasicBlock::from_vec(vec![Mnemonic::new(3..4,
                                                          "add x and n".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(x_var.clone().into(), n_var.clone().into()),
                                                                   assignee: x_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(4..5,
                                                          "inc n".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(n_var.clone().into(), Rvalue::new_u64(1)),
                                                                   assignee: n_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(5..6,
                                                          "cmp n".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::LessOrEqualSigned(n_var.clone().into(), Rvalue::Undefined),
                                                                   assignee: flag.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb2 = BasicBlock {
            area: Bound::new(4, 5),
            mnemonics: vec![],
        };
        let mut cfg = ControlFlowGraph::new();

        let g = Guard::from_flag(&flag.clone().into()).ok().unwrap();
        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));

        cfg.add_edge(g.negation(), v0, v2);
        cfg.add_edge(g.negation(), v1, v2);
        cfg.add_edge(g.clone(), v0, v1);
        cfg.add_edge(g.clone(), v1, v1);

        let mut func = Function::new("func".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        ssa_convertion(&mut func);

        let vals = approximate::<Sign>(&func).ok().unwrap();
        let res = results::<Sign>(&func, &vals);

        assert_eq!(res[&(Cow::Borrowed("x"), 32)], Sign::Join);
        assert_eq!(res[&(Cow::Borrowed("n"), 32)], Sign::Positive);
    }

    // a = -256
    // b = 1
    // while(a <= 0) {
    //   a = a + 1
    //   b = a * 2
    // }
    // f(a)
    // f(b)
    //
    #[test]
    fn signedness_narrow() {
        let a_var = Lvalue::Variable {
            name: Cow::Borrowed("a"),
            size: 32,
            subscript: None,
        };
        let b_var = Lvalue::Variable {
            name: Cow::Borrowed("b"),
            size: 32,
            subscript: None,
        };
        let flag = Lvalue::Variable {
            name: Cow::Borrowed("flag"),
            size: 1,
            subscript: None,
        };
        let bb0 = BasicBlock::from_vec(vec![Mnemonic::new(0..1,
                                                          "assign a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u64(0xffffffffffffff00)),
                                                                   assignee: a_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(1..2,
                                                          "assign b".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u64(1)),
                                                                   assignee: b_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(2..3,
                                                          "cmp a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::LessOrEqualSigned(a_var.clone().into(), Rvalue::new_u64(0)),
                                                                   assignee: flag.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb1 = BasicBlock::from_vec(vec![Mnemonic::new(3..4,
                                                          "inc a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(a_var.clone().into(), Rvalue::new_u64(1)),
                                                                   assignee: a_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(4..5,
                                                          "mul a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Multiply(a_var.clone().into(), Rvalue::new_u64(2)),
                                                                   assignee: b_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(5..6,
                                                          "cmp a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::LessOrEqualSigned(a_var.clone().into(), Rvalue::new_u64(0)),
                                                                   assignee: flag.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb2 = BasicBlock::from_vec(vec![Mnemonic::new(6..7,
                                                          "use a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(a_var.clone()
                                                                       .into()),
                                                                   assignee: a_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(7..8,
                                                          "use b".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(b_var.clone()
                                                                       .into()),
                                                                   assignee: b_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let mut cfg = ControlFlowGraph::new();
        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));

        let g = Guard::from_flag(&flag.clone().into()).ok().unwrap();

        cfg.add_edge(g.negation(), v0, v2);
        cfg.add_edge(g.negation(), v1, v2);
        cfg.add_edge(g.clone(), v0, v1);
        cfg.add_edge(g.clone(), v1, v1);

        let mut func = Function::new("func".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        ssa_convertion(&mut func);

        let vals = approximate::<Sign>(&func).ok().unwrap();
        let res = results::<Sign>(&func, &vals);

        println!("vals: {:?}", vals);
        println!("res: {:?}", res);

        assert_eq!(res.get(&(Cow::Borrowed("a"), 32)), Some(&Sign::Positive));
        assert_eq!(res.get(&(Cow::Borrowed("b"), 32)), Some(&Sign::Positive));
    }

    // a = 10
    // b = 0
    // c = 4
    // if (c == 1) {
    //   a += 5;
    //   b = a * c;
    //   c = 2
    // } else {
    //   while(a > 0) {
    //     a -= 1
    //     b += 3
    //     c = 3
    //   }
    // }
    // x = a + b;
    //
    #[test]
    fn kset_test() {
        let a_var = Lvalue::Variable {
            name: Cow::Borrowed("a"),
            size: 32,
            subscript: None,
        };
        let b_var = Lvalue::Variable {
            name: Cow::Borrowed("b"),
            size: 32,
            subscript: None,
        };
        let c_var = Lvalue::Variable {
            name: Cow::Borrowed("c"),
            size: 32,
            subscript: None,
        };
        let x_var = Lvalue::Variable {
            name: Cow::Borrowed("x"),
            size: 32,
            subscript: None,
        };
        let flag = Lvalue::Variable {
            name: Cow::Borrowed("flag"),
            size: 1,
            subscript: None,
        };
        let bb0 = BasicBlock::from_vec(vec![Mnemonic::new(0..1,
                                                          "assign a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u32(10)),
                                                                   assignee: a_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(1..2,
                                                          "assign b".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u32(0)),
                                                                   assignee: b_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(2..3,
                                                          "assign c".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u32(4)),
                                                                   assignee: c_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(3..4,
                                                          "cmp c".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Equal(c_var.clone().into(), Rvalue::new_u32(1)),
                                                                   assignee: flag.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);

        let bb1 = BasicBlock::from_vec(vec![Mnemonic::new(4..5,
                                                          "add a and 5".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(a_var.clone().into(), Rvalue::new_u32(5)),
                                                                   assignee: a_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(5..6,
                                                          "mul a and c".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(a_var.clone().into(), c_var.clone().into()),
                                                                   assignee: b_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(6..7,
                                                          "assign c".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u32(2)),
                                                                   assignee: c_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb2 = BasicBlock::from_vec(vec![Mnemonic::new(7..8,
                                                          "dec a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Subtract(a_var.clone().into(), Rvalue::new_u32(1)),
                                                                   assignee: a_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(8..9,
                                                          "add 3 to b".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(b_var.clone().into(), Rvalue::new_u32(3)),
                                                                   assignee: b_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(9..10,
                                                          "assign c".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u32(3)),
                                                                   assignee: c_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb3 = BasicBlock::from_vec(vec![Mnemonic::new(10..11,
                                                          "add a and b".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Add(a_var.clone()
                                                                                          .into(),
                                                                                      b_var.clone()
                                                                                          .into()),
                                                                   assignee: x_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb4 = BasicBlock::from_vec(vec![Mnemonic::new(11..12,
                                                          "cmp a".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::LessOrEqualSigned(a_var.clone().into(), Rvalue::new_u32(0)),
                                                                   assignee: flag.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);


        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));

        let g = Guard::from_flag(&flag.into()).ok().unwrap();

        cfg.add_edge(g.clone(), v0, v1);
        cfg.add_edge(g.negation(), v0, v4);
        cfg.add_edge(g.negation(), v4, v2);
        cfg.add_edge(g.clone(), v4, v3);
        cfg.add_edge(Guard::always(), v2, v4);
        cfg.add_edge(Guard::always(), v1, v3);

        let mut func = Function::new("func".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        ssa_convertion(&mut func);

        let vals = approximate::<Kset>(&func).ok().unwrap();
        let res = results::<Kset>(&func, &vals);

        assert_eq!(res[&(Cow::Borrowed("a"), 32)], Kset::Join);
        assert_eq!(res[&(Cow::Borrowed("b"), 32)], Kset::Join);
        assert_eq!(res[&(Cow::Borrowed("c"), 32)],
                   Kset::Set(vec![(2, 32), (3, 32), (4, 32)]));
        assert_eq!(res[&(Cow::Borrowed("x"), 32)], Kset::Join);
    }

    #[test]
    fn bit_extract() {
        let p_var = Lvalue::Variable {
            name: Cow::Borrowed("p"),
            size: 22,
            subscript: None,
        };
        let r1_var = Lvalue::Variable {
            name: Cow::Borrowed("r1"),
            size: 8,
            subscript: None,
        };
        let r2_var = Lvalue::Variable {
            name: Cow::Borrowed("r2"),
            size: 8,
            subscript: None,
        };
        let next = Lvalue::Variable {
            name: Cow::Borrowed("R30:R31"),
            size: 22,
            subscript: None,
        };
        let bb0 = BasicBlock::from_vec(vec![Mnemonic::new(0..1,
                                                          "init r1".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u8(7)),
                                                                   assignee: r1_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(1..2,
                                                          "init r2".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(Rvalue::new_u8(88)),
                                                                   assignee: r2_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let bb1 = BasicBlock::from_vec(vec![Mnemonic::new(2..3,
                                                          "zext r1".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::ZeroExtend(22, r1_var.clone().into()),
                                                                   assignee: p_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(3..4,
                                                          "mov r2".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Select(8, p_var.clone().into(), r2_var.clone().into()),
                                                                   assignee: p_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(4..5,
                                                          "mov 0".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Select(16,
                                                                                         p_var.clone().into(),
                                                                                         Rvalue::Constant {
                                                                                             value: 0,
                                                                                             size: 6,
                                                                                         }),
                                                                   assignee: p_var.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap(),
                                            Mnemonic::new(5..6,
                                                          "mov next".to_string(),
                                                          "".to_string(),
                                                          vec![].iter(),
                                                          vec![Statement {
                                                                   op: Operation::Move(p_var.clone().into()),
                                                                   assignee: next.clone(),
                                                               }]
                                                              .iter())
                                                .ok()
                                                .unwrap()]);
        let mut cfg = ControlFlowGraph::new();
        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));

        cfg.add_edge(Guard::always(), v0, v1);

        let mut func = Function::new("func".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        ssa_convertion(&mut func);

        let vals = approximate::<Kset>(&func).ok().unwrap();

        for i in vals {
            println!("{:?}", i);
        }
    }
}
