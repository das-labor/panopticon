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
use std::collections::HashMap;
use panopticon::{
    Project
};
use uuid::Uuid;
use errors::*;

Q_LISTMODEL! {
    pub QRecentSessions {
        timestamp: i32,
        title: String,
        typ: String,
        path: String
    }
}

Q_LISTMODEL! {
    pub QFunctions {
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
    nodeDimensions: HashMap<usize,(f32,f32)>,
    //layout: sugiyama::LinearLayout,
    nodePositions: HashMap<usize,(f32,f32)>,
    edges: HashMap<usize,(Vec<(f32,f32,f32,f32)>,(f32,f32),(f32,f32))>,
}

pub struct Panopticon {
    // QML
    pub recentSessions: QRecentSessions,
    pub functions: QFunctions,
    pub controlFlowNodes: QControlFlowNodes,
    pub controlFlowEdges: QControlFlowEdges,

    pub controlFlowLayouts: HashMap<Uuid,ControlFlowLayout>,

    pub controlFlowComments: HashMap<u64,String>,
    pub controlFlowValues: HashMap<(Uuid,String),String>,

    pub project: Option<Project>,
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

    fn open_program(&mut self,path: String) -> Option<&QVariant> {
        use std::path::Path;
        use panopticon::{ControlFlowTarget,CallTarget};
        use graph_algos::{
            VertexListGraphTrait,
            GraphTrait,
        };


        debug!("open_program() path={}",path);
        debug_assert!(self.project.is_none());

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

                        self.functions.append_row(func.name.to_string(),str_entry,func.uuid.to_string());
                    }
                }
            }
        }

        // XXX: error
        None
        /*for &(name,uuid,pos) in SIDEBAR.iter() {
        }*/

        //self.functions_changed();
        //self.set_visible_function("".to_string());
    }

}

impl Default for Panopticon {
    fn default() -> Panopticon {
        let functions = QFunctions::new();
        let controlFlowNodes = QControlFlowNodes::new();
        let controlFlowEdges = QControlFlowEdges::new();
        let recentSessions = Self::read_recent_sessions().unwrap_or_else(|_| QRecentSessions::new());

        Panopticon{
            recentSessions: recentSessions,
            functions: functions,
            controlFlowNodes: controlFlowNodes,
            controlFlowEdges: controlFlowEdges,
            controlFlowLayouts: HashMap::new(),
            controlFlowComments: HashMap::new(),
            controlFlowValues: HashMap::new(),
            project: None,
        }
    }
}

Q_OBJECT!(
pub Panopticon as QPanopticon {
  signals:
    slots:
        //fn new_program(path: String);
        fn open_program(path: String);
    properties:
        // recent sessions
        recentSessions: QVariant; read: get_recent_sessions, write: set_recent_sessions, notify: recent_sessions_changed;
        haveRecentSessions: bool; read: get_have_recent_sessions, write: set_have_recent_sessions, notify: have_recent_sessions_changed;

        // functions
        functions: QVariant; read: get_functions, write: set_functions, notify: functions_changed;
});
