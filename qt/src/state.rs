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

use panopticon::project::Project;
use panopticon::function::Function;
use panopticon::region::Region;
use panopticon::program::{Program,CallTarget};
use panopticon::avr;

use std::path::Path;
use std::thread;
use uuid::Uuid;
use qmlrs::{Variant,Object};
use graph_algos::traits::{VertexListGraph,Graph,MutableGraph};
use controller::{
    STATE_CHANGED,
    DIRTY_CHANGED,
    DISCOVERED_FUNCTION,
    STARTED_FUNCTION,
    FINISHED_FUNCTION,
    DONE,
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
        ctrl.emit(STATE_CHANGED,&[]);
    } else {
        panic!("Invalid controller state transition '{}' -> '{}'",state(ctrl),st);
    }
}

/// Sets the controller dirty bit to `d`.
pub fn set_dirty(d: bool, ctrl: &mut Object) {
    ctrl.set_property("dirty",Variant::I64(if d { 1 } else { 0 }));
    ctrl.emit(DIRTY_CHANGED,&[]);
}

/// Prepares to disassemble an AVR dump.
///
/// Returns true on success, false otherwise
pub fn create_avr_session(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            let p = Project::new("AVR".to_string(),Region::open("flash".to_string(),&Path::new(s)).unwrap());

            *PROJECT.write().unwrap() = Some(p);
            set_state("READY",ctrl);
            set_dirty(true,ctrl);

            true
        } else {
            false
        }
    } else {
        false
    })
}

pub fn create_raw_session(_: &Variant, _: &mut Object) -> Variant {
    unimplemented!();
}

/// Prepares to open a saved Panopticon session.
///
/// Returns true on success, false otherwise
pub fn open_session(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            match Project::open(&Path::new(s)) {
                Ok(p) => {
                    *PROJECT.write().unwrap() = Some(p);
                    set_state("READY",ctrl);
                    set_dirty(true,ctrl);
                    true
                },
                Err(s) => {
                    println!("open: {}",s);
                    false
                }
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

pub fn snapshot_session(_path: &Variant, ctrl: &mut Object) -> Variant {
    let ret = if let &Variant::String(ref s) = _path {
        let maybe_project: &Option<Project> = &*PROJECT.read().unwrap();

        if let &Some(ref p) = maybe_project {
            match p.snapshot(&Path::new(s)) {
                Ok(_) => {
                    true
                },
                Err(s) => {
                    println!("snapshot: {}",s);
                    false
                }
            }
        } else {
            false
        }
    } else {
        // _path isn't a string
        false
    };

    set_dirty(false,ctrl);
    Variant::Bool(ret)
}

pub fn start(ctrl: &mut Object) -> Variant {
    Variant::Bool(if state(ctrl) == "READY" {
        let next: Option<Box<Fn(&mut Object) -> bool>> = {
            let guard = PROJECT.read().unwrap();

            if let &Some(ref proj) = &*guard {
                if proj.code.len() == 0 {
                    Some(Box::new(start_new))
                } else {
                    Some(Box::new(start_resume))
                }
            } else {
                None
            }
        };

        match next {
            Some(f) => f(ctrl),
            None => false,
        }
    } else {
        unreachable!("Wrong UI state for start()");
    })
}

/// Starts disassembly
pub fn start_new(_ctrl: &mut Object) -> bool {
    set_state("WORKING",_ctrl);

    let mut ctrl = Object::from_ptr(_ctrl.as_ptr());
    thread::spawn(move || {
        let mut prog = Program::new("prog0");
        let prog_uuid = prog.uuid;
        let start = 0;
        let dec = avr::syntax::disassembler();
        let init = avr::Mcu::new();
        let uu = Uuid::new_v4();

        // Add empty program
        {
            let mut write_guard = PROJECT.write().unwrap();
            let proj: &mut Project = write_guard.as_mut().unwrap();
            let root = proj.sources.dependencies.vertex_label(proj.sources.root).unwrap();

            prog.call_graph.add_vertex(CallTarget::Todo(start,uu.clone()));
            proj.code.push(prog);
            proj.comments.insert((root.name().clone(),0),"MCU entry point".to_string());
        }

        ctrl.emit(DISCOVERED_FUNCTION,&vec!(Variant::String(uu.to_string())));
        set_dirty(true,&mut ctrl);

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
                set_dirty(true,&mut ctrl);

                let new_fun = {
                    let read_guard = PROJECT.read().unwrap();
                    let pro: &Project = read_guard.as_ref().unwrap();
                    let root = pro.sources.dependencies.vertex_label(pro.sources.root).unwrap();
                    let i = root.iter();
                    let mut fun = Function::with_uuid(format!("func_{}",tgt),uuid,root.name().clone());

                    fun = Function::disassemble::<avr::Avr>(Some(fun),dec.clone(),init.clone(),i,tgt,root.name().clone());
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
                    set_dirty(true,&mut ctrl);

                    for a in new_tgt {
                        ctrl.emit(DISCOVERED_FUNCTION,&vec!(Variant::String(a.to_string())));
                        set_dirty(true,&mut ctrl);
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

pub fn start_resume(_ctrl: &mut Object) -> bool {
    set_state("WORKING",_ctrl);

    let ctrl = Object::from_ptr(_ctrl.as_ptr());
    thread::spawn(move || {
        let uuids = {
            let read_guard = PROJECT.read().unwrap();
            let proj: &Project = read_guard.as_ref().unwrap();

            proj.code.iter().flat_map(|p| p.call_graph.vertices().filter_map(move |vx| {
                p.call_graph.vertex_label(vx).map(|x| x.uuid()) })).collect::<Vec<_>>()
        };

        for uu in uuids {
            ctrl.emit(FINISHED_FUNCTION,&vec!(Variant::String(uu.to_string())));
        }

        ctrl.call(DONE,&[]);
    });

    true
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
