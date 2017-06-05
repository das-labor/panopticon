/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017 Panopticon authors
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

//! Disassembly-Analysis loop.

use futures::sync::mpsc;
use futures::{stream,Stream,Sink,Future};
use std::thread;
use std::collections::{
    HashMap,
};
use std::iter::FromIterator;
use {
    Architecture,
    Program,
    Function,
    Region,
    CallTarget,
    Rvalue,
    ssa_convertion,
};
use graph_algos::{
    VertexListGraphTrait,
    GraphTrait,
};
use std::fmt::Debug;

/// Starts disassembling insructions in `region` and puts them into `program`. Returns a stream of
/// of newly discovered functions.
pub fn pipeline<A: Architecture + Debug + 'static >(program: Program, region: Region, config: A::Configuration) -> Box<Stream<Item=Function,Error=()> + Send>
where A::Configuration: Debug {
    let (tx,rx) = mpsc::channel::<Function>(10);
    thread::spawn(move || {
        let tx = tx;
        let mut functions = HashMap::<u64,Function>::new();
        let mut targets = HashMap::<u64,Function>::from_iter(program.call_graph.vertices().filter_map(|vx| {
            match program.call_graph.vertex_label(vx) {
                Some(&CallTarget::Todo(Rvalue::Constant{ value: entry,.. },ref maybe_name,ref uuid)) => {
                    let name = maybe_name.clone().unwrap_or_else(|| format!("func_0x{:x}",entry));
                    let f = Function::with_uuid(name,uuid.clone(),region.name().clone());
                    Some((entry,f))
                }
                Some(_) => None,
                None => unreachable!(),
            }
        }));

        while !targets.is_empty() {
            info!("disassemble {:?}",targets);
            let new_targets: Vec<Vec<(u64,Function)>> = targets.into_iter().map(|(entry,f)| {
                let tx = tx.clone();
                let mut f = Function::disassemble::<A>(Some(f),config.clone(),&region,entry);
                f.entry_point = f.find_basic_block_by_start(entry);
                let new_ct = f.collect_calls().into_iter().filter_map(|rv| {
                    if let Rvalue::Constant{ value,.. } = rv {
                        if !functions.contains_key(&value) && entry != value {
                            return Some((value,Function::new(format!("func_0x{:x}",value),region.name().clone())));
                        }
                    }
                    None
                }).collect::<Vec<(u64,Function)>>();

                let _ = ssa_convertion(&mut f);

                functions.insert(entry,f.clone());
                tx.send_all(stream::iter(vec![Ok(f)])).wait().unwrap().0;
                new_ct
            }).collect();
            targets = new_targets.into_iter().flat_map(|x| x).collect();
        }
    });

    Box::new(rx)
}
