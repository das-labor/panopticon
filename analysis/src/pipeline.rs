/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

use futures::{Future, Sink, Stream, stream};
use futures::sync::mpsc;
use panopticon_core::{Architecture, CallTarget, Function, Program, Region, Rvalue};
use panopticon_data_flow::ssa_convertion;
use panopticon_graph_algos::{GraphTrait, VertexListGraphTrait};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::iter::FromIterator;
use std::thread;
use std::sync::Arc;

/// Starts disassembling insructions in `region` and puts them into `program`. Returns a stream of
/// of newly discovered functions.
pub fn pipeline<A: Architecture + Debug + 'static>(
    program: Arc<Program>,
    region: Region,
    config: A::Configuration,
) -> Box<Stream<Item = Function, Error = ()> + Send>
where
    A::Configuration: Debug,
{
    let (tx, rx) = mpsc::channel::<Function>(10);
    thread::spawn(
        move || {
            let tx = tx;
            let mut functions = HashSet::<u64>::new();
            let mut targets = HashMap::<u64, Function>::from_iter(
                program
                    .call_graph
                    .vertices()
                    .filter_map(
                        |vx| match program.call_graph.vertex_label(vx) {
                            Some(&CallTarget::Todo(Rvalue::Constant { value: entry, .. }, ref maybe_name, ref uuid)) => {
                                let f = Function::with_uuid(entry, uuid.clone(), &region, maybe_name.clone());
                                functions.insert(entry);
                                Some((entry, f))
                            }
                            Some(_) => None,
                            None => unreachable!(),
                        }
                    )
            );

            while !targets.is_empty() {
                info!("disassemble({}) {:?}", targets.len(), &targets);
                let new_targets: Vec<Vec<(u64, Function)>> = targets
                    .into_iter()
                    .map(
                        |(entry, mut f)| {
                            debug!("entry {:#x} = f.start {:#x}", entry, f.start);
                            let tx = tx.clone();
                            f.disassemble::<A>(config.clone(), &region);
                            let new_ct = f.collect_calls()
                                .into_iter()
                                .filter_map(
                                    |rv| {
                                        if let Rvalue::Constant { value, .. } = rv {
                                            info!("checking if {} is in {:?}", value, &functions);
                                            if !functions.contains(&value) && entry != value {
                                                functions.insert(value);
                                                info!("adding {:#x} - func_0x{:x}", value, value);
                                                return Some((value, Function::new(value, &region, None)));
                                            }
                                        }
                                        None
                                    }
                                )
                                .collect::<Vec<(u64, Function)>>();

                            let _ = ssa_convertion(&mut f);
                            tx.send_all(stream::iter(vec![Ok(f)])).wait().unwrap().0;
                            new_ct
                        }
                    )
                    .collect();
                targets = new_targets.into_iter().flat_map(|x| x).collect();
            }
        }
    );

    Box::new(rx)
}
