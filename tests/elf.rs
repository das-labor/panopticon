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

use panopticon::elf::*;
use std::path::Path;
use std::fs::File;

#[test]
fn elf_parse_self() {
    let mut fd = File::open(Path::new("target/debug/qtpanopticon")).ok().unwrap();

    match parse::Ehdr::read(&mut fd) {
        Ok(ehdr) => {
            println!("{:?}",ehdr);
            for p in ehdr.progam_headers.iter() {
                println!("{:?}",p);
            }
            for s in ehdr.segment_headers.iter() {
                println!("{:?}",s);
            }
        },
        Err(e) => { panic!(e) }
    }
}

#[test]
fn elf_load_static() {
    match load::load(Path::new("tests/data/static")) {
        Ok(proj) => println!("{}",proj.name),
        Err(_) => panic!()
    }
}
