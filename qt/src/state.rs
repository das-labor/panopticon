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
use panopticon::program::{Program,CallTarget};
use panopticon::elf;
use panopticon::mos;
use panopticon::target::Target;
use panopticon::value::Rvalue;

use std::path::Path;
use std::thread;
use qmlrs::{Variant,Object};
use graph_algos::{
    VertexListGraphTrait,
    GraphTrait
};
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
///
/// # panics
/// If the state is not a string.
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

        (cur,st) == ("NEW","SYNC") ||
        (cur,st) == ("NEW","DIRTY") ||
        (cur,st) == ("DIRTY","SYNC") ||
        (cur,st) == ("SYNC","DIRTY") ||
        (cur,st) == ("SYNC","NEW")
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

/// Prepares to disassemble a memory image.
pub fn create_raw_project(_path: &Variant, _tgt: &Variant, ctrl: &mut Object) -> Variant {
    Variant::String(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            if let &Variant::String(ref tgt_s) = _tgt {
                if let Some(tgt) = Target::for_name(tgt_s) {
                    match Project::raw(tgt,&Path::new(s)) {
                        Some(proj) => {
                            *PROJECT.write().unwrap() = Some(proj);
                            set_state("READY",ctrl);
                            set_dirty(true,ctrl);

                            "{\"status\": \"ok\"}".to_string()
                        },
                        None =>
                            "{{\"status\": \"err\", \"error\": \"Can't open project: Unknown error\"}}".to_string()
                    }
                } else {
                    format!("{{\"status\": \"err\", \"error\": \"No target named '{}'\"}}",tgt_s)
                }
            } else {
                "{\"status\": \"err\", \"error\": \"2nd argument is not a string\"}".to_string()
            }
        } else {
            "{\"status\": \"err\", \"error\": \"1st argument is not a string\"}".to_string()
        }
    } else {
        "{\"status\": \"err\", \"error\": \"Wrong state\"}".to_string()
    })
}

/// Prepares to disassemble an ELF file.
pub fn create_elf_project(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::String(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            match elf::load::load(Path::new(s)) {
                Ok(proj) => {
                    *PROJECT.write().unwrap() = Some(proj);
                    set_state("READY",ctrl);
                    set_dirty(true,ctrl);
                    "{\"status\": \"ok\"}".to_string()
                },
                Err(_) =>
                    "{\"status\": \"err\", \"error\": \"Failed to read ELF file\"}".to_string()
            }
        } else {
            "{\"status\": \"err\", \"error\": \"1st argument is not a string\"}".to_string()
        }
    } else {
        "{\"status\": \"err\", \"error\": \"Wrong state\"}".to_string()
    })
}

pub fn create_mos6502_project(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::String(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            match mos::load::load(Path::new(s)) {
                Ok(proj) => {
                    *PROJECT.write().unwrap() = Some(proj);
                    set_state("READY",ctrl);
                    set_dirty(true,ctrl);
                    "{\"status\": \"ok\"}".to_string()
                },
                Err(e) =>
                    format!("{{\"status\": \"err\", \"error\": \"Failed to read ELF file: {:?}\"}}",e)
            }
        } else {
            "{\"status\": \"err\", \"error\": \"1st argument is not a string\"}".to_string()
        }
    } else {
        "{\"status\": \"err\", \"error\": \"Wrong state\"}".to_string()
    })
}

/// Prepares to open a saved Panopticon project.
pub fn open_project(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::String(if state(ctrl) == "NEW" {
        if let &Variant::String(ref s) = _path {
            match Project::open(&Path::new(s)) {
                Ok(p) => {
                    *PROJECT.write().unwrap() = Some(p);
                    set_state("READY_RESUME",ctrl);
                    set_dirty(true,ctrl);
                    "{\"status\": \"ok\"}".to_string()
                },
                Err(s) =>
                    format!("{{\"status\": \"err\", \"error\": \"Failed to open file: {:?}\"}}",s)
            }
        } else {
            "{\"status\": \"err\", \"error\": \"1st argument is not a string\"}".to_string()
        }
    } else {
        "{\"status\": \"err\", \"error\": \"Wrong state\"}".to_string()
    })
}

pub fn snapshot_project(_path: &Variant, ctrl: &mut Object) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        let maybe_project: &Option<Project> = &*PROJECT.read().unwrap();

        if let &Some(ref p) = maybe_project {
            match p.snapshot(&Path::new(s)) {
                Ok(_) => {
                    set_dirty(false,ctrl);
                    "{\"status\": \"ok\"}".to_string()
                },
                Err(s) => {
                    format!("{{\"status\": \"err\", \"error\": \"Failed to save file: {}\"}}",s)
                }
            }
        } else {
            "{\"status\": \"err\", \"error\": \"No project state to save\"}".to_string()
        }
    } else {
        "{\"status\": \"err\", \"error\": \"1st argument is not a string\"}".to_string()
    })
}

pub fn start(ctrl: &mut Object) -> Variant {
    Variant::String(if state(ctrl) == "READY" {
        start_new(ctrl);
        "{\"status\": \"ok\"}"
    } else if state(ctrl) == "READY_RESUME" {
        start_resume(ctrl);
        "{\"status\": \"ok\"}"
    } else {
        "{\"status\": \"err\", \"error\": \"Wrong state\"}"
    }.to_string())
}

/// Starts disassembly
pub fn start_new(_ctrl: &mut Object) {
    set_state("WORKING",_ctrl);

    let mut ctrl = Object::from_ptr(_ctrl.as_ptr());
    thread::spawn(move || {
        let maybe_prog_uuid = {
            let read_guard = PROJECT.read().unwrap();
            let proj: &Project = read_guard.as_ref().unwrap();

            proj.code.first().map(|x| x.uuid)
        };

        if let Some(prog_uuid) = maybe_prog_uuid {
            let todo_funcs = {
                let read_guard = PROJECT.read().unwrap();

                let proj: &Project = read_guard.as_ref().unwrap();
                let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                prog.call_graph.vertices().filter_map(|x| {
                    if let Some(&CallTarget::Todo(_,_,uuid)) = prog.call_graph.vertex_label(x) {
                        Some(uuid)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            };

            for uu in todo_funcs {
                ctrl.emit(DISCOVERED_FUNCTION,&vec!(Variant::String(uu.to_string())));
            }
            set_dirty(true,&mut ctrl);

            loop {
                let maybe_tgt = {
                    let read_guard = PROJECT.read().unwrap();

                    let proj: &Project = read_guard.as_ref().unwrap();
                    let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                    prog.call_graph.vertices().filter_map(|x| {
                        if let Some(&CallTarget::Todo(ref tgt,ref name,uuid)) = prog.call_graph.vertex_label(x) {
                            Some((tgt.clone(),name.clone(),uuid))
                        } else {
                            None
                        }
                    }).next()
                };

                if let Some((Rvalue::Constant(tgt),maybe_name,uuid)) = maybe_tgt {
                    ctrl.emit(STARTED_FUNCTION,&vec!(Variant::String(uuid.to_string())));
                    set_dirty(true,&mut ctrl);

                    let new_fun = {
                        let read_guard = PROJECT.read().unwrap();
                        let pro: &Project = read_guard.as_ref().unwrap();
                        let root = pro.sources.dependencies.vertex_label(pro.sources.root).unwrap();
                        let i = root.iter();
                        let name = maybe_name.unwrap_or(format!("func_{:x}",tgt));
                        let mut fun = Function::with_uuid(name,uuid,root.name().clone());

                        fun = pro.code[0].target.disassemble(Some(fun),i,tgt,root.name().clone());
                        fun.entry_point = fun.find_basic_block_at_address(tgt);
                        fun
                    };

                    if new_fun.cflow_graph.num_vertices() > 0 {
                        let fun_uuid = new_fun.uuid.clone();
                        let new_tgt = {
                            let mut write_guard = PROJECT.write().unwrap();
                            let proj: &mut Project = write_guard.as_mut().unwrap();
                            let prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();

                            prog.insert(CallTarget::Concrete(new_fun))
                        };

                        ctrl.emit(FINISHED_FUNCTION,&vec!(Variant::String(fun_uuid.to_string())));
                        set_dirty(true,&mut ctrl);

                        for a in new_tgt {
                            ctrl.emit(DISCOVERED_FUNCTION,&vec!(Variant::String(a.to_string())));
                            set_dirty(true,&mut ctrl);
                        }
                    } else {
                        println!("failed to disassemble for {}",new_fun.name);

                        {
                            let mut write_guard = PROJECT.write().unwrap();
                            let proj: &mut Project = write_guard.as_mut().unwrap();
                            let prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();

                            prog.insert(CallTarget::Symbolic(new_fun.name,new_fun.uuid));
                        }

                        ctrl.emit(FINISHED_FUNCTION,&vec!(Variant::String(new_fun.uuid.to_string())));
                        set_dirty(true,&mut ctrl);
                    }
                } else {
                    break;
                }
            }
        }

        ctrl.call(DONE,&[]);
    });
}

pub fn start_resume(_ctrl: &mut Object) {
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
}

/// Change the controller state to DONE
pub fn done(ctrl: &mut Object) -> Variant {
    Variant::String(if state(ctrl) == "WORKING" {
        set_state("DONE",ctrl);
        "{\"status\": \"ok\"}"
    } else {
        "{\"status\": \"err\", \"error\": \"Wrong state\"}"
    }.to_string())
}
