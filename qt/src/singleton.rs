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

use action::Action;
use control_flow_layout::{BasicBlockLine, ControlFlowLayout};
use errors::*;
use futures::{Future, future};
use multimap::MultiMap;
use panopticon_abstract_interp::Kset;
use panopticon_core::{CallTarget, Function, Program, Project, Region, loader};
use panopticon_glue::Glue;
use panopticon_graph_algos::{GraphTrait, VertexListGraphTrait};
use parking_lot::Mutex;
use qt;
use qt::Qt;
use std::borrow::Cow;
use std::collections::HashMap;
use std::thread;
use uuid::Uuid;

#[derive(PartialEq,Eq,Clone,Debug,Hash)]
pub struct VarName {
    pub name: Cow<'static, str>,
    pub subscript: usize,
}

#[derive(Clone,Debug)]
pub struct AbstractInterpretation {
    pub input: HashMap<VarName, Kset>,
    pub output: HashMap<VarName, Kset>,
}

lazy_static!{
    pub static ref PANOPTICON: Mutex<Panopticon> = {
        Mutex::new(Panopticon::default())
    };
}

pub type NodePosition = (usize, f32, f32, bool, Vec<BasicBlockLine>);
pub type EdgePosition = (usize, &'static str, String, (f32, f32), (f32, f32), Vec<(f32, f32, f32, f32)>);

pub struct Panopticon {
    pub control_flow_layouts: HashMap<Uuid, ControlFlowLayout>,

    pub control_flow_comments: HashMap<u64, String>,
    pub control_flow_values: HashMap<Uuid, AbstractInterpretation>,

    pub functions: HashMap<Uuid, Function>,
    pub by_entry: HashMap<u64, Uuid>,
    pub unresolved_calls: MultiMap<Option<u64>, (Uuid, u64)>,
    pub resolved_calls: MultiMap<Uuid, (Uuid, u64)>, // callee -> caller
    pub region: Option<Region>,
    pub project: Option<Project>,

    pub undo_stack: Vec<Action>,
    pub undo_stack_top: usize,

    pub layout_task: Option<future::BoxFuture<ControlFlowLayout, Error>>,
}

impl Panopticon {
    pub fn layout_function_async(&mut self, uuid: &Uuid) -> future::BoxFuture<(Vec<NodePosition>, Vec<EdgePosition>), Error> {
        if !self.control_flow_layouts.contains_key(&uuid) {
            let func = self.functions.get(&uuid).unwrap();
            let cmnts = &self.control_flow_comments;
            let values = self.control_flow_values.get(&uuid);
            let funcs = &self.functions;
            let uuid2 = uuid.clone();

            ControlFlowLayout::new_async(func, cmnts, values, funcs, 8, 3, 8, 26, 17, 150)
                .and_then(
                    move |cfl| {
                        let uuid = uuid2;
                        let nodes = cfl.get_all_nodes();
                        let edges = cfl.get_all_edges();

                        PANOPTICON.lock().control_flow_layouts.insert(uuid, cfl);
                        future::ok((nodes, edges))
                    }
                )
                .boxed()
        } else {
            let cfl = &self.control_flow_layouts.get(uuid).unwrap();
            let nodes = cfl.get_all_nodes();
            let edges = cfl.get_all_edges();

            future::ok((nodes, edges)).boxed()
        }
    }

    fn get_function<'a>(&'a mut self, uuid: &Uuid) -> Result<&'a mut ControlFlowLayout> {
        if !self.control_flow_layouts.contains_key(&uuid) {
            let func = self.functions.get(&uuid).unwrap();
            let cmnts = &self.control_flow_comments;
            let values = self.control_flow_values.get(&uuid);
            let funcs = &self.functions;
            let cfl = ControlFlowLayout::new(func, cmnts, values, funcs, 8, 3, 8, 26, 17, 150)?;

            self.control_flow_layouts.insert(uuid.clone(), cfl);
        }

        Ok(self.control_flow_layouts.get_mut(&uuid).unwrap())
    }

    pub fn get_function_nodes(&mut self, uuid: String) -> Result<Vec<NodePosition>> {
        let uuid = Uuid::parse_str(&uuid)?;
        let cfl = self.get_function(&uuid)?;

        Ok(cfl.get_all_nodes())
    }

    pub fn open_program(&mut self, path: String) -> Result<()> {
        use std::path::Path;
        use panopticon_core::{CallTarget, Machine};
        use panopticon_amd64 as amd64;
        use panopticon_avr as avr;
        use panopticon_analysis::pipeline;
        use futures::Stream;
        use std::ffi::CString;

        debug!("open_program() path={}", path);

        if let Ok(proj) = Project::open(&Path::new(&path)) {
            if !proj.code.is_empty() {
                {
                    let cg = &proj.code[0].call_graph;

                    for f in cg.vertices() {
                        if let Some(&CallTarget::Concrete(ref func)) = cg.vertex_label(f) {
                            self.functions.insert(func.uuid().clone(), func.clone());
                            Qt::update_sidebar(&[func.clone()]);
                        }
                    }
                }

                self.project = Some(proj);
                Ok(Qt::send_current_session(CString::new(path.as_bytes())?)?)
            } else {
                Ok(())
            }
        } else if let Ok((mut proj, machine)) = loader::load(&Path::new(&path)) {
            let maybe_prog = proj.code.pop();
            let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap().clone();

            if let Some(prog) = maybe_prog {
                let prog = ::std::sync::Arc::new(prog);
                let pipe = match machine {
                    Machine::Avr => pipeline::<avr::Avr>(prog, reg.clone(), avr::Mcu::atmega103()),
                    Machine::Ia32 => pipeline::<amd64::Amd64>(prog, reg.clone(), amd64::Mode::Protected),
                    Machine::Amd64 => pipeline::<amd64::Amd64>(prog, reg.clone(), amd64::Mode::Long),
                };
                self.region = Some(reg);

                thread::spawn(
                    || -> Result<()> {
                        info!("disassembly thread started");
                        for i in pipe.wait() {
                            if let Ok(func) = i {
                                PANOPTICON.lock().new_function(func.clone())?;
                                Qt::update_sidebar(&[func]);
                            }
                        }
                        info!("disassembly thread finished");

                        Ok(())
                    }
                );

                use paths::session_directory;
                use tempdir::TempDir;

                let dir = session_directory().unwrap();
                let dir = format!(
                    "{}",
                    TempDir::new_in(dir, "panop-backing").unwrap().path().display()
                );
                Ok(Qt::send_current_session(CString::new(dir.as_bytes())?)?)
            } else {
                Err(
                    format!(
                        "{} is neither a saved session nor a recognized executable type",
                        path
                    )
                            .into()
                )
            }
        } else {
            Err(
                format!(
                    "{} is neither a saved session nor a recognized executable type",
                    path
                )
                        .into()
            )
        }
    }

    pub fn comment_on(&mut self, addr: u64, comment: String) -> Result<()> {
        debug!("comment_on(): address={}, comment={}", addr, comment);

        let act = Action::new_comment(self, addr, comment.clone())?;
        self.push_action(act)?;

        Ok(())
    }

    pub fn rename_function(&mut self, uuid: String, name: String) -> Result<()> {
        debug!("rename_function(): uuid={}, name={}", uuid, name);

        let func = Uuid::parse_str(&uuid)?;
        let act = Action::new_rename(self, func, name.clone())?;
        self.push_action(act)?;

        Ok(())
    }

    pub fn set_value_for(&mut self, func: String, variable: String, value: String) -> Result<()> {
        use std::str::FromStr;

        let toks: Vec<String> = variable.split('_').map(str::to_string).collect();
        if toks.len() == 2 {
            if let Ok(subscript) = usize::from_str(&toks[1]) {

                let var = VarName { name: toks[0].clone().into(), subscript: subscript };
                let val = if value == "" {
                    None
                } else {
                    let vals = value
                        .split(',')
                        .filter_map(
                            |x| {
                                let x = x.trim();
                                if x.starts_with("0x") {
                                    let s: &[_] = &['0', 'x'];
                                    u64::from_str_radix(x.trim_matches(s), 16).ok()
                                } else if x.starts_with("0b") {
                                    let s: &[_] = &['0', 'b'];
                                    u64::from_str_radix(x.trim_matches(s), 2).ok()
                                } else {
                                    u64::from_str(x).ok()
                                }
                            }
                        )
                        .map(|x| x)
                        .collect::<Vec<_>>();

                    if !vals.is_empty() {
                        Some(vals)
                    } else {
                        return Err(format!("'{}' is not a valid value", value).into());
                    }
                };
                debug!(
                    "set_value_for(): func={}, variable={}, value={}",
                    func,
                    variable,
                    value
                );

                let act = Action::new_setvalue(self, Uuid::parse_str(&func).unwrap(), var, val)?;
                self.push_action(act)?;

                Ok(())
            } else {
                Err(format!("'{}' is not an integer", toks[1]).into())
            }
        } else {
            Err(format!("'{:?}' is not a valid variable", toks).into())
        }
    }

    pub fn save_session(&mut self, path: String) -> Result<()> {
        use std::path::Path;

        debug!("save_session() path={}", path);

        if let Some(ref proj) = self.project {
            proj.snapshot(&Path::new(&path))?;
        } else if let Some(ref region) = self.region {
            let mut proj = Project::new("(none)".to_string(), region.clone());
            let mut prog = Program::new("(none");

            for f in self.functions.iter() {
                prog.insert(CallTarget::Concrete(f.1.clone()));
            }

            proj.code.push(prog);
            proj.snapshot(&Path::new(&path))?;
        } else {
            return Err(format!("Saving failed: no session to save").into());
        }

        Ok(())
    }

    pub fn undo(&mut self) -> Result<()> {
        let top = self.undo_stack_top;

        if top == 0 || self.undo_stack.get(top - 1).is_none() {
            return Err(format!("call to undo() when canUndo() is false").into());
        }

        let act = self.undo_stack[top - 1].clone();
        act.undo(self)?;

        self.undo_stack_top = top - 1;

        Ok(Qt::send_undo_redo_update(top - 1 != 0, true)?)
    }

    pub fn redo(&mut self) -> Result<()> {
        let top = self.undo_stack_top;

        if self.undo_stack.get(top).is_none() {
            return Err(format!("call to redo() when canRedo() is false").into());
        }

        let act = self.undo_stack[top].clone();
        act.redo(self)?;

        self.undo_stack_top = top + 1;

        let len = self.undo_stack.len();

        Ok(Qt::send_undo_redo_update(true, top + 1 < len)?)
    }

    pub fn update_control_flow_nodes(&mut self, uuid: &Uuid, addrs: Option<&[u64]>) -> Result<()> {
        use std::ffi::CString;
        use panopticon_glue::{CBasicBlockLine, CBasicBlockOperand};

        debug!(
            "update_control_flow_nodes() func={}, addrs={:?}",
            uuid,
            addrs
        );

        let ids = if let Some(ref mut cfl) = self.control_flow_layouts.get_mut(uuid) {
            let func = self.functions.get(&uuid).unwrap();
            let cmnts = &self.control_flow_comments;
            let values = self.control_flow_values.get(&uuid);
            let funcs = &self.functions;

            cfl.update_nodes(addrs, func, cmnts, values, funcs)?
        } else {
            vec![]
        };

        if !qt::SUBSCRIBED_FUNCTIONS.lock().contains(uuid) {
            return Ok(());
        }

        let bbls: Vec<_> = self.get_function_nodes(uuid.clone().to_string())
            .unwrap()
            .into_iter()
            .filter_map(
                |bbl| if bbl.4.is_empty() || ids.is_empty() || ids.iter().any(|&x| x == bbl.0 as i32) {
                    Some(bbl)
                } else {
                    None
                }
            )
            .map(
                |(id, x, y, is_entry, blk)| {
                    let blk = blk.into_iter()
                        .filter_map(
                            |bbl| {
                                let args = bbl.args
                                    .into_iter()
                                    .filter_map(|x| CBasicBlockOperand::new(x.kind.to_string(), x.display, x.alt, x.data).ok())
                                    .collect::<Vec<_>>();
                                CBasicBlockLine::new(bbl.opcode, bbl.region, bbl.offset, bbl.comment, args).ok()
                            }
                        )
                        .collect::<Vec<_>>();
                    (id, x, y, is_entry, blk)
                }
            )
            .collect();
        let uuid = CString::new(uuid.clone().to_string().as_bytes()).unwrap();

        for (id, x, y, is_entry, bbl) in bbls.into_iter() {
            debug!("send update for id {}", id);
            Qt::send_function_node(uuid.clone(), id, x, y, is_entry, bbl.as_slice())?;
        }

        Ok(())
    }

    pub fn update_sidebar(&mut self, uuid: &Uuid) -> Result<()> {
        debug!("update_sidebar() func={}", uuid);

        let func = self.functions.get(uuid).cloned();
        match func {
            Some(func) => {
                Qt::update_sidebar(&[func]);
                Ok(())
            }
            None => Err(format!("unknown function {}", uuid).into()),
        }
    }

    pub fn new_function(&mut self, func: Function) -> Result<()> {
        use panopticon_core::{ControlFlowTarget, Operation, Rvalue, Statement};

        let pairs = {
            let maybe_entry = {
                let cfg = &func.cflow_graph;
                for vx in cfg.vertices() {
                    if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
                        bb.execute(
                            |stmt| {
                                if let &Statement { op: Operation::Call(ref rv), .. } = stmt {
                                    match rv {
                                        &Rvalue::Constant { value, .. } => {
                                            // their addr
                                            let maybe_callee = self.by_entry.get(&value);

                                            if let Some(callee) = maybe_callee {
                                                self.resolved_calls.insert(callee.clone(), (func.uuid().clone(), bb.area.start));
                                            } else {
                                                self.unresolved_calls.insert(Some(value), (func.uuid().clone(), bb.area.start));
                                            }
                                        }
                                        _ => self.unresolved_calls.insert(None, (func.uuid().clone(), bb.area.start)),
                                    }
                                }
                            }
                        );
                    }
                }

                // we should be able to just write func.start but i suspect there are bugs which modify the cfg directly without updating entry_point
                match func.entry_point() {
                    &ControlFlowTarget::Resolved(ref bb) => Some(bb.area.start),
                    _ => None,
                }
            };

            if let Some(entry) = maybe_entry {
                // my addr
                let pairs_owned = self.unresolved_calls.remove(&Some(entry)).unwrap_or(vec![]).into_iter();
                let pairs_ref = self.unresolved_calls.get_vec(&None).cloned().unwrap_or(vec![]).into_iter();

                self.by_entry.insert(entry, func.uuid().clone());

                for (uuid, addr) in pairs_owned.clone() {
                    self.resolved_calls.insert(func.uuid().clone(), (uuid.clone(), addr));
                }

                self.functions.insert(func.uuid().clone(), func);

                pairs_owned.chain(pairs_ref).collect::<Vec<_>>()
            } else {
                vec![]
            }
        };

        for (uuid, addr) in pairs.into_iter() {
            self.update_control_flow_nodes(&uuid, Some(&[addr])).unwrap();
        }

        Ok(())
    }

    fn push_action(&mut self, act: Action) -> Result<()> {
        let top = self.undo_stack_top;

        act.redo(self)?;

        self.undo_stack.truncate(top);
        self.undo_stack.push(act);
        self.undo_stack_top = self.undo_stack.len();

        let top = self.undo_stack_top;
        let len = self.undo_stack.len();

        Ok(Qt::send_undo_redo_update(top != 0, top < len)?)
    }
}

impl Default for Panopticon {
    fn default() -> Panopticon {
        Panopticon {
            control_flow_layouts: HashMap::new(),
            control_flow_comments: HashMap::new(),
            control_flow_values: HashMap::new(),
            functions: HashMap::new(),
            by_entry: HashMap::new(),
            unresolved_calls: MultiMap::new(),
            resolved_calls: MultiMap::new(),
            project: None,
            region: None,
            undo_stack: Vec::new(),
            undo_stack_top: 0,
            layout_task: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let _ = Panopticon::default();
    }

    #[test]
    fn open_save() {
        let mut panop = Panopticon::default();
        panop.open_program("../test-data/save.panop".to_string()).unwrap();
    }
}
