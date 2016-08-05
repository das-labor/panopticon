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

use panopticon;
use panopticon::{
    Rvalue,Lvalue,
    Function,ControlFlowTarget,
    CallTarget,
    MnemonicFormatToken,
    Error,
    Result,
    elf,
    Kset,
};

use std::hash::{Hash,Hasher,SipHasher};
use std::thread;
use std::fs;
use std::path::{PathBuf};
use std::iter::FromIterator;
use std::collections::{HashSet,HashMap};
use std::io::{
    SeekFrom,
    Seek,
    Read,
};
use std::env::home_dir;

use qmlrs::{Variant};
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
///     "kind": "function",     // or "symbol" or "todo"
///     "name": "func_001",     // not present if type is "todo"
///     "uuid": <UUID>,
///     "entry_point": 0x1002,  // optional: entry point
///     "calls": [              // outgoing calls
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
                                Some(&ControlFlowTarget::Unresolved(Rvalue::Constant{ value: c,.. })) =>
                                    return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: Some(c), calls: calls })),
                                Some(&ControlFlowTarget::Unresolved(_)) =>
                                    return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: None, calls: calls })),
                                Some(&ControlFlowTarget::Failed(pos,_)) =>
                                    return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: Some(pos), calls: calls })),
                                None => unreachable!(),
                            },
                        Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: None,..})) =>
                            return_json(Ok(Metainfo{ kind: "function", name: Some(name.clone()), uuid: uuid.to_string(), entry_point: None, calls: calls })),
                        Some(&CallTarget::Symbolic(ref sym,ref uuid)) =>
                            return_json(Ok(Metainfo{ kind: "symbol", name: Some(sym.clone()), uuid: uuid.to_string(), entry_point: None, calls: calls })),
                        Some(&CallTarget::Todo(Rvalue::Constant{ value: a,.. },_,ref uuid)) =>
                            return_json(Ok(Metainfo{ kind: "todo", name: None, uuid: uuid.to_string(), entry_point: Some(a), calls: calls })),
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
    args: Vec<CfgOperand>,
}

#[derive(RustcEncodable)]
struct CfgOperand {
    kind: &'static str, // constant, variable, function, literal
    display: String, // string to display
    data: String, // constant: value, variable: ssa var, function: UUID, literal: empty string
}

#[derive(RustcEncodable)]
struct ControlFlowGraph {
    entry_point: Option<String>,
    nodes: Vec<String>,
    edges: Vec<CfgEdge>,
    code: HashMap<String,Vec<CfgMnemonic>>,
    targets: HashMap<String,String>,
    errors: HashMap<String,String>,
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
///     "code": {
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
///     "targets": {
///         <IDENT>: <TARGET>,
///         ...
///     },
///     "errors": {
///         <IDENT>: <MSG>,
///         ...
///     },
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
                        let code = cfg.vertices().filter_map(|x| {
                            let lb = cfg.vertex_label(x);
                            match lb {
                                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                                    let mnes = bb.mnemonics.iter().filter_map(|x| {
                                        if x.opcode.starts_with("__") {
                                            None
                                        } else {
                                            let mut ops = x.operands.clone();
                                            ops.reverse();
                                            let args = x.format_string.iter().map(|x| match x {
                                                &MnemonicFormatToken::Literal(ref s) =>
                                                    CfgOperand{
                                                        kind: "literal",
                                                        display: s.to_string(),
                                                        data: "".to_string(),
                                                    },
                                                &MnemonicFormatToken::Variable{ ref has_sign } =>
                                                    match ops.pop() {
                                                        Some(Rvalue::Constant{ value: c, size: s }) => {
                                                            let val = if s < 64 { c % (1u64 << s) } else { c };
                                                            let sign_bit = if s < 64 { 1u64 << (s - 1) } else { 0x8000000000000000 };
                                                            let s = if !has_sign || val & sign_bit == 0 {
                                                                format!("{}",val)
                                                            } else {
                                                                format!("{}",(val as i64).wrapping_neg())
                                                            };
                                                            CfgOperand{
                                                                kind: "constant",
                                                                display: s.clone(),
                                                                data: s,
                                                            }
                                                        },
                                                        Some(Rvalue::Variable{ ref name, subscript: Some(ref subscript),.. }) =>
                                                            CfgOperand{
                                                                kind: "variable",
                                                                display: name.to_string(),
                                                                data: format!("{}_{}",*name,*subscript),
                                                            },
                                                        _ =>
                                                            CfgOperand{
                                                                kind: "variable",
                                                                display: "?".to_string(),
                                                                data: "".to_string(),
                                                            },
                                                    },
                                                &MnemonicFormatToken::Pointer{ is_code,.. } =>
                                                    match ops.pop() {
                                                        Some(Rvalue::Constant{ value: c, size: s }) => {
                                                            let val = if s < 64 { c % (1u64 << s) } else { c };
                                                            let (display,data) = if is_code {
                                                                if let Some(vx) = prog.find_function_by_entry(val) {
                                                                    if let Some(&CallTarget::Concrete(Function{ ref name, ref uuid,.. })) = prog.call_graph.vertex_label(vx) {
                                                                        (name.clone(),format!("{}",uuid))
                                                                    } else {
                                                                        (format!("{}",val),"".to_string())
                                                                    }
                                                                } else {
                                                                    (format!("{}",val),"".to_string())
                                                                }
                                                            } else {
                                                                (format!("{}",val),"".to_string())
                                                            };

                                                            CfgOperand{
                                                                kind: "pointer",
                                                                display: display,
                                                                data: data,
                                                            }
                                                        },
                                                        Some(Rvalue::Variable{ ref name, subscript: Some(_),.. }) =>
                                                            CfgOperand{
                                                                kind: "pointer",
                                                                display: name.to_string(),
                                                                data: "".to_string(),
                                                            },
                                                        _ =>
                                                            CfgOperand{
                                                                kind: "pointer",
                                                                display: "?".to_string(),
                                                                data: "".to_string(),
                                                            },
                                                    },
                                            });
                                            let cmnt = proj.comments.get(&(fun.region.clone(),x.area.start)).unwrap_or(&"".to_string()).clone();
                                            Some(CfgMnemonic{
                                                opcode: x.opcode.clone(),
                                                args: args.collect(),
                                                region: fun.region.clone(),
                                                offset: x.area.start,
                                                comment: cmnt,
                                            })
                                        }
                                    });
                                    Some((to_ident(lb.unwrap()),mnes.collect()))
                                },
                                _ => None,
                            }
                        });
                        let targets = cfg.vertices().filter_map(|x| {
                            let lb = cfg.vertex_label(x);
                            match lb {
                                Some(&ControlFlowTarget::Unresolved(ref rv)) =>
                                    Some((to_ident(lb.unwrap()),format!("{}",rv))),
                                _ => None,
                            }
                        });
                        let errors = cfg.vertices().filter_map(|x| {
                            let lb = cfg.vertex_label(x);
                            match lb {
                                Some(&ControlFlowTarget::Failed(_,ref msg)) =>
                                    Some((to_ident(lb.unwrap()),format!("{}",msg))),
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
                            code: HashMap::from_iter(code),
                            targets: HashMap::from_iter(targets),
                            errors: HashMap::from_iter(errors),
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

pub fn approximate(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref uuid_str) = arg {
        if let Some(tgt_uuid) = Uuid::parse_str(uuid_str).ok() {
            let ret = Controller::read(|proj| {
                if let Some((vx,prog)) = proj.find_call_target_by_uuid(&tgt_uuid) {
                    if let Some(&CallTarget::Concrete(ref fun)) = prog.call_graph.vertex_label(vx) {
                        return_json(panopticon::approximate::<Kset>(&fun).and_then(|x| Ok(x.iter().filter_map(|(k,v)| {
                            if let &Lvalue::Variable{ ref name, subscript: Some(ref subscript),.. } = k {
                                if let &Kset::Set(ref s) = v {
                                    if s.len() == 1 {
                                        return Some((format!("{}_{}",*name,*subscript),format!("{}",s[0].0)));
                                    } else if s.len() > 1 {
                                        return Some((format!("{}_{}",*name,*subscript),format!("{:?}",s.iter().map(|&(a,_)| a).collect::<Vec<_>>())));
                                    }
                                }
                            }
                            return None;
                        }).collect::<Vec<_>>())))
                    } else {
                        return_json::<String>(Err("This function is unresolved".into()))
                    }
                } else {
                    return_json::<String>(Err("No function found for this UUID".into()))
                }
            });
            match ret {
                Ok(s) => s,
                e@Err(_) => return_json::<String>(e),
            }
        } else {
            return_json::<String>(Err("1st argument is not a valid UUID".into()))
        }
    } else {
        return_json::<String>(Err("1st argument is not a string".into()))
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
        let path = if p == "" {
            if let Some(ref home) = home_dir() {
                home.clone()
            } else {
                PathBuf::from("/")
            }
        } else {
            PathBuf::from(p)
        };
        let mut ret = vec![];

        match fs::read_dir(&path) {
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
                    current: path.to_str().unwrap_or(p).to_string(),
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
    state: String, // writable, readable, directory, free, inaccessable
    format: Option<String>, // elf, pe, raw
    info: Vec<String>,
}

pub fn file_details(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref p) = arg {
        let path = PathBuf::from(p);
        let ret: Result<FileDetails> = fs::File::open(&path).and_then(|mut fd| {
            let meta = try!(fd.metadata());

            if meta.is_dir() {
                Ok(FileDetails{
                    state: "directory".to_string(),
                    format: None,
                    info: vec![],
                })
            } else {
                let ro = meta.permissions().readonly();

                if let Ok(id) = elf::parse::Ident::read(&mut fd) {
                    Ok(FileDetails{
                        state: if ro { "readable" } else { "writable" }.to_string(),
                        format: Some("elf".to_string()),
                        info: vec![format!("{:?}, {:?}",id.class,id.data)],
                    })
                } else {
                    let mut buf = [0u8;2];

                    try!(fd.seek(SeekFrom::Start(0)));
                    try!(fd.read(&mut buf));

                    if buf == [0x4d,0x5a] {
                        Ok(FileDetails{
                            state: if ro { "readable" } else { "writable" }.to_string(),
                            format: Some("pe".to_string()),
                            info: vec!["PE".to_string()],
                        })
                    } else {
                        let mut magic = [0u8;10];

                        try!(fd.seek(SeekFrom::Start(0)));
                        if try!(fd.read(&mut magic)) == 10 && magic == *b"PANOPTICON" {
                            let version = try!(fd.read_u32::<BigEndian>());

                            Ok(FileDetails{
                                state: if ro { "readable" } else { "writable" }.to_string(),
                                format: Some("panop".to_string()),
                                info: vec![format!("Version {}",version)],
                            })
                        } else {
                            Ok(FileDetails{
                                state: if ro { "readable" } else { "writable" }.to_string(),
                                format: Some("raw".to_string()),
                                info: vec![],
                            })
                        }
                    }
                }
            }
        }).or_else(|_| {
            if let Some(parent) = path.parent() {
                if let Ok(fd) = fs::File::open(parent) {
                    Ok(FileDetails{
                        state: if try!(fd.metadata()).permissions().readonly() { "inaccessible" } else { "free" }.to_string(),
                        format: None,
                        info: vec![],
                    })
                } else {
                    Ok(FileDetails{
                        state: "inaccessible".to_string(),
                        format: None,
                        info: vec![],
                    })
                }
            } else {
                Ok(FileDetails{
                    state: "inaccessible".to_string(),
                    format: None,
                    info: vec![],
                })
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
        &ControlFlowTarget::Failed(ref pos,_) =>
            format!("err{}",pos),
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
                    let mut unseen_verts = HashSet::<_>::from_iter(vertices.iter().map(|v|
                        to_ident(func.cflow_graph.vertex_label(*v).unwrap())));

                    for (k,v) in dims.iter() {
                        let _k = vertices.iter().position(|&x| {
                            let a = to_ident(func.cflow_graph.vertex_label(x).unwrap());
                            a == *k
                        }).unwrap();
                        unseen_verts.remove(&to_ident(func.cflow_graph.vertex_label(vertices[_k]).unwrap()));
                        dims_transformed.insert(_k,(v.width as f32,v.height as f32));
                    }
                    let maybe_entry = func.entry_point.map(|k| vertices.iter().position(|&x| x == k).unwrap());
                    let idents = vertices.iter().map(|x| to_ident(func.cflow_graph.vertex_label(*x).unwrap())).collect::<Vec<_>>();

                    Some((maybe_entry,idents,dims_transformed,vertices,edges,unseen_verts))
                } else {
                    None
                }
            });

            if let Ok(Some((maybe_entry,idents,dims_transformed,vertices,edges,unseen_verts))) = ret {
                if !unseen_verts.is_empty() {
                    let e = format!("Missing dimension for {:?}",unseen_verts);
                    return Variant::String(return_json::<()>(Err(e.into())));
                }

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
                            Controller::emit(LAYOUTED_FUNCTION,&try!(json::encode(&(ret_v,ret_e))))
                        },
                        // XXX tell the frontend
                        Err(e) => {
                            println!("layouting thread failed with '{:?}'",e);
                            Err(Error(e.into()))
                        },
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
                            try!(Controller::emit(CHANGED_FUNCTION,&func.uuid.to_string()));
                        }
                    },
                    _ => {},
                }
            }
        }
        Ok(())
    }).and_then(|x| x))))
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
        Variant::String(return_json(Controller::emit(CHANGED_FUNCTION,&uu.to_string())))
    } else {
        Variant::String(return_json::<()>(Err("No function found for this UUID".into())))
    }
}

#[derive(RustcEncodable,Debug)]
struct SessionInfo {
    title: String,
    age: String,
    file: String,
    path: String,
}

pub fn sessions() -> Variant {
    use std::time::SystemTime;
    use chrono;
    use chrono_humanize::HumanTime;
    use paths::session_directory;

    let p = session_directory().and_then(|p| {
        fs::read_dir(p).and_then(|dir| {
            Ok(dir.filter_map(|f| {
                match f {
                    Ok(f) => {
                        let ts = f.metadata().ok().and_then(|t| {
                            t.modified().ok().and_then(|t| {
                                t.elapsed().ok().and_then(|x| {
                                    chrono::Duration::from_std(x).ok().and_then(|x| {
                                        chrono::Duration::zero().checked_sub(&x)
                                    })
                                })
                            })
                        });

                        match ts {
                            Some(ts) => Some(SessionInfo{
                                title: f.file_name().to_str().unwrap_or("(error)").to_string(),
                                age: format!("{}",HumanTime::from(ts)),
                                file: f.file_name().to_str().unwrap_or("(error)").to_string(),
                                path: f.path().to_str().unwrap_or("(error)").to_string(),
                            }),
                            _ => None,
                        }
                    },
                    Err(_) => None,
                }
            }).collect::<Vec<_>>())
        }).map_err(|e| e.into())
    }).ok();

    match p {
        Some(p) => Variant::String(return_json::<Vec<SessionInfo>>(Ok(p))),
        None => Variant::String(return_json::<Vec<SessionInfo>>(Ok(vec![]))),
    }
}

pub fn delete_session(arg0: &Variant) -> Variant {
    use paths::session_directory;

    let name = if let &Variant::String(ref x) = arg0 {
        x.clone()
    } else {
        return Variant::String(return_json::<()>(Err("1st argument is not a string".into())));
    };

    let p: Result<()> = session_directory().and_then(|mut p| {
        p.push(name);
        fs::remove_file(p).map_err(|e| e.into())
    });

    Variant::String(return_json::<()>(p))
}

pub fn find_data_file(arg0: &Variant) -> Variant {
    use paths;
    use std::path::Path;

    let path = if let &Variant::String(ref x) = arg0 {
        x.clone()
    } else {
        return Variant::String(return_json::<()>(Err("1st argument is not a string".into())));
    };

    let res = paths::find_data_file(&Path::new(&path));

    Variant::String(return_json(res.map(|x| x.map(|x| x.to_string_lossy().into_owned()))))
}
