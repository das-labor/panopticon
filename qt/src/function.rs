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

use std::hash::{Hash,Hasher,SipHasher};
use std::thread;
use qmlrs::{Object,Variant};
use graph_algos::traits::{
    VertexListGraph,
    Graph,
    IncidenceGraph,
    EdgeListGraph
};
use uuid::Uuid;
use controller::PROJECT;
use rustc_serialize::json;
use std::collections::HashMap;
use controller::{
    LAYOUTED_FUNCTION,
    ROUTED_FUNCTION,
    CHANGED_FUNCTION
};

use sugiyama;
use route;

/*
 * emit DISCOVERED_FUNCTION -> emit STARTED_FUNCTION -> emit FINISHED_FUNCTION -> layout(UUID) ->
 * emit LAYOUTED_FUNCTION -> emit ROUTED_FUNCTION
 */

/// JSON describing the function with UUID `arg`.
///
/// The JSON looks like this:
/// ```json
/// {
///     "type": "function", // or "symbol" or "todo"
///     "name": "func_001", // not present if type is "todo"
///     "uuid": arg,
///     "start": 0x1002,    // optional: entry point
///     "calls": {          // outgoing calls
///         "0": <UUID>,
///         "1": <UUID>,
///         ...
///     }
/// }
/// ```
pub fn metainfo(arg: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref uuid_str) = arg {
        if let Some(tgt_uuid) = Uuid::parse_str(uuid_str).ok() {
            let read_guard = PROJECT.read().unwrap();
            let proj: &Project = read_guard.as_ref().unwrap();

            if let Some((vx,prog)) = proj.find_call_target_by_uuid(&tgt_uuid) {
                // collect called functions' UUIDs
                let callees = prog.call_graph.out_edges(vx).
                    map(|x| prog.call_graph.target(x)).
                    filter_map(|x| prog.call_graph.vertex_label(x)).
                    enumerate().
                    map(|(i,x)| format!("\"{}\":\"{}\"",i,x.uuid())).
                    fold("".to_string(),|acc,x| if acc != "" { acc + "," + &x } else { x });

                // match function
                match prog.call_graph.vertex_label(vx) {
                    Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: Some(ref ent), cflow_graph: ref cg,..})) => {

                        // match entry point
                        match cg.vertex_label(*ent) {
                            Some(&ControlFlowTarget::Resolved(ref bb)) =>
                                format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"start\":{},\"calls\":{{{}}}}}",name,uuid,bb.area.start,callees),
                            Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))) =>
                                format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"start\":{},\"calls\":{{{}}}}}",name,uuid,c,callees),
                            Some(&ControlFlowTarget::Unresolved(_)) =>
                                format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"calls\":{{{}}}}}",name,uuid,callees),
                            None => unreachable!()
                        }
                    },
                    Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: None,..})) => {
                        format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"calls\":{{{}}}}}",name,uuid,callees)
                    },
                    Some(&CallTarget::Symbolic(ref sym,ref uuid)) => {
                        format!("{{\"type\":\"symbol\",\"name\":\"{}\",\"uuid\":\"{}\",\"calls\":{{{}}}}}",sym,uuid,callees)
                    },
                    Some(&CallTarget::Todo(ref a,ref uuid)) => {
                        format!("{{\"type\":\"todo\",\"start\":{},\"uuid\":\"{}\",\"calls\":{{{}}}}}",a,uuid,callees)
                    },
                    None => {
                        "".to_string()
                    }
                }
            } else {
                // unknown uuid
                "".to_string()
            }
        } else {
        // arg not a valid uuid
        "".to_string()
        }
    } else {
        // arg not a string
        "".to_string()
    })
}

/// JSON-encoded control flow graph of the function w/ UUID `arg`.
///
/// The JSON will look like this:
/// ```json
/// {
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
            let read_guard = PROJECT.read().unwrap();
            let proj: &Project = read_guard.as_ref().unwrap();

            if let Some((vx,prog)) = proj.find_call_target_by_uuid(&tgt_uuid) {
                if let Some(&CallTarget::Concrete(ref fun)) = prog.call_graph.vertex_label(vx) {
                    let cfg = &fun.cflow_graph;

                    // nodes
                    let nodes = cfg.vertices()
                        .filter_map(|x| {
                            cfg.vertex_label(x).map(|x| "\"".to_string() + &to_ident(x) + "\"")
                        })
                        .fold("".to_string(),|acc,x| {
                            if acc != "" { acc + "," + &x } else { x }
                        });

                    // basic block contents
                    // XXX: skips unresolved control transfers
                    let contents = cfg.vertices()
                        .filter_map(|x| {
                            match cfg.vertex_label(x) {
                                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                                    let mnes = bb.mnemonics.iter().
                                        map(|x| {
                                            let args = x.format();
                                            let cmnt = proj.comments.get(&(fun.region.clone(),x.area.start)).unwrap_or(&"".to_string()).clone();

                                            format!("{{
                                                \"opcode\":\"{}\",
                                                \"args\":\"{}\",
                                                \"region\":\"{}\",
                                                \"offset\":{},
                                                \"comment\":\"{}\"
                                            }}",x.opcode,args,fun.region,x.area.start,cmnt)
                                        }).
                                        fold("".to_string(),|acc,x| if acc != "" { acc + "," + &x } else { x });
                                    Some(format!("\"bb{}\":[{}]",bb.area.start,mnes))
                                },
                                _ => None,
                            }
                        })
                        .fold("".to_string(),|acc,x| {
                            if acc != "" { acc + "," + &x } else { x }
                        });

                    // control flow edges
                    let edges = cfg.edges()
                        .filter_map(|x| {
                            let from = cfg.source(x);
                            let to = cfg.target(x);
                            let from_ident = cfg.vertex_label(from).map(to_ident);
                            let to_ident = cfg.vertex_label(to).map(to_ident);

                            if let (Some(f),Some(t)) = (from_ident,to_ident) {
                                Some(format!("{{\"from\":\"{}\",\"to\":\"{}\"}}",f,t))
                            } else {
                                None
                            }
                        })
                        .fold("".to_string(),|acc,x| {
                            if acc != "" { acc + "," + &x } else { x }
                        });

                    format!("{{\"nodes\":[{}],\"edges\":[{}],\"contents\":{{{}}}}}",nodes,edges,contents)
                } else {
                    // not a concrete call
                    "".to_string()
                }
            } else {
                // call not found
                "".to_string()
            }
        } else {
            // invalid uuid
            "".to_string()
        }
    } else {
        // arg is not a string
        "".to_string()
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
pub fn layout(arg0: &Variant, arg1: &Variant, arg2: &Variant, arg3: &Variant, _ctrl: &mut Object) -> Variant {
    let dims = if let &Variant::String(ref st) = arg1 {
        match json::decode::<HashMap<String,LayoutInputDimension>>(st) {
            Ok(input) => {
                input
            },
            Err(err) => {
                println!("can't parse layout request: {}",err);
                return Variant::String("{}".to_string());
            }
        }
    } else {
        return Variant::String("{}".to_string());
    };

    let rank_spacing = if let &Variant::I64(ref x) = arg2 {
        *x
    } else {
        return Variant::String("{}".to_string());
    };

    let node_spacing = if let &Variant::I64(ref x) = arg3 {
        *x
    } else {
        return Variant::String("{}".to_string());
    };

    if let &Variant::String(ref st) = arg0 {
        if let Some(uuid) = Uuid::parse_str(st).ok() {
            let read_guard = PROJECT.read().unwrap();
            let proj: &Project = read_guard.as_ref().unwrap();

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
                let ctrl = Object::from_ptr(_ctrl.as_ptr());

                thread::spawn(move || {
                    let res = sugiyama::layout(&(0..vertices.len()).collect::<Vec<usize>>(),
                                               &edges,
                                               &dims_transformed,
                                               maybe_entry,
                                               node_spacing as usize,
                                               rank_spacing as usize);
                    let mut ret_v = HashMap::<String,LayoutOutputPosition>::new();
                    let mut ret_e = Vec::<LayoutOutputEdge>::new();
                    for (k,v) in (res.0).iter() {
                        ret_v.insert(idents[*k].clone(),LayoutOutputPosition{ x: v.0 as f32, y: v.1 as f32 });
                    }
                    for v in (res.1).iter() {
                        ret_e.push(LayoutOutputEdge{
                            x1: v.0 as f32,
                            y1: v.1 as f32,
                            x2: v.2 as f32,
                            y2: v.3 as f32,
                        });
                    }
                    ctrl.emit(LAYOUTED_FUNCTION,&vec![Variant::String(json::encode(&(ret_v,ret_e)).ok().unwrap())]);
                });
            }
        }
    }

    Variant::String("{}".to_string())
}

#[derive(RustcDecodable,Debug)]
struct RouteInputObstacle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(RustcEncodable,RustcDecodable,Debug,Clone)]
struct RouteWaypoint {
    x: f32,
    y: f32,
}

#[derive(RustcDecodable,Debug)]
struct RouteInputEdge {
    from: usize,
    to: usize,
}


#[derive(RustcDecodable,Debug)]
struct RouteInput {
    obstacles: Vec<RouteInputObstacle>,
    waypoints: Vec<RouteWaypoint>,
    edges: Vec<RouteInputEdge>,
}


/// Route a control flow graph.
///
/// Input has to look loke this:
/// ```json
/// {
///     "obstacles": [
///         {
///             "x": <NUM>,
///             "y": <NUM>,
///             "height": <NUM>,
///             "width": <NUM,
///         },
///         ...
///     ],
///     waypoints: [
///         { "x": <NUM>, "y": <NUM> },
///         ...
///     ],
///     edges: [
///         { "from": <NUM>, "to": <NUM> }, // refs waypoints
///         ...
///     ]
/// }```
///
/// Output:
/// ```json
/// [
///     [[{"x": <NUM>, "y": <NUM>},...]],
///     ...
/// ]
/// }```
pub fn route(arg0: &Variant, _ctrl: &mut Object) -> Variant {
    let input = if let &Variant::String(ref st) = arg0 {
        match json::decode::<RouteInput>(st) {
            Ok(input) => {
                input
            },
            Err(err) => {
                println!("can't parse routing request: {}",err);
                return Variant::String("{}".to_string());
            }
        }
    } else {
        return Variant::String("{}".to_string());
    };

    let ctrl = Object::from_ptr(_ctrl.as_ptr());

    thread::spawn(move || {
        let points = input.waypoints.iter().map(|p| route::Point{ x: p.x, y: p.y }).collect::<Vec<_>>();
        let segments = input.obstacles.iter().flat_map(|o| {
            let tl = route::Point{ x: o.x, y: o.y };
            let tr = route::Point{ x: o.x + o.width, y: o.y };
            let bl = route::Point{ x: o.x, y: o.y + o.height };
            let br = route::Point{ x: o.x + o.width, y: o.y + o.height };

            vec![
                route::Segment{ start: tl.clone(), end: tr.clone() },
                route::Segment{ start: tr, end: br.clone() },
                route::Segment{ start: br, end: bl.clone() },
                route::Segment{ start: bl, end: tl }
            ].into_iter()
        }).collect::<Vec<_>>();
        let graph = route::visibility_graph(&segments,&points);

        let ret = input.edges.iter().map(|e| {
            route::dijkstra(e.from,e.to,&graph,&points).iter().map(|&w| input.waypoints[w].clone()).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        ctrl.emit(ROUTED_FUNCTION,&vec![Variant::String(json::encode(&ret).ok().unwrap())]);
    });

    Variant::String("{}".to_string())
}

pub fn comment(arg0: &Variant, arg1: &Variant, arg2: &Variant, ctrl: &mut Object) -> Variant {
    let reg = if let &Variant::String(ref x) = arg0 {
        x.clone()
    } else {
        return Variant::String("{}".to_string());
    };

    let offset = if let &Variant::I64(ref x) = arg1 {
        *x as u64
    } else {
        return Variant::String("{}".to_string());
    };

    let cmnt = if let &Variant::String(ref x) = arg2 {
        x.clone()
    } else {
        return Variant::String("{}".to_string());
    };

    // write comment
    {
        let mut write_guard = PROJECT.write().unwrap();
        let proj: &mut Project = write_guard.as_mut().unwrap();
        proj.comments.insert((reg.clone(),offset),cmnt);
    }

    {
        let read_guard = PROJECT.read().unwrap();
        let proj: &Project = read_guard.as_ref().unwrap();

        for prog in proj.code.iter() {
            for ct in prog.call_graph.vertices() {
                match prog.call_graph.vertex_label(ct) {
                    Some(&CallTarget::Concrete(ref func)) => {
                        if func.region == reg {
                            // XXX: check offset?
                            ctrl.emit(CHANGED_FUNCTION,&vec![Variant::String(func.uuid.to_string())]);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    Variant::String("".to_string())
}

pub fn rename(arg0: &Variant, arg1: &Variant, ctrl: &mut Object) -> Variant {
    let name = if let &Variant::String(ref x) = arg1 {
        x.clone()
    } else {
        return Variant::String("{}".to_string());
    };

    let maybe_uu = if let &Variant::String(ref st) = arg0 {
        if let Some(uuid) = Uuid::parse_str(st).ok() {
            let mut write_guard = PROJECT.write().unwrap();
            let proj: &mut Project = write_guard.as_mut().unwrap();

            if let Some(func) = proj.find_function_by_uuid_mut(&uuid) {
                func.name = name;
                Some(uuid.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(uu) = maybe_uu {
        ctrl.emit(CHANGED_FUNCTION,&vec![Variant::String(uu.to_string())]);
    }

    Variant::String("".to_string())
}
