extern crate panopticon;

extern crate qmlrs;
extern crate libc;
extern crate graph_algos;
extern crate uuid;

#[macro_use]
extern crate lazy_static;

use libc::c_int;
use std::sync::RwLock;
use std::path::Path;
use std::thread;

use panopticon::value::Rvalue;
use panopticon::project::Project;
use panopticon::function::{Function,ControlFlowTarget};
use panopticon::region::Region;
use panopticon::program::{Program,CallTarget};
use panopticon::avr;

use graph_algos::traits::{VertexListGraph,Graph,MutableGraph,IncidenceGraph};
use qmlrs::{ffi,MetaObject,Variant,Object,ToQVariant};
use uuid::Uuid;

/*
 * --> Virgin --[ create/open Session ]--> Ready --[ start() ]--> Working --> Done -.
 *        ^-----------------------[ close() ]---------------------------------------'
 * Ready: emit discoverFunction, startFunction, doneFunction
 */

lazy_static! {
    static ref PROJECT: RwLock<Option<Project>> = RwLock::new(None);
}

extern "C" fn panopticon_slot(this: *mut ffi::QObject, id: libc::c_int, a: *const ffi::QVariantList, ret: *mut ffi::QVariant) {
    let mut obj = Object::from_ptr(this);
    let args = qmlrs::unpack_varlist(a);
    let state = obj.get_property("state");

    let new = Variant::String("NEW".to_string());
    let ready = Variant::String("READY".to_string());
    let working = Variant::String("WORKING".to_string());

    match (id,state,args.len()) {
        (5...7,_new,1) => {
            if _new != new {
                panic!("Called {} in wrong state!",id);
            }

            if let Variant::String(ref s) = args[0] {
                let p = if id == 6 {
                    Project::open(&Path::new(s))
                } else {
                    Some(Project::new("".to_string(),Region::open("".to_string(),&Path::new(s)).unwrap()))
                };

                if p.is_some() {
                    *PROJECT.write().unwrap() = p;
                    obj.set_property("state",Variant::String("READY".to_string()));
                    obj.emit(0,&[]);
                    true.to_qvariant(ret);
                } else {
                    false.to_qvariant(ret);
                }
            } else {
                false.to_qvariant(ret);
            }
        },
        (8,_ready,0) => {
            if _ready != ready {
                panic!("Called {} in wrong state!",id);
            }

            let guard = PROJECT.read().unwrap();
            let maybe_project = guard;

            if !maybe_project.is_some() {
                false.to_qvariant(ret);
            } else {

            obj.set_property("state",Variant::String("WORKING".to_string()));
            obj.emit(0,&[]);

            thread::spawn(move || {
                let mut prog = Program::new("prog0");
                let prog_uuid = prog.uuid;
                let start = 0;
                let dec = avr::disassembler();
                let init = avr::Mcu::new();
                let uu = Uuid::new_v4();

                // Add empty program
                {
                    let mut write_guard = PROJECT.write().unwrap();
                    let proj: &mut Project = write_guard.as_mut().unwrap();

                    prog.call_graph.add_vertex(CallTarget::Todo(start,uu.clone()));
                    proj.code.push(prog);
                }

                obj.emit(1,&vec!(Variant::String(uu.to_string())));

                loop {
                    let maybe_tgt = {
                        let read_guard = PROJECT.read().unwrap();
                        let proj: &Project = read_guard.as_ref().unwrap();
                        let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                        println!("num: {}",prog.call_graph.num_vertices());

                        prog.call_graph.vertices().filter_map(|x| {
                            if let Some(&CallTarget::Todo(tgt,uuid)) = prog.call_graph.vertex_label(x) {
                                Some((tgt,uuid))
                            } else {
                                None
                            }
                        }).next()
                    };

                    if let Some((tgt,uuid)) = maybe_tgt {
                        obj.emit(2,&vec!(Variant::String(uuid.to_string())));

                        println!("Disassemble at {}",tgt);

                        let new_fun = {
                            let read_guard = PROJECT.read().unwrap();
                            let pro: &Project = read_guard.as_ref().unwrap();
                            let i = pro.sources.dependencies.vertex_label(pro.sources.root).unwrap().iter();
                            let mut fun = Function::with_uuid(format!("func_{}",tgt),uuid);

                            fun = Function::disassemble::<avr::Avr>(Some(fun),dec.clone(),init.clone(),i,tgt);
                            fun.entry_point = fun.find_basic_block_at_address(tgt);
                            fun
                        };

                        if new_fun.cflow_graph.num_vertices() > 0 {
                            let fun_uuid = new_fun.uuid.clone();
                            let new_tgt = {
                                let mut write_guard = PROJECT.write().unwrap();
                                let proj: &mut Project = write_guard.as_mut().unwrap();
                                let prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();

                                prog.insert(new_fun)
                            };

                            obj.emit(3,&vec!(Variant::String(fun_uuid.to_string())));

                            for a in new_tgt {
                                obj.emit(1,&vec!(Variant::String(a.to_string())));
                            }
                        }
                    } else {
                        break;
                    }
                }

                obj.call(9,&[]);
            });
            true.to_qvariant(ret);
            }
        },
        (9,_working,0) => {
            if _working != working {
                panic!("Called {} in wrong state!",id);
            }

            obj.set_property("state",Variant::String("DONE".to_string()));
            obj.emit(0,&[]);
        },
        (10,_,1) => {
            "".to_qvariant(ret);

            if let Variant::String(ref uuid_str) = args[0] {
                if let Some(tgt_uuid) = Uuid::parse_str(uuid_str).ok() {
                    let read_guard = PROJECT.read().unwrap();
                    let proj: &Project = read_guard.as_ref().unwrap();
                    if let Some((vx,prog)) = proj.find_call_target_by_uuid(&tgt_uuid) {
                        let callees = prog.call_graph.out_edges(vx).
                            map(|x| prog.call_graph.target(x)).
                            filter_map(|x| prog.call_graph.vertex_label(x)).
                            map(|x| format!("\"{}\"",x.uuid())).
                            fold("".to_string(),|acc,x| if acc != "" { acc + "," + &x } else { x });

                        match prog.call_graph.vertex_label(vx) {
                            Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: Some(ref ent), cflow_graph: ref cg,..})) => {
                                match cg.vertex_label(*ent) {
                                    Some(&ControlFlowTarget::Resolved(ref bb)) =>
                                        format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"start\":{},\"calls\":[{}]}}",name,uuid,bb.area.start,callees),
                                    Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))) =>
                                        format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"start\":{},\"calls\":[{}]}}",name,uuid,c,callees),
                                    Some(&ControlFlowTarget::Unresolved(_)) =>
                                        format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"calls\":[{}]}}",name,uuid,callees),
                                    None => unreachable!()
                                }
                            },
                            Some(&CallTarget::Concrete(Function{ ref uuid, ref name, entry_point: None,..})) => {
                                format!("{{\"type\":\"function\",\"name\":\"{}\",\"uuid\":\"{}\",\"calls\":[{}]}}",name,uuid,callees)
                            },
                            Some(&CallTarget::Symbolic(ref sym,ref uuid)) => {
                                format!("{{\"type\":\"symbol\",\"name\":\"{}\",\"uuid\":\"{}\",\"calls\":[{}]}}",sym,uuid,callees)
                            },
                            Some(&CallTarget::Todo(ref a,ref uuid)) => {
                                format!("{{\"type\":\"todo\",\"start\":\"{}\",\"uuid\":\"{}\",\"calls\":[{}]}}",a,uuid,callees)
                            },
                            None => {
                                "".to_string()
                            }
                        }.to_qvariant(ret);
                    } else {
                        // unknown uuid
                    }
                }
            }
        }
        _ => panic!("unknown Panopticon type call error (id: {}, args: {})",id,args.len())
    }
}

extern "C" fn create_panopticon_singleton(_: *mut qmlrs::ffi::QQmlEngine, _: *mut qmlrs::ffi::QJSEngine) -> *mut qmlrs::ffi::QObject {
    let mut metaobj = MetaObject::new("Panopticon",panopticon_slot);

    assert_eq!(metaobj.add_signal("stateChanged()"),0);
    metaobj.add_property("state","QString",Some("stateChanged()"));

    // WORKING signals
    assert_eq!(metaobj.add_signal("discoveredFunction(QString)"),1);
    assert_eq!(metaobj.add_signal("startedFunction(QString)"),2);
    assert_eq!(metaobj.add_signal("finishedFunction(QString)"),3);

    // WORKING and DONE signals
    assert_eq!(metaobj.add_signal("changedFunction(QString)"),4);

    // state = NEW -> READY
    assert_eq!(metaobj.add_method("createAvrSession(QString)","bool"),5);
    assert_eq!(metaobj.add_method("createRawSession(QString)","bool"),6);
    assert_eq!(metaobj.add_method("openSession(QString)","bool"),7);

    // state = READY -> WORKING
    assert_eq!(metaobj.add_method("start()","bool"),8);

    // state = WORKING -> DONE
    assert_eq!(metaobj.add_method("done()","void"),9);

    // getter
    assert_eq!(metaobj.add_method("functionInfo(QString)","QString"),10);

    let mut obj = metaobj.instantiate();

    obj.set_property("state",Variant::String("NEW".to_string()));
    obj.emit(0,&[]);
    obj.as_ptr()
}

pub fn main() {
    qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_panopticon_singleton);

    let mut engine = qmlrs::Engine::new();
    engine.load_local_file("qt/res/Window.qml");
    engine.exec();
}
