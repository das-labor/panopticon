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

extern crate panopticon;

use panopticon::loader;
use std::path::Path;

#[test]
fn elf_load_static() {
    match loader::load(Path::new("tests/data/static")) {
        Ok((proj, _)) => {
            println!("{:?}", proj);
            assert_eq!(proj.imports.len(), 0);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}

#[test]
fn elf_load_dynamic() {
    match loader::load(Path::new("tests/data/libfoo.so")) {
        Ok((proj, _)) => {
            println!("{:?}", &proj);
            assert_eq!(proj.name, "libfoo.so");
            assert_eq!(proj.code.len(), 1);
            assert_eq!(proj.imports.len(), 6);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}

#[test]
fn mach_load_lib() {
    match loader::load(Path::new("tests/data/libbeef.dylib")) {
        Ok((proj, _)) => {
            println!("{:?}", &proj);
            assert_eq!(proj.imports.len(), 0);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}

#[test]
fn mach_load_exe() {
    match loader::load(Path::new("tests/data/deadbeef.mach")) {
        Ok((proj, _)) => {
            println!("{:?}", &proj);
            assert_eq!(proj.imports.len(), 2);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}

#[test]
fn mach_load_bytes() {
    use std::fs::File;
    use std::io::Read;
    let bytes = {
        let mut v = Vec::new();
        let mut fd = File::open(Path::new("tests/data/deadbeef.mach")).unwrap();
        fd.read_to_end(&mut v).unwrap();
        v
    };
    match loader::load_mach(&bytes, 0, "tests/data/deadbeef.mach".to_owned()) {
        Ok((proj, _)) => {
            println!("{}", proj.name);
            assert_eq!(proj.imports.len(), 2);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}

// TODO: add imports into the PE loader

#[test]
fn load_pe32() {
    let project = loader::load(Path::new("tests/data/test.exe"));
    match project {
        Ok((proj, _)) => {
            println!("{:?}", proj);
            assert_eq!(proj.imports.len(), 0);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}

#[test]
fn load_pe32_dll() {
    let project = loader::load(Path::new("tests/data/libbeef.dll"));
    match project {
        Ok((proj, _)) => {
            println!("{:?}", proj);
            assert_eq!(proj.imports.len(), 0);
        },
        Err(error) => {
            println!("{:?}", error);
            assert!(false);
        }
    }
}
