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
    elf,
    pe,
    Target,
    Rvalue,
    Result,
    ssa_convertion,
};

use std::path::Path;
use std::thread;
use qmlrs::{Variant,Object};
use graph_algos::{
    VertexListGraphTrait,
    GraphTrait
};
use controller::{
    DISCOVERED_FUNCTION,
    STARTED_FUNCTION,
    FINISHED_FUNCTION,
    Controller,
    return_json,
};

/// Prepares to disassemble a memory image.
pub fn create_raw_project(_path: &Variant, _tgt: &Variant, _base: &Variant, _entry: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        if let &Variant::String(ref tgt_s) = _tgt {
            if let Some(tgt) = Target::for_name(tgt_s) {
                if let &Variant::I64(ref base) = _base {
                    if let &Variant::I64(ref entry) = _entry {
                        match Project::raw(&Path::new(s),tgt,*base as u64,if *entry >= 0 { Some(*entry as u64) } else { None }) {
                            Some(proj) => {
                                let ret = return_json(Controller::replace(proj,None));
                                spawn_disassembler();
                                ret
                            },
                            None => return_json::<()>(Err("Can't open project: Unknown error".into())),
                        }
                    } else {
                        return_json::<()>(Err("4th argument is not an integer".into()))
                    }
                } else {
                    return_json::<()>(Err("3rd argument is not an integer".into()))
                }
            } else {
                return_json::<()>(Err(format!("No such target '{}'",tgt_s).into()))
            }
        } else {
            return_json::<()>(Err("2nd argument is not a string".into()))
        }
    } else {
        return_json::<()>(Err("1st argument is not a string".into()))
    })
}

/// Prepares to disassemble an ELF file.
pub fn create_elf_project(_path: &Variant) -> Variant {
    Variant::String(if let &Variant::String(ref s) = _path {
        match elf::load::load(Path::new(s)) {
            Ok(proj) => {
                let ret = return_json(Controller::replace(proj,None));
                spawn_disassembler();
                ret
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
            Some(proj) => {
                let ret = return_json(Controller::replace(proj,None));
                spawn_disassembler();
                ret
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
pub fn spawn_disassembler() {
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
                Controller::emit1(DISCOVERED_FUNCTION,&uu.to_string());
            }

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

                if let Some((Rvalue::Constant(tgt),maybe_name,uuid)) = maybe_tgt {
                    try!(Controller::emit1(STARTED_FUNCTION,&uuid.to_string()));

                    let name = maybe_name.unwrap_or(format!("func_{:x}",tgt));
                    let new_fun = try!(Controller::read(|proj| {
                        let root = proj.sources.dependencies.vertex_label(proj.sources.root).unwrap();
                        let i = root.iter();
                        let mut fun = Function::with_uuid(name,uuid,root.name().clone());

                        fun = proj.code[0].target.disassemble(Some(fun),i,tgt,root.name().clone());
                        fun.entry_point = fun.find_basic_block_at_address(tgt);

                        if fun.entry_point.is_some() && fun.cflow_graph.num_vertices() > 0 {
                            println!("{}",fun.to_dot());

                            ssa_convertion(&mut fun);
                        }
                        fun
                    }));

                    if new_fun.cflow_graph.num_vertices() > 0 {
                        let fun_uuid = new_fun.uuid.clone();
                        let new_tgt = try!(Controller::modify(|proj| {
                            let mut prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();

                            prog.insert(CallTarget::Concrete(new_fun))
                        }));

                        Controller::emit1(FINISHED_FUNCTION,&fun_uuid.to_string());

                        for a in new_tgt {
                            Controller::emit1(DISCOVERED_FUNCTION,&a.to_string());
                        }
                    } else {
                        println!("failed to disassemble for {}",new_fun.name);

                        try!(Controller::modify(|proj| {
                            let mut prog: &mut Program = proj.find_program_by_uuid_mut(&prog_uuid).unwrap();
                            prog.insert(CallTarget::Symbolic(new_fun.name.clone(),new_fun.uuid));
                        }));

                        try!(Controller::emit1(FINISHED_FUNCTION,&new_fun.uuid.to_string()));
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
            try!(Controller::emit1(FINISHED_FUNCTION,&uu.to_string()));
        }

        Ok(())
    });
}
