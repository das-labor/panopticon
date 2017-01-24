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

//! Loader for 32 and 64-bit ELF and PE files.

use std::io::{Seek,SeekFrom,Read,Cursor};
use std::fs::File;
use std::path::Path;

use graph_algos::MutableGraphTrait;
use uuid::Uuid;
use goblin::{self, Hint, pe, elf, mach, archive};
use goblin::elf::{program_header};

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

/// CPU the binary file is intended for.
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
fn load_elf(fd: &mut File, name: String) -> Result<(Project,Machine)> {
    // it seems more efficient to load all bytes into in-memory buffer and parse those...
    // for larger binaries we should perhaps let the elf parser read from the fd though
    let mut bytes = Vec::new();
    try!(fd.read_to_end(&mut bytes));
    let mut cursor = Cursor::new(&bytes);
    let binary = elf::Elf::parse(&bytes)?;
    debug!("elf: {:#?}", &binary);
    let entry = binary.entry;
    let (machine, mut reg) = match binary.header.e_machine() {
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

    for ph in binary.program_headers {
        if ph.p_type() == program_header::PT_LOAD {
            let mut buf = vec![0u8; ph.p_filesz() as usize];

            debug!("Load ELF {} bytes segment to {:#x}",ph.p_filesz(),ph.p_vaddr());

            if cursor.seek(SeekFrom::Start(ph.p_offset() as u64)).ok() == Some(ph.p_offset() as u64) {
                try!(cursor.read_exact(&mut buf));
                reg.cover(Bound::new(ph.p_vaddr() as u64, (ph.p_vaddr() + ph.p_filesz()) as u64), Layer::wrap(buf));
            } else {
                return Err("Failed to read segment".into())
            }
        }
    }

    let name =
        if let Some(soname) = binary.soname {
            soname.to_owned()
        } else {
            name
        };

    debug!("interpreter: {:?}", &binary.interpreter);

    let mut prog = Program::new("prog0");
    let mut proj = Project::new(name.clone(),reg);

    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry as u64),Some(name),Uuid::new_v4()));

    let dynstrtab = binary.dynstrtab;
    for sym in binary.dynsyms {
        let name = dynstrtab[sym.st_name() as usize].to_string();
        let addr = sym.st_value();
        debug!("{} @ 0x{:x}: {:?}", name, addr, sym);
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

/// Loads a PE file from disk and create a project from it.
fn load_pe(fd: &mut File, name: String) -> Result<(Project, Machine)> {
    let mut bytes = Vec::new();
    fd.read_to_end(&mut bytes)?;
    let pe = pe::PE::from_bytes(&bytes)?;
    debug!("pe: {:#?}", &pe);
    let image_base = pe.image_base as u64;
    let mut ram = Region::undefined("RAM".to_string(),0x100000000);
    for section in &pe.sections {
        let name = String::from_utf8_lossy(&section.name);
        debug!("section: {}", name);
        let virtual_address = section.virtual_address as u64;
        let offset = section.pointer_to_raw_data as usize;
        let (layer, size) = {
            let vsize = section.virtual_size as u64;
            let size = section.size_of_raw_data as usize;
            if size > 0 {
                if offset + size >= bytes.len() {
                    debug!("bad section pointer: {:#x} + {:#x} >= {:#x}", offset, size, bytes.len());
                    (Layer::undefined(0), 0)
                } else {
                    debug!("mapped '{}': {:?}",name, offset..offset+size);
                    (Layer::wrap(bytes[offset..offset+size].to_vec()), size as u64)
                }
            } else {
                debug!("bss '{}'",name);
                (Layer::undefined(vsize), vsize)
            }
        };
        let begin = image_base + virtual_address;
        let end = image_base + virtual_address + size as u64;
        let bound = Bound::new(begin, end);
        debug!("bound: {:?}", &bound);
        if !ram.cover(bound, layer) {
            debug!("bad cover");
            return Err(format!("Cannot cover bound: {:?}", Bound::new(begin, end)).into());
        }
    }
    let entry = (pe.image_base + pe.entry) as u64;
    debug!("entry: {:#x}", entry);
    let mut prog = Program::new("prog0");
    let mut proj = Project::new(name.to_string(),ram);

    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry),Some(name.to_string()),Uuid::new_v4()));

    for export in pe.exports {
        debug!("adding export: {:?}", &export);
        prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(export.rva as u64 + image_base),Some(export.name),Uuid::new_v4()));
    }

    proj.comments.insert(("base".to_string(),entry),"main".to_string());
    proj.code.push(prog);
    Ok((proj, Machine::Ia32))
}

/// Load an ELF or PE file from disk and creates a `Project` from it. Returns the `Project` instance and
/// the CPU its intended for.
pub fn load(path: &Path) -> Result<(Project,Machine)> {
    let name = path.file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("(encoding error)".to_string());
    let mut fd = File::open(path)?;
    match goblin::peek(&mut fd)? {
        Hint::Elf(_) => {
            load_elf(&mut fd, name)
        },
        Hint::PE => {
            load_pe(&mut fd, name)
        },
        // wip
        Hint::Mach => {
            let mach = mach::Mach::try_from(&mut fd)?;
            println!("mach: {:#?}", &mach);
            Err("Tried to load a mach file, unsupported format".into())
        },
        Hint::Archive => {
            let archive = archive::Archive::try_from(&mut fd)?;
            println!("archive: {:#?}", &archive);
            Err("Tried to load an archive, unsupported format".into())
        },
        _ => {
            Err("Tried to load an unknown file".into())
        }
    }
}
