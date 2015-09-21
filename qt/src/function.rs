use panopticon::value::Rvalue;
use panopticon::project::Project;
use panopticon::function::{Function,ControlFlowTarget};
use panopticon::program::CallTarget;

use std::hash::{Hash,Hasher,SipHasher};
use qmlrs::Variant;
use graph_algos::traits::{VertexListGraph,Graph,IncidenceGraph,EdgeListGraph};
use uuid::Uuid;
use controller::PROJECT;

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
///             "args": ["r1", "r2"],
///         },{
///             "opcode": "add",
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
                            cfg.vertex_label(x).map(to_ident)
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
                                            let args = x.operands.iter().map(|y| format!("\"{}\"",y))
                                                .fold("".to_string(),|acc,x| if acc != "" { acc + "," + &x } else { x });

                                            format!("{{\"opcode\":\"{}\",\"args\":[{}]}}",x.opcode,args)
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
                                Some(format!("{{\"from\":{},\"to\":{}}}",f,t))
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
            format!("\"bb{}\"",bb.area.start),
        &ControlFlowTarget::Unresolved(ref c) => {
            let ref mut h = SipHasher::new();
            c.hash::<SipHasher>(h);
            format!("\"c{}\"",h.finish())
        }
    }
}
