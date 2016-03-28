/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015 Kai Michaelis
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

use std::collections::{
    HashMap,
    HashSet,
};
use std::iter::FromIterator;
use graph_algos::{
    GraphTrait,
    IncidenceGraphTrait,
    MutableGraphTrait,
    BidirectionalGraphTrait,
    EdgeListGraphTrait,
    VertexListGraphTrait,
};
use graph_algos::dominator::{
    immediate_dominator,
    dominance_frontiers,
};
use mnemonic::Mnemonic;
use guard::Guard;
use function::{
    Function,
    ControlFlowTarget,
    ControlFlowRef,
    ControlFlowGraph,
};
use instr::{
    Operation,
    Instr,
};
use value::{
    Rvalue,
    Lvalue,
};

/// returns (varkill,uevar)
pub fn liveness_sets(func: &Function) ->  (HashMap<ControlFlowRef,HashSet<String>>,HashMap<ControlFlowRef,HashSet<String>>) {
    let mut uevar = HashMap::<ControlFlowRef,HashSet<&str>>::new();
    let mut varkill = HashMap::<ControlFlowRef,HashSet<String>>::new();
    let ord = func.postorder();
    let cfg = &func.cflow_graph;

    // init UEVar and VarKill sets
    for &vx in ord.iter() {
        let uev = uevar.entry(vx).or_insert(HashSet::<&str>::new());
        let vk = varkill.entry(vx).or_insert(HashSet::<String>::new());

        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
            bb.execute(|instr| {
                let &Instr{ ref op, ref assignee } = instr;

                if let &Operation::Phi(_) = op {
                    ;
                } else {
                    for &rv in op.operands().iter() {
                        if let &Rvalue::Variable{ ref name,.. } = rv {
                            if !vk.contains(name) {
                                uev.insert(name);
                            }
                        }
                    }

                    if let &Lvalue::Variable{ ref name,.. } = assignee {
                        vk.insert(name.clone());
                    }
                }
            });
        }

        for e in cfg.out_edges(vx) {
            let g = cfg.edge_label(e).unwrap();
            for op in g.relation.operands() {
                if let &Rvalue::Variable{ ref name,.. } = op {
                    if !vk.contains(name) {
                        uev.insert(name);
                    }
                }
            }
        }
    }

    (varkill,HashMap::from_iter(uevar.iter().map(|(&k,v)| {
        (k,HashSet::from_iter(v.iter().map(|x| x.to_string()))) })))
}

pub fn liveness(func: &Function) ->  HashMap<ControlFlowRef,HashSet<String>> {
    let (varkill,uevar) = liveness_sets(func);
    let mut liveout = HashMap::<ControlFlowRef,HashSet<&str>>::new();
    let ord = func.postorder();
    let cfg = &func.cflow_graph;

    for &vx in ord.iter() {
        if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(vx) {
            liveout.insert(vx,HashSet::<&str>::new());
        }
    }

    // compute LiveOut sets
    let mut fixpoint = false;
    while !fixpoint {
        let mut new_liveout = HashMap::<ControlFlowRef,HashSet<&str>>::new();

        fixpoint = true;
        for &vx in ord.iter() {
            let mut s = HashSet::<&str>::new();

            if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(vx) {
                for e in cfg.out_edges(vx) {
                    let m = cfg.target(e);

                    for x in uevar[&m].iter() { s.insert(x); }

                    if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(m) {
                        for x in liveout[&m].iter() {
                            if !varkill[&m].contains(&x.to_string()) {
                                s.insert(x);
                            }
                        }
                    }
                }

                if liveout[&vx] != s {
                    fixpoint = false;
                }

                new_liveout.insert(vx,s);
            }
        }

        liveout = new_liveout;
    }

    HashMap::from_iter(liveout.iter().map(|(&k,v)| {
        (k,HashSet::from_iter(v.iter().map(|x| x.to_string()))) }))
}

pub fn type_check(func: &Function) -> HashMap<String,u16> {
    let mut ret = HashMap::<String,u16>::new();
    let cfg = &func.cflow_graph;
    fn check_or_set_len(v: &Rvalue, ret: &mut HashMap<String,u16>) {
        match v {
            &Rvalue::Variable{ ref name, ref width, .. } =>
                if *ret.entry(name.clone()).or_insert(*width) != *width {
                    panic!("Type check failed!");
                },
            &Rvalue::Memory{ ref offset, .. } =>
                check_or_set_len(&*offset,ret),
            _ => {}
        }
    }

    for vx in cfg.vertices() {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
            bb.execute(|instr| {
                let ops = instr.op.operands();
                match ops.len() {
                    0 => panic!("Operation w/o arguments"),
                    _ => for o in ops.iter() {
                        check_or_set_len(o,&mut ret);
                    }
                }

                check_or_set_len(&instr.assignee.to_rv(),&mut ret);
            });
        }
    }

    for ed in cfg.edges() {
        if let Some(ref g) = cfg.edge_label(ed) {
            for o in g.relation.operands().iter() {
                check_or_set_len(o,&mut ret);
            }
        }
    }

    ret
}

/// returns globals,usage
pub fn global_names(func: &Function) -> (HashSet<String>,HashMap<String,HashSet<ControlFlowRef>>) {
    let (varkill,uevar) = liveness_sets(func);
    let mut usage = HashMap::<String,HashSet<ControlFlowRef>>::new();
    let mut globals = HashSet::<String>::new();

    for (_,uev) in uevar {
        for v in uev {
            globals.insert(v);
        }
    }

    for (vx,vk) in varkill {
        for v in vk {
            usage.entry(v.clone()).or_insert(HashSet::new()).insert(vx);
        }
    }

    (globals,usage)
}

pub fn phi_functions(func: &mut Function) {
    assert!(func.entry_point.is_some());

    let (globals,usage) = global_names(func);
    let lens = type_check(func);
    let mut cfg = &mut func.cflow_graph;

    // initalize all variables
    if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(func.entry_point.unwrap()) {
        let pos = bb.area.start;
        let instrs = globals.iter().map(|nam| Instr{
            op: Operation::Nop(Rvalue::Undefined),
            assignee: Lvalue::Variable{ width: lens[nam], name: nam.clone(), subscript: None }}
        ).collect::<Vec<_>>();

        let mne = Mnemonic::new(
            pos..pos,
            "__init".to_string(),
            "".to_string(),
            vec![].iter(),
            instrs.iter());

        bb.mnemonics.insert(0,mne);
    } else {
        unreachable!("Entry point is unresolved!");
    }

    let idom = immediate_dominator(func.entry_point.unwrap(),cfg);
    let df = dominance_frontiers(&idom,cfg);
    let mut phis = HashSet::<(&String,ControlFlowRef)>::new();

    for v in globals.iter() {
        let mut worklist = if let Some(wl) = usage.get(v) { wl.clone() } else { HashSet::new() };

        while !worklist.is_empty() {
            let w = worklist.iter().next().unwrap().clone();

            worklist.remove(&w);
            for d in df[&w].iter() {
                let arg_num = cfg.in_edges(*d).filter_map(|p| usage.get(v).map(|b| b.contains(&cfg.source(p)))).count();
                if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(*d) {
                    if !phis.contains(&(v,*d)) {

                        let pos = bb.area.start;
                        let mne = Mnemonic::new(
                            pos..pos,
                            "__phi".to_string(),
                            "".to_string(),
                            vec![].iter(),
                            vec![Instr{
                                op: Operation::Phi(vec![Rvalue::Variable{ width: lens[v], name: v.clone(), subscript: None };arg_num]),
                                assignee: Lvalue::Variable{ width: lens[v], name: v.clone(), subscript: None }}].iter()
                        );

                        bb.mnemonics.insert(0,mne);
                        phis.insert((v,*d));
                        worklist.insert(*d);
                    }
                }
            }
        }
    }
}

pub fn rename_variables(func: &mut Function) {
    let (globals,_) = global_names(func);
    let mut cfg = &mut func.cflow_graph;
    let mut stack = HashMap::<String,Vec<u32>>::from_iter(globals.iter().map(|x| (x.clone(),Vec::new())));
    let mut counter = HashMap::<String,u32>::new();
    let idom = immediate_dominator(func.entry_point.unwrap(),cfg);
    fn new_name(n: &String, counter: &mut HashMap<String,u32>, stack: &mut HashMap<String,Vec<u32>>) -> u32 {
        let i = *counter.entry(n.clone()).or_insert(0);

        counter.get_mut(n).map(|x| *x += 1);
        stack.entry(n.clone()).or_insert(Vec::new()).push(i);

        i
    }
    fn rename(b: ControlFlowRef, counter: &mut HashMap<String,u32>, stack: &mut HashMap<String,Vec<u32>>, cfg: &mut ControlFlowGraph, idom: &HashMap<ControlFlowRef,ControlFlowRef>) {
        if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(b) {
            bb.rewrite(|i| match i {
                &mut Instr{ op: Operation::Phi(_), assignee: Lvalue::Variable{ ref name, ref mut subscript,.. } } =>
                    *subscript = Some(new_name(name,counter,stack)),
                _ => {},
            });
            bb.rewrite(|i| {
                let &mut Instr{ ref mut op, ref mut assignee } = i;

                if let &mut Operation::Phi(_) = op {} else {
                    for o in op.operands_mut() {
                        if let &mut Rvalue::Variable{ ref name, ref mut subscript,.. } = o {
                            *subscript = stack[name].last().cloned();
                        }
                    }

                    if let &mut Lvalue::Variable{ ref name, ref mut subscript,.. } = assignee {
                        *subscript = Some(new_name(name,counter,stack));
                    }
                }
            });
        }

        let mut succ = cfg.out_edges(b).collect::<Vec<_>>();
        succ.sort();

        for s in succ {
            if let Some(&mut Guard{ ref mut relation }) = cfg.edge_label_mut(s) {
                for rv in relation.operands_mut() {
                    if let &mut Rvalue::Variable{ ref name, ref mut subscript,.. } = rv {
                        *subscript = stack[name].last().cloned();
                    }
                }
            }

            let v = cfg.target(s);
            match cfg.vertex_label_mut(v) {
                Some(&mut ControlFlowTarget::Resolved(ref mut bb)) => {
                    bb.rewrite(|i| match i {
                        &mut Instr{ op: Operation::Phi(ref mut ops),.. } =>
                            for o in ops.iter_mut() {
                                if let &mut Rvalue::Variable{ ref name, ref mut subscript,.. } = o {
                                    if subscript.is_none() {
                                       *subscript = stack[name].last().cloned();
                                       break;
                                    }
                                }
                            },
                        _ => {},
                    });
                }
                Some(&mut ControlFlowTarget::Unresolved(Rvalue::Variable{ ref name, ref mut subscript,.. })) =>
                    *subscript = stack[name].last().cloned(),
                _ => {}
            }
        }

        for (k,_) in idom.iter().filter(|&(_,&v)| v == b) {
            if *k != b {
                rename(*k,counter,stack,cfg,idom);
            }
        }

        if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(b) {
            bb.execute(|i| match i {
                &Instr{ assignee: Lvalue::Variable{ ref name,.. },.. } => {
                    stack.get_mut(name).map(|x| x.pop());
                },
                _ => {}
            });
        }
    }

    rename(func.entry_point.unwrap(),&mut counter,&mut stack,cfg,&idom);
}

pub fn ssa_convertion(func: &mut Function) {
    phi_functions(func);
    rename_variables(func);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use graph_algos::{
        GraphTrait,
        VertexListGraphTrait,
        MutableGraphTrait,
    };
    use mnemonic::Mnemonic;
    use guard::Guard;
    use function::{
        Function,
        ControlFlowTarget,
        ControlFlowGraph,
    };
    use instr::{
        Operation,
        Instr,
    };
    use value::{
        Rvalue,
        Lvalue,
    };
    use basic_block::BasicBlock;

    #[test]
    fn live() {
        let i = Lvalue::Variable{ name: "i".to_string(), width: 32, subscript: None };
        let s = Lvalue::Variable{ name: "s".to_string(), width: 32, subscript: None };
        let x = Lvalue::Variable{ name: "x".to_string(), width: 1, subscript: None };
        let mne0 = Mnemonic::new(0..1,"b0".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::Nop(Rvalue::Constant(1)), assignee: i.clone() }].iter());
        let mne1 = Mnemonic::new(1..2,"b1".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::IntLess(i.to_rv(),Rvalue::Constant(1)), assignee: x.clone() }].iter());
        let mne2 = Mnemonic::new(2..3,"b2".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::Nop(Rvalue::Constant(0)), assignee: s.clone() }].iter());
        let mne30 = Mnemonic::new(3..4,"b3.0".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::IntAdd(i.to_rv(),s.to_rv()), assignee: s.clone() }].iter());
        let mne31 = Mnemonic::new(4..5,"b3.1".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::IntAdd(i.to_rv(),i.to_rv()), assignee: i.clone() }].iter());
        let mne32 = Mnemonic::new(5..6,"b3.2".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::IntLess(i.to_rv(),Rvalue::Constant(1)), assignee: x.clone() }].iter());
        let mne4 = Mnemonic::new(6..7,"b4".to_string(),"".to_string(),vec![].iter(),vec![Instr{ op: Operation::Nop(s.to_rv()), assignee: Lvalue::Undefined }].iter());
        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne1]);
        let bb2 = BasicBlock::from_vec(vec![mne2]);
        let bb3 = BasicBlock::from_vec(vec![mne30,mne31,mne32]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let mut cfg = ControlFlowGraph::new();

        let v0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let v1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let v2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));
        let v3 = cfg.add_vertex(ControlFlowTarget::Resolved(bb3));
        let v4 = cfg.add_vertex(ControlFlowTarget::Resolved(bb4));

        cfg.add_edge(Guard::always(),v0,v1);
        cfg.add_edge(Guard::eq(&Rvalue::Constant(0),&i.to_rv()),v1,v2);
        cfg.add_edge(Guard::neq(&Rvalue::Constant(0),&i.to_rv()),v1,v3);
        cfg.add_edge(Guard::always(),v2,v3);
        cfg.add_edge(Guard::eq(&Rvalue::Constant(0),&i.to_rv()),v3,v1);
        cfg.add_edge(Guard::neq(&Rvalue::Constant(0),&i.to_rv()),v3,v4);

        let mut func = Function::new("test".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        let all = HashSet::from_iter(vec!["i".to_string(),"s".to_string()]);
        let (vk,ue) = liveness_sets(&func);

        assert_eq!(ue.len(), 5);
        assert_eq!(ue.get(&v0), Some(&HashSet::new()));
        assert_eq!(ue.get(&v1), Some(&HashSet::from_iter(vec!["i".to_string()])));
        assert_eq!(ue.get(&v2), Some(&HashSet::new()));
        assert_eq!(ue.get(&v3), Some(&HashSet::from_iter(vec!["i".to_string(),"s".to_string()])));
        assert_eq!(ue.get(&v4), Some(&HashSet::from_iter(vec!["s".to_string()])));

        assert_eq!(vk.len(), 5);
        assert_eq!(vk.get(&v0), Some(&HashSet::from_iter(vec!["i".to_string()])));
        assert_eq!(vk.get(&v1), Some(&HashSet::from_iter(vec!["x".to_string()])));
        assert_eq!(vk.get(&v2), Some(&HashSet::from_iter(vec!["s".to_string()])));
        assert_eq!(vk.get(&v3), Some(&HashSet::from_iter(vec!["x".to_string(),"i".to_string(),"s".to_string()])));
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
        let a = Lvalue::Variable{ name: "a".to_string(), width: 32, subscript: None };
        let b = Lvalue::Variable{ name: "b".to_string(), width: 32, subscript: None };
        let c = Lvalue::Variable{ name: "c".to_string(), width: 32, subscript: None };
        let d = Lvalue::Variable{ name: "d".to_string(), width: 32, subscript: None };
        let y = Lvalue::Variable{ name: "y".to_string(), width: 32, subscript: None };
        let z = Lvalue::Variable{ name: "z".to_string(), width: 32, subscript: None };
        let i = Lvalue::Variable{ name: "i".to_string(), width: 32, subscript: None };

        let mne0 = Mnemonic::new(0..1,"b0".to_string(),"".to_string(),vec![].iter(),vec![
                                Instr{ op: Operation::Nop(Rvalue::Constant(1)), assignee: i.clone() }].iter());

        let mne10 = Mnemonic::new(1..2,"b1.0".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: a.clone() }].iter());
        let mne11 = Mnemonic::new(2..3,"b1.1".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c.clone() }].iter());

        let mne20 = Mnemonic::new(3..4,"b2.0".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: b.clone() }].iter());
        let mne21 = Mnemonic::new(4..5,"b2.1".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c.clone() }].iter());
        let mne22 = Mnemonic::new(5..6,"b2.2".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: d.clone() }].iter());

        let mne30 = Mnemonic::new(6..7,"b3.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::IntAdd(a.to_rv(),b.to_rv()), assignee: y.clone() }].iter());
        let mne31 = Mnemonic::new(7..8,"b3.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::IntAdd(c.to_rv(),d.to_rv()), assignee: z.clone() }].iter());
        let mne32 = Mnemonic::new(8..9,"b3.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::IntAdd(i.to_rv(),i.to_rv()), assignee: i.clone() }].iter());

        let mne4 = Mnemonic::new(9..10,"b4".to_string(),"".to_string(),vec![].iter(),vec![].iter());

        let mne50 = Mnemonic::new(10..11,"b5.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: a.clone() }].iter());
        let mne51 = Mnemonic::new(11..12,"b5.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: d.clone() }].iter());

        let mne6 = Mnemonic::new(12..13,"b6".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: d.clone() }].iter());

        let mne7 = Mnemonic::new(13..14,"b7".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: b.clone() }].iter());

        let mne8 = Mnemonic::new(14..15,"b8".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c.clone() }].iter());

        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne10,mne11]);
        let bb2 = BasicBlock::from_vec(vec![mne20,mne21,mne22]);
        let bb3 = BasicBlock::from_vec(vec![mne30,mne31,mne32]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let bb5 = BasicBlock::from_vec(vec![mne50,mne51]);
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

        cfg.add_edge(Guard::always(),v0,v1);
        cfg.add_edge(Guard::unsi_less(&a.to_rv(),&c.to_rv()),v1,v2);
        cfg.add_edge(Guard::unsi_geq(&a.to_rv(),&c.to_rv()),v1,v5);
        cfg.add_edge(Guard::always(),v2,v3);
        cfg.add_edge(Guard::unsi_leq(&i.to_rv(),&Rvalue::Constant(100)),v3,v1);
        cfg.add_edge(Guard::unsi_gt(&i.to_rv(),&Rvalue::Constant(100)),v3,v4);
        cfg.add_edge(Guard::unsi_leq(&a.to_rv(),&d.to_rv()),v5,v6);
        cfg.add_edge(Guard::unsi_gt(&a.to_rv(),&d.to_rv()),v5,v8);
        cfg.add_edge(Guard::always(),v6,v7);
        cfg.add_edge(Guard::always(),v7,v3);
        cfg.add_edge(Guard::always(),v8,v7);

        let mut func = Function::new("test".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        phi_functions(&mut func);

        let a0 = Lvalue::Variable{ name: "a".to_string(), width: 32, subscript: None };
        let b0 = Lvalue::Variable{ name: "b".to_string(), width: 32, subscript: None };
        let c0 = Lvalue::Variable{ name: "c".to_string(), width: 32, subscript: None };
        let d0 = Lvalue::Variable{ name: "d".to_string(), width: 32, subscript: None };
        let i0 = Lvalue::Variable{ name: "i".to_string(), width: 32, subscript: None };

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
        let a = Lvalue::Variable{ name: "a".to_string(), width: 32, subscript: None };
        let b = Lvalue::Variable{ name: "b".to_string(), width: 32, subscript: None };
        let c = Lvalue::Variable{ name: "c".to_string(), width: 32, subscript: None };
        let d = Lvalue::Variable{ name: "d".to_string(), width: 32, subscript: None };
        let y = Lvalue::Variable{ name: "y".to_string(), width: 32, subscript: None };
        let z = Lvalue::Variable{ name: "z".to_string(), width: 32, subscript: None };
        let i = Lvalue::Variable{ name: "i".to_string(), width: 32, subscript: None };

        let mne0 = Mnemonic::new(0..1,"b0".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Constant(1)), assignee: i.clone() }].iter());

        let mne10 = Mnemonic::new(1..2,"b1.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: a.clone() }].iter());
        let mne11 = Mnemonic::new(2..3,"b1.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c.clone() }].iter());

        let mne20 = Mnemonic::new(3..4,"b2.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: b.clone() }].iter());
        let mne21 = Mnemonic::new(4..5,"b2.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c.clone() }].iter());
        let mne22 = Mnemonic::new(5..6,"b2.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: d.clone() }].iter());

        let mne30 = Mnemonic::new(6..7,"b3.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::IntAdd(a.to_rv(),b.to_rv()), assignee: y.clone() }].iter());
        let mne31 = Mnemonic::new(7..8,"b3.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::IntAdd(c.to_rv(),d.to_rv()), assignee: z.clone() }].iter());
        let mne32 = Mnemonic::new(8..9,"b3.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::IntAdd(i.to_rv(),i.to_rv()), assignee: i.clone() }].iter());

        let mne4 = Mnemonic::new(9..10,"b4".to_string(),"".to_string(),vec![].iter(),vec![].iter());

        let mne50 = Mnemonic::new(10..11,"b5.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: a.clone() }].iter());
        let mne51 = Mnemonic::new(11..12,"b5.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: d.clone() }].iter());

        let mne6 = Mnemonic::new(12..13,"b6".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: d.clone() }].iter());

        let mne7 = Mnemonic::new(13..14,"b7".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: b.clone() }].iter());

        let mne8 = Mnemonic::new(14..15,"b8".to_string(),"".to_string(),vec![].iter(),vec![
                                 Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: c.clone() }].iter());

        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne10,mne11]);
        let bb2 = BasicBlock::from_vec(vec![mne20,mne21,mne22]);
        let bb3 = BasicBlock::from_vec(vec![mne30,mne31,mne32]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let bb5 = BasicBlock::from_vec(vec![mne50,mne51]);
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

        cfg.add_edge(Guard::always(),v0,v1);
        cfg.add_edge(Guard::unsi_less(&a.to_rv(),&c.to_rv()),v1,v2);
        cfg.add_edge(Guard::unsi_geq(&a.to_rv(),&c.to_rv()),v1,v5);
        cfg.add_edge(Guard::always(),v2,v3);
        cfg.add_edge(Guard::unsi_leq(&i.to_rv(),&Rvalue::Constant(100)),v3,v1);
        cfg.add_edge(Guard::unsi_gt(&i.to_rv(),&Rvalue::Constant(100)),v3,v4);
        cfg.add_edge(Guard::unsi_leq(&a.to_rv(),&d.to_rv()),v5,v6);
        cfg.add_edge(Guard::unsi_gt(&a.to_rv(),&d.to_rv()),v5,v8);
        cfg.add_edge(Guard::always(),v6,v7);
        cfg.add_edge(Guard::always(),v7,v3);
        cfg.add_edge(Guard::always(),v8,v7);

        let mut func = Function::new("test".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        phi_functions(&mut func);
        rename_variables(&mut func);

        for v in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v) {
                bb.execute(|i| {
                    if let Lvalue::Variable{ subscript,.. } = i.assignee {
                        assert!(subscript.is_some());
                    }

                    for op in i.op.operands() {
                        if let &Rvalue::Variable{ subscript,.. } = op {
                            assert!(subscript.is_some());
                        }
                    }
                });
            } else {
                unreachable!()
            }
        }
    }
}
