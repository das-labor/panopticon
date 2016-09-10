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

extern crate goblin;

use std::io::{Seek,SeekFrom,Read};
use std::fs::File;
use std::path::Path;

use graph_algos::MutableGraphTrait;
use uuid::Uuid;
use goblin::elf::Binary;

use {
    Program,
    CallTarget,
    Project,
    Layer,
    Region,
    Bound,
    Rvalue,
    Result,
};
use elf::{
    Machine,
};

macro_rules! extract_data { ($elf:expr, $fd:expr, $interp:expr, $entry:expr, $reg:expr) => {
    $entry = $elf.entry;
    $interp = $elf.interpreter;
    println!("Soname: {:?} with interpreter: {:?}", $elf.soname, $interp);
    // psst i don't really know what this is doing semantically, just copied from old code
    for ph in $elf.program_headers {
        let mut buf = vec![0u8; ph.p_filesz as usize];
        if $fd.seek(SeekFrom::Start(ph.p_offset as u64)).ok() == Some(ph.p_offset as u64) {
            $reg.cover(Bound::new(ph.p_vaddr as u64, (ph.p_vaddr + ph.p_filesz) as u64), Layer::wrap(buf));
        }
        else {
            return Err("Failed to read segment".into())
        }
    }
};}

pub fn load(p: &Path) -> Result<(Project,Machine)> {
    let mut entry = 0x0;
    let mut interp = None;
    let mut fd = File::open(p).ok().unwrap();
    let mut reg = Region::undefined("base".to_string(), 0x1000000000000);
    match goblin::elf::from_fd(&mut fd) {
        Ok(Binary::Elf64(elf)) => {
            extract_data!(elf, fd, interp, entry, reg);
        },
        Ok(Binary::Elf32(elf)) => {
            extract_data!(elf, fd, interp, entry, reg);
        },
        _ => {}
    }

    let name = p.file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("(encoding error)".to_string());

    let mut prog = Program::new("prog0");
    let mut proj = Project::new(name.clone(),reg);

    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry as u64),Some(name),Uuid::new_v4()));
    proj.comments.insert(("base".to_string(),entry as u64),"main".to_string());
    proj.code.push(prog);

    // TODO: add proper ELF machine -> Enum translation here
    // my guess is you'll want to pull out the Machine::* value
    // from the appropriate ELF datastructure and add it to generic project data
    // since machine type is independent of binary format and could be used for mach, pe, etc.
    Ok((proj,Machine::X86_64))
}
