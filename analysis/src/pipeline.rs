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
use panopticon_core::{Architecture, CallTarget, Error, Fun, Function, Program, Result, Region, Rvalue};
use panopticon_data_flow::ssa_convertion;
use std::collections::HashSet;
use std::fmt::Debug;
use std::thread;
use std::sync::Arc;
use uuid::Uuid;
use std::result;
use parking_lot::{Mutex, RwLock};

pub fn analyze<A: Architecture + Debug + Sync + 'static, Function: Fun>(
    program: Program<Function>,
    region: Region,
    config: A::Configuration,
) -> Result<Program<Function>>
where
    A::Configuration: Debug + Sync,
    Function: Send,
{
    use rayon::prelude::*;
    use chashmap::CHashMap;

    struct Init {
        name: Option<String>,
        entry: u64,
        uuid: Uuid,
    }

    let attempts = CHashMap::<u64, result::Result<(), Error>>::new();
    let targets = CHashMap::<u64, bool>::new();
    let failures = RwLock::new(0);
    info!("initializing first wave");
    let functions =
        program
        .call_graph
        .into_iter()
        .filter_map(
            |ct| match ct {
                &CallTarget::Todo(Rvalue::Constant { value: entry, .. }, ref name, ref uuid) => {
                    Some(Init { entry, name: name.clone(), uuid: *uuid })
                }
                _ => None,
            }
        ).collect::<Vec<Init>>();

    // we now lock the program
    let program = Mutex::new(program);

    info!("begin first wave {}", functions.len());
    functions.into_par_iter().for_each(| Init { entry, name, uuid }| {
        let name = &name;
        attempts.upsert(entry,
                        || {
                            match Function::with_uuid::<A>(entry, &uuid, &region, name.clone(), config.clone()) {
                                Ok(mut f) => {
                                    for address in f.collect_call_addresses() {
                                        targets.upsert(address, || { true }, |_| ());
                                    }
                                    let _ = ssa_convertion(&mut f);
                                    {
                                        let mut program = program.lock();
                                        let _ = program.insert(f);
                                    }
                                    Ok(())
                                },
                                Err(e) => { *failures.write() += 1; Err(e) },
                            }
                        },
                        |f2| {
                            match f2 {
                                &mut Ok(_) => {
                                    let name = name.clone().unwrap_or(format!("func_{:#x}", entry));
                                    let mut program = program.lock();
                                    let f2 = program.find_function_mut(|f| f.start() == entry).unwrap();
                                    info!("New alias ({}) found at {:#x} with canonical name {:?}", &name, entry, &f2.name());
                                    f2.add_alias(name);
                                },
                                _ => ()
                            }
                        });
    });

    info!("first wave done: success: {} failures: {} targets: {}", attempts.len(), *failures.read(), targets.len());

    let mut targets = targets.into_iter().map(|(x, _)| x).collect::<Vec<u64>>();
    while !targets.is_empty() {
        info!("targets - ({})", targets.len());
        let new_targets = CHashMap::<u64, bool>::new();
        targets.into_par_iter().for_each(| address | {
            attempts.upsert(address, || {
                match Function::new::<A>(address, &region, None, config.clone()) {
                    Ok(mut f) => {
                        for address in f.collect_call_addresses() {
                            new_targets.upsert(address, || { true }, |_| ());
                        }
                        let _ = ssa_convertion(&mut f);
                        {
                            let mut program = program.lock();
                            let _ = program.insert(f);
                        }
                        Ok(())
                    },
                    Err(e) => { let mut failures = failures.write(); *failures += 1; Err(e) }
                }
            },
            |_| ());
        });
        targets = new_targets.into_iter().map(|(x, _)| x).collect::<Vec<u64>>();
    }

    let mut program = program.into_inner();
    info!("Finished analysis: {} failures {}", attempts.len(), *failures.read());
    program.update_plt();
    Ok(program)
}

/// Starts disassembling insructions in `region` and puts them into `program`. Returns a stream of
/// of newly discovered functions.
pub fn pipeline<A: Architecture + Debug + 'static>(
    program: Arc<Program<Function>>,
    region: Region,
    config: A::Configuration,
) -> Box<Stream<Item = Function, Error = ()> + Send>
where
    A::Configuration: Debug,
{
    let (tx, rx) = mpsc::channel::<Function>(10);
    thread::spawn(
        move || {
            let mut finished_functions = HashSet::<u64>::new();
            let mut targets: Vec<u64> = Vec::new();
            let mut failures: Vec<(u64, Error)> = Vec::new();
            // TODO: this is the exact code below, modulo how we construct the function
            for ct in program.call_graph.into_iter() {
                match ct {
                    &CallTarget::Todo(Rvalue::Constant { value: entry, .. }, ref maybe_name, ref uuid) => {
                        finished_functions.insert(entry);
                        match Function::with_uuid::<A>(entry, uuid, &region, maybe_name.clone(), config.clone()) {
                            Ok(mut f) => {
                                let addresses = f.collect_call_addresses();
                                targets.extend_from_slice(&addresses);
                                let _ = ssa_convertion(&mut f);
                                let tx = tx.clone();
                                tx.send_all(stream::iter(vec![Ok(f)])).wait().unwrap().0;
                            },
                            Err(e) => { failures.push((entry, e)); },
                        }
                    }
                    _ => (),
                }
            }

            while !targets.is_empty() {
                info!("disassemble({}) {:?}", targets.len(), &targets);
                let mut new_targets = Vec::new();
                for address in targets.drain(..) {
                    info!("checking if {} is in {:?}", address, &finished_functions);
                    if !finished_functions.contains(&address) {
                        finished_functions.insert(address);
                        info!("adding func_0x{:x}", address);
                        match Function::new::<A>(address, &region, None, config.clone()) {
                            Ok(mut f) => {
                                let addresses = f.collect_call_addresses();
                                new_targets.extend_from_slice(&addresses);
                                let _ = ssa_convertion(&mut f);
                                {
                                    let tx = tx.clone();
                                    tx.send_all(stream::iter(vec![Ok(f)])).wait().unwrap().0;
                                }
                            },
                            Err(e) => failures.push((address, e)),
                        }
                    }
                }
                targets = new_targets;
            }
        }
    );

    Box::new(rx)
}
