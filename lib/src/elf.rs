/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015, 2016  Panopticon authors
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

//! Loader for 32 and 64-bit ELF files.

use std::io::{Seek,SeekFrom,Read};
use std::fs::File;
use std::path::Path;

use graph_algos::MutableGraphTrait;
use uuid::Uuid;
use goblin::elf::{self,program_header};

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

/// CPU the ELF file is intended for.
#[derive(Clone,Copy,Debug)]
pub enum Machine {
    /// 8-bit AVR
    Avr,
    /// AMD64
    Amd64,
    /// Intel x86
    Ia32,
}

/// Load an ELF file from disk and creates a `Project` from it. Returns the `Project` instance and
/// the CPU its intended for.
pub fn load(p: &Path) -> Result<(Project,Machine)> {
    let mut fd = try!(File::open(p));
    let binary = try!(elf::parse(&mut fd));
    let entry = binary.entry();
    let (machine, mut reg) = match binary.header().e_machine() {
        elf::header::EM_X86_64 => {
            let reg = Region::undefined("RAM".to_string(), 0xFFFF_FFFF_FFFF_FFFF);
            (Machine::Amd64,reg)
        },
        elf::header::EM_386 => {
            let reg = Region::undefined("RAM".to_string(), 0x1_0000_0000);
            (Machine::Ia32,reg)
        },
        elf::header::EM_AVR => {
            let reg = Region::undefined("Flash".to_string(), 0x2_0000);
            (Machine::Avr,reg)
        },
        machine => return Err(format!("Unsupported machine: {}", machine).into())
    };

    for ph in binary.program_headers() {
        if ph.p_type() == program_header::PT_LOAD {
            let mut buf = vec![0u8; ph.p_filesz() as usize];

            debug!("Load ELF {} bytes segment to {:#x}",ph.p_filesz(),ph.p_vaddr());

            if fd.seek(SeekFrom::Start(ph.p_offset() as u64)).ok() == Some(ph.p_offset() as u64) {
                try!(fd.read_exact(&mut buf));
                reg.cover(Bound::new(ph.p_vaddr() as u64, (ph.p_vaddr() + ph.p_filesz()) as u64), Layer::wrap(buf));
            } else {
                return Err("Failed to read segment".into())
            }
        }
    }

    let name =
        if let &Some(ref soname) = binary.soname() {
            soname.to_owned()
        } else {
            p.file_name()
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or("(encoding error)".to_string())
        };

    if let Some(dynamic) = binary.dynamic() {
        for dyn in dynamic {
            println!("{:?}", dyn);
        }
    }

    println!("interpreter: {:?}", binary.interpreter());

    let mut prog = Program::new("prog0");
    let mut proj = Project::new(name.clone(),reg);

    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry as u64),Some(name),Uuid::new_v4()));

    let dynstrtab = binary.dynstrtab();
    for sym in binary.dynsyms() {
        let name = dynstrtab[sym.st_name()].to_string();
        let addr = sym.st_value();
        println!("{} @ 0x{:x}: {:?}", name, addr, sym);
        if sym.is_function() {
            if sym.is_import() {
                prog.call_graph.add_vertex(CallTarget::Symbolic(name,Uuid::new_v4()));
            } else {
                prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(addr),Some(name),Uuid::new_v4()));
            }
        }
    }

    proj.comments.insert(("base".to_string(),entry),"main".to_string());
    proj.code.push(prog);

    Ok((proj,machine))
}
