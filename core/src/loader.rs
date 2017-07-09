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

//! Loader for 32 and 64-bit ELF, PE, and Mach-o files.


use {Bound, CallTarget, Layer, Program, Project, Region, Result, Rvalue};
use goblin::{self, Hint, archive, elf, mach, pe};
use goblin::elf::program_header;

use panopticon_graph_algos::MutableGraphTrait;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use uuid::Uuid;

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

/// Parses a non-fat Mach-o binary from `bytes` at `offset` and creates a `Project` from it. Returns the `Project` instance and
/// the CPU its intended for.
pub fn load_mach(bytes: &[u8], offset: usize, name: String) -> Result<(Project, Machine)> {
    let binary = mach::MachO::parse(&bytes, offset)?;
    debug!("mach: {:#?}", &binary);
    let mut base = 0x0;
    let cputype = binary.header.cputype;
    let (machine, mut reg) = match cputype {
        mach::cputype::CPU_TYPE_X86 => {
            let reg = Region::undefined("RAM".to_string(), 0x1_0000_0000);
            (Machine::Ia32, reg)
        }
        mach::cputype::CPU_TYPE_X86_64 => {
            let reg = Region::undefined("RAM".to_string(), 0xFFFF_FFFF_FFFF_FFFF);
            (Machine::Amd64, reg)
        }
        machine => {
            return Err(
                format!(
                    "Unsupported machine ({:#x}): {}",
                    machine,
                    mach::cputype::cpu_type_to_str(machine)
                )
                        .into()
            )
        }
    };

    for segment in &*binary.segments {
        let offset = segment.fileoff as usize;
        let filesize = segment.filesize as usize;
        if offset + filesize > bytes.len() {
            return Err(
                format!(
                    "Failed to read segment: range {:?} greater than len {}",
                    offset..offset + filesize,
                    bytes.len()
                )
                        .into()
            );
        }
        let section = &bytes[offset..offset + filesize];
        let start = segment.vmaddr;
        let end = start + segment.vmsize;
        let name = segment.name()?;
        debug!(
            "Load mach segment {:?}: {} bytes segment to {:#x}",
            name,
            segment.vmsize,
            start
        );
        reg.cover(Bound::new(start, end), Layer::wrap(Vec::from(section)));
        if name == "__TEXT" {
            base = segment.vmaddr;
            debug!("Setting vm address base to {:#x}", base);
        }
    }

    let name = if let &Some(ref name) = &binary.name {
        name.to_string()
    } else {
        name
    };

    let mut prog = Program::new("prog0");
    let mut proj = Project::new(name.clone(), reg);

    let entry = binary.entry;

    if entry != 0 {
        prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry as u64), Some(name), Uuid::new_v4()));
    }

    for export in binary.exports()? {
        if export.offset != 0 {
            debug!("adding: {:?}", &export);
            prog.call_graph
                .add_vertex(
                    CallTarget::Todo(
                        Rvalue::new_u64(export.offset as u64 + base),
                        Some(export.name),
                        Uuid::new_v4(),
                    )
                );
        }
    }

    for import in binary.imports()? {
        debug!("Import {}: {:#x}", import.name, import.offset);
        proj.imports.insert(import.offset, import.name.to_string());
    }

    debug!("Imports: {:?}", &proj.imports);

    proj.comments.insert(("base".to_string(), entry), "main".to_string());
    proj.code.push(prog);

    Ok((proj, machine))
}

/// Parses an ELF 32/64-bit binary from `bytes` and creates a `Project` from it. Returns the `Project` instance and
/// the CPU its intended for.
fn load_elf(bytes: &[u8], name: String) -> Result<(Project, Machine)> {
    use std::collections::HashSet;

    let mut cursor = Cursor::new(&bytes);
    let binary = elf::Elf::parse(&bytes)?;
    debug!("elf: {:#?}", &binary);

    let entry = binary.entry;
    let (machine, mut reg) = match binary.header.e_machine {
        elf::header::EM_X86_64 => {
            let reg = Region::undefined("RAM".to_string(), 0xFFFF_FFFF_FFFF_FFFF);
            (Machine::Amd64, reg)
        }
        elf::header::EM_386 => {
            let reg = Region::undefined("RAM".to_string(), 0x1_0000_0000);
            (Machine::Ia32, reg)
        }
        elf::header::EM_AVR => {
            let reg = Region::undefined("Flash".to_string(), 0x2_0000);
            (Machine::Avr, reg)
        }
        machine => return Err(format!("Unsupported machine: {}", machine).into()),
    };

    for ph in &binary.program_headers {
        if ph.p_type == program_header::PT_LOAD {
            let mut buf = vec![0u8; ph.p_filesz as usize];

            debug!(
                "Load ELF {} bytes segment to {:#x}",
                ph.p_filesz,
                ph.p_vaddr
            );

            if cursor.seek(SeekFrom::Start(ph.p_offset)).ok() == Some(ph.p_offset) {
                cursor.read_exact(&mut buf)?;
                reg.cover(
                    Bound::new(ph.p_vaddr, ph.p_vaddr + ph.p_filesz),
                    Layer::wrap(buf),
                );
            } else {
                return Err("Failed to read segment".into());
            }
        }
    }

    let name = if let &Some(ref soname) = &binary.soname {
        soname.to_owned()
    } else {
        name
    };

    debug!("interpreter: {:?}", &binary.interpreter);

    let mut prog = Program::new("prog0");
    let mut proj = Project::new(name.clone(), reg);

    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(entry as u64), Some(name), Uuid::new_v4()));

    let add_sym = |prog: &mut Program, sym: &elf::Sym, name: &str| {
        let name = name.to_string();
        let addr = sym.st_value;
        debug!("Symbol: {} @ 0x{:x}: {:?}", name, addr, sym);
        if sym.is_function() {
            if sym.is_import() {
                prog.call_graph.add_vertex(CallTarget::Symbolic(name, Uuid::new_v4()));
            } else {
                prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::new_u64(addr), Some(name), Uuid::new_v4()));
            }
        }
    };

    let resolve_import_address = |proj: &mut Project, relocs: &[elf::Reloc], name: &str| {
        for reloc in relocs {
            let pltsym = &binary.dynsyms[reloc.r_sym];
            let pltname = &binary.dynstrtab[pltsym.st_name];
            if pltname == name {
                debug!("Import match {}: {:#x} {:?}", name, reloc.r_offset, pltsym);
                proj.imports.insert(reloc.r_offset as u64, name.to_string());
                return true;
            }
        }
        false
    };

    let mut seen_syms = HashSet::<u64>::new();

    // add dynamic symbol information (non-strippable)
    for sym in &binary.dynsyms {
        let name = &binary.dynstrtab[sym.st_name];

        add_sym(&mut prog, sym, name);
        seen_syms.insert(sym.st_value);

        if sym.is_function() {
            let name = &binary.dynstrtab[sym.st_name];
            if !resolve_import_address(&mut proj, &binary.pltrelocs, name) {
                if !resolve_import_address(&mut proj, &binary.dynrelas, name) {
                    resolve_import_address(&mut proj, &binary.dynrels, name);
                }
            }
        }
    }
    debug!("Imports: {:#?}", &proj.imports);

    // add strippable symbol information
    for sym in &binary.syms {
        let name = &binary.strtab[sym.st_name];
        if !seen_syms.contains(&sym.st_value) {
            add_sym(&mut prog, sym, &name);
        }
        seen_syms.insert(sym.st_value);
    }
    prog.imports = proj.imports.clone();
    proj.comments.insert(("base".to_string(), entry), "main".to_string());
    proj.code.push(prog);

    Ok((proj, machine))
}

/// Parses a PE32/PE32+ file from `bytes` and create a project from it.
fn load_pe(bytes: &[u8], name: String) -> Result<(Project, Machine)> {
    let pe = pe::PE::parse(&bytes)?;
    debug!("pe: {:#?}", &pe);
    let image_base = pe.image_base as u64;
    let mut ram = Region::undefined("RAM".to_string(), 0x100000000);
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
                    debug!(
                        "bad section pointer: {:#x} + {:#x} >= {:#x}",
                        offset,
                        size,
                        bytes.len()
                    );
                    (Layer::undefined(0), 0)
                } else {
                    debug!("mapped '{}': {:?}", name, offset..offset + size);
                    (Layer::wrap(bytes[offset..offset + size].to_vec()), size as u64)
                }
            } else {
                debug!("bss '{}'", name);
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
    let mut proj = Project::new(name.to_string(), ram);

    prog.call_graph
        .add_vertex(
            CallTarget::Todo(
                Rvalue::new_u64(entry),
                Some(name.to_string()),
                Uuid::new_v4(),
            )
        );

    for export in pe.exports {
        debug!("adding export: {:?}", &export);
        prog.call_graph
            .add_vertex(
                CallTarget::Todo(
                    Rvalue::new_u64(export.rva as u64 + image_base),
                    Some(export.name.to_string()),
                    Uuid::new_v4(),
                )
            );
    }

    for import in pe.imports {
        debug!(
            "adding import: {:?} @ {:#x}",
            &import,
            import.rva + pe.image_base
        );
        prog.call_graph.add_vertex(CallTarget::Symbolic(import.name.into_owned(), Uuid::new_v4()));
    }

    proj.comments.insert(("base".to_string(), entry), "main".to_string());
    proj.code.push(prog);
    Ok((proj, Machine::Ia32))
}

/// Load an ELF or PE file from disk and creates a `Project` from it. Returns the `Project` instance and
/// the CPU its intended for.
pub fn load(path: &Path) -> Result<(Project, Machine)> {
    let name = path.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or("(encoding error)".to_string());
    let mut fd = File::open(path)?;
    let peek = goblin::peek(&mut fd)?;
    if let Hint::Unknown(magic) = peek {
        Err(format!("Tried to load an unknown file. Magic: {}", magic).into())
    } else {
        let mut bytes = Vec::new();
        fd.read_to_end(&mut bytes)?;
        match peek {
            Hint::Elf(_) => load_elf(&bytes, name),
            Hint::PE => load_pe(&bytes, name),
            Hint::Mach(_) => load_mach(&bytes, 0, name),
            Hint::MachFat(_) => Err("Cannot directly load a fat mach-o binary (e.g., which one do I load?)".into()),
            Hint::Archive => {
                let archive = archive::Archive::parse(&bytes)?;
                debug!("archive: {:#?}", &archive);
                Err("Tried to load an archive, unsupported format".into())
            }
            _ => {
                println!(
                    "Loader branch hit wildcard, should be unreachable (a new variant must have been added but code was not updated)",
                );
                unreachable!()
            }
        }
    }
}
