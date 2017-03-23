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

use panopticon::{
    Project,
    Function,
    Program,CallTarget,
    ControlFlowTarget,
    Rvalue,
    Result,
    ssa_convertion,
    Lvalue,
    Architecture,
    OpaqueLayer,
    Layer,
    Region,
    Bound,
    World,
    approximate,
    Kset,
};
use panopticon::amd64;
use panopticon::mos;
use panopticon::avr;
use panopticon::loader;

use std::path::Path;
use std::thread;
use std::collections::{
    HashMap,
    HashSet,
};
use std::fmt::Debug;

use qmlrs::{Variant};
use graph_algos::{
    VertexListGraphTrait,
    MutableGraphTrait,
    GraphTrait
};
use controller::{
    DISCOVERED_FUNCTION,
    STARTED_FUNCTION,
    FINISHED_FUNCTION,
    Controller,
    return_json,
};
use uuid::Uuid;

/// Prepares to disassemble a memory image.
pub fn create_raw_project(_path: &Variant, _tgt: &Variant, _base: &Variant, _entry: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        let p = Path::new(s);
        if let &Variant::I64(base) = _base {
            if let &Variant::I64(entry) = _entry {
                if let Some(nam) = p.file_name().and_then(|x| x.to_str()).or(p.to_str()) {
                    if let Ok(b) = OpaqueLayer::open(p) {
                        let mut reg = Region::undefined(nam.to_string(),b.iter().len() + base as u64);

                        reg.cover(Bound::new(base as u64,base as u64 + b.iter().len()),Layer::Opaque(b));

                        if let &Variant::String(ref tgt_s) = _tgt {
                            let iv: Result<Vec<(&'static str,u64,&'static str)>> = {
                                match tgt_s.as_str() {
                                    "mos6502" => mos::Mos::prepare(&reg,&mos::Variant::mos6502()),
                                    "atmega103" => avr::Avr::prepare(&reg,&avr::Mcu::atmega103()),
                                    "atmega8" => avr::Avr::prepare(&reg,&avr::Mcu::atmega8()),
                                    "atmega88" => avr::Avr::prepare(&reg,&avr::Mcu::atmega88()),
                                    "atmega16" => avr::Avr::prepare(&reg,&avr::Mcu::atmega16()),
                                    _ => Err(format!("No such target '{}'",tgt_s).into()),
                                }
                            };

                            if let Ok(ref iv) = iv {
                                let mut proj = Project{
                                    name: nam.to_string(),
                                    code: Vec::new(),
                                    data: World::new(reg),
                                    comments: HashMap::new(),
                                };
                                let mut prog = Program::new("prog0");

                                if entry >= 0 {
                                    let uu =  Uuid::new_v4();
                                    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry as u64),Some("Entry point".to_string()),uu));
                                    proj.comments.insert((nam.to_string(),entry as u64),"User supplied entry point".to_string());
                                } else {
                                    for &(name,off,cmnt) in iv.iter() {
                                        let uu =  Uuid::new_v4();
                                        prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(off),Some(name.to_string()),uu));
                                        proj.comments.insert((nam.to_string(),off),cmnt.to_string());
                                    }
                                }

                                proj.code.push(prog);

                                let ret = return_json(Controller::replace(proj,None));
                                match tgt_s.as_str() {
                                    "mos6502" => spawn_disassembler::<mos::Mos>(mos::Variant::mos6502()),
                                    "atmega103" => spawn_disassembler::<avr::Avr>(avr::Mcu::atmega103()),
                                    "atmega8" => spawn_disassembler::<avr::Avr>(avr::Mcu::atmega8()),
                                    "atmega88" => spawn_disassembler::<avr::Avr>(avr::Mcu::atmega88()),
                                    "atmega16" => spawn_disassembler::<avr::Avr>(avr::Mcu::atmega16()),
                                    _ => unreachable!()
                                }

                                ret
                            } else {
                                return_json::<()>(Err(iv.err().unwrap()))
                            }
                        } else {
                            return_json::<()>(Err("2nd argument is not a string".into()))
                        }
                    } else {
                        return_json::<()>(Err("Can't open file".into()))
                    }
                } else {
                    return_json::<()>(Err("Can't get file name".into()))
                }
            } else {
                return_json::<()>(Err("4th argument is not an integer".into()))
            }
        } else {
            return_json::<()>(Err("3rd argument is not an integer".into()))
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

/// Prepares to disassemble an ELF or PE file.
pub fn create_project(_path: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        match loader::load(Path::new(s)) {
            Ok((proj,machine)) => {
                match machine {
                    loader::Machine::Ia32 => spawn_disassembler::<amd64::Amd64>(amd64::Mode::Protected),
                    loader::Machine::Amd64 => spawn_disassembler::<amd64::Amd64>(amd64::Mode::Long),
                    loader::Machine::Avr => spawn_disassembler::<avr::Avr>(avr::Mcu::atmega88()),
                }

                return_json(Controller::replace(proj,None))
            },
            Err(err) => return_json::<()>(Err(format!("Failed to read file: {:?}", err).into())),
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

/// Prepares to open a saved Panopticon project.
pub fn open_project(_path: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        match Project::open(&Path::new(s)) {
            Ok(proj) => {
                let ret = return_json(Controller::replace(proj,Some(&Path::new(s))));
                spawn_discoverer();
                ret
            },
            Err(_) => return_json::<()>(Err("Failed to open file".into())),
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

pub fn snapshot_project(_path: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        return_json(Controller::set_backing(&Path::new(s)).and_then(|_| {
            Controller::sync()
        }))
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

pub fn request() -> Variant {
    Variant::String(return_json(Controller::request()))
}

pub fn set_request(_req: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _req {
        return_json(Controller::set_request(s))
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

/// Starts disassembly
pub fn spawn_disassembler<A: 'static + Architecture + Debug>(_cfg: A::Configuration) where A::Configuration: Debug + Sync, A::Token: Sync + Send {
    use std::sync::Mutex;

    thread::spawn(move || -> Result<()> {
        let maybe_prog_uuid = try!(Controller::read(|proj| {
            proj.code.first().map(|x| x.uuid)
        }));

        if let Some(prog_uuid) = maybe_prog_uuid {
            let todo_funcs = try!(Controller::read(|proj| {
                let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                prog.call_graph.vertices().filter_map(|x| {
                    if let Some(&CallTarget::Todo(_,_,uuid)) = prog.call_graph.vertex_label(x) {
                        Some(uuid)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            }));

            for uu in todo_funcs {
                try!(Controller::emit(DISCOVERED_FUNCTION,&uu.to_string()));
            }

            loop {
                let maybe_tgt = try!(Controller::read(|proj| {
                    let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                    prog.call_graph.vertices().filter_map(|x| {
                        if let Some(&CallTarget::Todo(ref tgt@Rvalue::Constant{ .. },ref name,uuid)) = prog.call_graph.vertex_label(x) {
                            Some((tgt.clone(),name.clone(),uuid))
                        } else {
                            None
                        }
                    }).next()
                }));

                match maybe_tgt {
                    Some((Rvalue::Constant{ value: tgt,.. },maybe_name,uuid)) => {
                        try!(Controller::emit(STARTED_FUNCTION,&uuid.to_string()));

                        let cfg = _cfg.clone();
                        let th = thread::spawn(move || -> Result<Vec<Uuid>> {
                            let entry = tgt;
                            let mut func = try!(Controller::read(|proj| {
                                let name = maybe_name.unwrap_or(format!("func_{:x}",tgt));
                                let root = proj.data.dependencies.vertex_label(proj.data.root).unwrap();
                                Function::with_uuid(name,uuid,root.name().clone())
                            }));

                            debug!("start new function {:?} at {:?}",uuid,entry);

                            func = try!(Controller::read(|proj| {
                                let root = proj.data.dependencies.vertex_label(proj.data.root).unwrap();
                                let mut func = {
                                    Function::disassemble::<A>(Some(func),cfg.clone(),&root,entry)
                                };

                                func.entry_point = func.find_basic_block_by_start(entry);

                                func
                            }));

                            if func.cflow_graph.num_vertices() == 0 || func.entry_point.is_none() {
                                debug!("failed to disassemble for {}",func.name);

                                let uu = func.uuid.clone();
                                try!(Controller::modify(|proj| {
                                    let mut prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();
                                    prog.insert(CallTarget::Concrete(func));
                                }));

                                try!(Controller::emit(FINISHED_FUNCTION,&uu.to_string()));
                                return Ok(vec![]);
                            }

                            debug!("primary pass done");

                            let mut fixpoint = false;

                            while !fixpoint {
                                fixpoint = true;
                                ssa_convertion(&mut func);

                                let vals = try!(approximate::<Kset>(&func));
                                let vxs = { func.cflow_graph.vertices().collect::<Vec<_>>() };
                                let mut resolved_jumps = HashSet::<u64>::new();

                                for &vx in vxs.iter() {
                                    if let Some(&mut ControlFlowTarget::Unresolved(ref mut var@Rvalue::Variable{..})) = func.cflow_graph.vertex_label_mut(vx) {
                                        if let Some(&Kset::Set(ref v)) = vals.get(&Lvalue::from_rvalue(var.clone()).unwrap()) {
                                            if let Some(&(val,sz)) = v.first() {
                                                *var = Rvalue::Constant{ value: val, size: sz };
                                                fixpoint = true;
                                                debug!("resolved {:?} to {:?}",var,val);
                                                resolved_jumps.insert(val);
                                            }
                                        }
                                    }
                                }

                                for addr in resolved_jumps {
                                    debug!("continue at {:?}",addr);
                                    func = try!(Controller::read(|proj| {
                                        let root = proj.data.dependencies.vertex_label(proj.data.root).unwrap();
                                        let mut func = {
                                            Function::disassemble::<A>(Some(func),cfg.clone(),&root,addr)
                                        };

                                        func.entry_point = func.find_basic_block_by_start(entry);

                                        func
                                    }));
                                }

                                debug!("secondary pass done");
                            }

                            let new_functions = try!(Controller::modify(|proj| {
                                let mut prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();

                                prog.insert(CallTarget::Concrete(func))
                            }));

                            debug!("function finished");

                            Ok(new_functions)
                        });

                        match th.join() {
                            Ok(Ok(ref new_functions)) => {
                                try!(Controller::emit(FINISHED_FUNCTION,&uuid.to_string()));
                                for a in new_functions {
                                    debug!("found new func at {:?}",a);
                                    try!(Controller::emit(DISCOVERED_FUNCTION,&a.to_string()));
                                }
                            },
                            Err(e) => {
                                error!("error while disassembling {:?}: {:?}",uuid,e);
                                try!(Controller::emit(FINISHED_FUNCTION,&uuid.to_string()))
                            },
                            Ok(Err(e)) => {
                                error!("error while disassembling {:?}: {:?}",uuid,e);
                                try!(Controller::emit(FINISHED_FUNCTION,&uuid.to_string()))
                            },
                        }
                    }
                    Some((rv,maybe_name,uuid)) => {
                        debug!("skip call to {:?} ({:?},{:?})",rv,maybe_name,uuid);
                    }
                    None => {
                        break;
                    }
                }
            }
        } else {
            unreachable!()
        }

        Ok(())
    });
}

pub fn spawn_discoverer() {
    thread::spawn(move || -> Result<()> {
        let uuids = try!(Controller::read(|proj| {
            proj.code.iter().flat_map(|p| p.call_graph.vertices().filter_map(move |vx| {
                p.call_graph.vertex_label(vx).map(|x| x.uuid()) })).collect::<Vec<_>>()
        }));

        for uu in uuids {
            try!(Controller::emit(FINISHED_FUNCTION,&uu.to_string()));
        }

        Ok(())
    });
}
