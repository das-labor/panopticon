/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015, 2017  Panopticon authors
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
extern crate panopticon_amd64;
extern crate panopticon_graph_algos;
extern crate panopticon_data_flow;

extern crate env_logger;
#[macro_use] extern crate log;

use panopticon_amd64 as amd64;
use panopticon_core::{Architecture, Region};
use panopticon_core::neo;
use panopticon_data_flow::neo::rewrite_to_ssa;
use panopticon_data_flow::ssa_convertion;
use std::path::Path;

#[test]
fn amd64_opcodes() {
    let reg = Region::open("com".to_string(), Path::new("../test-data/amd64.com")).unwrap();
    let mut addr = 0;

    loop {
        let maybe_match = <amd64::Amd64 as Architecture>::decode(&reg, addr, &amd64::Mode::Long);

        if let Ok(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}", mne.area.start, mne.opcode);
                addr = mne.area.end;

                if addr >= reg.size() {
                    return;
                }
            }
        } else if addr < reg.size() {
            unreachable!("failed to match anything at {:x}", addr);
        } else {
            break;
        }
    }
}

#[test]
fn ia32_opcodes() {
    env_logger::init().unwrap();

    let reg = Region::open("com".to_string(), Path::new("../test-data/ia32.com")).unwrap();
    let mut addr = 0;

    loop {
        let maybe_match = amd64::Amd64::decode(&reg, addr, &amd64::Mode::Protected);

        if let Ok(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}", mne.area.start, mne.opcode);
                addr = mne.area.end;

                if addr >= reg.size() {
                    return;
                }
            }
        } else if addr < reg.size() {
            unreachable!("failed to match anything at {:x}", addr);
        } else {
            break;
        }
    }
}
#[test]
fn disassemble_static_new() {
    use panopticon_core::{loader,neo,CallTarget,Rvalue};
    use panopticon_graph_algos::{VertexListGraphTrait,GraphTrait};
    use std::path::Path;

    let _ = env_logger::init();
    let (proj,_) = loader::load(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].call_graph.vertices().filter_map(|vx| if let Some(&CallTarget::Todo(Rvalue::Constant{ value,.. },_,_)) = proj.code[0].call_graph.vertex_label(vx) { Some(value) } else { None }).collect::<Vec<_>>();
    let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        println!("start {:#x}",ep);
        let mut func = neo::Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
        println!("convert {} to ssa",func.name);
        rewrite_to_ssa(&mut func).unwrap();
        funcs.push(func);
    }
}

#[test]
fn disassemble_static_old() {
    use panopticon_core::{loader,neo,CallTarget,Rvalue,Function};
    use panopticon_graph_algos::{VertexListGraphTrait,GraphTrait};
    use std::path::Path;

    let _ = env_logger::init();
    let (proj,_) = loader::load(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].call_graph.vertices().filter_map(|vx| if let Some(&CallTarget::Todo(Rvalue::Constant{ value,.. },_,_)) = proj.code[0].call_graph.vertex_label(vx) { Some(value) } else { None }).collect::<Vec<_>>();
    let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        println!("start {:#x}",ep);
        let mut func = Function::new::<amd64::Amd64>(ep,&reg,Some("".to_string()),amd64::Mode::Long).unwrap();
        println!("convert {} to ssa",func.name);
        ssa_convertion(&mut func).unwrap();
        funcs.push(func);
    }
}
