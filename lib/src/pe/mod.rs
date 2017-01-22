/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
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

//! Loader for PE32 and PE32+ files.

use std::path::Path;
use std::mem;
use std::fs::File;
use std::io::{Read,Seek,SeekFrom};
use goblin;

use project::Project;
use region::Region;
use mnemonic::Bound;
use layer::Layer;

use graph_algos::MutableGraphTrait;
use uuid::Uuid;
use { Program, CallTarget, Rvalue };

/// Loads a PE file from disk and create a project from it.
// todo need to change the signature to a result, so we don't unwrap
pub fn pe(p: &Path) -> Option<Project> {
    let name = p.file_name().and_then(|x| x.to_str()).or(p.to_str()).unwrap_or("unknown pe");

    if let Some(mut fd) = File::open(p).ok() {
        let mut bytes = Vec::new();
        fd.read_to_end(&mut bytes).unwrap();
        let pe = goblin::pe::PE::from_bytes(&bytes).unwrap();
        println!("pe: {:#?}", &pe);
        let image_base = pe.image_base as u64;
        let mut ram = Region::undefined("RAM".to_string(),0x100000000);
        for section in &pe.sections {
            let name = String::from_utf8_lossy(&section.name);
            println!("section: {}", name);
            let size = section.size_of_raw_data as usize;
            let virtual_address = section.virtual_address as u64;
            let offset = section.pointer_to_raw_data as usize;
            let l =
                if size > 0 {
                    if offset + size >= bytes.len() {
                        println!("bad section pointer: {:#x} + {:#x} >= {:#x}", offset, size, bytes.len());
                        Layer::undefined(section.virtual_size as u64)
                            //return None;
                    } else {
                        println!("mapped '{}'",name);
                        Layer::wrap(bytes[offset..offset+size].to_vec())
                    }
                } else {
                    println!("not mapped '{}'",name);
                    Layer::undefined(section.virtual_size as u64)
                };
            let begin = image_base + virtual_address;
            let end = image_base + virtual_address + size as u64;
            if !ram.cover(Bound::new(begin, end),l) {
                println!("bad cover");
                return None;
            }
        }
        let entry = (pe.image_base + pe.entry) as u64;
        println!("entry: {:#x}", entry);
        let mut prog = Program::new("prog0");
        let mut proj = Project::new(name.to_string(),ram);

        prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry),Some(name.to_string()),Uuid::new_v4()));

        proj.comments.insert(("base".to_string(),entry),"main".to_string());
        proj.code.push(prog);
        Some(proj)
    } else {
        None
    }
}
