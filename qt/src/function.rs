/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
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

use panopticon::value::Rvalue;
use panopticon::project::Project;
use panopticon::function::{Function,ControlFlowTarget};
use panopticon::program::CallTarget;
use panopticon::target::Target;
use panopticon::result::{
    Error,
    Result,
};
use panopticon::elf;

use std::hash::{Hash,Hasher,SipHasher};
use std::thread;
use std::fs;
use std::path::Path;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::io::{
    SeekFrom,
    Seek,
    Read,
};

use qmlrs::{Object,Variant};
use graph_algos::{
    VertexListGraphTrait,
    GraphTrait,
    IncidenceGraphTrait,
    EdgeListGraphTrait
};
use uuid::Uuid;
use rustc_serialize::json;
use byteorder::{BigEndian,ReadBytesExt};
use controller::{
    LAYOUTED_FUNCTION,
    CHANGED_FUNCTION,
    return_json,
    Controller,
};

use sugiyama;

#[derive(RustcEncodable)]
struct Metainfo {
    kind: &'static str,
    name: Option<String>,
    uuid: String,
    entry_point: Option<u64>,
    calls: Vec<String>,
}

/// JSON describing the function with UUID `arg`.
///
/// The JSON looks like this:
/// ```json
/// {
///     "type": "function", // or "symbol" or "todo"
///     "name": "func_001", // not present if type is "todo"
///     "uuid": arg,
///     "entry_point": 0x1002,    // optional: entry point
///     "calls": [          // outgoing calls
///         <UUID>,
///         <UUID>,
///         ...
///     ]
/// }
/// ```
pub fn metainfo(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref uuid_str) = arg {
        if let Some(tgt_uuid) = Uuid::parse_str(uuid_str).ok() {
            let ret = Controller::read(|proj| {
                if let Some((vx,prog)) = proj.find_call_target_by_uuid(&tgt_uuid) {
                    // collect called functions' UUIDs
                    let calls = prog.call_graph.out_edges(vx).
                        map(|x| prog.call_graph.target(x)).
                        filter_map(|x| prog.call_graph.vertex_label(x)).
                        map(|x| x.uuid().to_string()).
                        collect();

                    // match function
                    match prog.call_graph.vertex_label(vx) {
                        Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: Some(ref ent), cflow_graph: ref cg,..})) =>
                            // match entry point
                            match cg.vertex_label(*ent) {
                                Some(&ControlFlowTarget::Resolved(ref bb)) =>
                                    return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: Some(bb.area.start), calls: calls })),
                                Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))) =>
                                    return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: Some(*c), calls: calls })),
                                _ =>
                                    return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: None, calls: calls })),
                            },
                        Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: None,..})) =>
                            return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: None, calls: calls })),
                        Some(&CallTarget::Symbolic(ref sym,ref uuid)) =>
                            return_json(Ok(Metainfo{ kind: "symbol", name: Some(sym.clone()), uuid: uuid.to_string(), entry_point: None, calls: calls })),
                        Some(&CallTarget::Todo(Rvalue::Constant(ref a),_,ref uuid)) =>
                            return_json(Ok(Metainfo{ kind: "todo", name: None, uuid: uuid.to_string(), entry_point: Some(*a), calls: calls })),
                        Some(&CallTarget::Todo(_,_,ref uuid)) =>
                            return_json(Ok(Metainfo{ kind: "todo", name: None, uuid: uuid.to_string(), entry_point: None, calls: calls })),
                        None =>
                            return_json::<()>(Err("Internal error".into())),
                    }
                } else {
                    return_json::<()>(Err("No function found for this UUID".into()))
                }
            });
            match ret {
                Ok(s) => s,
                e@Err(_) => return_json(e),
            }
        } else {
            return_json::<()>(Err("1st argument is not a valid UUID".into()))
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

#[derive(RustcEncodable)]
struct CfgEdge {
    from: String,
    to: String,
}

#[derive(RustcEncodable)]
struct CfgMnemonic {
    opcode: String,
    region: String,
    offset: u64,
    comment: String,
    args: Vec<String>,
}

#[derive(RustcEncodable)]
struct ControlFlowGraph {
    entry_point: Option<String>,
    nodes: Vec<String>,
    edges: Vec<CfgEdge>,
    contents: HashMap<String,Vec<CfgMnemonic>>,
}

/// JSON-encoded control flow graph of the function w/ UUID `arg`.
///
/// The JSON will look like this:
/// ```json
/// {
///     "entry_point": <IDENT>,
///     "nodes": [ <IDENT>,... ],
///     "edges": [
///         {"from": <IDENT>, "to": <IDENT>},
///         {"from": <IDENT>, "to": <IDENT>},
///         ...
///     ],
///     "contents": {
///         <IDENT>: [{
///             "opcode": "mov",
///             "reg": "ram",
///             "offset": 100,
///             "args": ["r1", "r2"],
///         },{
///             "opcode": "add",
///             "reg": "ram",
///             "offset": 102,
///             "args": ["1", "r2"],
///         },
///             ...
///         ],
///         ...
///     }
/// }```
pub fn control_flow_graph(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref uuid_str) = arg {
        if let Some(tgt_uuid) = Uuid::parse_str(uuid_str).ok() {
            let ret = Controller::read(|proj| {
                if let Some((vx,prog)) = proj.find_call_target_by_uuid(&tgt_uuid) {
                    if let Some(&CallTarget::Concrete(ref fun)) = prog.call_graph.vertex_label(vx) {
                        let cfg = &fun.cflow_graph;

                        // entry
                        let entry = if let Some(ent) = fun.entry_point {
                            if let Some(a@&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(ent) {
                                Some(to_ident(a))
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        // nodes
                        let nodes = cfg.vertices().
                            filter_map(|x| {
                                cfg.vertex_label(x).map(|x| to_ident(x))
                            }).
                            collect();


                        // basic block contents
                        // XXX: skips unresolved control transfers
                        let contents = cfg.vertices().filter_map(|x| {
                            let lb = cfg.vertex_label(x);
                            match lb {
                                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                                    let mnes = bb.mnemonics.iter().map(|x| {
                                        let args = x.format();
                                        let cmnt = proj.comments.get(&(fun.region.clone(),x.area.start)).unwrap_or(&"".to_string()).clone();
                                        CfgMnemonic{
                                            opcode: x.opcode.clone(),
                                            args: vec![args.clone()],
                                            region: fun.region.clone(),
                                            offset: x.area.start,
                                            comment: cmnt,
                                        }
                                    });
                                    Some((to_ident(lb.unwrap()),mnes.collect()))
                                },
                                _ => None,
                            }
                        });

                        // control flow edges
                        let edges = cfg.edges().filter_map(|x| {
                            let from = cfg.source(x);
                            let to = cfg.target(x);
                            let from_ident = cfg.vertex_label(from).map(to_ident);
                            let to_ident = cfg.vertex_label(to).map(to_ident);

                            if let (Some(f),Some(t)) = (from_ident,to_ident) {
                                Some(CfgEdge{ from: f, to: t })
                            } else {
                                None
                            }
                        }).collect();

                        return_json(Ok(ControlFlowGraph {
                            entry_point: entry,
                            nodes: nodes,
                            edges: edges,
                            contents: HashMap::from_iter(contents),
                        }))
                    } else {
                        return_json::<()>(Err("This function is unresolved".into()))
                    }
                } else {
                    return_json::<()>(Err("No function found for this UUID".into()))
                }
            });
            match ret {
                Ok(s) => s,
                e@Err(_) => return_json(e),
            }
        } else {
            return_json::<()>(Err("1st argument is not a valid UUID".into()))
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

#[derive(Clone,RustcEncodable)]
struct DirectoryEntry {
    path: String,
    name: String,
    is_folder: bool,
}

#[derive(Clone,RustcEncodable)]
struct DirectoryListing {
    current: String,
    parent: String,
    listing: Vec<DirectoryEntry>,
}

pub fn read_directory(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref p) = arg {
        let path = Path::new(p);
        let mut ret = vec![];

        match fs::read_dir(path) {
            Ok(iter) => {
                for maybe_ent in iter {
                    if let Ok(ent) = maybe_ent {
                        if let Some(s) = ent.file_name().to_str() {
                            if let Ok(m) = ent.metadata() {
                                let mut full = path.to_path_buf();
                                full.push(s);
                                if let Some(f) = full.to_str() {
                                    ret.push(DirectoryEntry{
                                        path: f.to_string(),
                                        name: s.to_string(),
                                        is_folder: m.is_dir(),
                                    });
                                }
                            }
                        }
                    }
                }

                return_json(Ok(DirectoryListing{
                    current: p.clone(),
                    parent: path.parent().and_then(|x| x.to_str()).unwrap_or("/").to_string(),
                    listing: ret,
                }))
            },
            Err(e) => return_json::<()>(Err(e.into())),
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

#[derive(RustcEncodable)]
struct FileDetails {
    format: String, // elf, pe, raw
    info: Vec<String>,
}

pub fn file_details(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref p) = arg {
        let path = Path::new(p);
        let ret = fs::File::open(path).and_then(|mut fd| {
            if let Ok(id) = elf::parse::Ident::read(&mut fd) {
                Ok(FileDetails{
                    format: "elf".to_string(),
                    info: vec![format!("{:?}, {:?}",id.class,id.data)],
                })
            } else {
                let mut buf = [0u8;2];

                try!(fd.seek(SeekFrom::Start(0)));
                try!(fd.read(&mut buf));

                if buf == [0x4d,0x5a] {
                    Ok(FileDetails{
                        format: "pe".to_string(),
                        info: vec!["PE".to_string()],
                    })
                } else {
                    let mut magic = [0u8;10];

                    try!(fd.seek(SeekFrom::Start(0)));
                    if try!(fd.read(&mut magic)) == 10 && magic == *b"PANOPTICON" {
                        let version = try!(fd.read_u32::<BigEndian>());

                        Ok(FileDetails{
                            format: "panop".to_string(),
                            info: vec![format!("Version {}",version)],
                        })
                    } else {
                        Ok(FileDetails{
                            format: "raw".to_string(),
                            info: vec![],
                        })
                    }
                }
            }
        });

        match ret {
            Ok(f) => return_json(Ok(f)),
            Err(err) => return_json::<FileDetails>(Err(err.into())),
        }
    } else {
        return_json::<FileDetails>(Err("1st argument is not a string".into()))
    })
}

/// Returns the unique string identifier for `t`.
fn to_ident(t: &ControlFlowTarget) -> String {
    match t {
        &ControlFlowTarget::Resolved(ref bb) =>
            format!("bb{}",bb.area.start),
        &ControlFlowTarget::Unresolved(ref c) => {
            let ref mut h = SipHasher::new();
            c.hash::<SipHasher>(h);
            format!("c{}",h.finish())
        }
    }
}

#[derive(RustcDecodable,Debug)]
struct LayoutInputDimension {
    width: f32,
    height: f32,
}

#[derive(RustcEncodable,Debug)]
struct LayoutOutputPosition {
    x: f32,
    y: f32,
}

#[derive(RustcEncodable,Debug)]
struct LayoutOutputEdge {
    segments: Vec<LayoutOutputSegment>,
    head_offset: LayoutOutputPosition,
    tail_offset: LayoutOutputPosition,
}

#[derive(RustcEncodable,Debug)]
struct LayoutOutputSegment {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

/// Layout a control flow graph.
///
/// Uses a layered graph drawing algorithm (Sugiyama's method) the
/// lay out a directed control flow graph.
///
/// Input has to look loke this:
/// ```json
/// {
///     "<ID>": {
///         "height": <NUM>,
///         "width": <NUM,
///     },
///     ...
/// }```
///
/// Output:
/// ```json
/// {
///     <ID>: {
///         "x": <X-COORD>,
///         "y": <Y-COORD>
///     }
/// }```
pub fn layout(arg0: &Variant, arg1: &Variant, arg2: &Variant, arg3: &Variant, arg4: &Variant) -> Variant {
    let dims = if let &Variant::String(ref st) = arg1 {
        match json::decode::<HashMap<String,LayoutInputDimension>>(st) {
            Ok(input) => {
                input
            },
            Err(e) => {
               return Variant::String(return_json::<()>(Err(e.into())));
            }
        }
    } else {
        return Variant::String(return_json::<()>(Err("1st argument is not valid JSON".into())));
    };

    let rank_spacing = if let &Variant::I64(ref x) = arg2 {
        *x
    } else {
        return Variant::String(return_json::<()>(Err("2nd argument is not an integer".into())));
    };

    let node_spacing = if let &Variant::I64(ref x) = arg3 {
        *x
    } else {
        return Variant::String(return_json::<()>(Err("3rd argument is not an integer".into())));
    };

    let port_spacing = if let &Variant::I64(ref x) = arg4 {
        *x
    } else {
        return Variant::String(return_json::<()>(Err("4th argument is not an integer".into())));
    };

    if let &Variant::String(ref st) = arg0 {
        if let Some(uuid) = Uuid::parse_str(st).ok() {
            let ret = Controller::read(|proj| {
                if let Some(func) = proj.find_function_by_uuid(&uuid) {
                    let vertices = func.cflow_graph.vertices().collect::<Vec<_>>();
                    let edges = func.cflow_graph.edges().map(|e| {
                        let f = vertices.iter().position(|&x| x == func.cflow_graph.source(e)).unwrap();
                        let t = vertices.iter().position(|&x| x == func.cflow_graph.target(e)).unwrap();
                        (f,t)
                    }).collect::<Vec<_>>();
                    let mut dims_transformed = HashMap::<usize,(f32,f32)>::new();

                    for (k,v) in dims.iter() {
                        let _k = vertices.iter().position(|&x| {
                            let a = to_ident(func.cflow_graph.vertex_label(x).unwrap());
                            a == *k
                        }).unwrap();
                        dims_transformed.insert(_k,(v.width as f32,v.height as f32));
                    }
                    let maybe_entry = func.entry_point.map(|k| vertices.iter().position(|&x| x == k).unwrap());
                    let idents = func.cflow_graph.vertices().map(|x| to_ident(func.cflow_graph.vertex_label(x).unwrap())).collect::<Vec<_>>();

                    Some((maybe_entry,idents,dims_transformed,vertices,edges))
                } else {
                    None
                }
            });

            if let Ok(Some((maybe_entry,idents,dims_transformed,vertices,edges))) = ret {
                thread::spawn(move || -> Result<()> {
                    match sugiyama::layout(&(0..vertices.len()).collect::<Vec<usize>>(),
                                           &edges,
                                           &dims_transformed,
                                           maybe_entry,
                                           node_spacing as usize,
                                           rank_spacing as usize,
                                           port_spacing as usize) {
                        Ok(res) => {
                            let mut ret_v = HashMap::<String,LayoutOutputPosition>::new();
                            let mut ret_e = HashMap::<usize,LayoutOutputEdge>::new();
                            for (k,v) in (res.0).iter() {
                                ret_v.insert(idents[*k].clone(),LayoutOutputPosition{ x: v.0 as f32, y: v.1 as f32 });
                            }
                            for v in (res.1).iter() {
                                ret_e.insert(*v.0,LayoutOutputEdge{
                                    segments: (v.1).0.iter().map(|w| {
                                        LayoutOutputSegment{
                                            x1: w.0 as f32,
                                            y1: w.1 as f32,
                                            x2: w.2 as f32,
                                            y2: w.3 as f32,
                                        }}).collect::<Vec<_>>(),
                                    tail_offset: LayoutOutputPosition{
                                        x: ((v.1).1).0,
                                        y: ((v.1).1).1,
                                    },
                                    head_offset: LayoutOutputPosition{
                                        x: ((v.1).2).0,
                                        y: ((v.1).2).1,
                                    }
                                });
                            }
                            Controller::emit1(LAYOUTED_FUNCTION,&try!(json::encode(&(ret_v,ret_e))))
                        },
                        // XXX tell the frontend
                        Err(e) => Err(Error(e.into())),
                    }
                });
                Variant::String(return_json(Ok(())))
            } else {
                Variant::String(return_json::<()>(Err("Function not found".into())))
            }
        } else {
            Variant::String(return_json::<()>(Err("Not a valid UUID".into())))
        }
    } else {
        Variant::String(return_json::<()>(Err("4th argument is not an integer".into())))
    }
}

pub fn comment(arg0: &Variant, arg1: &Variant, arg2: &Variant) -> Variant {
    let reg = if let &Variant::String(ref x) = arg0 {
        x.clone()
    } else {
        return Variant::String(return_json::<()>(Err("1st argument is not a string".into())));
    };

    let offset = if let &Variant::I64(ref x) = arg1 {
        *x as u64
    } else {
        return Variant::String(return_json::<()>(Err("2nd argument is not an integer".into())));
    };

    let cmnt = if let &Variant::String(ref x) = arg2 {
        x.clone()
    } else {
        return Variant::String(return_json::<()>(Err("3rd argument is not a string".into())));
    };

    // write comment
    Variant::String(return_json(Controller::modify(|proj| {
        proj.comments.insert((reg.clone(),offset),cmnt);
    }).and(Controller::read(|proj| {
        for prog in proj.code.iter() {
            for ct in prog.call_graph.vertices() {
                match prog.call_graph.vertex_label(ct) {
                    Some(&CallTarget::Concrete(ref func)) => {
                        if func.region == reg {
                            // XXX: check offset?
                            Controller::emit1(CHANGED_FUNCTION,&func.uuid.to_string());
                        }
                    },
                    _ => {}
                }
            }
        }
    }))))
}

pub fn rename(arg0: &Variant, arg1: &Variant) -> Variant {
    let name = if let &Variant::String(ref x) = arg1 {
        x.clone()
    } else {
        return Variant::String(return_json::<()>(Err("1st argument is not a string".into())));
    };

    let maybe_uu = if let &Variant::String(ref st) = arg0 {
        if let Some(uuid) = Uuid::parse_str(st).ok() {
            Controller::modify(|proj| {
                if let Some(func) = proj.find_function_by_uuid_mut(&uuid) {
                    func.name = name;
                    Some(uuid.clone())
                } else {
                    None
                }
            }).ok()
        } else {
            None
        }
    } else {
        None
    };

    if let Some(Some(uu)) = maybe_uu {
        Controller::emit1(CHANGED_FUNCTION,&uu.to_string());
        Variant::String(return_json(Ok(())))
    } else {
        Variant::String(return_json::<()>(Err("No function found for this UUID".into())))
    }
}

pub fn targets() -> Variant {
    Variant::String(return_json(Ok(Target::all().iter().map(|x| x.name()).collect::<Vec<_>>())))
}
