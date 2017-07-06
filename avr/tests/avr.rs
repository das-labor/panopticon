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

extern crate panopticon_core;
extern crate panopticon_avr;
extern crate panopticon_graph_algos;
extern crate env_logger;

use panopticon_avr::{Avr, Mcu};
use panopticon_core::{ControlFlowTarget, Function, Region, loader};
use panopticon_graph_algos::{EdgeListGraphTrait, GraphTrait, VertexListGraphTrait};

use std::path::Path;

#[test]
fn avr_jmp_overflow() {
    let reg = Region::open(
        "flash".to_string(),
        Path::new("../test-data/avr-jmp-overflow.bin"),
    )
            .unwrap();

    let func = Function::new::<Avr>(0, &reg, None, Mcu::atmega88()).unwrap();

    assert_eq!(func.cfg().num_vertices(), 2);
    assert_eq!(func.cfg().num_edges(), 2);

    let mut vxs = func.cfg().vertices();
    if let Some(&ControlFlowTarget::Resolved(ref bb1)) = func.cfg().vertex_label(vxs.next().unwrap()) {
        if let Some(&ControlFlowTarget::Resolved(ref bb2)) = func.cfg().vertex_label(vxs.next().unwrap()) {
            assert!(bb1.area.start == 0 || bb1.area.start == 6000);
            assert!(bb2.area.start == 0 || bb2.area.start == 6000);
            assert!(bb1.area.end == 2 || bb1.area.end == 6004);
            assert!(bb2.area.end == 2 || bb2.area.end == 6004);
        }
    }
}

#[test]
fn avr_wrap_around() {
    let reg = Region::open(
        "flash".to_string(),
        Path::new("../test-data/avr-overflow.bin"),
    )
            .unwrap();
    let func = Function::new::<Avr>(0, &reg, None, Mcu::atmega88()).unwrap();

    assert_eq!(func.cfg().num_vertices(), 2);
    assert_eq!(func.cfg().num_edges(), 2);

    let mut vxs = func.cfg().vertices();
    if let Some(&ControlFlowTarget::Resolved(ref bb1)) = func.cfg().vertex_label(vxs.next().unwrap()) {
        if let Some(&ControlFlowTarget::Resolved(ref bb2)) = func.cfg().vertex_label(vxs.next().unwrap()) {
            println!("bb1: {:?}, bb2: {:?}", bb1.area, bb2.area);
            assert!(bb1.area.start == 0 || bb1.area.start == 8190);
            assert!(bb2.area.start == 0 || bb2.area.start == 8190);
            assert!(bb1.area.end == 2 || bb1.area.end == 8192);
            assert!(bb2.area.end == 2 || bb2.area.end == 8192);
        }
    }
}

#[test]
fn avr_elf() {
    let proj = loader::load(Path::new("../test-data/hello-world")).ok();
    assert!(proj.is_some());
}
