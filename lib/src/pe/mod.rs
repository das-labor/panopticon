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

use std::path::Path;
use std::mem;
use std::fs::File;
use std::io::{Read,Seek,SeekFrom};

use project::Project;
use region::Region;
use mnemonic::Bound;
use layer::Layer;

#[repr(C,packed)]
struct Mz {
    signature: u16,
    extra_bytes: u16,
    pages: u16,
    reloc_items: u16,
    header_size: u16,
    min_alloc: u16,
    max_alloc: u16,
    initial_ss: u16,
    initial_sp: u16,
    checksum: u16,
    initial_ip: u16,
    initial_cs: u16,
    reloc_table: u16,
    overlay: u16,
    reserved1: [u16; 4],
    oem_id: u16,
    oem_info: u16,
    reserved2: [u16; 10],
    e_lfanew: u32,
}

const MZ_MAGIC: u16 = 0x5a4d;

#[repr(C,packed)]
struct MzReloc {
    offset: u16,
    segment: u16,
}

#[repr(C,packed)]
struct Pe {
    magic: u32,
    machine: u16,
    num_section: u16,
    timestamp: u32,
    symbol_table: u32,
    num_symbols: u32,
    opthdr_size: u16,
    characteristics: u16,
}

#[repr(C,packed)]
struct PeOptional32 {
    magic: u16,
    major: u8,
    minor: u8,
    text_size: u32,
    data_size: u32,
    bss_size: u32,
    entry_point: u32,
    text_base: u32,
    data_base: u32, // w
    image_base: u32, // w
    section_align: u32,
    file_align: u32,
    os_major: u16,
    os_minor: u16,
    imgae_major: u16,
    image_minor: u16,
    subsys_major: u16,
    subsys_minor: u16,
    win32_ver: u32,
    image_size: u32,
    header_size: u32,
    checksum: u32,
    subsys: u16,
    dll_flags: u16,
    stack_reserve: u32, // w
    stack_commit: u32, // w
    heap_reserve: u32, // w
    heap_commit: u32, // w
    loader_flags: u32,
    datadir_entries: u32,
}

#[repr(C,packed)]
struct PeOptional64 {
    magic: u16,
    major: u8,
    minor: u8,
    text_size: u32,
    data_size: u32,
    bss_size: u32,
    entry_point: u32,
    text_base: u32,
    image_base: u64, // w
    section_align: u32,
    file_align: u32,
    os_major: u16,
    os_minor: u16,
    image_major: u16,
    image_minor: u16,
    subsys_major: u16,
    subsys_minor: u16,
    win32_ver: u32,
    image_size: u32,
    header_size: u32,
    checksum: u32,
    subsys: u16,
    dll_flags: u16,
    stack_reserve: u64, // w
    stack_commit: u64, // w
    heap_reserve: u64, // w
    heap_commit: u64, // w
    loader_flags: u32,
    datadir_entries: u32,
}

#[repr(C,packed)]
struct PeDataDirectory {
    rva: i32,
    size: u32,
}

#[repr(C,packed)]
struct PeSection {
    name: [u8; 8],
    virt_sz_or_phy_addr: u32,
    virt_address: u32,
    raw_sz: u32,
    raw_ptr: u32,
    reloc_ptr: u32,
    linenr_ptr: u32,
    reloc_count: u16,
    linenr_cout: u16,
    flags: u32,
}

pub fn pe(p: &Path) -> Option<Project> {
    let name = p.file_name().and_then(|x| x.to_str()).or(p.to_str()).unwrap_or("unknown pe");

    if let Some(mut fd) = File::open(p).ok() {
        // read MZ header
        const MZ_SIZE: usize = 64;
        assert_eq!(MZ_SIZE, mem::size_of::<Mz>());
        let mut mz_raw = [0; MZ_SIZE];

        if Some(MZ_SIZE) != fd.read(&mut mz_raw).ok() {
            return None;
        }

        let mz: Mz = unsafe { mem::transmute(mz_raw) };

        // check MZ signature
        if mz.signature != MZ_MAGIC {
            return None;
        }

        // read PE header
        const PE_SIZE: usize = 24;
        assert_eq!(PE_SIZE, mem::size_of::<Pe>());
        let mut pe_raw = [0; PE_SIZE];

        if Some(mz.e_lfanew as u64) != fd.seek(SeekFrom::Start(mz.e_lfanew as u64)).ok() {
            return None;
        }

        if Some(PE_SIZE) != fd.read(&mut pe_raw).ok() {
            return None;
        }

        let pe: Pe = unsafe { mem::transmute(pe_raw) };

        if pe.magic != 0x00004550 {
            return None;
        }

        match pe.machine {
            0x8664 => println!("AMD64"),
            0x014c => println!("IA32"),
            _ => return None
        }

        if pe.characteristics & 2 == 0 {
            return None;
        }

        // read optional PE header
        let mut opt_magic = [0; 2];

        if Some(2) != fd.read(&mut opt_magic).ok() {
            return None;
        }

        const PE32_SIZE: usize = 96;
        const PE64_SIZE: usize = 112;

        assert_eq!(PE32_SIZE, mem::size_of::<PeOptional32>());
        assert_eq!(PE64_SIZE, mem::size_of::<PeOptional64>());

        let (img_base,datadir_entries) = if opt_magic == [0xb,0x1] {
            let mut peopt_raw = [0; PE32_SIZE];

            if Some(PE32_SIZE) != fd.read(&mut peopt_raw).ok() {
                return None;
            }

            let peopt: PeOptional32 = unsafe { mem::transmute(peopt_raw) };
            (peopt.image_base as u64,peopt.datadir_entries)
        } else if opt_magic == [0xb,0x2] {
            let mut peopt_raw = [0; PE64_SIZE];

            if Some(PE64_SIZE) != fd.read(&mut peopt_raw).ok() {
                return None;
            }

            let peopt: PeOptional64 = unsafe { mem::transmute(peopt_raw) };
            (peopt.image_base,peopt.datadir_entries)
        } else {
            return None
        };

        // XXX: data directory

        // read sections
        const PESEC_SIZE: usize = 40;
        assert_eq!(PESEC_SIZE, mem::size_of::<PeSection>());
        let mut ram = Region::undefined("ram".to_string(),0xc0000000);

        for i in 0..pe.num_section {
            let sec_off = (mz.e_lfanew as u64) + (PE_SIZE as u64) + (PESEC_SIZE as u64) * (i as u64) + (pe.opthdr_size as u64);
            let mut sec_raw = [0; PESEC_SIZE];

            if Some(sec_off) != fd.seek(SeekFrom::Start(sec_off)).ok() {
                return None;
            }

            if Some(PESEC_SIZE) != fd.read(&mut sec_raw).ok() {
                return None;
            }

            let sec: PeSection = unsafe { mem::transmute(sec_raw) };
            let name = String::from_utf8_lossy(&sec.name);

            println!("{}",name);

            let l = if sec.raw_sz > 0 {
                let mut buf = vec![0; sec.raw_sz as usize];

                if Some(sec.raw_ptr as u64) != fd.seek(SeekFrom::Start(sec.raw_ptr as u64)).ok() {
                    return None;
                }

                if Some(sec.raw_sz as usize) != fd.read(&mut buf).ok() {
                    return None;
                }

                println!("mapped '{}'",name);
                Layer::wrap(buf.to_vec())
            } else {
                println!("not mapped '{}'",name);
                Layer::undefined(sec.virt_sz_or_phy_addr as u64)
            };

            if !ram.cover(Bound::new(img_base + (sec.virt_address as u64),img_base + (sec.virt_address as u64) + (sec.raw_sz as u64)),l) {
                return None;
            }
        }

        Some(Project::new(name.to_string(),ram))
    } else {
        None
    }
}
