/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015, 2017  Panopticon authors
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

use panopticon_core::{ControlFlowGraph, ControlFlowRef, ControlFlowTarget, Function, Guard, Lvalue, Operation, Result, Rvalue, Statement};

use panopticon_data_flow::flag_operations;

use panopticon_graph_algos::{BidirectionalGraphTrait, GraphTrait, IncidenceGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::dominator::immediate_dominator;
use panopticon_graph_algos::order::{HierarchicalOrdering, weak_topo_order};
use std::borrow::Cow;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;

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
#[derive(Debug,PartialEq,Eq,Clone,PartialOrd,Ord,Hash)]
pub struct ProgramPoint {
    pub address: u64,
    pub position: usize,
}

/// Abstract Domain. Models both under- and over-approximation.
pub trait Avalue: Clone + PartialEq + Eq + Hash + Debug {
    /// Alpha function. Returns domain element that approximates the concrete value the best
    fn abstract_value(&Rvalue) -> Self;
    /// Alpha function. Returns domain element that approximates the concrete value that fullfil
    /// the constraint the best.
    fn abstract_constraint(&Constraint) -> Self;
    /// Execute the abstract version of the operation, yielding the result.
    fn execute(&ProgramPoint, &Aoperation<Self>) -> Self;
    /// Narrows `self` with the argument.
    fn narrow(&self, &Self) -> Self;
    /// Widens `self` with the argument.
    fn widen(&self, other: &Self) -> Self;
    /// Computes the lowest upper bound of self and the argument.
    fn combine(&self, &Self) -> Self;
    /// Returns true if `self` <= `other`.
    fn more_exact(&self, other: &Self) -> bool;
    /// Returns the meet of the domain
    fn initial() -> Self;
    /// Mimics the Select operation.
    fn extract(&self, size: usize, offset: usize) -> Self;
}

/// A lifted RREIL operation.
#[derive(Debug)]
pub enum Aoperation<A: Avalue> {
    /// Integer addition
    Add(A, A),
    /// Integer subtraction
    Subtract(A, A),
    /// Unsigned integer multiplication
    Multiply(A, A),
    /// Unsigned integer division
    DivideUnsigned(A, A),
    /// Signed integer division
    DivideSigned(A, A),
    /// Bitwise left shift
    ShiftLeft(A, A),
    /// Bitwise logical right shift
    ShiftRightUnsigned(A, A),
    /// Bitwise arithmetic right shift
    ShiftRightSigned(A, A),
    /// Integer modulo
    Modulo(A, A),
    /// Bitwise logical and
    And(A, A),
    /// Bitwise logical or
    InclusiveOr(A, A),
    /// Bitwise logical xor
    ExclusiveOr(A, A),

    /// Compare both operands for equality and returns `1` or `0`
    Equal(A, A),
    /// Returns `1` if the first operand is less than or equal to the second and `0` otherwise.
    /// Comparison assumes unsigned values.
    LessOrEqualUnsigned(A, A),
    /// Returns `1` if the first operand is less than or equal to the second and `0` otherwise.
    /// Comparison assumes signed values.
    LessOrEqualSigned(A, A),
    /// Returns `1` if the first operand is less than the second and `0` otherwise.
    /// Comparison assumes unsigned values.
    LessUnsigned(A, A),
    /// Returns `1` if the first operand is less than the second and `0` otherwise.
    /// Comparison assumes signed values.
    LessSigned(A, A),

    /// Zero extends the operand.
    ZeroExtend(usize, A),
    /// Sign extends the operand.
    SignExtend(usize, A),
    /// Copies the operand without modification.
    Move(A),
    /// Calls the function located at the address pointed to by the operand.
    Call(A),
    /// Copies only a range of bit from the operand.
    Select(usize, A, A),

    /// Reads a memory cell
    Load(Cow<'static, str>, A),
    /// Writes a memory cell
    Store(Cow<'static, str>, A),

    /// SSA Phi function
    Phi(Vec<A>),
}

impl<A: Avalue> Aoperation<A> {
    /// Returns its operands
    pub fn operands(&self) -> Vec<&A> {
        match *self {
            Aoperation::Add(ref a, ref b) => return vec![a, b],
            Aoperation::Subtract(ref a, ref b) => return vec![a, b],
            Aoperation::Multiply(ref a, ref b) => return vec![a, b],
            Aoperation::DivideUnsigned(ref a, ref b) => return vec![a, b],
            Aoperation::DivideSigned(ref a, ref b) => return vec![a, b],
            Aoperation::ShiftLeft(ref a, ref b) => return vec![a, b],
            Aoperation::ShiftRightUnsigned(ref a, ref b) => return vec![a, b],
            Aoperation::ShiftRightSigned(ref a, ref b) => return vec![a, b],
            Aoperation::Modulo(ref a, ref b) => return vec![a, b],
            Aoperation::And(ref a, ref b) => return vec![a, b],
            Aoperation::InclusiveOr(ref a, ref b) => return vec![a, b],
            Aoperation::ExclusiveOr(ref a, ref b) => return vec![a, b],

            Aoperation::Equal(ref a, ref b) => return vec![a, b],
            Aoperation::LessOrEqualUnsigned(ref a, ref b) => return vec![a, b],
            Aoperation::LessOrEqualSigned(ref a, ref b) => return vec![a, b],
            Aoperation::LessUnsigned(ref a, ref b) => return vec![a, b],
            Aoperation::LessSigned(ref a, ref b) => return vec![a, b],

            Aoperation::ZeroExtend(_, ref a) => return vec![a],
            Aoperation::SignExtend(_, ref a) => return vec![a],
            Aoperation::Move(ref a) => return vec![a],
            Aoperation::Call(ref a) => return vec![a],
            Aoperation::Select(_, ref a, ref b) => return vec![a, b],

            Aoperation::Load(_, ref b) => return vec![b],
            Aoperation::Store(_, ref b) => return vec![b],

            Aoperation::Phi(ref vec) => return vec.iter().collect(),
        }
    }
}

/// Does an abstract interpretation of `func` using the abstract domain `A`. The function uses a
/// fixed point iteration and the widening strategy outlined in
/// Bourdoncle: "Efficient chaotic iteration strategies with widenings".
pub fn approximate<A: Avalue>(func: &Function, fixed: &HashMap<(Cow<'static, str>, usize), A>) -> Result<HashMap<Lvalue, A>> {
    if func.entry_point.is_none() {
        return Err("function has no entry point".into());
    }

    let wto = weak_topo_order(func.entry_point.unwrap(), &func.cflow_graph);
    let edge_ops = flag_operations(func);
    fn stabilize<A: Avalue>(
        h: &Vec<Box<HierarchicalOrdering<ControlFlowRef>>>,
        graph: &ControlFlowGraph,
        constr: &HashMap<Lvalue, A>,
        sizes: &HashMap<Cow<'static, str>, usize>,
        ret: &mut HashMap<(Cow<'static, str>, usize), A>,
        fixed: &HashMap<(Cow<'static, str>, usize), A>,
    ) -> Result<()> {
        let mut stable = true;
        let mut iter_cnt = 0;
        let head = if let Some(h) = h.first() {
            match &**h {
                &HierarchicalOrdering::Element(ref vx) => vx.clone(),
                &HierarchicalOrdering::Component(ref vec) => return stabilize(vec, graph, constr, sizes, ret, fixed),
            }
        } else {
            return Ok(());
        };

        loop {
            for x in h.iter() {
                match &**x {
                    &HierarchicalOrdering::Element(ref vx) => {
                        stable &= !execute(
                            *vx,
                            iter_cnt >= 2 && *vx == head,
                            graph,
                            constr,
                            sizes,
                            ret,
                            fixed,
                        )?
                    }
                    &HierarchicalOrdering::Component(ref vec) => {
                        stabilize(&*vec, graph, constr, sizes, ret, fixed)?;
                        stable = true;
                    }
                }
            }

            if stable {
                for (lv, a) in constr.iter() {
                    if let &Lvalue::Variable { ref name, subscript: Some(ref subscript), .. } = lv {
                        let nam = (name.clone(), *subscript);

                        if let Some(ref mut x) = ret.get_mut(&nam) {
                            let n = x.narrow(&a);
                            **x = n;
                        }
                    }
                }

                //execute(*vx,do_widen && vx == head,graph,ret),
                return Ok(());
            }

            stable = true;
            iter_cnt += 1;
        }
    }
    fn execute<A: Avalue>(
        t: ControlFlowRef,
        do_widen: bool,
        graph: &ControlFlowGraph,
        _: &HashMap<Lvalue, A>,
        sizes: &HashMap<Cow<'static, str>, usize>,
        ret: &mut HashMap<(Cow<'static, str>, usize), A>,
        fixed: &HashMap<(Cow<'static, str>, usize), A>,
    ) -> Result<bool> {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = graph.vertex_label(t) {
            let mut change = false;
            let mut pos = 0usize;
            bb.execute(
                |i| {
                    if let Statement {
                               ref op,
                               assignee: Lvalue::Variable { ref name, subscript: Some(ref subscript), .. },
                           } = *i {
                        let pp = ProgramPoint { address: bb.area.start, position: pos };
                        let op = lift(op, &|x| res::<A>(x, sizes, &ret, fixed));
                        let new = A::execute(&pp, &op);
                        let assignee = (name.clone(), *subscript);
                        let cur = ret.get(&assignee).cloned();

                        debug!("{:?} {:?}: {:?} = {:?}", pp, assignee, op, new);
                        debug!("    prev: {:?}", cur);

                        if let Some(cur) = cur {
                            if do_widen {
                                let w = cur.widen(&new);

                                debug!("    widen to {:?}", w);

                                if w != cur {
                                    change = true;
                                    ret.insert(assignee, w.clone());
                                    debug!("    new value {:?}", w);
                                }
                            } else if !cur.more_exact(&new) && cur != new {
                                change = true;
                                ret.insert(assignee, new.clone());
                                debug!("    new value {:?}", new);
                            } else {
                                debug!("    {:?} is more exact than {:?}", cur, new);
                            }
                        } else {
                            change = true;
                            ret.insert(assignee, new.clone());
                            debug!("    new value {:?}", new);
                        }
                    }

                    pos += 1;
                }
            );

            Ok(change)
        } else {
            Ok(false)
        }
    }
    fn res<A: Avalue>(
        v: &Rvalue,
        sizes: &HashMap<Cow<'static, str>, usize>,
        env: &HashMap<(Cow<'static, str>, usize), A>,
        fixed: &HashMap<(Cow<'static, str>, usize), A>,
    ) -> A {
        if let &Rvalue::Variable {
                   ref name,
                   subscript: Some(ref subscript),
                   ref size,
                   ref offset,
               } = v {
            let nam = (name.clone(), *subscript);
            let t = fixed.get(&nam).or_else(|| env.get(&nam)).unwrap_or(&A::initial()).clone();

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
            bb.execute(
                |i| if let Lvalue::Variable { ref name, ref size, .. } = i.assignee {
                    let t = *size;
                    let s = *sizes.get(name).unwrap_or(&t);
                    sizes.insert(name.clone(), max(s, t));
                }
            );
        }
    }

    for vx in func.cflow_graph.vertices() {
        for e in func.cflow_graph.in_edges(vx) {
            if let Some(&Guard::Predicate { .. }) = func.cflow_graph.edge_label(e) {
                match edge_ops.get(&e).cloned() {
                    Some(Operation::Equal(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(right).unwrap(),
                            A::abstract_constraint(&Constraint::Equal(left.clone())),
                        );
                    }
                    Some(Operation::Equal(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(left).unwrap(),
                            A::abstract_constraint(&Constraint::Equal(right.clone())),
                        );
                    }
                    Some(Operation::LessUnsigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(right).unwrap(),
                            A::abstract_constraint(&Constraint::LessUnsigned(left.clone())),
                        );
                    }
                    Some(Operation::LessUnsigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(left).unwrap(),
                            A::abstract_constraint(&Constraint::LessUnsigned(right.clone())),
                        );
                    }
                    Some(Operation::LessSigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(right).unwrap(),
                            A::abstract_constraint(&Constraint::LessSigned(left.clone())),
                        );
                    }
                    Some(Operation::LessSigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(left).unwrap(),
                            A::abstract_constraint(&Constraint::LessSigned(right.clone())),
                        );
                    }
                    Some(Operation::LessOrEqualUnsigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(right).unwrap(),
                            A::abstract_constraint(&Constraint::LessOrEqualUnsigned(left.clone())),
                        );
                    }
                    Some(Operation::LessOrEqualUnsigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(left).unwrap(),
                            A::abstract_constraint(&Constraint::LessOrEqualUnsigned(right.clone())),
                        );
                    }
                    Some(Operation::LessOrEqualSigned(left @ Rvalue::Constant { .. }, right @ Rvalue::Variable { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(right).unwrap(),
                            A::abstract_constraint(&Constraint::LessOrEqualSigned(left.clone())),
                        );
                    }
                    Some(Operation::LessOrEqualSigned(left @ Rvalue::Variable { .. }, right @ Rvalue::Constant { .. })) => {
                        constr.insert(
                            Lvalue::from_rvalue(left).unwrap(),
                            A::abstract_constraint(&Constraint::LessOrEqualSigned(right.clone())),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    match wto {
        HierarchicalOrdering::Component(ref v) => {
            stabilize(v, &func.cflow_graph, &constr, &sizes, &mut ret, fixed)?;
        }
        HierarchicalOrdering::Element(ref v) => {
            execute(
                *v,
                false,
                &func.cflow_graph,
                &constr,
                &sizes,
                &mut ret,
                fixed,
            )?;
        }
    }

    for (k, v) in fixed.iter() {
        ret.insert(k.clone(), v.clone());
    }

    Ok(
        HashMap::from_iter(
            ret.iter()
                .filter_map(
                    |(&(ref name, ref subscript), val)| if let Some(sz) = sizes.get(name) {
                        Some((Lvalue::Variable { name: name.clone(), subscript: Some(*subscript), size: *sz }, val.clone()))
                    } else {
                        None
                    }
                )
        )
    )
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
            bb.execute(
                |i| if let Lvalue::Variable { ref name, .. } = i.assignee {
                    names.insert(name.clone());
                }
            );
        }
    }

    for v in cfg.vertices() {
        if cfg.out_degree(v) == 0 {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(v) {
                for lv in names.iter() {
                    let mut bbv = (bb, v);

                    loop {
                        let mut hit = false;
                        bb.execute_backwards(
                            |i| if let Lvalue::Variable { ref name, ref size, .. } = i.assignee {
                                if name == lv {
                                    hit = true;
                                    ret.insert(
                                        (name.clone(), *size),
                                        vals.get(&i.assignee).unwrap_or(&A::initial()).clone(),
                                    );
                                }
                            }
                        );

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

/// Maps the function `m` over all operands of `op`.
pub fn lift<A, F>(op: &Operation, m: &F) -> Aoperation<A>
where A: Avalue + Clone + PartialEq + Eq + Debug,
      F: Fn(&Rvalue) -> A
{
    let args = op.operands().iter().cloned().map(m).collect::<Vec<_>>();
    match op {
        &Operation::Phi(_) => Aoperation::Phi(args),
        &Operation::Load(ref s, _) => Aoperation::Load(s.clone(), args[0].clone()),
        &Operation::Store(ref s, _) => Aoperation::Store(s.clone(), args[0].clone()),
        &Operation::Add(_, _) => Aoperation::Add(args[0].clone(), args[1].clone()),
        &Operation::Subtract(_, _) => Aoperation::Subtract(args[0].clone(), args[1].clone()),
        &Operation::Multiply(_, _) => Aoperation::Multiply(args[0].clone(), args[1].clone()),
        &Operation::DivideUnsigned(_, _) => Aoperation::DivideUnsigned(args[0].clone(), args[1].clone()),
        &Operation::DivideSigned(_, _) => Aoperation::DivideSigned(args[0].clone(), args[1].clone()),
        &Operation::ShiftLeft(_, _) => Aoperation::ShiftLeft(args[0].clone(), args[1].clone()),
        &Operation::ShiftRightUnsigned(_, _) => Aoperation::ShiftRightUnsigned(args[0].clone(), args[1].clone()),
        &Operation::ShiftRightSigned(_, _) => Aoperation::ShiftRightSigned(args[0].clone(), args[1].clone()),
        &Operation::Modulo(_, _) => Aoperation::Modulo(args[0].clone(), args[1].clone()),
        &Operation::And(_, _) => Aoperation::And(args[0].clone(), args[1].clone()),
        &Operation::InclusiveOr(_, _) => Aoperation::InclusiveOr(args[0].clone(), args[1].clone()),
        &Operation::ExclusiveOr(_, _) => Aoperation::ExclusiveOr(args[0].clone(), args[1].clone()),
        &Operation::Equal(_, _) => Aoperation::Equal(args[0].clone(), args[1].clone()),
        &Operation::LessUnsigned(_, _) => Aoperation::LessUnsigned(args[0].clone(), args[1].clone()),
        &Operation::LessSigned(_, _) => Aoperation::LessSigned(args[0].clone(), args[1].clone()),
        &Operation::LessOrEqualUnsigned(_, _) => Aoperation::LessOrEqualUnsigned(args[0].clone(), args[1].clone()),
        &Operation::LessOrEqualSigned(_, _) => Aoperation::LessOrEqualSigned(args[0].clone(), args[1].clone()),
        &Operation::Call(_) => Aoperation::Call(args[0].clone()),
        &Operation::Move(_) => Aoperation::Move(args[0].clone()),
        &Operation::Select(ref off, _, _) => Aoperation::Select(*off, args[0].clone(), args[1].clone()),
        &Operation::ZeroExtend(ref sz, _) => Aoperation::ZeroExtend(*sz, args[0].clone()),
        &Operation::SignExtend(ref sz, _) => Aoperation::SignExtend(*sz, args[0].clone()),
    }
}

/// Maps the function `m` over all operands of `op`.
pub fn translate<A, B, F>(op: &Aoperation<B>, m: &F) -> Aoperation<A>
where A: Avalue + Clone,
      B: Avalue + Clone,
      F: Fn(&B) -> A
{
    let args = op.operands().iter().cloned().map(m).collect::<Vec<_>>();
    match op {
        &Aoperation::Phi(_) => Aoperation::Phi(args),
        &Aoperation::Load(ref s, _) => Aoperation::Load(s.clone(), args[0].clone()),
        &Aoperation::Store(ref s, _) => Aoperation::Store(s.clone(), args[0].clone()),
        &Aoperation::Add(_, _) => Aoperation::Add(args[0].clone(), args[1].clone()),
        &Aoperation::Subtract(_, _) => Aoperation::Subtract(args[0].clone(), args[1].clone()),
        &Aoperation::Multiply(_, _) => Aoperation::Multiply(args[0].clone(), args[1].clone()),
        &Aoperation::DivideUnsigned(_, _) => Aoperation::DivideUnsigned(args[0].clone(), args[1].clone()),
        &Aoperation::DivideSigned(_, _) => Aoperation::DivideSigned(args[0].clone(), args[1].clone()),
        &Aoperation::ShiftLeft(_, _) => Aoperation::ShiftLeft(args[0].clone(), args[1].clone()),
        &Aoperation::ShiftRightUnsigned(_, _) => Aoperation::ShiftRightUnsigned(args[0].clone(), args[1].clone()),
        &Aoperation::ShiftRightSigned(_, _) => Aoperation::ShiftRightSigned(args[0].clone(), args[1].clone()),
        &Aoperation::Modulo(_, _) => Aoperation::Modulo(args[0].clone(), args[1].clone()),
        &Aoperation::And(_, _) => Aoperation::And(args[0].clone(), args[1].clone()),
        &Aoperation::InclusiveOr(_, _) => Aoperation::InclusiveOr(args[0].clone(), args[1].clone()),
        &Aoperation::ExclusiveOr(_, _) => Aoperation::ExclusiveOr(args[0].clone(), args[1].clone()),
        &Aoperation::Equal(_, _) => Aoperation::Equal(args[0].clone(), args[1].clone()),
        &Aoperation::LessUnsigned(_, _) => Aoperation::LessUnsigned(args[0].clone(), args[1].clone()),
        &Aoperation::LessSigned(_, _) => Aoperation::LessSigned(args[0].clone(), args[1].clone()),
        &Aoperation::LessOrEqualUnsigned(_, _) => Aoperation::LessOrEqualUnsigned(args[0].clone(), args[1].clone()),
        &Aoperation::LessOrEqualSigned(_, _) => Aoperation::LessOrEqualSigned(args[0].clone(), args[1].clone()),
        &Aoperation::Call(_) => Aoperation::Call(args[0].clone()),
        &Aoperation::Move(_) => Aoperation::Move(args[0].clone()),
        &Aoperation::Select(ref off, _, _) => Aoperation::Select(*off, args[0].clone(), args[1].clone()),
        &Aoperation::ZeroExtend(ref sz, _) => Aoperation::ZeroExtend(*sz, args[0].clone()),
        &Aoperation::SignExtend(ref sz, _) => Aoperation::SignExtend(*sz, args[0].clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use panopticon_core::{BasicBlock, Bound, ControlFlowGraph, ControlFlowTarget, Function, Guard, Lvalue, Mnemonic, Operation, Rvalue, Statement};
    use panopticon_data_flow::ssa_convertion;
    use panopticon_graph_algos::MutableGraphTrait;
    use std::borrow::Cow;

    #[derive(Debug,Clone,PartialEq,Eq,Hash)]
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

        fn execute(_: &ProgramPoint, op: &Aoperation<Self>) -> Self {
            match op {
                &Aoperation::Add(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Aoperation::Add(Sign::Positive, Sign::Zero) => Sign::Positive,
                &Aoperation::Add(Sign::Zero, Sign::Positive) => Sign::Positive,
                &Aoperation::Add(Sign::Negative, Sign::Negative) => Sign::Negative,
                &Aoperation::Add(Sign::Negative, Sign::Zero) => Sign::Negative,
                &Aoperation::Add(Sign::Zero, Sign::Negative) => Sign::Negative,
                &Aoperation::Add(Sign::Positive, Sign::Negative) => Sign::Join,
                &Aoperation::Add(Sign::Negative, Sign::Positive) => Sign::Join,
                &Aoperation::Add(_, Sign::Join) => Sign::Join,
                &Aoperation::Add(Sign::Join, _) => Sign::Join,
                &Aoperation::Add(ref a, Sign::Meet) => a.clone(),
                &Aoperation::Add(Sign::Meet, ref b) => b.clone(),

                &Aoperation::Subtract(Sign::Positive, Sign::Positive) => Sign::Join,
                &Aoperation::Subtract(Sign::Positive, Sign::Zero) => Sign::Positive,
                &Aoperation::Subtract(Sign::Zero, Sign::Positive) => Sign::Negative,
                &Aoperation::Subtract(Sign::Negative, Sign::Negative) => Sign::Join,
                &Aoperation::Subtract(Sign::Negative, Sign::Zero) => Sign::Negative,
                &Aoperation::Subtract(Sign::Zero, Sign::Negative) => Sign::Positive,
                &Aoperation::Subtract(Sign::Positive, Sign::Negative) => Sign::Positive,
                &Aoperation::Subtract(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Aoperation::Subtract(_, Sign::Join) => Sign::Join,
                &Aoperation::Subtract(Sign::Join, _) => Sign::Join,
                &Aoperation::Subtract(ref a, Sign::Meet) => a.clone(),
                &Aoperation::Subtract(Sign::Meet, ref b) => b.clone(),

                &Aoperation::Multiply(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Aoperation::Multiply(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Aoperation::Multiply(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Aoperation::Multiply(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Aoperation::Multiply(_, Sign::Zero) => Sign::Zero,
                &Aoperation::Multiply(Sign::Zero, _) => Sign::Zero,
                &Aoperation::Multiply(_, Sign::Join) => Sign::Join,
                &Aoperation::Multiply(Sign::Join, _) => Sign::Join,
                &Aoperation::Multiply(ref a, Sign::Meet) => a.clone(),
                &Aoperation::Multiply(Sign::Meet, ref b) => b.clone(),

                &Aoperation::DivideSigned(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Aoperation::DivideSigned(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Aoperation::DivideSigned(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Aoperation::DivideSigned(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Aoperation::DivideSigned(_, Sign::Zero) => Sign::Zero,
                &Aoperation::DivideSigned(Sign::Zero, _) => Sign::Zero,
                &Aoperation::DivideSigned(_, Sign::Join) => Sign::Join,
                &Aoperation::DivideSigned(Sign::Join, _) => Sign::Join,
                &Aoperation::DivideSigned(ref a, Sign::Meet) => a.clone(),
                &Aoperation::DivideSigned(Sign::Meet, ref b) => b.clone(),

                &Aoperation::DivideUnsigned(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Aoperation::DivideUnsigned(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Aoperation::DivideUnsigned(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Aoperation::DivideUnsigned(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Aoperation::DivideUnsigned(_, Sign::Zero) => Sign::Zero,
                &Aoperation::DivideUnsigned(Sign::Zero, _) => Sign::Zero,
                &Aoperation::DivideUnsigned(_, Sign::Join) => Sign::Join,
                &Aoperation::DivideUnsigned(Sign::Join, _) => Sign::Join,
                &Aoperation::DivideUnsigned(ref a, Sign::Meet) => a.clone(),
                &Aoperation::DivideUnsigned(Sign::Meet, ref b) => b.clone(),

                &Aoperation::Modulo(Sign::Positive, Sign::Positive) => Sign::Positive,
                &Aoperation::Modulo(Sign::Negative, Sign::Negative) => Sign::Positive,
                &Aoperation::Modulo(Sign::Positive, Sign::Negative) => Sign::Negative,
                &Aoperation::Modulo(Sign::Negative, Sign::Positive) => Sign::Negative,
                &Aoperation::Modulo(_, Sign::Zero) => Sign::Zero,
                &Aoperation::Modulo(Sign::Zero, _) => Sign::Zero,
                &Aoperation::Modulo(_, Sign::Join) => Sign::Join,
                &Aoperation::Modulo(Sign::Join, _) => Sign::Join,
                &Aoperation::Modulo(ref a, Sign::Meet) => a.clone(),
                &Aoperation::Modulo(Sign::Meet, ref b) => b.clone(),

                &Aoperation::Move(ref a) => a.clone(),
                &Aoperation::ZeroExtend(_, Sign::Negative) => Sign::Join,
                &Aoperation::ZeroExtend(_, ref a) => a.clone(),
                &Aoperation::SignExtend(_, ref a) => a.clone(),

                &Aoperation::Phi(ref ops) => {
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

        fn extract(&self, _: usize, _: usize) -> Self {
            match self {
                &Sign::Join => Sign::Join,
                &Sign::Meet => Sign::Meet,
                &Sign::Positive => Sign::Positive,
                &Sign::Negative => Sign::Negative,
                &Sign::Zero => Sign::Zero,
            }
        }
    }

    /*
     * x = 0;
     * n = 1;
     * while (n <= undef) {
     *  x = x + n;
     *  n = n + 1;
     * }
     */
    #[test]
    fn signedness_analysis() {
        let x_var = Lvalue::Variable { name: Cow::Borrowed("x"), size: 32, subscript: None };
        let n_var = Lvalue::Variable { name: Cow::Borrowed("n"), size: 32, subscript: None };
        let flag = Lvalue::Variable { name: Cow::Borrowed("flag"), size: 1, subscript: None };
        let bb0 = BasicBlock::from_vec(
            vec![
                Mnemonic::new(
                    0..1,
                    "assign x".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Move(Rvalue::new_u64(0)),
                            assignee: x_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    1..2,
                    "assign n".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Move(Rvalue::new_u64(1)),
                            assignee: n_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    2..3,
                    "cmp n".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::LessOrEqualSigned(n_var.clone().into(), Rvalue::Undefined),
                            assignee: flag.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
            ]
        );
        let bb1 = BasicBlock::from_vec(
            vec![
                Mnemonic::new(
                    3..4,
                    "add x and n".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Add(x_var.clone().into(), n_var.clone().into()),
                            assignee: x_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    4..5,
                    "inc n".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Add(n_var.clone().into(), Rvalue::new_u64(1)),
                            assignee: n_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    5..6,
                    "cmp n".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::LessOrEqualSigned(n_var.clone().into(), Rvalue::Undefined),
                            assignee: flag.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
            ]
        );
        let bb2 = BasicBlock { area: Bound::new(4, 5), mnemonics: vec![] };
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

        assert!(ssa_convertion(&mut func).is_ok());

        let vals = approximate::<Sign>(&func, &HashMap::new()).ok().unwrap();
        let res = results::<Sign>(&func, &vals);

        assert_eq!(res[&(Cow::Borrowed("x"), 32)], Sign::Join);
        assert_eq!(res[&(Cow::Borrowed("n"), 32)], Sign::Positive);
    }

    /*
     * a = -256
     * b = 1
     * while(a <= 0) {
     *   a = a + 1
     *   b = a * 2
     * }
     * f(a)
     * f(b)
     */
    #[test]
    fn signedness_narrow() {
        let a_var = Lvalue::Variable { name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b_var = Lvalue::Variable { name: Cow::Borrowed("b"), size: 32, subscript: None };
        let flag = Lvalue::Variable { name: Cow::Borrowed("flag"), size: 1, subscript: None };
        let bb0 = BasicBlock::from_vec(
            vec![
                Mnemonic::new(
                    0..1,
                    "assign a".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Move(Rvalue::new_u64(0xffffffffffffff00)),
                            assignee: a_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    1..2,
                    "assign b".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Move(Rvalue::new_u64(1)),
                            assignee: b_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    2..3,
                    "cmp a".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::LessOrEqualSigned(a_var.clone().into(), Rvalue::new_u64(0)),
                            assignee: flag.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
            ]
        );
        let bb1 = BasicBlock::from_vec(
            vec![
                Mnemonic::new(
                    3..4,
                    "inc a".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Add(a_var.clone().into(), Rvalue::new_u64(1)),
                            assignee: a_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    4..5,
                    "mul a".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Multiply(a_var.clone().into(), Rvalue::new_u64(2)),
                            assignee: b_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    5..6,
                    "cmp a".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::LessOrEqualSigned(a_var.clone().into(), Rvalue::new_u64(0)),
                            assignee: flag.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
            ]
        );
        let bb2 = BasicBlock::from_vec(
            vec![
                Mnemonic::new(
                    6..7,
                    "use a".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Move(a_var.clone().into()),
                            assignee: a_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
                Mnemonic::new(
                    7..8,
                    "use b".to_string(),
                    "".to_string(),
                    vec![].iter(),
                    vec![
                        Statement {
                            op: Operation::Move(b_var.clone().into()),
                            assignee: b_var.clone(),
                        },
                    ]
                            .iter(),
                )
                        .ok()
                        .unwrap(),
            ]
        );
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

        assert!(ssa_convertion(&mut func).is_ok());

        let vals = approximate::<Sign>(&func, &HashMap::new()).ok().unwrap();
        let res = results::<Sign>(&func, &vals);

        println!("vals: {:?}", vals);
        println!("res: {:?}", res);

        assert_eq!(res.get(&(Cow::Borrowed("a"), 32)), Some(&Sign::Positive));
        assert_eq!(res.get(&(Cow::Borrowed("b"), 32)), Some(&Sign::Positive));
    }
}
