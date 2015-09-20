use panopticon::value::Rvalue;
use panopticon::project::Project;
use panopticon::function::{Function,ControlFlowTarget};
use panopticon::region::Region;
use panopticon::program::{Program,CallTarget};
use panopticon::avr;

use std::path::Path;
use std::thread;
use uuid::Uuid;
use qmlrs::{ffi,MetaObject,Variant,Object,ToQVariant,unpack_varlist};
use graph_algos::traits::{VertexListGraph,Graph,MutableGraph,IncidenceGraph,EdgeListGraph};
use controller::{
    DISCOVERED_FUNCTION,
    STARTED_FUNCTION,
    FINISHED_FUNCTION,
    CREATE_AVR_SESSION,
    CREATE_RAW_SESSION,
    OPEN_SESSION,
    DONE,
    START,
    PROJECT
};

/// Returns the current controller state.
pub fn state<'a>(ctrl: &Object) -> String {
    if let Variant::String(ref ret) = ctrl.get_property("state") {
        ret.to_string()
    } else {
        unreachable!()
    }
}

/// Sets the controller state to `st`.
///
/// # panics
/// If the transition is not allowed by the state machine.
fn set_state(st: &str, ctrl: &mut Object) {
    let ok = {
        let _cur = state(ctrl);
        let cur = _cur.as_ref();

        (cur,st) == ("NEW","READY") ||
        (cur,st) == ("READY","WORKING") ||
        (cur,st) == ("WORKING","DONE")
    };

    if ok {
        ctrl.set_property("state",Variant::String(st.to_string()));
        ctrl.emit(0,&[]);
    } else {
        panic!("Invalid controller state transition '{}' -> '{}'",state(ctrl),st);
    }
}

/// Prepares to disassemble an AVR dump.
///
/// Returns true on success, false otherwise
pub fn create_avr_session(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            let p = Project::new("".to_string(),Region::open("".to_string(),&Path::new(s)).unwrap());

            *PROJECT.write().unwrap() = Some(p);
            set_state("READY",ctrl);

            true
        } else {
            false
        }
    } else {
        false
    })
}

pub fn create_raw_session(_path: &Variant, ctrl: &mut Object) -> Variant {
    unimplemented!();
}

/// Prepares to open a saved Panopticon session.
///
/// Returns true on success, false otherwise
pub fn open_session(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            if let Some(p) = Project::open(&Path::new(s)) {
                *PROJECT.write().unwrap() = Some(p);
                set_state("READY",ctrl);

                true
            } else {
                // open failed
                false
            }
        } else {
            // _path isn't a string
            false
        }
    } else {
       // wrong controller state
       false
    })
}

/// Starts disassembly
pub fn start(_ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(_ctrl) == "READY" {
        let guard = PROJECT.read().unwrap();
        let maybe_project = guard;

        if !maybe_project.is_some() {
            false
        } else {
            set_state("WORKING",_ctrl);

            let mut ctrl = Object::from_ptr(_ctrl.as_ptr());
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

                ctrl.emit(DISCOVERED_FUNCTION,&vec!(Variant::String(uu.to_string())));

                loop {
                    let maybe_tgt = {
                        let read_guard = PROJECT.read().unwrap();
                        let proj: &Project = read_guard.as_ref().unwrap();
                        let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                        prog.call_graph.vertices().filter_map(|x| {
                            if let Some(&CallTarget::Todo(tgt,uuid)) = prog.call_graph.vertex_label(x) {
                                Some((tgt,uuid))
                            } else {
                                None
                            }
                        }).next()
                    };

                    if let Some((tgt,uuid)) = maybe_tgt {
                        ctrl.emit(STARTED_FUNCTION,&vec!(Variant::String(uuid.to_string())));

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

                            ctrl.emit(FINISHED_FUNCTION,&vec!(Variant::String(fun_uuid.to_string())));

                            for a in new_tgt {
                                ctrl.emit(DISCOVERED_FUNCTION,&vec!(Variant::String(a.to_string())));
                            }
                        }
                    } else {
                        break;
                    }
                }

                ctrl.call(DONE,&[]);
            });

            true
        }
    } else {
        // wrong controller state
        false
    })
}

/// Change the controller state to DONE
pub fn done(ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(ctrl) == "WORKING" {
        set_state("DONE",ctrl);
        true
    } else {
        false
    })
}
