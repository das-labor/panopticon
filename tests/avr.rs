// Panopticon - A libre disassembler
// Copyright (C) 2015  Panopticon authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

extern crate panopticon;
extern crate graph_algos;

use panopticon::region::Region;
use panopticon::avr::{Mcu, Avr};
use panopticon::avr::syntax::disassembler;
use panopticon::function::{ControlFlowTarget, Function};
use panopticon::loader;

use std::path::Path;

use graph_algos::{VertexListGraphTrait, GraphTrait, EdgeListGraphTrait};

#[test]
fn avr_jmp_overflow() {
    let reg = Region::open("flash".to_string(),
                           Path::new("tests/data/avr-jmp-overflow.bin"))
        .unwrap();
    let fun = Function::disassemble::<Avr>(None, Mcu::atmega88(), &reg, 0);

    assert_eq!(fun.cflow_graph.num_vertices(), 2);
    assert_eq!(fun.cflow_graph.num_edges(), 2);

    let mut vxs = fun.cflow_graph.vertices();
    if let Some(&ControlFlowTarget::Resolved(ref bb1)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
        if let Some(&ControlFlowTarget::Resolved(ref bb2)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
            assert!(bb1.area.start == 0 || bb1.area.start == 6000);
            assert!(bb2.area.start == 0 || bb2.area.start == 6000);
            assert!(bb1.area.end == 2 || bb1.area.end == 6004);
            assert!(bb2.area.end == 2 || bb2.area.end == 6004);
        }
    }
}

#[test]
fn avr_wrap_around() {
    let reg = Region::open("flash".to_string(),
                           Path::new("tests/data/avr-overflow.bin"))
        .unwrap();
    let fun = Function::disassemble::<Avr>(None, Mcu::atmega88(), &reg, 0);

    assert_eq!(fun.cflow_graph.num_vertices(), 2);
    assert_eq!(fun.cflow_graph.num_edges(), 2);

    let mut vxs = fun.cflow_graph.vertices();
    if let Some(&ControlFlowTarget::Resolved(ref bb1)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
        if let Some(&ControlFlowTarget::Resolved(ref bb2)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
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
    let proj = loader::load(Path::new("tests/data/hello-world")).ok();
    assert!(proj.is_some());
}
