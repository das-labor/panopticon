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
use std::cmp::max;
use std::iter::FromIterator;
use std::borrow::Cow;
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
use {
    Mnemonic,
    Guard,
    Function,
    ControlFlowTarget,
    ControlFlowRef,
    ControlFlowEdge,
    ControlFlowGraph,
    Operation,
    Statement,
    Rvalue,
    Lvalue,
};

/// returns (varkill,uevar)
pub fn liveness_sets(func: &Function) ->  (HashMap<ControlFlowRef,HashSet<Cow<'static,str>>>,HashMap<ControlFlowRef,HashSet<Cow<'static,str>>>) {
    let mut uevar = HashMap::<ControlFlowRef,HashSet<&str>>::new();
    let mut varkill = HashMap::<ControlFlowRef,HashSet<Cow<'static,str>>>::new();
    let ord = func.postorder();
    let cfg = &func.cflow_graph;

    // init UEVar and VarKill sets
    for &vx in ord.iter() {
        let uev = uevar.entry(vx).or_insert(HashSet::<&str>::new());
        let vk = varkill.entry(vx).or_insert(HashSet::<Cow<'static,str>>::new());

        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
            bb.execute(|instr| {
                let &Statement{ ref op, ref assignee } = instr;

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
            if let Some(&Guard::Predicate{ flag: Rvalue::Variable{ ref name,.. },.. }) = cfg.edge_label(e) {
                if !vk.contains(name) {
                    uev.insert(name);
                }
            }
        }
    }

    (varkill,HashMap::from_iter(uevar.iter().map(|(&k,v)| {
        (k,HashSet::from_iter(v.iter().map(|x| Cow::Owned(x.to_string())))) })))
}

pub fn liveness(func: &Function) ->  HashMap<ControlFlowRef,HashSet<Cow<'static,str>>> {
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
                            if !varkill[&m].contains(*x) {
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
        (k,HashSet::from_iter(v.iter().map(|x| Cow::Owned(x.to_string())))) }))
}

pub fn type_check(func: &Function) -> HashMap<Cow<'static,str>,usize> {
    let mut ret = HashMap::<Cow<'static,str>,usize>::new();
    let cfg = &func.cflow_graph;
    fn set_len(v: &Rvalue, ret: &mut HashMap<Cow<'static,str>,usize>) {
        match v {
            &Rvalue::Variable{ ref name, ref size, .. } => {
                let val = *max(ret.get(name).unwrap_or(&0),size);
                ret.insert(name.clone(),val);
            },
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
                        set_len(o,&mut ret);
                    }
                }

                set_len(&instr.assignee.clone().into(),&mut ret);
            });
        }
    }

    for ed in cfg.edges() {
        if let Some(&Guard::Predicate{ ref flag,.. }) = cfg.edge_label(ed) {
            set_len(flag,&mut ret);
        }
    }

    ret
}

/// returns globals,usage
pub fn global_names(func: &Function) -> (HashSet<Cow<'static,str>>,HashMap<Cow<'static,str>,HashSet<ControlFlowRef>>) {
    let (varkill,uevar) = liveness_sets(func);
    let mut usage = HashMap::<Cow<'static,str>,HashSet<ControlFlowRef>>::new();
    let mut globals = HashSet::<Cow<'static,str>>::new();

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
        let instrs = globals.iter().map(|nam| Statement{
            op: Operation::Move(Rvalue::Undefined),
            assignee: Lvalue::Variable{ size: lens[nam], name: nam.clone(), subscript: None }}
        ).collect::<Vec<_>>();

        let mne = Mnemonic::new(
            pos..pos,
            "__init".to_string(),
            "".to_string(),
            vec![].iter(),
            instrs.iter()).ok().unwrap();

        bb.mnemonics.insert(0,mne);
    } else {
        unreachable!("Entry point is unresolved!");
    }

    let idom = immediate_dominator(func.entry_point.unwrap(),cfg);
    let df = dominance_frontiers(&idom,cfg);
    let mut phis = HashSet::<(&Cow<'static,str>,ControlFlowRef)>::new();

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
                            vec![Statement{
                                op: Operation::Phi(vec![Rvalue::Variable{ offset: 0, size: lens[v], name: v.clone(), subscript: None };arg_num]),
                                assignee: Lvalue::Variable{ size: lens[v], name: v.clone(), subscript: None }}].iter()
                        ).ok().unwrap();

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
    let mut stack = HashMap::<Cow<'static,str>,Vec<usize>>::from_iter(globals.iter().map(|x| (x.clone(),Vec::new())));
    let mut counter = HashMap::<Cow<'static,str>,usize>::new();
    let idom = immediate_dominator(func.entry_point.unwrap(),cfg);
    fn new_name(n: &Cow<'static,str>, counter: &mut HashMap<Cow<'static,str>,usize>, stack: &mut HashMap<Cow<'static,str>,Vec<usize>>) -> usize {
        let i = *counter.entry(n.clone()).or_insert(0);

        counter.get_mut(n).map(|x| *x += 1);
        stack.entry(n.clone()).or_insert(Vec::new()).push(i);

        i
    }
    fn rename(b: ControlFlowRef, counter: &mut HashMap<Cow<'static,str>,usize>, stack: &mut HashMap<Cow<'static,str>,Vec<usize>>, cfg: &mut ControlFlowGraph, idom: &HashMap<ControlFlowRef,ControlFlowRef>) {
        if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(b) {
            bb.rewrite(|i| match i {
                &mut Statement{ op: Operation::Phi(_), assignee: Lvalue::Variable{ ref name, ref mut subscript,.. } } =>
                    *subscript = Some(new_name(name,counter,stack)),
                _ => {},
            });

            for mne in bb.mnemonics.iter_mut() {
                if mne.opcode != "__phi" {
                    for o in mne.operands.iter_mut() {
                        if let &mut Rvalue::Variable{ ref name, ref mut subscript,.. } = o {
                            *subscript = stack.get(name).and_then(|x| x.last()).cloned();
                            if !stack.contains_key(name) {
                                println!("Mnemonic {} has {} as arguments but does not read it",mne.opcode,name);
                            }
                        }
                    }

                    for i in mne.instructions.iter_mut() {
                        let &mut Statement{ ref mut op, ref mut assignee } = i;

                        if let &mut Operation::Phi(_) = op {
                            unreachable!("Phi instruction outside __phi mnemonic");
                        } else {
                            for o in op.operands_mut() {
                                if let &mut Rvalue::Variable{ ref name, ref mut subscript,.. } = o {
                                    *subscript = stack[name].last().cloned();
                                }
                            }

                            if let &mut Lvalue::Variable{ ref name, ref mut subscript,.. } = assignee {
                                *subscript = Some(new_name(name,counter,stack));
                            }
                        }
                    }
                }
            }
        }

        let mut succ = cfg.out_edges(b).collect::<Vec<_>>();
        succ.sort();

        for s in succ {
            if let Some(&mut Guard::Predicate{ flag: Rvalue::Variable{ ref name, ref mut subscript,.. },.. }) = cfg.edge_label_mut(s) {
                *subscript = stack[name].last().cloned();
            }

            let v = cfg.target(s);
            match cfg.vertex_label_mut(v) {
                Some(&mut ControlFlowTarget::Resolved(ref mut bb)) => {
                    bb.rewrite(|i| match i {
                        &mut Statement{ op: Operation::Phi(ref mut ops),.. } =>
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
                &Statement{ assignee: Lvalue::Variable{ ref name,.. },.. } => {
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

pub fn flag_operations(func: &Function) -> HashMap<ControlFlowEdge,Operation<Rvalue>> {
    let mut ret = HashMap::new();

    for e in func.cflow_graph.edges() {
        if !ret.contains_key(&e) {
            if let Some(&Guard::Predicate{ ref flag,.. }) = func.cflow_graph.edge_label(e) {
                let maybe_bb = func.cflow_graph.vertex_label(func.cflow_graph.source(e));
                if let Some(&ControlFlowTarget::Resolved(ref bb)) = maybe_bb {
                    let mut maybe_stmt = None;
                    bb.execute(|s| {
                        let a: Rvalue = s.assignee.clone().into();
                        if a == *flag {
                            match s.op {
                                Operation::Equal(_,_) | Operation::LessOrEqualUnsigned(_,_) |
                                Operation::LessOrEqualSigned(_,_) | Operation::LessUnsigned(_,_) |
                                Operation::LessSigned(_,_) => maybe_stmt = Some(s.op.clone()),
                                _ => {}
                            }
                        }
                    });

                    if maybe_stmt.is_some() {
                        ret.insert(e.clone(),maybe_stmt.unwrap());
                    }
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::borrow::Cow;
    use graph_algos::{
        GraphTrait,
        VertexListGraphTrait,
        MutableGraphTrait,
    };
    use {
        Mnemonic,
        Guard,
        Function,
        ControlFlowTarget,
        ControlFlowGraph,
        Operation,
        Statement,
        Rvalue,
        Lvalue,
        BasicBlock,
    };

    #[test]
    fn live() {
        let i = Lvalue::Variable{ name: Cow::Borrowed("i"), size: 32, subscript: None };
        let s = Lvalue::Variable{ name: Cow::Borrowed("s"), size: 32, subscript: None };
        let x = Lvalue::Variable{ name: Cow::Borrowed("x"), size: 1, subscript: None };
        let mne0 = Mnemonic::new(0..1,"b0".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::new_u32(1)), assignee: i.clone() }].iter()).ok().unwrap();
        let mne1 = Mnemonic::new(1..2,"b1".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::LessUnsigned(i.clone().into(),Rvalue::new_u32(1)), assignee: x.clone() }].iter()).ok().unwrap();
        let mne2 = Mnemonic::new(2..3,"b2".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::new_u32(0)), assignee: s.clone() }].iter()).ok().unwrap();
        let mne30 = Mnemonic::new(3..4,"b3.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(i.clone().into(),s.clone().into()), assignee: s.clone() }].iter()).ok().unwrap();
        let mne31 = Mnemonic::new(4..5,"b3.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(i.clone().into(),i.clone().into()), assignee: i.clone() }].iter()).ok().unwrap();
        let mne32 = Mnemonic::new(5..6,"b3.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessUnsigned(i.clone().into(),Rvalue::new_u32(1)), assignee: x.clone() }].iter()).ok().unwrap();
        let mne4 = Mnemonic::new(6..7,"b4".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(s.clone().into()), assignee: Lvalue::Undefined }].iter()).ok().unwrap();
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

        let g = Guard::from_flag(&x.clone().into()).ok().unwrap();

        cfg.add_edge(Guard::always(),v0,v1);
        cfg.add_edge(g.negation(),v1,v2);
        cfg.add_edge(g.clone(),v1,v3);
        cfg.add_edge(Guard::always(),v2,v3);
        cfg.add_edge(g.negation(),v3,v1);
        cfg.add_edge(g.clone(),v3,v4);

        let mut func = Function::new("test".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        let all = HashSet::from_iter(vec![Cow::Borrowed("i"),Cow::Borrowed("s")]);
        let (vk,ue) = liveness_sets(&func);

        assert_eq!(ue.len(), 5);
        assert_eq!(ue.get(&v0), Some(&HashSet::new()));
        assert_eq!(ue.get(&v1), Some(&HashSet::from_iter(vec![Cow::Borrowed("i")])));
        assert_eq!(ue.get(&v2), Some(&HashSet::new()));
        assert_eq!(ue.get(&v3), Some(&HashSet::from_iter(vec![Cow::Borrowed("i"),Cow::Borrowed("s")])));
        assert_eq!(ue.get(&v4), Some(&HashSet::from_iter(vec![Cow::Borrowed("s")])));

        assert_eq!(vk.len(), 5);
        assert_eq!(vk.get(&v0), Some(&HashSet::from_iter(vec![Cow::Borrowed("i")])));
        assert_eq!(vk.get(&v1), Some(&HashSet::from_iter(vec![Cow::Borrowed("x")])));
        assert_eq!(vk.get(&v2), Some(&HashSet::from_iter(vec![Cow::Borrowed("s")])));
        assert_eq!(vk.get(&v3), Some(&HashSet::from_iter(vec![Cow::Borrowed("x"),Cow::Borrowed("i"),Cow::Borrowed("s")])));
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
        let a = Lvalue::Variable{ name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b = Lvalue::Variable{ name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c = Lvalue::Variable{ name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d = Lvalue::Variable{ name: Cow::Borrowed("d"), size: 32, subscript: None };
        let y = Lvalue::Variable{ name: Cow::Borrowed("y"), size: 32, subscript: None };
        let z = Lvalue::Variable{ name: Cow::Borrowed("z"), size: 32, subscript: None };
        let i = Lvalue::Variable{ name: Cow::Borrowed("i"), size: 32, subscript: None };
        let f = Lvalue::Variable{ name: Cow::Borrowed("f"), size: 1, subscript: None };

        let mne0 = Mnemonic::new(0..1,"b0".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::new_u32(1)), assignee: i.clone() }].iter()).ok().unwrap();

        let mne10 = Mnemonic::new(1..2,"b1.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter()).ok().unwrap();
        let mne11 = Mnemonic::new(2..3,"b1.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter()).ok().unwrap();
        let mne12 = Mnemonic::new(3..4,"b1.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessUnsigned(a.clone().into(),c.clone().into()), assignee: f.clone() }].iter()).ok().unwrap();

        let mne20 = Mnemonic::new(4..5,"b2.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter()).ok().unwrap();
        let mne21 = Mnemonic::new(5..6,"b2.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter()).ok().unwrap();
        let mne22 = Mnemonic::new(6..7,"b2.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter()).ok().unwrap();

        let mne30 = Mnemonic::new(7..8,"b3.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(a.clone().into(),b.clone().into()), assignee: y.clone() }].iter()).ok().unwrap();
        let mne31 = Mnemonic::new(8..9,"b3.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(c.clone().into(),d.clone().into()), assignee: z.clone() }].iter()).ok().unwrap();
        let mne32 = Mnemonic::new(9..10,"b3.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(i.clone().into(),i.clone().into()), assignee: i.clone() }].iter()).ok().unwrap();
        let mne33 = Mnemonic::new(10..11,"b3.3".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessOrEqualUnsigned(i.clone().into(),Rvalue::new_u32(100)), assignee: f.clone() }].iter()).ok().unwrap();

        let mne4 = Mnemonic::new(11..12,"b4".to_string(),"".to_string(),vec![].iter(),vec![].iter()).ok().unwrap();

        let mne50 = Mnemonic::new(12..13,"b5.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter()).ok().unwrap();
        let mne51 = Mnemonic::new(13..14,"b5.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter()).ok().unwrap();
        let mne52 = Mnemonic::new(14..15,"b5.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessOrEqualUnsigned(a.clone().into(),d.clone().into()), assignee: f.clone() }].iter()).ok().unwrap();

        let mne6 = Mnemonic::new(15..16,"b6".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter()).ok().unwrap();

        let mne7 = Mnemonic::new(16..17,"b7".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter()).ok().unwrap();

        let mne8 = Mnemonic::new(17..18,"b8".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter()).ok().unwrap();

        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne10,mne11,mne12]);
        let bb2 = BasicBlock::from_vec(vec![mne20,mne21,mne22]);
        let bb3 = BasicBlock::from_vec(vec![mne30,mne31,mne32,mne33]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let bb5 = BasicBlock::from_vec(vec![mne50,mne51,mne52]);
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

        let g1 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g1.clone(),v1,v2);
        cfg.add_edge(g1.negation(),v1,v5);

        cfg.add_edge(Guard::always(),v2,v3);

        let g3 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g3.clone(),v3,v1);
        cfg.add_edge(g3.negation(),v3,v4);

        let g5 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g5.clone(),v5,v6);
        cfg.add_edge(g5.negation(),v5,v8);

        cfg.add_edge(Guard::always(),v6,v7);
        cfg.add_edge(Guard::always(),v7,v3);
        cfg.add_edge(Guard::always(),v8,v7);

        let mut func = Function::new("test".to_string(),"ram".to_string());

        func.cflow_graph = cfg;
        func.entry_point = Some(v0);

        phi_functions(&mut func);

        let a0 = Lvalue::Variable{ name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b0 = Lvalue::Variable{ name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c0 = Lvalue::Variable{ name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d0 = Lvalue::Variable{ name: Cow::Borrowed("d"), size: 32, subscript: None };
        let i0 = Lvalue::Variable{ name: Cow::Borrowed("i"), size: 32, subscript: None };

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
        let a = Lvalue::Variable{ name: Cow::Borrowed("a"), size: 32, subscript: None };
        let b = Lvalue::Variable{ name: Cow::Borrowed("b"), size: 32, subscript: None };
        let c = Lvalue::Variable{ name: Cow::Borrowed("c"), size: 32, subscript: None };
        let d = Lvalue::Variable{ name: Cow::Borrowed("d"), size: 32, subscript: None };
        let y = Lvalue::Variable{ name: Cow::Borrowed("y"), size: 32, subscript: None };
        let z = Lvalue::Variable{ name: Cow::Borrowed("z"), size: 32, subscript: None };
        let i = Lvalue::Variable{ name: Cow::Borrowed("i"), size: 32, subscript: None };
        let f = Lvalue::Variable{ name: Cow::Borrowed("f"), size: 1, subscript: None };

        let mne0 = Mnemonic::new(0..1,"b0".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::new_u32(1)), assignee: i.clone() }].iter()).ok().unwrap();

        let mne10 = Mnemonic::new(1..2,"b1.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter()).ok().unwrap();
        let mne11 = Mnemonic::new(2..3,"b1.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter()).ok().unwrap();
        let mne12 = Mnemonic::new(3..4,"b1.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessUnsigned(a.clone().into(),c.clone().into()), assignee: f.clone() }].iter()).ok().unwrap();

        let mne20 = Mnemonic::new(4..5,"b2.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter()).ok().unwrap();
        let mne21 = Mnemonic::new(5..6,"b2.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter()).ok().unwrap();
        let mne22 = Mnemonic::new(6..7,"b2.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter()).ok().unwrap();

        let mne30 = Mnemonic::new(7..8,"b3.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(a.clone().into(),b.clone().into()), assignee: y.clone() }].iter()).ok().unwrap();
        let mne31 = Mnemonic::new(8..9,"b3.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(c.clone().into(),d.clone().into()), assignee: z.clone() }].iter()).ok().unwrap();
        let mne32 = Mnemonic::new(9..10,"b3.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Add(i.clone().into(),i.clone().into()), assignee: i.clone() }].iter()).ok().unwrap();
        let mne33 = Mnemonic::new(10..11,"b3.3".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessOrEqualUnsigned(i.clone().into(),Rvalue::new_u32(100)), assignee: f.clone() }].iter()).ok().unwrap();

        let mne4 = Mnemonic::new(11..12,"b4".to_string(),"".to_string(),vec![].iter(),vec![].iter()).ok().unwrap();

        let mne50 = Mnemonic::new(12..13,"b5.0".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: a.clone() }].iter()).ok().unwrap();
        let mne51 = Mnemonic::new(13..14,"b5.1".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter()).ok().unwrap();
        let mne52 = Mnemonic::new(14..15,"b5.2".to_string(),"".to_string(),vec![].iter(),vec![
                                  Statement{ op: Operation::LessOrEqualUnsigned(a.clone().into(),d.clone().into()), assignee: f.clone() }].iter()).ok().unwrap();

        let mne6 = Mnemonic::new(15..16,"b6".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::Undefined), assignee: d.clone() }].iter()).ok().unwrap();

        let mne7 = Mnemonic::new(16..17,"b7".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::Undefined), assignee: b.clone() }].iter()).ok().unwrap();

        let mne8 = Mnemonic::new(17..18,"b8".to_string(),"".to_string(),vec![].iter(),vec![
                                 Statement{ op: Operation::Move(Rvalue::Undefined), assignee: c.clone() }].iter()).ok().unwrap();

        let bb0 = BasicBlock::from_vec(vec![mne0]);
        let bb1 = BasicBlock::from_vec(vec![mne10,mne11,mne12]);
        let bb2 = BasicBlock::from_vec(vec![mne20,mne21,mne22]);
        let bb3 = BasicBlock::from_vec(vec![mne30,mne31,mne32,mne33]);
        let bb4 = BasicBlock::from_vec(vec![mne4]);
        let bb5 = BasicBlock::from_vec(vec![mne50,mne51,mne52]);
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

        let g1 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g1.clone(),v1,v2);
        cfg.add_edge(g1.negation(),v1,v5);

        cfg.add_edge(Guard::always(),v2,v3);

        let g3 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g3.clone(),v3,v1);
        cfg.add_edge(g3.negation(),v3,v4);

        let g5 = Guard::from_flag(&f.clone().into()).ok().unwrap();
        cfg.add_edge(g5.clone(),v5,v6);
        cfg.add_edge(g5.negation(),v5,v8);

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
