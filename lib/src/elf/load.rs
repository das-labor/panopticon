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

use std::io::{Seek,SeekFrom,Read};
use std::fs::File;
use std::path::Path;

use program::{Program,CallTarget};
use project::Project;
use layer::Layer;
use region::Region;
use mnemonic::Bound;
use target::Target;
use value::Rvalue;

use graph_algos::MutableGraphTrait;
use uuid::Uuid;
use elf::*;
use elf::parse::*;

pub fn load(p: &Path) -> Result<Project,Error> {
    let mut fd = File::open(p).ok().unwrap();
    let ehdr = try!(Ehdr::read(&mut fd));
    let mut reg = Region::undefined("base".to_string(), 0x1000000000000);

    match ehdr.file_type {
        Type::Core | Type::Executable | Type::Shared => {
            for ph in ehdr.progam_headers.iter() {
                match ph.seg_type {
                    SegmentType::Load => {
                        println!("load segment of {} bytes from {:x} to {:x}",ph.filesz,ph.offset,ph.vaddr);
                        if fd.seek(SeekFrom::Start(ph.offset)).ok() == Some(ph.offset) {
                            let mut buf = vec![0u8; ph.filesz as usize];
                            if let Err(_) = fd.read(&mut buf) {
                                return Err(Error::new("Failed to read segment"));
                            }

                            reg.cover(Bound::new(ph.vaddr,ph.vaddr + ph.filesz),Layer::wrap(buf));
                        } else {
                            return Err(Error::new("Failed to read segment"));
                        }
                    },
                    SegmentType::Interp => {
                        if fd.seek(SeekFrom::Start(ph.offset)).ok() == Some(ph.offset) {
                            let mut interp = vec![0u8; ph.filesz as usize];
                            if let Err(_) = fd.read(&mut interp) {
                                return Err(Error::new("Failed to read interpreter path"));
                            }

                            match String::from_utf8(interp) {
                                Ok(s) => println!("load interpreter {}",s),
                                Err(_) => println!("load intepreter (encoding error)"),
                            }
                        } else {
                            return Err(Error::new("Failed to read interpreter path"));
                        }
                    },
                    _ => {},
                }
            }
        },
        _ => {}
    }

    let name = p.file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("(encoding error)".to_string());
    if let Some(target) = Target::for_elf(ehdr.machine).first() {
        let mut prog = Program::new("prog0",*target);
        let mut proj = Project::new(name.clone(),reg);

        prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::Constant(ehdr.entry),Some(name),Uuid::new_v4()));
        proj.comments.insert(("base".to_string(),ehdr.entry),"main".to_string());
        proj.code.push(prog);

        Ok(proj)
    } else {
        Err(Error::new("Unknown ELF machine type"))
    }
}
