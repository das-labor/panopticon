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

use panopticon_core::{ControlFlowRef, ControlFlowTarget, Function, Guard, Lvalue, Operation, Rvalue, Statement};
use panopticon_graph_algos::{GraphTrait, IncidenceGraphTrait};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Computes the set of killed (VarKill) and upward exposed variables (UEvar) for each basic block
/// in `func`. Returns (VarKill,UEvar).
pub fn liveness_sets(func: &Function) -> (HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>, HashMap<ControlFlowRef, HashSet<Cow<'static, str>>>) {
    let mut uevar = HashMap::<ControlFlowRef, HashSet<&str>>::new();
    let mut varkill = HashMap::<ControlFlowRef, HashSet<Cow<'static, str>>>::new();
    let ord = func.postorder();
    let cfg = &func.cflow_graph;

    // init UEVar and VarKill sets
    for &vx in ord.iter() {
        let uev = uevar.entry(vx).or_insert(HashSet::<&str>::new());
        let vk = varkill.entry(vx).or_insert(HashSet::<Cow<'static, str>>::new());

        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
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

        for e in cfg.out_edges(vx) {
            if let Some(&Guard::Predicate { flag: Rvalue::Variable { ref name, .. }, .. }) = cfg.edge_label(e) {
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
pub fn liveness(func: &Function) -> HashMap<ControlFlowRef, HashSet<Cow<'static, str>>> {
    let (varkill, uevar) = liveness_sets(func);
    let mut liveout = HashMap::<ControlFlowRef, HashSet<&str>>::new();
    let ord = func.postorder();
    let cfg = &func.cflow_graph;

    for &vx in ord.iter() {
        if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(vx) {
            liveout.insert(vx, HashSet::<&str>::new());
        }
    }

    // compute LiveOut sets
    let mut fixpoint = false;
    while !fixpoint {
        let mut new_liveout = HashMap::<ControlFlowRef, HashSet<&str>>::new();

        fixpoint = true;
        for &vx in ord.iter() {
            let mut s = HashSet::<&str>::new();

            if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(vx) {
                for e in cfg.out_edges(vx) {
                    let m = cfg.target(e);

                    for x in uevar[&m].iter() {
                        s.insert(x);
                    }

                    if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(m) {
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
    use panopticon_core::{BasicBlock, ControlFlowGraph, ControlFlowTarget, Function, Guard, Lvalue, Mnemonic, Operation, Rvalue, Statement};
    use panopticon_graph_algos::{GraphTrait, MutableGraphTrait, VertexListGraphTrait};
    use ssa::{phi_functions, rename_variables};
    use std::borrow::Cow;
    use std::collections::HashSet;
    use std::iter::FromIterator;

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

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));

        let g = Guard::from_flag(&x.clone().into()).ok().unwrap();

        cfg.add_edge(Guard::always(), v0, v1);
        cfg.add_edge(g.negation(), v1, v2);
        cfg.add_edge(g.clone(), v1, v3);
        cfg.add_edge(Guard::always(), v2, v3);
        cfg.add_edge(g.negation(), v3, v1);
        cfg.add_edge(g.clone(), v3, v4);

        let mut func = Function::new("test".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

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

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));
        let v5 = cfg.add_vertex(ControlFlowTarget::Resolved(bb5));
        let v6 = cfg.add_vertex(ControlFlowTarget::Resolved(bb6));
        let v7 = cfg.add_vertex(ControlFlowTarget::Resolved(bb7));
        let v8 = cfg.add_vertex(ControlFlowTarget::Resolved(bb8));

        cfg.add_edge(Guard::always(), v0, v1);

        let g1 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g1.clone(), v1, v2);
        cfg.add_edge(g1.negation(), v1, v5);

        cfg.add_edge(Guard::always(), v2, v3);

        let g3 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g3.clone(), v3, v1);
        cfg.add_edge(g3.negation(), v3, v4);

        let g5 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g5.clone(), v5, v6);
        cfg.add_edge(g5.negation(), v5, v8);

        cfg.add_edge(Guard::always(), v6, v7);
        cfg.add_edge(Guard::always(), v7, v3);
        cfg.add_edge(Guard::always(), v8, v7);

        let mut func = Function::new("test".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        assert!(phi_functions(&mut func).is_ok());

        let a0 = Lvalue::Variable { name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b0 = Lvalue::Variable { name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c0 = Lvalue::Variable { name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d0 = Lvalue::Variable { name: Cow::Borrowed("d"), size: 32, subscript: None };
        let i0 = Lvalue::Variable { name: Cow::Borrowed("i"), size: 32, subscript: None };

        // bb0
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v0) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb1
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v1) {
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
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v2) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb3
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v3) {
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
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v4) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb5
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v5) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb6
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v6) {
            assert!(bb.mnemonics[0].opcode != "__phi".to_string());
        } else {
            unreachable!()
        }

        // bb7
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v7) {
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

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));
        let v5 = cfg.add_vertex(ControlFlowTarget::Resolved(bb5));
        let v6 = cfg.add_vertex(ControlFlowTarget::Resolved(bb6));
        let v7 = cfg.add_vertex(ControlFlowTarget::Resolved(bb7));
        let v8 = cfg.add_vertex(ControlFlowTarget::Resolved(bb8));

        cfg.add_edge(Guard::always(), v0, v1);

        let g1 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g1.clone(), v1, v2);
        cfg.add_edge(g1.negation(), v1, v5);

        cfg.add_edge(Guard::always(), v2, v3);

        let g3 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g3.clone(), v3, v1);
        cfg.add_edge(g3.negation(), v3, v4);

        let g5 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g5.clone(), v5, v6);
        cfg.add_edge(g5.negation(), v5, v8);

        cfg.add_edge(Guard::always(), v6, v7);
        cfg.add_edge(Guard::always(), v7, v3);
        cfg.add_edge(Guard::always(), v8, v7);

        let mut func = Function::new("test".to_string(), "ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        assert!(phi_functions(&mut func).is_ok());
        assert!(rename_variables(&mut func).is_ok());

        for v in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v) {
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
