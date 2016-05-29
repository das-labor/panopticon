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
    elf,
    pe,
    Rvalue,
    Result,
    ssa_convertion,
    Lvalue,
    Architecture,
    OpaqueLayer,
    Layer,
    Region,
    Bound,
    Regions,
    approximate,
    Kset,
};
use panopticon::amd64;
use panopticon::mos;
use panopticon::mos::{Mos};
use panopticon::avr::{Avr,Mcu};

use std::path::Path;
use std::thread;
use std::collections::HashMap;

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
                    if let Some(b) = OpaqueLayer::open(p) {
                        let mut reg = Region::undefined(nam.to_string(),b.iter().len() + base as u64);

                        reg.cover(Bound::new(base as u64,base as u64 + b.iter().len()),Layer::Opaque(b));

                        if let &Variant::String(ref tgt_s) = _tgt {
                            let iv = {
                                let i = reg.iter();
                                match tgt_s.as_str() {
                                    "mos6502" => Mos::prepare(i,&mos::Variant::mos6502()),
                                    "atmega103" => Avr::prepare(i,&Mcu::atmega103()),
                                    "atmega8" => Avr::prepare(i,&Mcu::atmega8()),
                                    "atmega88" => Avr::prepare(i,&Mcu::atmega88()),
                                    "atmega16" => Avr::prepare(i,&Mcu::atmega16()),
                                    _ => Err(format!("No such target '{}'",tgt_s).into()),
                                }
                            };

                            if let Ok(ref iv) = iv {
                                let mut proj = Project{
                                    name: nam.to_string(),
                                    code: Vec::new(),
                                    sources: Regions::new(reg),
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
                                    "mos6502" => spawn_disassembler::<Mos>(mos::Variant::mos6502()),
                                    "atmega103" => spawn_disassembler::<Avr>(Mcu::atmega103()),
                                    "atmega8" => spawn_disassembler::<Avr>(Mcu::atmega8()),
                                    "atmega88" => spawn_disassembler::<Avr>(Mcu::atmega88()),
                                    "atmega16" => spawn_disassembler::<Avr>(Mcu::atmega16()),
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

/// Prepares to disassemble an ELF file.
pub fn create_elf_project(_path: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        match elf::load::load(Path::new(s)) {
            Ok((proj,m)) => {
                match m {
                    elf::Machine::i386 => spawn_disassembler::<amd64::Amd64>(amd64::Config::new(amd64::Mode::Protected)),
                    elf::Machine::X86_64 => spawn_disassembler::<amd64::Amd64>(amd64::Config::new(amd64::Mode::Long)),
                    _ => return Variant::String(return_json::<()>(Err("Unsupported architecture".into()))),
                }

                return_json(Controller::replace(proj,None))
            },
            Err(_) => return_json::<()>(Err("Failed to read ELF file".into())),
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

/// Prepares to disassemble an PE file.
pub fn create_pe_project(_path: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        match pe::pe(Path::new(s)) {
            Some(_) => {
                return Variant::String(return_json::<()>(Err("Unsupported format".into())));
            },
            None => return_json::<()>(Err("Failed to read PE file".into())),
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

/// Starts disassembly
pub fn spawn_disassembler<A: 'static + Architecture>(cfg: A::Configuration) {
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

            let dec = A::disassembler(&cfg);

            loop {
                let maybe_tgt = try!(Controller::read(|proj| {
                    let prog: &Program = proj.find_program_by_uuid(&prog_uuid).unwrap();

                    prog.call_graph.vertices().filter_map(|x| {
                        if let Some(&CallTarget::Todo(ref tgt,ref name,uuid)) = prog.call_graph.vertex_label(x) {
                            Some((tgt.clone(),name.clone(),uuid))
                        } else {
                            None
                        }
                    }).next()
                }));

                if let Some((Rvalue::Constant{ value: tgt,.. },maybe_name,uuid)) = maybe_tgt {
                    try!(Controller::emit(STARTED_FUNCTION,&uuid.to_string()));

                    let name = maybe_name.unwrap_or(format!("func_{:x}",tgt));
                    let mut new_fun = try!(Controller::read(|proj| {
                        let root = proj.sources.dependencies.vertex_label(proj.sources.root).unwrap();
                        let i = root.iter();
                        let mut fun = Function::with_uuid(name,uuid,root.name().clone());

                        fun = Function::disassemble::<A>(Some(fun),dec.clone(),cfg.clone(),i,tgt,root.name().clone());
                        fun.entry_point = fun.find_basic_block_at_address(tgt);

                        if fun.entry_point.is_some() && fun.cflow_graph.num_vertices() > 0 {
                            ssa_convertion(&mut fun);
                        }
                        fun
                    }));

                    if new_fun.cflow_graph.num_vertices() > 0 {
                        let mut fixpoint = false;

                        while !fixpoint {
                            let vals = approximate::<Kset>(&new_fun);
                            let vxs = { new_fun.cflow_graph.vertices().collect::<Vec<_>>() };
                            let mut new_tgts = vec![];

                            println!("{:?}",vals);
                            fixpoint = false;

                            for &vx in vxs.iter() {
                                if let Some(&mut ControlFlowTarget::Unresolved(ref mut var@Rvalue::Variable{..})) = new_fun.cflow_graph.vertex_label_mut(vx) {
                                    if let Some(&Kset::Set(ref v)) = vals.get(&Lvalue::from_rvalue(var.clone()).unwrap()) {
                                        if let Some(&(val,sz)) = v.first() {
                                            println!("resolved {:?} to {:?}",var,val);
                                            *var = Rvalue::Constant{ value: val, size: sz };
                                            fixpoint = true;
                                            new_tgts.push(val);
                                        }
                                    }
                                }
                            }

                            for tgt in new_tgts {
                                new_fun = try!(Controller::read(|proj| {
                                    let root = proj.sources.dependencies.vertex_label(proj.sources.root).unwrap();
                                    let i = root.iter();

                                    let mut fun = Function::disassemble::<A>(Some(new_fun),dec.clone(),cfg.clone(),i,tgt,root.name().clone());
                                    fun.entry_point = fun.find_basic_block_at_address(tgt);

                                    if fun.entry_point.is_some() && fun.cflow_graph.num_vertices() > 0 {
                                 //       ssa_convertion(&mut fun);
                                    }
                                    fun
                                }));
                            }

                            break;
                        }

                        let fun_uuid = new_fun.uuid.clone();
                        let new_tgt = try!(Controller::modify(|proj| {
                            let mut prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();

                            prog.insert(CallTarget::Concrete(new_fun))
                        }));

                        try!(Controller::emit(FINISHED_FUNCTION,&fun_uuid.to_string()));

                        for a in new_tgt {
                            try!(Controller::emit(DISCOVERED_FUNCTION,&a.to_string()));
                        }
                    } else {
                        println!("failed to disassemble for {}",new_fun.name);

                        try!(Controller::modify(|proj| {
                            let mut prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();
                            prog.insert(CallTarget::Symbolic(new_fun.name.clone(),new_fun.uuid));
                        }));

                        try!(Controller::emit(FINISHED_FUNCTION,&new_fun.uuid.to_string()));
                    }
                } else {
                    break;
                }
            }
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
