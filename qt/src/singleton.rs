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
use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;
use panopticon::{
    Project,
    loader,
    Function,
    ControlFlowTarget,
};
use uuid::Uuid;
use parking_lot::Mutex;
use errors::*;
use sugiyama;
use graph_algos::{
    GraphTrait,
    MutableGraphTrait,
    VertexListGraphTrait,
    EdgeListGraphTrait,
};

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

Q_LISTMODEL! {
    pub QTasks {
        title: String,
        description: String,
        state: String,
        uuid: String
    }
}

#[derive(Clone)]
pub struct ControlFlowLayout {
    node_dimensions: HashMap<usize,(f32,f32)>,
    layout: sugiyama::LinearLayout,
    node_positions: HashMap<usize,(f32,f32)>,
    edges: HashMap<usize,(Vec<(f32,f32,f32,f32)>,(f32,f32),(f32,f32))>,
}

pub struct Panopticon {
    // QML
    pub recent_sessions: QRecentSessions,
    pub sidebar: QSidebar,
    pub control_flow_nodes: QControlFlowNodes,
    pub control_flow_edges: QControlFlowEdges,
    pub tasks: QTasks,

    pub control_flow_layouts: HashMap<Uuid,ControlFlowLayout>,

    pub control_flow_comments: HashMap<u64,String>,
    pub control_flow_values: HashMap<(Uuid,String),String>,

    pub new_functions: Mutex<Vec<Function>>,
    pub functions: HashMap<Uuid,Function>,
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
                    }
                }
            }
        } else if let Ok((mut proj,machine)) = loader::load(&Path::new(&path)) {
            let maybe_prog = proj.code.pop();
            let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap().clone();

            if let Some(prog) = maybe_prog {
                let pipe = match machine {
                    Machine::Avr => pipeline::<avr::Avr>(prog,reg,avr::Mcu::atmega103()),
                    Machine::Ia32 => pipeline::<amd64::Amd64>(prog,reg,amd64::Mode::Protected),
                    Machine::Amd64 => pipeline::<amd64::Amd64>(prog,reg,amd64::Mode::Long),
                };

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
            }
        }


        // XXX: error
        None
    }

    fn display_control_flow_for(&mut self, uuid_str: String) -> Option<&QVariant> {
        use std::str::FromStr;

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
                    node_positions: HashMap::from_iter(vertices.iter().map(|&vx| (vx,(0.,0.)))),
                    edges: HashMap::new(),
                };
                self.control_flow_layouts.insert(uuid.clone(),layout.clone());
            } else {
                return None
            }
        }

        let need_dims = self.control_flow_layouts[&uuid].node_dimensions.is_empty();

        if need_dims {
            self.update_control_flow_dimensions(&uuid);
        }

        //self.set_control_flow_properties(&uuid);
        self.set_visible_function(uuid.to_string());
        println!("layout done");
        None
    }

    fn update_control_flow_dimensions(&mut self,uuid: &Uuid) -> Result<()> {
        use rustc_serialize::json;

        info!("update_control_flow_dimensions() uuid={}",uuid);
        let bb_char_width = self.get_basic_block_character_width().to_int() as usize;
        let bb_padding = self.get_basic_block_padding().to_int() as usize;
        let bb_margin = self.get_basic_block_margin().to_int() as usize;
        let bb_col_padding = self.get_basic_block_column_padding().to_int() as usize;
        let bb_line_height = self.get_basic_block_line_height().to_int() as usize;
        let bb_cmnt_width = self.get_basic_block_comment_width().to_int() as usize;
        let mut vertices = HashSet::new();
        let mut edges = vec![];
        let mut dims = HashMap::new();

        {
            let func = &self.functions[uuid];
            let cfg = &func.cflow_graph;

            for vx in cfg.vertices() {
                let maybe_lb = cfg.vertex_label(vx);

                match maybe_lb {
                    Some(&ControlFlowTarget::Resolved(ref bb)) => {
                        let linew = bb.mnemonics.iter().map(|mne| {
                            mne.opcode.len() + mne.operands.iter().map(|a| format!("{}",a).len()).sum::<usize>()
                        }).max().unwrap_or(0);
                        let has_cmnt = bb.mnemonics.iter().any(|mne| {
                            self.control_flow_comments.contains_key(&mne.area.start)
                        });
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

            let edge_iter = cfg.edges().map(|e| (cfg.source(e).0,cfg.target(e).0));

            for (from_idx,to_idx) in edge_iter {
                edges.push((from_idx,to_idx));
            }
        }

        if let Some(layout) = self.control_flow_layouts.get_mut(uuid) {
            let layout_res = sugiyama::linear_layout_placement(
                &vertices.iter().map(|&vx| vx).collect::<Vec<_>>(),
                &edges,&layout.layout,&dims,
                bb_cmnt_width as f32 + 20.,20.,50.,30.,30.,8.);

            if let Ok(l) = layout_res {
                layout.node_positions = l.0;
                layout.node_dimensions = dims;
                layout.edges = l.1;
            }
        }

        Ok(())
    }

    fn set_control_flow_properties(&mut self, uuid: &Uuid) -> Result<()> {
        let ControlFlowLayout{ node_positions: ref positions, ref edges, ref node_dimensions,.. } = self.control_flow_layouts[uuid].clone();

        self.control_flow_nodes.clear();
        self.control_flow_edges.clear();

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

        for (&vx,&(x,y)) in positions.iter() {
            let mut contents = json::decode::<Vec<CfgMnemonic>>(&FUNCTION.1[vx].2).unwrap();

            for mne in contents.iter_mut() {
                if let Some(cmnt) = self.controlFlowComments.get(&mne.offset) {
                    mne.comment = cmnt.clone();
                }
            }

            self.control_flow_nodes.append_row(x - min_x,y - min_y,vx as i32,FUNCTION.0 == vx,json::encode(&contents).unwrap());
        }

        use rustc_serialize::json;

        for (&edge_idx,&(ref trail,(start_x,start_y),(end_x,end_y))) in edges.iter() {
            let f = |&(x,y,_,_)| (x - min_x,y - min_y);
            let g = |&(_,_,x,y)| (x - min_x,y - min_y);
            let path = trail.clone().iter().take(1).map(&f).chain(trail.iter().map(&g)).collect::<Vec<_>>();
            let (mut x,mut y): (Vec<f32>,Vec<f32>) = path.into_iter().unzip();
            let x_res: json::EncodeResult<String> = json::encode(&x);
            let y_res: json::EncodeResult<String> = json::encode(&y);
            let from = FUNCTION.2[edge_idx].0;
            let to = FUNCTION.2[edge_idx].1;
            let nodes = &FUNCTION.1;

            if let (Ok(x),Ok(y)) = (x_res,y_res) {
                self.control_flow_edges.append_row(
                    x,
                    y,
                    start_x - min_x,
                    start_y - min_y,
                    end_x - min_x,
                    end_y - min_y,
                    FUNCTION.2[edge_idx].2.clone(),
                    FUNCTION.2[edge_idx].3.clone());
            }
        }

        self.control_flow_nodes_changed();
        self.control_flow_edges_changed();
        Ok(())
    }
}

impl Default for Panopticon {
    fn default() -> Panopticon {
        let sidebar = QSidebar::new();
        let nodes = QControlFlowNodes::new();
        let edges = QControlFlowEdges::new();
        let tasks = QTasks::new();
        let recent = Self::read_recent_sessions().unwrap_or_else(|_| QRecentSessions::new());

        Panopticon{
            recent_sessions: recent,
            sidebar: sidebar,
            control_flow_nodes: nodes,
            control_flow_edges: edges,
            tasks: tasks,
            control_flow_layouts: HashMap::new(),
            control_flow_comments: HashMap::new(),
            control_flow_values: HashMap::new(),
            new_functions: Mutex::new(vec![]),
            functions: HashMap::new(),
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

        // control flow / preview
        fn display_control_flow_for(uuid: String);

    properties:
        initialFile: String; read: get_initial_file, write: set_initial_file, notify: initial_file_changed;

        // recent sessions
        recentSessions: QVariant; read: get_recent_sessions, write: set_recent_sessions, notify: recent_sessions_changed;
        haveRecentSessions: bool; read: get_have_recent_sessions, write: set_have_recent_sessions, notify: have_recent_sessions_changed;

        // sidebar
        sidebar: QVariant; read: get_sidebar, write: set_sidebar, notify: sidebar_changed;

        // tasks
        tasks: QVariant; read: get_tasks, write: set_tasks, notify: tasks_changed;

        // control flow / preview
        visibleFunction: String; read: get_visible_function, write: set_visible_function, notify: visible_function_changed;
        controlFlowNodes: QVariant; read: get_control_flow_nodes, write: set_control_flow_nodes, notify: control_flow_nodes_changed;
        controlFlowEdges: QVariant; read: get_control_flow_edges, write: set_control_flow_edges, notify: control_flow_edges_changed;
        //previewNode: String; read: get_preview_node, write: set_preview_node, notify: preview_node_changed;
        //previewFunction: String; read: get_preview_function, write: set_preview_function, notify: preview_function_changed;

        basicBlockPadding: i32; read: get_basic_block_padding, write: set_basic_block_padding, notify: basic_block_padding_changed;
        basicBlockMargin: i32; read: get_basic_block_margin, write: set_basic_block_margin, notify: basic_block_margin_changed;
        basicBlockLineHeight: i32; read: get_basic_block_line_height, write: set_basic_block_line_height, notify: basic_block_line_height_changed;
        basicBlockCharacterWidth: i32; read: get_basic_block_character_width, write: set_basic_block_character_width, notify: basic_block_character_width_changed;
        basicBlockColumnPadding: i32; read: get_basic_block_column_padding, write: set_basic_block_column_padding, notify: basic_block_column_padding_changed;
        basicBlockCommentWidth: i32; read: get_basic_block_comment_width, write: set_basic_block_comment_width, notify: basic_block_comment_width_changed;
});
