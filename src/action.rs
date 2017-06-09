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

use errors::*;
use panopticon_abstract_interp::{Kset, approximate};
use panopticon_core::{BasicBlock, ControlFlowTarget, Function, Lvalue, Rvalue};
use panopticon_graph_algos::GraphTrait;
use singleton::{AbstractInterpretation, Panopticon, VarName};
use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, IntoIterator};
use uuid::Uuid;

#[derive(Clone)]
pub enum Action {
   Comment { function: Uuid, address: u64, before: String, after: String },
   Rename { function: Uuid, before: String, after: String },
   SetValue {
      function: Uuid,
      variable: VarName,
      before: Option<AbstractInterpretation>,
      after: Option<AbstractInterpretation>,
      modified_basic_blocks: Vec<u64>,
   },
}

impl Action {
   pub fn new_comment(panopticon: &mut Panopticon, address: u64, comment: String) -> Result<Action> {
      let maybe_func = panopticon.functions.iter().find(|f| f.1.find_basic_block_at(address).is_some());

      match maybe_func {
         Some((uuid, _)) => {
            Ok(
               Action::Comment {
                  function: uuid.clone(),
                  address: address,
                  before: panopticon.control_flow_comments.get(&address).cloned().unwrap_or("".to_string()),
                  after: comment,
               }
            )
         }
         None => Err(format!("no function at {}", address).into()),
      }
   }

   pub fn new_rename(panopticon: &mut Panopticon, func: Uuid, name: String) -> Result<Action> {
      Ok(
         Action::Rename {
            function: func,
            before: panopticon.functions.get(&func).map(|f| f.name.clone()).unwrap_or("".to_string()),
            after: name,
         }
      )
   }

   pub fn new_setvalue(panopticon: &mut Panopticon, func: Uuid, variable: VarName, value: Option<Vec<u64>>) -> Result<Action> {
      use panopticon_data_flow::type_check;
      let function = panopticon.functions.get(&func).unwrap();
      let lens = type_check(function)?;
      let len = lens.get(&variable.name).unwrap();
      let value = value.map(|x| Kset::Set(x.into_iter().map(|x| (x, *len)).collect()));

      let before = panopticon.control_flow_values.get(&func);
      let mut input = before.map(|x| x.input.clone()).unwrap_or(HashMap::new());

      if let Some(ref value) = value {
         input.insert(variable.clone(), value.clone());
      } else {
         input.remove(&variable);
      }

      let after = if input.is_empty() {
         None
      } else {
         let output = {
            let i = input.iter().map(|(k, v)| ((k.name.clone(), k.subscript), v.clone()));
            let fixed = HashMap::from_iter(i);

            approximate(function, &fixed)?
         };
         let o = output
            .into_iter()
            .filter_map(
               |(k, v)| match k {
                  Lvalue::Variable { name, subscript: Some(subscript), .. } => Some((VarName { name: name, subscript: subscript }, v)),
                  Lvalue::Variable { subscript: None, .. } => None,
                  Lvalue::Undefined { .. } => None,
               }
            );

         Some(AbstractInterpretation { input: input, output: HashMap::from_iter(o) })
      };

      let function = panopticon.functions.get(&func).unwrap();
      let addrs = diff_abstract_interpretations(after.as_ref(), before, &function);

      Ok(
         Action::SetValue {
            function: func,
            variable: variable,
            before: before.cloned(),
            after: after,
            modified_basic_blocks: addrs,
         }
      )
   }

   pub fn undo(&self, panopticon: &mut Panopticon) -> Result<()> {
      match self {
         &Action::Comment { ref function, address, ref before, ref after } => {
            debug_assert!(panopticon.control_flow_comments.get(&address).unwrap_or(&"".to_string()) == after);
            panopticon.control_flow_comments.insert(address, before.clone());
            panopticon.update_control_flow_nodes(function, Some(&vec![address]))
         }
         &Action::Rename { ref function, ref before, .. } => {
            if let Some(func) = panopticon.functions.get_mut(function) {
               func.name = before.clone();
            }

            for (uuid, addr) in panopticon.resolved_calls.get_vec(function).cloned().unwrap_or(vec![]) {
               panopticon.update_control_flow_nodes(&uuid, Some(&[addr]))?;
            }

            panopticon.update_sidebar(function)

         }
         &Action::SetValue { ref function, ref before, ref modified_basic_blocks, .. } => {
            if let &Some(ref before) = before {
               panopticon.control_flow_values.insert(function.clone(), before.clone());
            } else {
               panopticon.control_flow_values.remove(function);
            }
            panopticon.update_control_flow_nodes(function, Some(modified_basic_blocks))
         }
      }
   }

   pub fn redo(&self, panopticon: &mut Panopticon) -> Result<()> {
      match self {
         &Action::Comment { ref function, address, ref before, ref after } => {
            debug_assert!(panopticon.control_flow_comments.get(&address).unwrap_or(&"".to_string()) == before);
            panopticon.control_flow_comments.insert(address, after.clone());
            panopticon.update_control_flow_nodes(function, Some(&vec![address]))
         }
         &Action::Rename { ref function, ref after, .. } => {
            if let Some(func) = panopticon.functions.get_mut(function) {
               func.name = after.clone();
            }

            for (uuid, addr) in panopticon.resolved_calls.get_vec(function).cloned().unwrap_or(vec![]) {
               panopticon.update_control_flow_nodes(&uuid, Some(&[addr]))?;
            }

            panopticon.update_sidebar(function)
         }
         &Action::SetValue { ref function, ref after, ref modified_basic_blocks, .. } => {
            if let &Some(ref after) = after {
               panopticon.control_flow_values.insert(function.clone(), after.clone());
            } else {
               panopticon.control_flow_values.remove(function);
            }
            panopticon.update_control_flow_nodes(function, Some(modified_basic_blocks))
         }
      }
   }
}

fn diff_abstract_interpretations(a: Option<&AbstractInterpretation>, b: Option<&AbstractInterpretation>, func: &Function) -> Vec<u64> {
   let a = if let Some(a) = a {
      HashSet::<(&VarName, &Kset)>::from_iter(a.output.iter())
   } else {
      HashSet::new()
   };
   let b = if let Some(b) = b {
      HashSet::<(&VarName, &Kset)>::from_iter(b.output.iter())
   } else {
      HashSet::new()
   };
   let blocks = func
      .postorder()
      .into_iter()
      .filter_map(
         |vx| if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
            Some(bb)
         } else {
            None
         }
      )
      .collect::<Vec<&BasicBlock>>();

   let mut ret = vec![];
   let nams = HashSet::<VarName>::from_iter(a.symmetric_difference(&b).map(|&(n, _)| n.clone()));

   for bb in blocks.iter() {
      let mut hit = false;

      bb.execute(
         |stmt| {
            if let Lvalue::Variable { ref name, subscript: Some(subscript), .. } = stmt.assignee {
               hit |= nams.contains(&VarName { name: name.clone(), subscript: subscript });
            }

            hit |= stmt
               .op
               .operands()
               .iter()
               .any(
                  |rv| if let &&Rvalue::Variable { ref name, subscript: Some(subscript), .. } = rv {
                     nams.contains(&VarName { name: name.clone(), subscript: subscript })
                  } else {
                     false
                  }
               );
         }
      );

      if hit {
         ret.push(bb.area.start);
      }
   }

   ret.sort();
   ret.dedup();

   ret
}
