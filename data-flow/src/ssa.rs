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

use liveness_sets;
use panopticon_core::{ControlFlowEdge, ControlFlowGraph, ControlFlowRef, ControlFlowTarget, Function, Guard, Lvalue, Mnemonic, Operation, Result, Rvalue,
                      Statement};
use panopticon_graph_algos::{BidirectionalGraphTrait, EdgeListGraphTrait, GraphTrait, IncidenceGraphTrait, MutableGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::dominator::{dominance_frontiers, immediate_dominator};
use std::borrow::Cow;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Does a simple sanity check on all RREIL statements in `func`, returns every variable name
/// found and its maximal size in bits.
pub fn type_check(func: &Function) -> Result<HashMap<Cow<'static, str>, usize>> {
   let mut ret = HashMap::<Cow<'static, str>, usize>::new();
   let cfg = &func.cflow_graph;
   fn set_len(v: &Rvalue, ret: &mut HashMap<Cow<'static, str>, usize>) {
      match v {
         &Rvalue::Variable { ref name, ref size, .. } => {
            let val = *max(ret.get(name).unwrap_or(&0), size);
            ret.insert(name.clone(), val);
         }
         _ => {}
      }
   }

   for vx in cfg.vertices() {
      if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
         for mne in bb.mnemonics.iter() {
            for o in mne.operands.iter() {
               set_len(o, &mut ret);
            }

            for instr in mne.instructions.iter() {
               let ops = instr.op.operands();
               match ops.len() {
                  0 => return Err("Operation w/o arguments".into()),
                  _ => {
                     for o in ops.iter() {
                        set_len(o, &mut ret);
                     }
                  }
               }

               set_len(&instr.assignee.clone().into(), &mut ret);
            }
         }
      }
   }

   for ed in cfg.edges() {
      if let Some(&Guard::Predicate { ref flag, .. }) = cfg.edge_label(ed) {
         set_len(flag, &mut ret);
      }
   }

   Ok(ret)
}

/// Computes the set of gloable variables in `func` and their points of usage. Globales are
/// variables that are used in multiple basic blocks. Returns (Globals,Usage).
pub fn global_names(func: &Function) -> (HashSet<Cow<'static, str>>, HashMap<Cow<'static, str>, HashSet<ControlFlowRef>>) {
   let (varkill, uevar) = liveness_sets(func);
   let mut usage = HashMap::<Cow<'static, str>, HashSet<ControlFlowRef>>::new();
   let mut globals = HashSet::<Cow<'static, str>>::new();

   for (_, uev) in uevar {
      for v in uev {
         globals.insert(v);
      }
   }

   for (vx, vk) in varkill {
      for v in vk {
         usage.entry(v.clone()).or_insert(HashSet::new()).insert(vx);
      }
   }

   (globals, usage)
}

/// Inserts SSA Phi functions at junction points in the control flow graph of `func`. The
/// algorithm produces the semi-pruned SSA form found in Cooper, Torczon: "Engineering a Compiler".
pub fn phi_functions(func: &mut Function) -> Result<()> {
   if func.entry_point.is_none() {
      return Err("Function has no entry point".into());
   }

   let lens = type_check(func)?;
   let (globals, usage) = global_names(func);
   let mut cfg = &mut func.cflow_graph;

   // initalize all variables
   if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(func.entry_point.unwrap()) {
      let pos = bb.area.start;
      let instrs = globals
         .iter()
         .map(
            |nam| {
               let len = lens.get(nam).ok_or(format!("No length for variable {}", nam))?;

               Ok(
                  Statement {
                     op: Operation::Move(Rvalue::Undefined),
                     assignee: Lvalue::Variable { size: *len, name: nam.clone(), subscript: None },
                  }
               )
            }
         )
         .collect::<Vec<_>>();

      if instrs.iter().find(|x| x.is_err()).is_some() {
         let e = instrs.into_iter().find(|x| x.is_err());
         return Err(e.unwrap().err().unwrap());
      }

      let instrs = instrs.into_iter().map(|x| x.ok().unwrap()).collect::<Vec<_>>();
      let mne = Mnemonic::new(
         pos..pos,
         "__init".to_string(),
         "".to_string(),
         vec![].iter(),
         instrs.iter(),
      )
            .ok()
            .unwrap();

      bb.mnemonics.insert(0, mne);
   } else {
      return Err("Function entry point is unresolved!".into());
   }

   let idom = immediate_dominator(func.entry_point.unwrap(), cfg);

   if idom.len() != cfg.num_vertices() {
      return Err("No all basic blocks are reachable from function entry point".into());
   }

   let df = dominance_frontiers(&idom, cfg);
   let mut phis = HashSet::<(&Cow<'static, str>, ControlFlowRef)>::new();

   for v in globals.iter() {
      let mut worklist = if let Some(wl) = usage.get(v) {
         wl.clone()
      } else {
         HashSet::new()
      };

      while !worklist.is_empty() {
         let w = worklist.iter().next().unwrap().clone();
         let frontiers = df.get(&w).ok_or("Incomplete dominance frontier set")?;

         worklist.remove(&w);
         for d in frontiers.iter() {
            let arg_num = cfg.in_edges(*d).filter_map(|p| usage.get(v).map(|b| b.contains(&cfg.source(p)))).count();
            if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(*d) {
               if !phis.contains(&(v, *d)) {

                  let pos = bb.area.start;
                  let len = lens.get(v).ok_or(format!("No length for variable {}", v))?;
                  let mne = Mnemonic::new(
                     pos..pos,
                     "__phi".to_string(),
                     "".to_string(),
                     vec![].iter(),
                     vec![
                        Statement {
                           op: Operation::Phi(vec![Rvalue::Variable{ offset: 0, size: *len, name: v.clone(), subscript: None };arg_num]),
                           assignee: Lvalue::Variable { size: *len, name: v.clone(), subscript: None },
                        },
                     ]
                           .iter(),
                  )
                        .ok()
                        .unwrap();

                  bb.mnemonics.insert(0, mne);
                  phis.insert((v, *d));
                  worklist.insert(*d);
               }
            }
         }
      }
   }

   Ok(())
}

/// Sets the SSA subscripts of all variables in `func`. Follows the algorithm outlined
/// Cooper, Torczon: "Engineering a Compiler". The function expects that Phi functions to be
/// already inserted.
pub fn rename_variables(func: &mut Function) -> Result<()> {
   if func.entry_point.is_none() {
      return Err("Function has no entry point".into());
   }

   let (globals, _) = global_names(func);
   let mut cfg = &mut func.cflow_graph;
   let mut stack = HashMap::<Cow<'static, str>, Vec<usize>>::from_iter(globals.iter().map(|x| (x.clone(), Vec::new())));
   let mut counter = HashMap::<Cow<'static, str>, usize>::new();
   let idom = immediate_dominator(func.entry_point.unwrap(), cfg);

   if idom.len() != cfg.num_vertices() {
      return Err("No all basic blocks are reachable from function entry point".into());
   }

   fn new_name(n: &Cow<'static, str>, counter: &mut HashMap<Cow<'static, str>, usize>, stack: &mut HashMap<Cow<'static, str>, Vec<usize>>) -> usize {
      let i = *counter.entry(n.clone()).or_insert(0);

      counter.get_mut(n).map(|x| *x += 1);
      stack.entry(n.clone()).or_insert(Vec::new()).push(i);

      i
   }
   fn rename(
      b: ControlFlowRef,
      counter: &mut HashMap<Cow<'static, str>, usize>,
      stack: &mut HashMap<Cow<'static, str>, Vec<usize>>,
      cfg: &mut ControlFlowGraph,
      idom: &HashMap<ControlFlowRef, ControlFlowRef>,
   ) -> Result<()> {
      if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(b) {
         bb.rewrite(
            |i| match i {
               &mut Statement {
                       op: Operation::Phi(_),
                       assignee: Lvalue::Variable { ref name, ref mut subscript, .. },
                    } => *subscript = Some(new_name(name, counter, stack)),
               _ => {}
            }
         );

         for mne in bb.mnemonics.iter_mut() {
            if mne.opcode != "__phi" {
               for o in mne.operands.iter_mut() {
                  if let &mut Rvalue::Variable { ref name, ref mut subscript, .. } = o {
                     *subscript = stack.get(name).and_then(|x| x.last()).cloned();
                  }
               }

               for i in mne.instructions.iter_mut() {
                  let &mut Statement { ref mut op, ref mut assignee } = i;

                  if let &mut Operation::Phi(_) = op {
                     return Err("Phi instruction outside __phi mnemonic".into());
                  } else {
                     for o in op.operands_mut() {
                        if let &mut Rvalue::Variable { ref name, ref mut subscript, .. } = o {
                           *subscript = stack[name].last().cloned();
                        }
                     }

                     if let &mut Lvalue::Variable { ref name, ref mut subscript, .. } = assignee {
                        *subscript = Some(new_name(name, counter, stack));
                     }
                  }
               }
            }
         }
      }

      let mut succ = cfg.out_edges(b).collect::<Vec<_>>();
      succ.sort();

      for s in succ {
         if let Some(&mut Guard::Predicate { flag: Rvalue::Variable { ref name, ref mut subscript, .. }, .. }) = cfg.edge_label_mut(s) {
            *subscript = stack[name].last().cloned();
         }

         let v = cfg.target(s);
         match cfg.vertex_label_mut(v) {
            Some(&mut ControlFlowTarget::Resolved(ref mut bb)) => {
               bb.rewrite(
                  |i| match i {
                     &mut Statement { op: Operation::Phi(ref mut ops), .. } => {
                        for o in ops.iter_mut() {
                           if let &mut Rvalue::Variable { ref name, ref mut subscript, .. } = o {
                              if subscript.is_none() {
                                 *subscript = stack[name].last().cloned();
                                 break;
                              }
                           }
                        }
                     }
                     _ => {}
                  }
               );
            }
            Some(&mut ControlFlowTarget::Unresolved(Rvalue::Variable { ref name, ref mut subscript, .. })) => *subscript = stack[name].last().cloned(),
            _ => {}
         }
      }

      for (k, _) in idom.iter().filter(|&(_, &v)| v == b) {
         if *k != b {
            rename(*k, counter, stack, cfg, idom)?;
         }
      }

      if let Some(&mut ControlFlowTarget::Resolved(ref mut bb)) = cfg.vertex_label_mut(b) {
         bb.execute(
            |i| match i {
               &Statement { assignee: Lvalue::Variable { ref name, .. }, .. } => {
                  stack.get_mut(name).map(|x| x.pop());
               }
               _ => {}
            }
         );
      }

      Ok(())
   }

   rename(
      func.entry_point.unwrap(),
      &mut counter,
      &mut stack,
      cfg,
      &idom,
   )
}

/// Convert `func` into semi-pruned SSA form.
pub fn ssa_convertion(func: &mut Function) -> Result<()> {
   phi_functions(func)?;
   rename_variables(func)
}

/// Computes for every control flow guard the dependend RREIL operation via reverse data flow
/// analysis.
pub fn flag_operations(func: &Function) -> HashMap<ControlFlowEdge, Operation<Rvalue>> {
   let mut ret = HashMap::new();

   for e in func.cflow_graph.edges() {
      if !ret.contains_key(&e) {
         if let Some(&Guard::Predicate { ref flag, .. }) = func.cflow_graph.edge_label(e) {
            let maybe_bb = func.cflow_graph.vertex_label(func.cflow_graph.source(e));
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = maybe_bb {
               let mut maybe_stmt = None;
               bb.execute(
                  |s| {
                     let a: Rvalue = s.assignee.clone().into();
                     if a == *flag {
                        match s.op {
                           Operation::Equal(_, _) |
                           Operation::LessOrEqualUnsigned(_, _) |
                           Operation::LessOrEqualSigned(_, _) |
                           Operation::LessUnsigned(_, _) |
                           Operation::LessSigned(_, _) => maybe_stmt = Some(s.op.clone()),
                           _ => {}
                        }
                     }
                  }
               );

               if maybe_stmt.is_some() {
                  ret.insert(e.clone(), maybe_stmt.unwrap());
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
   use panopticon_core::{BasicBlock, ControlFlowGraph, ControlFlowTarget, Function, Guard, Lvalue, Mnemonic, Operation, Rvalue, Statement};
   use panopticon_graph_algos::{GraphTrait, MutableGraphTrait, VertexListGraphTrait};
   use std::borrow::Cow;
   use std::collections::HashSet;

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
