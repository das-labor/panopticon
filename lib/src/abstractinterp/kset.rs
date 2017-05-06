/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2016 Panopticon Authors
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

//! Kindler et.al style Kset domain.
//!
//! TODO

use std::collections::{HashSet};
use std::iter::FromIterator;
use std::fmt;

use {
    Rvalue,
    Avalue,
    Constraint,
    ProgramPoint,
    Operation,
    execute,
};

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
    Set(Vec<(u64,usize)>),
    /// Lattice meet, equal to the empty set.
    Meet,
}

impl fmt::Display for Kset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Kset::Meet => write!(f, "Ø"),
            &Kset::Set(ref vec) if vec.is_empty() => write!(f, "Ø"),
            &Kset::Set(ref vec) if vec.len() == 1 => write!(f, "{{0x{:x}}}", vec[0].0),
            &Kset::Set(ref vec) => {
                write!(f, "{{0x{:x}", vec[0].0)?;
                for &(v,_) in vec.iter() {
                    write!(f, ", 0x{:x}",v)?;
                }
                write!(f, "}}")
            }
            &Kset::Join => write!(f, "⫟"),
        }
    }
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
    fn abstract_value(v: &Rvalue) -> Self {
        if let &Rvalue::Constant{ ref value, ref size } = v {
            Kset::Set(vec![(if *size < 64 { *value % (1u64 << *size) } else { *value },*size)])
        } else {
            Kset::Join
        }
    }

    fn abstract_constraint(constr: &Constraint) -> Self {
        if let &Constraint::Equal(Rvalue::Constant{ ref value, ref size }) = constr {
            Kset::Set(vec![(if *size < 64 { *value % (1u64 << *size) } else { *value },*size)])
        } else {
            Kset::Join
        }
    }

    fn execute(_: &ProgramPoint, op: &Operation<Self>) -> Self {
        fn permute(_a: &Kset, _b: &Kset, f: &Fn(Rvalue,Rvalue) -> Rvalue) -> Kset {
            match (_a,_b) {
                (&Kset::Join,_) => Kset::Join,
                (_,&Kset::Join) => Kset::Join,
                (&Kset::Set(ref a),&Kset::Set(ref b)) => {
                    let mut ret = HashSet::<(u64,usize)>::new();
                    for &(_x,_xs) in a.iter() {
                        let x = Rvalue::Constant{ value: _x, size: _xs };
                        for &(_y,_ys) in b.iter() {
                            let y = Rvalue::Constant{ value: _y, size: _ys };
                            if let Rvalue::Constant{ value, size } = f(x.clone(),y) {
                                ret.insert((value,size));
                                if ret.len() > KSET_MAXIMAL_CARDINALITY {
                                    return Kset::Join;
                                }
                            }
                        }
                    }

                    if ret.is_empty() {
                        Kset::Meet
                    } else {
                        let mut v = ret.drain().collect::<Vec<(u64,usize)>>();
                        v.sort();
                        Kset::Set(v)
                    }
                },
                _ => Kset::Meet,
            }
        };
        fn map(_a: &Kset, f: &Fn(Rvalue) -> Rvalue) -> Kset {
            if let &Kset::Set(ref a) = _a {
                let mut s = HashSet::<(u64,usize)>::from_iter(
                    a.iter().filter_map(|&(a,_as)| {
                        if let Rvalue::Constant{ value, size } = f(Rvalue::Constant{ value: a, size: _as }) {
                            Some((value,size))
                        } else {
                            None
                        }
                    }));

                if s.len() > KSET_MAXIMAL_CARDINALITY {
                    Kset::Join
                } else if s.is_empty() {
                    Kset::Meet
                } else {
                    let mut v = s.drain().collect::<Vec<(_,_)>>();
                    v.sort();
                    Kset::Set(v)
                }
            } else {
                _a.clone()
            }
        };

        match *op {
            Operation::And(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::And(a,b))),
            Operation::InclusiveOr(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::InclusiveOr(a,b))),
            Operation::ExclusiveOr(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::ExclusiveOr(a,b))),
            Operation::Add(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::Add(a,b))),
            Operation::Subtract(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::Subtract(a,b))),
            Operation::Multiply(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::Multiply(a,b))),
            Operation::DivideSigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::DivideSigned(a,b))),
            Operation::DivideUnsigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::DivideUnsigned(a,b))),
            Operation::Modulo(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::Modulo(a,b))),
            Operation::ShiftRightSigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::ShiftRightSigned(a,b))),
            Operation::ShiftRightUnsigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::ShiftRightUnsigned(a,b))),
            Operation::ShiftLeft(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::ShiftLeft(a,b))),

            Operation::LessOrEqualSigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::LessOrEqualSigned(a,b))),
            Operation::LessOrEqualUnsigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::LessOrEqualUnsigned(a,b))),
            Operation::LessSigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::LessSigned(a,b))),
            Operation::LessUnsigned(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::LessUnsigned(a,b))),
            Operation::Equal(ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::Equal(a,b))),

            Operation::Move(ref a) =>
                map(a,&|a| execute(Operation::Move(a))),
            Operation::Call(ref a) =>
                map(a,&|a| execute(Operation::Call(a))),
            Operation::ZeroExtend(ref sz,ref a) =>
                map(a,&|a| execute(Operation::ZeroExtend(*sz,a))),
            Operation::SignExtend(ref sz,ref a) =>
                map(a,&|a| execute(Operation::SignExtend(*sz,a))),
            Operation::Select(ref off,ref a,ref b) =>
                permute(a,b,&|a,b| execute(Operation::Select(*off,a,b))),

            Operation::Load(ref r,ref a) =>
                map(a,&|a| execute(Operation::Load(r.clone(),a))),
            Operation::Store(ref r,ref a) =>
                map(a,&|a| execute(Operation::Store(r.clone(),a))),

            Operation::Phi(ref ops) => {
                match ops.len() {
                    0 => unreachable!("Phi function w/o arguments"),
                    1 => ops[0].clone(),
                    _ => ops.iter().fold(Kset::Meet,|acc,x| acc.combine(&x))
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
                        let set = HashSet::<&(u64,usize)>::from_iter(v.iter());
                        Kset::Set(w.iter().filter(|x| set.contains(x)).cloned().collect::<Vec<_>>())
                    },
                }
            },
        }
    }

    fn combine(&self,a: &Self) -> Self {
        match (self,a) {
            (&Kset::Join,_) => Kset::Join,
            (_,&Kset::Join) => Kset::Join,
            (a,&Kset::Meet) => a.clone(),
            (&Kset::Meet,b) => b.clone(),
            (&Kset::Set(ref a),&Kset::Set(ref b)) => {
                let mut ret = HashSet::<&(u64,usize)>::from_iter(a.iter().chain(b.iter()))
                    .iter().cloned().cloned().collect::<Vec<(u64,usize)>>();
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

    fn widen(&self,s: &Self) -> Self {
        s.clone()
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
                    HashSet::<&(u64,usize)>::from_iter(a.iter())
                    .is_superset(&HashSet::from_iter(b.iter())),
                    _ => false,
            }
        }
    }

    fn extract(&self,size: usize,offset: usize) -> Self {
        match self {
            &Kset::Join => Kset::Join,
            &Kset::Meet => Kset::Meet,
            &Kset::Set(ref v) =>
                Kset::Set(v.iter().map(|&(v,_)| {
                    ((v >> offset) % (1 << (size - 1)),size)
                }).collect::<Vec<_>>()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {
        Statement,Operation,
        ControlFlowTarget,Function,ControlFlowGraph,
        Guard,
        Lvalue,Rvalue,
        Mnemonic,
        ssa_convertion,
        BasicBlock,
        approximate,
        results,
    };

    use graph_algos::{
        MutableGraphTrait,
    };
    use std::borrow::Cow;

    /*
     * a = 10
     * b = 0
     * c = 4
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
        let a_var = Lvalue::Variable{ name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b_var = Lvalue::Variable{ name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c_var = Lvalue::Variable{ name: Cow::Borrowed("c"), size: 32, subscript: None };
        let x_var = Lvalue::Variable{ name: Cow::Borrowed("x"), size: 32, subscript: None };
        let flag = Lvalue::Variable{ name: Cow::Borrowed("flag"), size: 1, subscript: None };
        let bb0 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(0..1,"assign a".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Move(Rvalue::new_u32(10)), assignee: a_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(1..2,"assign b".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Move(Rvalue::new_u32(0)), assignee: b_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(2..3,"assign c".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Move(Rvalue::new_u32(4)), assignee: c_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(3..4,"cmp c".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Equal(c_var.clone().into(),Rvalue::new_u32(1)), assignee: flag.clone()}].iter()).ok().unwrap()]);

        let bb1 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(4..5,"add a and 5".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Add(a_var.clone().into(),Rvalue::new_u32(5)), assignee: a_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(5..6,"mul a and c".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Add(a_var.clone().into(),c_var.clone().into()), assignee: b_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(6..7,"assign c".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Move(Rvalue::new_u32(2)), assignee: c_var.clone()}].iter()).ok().unwrap()]);
        let bb2 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(7..8,"dec a".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Subtract(a_var.clone().into(),Rvalue::new_u32(1)), assignee: a_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(8..9,"add 3 to b".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Add(b_var.clone().into(),Rvalue::new_u32(3)), assignee: b_var.clone()}].iter()).ok().unwrap(),
                                       Mnemonic::new(9..10,"assign c".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Move(Rvalue::new_u32(3)), assignee: c_var.clone()}].iter()).ok().unwrap()]);
        let bb3 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(10..11,"add a and b".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::Add(a_var.clone().into(),b_var.clone().into()), assignee: x_var.clone()}].iter()).ok().unwrap()]);
        let bb4 = BasicBlock::from_vec(vec![
                                       Mnemonic::new(11..12,"cmp a".to_string(),"".to_string(),vec![].iter(),vec![
                                                     Statement{ op: Operation::LessOrEqualSigned(a_var.clone().into(),Rvalue::new_u32(0)), assignee: flag.clone()}].iter()).ok().unwrap()]);


        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));

        let g = Guard::from_flag(&flag.into()).ok().unwrap();

        cfg.add_edge(g.clone(),v0,v1);
        cfg.add_edge(g.negation(),v0,v4);
        cfg.add_edge(g.negation(),v4,v2);
        cfg.add_edge(g.clone(),v4,v3);
        cfg.add_edge(Guard::always(),v2,v4);
        cfg.add_edge(Guard::always(),v1,v3);

        let mut func = Function::new("func".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        assert!(ssa_convertion(&mut func).is_ok());

        let vals = approximate::<Kset>(&func,&HashMap::new()).ok().unwrap();
        let res = results::<Kset>(&func,&vals);

        assert_eq!(res[&(Cow::Borrowed("a"),32)],Kset::Join);
        assert_eq!(res[&(Cow::Borrowed("b"),32)],Kset::Join);
        assert_eq!(res[&(Cow::Borrowed("c"),32)],Kset::Set(vec![(2,32),(3,32),(4,32)]));
        assert_eq!(res[&(Cow::Borrowed("x"),32)],Kset::Join);
    }

    #[test]
    fn bit_extract() {
        let p_var = Lvalue::Variable{ name: Cow::Borrowed("p"), size: 22, subscript: None };
        let r1_var = Lvalue::Variable{ name: Cow::Borrowed("r1"), size: 8, subscript: None };
        let r2_var = Lvalue::Variable{ name: Cow::Borrowed("r2"), size: 8, subscript: None };
        let next = Lvalue::Variable{ name: Cow::Borrowed("R30:R31"), size: 22, subscript: None };
        let bb0 = BasicBlock::from_vec(vec![
            Mnemonic::new(0..1,"init r1".to_string(),"".to_string(),vec![].iter(),vec![
                Statement{ op: Operation::Move(Rvalue::new_u8(7)), assignee: r1_var.clone()}].iter()).ok().unwrap(),
            Mnemonic::new(1..2,"init r2".to_string(),"".to_string(),vec![].iter(),vec![
                Statement{ op: Operation::Move(Rvalue::new_u8(88)), assignee: r2_var.clone()}].iter()).ok().unwrap()
        ]);
        let bb1 = BasicBlock::from_vec(vec![
            Mnemonic::new(2..3,"zext r1".to_string(),"".to_string(),vec![].iter(),vec![
                Statement{ op: Operation::ZeroExtend(22,r1_var.clone().into()), assignee: p_var.clone()}].iter()).ok().unwrap(),
            Mnemonic::new(3..4,"mov r2".to_string(),"".to_string(),vec![].iter(),vec![
                Statement{ op: Operation::Select(8,p_var.clone().into(),r2_var.clone().into()), assignee: p_var.clone()}].iter()).ok().unwrap(),
            Mnemonic::new(4..5,"mov 0".to_string(),"".to_string(),vec![].iter(),vec![
                Statement{ op: Operation::Select(16,p_var.clone().into(),Rvalue::Constant{ value: 0, size: 6 }), assignee: p_var.clone()}].iter()).ok().unwrap(),
            Mnemonic::new(5..6,"mov next".to_string(),"".to_string(),vec![].iter(),vec![
                Statement{ op: Operation::Move(p_var.clone().into()), assignee: next.clone()}].iter()).ok().unwrap()
        ]);
        let mut cfg = ControlFlowGraph::new();
        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));

        cfg.add_edge(Guard::always(),v0,v1);

        let mut func = Function::new("func".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        assert!(ssa_convertion(&mut func).is_ok());

        let vals = approximate::<Kset>(&func,&HashMap::new()).ok().unwrap();

        for i in vals {
            println!("{:?}",i);
        }
    }
}
