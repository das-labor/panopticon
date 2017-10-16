/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

use panopticon_core::{ControlFlowTarget, ControlFlowRef, Guard, Lvalue, Operation, Rvalue, Statement};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use DataFlow;

use petgraph::Direction;
use petgraph::visit::{Walker, EdgeRef, DfsPostOrder};

/// Computes the set of killed (VarKill) and upward exposed variables (UEvar) for each basic block
/// in `func`. Returns (VarKill,UEvar).
pub(crate) fn liveness_sets<Function: DataFlow>(func: &Function) -> (HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>, HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>) {
    let mut uevar = HashMap::<ControlFlowRef, HashSet<&str>>::new();
    let mut varkill = HashMap::<ControlFlowRef, HashSet<Cow<'static, str>>>::new();
    // don't allocate the postorder
    let ord = DfsPostOrder::new(func.cfg(), func.entry_point_ref()).iter(func.cfg());
    let cfg = func.cfg();

    // init UEVar and VarKill sets
    for vx in ord {
        let uev = uevar.entry(vx).or_insert(HashSet::<&str>::new());
        let vk = varkill.entry(vx).or_insert(HashSet::<Cow<'static, str>>::new());

        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.node_weight(vx.into()) {
            for mne in bb.mnemonics.iter() {
                for rv in mne.operands.iter() {
                    if let &Rvalue::Variable { ref name, .. } = rv {
                        if !vk.contains(name) {
                            uev.insert(name);
                        }
                    }
                }

                for instr in mne.instructions.iter() {
                    let &Statement { ref op, ref assignee } = instr;

                    if let &Operation::Phi(_) = op {
;
                    } else {
                        for &rv in op.operands().iter() {
                            if let &Rvalue::Variable { ref name, .. } = rv {
                                if !vk.contains(name) {
                                    uev.insert(name);
                                }
                            }
                        }

                        if let &Lvalue::Variable { ref name, .. } = assignee {
                            vk.insert(name.clone());
                        }
                    }
                }
            }
        }

        for e in cfg.edges_directed(vx.into(), Direction::Outgoing) {
            if let &Guard::Predicate { flag: Rvalue::Variable { ref name, .. }, .. } = e.weight() {
                if !vk.contains(name) {
                    uev.insert(name);
                }
            }
        }
    }

    (varkill, HashMap::from_iter(uevar.iter().map(|(&k, v)| (k, HashSet::from_iter(v.iter().map(|x| Cow::Owned(x.to_string())))))))
}

/// Computes for each basic block in `func` the set of live variables using simple fixed point
/// iteration.
pub(crate) fn liveness<Function: DataFlow>(func: &Function) -> HashMap<ControlFlowRef, HashSet<Cow<'static, str>>> {
    let (varkill, uevar) = liveness_sets(func);
    let mut liveout = HashMap::<ControlFlowRef, HashSet<&str>>::new();
    let ord = DfsPostOrder::new(func.cfg(), func.entry_point_ref()).iter(func.cfg());
    let cfg = func.cfg();

    for vx in ord.clone() {
        if let Some(&ControlFlowTarget::Resolved(_)) = cfg.node_weight(vx) {
            liveout.insert(vx, HashSet::<&str>::new());
        }
    }

    // compute LiveOut sets
    let mut fixpoint = false;
    while !fixpoint {
        let mut new_liveout = HashMap::<ControlFlowRef, HashSet<&str>>::new();

        fixpoint = true;
        for vx in ord.clone() {
            let mut s = HashSet::<&str>::new();

            if let Some(&ControlFlowTarget::Resolved(_)) = cfg.node_weight(vx) {
                for e in cfg.edges_directed(vx, Direction::Outgoing) {
                    let m = e.target();

                    for x in uevar[&m].iter() {
                        s.insert(x);
                    }

                    if let Some(&ControlFlowTarget::Resolved(_)) = cfg.node_weight(m) {
                        for x in liveout[&m].iter() {
                            if !varkill[&m].contains(*x) {
                                s.insert(x);
                            }
                        }
                    }
                }

                if liveout[&vx] != s {
                    fixpoint = false;
                }
                new_liveout.insert(vx, s);
            }
        }
        liveout = new_liveout;
    }
    HashMap::from_iter(liveout.iter().map(|(&k, v)| (k, HashSet::from_iter(v.iter().map(|x| Cow::Owned(x.to_string()))))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use panopticon_core::{BasicBlock, ControlFlowGraph, ControlFlowTarget, Function, Guard, Lvalue, Mnemonic, Operation, Rvalue, Statement, Region};
    use ssa::{phi_functions};
    use std::borrow::Cow;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use DataFlow;

    #[test]
    fn live() {
        let i = Lvalue::Variable { name: Cow::Borrowed("i"), size: 32, subscript: None };
        let s = Lvalue::Variable { name: Cow::Borrowed("s"), size: 32, subscript: None };
        let x = Lvalue::Variable { name: Cow::Borrowed("x"), size: 1, subscript: None };
        let mne0 = Mnemonic::new(
            0..1,
            "b0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::new_u32(1)), assignee: i.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne1 = Mnemonic::new(
            1..2,
            "b1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessUnsigned(i.clone().into(), Rvalue::new_u32(1)),
                    assignee: x.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne2 = Mnemonic::new(
            2..3,
            "b2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::new_u32(0)), assignee: s.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne30 = Mnemonic::new(
            3..4,
            "b3.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(i.clone().into(), s.clone().into()),
                    assignee: s.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne31 = Mnemonic::new(
            4..5,
            "b3.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(i.clone().into(), i.clone().into()),
                    assignee: i.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne32 = Mnemonic::new(
            5..6,
            "b3.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessUnsigned(i.clone().into(), Rvalue::new_u32(1)),
                    assignee: x.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne4 = Mnemonic::new(
            6..7,
            "b4".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Move(s.clone().into()),
                    assignee: Lvalue::Undefined,
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne1]);
        let bb2 = BasicBlock::from_vec(vec![mne2]);
        let bb3 = BasicBlock::from_vec(vec![mne30, mne31, mne32]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_node(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_node(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_node(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_node(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_node(ControlFlowTarget::Resolved(bb4));

        let g = Guard::from_flag(&x.clone().into()).ok().unwrap();

        cfg.add_edge( v0,  v1, Guard::always());
        cfg.add_edge( v1,  v2, g.negation());
        cfg.add_edge( v1,  v3, g.clone());
        cfg.add_edge( v2,  v3, Guard::always());
        cfg.add_edge( v3,  v1, g.negation());
        cfg.add_edge( v3,  v4, g.clone());
        let mut func = Function::undefined(0, None, &Region::undefined("ram".to_owned(), 100), None);

        *func.cfg_mut() = cfg;
        func.set_entry_point_ref(v0);

        let all = HashSet::from_iter(vec![Cow::Borrowed("i"), Cow::Borrowed("s")]);
        let (vk, ue) = liveness_sets(&func);

        assert_eq!(ue.len(), 5);
        assert_eq!(ue.get(&v0), Some(&HashSet::new()));
        assert_eq!(
            ue.get(&v1),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("i")]))
        );
        assert_eq!(ue.get(&v2), Some(&HashSet::new()));
        assert_eq!(
            ue.get(&v3),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("i"), Cow::Borrowed("s")]))
        );
        assert_eq!(
            ue.get(&v4),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("s")]))
        );

        assert_eq!(vk.len(), 5);
        assert_eq!(
            vk.get(&v0),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("i")]))
        );
        assert_eq!(
            vk.get(&v1),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("x")]))
        );
        assert_eq!(
            vk.get(&v2),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("s")]))
        );
        assert_eq!(
            vk.get(&v3),
            Some(&HashSet::from_iter(vec![Cow::Borrowed("x"), Cow::Borrowed("i"), Cow::Borrowed("s")]))
        );
        assert_eq!(vk.get(&v4), Some(&HashSet::new()));
        let res = liveness(&func);

        assert_eq!(res.len(), 5);
        assert_eq!(res.get(&v0), Some(&all));
        assert_eq!(res.get(&v1), Some(&all));
        assert_eq!(res.get(&v2), Some(&all));
        assert_eq!(res.get(&v3), Some(&all));
        assert_eq!(res.get(&v4), Some(&HashSet::new()));
    }

    #[test]
    fn phi() {
        let a = Lvalue::Variable { name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b = Lvalue::Variable { name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c = Lvalue::Variable { name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d = Lvalue::Variable { name: Cow::Borrowed("d"), size: 32, subscript: None };
        let y = Lvalue::Variable { name: Cow::Borrowed("y"), size: 32, subscript: None };
        let z = Lvalue::Variable { name: Cow::Borrowed("z"), size: 32, subscript: None };
        let i = Lvalue::Variable { name: Cow::Borrowed("i"), size: 32, subscript: None };
        let f = Lvalue::Variable { name: Cow::Borrowed("f"), size: 1, subscript: None };

        let mne0 = Mnemonic::new(
            0..1,
            "b0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::new_u32(1)), assignee: i.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne10 = Mnemonic::new(
            1..2,
            "b1.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne11 = Mnemonic::new(
            2..3,
            "b1.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne12 = Mnemonic::new(
            3..4,
            "b1.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessUnsigned(a.clone().into(), c.clone().into()),
                    assignee: f.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();

        let mne20 = Mnemonic::new(
            4..5,
            "b2.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne21 = Mnemonic::new(
            5..6,
            "b2.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne22 = Mnemonic::new(
            6..7,
            "b2.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne30 = Mnemonic::new(
            7..8,
            "b3.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(a.clone().into(), b.clone().into()),
                    assignee: y.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne31 = Mnemonic::new(
            8..9,
            "b3.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(c.clone().into(), d.clone().into()),
                    assignee: z.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne32 = Mnemonic::new(
            9..10,
            "b3.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(i.clone().into(), i.clone().into()),
                    assignee: i.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne33 = Mnemonic::new(
            10..11,
            "b3.3".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessOrEqualUnsigned(i.clone().into(), Rvalue::new_u32(100)),
                    assignee: f.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();

        let mne4 = Mnemonic::new(
            11..12,
            "b4".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![].iter(),
        )
                .ok()
                .unwrap();

        let mne50 = Mnemonic::new(
            12..13,
            "b5.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne51 = Mnemonic::new(
            13..14,
            "b5.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne52 = Mnemonic::new(
            14..15,
            "b5.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessOrEqualUnsigned(a.clone().into(), d.clone().into()),
                    assignee: f.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();

        let mne6 = Mnemonic::new(
            15..16,
            "b6".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne7 = Mnemonic::new(
            16..17,
            "b7".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne8 = Mnemonic::new(
            17..18,
            "b8".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne10, mne11, mne12]);
        let bb2 = BasicBlock::from_vec(vec![mne20, mne21, mne22]);
        let bb3 = BasicBlock::from_vec(vec![mne30, mne31, mne32, mne33]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let bb5 = BasicBlock::from_vec(vec![mne50, mne51, mne52]);
        let bb6 = BasicBlock::from_vec(vec![mne6]);
        let bb7 = BasicBlock::from_vec(vec![mne7]);
        let bb8 = BasicBlock::from_vec(vec![mne8]);
        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_node(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_node(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_node(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_node(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_node(ControlFlowTarget::Resolved(bb4));
        let v5 = cfg.add_node(ControlFlowTarget::Resolved(bb5));
        let v6 = cfg.add_node(ControlFlowTarget::Resolved(bb6));
        let v7 = cfg.add_node(ControlFlowTarget::Resolved(bb7));
        let v8 = cfg.add_node(ControlFlowTarget::Resolved(bb8));

        cfg.add_edge( v0,  v1, Guard::always());

        let g1 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge( v1,  v2, g1.clone());
        cfg.add_edge( v1,  v5, g1.negation());

        cfg.add_edge( v2,  v3, Guard::always());

        let g3 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge( v3,  v1, g3.clone());
        cfg.add_edge( v3,  v4, g3.negation());

        let g5 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge( v5,  v6, g5.clone());
        cfg.add_edge( v5,  v8, g5.negation());

        cfg.add_edge( v6,  v7, Guard::always());
        cfg.add_edge( v7,  v3, Guard::always());
        cfg.add_edge( v8,  v7, Guard::always());

        let mut func = Function::undefined(0, None, &Region::undefined("ram".to_owned(), 100), None);

        *func.cfg_mut() = cfg;
        func.set_entry_point_ref(v0);
        let (globals, usage) = func.global_names();
        assert!(phi_functions(&mut func, &globals, &usage).is_ok());

        let a0 = Lvalue::Variable { name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b0 = Lvalue::Variable { name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c0 = Lvalue::Variable { name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d0 = Lvalue::Variable { name: Cow::Borrowed("d"), size: 32, subscript: None };
        let i0 = Lvalue::Variable { name: Cow::Borrowed("i"), size: 32, subscript: None };

        // bb0
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v0) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb1
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v1) {
            assert_eq!(bb.mnemonics[0].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[1].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[2].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[3].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[4].opcode, "__phi".to_string());
            assert!(bb.mnemonics[5].opcode != "__phi".to_string());

            let mut tmp = HashSet::<Lvalue>::new();
            tmp.insert(bb.mnemonics[0].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[1].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[2].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[3].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[4].instructions[0].assignee.clone());

            assert_eq!(tmp.len(), 5);
            assert!(tmp.contains(&a0));
            assert!(tmp.contains(&b0));
            assert!(tmp.contains(&c0));
            assert!(tmp.contains(&d0));
            assert!(tmp.contains(&i0));
        } else {
            unreachable!()
        }

        // bb2
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v2) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb3
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v3) {
            assert_eq!(bb.mnemonics[0].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[1].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[2].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[3].opcode, "__phi".to_string());
            assert!(bb.mnemonics[4].opcode != "__phi".to_string());

            let mut tmp = HashSet::<Lvalue>::new();
            tmp.insert(bb.mnemonics[0].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[1].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[2].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[3].instructions[0].assignee.clone());

            assert_eq!(tmp.len(), 4);
            assert!(tmp.contains(&a0));
            assert!(tmp.contains(&b0));
            assert!(tmp.contains(&c0));
            assert!(tmp.contains(&d0));
        } else {
            unreachable!()
        }

        // bb4
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v4) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb5
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v5) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb6
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v6) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb7
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v7) {
            assert_eq!(bb.mnemonics[0].opcode, "__phi".to_string());
            assert_eq!(bb.mnemonics[1].opcode, "__phi".to_string());
            assert!(bb.mnemonics[2].opcode != "__phi".to_string());

            let mut tmp = HashSet::<Lvalue>::new();
            tmp.insert(bb.mnemonics[0].instructions[0].assignee.clone());
            tmp.insert(bb.mnemonics[1].instructions[0].assignee.clone());

            assert_eq!(tmp.len(), 2);
            assert!(tmp.contains(&c0));
            assert!(tmp.contains(&d0));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn rename() {
        let a = Lvalue::Variable { name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b = Lvalue::Variable { name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c = Lvalue::Variable { name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d = Lvalue::Variable { name: Cow::Borrowed("d"), size: 32, subscript: None };
        let y = Lvalue::Variable { name: Cow::Borrowed("y"), size: 32, subscript: None };
        let z = Lvalue::Variable { name: Cow::Borrowed("z"), size: 32, subscript: None };
        let i = Lvalue::Variable { name: Cow::Borrowed("i"), size: 32, subscript: None };
        let f = Lvalue::Variable { name: Cow::Borrowed("f"), size: 1, subscript: None };

        let mne0 = Mnemonic::new(
            0..1,
            "b0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::new_u32(1)), assignee: i.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne10 = Mnemonic::new(
            1..2,
            "b1.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne11 = Mnemonic::new(
            2..3,
            "b1.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne12 = Mnemonic::new(
            3..4,
            "b1.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessUnsigned(a.clone().into(), c.clone().into()),
                    assignee: f.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();

        let mne20 = Mnemonic::new(
            4..5,
            "b2.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne21 = Mnemonic::new(
            5..6,
            "b2.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne22 = Mnemonic::new(
            6..7,
            "b2.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne30 = Mnemonic::new(
            7..8,
            "b3.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(a.clone().into(), b.clone().into()),
                    assignee: y.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne31 = Mnemonic::new(
            8..9,
            "b3.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(c.clone().into(), d.clone().into()),
                    assignee: z.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne32 = Mnemonic::new(
            9..10,
            "b3.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::Add(i.clone().into(), i.clone().into()),
                    assignee: i.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();
        let mne33 = Mnemonic::new(
            10..11,
            "b3.3".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessOrEqualUnsigned(i.clone().into(), Rvalue::new_u32(100)),
                    assignee: f.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();

        let mne4 = Mnemonic::new(
            11..12,
            "b4".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![].iter(),
        )
                .ok()
                .unwrap();

        let mne50 = Mnemonic::new(
            12..13,
            "b5.0".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne51 = Mnemonic::new(
            13..14,
            "b5.1".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter(),
        )
                .ok()
                .unwrap();
        let mne52 = Mnemonic::new(
            14..15,
            "b5.2".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![
                Statement {
                    op: Operation::LessOrEqualUnsigned(a.clone().into(), d.clone().into()),
                    assignee: f.clone(),
                },
            ]
                    .iter(),
        )
                .ok()
                .unwrap();

        let mne6 = Mnemonic::new(
            15..16,
            "b6".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne7 = Mnemonic::new(
            16..17,
            "b7".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let mne8 = Mnemonic::new(
            17..18,
            "b8".to_string(),
            "".to_string(),
            vec![].iter(),
            vec![Statement { op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter(),
        )
                .ok()
                .unwrap();

        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne10, mne11, mne12]);
        let bb2 = BasicBlock::from_vec(vec![mne20, mne21, mne22]);
        let bb3 = BasicBlock::from_vec(vec![mne30, mne31, mne32, mne33]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let bb5 = BasicBlock::from_vec(vec![mne50, mne51, mne52]);
        let bb6 = BasicBlock::from_vec(vec![mne6]);
        let bb7 = BasicBlock::from_vec(vec![mne7]);
        let bb8 = BasicBlock::from_vec(vec![mne8]);
        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_node(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_node(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_node(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_node(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_node(ControlFlowTarget::Resolved(bb4));
        let v5 = cfg.add_node(ControlFlowTarget::Resolved(bb5));
        let v6 = cfg.add_node(ControlFlowTarget::Resolved(bb6));
        let v7 = cfg.add_node(ControlFlowTarget::Resolved(bb7));
        let v8 = cfg.add_node(ControlFlowTarget::Resolved(bb8));

        cfg.add_edge( v0,  v1, Guard::always());

        let g1 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge( v1,  v2, g1.clone());
        cfg.add_edge( v1,  v5, g1.negation());

        cfg.add_edge( v2,  v3, Guard::always());

        let g3 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge( v3,  v1, g3.clone());
        cfg.add_edge( v3,  v4, g3.negation());

        let g5 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge( v5,  v6, g5.clone());
        cfg.add_edge( v5,  v8, g5.negation());

        cfg.add_edge( v6,  v7, Guard::always());
        cfg.add_edge( v7,  v3, Guard::always());
        cfg.add_edge( v8,  v7, Guard::always());

        let mut func = Function::undefined(0, None, &Region::undefined("ram".to_owned(), 100), None);

        *func.cfg_mut() = cfg;
        func.set_entry_point_ref(v0);
        assert!(func.ssa_conversion().is_ok());

        for v in func.cfg().node_indices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cfg().node_weight(v) {
                bb.execute(
                    |i| {
                        if let Lvalue::Variable { subscript, .. } = i.assignee {
                            assert!(subscript.is_some());
                        }

                        for op in i.op.operands() {
                            if let &Rvalue::Variable { subscript, .. } = op {
                                assert!(subscript.is_some());
                            }
                        }
                    }
                );
            } else {
                unreachable!()
            }
        }
    }
}
