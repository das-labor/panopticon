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

// Qt uses camelCase
#![allow(non_snake_case)]

use qml::{
    QObject,
    QVariant,
    QMetaType,
    QMetaTypable,
    QObjectMacro,
    QListModel,
    emit_signal,
};
use paths::{
    session_directory
};
use std::fs;
use std::str::FromStr;
use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;
use std::borrow::Cow;
use panopticon::{
    Project,
    Program,
    loader,
    Function,
    ControlFlowTarget,
    Mnemonic,
    Guard,
    Kset,
    Region,
    CallTarget,
};
use uuid::Uuid;
use parking_lot::Mutex;
use errors::*;
use sugiyama;
use action::Action;
use graph_algos::{
    GraphTrait,
    VertexListGraphTrait,
    EdgeListGraphTrait,
    IncidenceGraphTrait,
};
use graph_algos::adjacency_list::{
    AdjacencyListEdgeDescriptor,
    AdjacencyListVertexDescriptor,
};
use rustc_serialize::json;


Q_LISTMODEL! {
    pub QRecentSessions {
        timestamp: i32,
        title: String,
        typ: String,
        path: String
    }
}

Q_LISTMODEL! {
    pub QSidebar {
        title: String,
        subtitle: String,
        uuid: String
    }
}

Q_LISTMODEL! {
    pub QControlFlowNodes {
        x: f32,
        y: f32,
        id: i32,
        is_entry: bool,
        contents: String
    }
}

Q_LISTMODEL! {
    pub QControlFlowEdges {
        path_x: String,
        path_y: String,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        kind: String,
        label: String
    }
}

#[derive(Clone)]
pub struct ControlFlowLayout {
    node_dimensions: HashMap<AdjacencyListVertexDescriptor,(f32,f32)>,
    layout: sugiyama::LinearLayout,
    node_positions: HashMap<AdjacencyListVertexDescriptor,(f32,f32)>,
    edges: HashMap<AdjacencyListEdgeDescriptor,(Vec<(f32,f32,f32,f32)>,(f32,f32),(f32,f32))>,
}

#[derive(RustcEncodable)]
struct BasicBlockLine {
    opcode: String,
    region: String,
    offset: u64,
    comment: String,
    args: Vec<BasicBlockOperand>,
}

#[derive(RustcEncodable)]
struct BasicBlockOperand {
    kind: &'static str, // constant, variable, function, literal
    display: String, // string to display
    alt: String, // alternative display string
    data: String, // constant: value, variable: ssa var, function: UUID, literal: empty string
}

#[derive(PartialEq,Eq,Clone,Debug,Hash)]
pub struct VarName {
    pub name: Cow<'static,str>,
    pub subscript: usize,
}

#[derive(Clone,Debug)]
pub struct AbstractInterpretation {
    pub input: HashMap<VarName,Kset>,
    pub output: HashMap<VarName,Kset>,
}

pub struct Panopticon {
    // QML
    pub recent_sessions: QRecentSessions,
    pub sidebar: QSidebar,
    pub control_flow_nodes: QControlFlowNodes,
    pub control_flow_edges: QControlFlowEdges,

    pub control_flow_layouts: HashMap<Uuid,ControlFlowLayout>,

    pub control_flow_comments: HashMap<u64,String>,
    pub control_flow_values: HashMap<Uuid,AbstractInterpretation>,

    pub new_functions: Mutex<Vec<Function>>,
    pub functions: HashMap<Uuid,Function>,
    pub region: Option<Region>,
    pub project: Option<Project>,

    pub undo_stack: Vec<Action>,
    pub undo_stack_top: usize,
}


impl Panopticon {
    fn read_recent_sessions() -> Result<QRecentSessions> {
        let path = session_directory()?;
        let mut ret = QRecentSessions::new();

        if let Ok(dir) = fs::read_dir(path) {

            for ent in dir.filter_map(|x| x.ok()) {
                if let Ok(ref project) = Project::open(&ent.path()) {
                    if let Ok(ref md) = ent.metadata() {
                        let mtime = md.modified()?.duration_since(::std::time::UNIX_EPOCH)?.as_secs() as i32;
                        let fname = ent.path().into_os_string().to_string_lossy().to_string();
                        ret.append_row(mtime,project.name.clone(),"".to_string(),fname);
                    }
                }
            }
        }
        Ok(ret)
    }
}

impl QPanopticon {
    fn callback(&mut self) -> Option<&QVariant> {
        let funcs = {
            let mut guard = self.new_functions.lock();
            let funcs = guard.drain(..).collect::<Vec<Function>>();
            funcs
        };

        for func in funcs {
            {
                let cfg = &func.cflow_graph;
                let entry = func.entry_point.
                    and_then(|vx| cfg.vertex_label(vx)).
                    and_then(|lb| {
                        if let &ControlFlowTarget::Resolved(ref bb) = lb {
                            Some(bb.area.start)
                        } else {
                            None
                        }
                    });
                let str_entry = entry.map(|x| format!("0x{:x}",x)).unwrap_or("".to_string());
                self.sidebar.append_row(func.name.to_string(),str_entry,func.uuid.to_string());
            }
            self.functions.insert(func.uuid.clone(),func);
        }

        None
    }

    fn open_program(&mut self,path: String) -> Option<&QVariant> {
        use std::path::Path;
        use panopticon::{
            ControlFlowTarget,
            CallTarget,
            amd64,
            avr,
            Machine,
            pipeline,
        };
        use futures::Stream;

        debug!("open_program() path={}",path);

        if let Ok(proj) = Project::open(&Path::new(&path)) {
            if !proj.code.is_empty() {
                {
                    let cg = &proj.code[0].call_graph;

                    for f in cg.vertices() {
                        if let Some(&CallTarget::Concrete(ref func)) = cg.vertex_label(f) {
                            let cfg = &func.cflow_graph;
                            let entry = func.entry_point.
                                and_then(|vx| cfg.vertex_label(vx)).
                                and_then(|lb| {
                                    if let &ControlFlowTarget::Resolved(ref bb) = lb {
                                        Some(bb.area.start)
                                    } else {
                                        None
                                    }
                                });
                            let str_entry = entry.map(|x| format!("0x{:x}",x)).unwrap_or("".to_string());
                            self.sidebar.append_row(func.name.to_string(),str_entry,func.uuid.to_string());
                            self.functions.insert(func.uuid.clone(),func.clone());
                        }
                    }
                }

                self.project = Some(proj);
                self.set_current_session(path);
                self.current_session_changed();
            }
        } else if let Ok((mut proj,machine)) = loader::load(&Path::new(&path)) {
            let maybe_prog = proj.code.pop();
            let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap().clone();

            if let Some(prog) = maybe_prog {
                let pipe = match machine {
                    Machine::Avr => pipeline::<avr::Avr>(prog,reg.clone(),avr::Mcu::atmega103()),
                    Machine::Ia32 => pipeline::<amd64::Amd64>(prog,reg.clone(),amd64::Mode::Protected),
                    Machine::Amd64 => pipeline::<amd64::Amd64>(prog,reg.clone(),amd64::Mode::Long),
                };
                self.region = Some(reg);

                self.threaded(|s| {
                    info!("disassembly thread started");
                    for i in pipe.wait() {
                        if let Ok(func) = i {
                            let mut guard = s.new_functions.lock();

                            guard.push(func);
                            s.call_me_maybe();
                        }
                    }
                    info!("disassembly thread finished");
                });

                use paths::session_directory;
                use tempdir::TempDir;

                let dir = session_directory().unwrap();
                let dir = format!("{}",TempDir::new_in(dir,"panop-backing").unwrap().path().display());
                self.set_current_session(dir);
                self.current_session_changed();
            }
        } else {
            error!("{} is neither a saved session nor a recognized executable type",path);
        }

        None
    }

    fn comment_on(&mut self,address: String, comment: String) -> Option<&QVariant> {
        use std::str::FromStr;

        println!("comment_on(): address={}, comment={}",address,comment);
        let addr = u64::from_str(&address).unwrap();
        let func: String = self.get_visible_function().into();
        let act = Action::new_comment(self,Uuid::parse_str(&func).unwrap(),addr,comment.clone()).unwrap();
        let _ = self.push_action(act);

        None
    }

    fn rename_function(&mut self,uuid: String, name: String) -> Option<&QVariant> {
        println!("rename_function(): uuid={}, name={}",uuid,name);
        let func = Uuid::parse_str(&uuid).unwrap();
        let act = Action::new_rename(self,func,name.clone()).unwrap();
        let _ = self.push_action(act);

        None
    }

    fn set_value_for(&mut self,variable: String, value: String) -> Option<&QVariant> {
        use std::str::FromStr;

        let toks: Vec<String> = variable.split('_').map(str::to_string).collect();
        if toks.len() == 2 {
            if let Ok(subscript) = usize::from_str(&toks[1]) {

                let var = VarName{
                    name: toks[0].clone().into(),
                    subscript: subscript,
                };
                let func: String = self.get_visible_function().into();
                let val = if value == "" {
                    None
                } else {
                    let vals = value
                        .split(',')
                        .filter_map(|x| u64::from_str(x.trim()).ok())
                        .map(|x| (x,64))
                        .collect::<Vec<_>>();

                    if !vals.is_empty() {
                        Some(Kset::Set(vals))
                    } else {
                        println!("'{}' is not a valid value",value);
                        return None;
                    }
                };
                println!("set_value_for(): variable={}, value={}",variable,value);

                let act = Action::new_setvalue(self,Uuid::parse_str(&func).unwrap(),var,val).unwrap();
                let _ = self.push_action(act);
            } else {
                println!("'{}' is not an integer",toks[1]);
            }
        } else {
            println!("'{:?}' is not a valid variable",toks);
        }

        None
    }

    fn save_session(&mut self, path: String) -> Option<&QVariant> {
        use std::path::Path;

        debug!("save_session() path={}",path);

        if let Some(ref proj) = self.project {
            if proj.snapshot(&Path::new(&path)).is_err() {
                error!("Saving failed");
            }
        } else if let Some(ref region) = self.region {
            let mut proj = Project::new("(none)".to_string(),region.clone());
            let mut prog = Program::new("(none");

            for f in self.functions.iter() {
                prog.insert(CallTarget::Concrete(f.1.clone()));
            }

            proj.code.push(prog);
            if proj.snapshot(&Path::new(&path)).is_err() {
                error!("Saving failed");
            }
        } else {
            error!("Saving failed");
        }

        None
    }

    fn display_control_flow_for(&mut self, uuid_str: String) -> Option<&QVariant> {
        debug!("display_control_flow_for() uuid={}",uuid_str);

        let uuid = Uuid::from_str(&uuid_str).unwrap();

        if !self.control_flow_layouts.contains_key(&uuid) {
            let mut vertices = HashSet::new();
            let mut edges = vec![];

            if !self.functions.contains_key(&uuid) { return None; }

            {
                let func = &self.functions[&uuid];
                let cfg = &func.cflow_graph;
                let edge_iter = cfg.edges().map(|e| (cfg.source(e).0,cfg.target(e).0));

                for (from_idx,to_idx) in edge_iter {
                    vertices.insert(from_idx);
                    vertices.insert(to_idx);
                    edges.push((from_idx,to_idx));
                }
            }

            let layout_res = sugiyama::linear_layout_structural(
                &vertices.iter().map(|&vx| vx).collect::<Vec<_>>(),
                &edges,
                None);

            if let Ok(layout) = layout_res {
                let layout = ControlFlowLayout{
                    node_dimensions: HashMap::new(),
                    layout: layout,
                    node_positions: HashMap::new(),
                    edges: HashMap::new(),
                };
                self.control_flow_layouts.insert(uuid.clone(),layout.clone());
            } else {
                error!("layouting failed");
                return None
            }
        }

        let need_dims = self.control_flow_layouts[&uuid].node_dimensions.is_empty();

        if need_dims {
            let _ = self.update_control_flow_dimensions(&uuid);
        }

        let _ = self.set_control_flow_properties(&uuid,None);
        self.set_visible_function(uuid.to_string());
        println!("layout done");
        None
    }

    fn display_preview_for(&mut self,uuid_str: String) -> Option<&QVariant> {
        debug!("display_preview_for(): uuid={}",uuid_str);

        let uuid = Uuid::from_str(&uuid_str).unwrap();
        let maybe_contents = self.functions.get(&uuid).and_then(|func| {
            func.entry_point.map(|x| (func,x))
        }).and_then(|(func,entry)| {
            let cfg = &func.cflow_graph;
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(entry) {
                Some(bb)
            } else {
                None
            }
        }).and_then(|bb| {
            let lines = bb.mnemonics.iter().map(|mne| {
                self.get_basic_block_line(&uuid,mne).unwrap()
            }).collect::<Vec<_>>();

            json::encode(&lines).ok()
        });

        if let Some(contents) = maybe_contents {
            self.set_preview_node(contents);
            self.set_preview_function(uuid_str);
            self.preview_node_changed();
            self.preview_function_changed();
        }
        None
    }

    fn undo(&mut self) -> Option<&QVariant> {
        let top = self.undo_stack_top;

        if top == 0 || self.undo_stack.get(top - 1).is_none() {
            unreachable!("call to undo() when canUndo() is false");
        }

        let act = self.undo_stack[top - 1].clone();
        act.undo(self).unwrap();

        self.undo_stack_top = top - 1;

        self.set_can_undo(top - 1 != 0);
        self.set_can_redo(true);

        self.can_undo_changed();
        self.can_redo_changed();

        None
    }

    fn redo(&mut self) -> Option<&QVariant> {
        let top = self.undo_stack_top;

        if self.undo_stack.get(top).is_none() {
            unreachable!("call to redo() when canRedo() is false");
        }

        let act = self.undo_stack[top].clone();
        act.redo(self).unwrap();

        self.undo_stack_top = top + 1;

        let len = self.undo_stack.len();

        self.set_can_undo(true);
        self.set_can_redo(top + 1 < len);

        self.can_undo_changed();
        self.can_redo_changed();

        None
    }

    fn update_control_flow_dimensions(&mut self,uuid: &Uuid) -> Result<()> {
        info!("update_control_flow_dimensions() uuid={}",uuid);
        let bb_char_width = self.get_basic_block_character_width().to_int() as usize;
        let bb_padding = self.get_basic_block_padding().to_int() as usize;
        let bb_margin = self.get_basic_block_margin().to_int() as usize;
        let bb_col_padding = self.get_basic_block_column_padding().to_int() as usize;
        let bb_line_height = self.get_basic_block_line_height().to_int() as usize;
        let bb_cmnt_width = self.get_basic_block_comment_width().to_int() as usize;
        let mut vertices = HashSet::new();
        let mut dims = HashMap::new();

        let (edges,edges_rev) = {
            let func = &self.functions[uuid];
            let cfg = &func.cflow_graph;

            for vx in cfg.vertices() {
                let maybe_lb = cfg.vertex_label(vx);

                match maybe_lb {
                    Some(&ControlFlowTarget::Resolved(ref bb)) => {
                        let linew = bb.mnemonics.iter().map(|mne| {
                            mne.opcode.len() + mne.operands.iter().map(|a| format!("{}",a).len()).sum::<usize>()
                        }).max().unwrap_or(0);
                        /*let has_cmnt = bb.mnemonics.iter().any(|mne| {
                            self.control_flow_comments.contains_key(&mne.area.start)
                        });*/
                        let height = bb.mnemonics.len() * bb_line_height
                            + 2 * bb_margin + 2 * bb_padding;
                        let width = linew * bb_char_width
                            + 2 * bb_margin + 2 * bb_padding + bb_col_padding;
                        //+ if has_cmnt { bb_cmnt_width } else { 0 };

                        vertices.insert(vx.0);
                        dims.insert(vx.0,(width as f32,height as f32));
                    }
                    Some(&ControlFlowTarget::Unresolved(_)) |
                        Some(&ControlFlowTarget::Failed(_,_)) => {
                            vertices.insert(vx.0);
                            dims.insert(vx.0,(1.,1.));
                        }
                    None => {
                        return Err(format!("Unlabeled vertex {}",vx.0).into())
                    }
                }
            }

            let edges = cfg.edges().map(|e| (cfg.source(e).0,cfg.target(e).0)).collect();
            let edges_rev = HashMap::<usize,AdjacencyListEdgeDescriptor>::from_iter(cfg.edges().enumerate());

            (edges,edges_rev)
        };

        if let Some(layout) = self.control_flow_layouts.get_mut(uuid) {
            let layout_res = sugiyama::linear_layout_placement(
                &vertices.iter().map(|&vx| vx).collect::<Vec<_>>(),
                &edges,&layout.layout,&dims,
                bb_cmnt_width as f32 + 20.,20.,50.,30.,30.,8.);

            if let Ok(l) = layout_res {
                layout.node_positions = HashMap::from_iter(l.0.into_iter().map(|(idx,pos)| (AdjacencyListVertexDescriptor(idx),pos)));
                layout.node_dimensions = HashMap::from_iter(dims.into_iter().map(|(idx,wh)| (AdjacencyListVertexDescriptor(idx),wh)));
                layout.edges = HashMap::from_iter(l.1.into_iter().map(|(idx,e)| (edges_rev[&idx],e)));
            }
        }

        Ok(())
    }

    fn get_basic_block_line(&self, func: &Uuid, mnemonic: &Mnemonic) -> Result<BasicBlockLine> {
        use panopticon::{
            Rvalue,
            MnemonicFormatToken
        };

        let mut ret = BasicBlockLine{
            opcode: mnemonic.opcode.clone(),
            region: "".to_string(),
            offset: mnemonic.area.start,
            comment: self.control_flow_comments.get(&mnemonic.area.start)
                                               .unwrap_or(&"".to_string()).to_string(),
            args: vec![],
        };
        let mut ops = mnemonic.operands.clone();
        let values = self.control_flow_values.get(func);

        ops.reverse();
        ret.args = mnemonic.format_string.iter().filter_map(|x| match x {
            &MnemonicFormatToken::Literal(ref s) => {
                Some(BasicBlockOperand{
                    kind: "literal",
                    display: s.to_string(),
                    alt: "".to_string(),
                    data: "".to_string(),
                })
            }
            &MnemonicFormatToken::Variable{ ref has_sign } => {
                match ops.pop() {
                    Some(Rvalue::Constant{ value: c, size: s }) => {
                        let val = if s < 64 { c % (1u64 << s) } else { c };
                        let sign_bit = if s < 64 { 1u64 << (s - 1) } else { 0x8000000000000000 };
                        let s = if !has_sign || val & sign_bit == 0 {
                            format!("{:x}",val)
                        } else {
                            format!("{:x}",(val as i64).wrapping_neg())
                        };
                        Some(BasicBlockOperand{
                            kind: "constant",
                            display: s.clone(),
                            alt: "".to_string(),
                            data: s,
                        })
                    },
                    Some(Rvalue::Variable{ ref name, subscript,.. }) => {
                        let data = if let Some(subscript) = subscript {
                            format!("{}_{}",*name,subscript)
                        } else {
                            format!("{}",*name)
                        };
                        let (display,alt) = values
                            .and_then(|x| subscript
                                .and_then(|s| {
                                    let nam = VarName{ name: name.clone(), subscript: s };
                                    x.output.get(&nam)
                                }))
                            .and_then(|val| if val != &Kset::Join && val != &Kset::Meet { Some(val) } else { None })
                            .map(|x| (format!("{}",x),name.to_string()))
                            .unwrap_or_else(|| (name.to_string(),"".to_string()));

                        Some(BasicBlockOperand{
                            kind: "variable",
                            display: display,
                            alt: alt,
                            data: data,
                        })
                    }
                    Some(Rvalue::Undefined) => {
                        Some(BasicBlockOperand{
                            kind: "variable",
                            display: "?".to_string(),
                            alt: "".to_string(),
                            data: "".to_string(),
                        })
                    }
                    None => {
                        error!("mnemonic at {:x} has invalid format string: {:?}",mnemonic.area.start,mnemonic);
                        None
                    }
                }
            }
            &MnemonicFormatToken::Pointer{ is_code,.. } => {
                match ops.pop() {
                    Some(Rvalue::Constant{ value: c, size: s }) => {
                        let val = if s < 64 { c % (1u64 << s) } else { c };
                        let (display,data) = if is_code {
                            let maybe_func = self.functions.iter().find(|&(_,f)| {
                                let maybe_entry = f.entry_point.and_then(|vx| f.cflow_graph.vertex_label(vx));
                                if let Some(&ControlFlowTarget::Resolved(ref bb)) = maybe_entry {
                                    bb.area.start == val
                                } else {
                                    false
                                }
                            });
                            if let Some((_,called_func)) = maybe_func {
                                (called_func.name.clone(),format!("{}",called_func.uuid))
                            } else {
                                (format!("{}",val),"".to_string())
                            }
                        } else {
                            (format!("{}",val),"".to_string())
                        };

                        Some(BasicBlockOperand{
                            kind: if data == "" { "pointer" } else { "function" },
                            display: display,
                            alt: "".to_string(),
                            data: data,
                        })
                    }
                    Some(Rvalue::Variable{ ref name,.. }) => {
                        Some(BasicBlockOperand{
                            kind: "pointer",
                            display: name.to_string(),
                            alt: "".to_string(),
                            data: "".to_string(),
                        })
                    }
                    Some(Rvalue::Undefined) => {
                        Some(BasicBlockOperand{
                            kind: "pointer",
                            display: "?".to_string(),
                            alt: "".to_string(),
                            data: "".to_string(),
                        })
                    }
                    None => {
                        error!("mnemonic at {:x} has invalid format string: {:?}",mnemonic.area.start,mnemonic);
                        None
                    }
                }
            }
        }).collect();

        Ok(ret)
    }

    fn set_control_flow_properties(&mut self, uuid: &Uuid, limit_to: Option<&Vec<u64>>) -> Result<()> {
        let ControlFlowLayout{ node_positions: ref positions, ref edges,.. } = self.control_flow_layouts[uuid].clone();

        if limit_to.is_none() {
            self.control_flow_nodes.clear();
            self.control_flow_edges.clear();
        }

        use std::f32;

        let initial = (f32::INFINITY,f32::INFINITY);
        let (min_x,min_y) = positions
            .iter().fold(initial,|(min_x,min_y),(_,&(x,y))| {
                let min_x = if min_x > x { x } else { min_x };
                let min_y = if min_y > y { y } else { min_y };

                (min_x,min_y)
            });
        let (min_x,min_y) = edges
            .iter().fold((min_x,min_y),|(min_x,min_y),(_,&(ref trail,_,_))| {
                let (x,y) = trail.iter().fold((min_x,min_y),|(min_x,min_y),&(from_x,from_y,to_x,to_y)| {
                    let min_x = if min_x > from_x { from_x } else { min_x };
                    let min_x = if min_x > to_x { to_x } else { min_x };
                    let min_y = if min_y > from_y { from_y } else { min_y };
                    let min_y = if min_y > to_y { to_y } else { min_y };

                    (min_x,min_y)
                });

                let min_x = if min_x > x { x } else { min_x };
                let min_y = if min_y > y { y } else { min_y };

                (min_x,min_y)
            });

        let tuples = positions.iter().filter_map(|(&vx,&(x,y))| {
            let func = &self.functions[uuid];
            let maybe_lb = func.cflow_graph.vertex_label(vx);

            if let Some(&ControlFlowTarget::Resolved(ref bb)) = maybe_lb {
                let b = if let Some(ref addrs) = limit_to {
                    addrs.iter().any(|&x| bb.area.start <= x && bb.area.end > x)
                } else {
                    limit_to.is_none()
                };

                if b {
                    let lines = bb.mnemonics.iter().map(|mne| {
                        self.get_basic_block_line(uuid,mne).unwrap()
                    }).collect::<Vec<_>>();

                    Some((x - min_x,y - min_y,vx.0 as i32,func.entry_point == Some(vx),json::encode(&lines).unwrap()))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect::<Vec<_>>();

        if limit_to.is_none() {
            for (x,y,vx,is_entry,content) in tuples {
                self.control_flow_nodes.append_row(x,y,vx,is_entry,content);
            }
        } else {
            let mut tuples = HashMap::<i32,(_,_,_,_,_)>::from_iter(tuples.into_iter().map(|t| (t.2,t)));
            let num = self.control_flow_nodes.view_data().len();
            let mut values = self.control_flow_nodes.view_data();

            for idx in 0..num {
                let p = tuples.remove(&values[idx].2);

                if let Some(t) = p {
                    values[idx] = t;
                }
            }
            self.control_flow_nodes.set_data(values);

            // don't change edges. this speeds up updates, may produce ugly output.
            return Ok(());
        }

        use rustc_serialize::json;


        for (&edge_desc,&(ref trail,(start_x,start_y),(end_x,end_y))) in edges.iter() {
            let f = |&(x,y,_,_)| (x - min_x,y - min_y);
            let g = |&(_,_,x,y)| (x - min_x,y - min_y);
            let path = trail.clone().iter().take(1).map(&f).chain(trail.iter().map(&g)).collect::<Vec<_>>();
            let (x,y): (Vec<f32>,Vec<f32>) = path.into_iter().unzip();
            let x_res: json::EncodeResult<String> = json::encode(&x);
            let y_res: json::EncodeResult<String> = json::encode(&y);
            let (kind,label) = {
                let func = &self.functions[uuid];
                let cfg = &func.cflow_graph;
                let label = cfg.edge_label(edge_desc).map(|guard| {
                    if *guard != Guard::always() && *guard != Guard::never() {
                        format!("{}",guard)
                    } else {
                        "".to_string()
                    }
                }).unwrap_or("".to_string());
                let from = cfg.source(edge_desc);
                let to = cfg.target(edge_desc);
                let from_addr = cfg.vertex_label(from).and_then(
                    |lb| if let &ControlFlowTarget::Resolved(ref bb) = lb { Some(bb.area.end) } else { None });
                let to_addr = cfg.vertex_label(to).and_then(
                    |lb| if let &ControlFlowTarget::Resolved(ref bb) = lb { Some(bb.area.start) } else { None });
                let kind = if cfg.out_degree(from) >= 2 {
                    if let (Some(from),Some(to)) = (from_addr,to_addr) {
                        if to == from {
                            "fallthru"
                        } else {
                            if from > to {
                                "branch-backedge"
                            } else {
                                "branch"
                            }
                        }
                    } else {
                        "jump"
                    }
                } else {
                    "jump"
                }.to_string();

                (kind,label)
            };

            if let (Ok(x),Ok(y)) = (x_res,y_res) {
                self.control_flow_edges.append_row(
                    x,
                    y,
                    start_x - min_x,
                    start_y - min_y,
                    end_x - min_x,
                    end_y - min_y,
                    kind,
                    label);
            }
        }

        self.control_flow_nodes_changed();
        self.control_flow_edges_changed();
        Ok(())
    }

    pub fn update_basic_block(&mut self, addresses: &Vec<u64>, uuid: &Uuid) -> Result<()> {
        let vis: String = self.get_visible_function().into();
        if vis == uuid.to_string() {
            self.set_control_flow_properties(uuid,Some(addresses))
        } else {
            Ok(())
        }
    }

    pub fn update_sidebar(&mut self, uuid: &Uuid) -> Result<()> {
        let uuid_str = uuid.to_string();
        let maybe_idx = self.sidebar.view_data().iter().position(|&(_,_,ref u)| *u == uuid_str);
        let tpl = {
            let func = &self.functions[uuid];
            let cfg = &func.cflow_graph;
            let entry = func.entry_point.
                and_then(|vx| cfg.vertex_label(vx)).
                and_then(|lb| {
                    if let &ControlFlowTarget::Resolved(ref bb) = lb {
                        Some(bb.area.start)
                    } else {
                        None
                    }
                });
            let str_entry = entry.map(|x| format!("0x{:x}",x)).unwrap_or("".to_string());
            (func.name.to_string(),str_entry,func.uuid.to_string())
        };

        if let Some(pos) = maybe_idx {
            self.sidebar.change_line(pos,tpl.0,tpl.1,tpl.2);
        }

        Ok(())
    }

    fn push_action(&mut self,act: Action) -> Result<()> {
        let top = self.undo_stack_top;

        act.redo(self)?;

        self.undo_stack.truncate(top);
        self.undo_stack.push(act);
        self.undo_stack_top = self.undo_stack.len();

        let top = self.undo_stack_top;
        let len = self.undo_stack.len();

        self.set_can_undo(top != 0);
        self.set_can_redo(top < len);

        self.can_undo_changed();
        self.can_redo_changed();

        Ok(())
    }
}

impl Default for Panopticon {
    fn default() -> Panopticon {
        let sidebar = QSidebar::new();
        let nodes = QControlFlowNodes::new();
        let edges = QControlFlowEdges::new();
        let recent = Self::read_recent_sessions().unwrap_or_else(|_| QRecentSessions::new());

        Panopticon{
            recent_sessions: recent,
            sidebar: sidebar,
            control_flow_nodes: nodes,
            control_flow_edges: edges,
            control_flow_layouts: HashMap::new(),
            control_flow_comments: HashMap::new(),
            control_flow_values: HashMap::new(),
            new_functions: Mutex::new(vec![]),
            functions: HashMap::new(),
            project: None,
            region: None,
            undo_stack: Vec::new(),
            undo_stack_top: 0,
        }
    }
}

Q_OBJECT!(
pub Panopticon as QPanopticon {
    signals:
        fn call_me_maybe();
    slots:
        fn callback();

        // session management
        fn open_program(path: String);
        fn save_session(path: String);

        // control flow / preview
        fn display_control_flow_for(uuid: String);
        fn display_preview_for(uuid: String);

        // actions
        fn comment_on(address: String,comment: String);
        fn rename_function(uuid: String, name: String);
        fn set_value_for(variable: String,value: String);

        // undo/redo
        fn undo();
        fn redo();

    properties:
        initialFile: String; read: get_initial_file, write: set_initial_file, notify: initial_file_changed;

        // recent sessions
        recentSessions: QVariant; read: get_recent_sessions, write: set_recent_sessions, notify: recent_sessions_changed;
        haveRecentSessions: bool; read: get_have_recent_sessions, write: set_have_recent_sessions, notify: have_recent_sessions_changed;
        currentSession: String; read: get_current_session, write: set_current_session, notify: current_session_changed;

        // sidebar
        sidebar: QVariant; read: get_sidebar, write: set_sidebar, notify: sidebar_changed;

        // control flow / preview
        visibleFunction: String; read: get_visible_function, write: set_visible_function, notify: visible_function_changed;
        controlFlowNodes: QVariant; read: get_control_flow_nodes, write: set_control_flow_nodes, notify: control_flow_nodes_changed;
        controlFlowEdges: QVariant; read: get_control_flow_edges, write: set_control_flow_edges, notify: control_flow_edges_changed;
        previewNode: String; read: get_preview_node, write: set_preview_node, notify: preview_node_changed;
        previewFunction: String; read: get_preview_function, write: set_preview_function, notify: preview_function_changed;

        basicBlockPadding: i32; read: get_basic_block_padding, write: set_basic_block_padding, notify: basic_block_padding_changed;
        basicBlockMargin: i32; read: get_basic_block_margin, write: set_basic_block_margin, notify: basic_block_margin_changed;
        basicBlockLineHeight: i32; read: get_basic_block_line_height, write: set_basic_block_line_height, notify: basic_block_line_height_changed;
        basicBlockCharacterWidth: i32; read: get_basic_block_character_width, write: set_basic_block_character_width, notify: basic_block_character_width_changed;
        basicBlockColumnPadding: i32; read: get_basic_block_column_padding, write: set_basic_block_column_padding, notify: basic_block_column_padding_changed;
        basicBlockCommentWidth: i32; read: get_basic_block_comment_width, write: set_basic_block_comment_width, notify: basic_block_comment_width_changed;

        // undo/redo
        canUndo: bool; read: get_can_undo, write: set_can_undo, notify: can_undo_changed;
        canRedo: bool; read: get_can_redo, write: set_can_redo, notify: can_redo_changed;
});
