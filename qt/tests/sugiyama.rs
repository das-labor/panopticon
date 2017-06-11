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

/*extern crate panopticon_amd64;
extern crate panopticon_core;
extern crate panopticon_analysis;
extern crate panopticon_graph_algos;
extern crate futures;
extern crate env_logger;

use std::path::Path;
use panopticon_core::{Machine, loader};
use panopticon_analysis::pipeline;
use panopticon_graph_algos::{
    GraphTrait,
    VertexListGraphTrait,
    EdgeListGraphTrait,
};
use panopticon_amd64 as amd64;
use panopticon::linear_layout;
use futures::Stream;
use std::collections::HashMap;

#[test]
    #[ignore]
    fn layout_all() {
        let _ = ::env_logger::init();

        for path in &[
            "tests/data/static",
            "tests/data/libbeef.dll",
            "tests/data/libbeef.dylib",
        ] {
            if let Ok((mut proj, machine)) = loader::load(&Path::new(path)) {
                let maybe_prog = proj.code.pop();
                let reg = proj.data
                    .dependencies
                    .vertex_label(proj.data.root)
                    .unwrap()
                    .clone();

                if let Some(prog) = maybe_prog {
                    let pipe: Box<_> = match machine {
                        Machine::Amd64 => {
                            pipeline::<amd64::Amd64>(
                                prog,
                                reg.clone(),
                                amd64::Mode::Long,
                            )
                        }
                        Machine::Ia32 => {
                            pipeline::<amd64::Amd64>(
                                prog,
                                reg.clone(),
                                amd64::Mode::Protected,
                            )
                        }
                        _ => unreachable!(),
                    };

                    for i in pipe.wait() {
                        if let Ok(func) = i {
                            let cfg = &func.cflow_graph;
                            let vertices =
                                HashMap::<AdjacencyListVertexDescriptor,
                                          usize>::from_iter(
                                    cfg.vertices().enumerate().map(
                                        |(idx,
                                          vx)| {
                                            (vx, idx)
                                        }
                                    )
                                );
                            let entry = func.entry_point.map(|x| vertices[&x]);
                            let edges: Vec<(usize,
                                            usize)> = cfg.edges()
                                .map(
                                    |e| {
                                        (vertices[&cfg.source(e)],
                                         vertices[&cfg.target(e)])
                                    }
                                )
                                .collect();
                            let dims = HashMap::from_iter(
                                vertices.iter().map(
                                    |(_, &x)| {
                                        (x, (42., 23.))
                                    }
                                )
                            );

                            linear_layout(
                                &vertices.into_iter().map(|(_, x)| x).collect(),
                                &edges,
                                &dims,
                                entry,
                                5.,
                                10.,
                                2.,
                                5.,
                                5.,
                                120.,
                            )
                                    .unwrap();
                        } else {
                            unreachable!();
                        }
                    }
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }
    }*/
