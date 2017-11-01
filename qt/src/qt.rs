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

use errors::*;
use futures::{Future, future};
use futures_cpupool::CpuPool;
use panopticon_glue as glue;
use panopticon_glue::{CBasicBlockLine, CBasicBlockOperand, Glue};
use parking_lot::Mutex;
use singleton::{EdgePosition, NodePosition, PANOPTICON};
use std::collections::HashSet;
use std::ffi::CString;
use uuid::Uuid;

lazy_static! {
    pub static ref LAYOUT_TASK: Mutex<Box<future::Future<Item=(),Error=Error> + Send + 'static>> = {
        Mutex::new(Box::new(future::ok(())))
    };

    pub static ref THREAD_POOL: Mutex<CpuPool> = {
        Mutex::new(CpuPool::new_num_cpus())
    };

    pub static ref SUBSCRIBED_FUNCTIONS: Mutex<HashSet<Uuid>> = {
        Mutex::new(HashSet::new())
    };
}

fn transform_nodes(only_entry: bool, nodes: Vec<NodePosition>) -> Vec<(usize, f32, f32, bool, Vec<CBasicBlockLine>)> {
    nodes
        .into_iter()
        .filter_map(|x| if !only_entry || x.3 { Some(x) } else { None })
        .map(
            |(id, x, y, is_entry, blk)| {
                let blk = blk.into_iter()
                    .filter_map(
                        |bbl| {
                            let args = bbl.args
                                .into_iter()
                                .filter_map(|x| CBasicBlockOperand::new(x.kind.to_string(), x.display, x.alt, x.data).ok())
                                .collect::<Vec<_>>();
                            CBasicBlockLine::new(bbl.opcode, bbl.region, bbl.offset, bbl.comment, args).ok()
                        }
                    )
                    .collect::<Vec<_>>();
                (id, x, y, is_entry, blk)
            }
        )
        .collect()
}

fn transform_edges(edges: Vec<EdgePosition>) -> (Vec<u32>, Vec<CString>, Vec<CString>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, CString) {
    use std::f32;

    let edges = edges
        .into_iter()
        .map(
            |(id, kind, label, (head_x, head_y), (tail_x, tail_y), segs)| {
                let segs = segs.iter();
                let f = |&(x, y, _, _)| (x, y);
                let g = |&(_, _, x, y)| (x, y);
                let mut min_x = f32::INFINITY;
                let mut max_x = f32::NEG_INFINITY;
                let mut min_y = f32::INFINITY;
                let mut max_y = f32::NEG_INFINITY;
                let svg = if let Some(&(x, y, _, _)) = segs.clone().next() {
                    let mut edge = format!("M {} {}", x, y);

                    if min_x > x {
                        min_x = x
                    }
                    if max_x < x + 1. {
                        max_x = x + 1.
                    }
                    if min_y > y {
                        min_y = y
                    }
                    if max_y < y + 1. {
                        max_y = y + 1.
                    }

                    for (x, y) in segs.clone().take(1).map(&f).chain(segs.clone().map(&g)) {
                        edge = format!("{} L {} {}", edge, x, y);
                        if min_x > x {
                            min_x = x
                        }
                        if max_x < x + 1. {
                            max_x = x + 1.
                        }
                        if min_y > y {
                            min_y = y
                        }
                        if max_y < y + 1. {
                            max_y = y + 1.
                        }
                    }

                    let color = if kind == "fallthru" || kind == "fallthru-backedge" {
                        "red"
                    } else if kind == "branch" || kind == "branch-backedge" {
                        "green"
                    } else {
                        "black"
                    };

                    let arrow = if let Some(&(_, _, x, y)) = segs.clone().rev().next() {
                        let width = 12.;
                        let height = 8.;

                        format!(
                            "M {} {} L {} {} L {} {} L {} {} Z",
                            x,
                            y,
                            x - width / 2.,
                            y - height / 2.,
                            x,
                            y + height,
                            x + width / 2.,
                            y - height / 2.
                        )
                    } else {
                        "".to_string()
                    };
                    format!(
                        "
    <path style='fill:none; stroke:{}; stroke-width:2' d='{}'/>
    <path style='fill:{}; stroke-width:0' d='{}'/>\n",
                        color,
                        edge,
                        color,
                        arrow
                    )
                } else {
                    "".to_string()
                };

                let label = CString::new(label.as_bytes()).unwrap();
                let kind = CString::new(kind.as_bytes()).unwrap();

                (id as u32, label, kind, head_x, head_y, tail_x, tail_y, svg, min_x, max_x, min_y, max_y)
            }
        );

    let mut head_xs = vec![];
    let mut head_ys = vec![];
    let mut tail_xs = vec![];
    let mut tail_ys = vec![];
    let mut ids = vec![];
    let mut labels = vec![];
    let mut kinds = vec![];
    let mut svg = "".to_string();
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for (id, label, kind, head_x, head_y, tail_x, tail_y, path, min_x_, max_x_, min_y_, max_y_) in edges {
        head_xs.push(head_x);
        head_ys.push(head_y);
        tail_xs.push(tail_x);
        tail_ys.push(tail_y);
        ids.push(id);
        labels.push(label);
        kinds.push(kind);
        svg += &path;

        if min_x > min_x_ {
            min_x = min_x_
        }
        if max_x < max_x_ {
            max_x = max_x_
        }
        if min_y > min_y_ {
            min_y = min_y_
        }
        if max_y < max_y_ {
            max_y = max_y_
        }
    }

    svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 {} {}'>\n{}</svg>",
        max_x + 10.,
        max_y + 10.,
        max_x + 10.,
        max_y + 10.,
        svg
    );

    (ids, labels, kinds, head_xs, head_ys, tail_xs, tail_ys, CString::new(svg.as_bytes()).unwrap())
}

fn transform_and_send_function(uuid: &Uuid, only_entry: bool, do_nodes: bool, do_edges: bool) -> Box<future::Future<Item=(), Error=Error> + Send + 'static> {
    let uuid = uuid.clone();
    let ret = PANOPTICON.lock().layout_function_async(&uuid);

    Box::new(ret.and_then(
        move |(nodes, edges)| {
            let uuid = uuid;
            let uuid = CString::new(uuid.clone().to_string().as_bytes()).unwrap();

            if do_nodes {
                let nodes = transform_nodes(only_entry, nodes);

                for (id, x, y, is_entry, bbl) in nodes {
                    Qt::send_function_node(uuid.clone(), id, x, y, is_entry, bbl.as_slice()).unwrap();
                }
            }

            if do_edges {
                let (ids, labels, kinds, head_xs, head_ys, tail_xs, tail_ys, svg) = transform_edges(edges);
                Qt::send_function_edges(
                    uuid,
                    ids.as_slice(),
                    labels.as_slice(),
                    kinds.as_slice(),
                    head_xs.as_slice(),
                    head_ys.as_slice(),
                    tail_xs.as_slice(),
                    tail_ys.as_slice(),
                    svg,
                    )
                    .unwrap();
            }

            future::ok(())
        }))
}

pub struct Qt;

impl Glue for Qt {
    fn get_function(uuid: &Uuid, only_entry: bool, do_nodes: bool, do_edges: bool) -> glue::Result<()> {
        Self::send_layout_task(&CString::new(uuid.to_string()).unwrap()).unwrap();

        let task = transform_and_send_function(&uuid, only_entry, do_nodes, do_edges).then(
            |x| {
                let uuid = CString::new("".to_string().as_bytes()).unwrap();
                Self::send_layout_task(&uuid).unwrap();

                future::result(x)
            }
        );
        let task = {
            THREAD_POOL.lock().spawn(task)
        };
        *LAYOUT_TASK.lock() = Box::new(task);

        Ok(())
    }

    fn subscribe_to(uuid: &Uuid, state: bool) -> glue::Result<()> {
        if !state {
            SUBSCRIBED_FUNCTIONS.lock().remove(uuid);
        } else {
            let subs = &mut SUBSCRIBED_FUNCTIONS.lock();

            subs.insert(uuid.clone());
        }

        Ok(())
    }

    fn open_program(path: &str) -> glue::Result<()> {
        PANOPTICON.lock().open_program(path.to_string()).map_err(|e| format!("{}", e).into())
    }

    fn save_session(path: &str) -> glue::Result<()> {
        PANOPTICON.lock().save_session(path.to_string()).map_err(|e| format!("{}", e).into())
    }

    fn comment_on(address: u64, comment: &str) -> glue::Result<()> {
        PANOPTICON.lock().comment_on(address, comment.to_string()).map_err(|e| format!("{}", e).into())
    }

    fn rename_function(uuid: &Uuid, name: &str) -> glue::Result<()> {
        PANOPTICON.lock().rename_function(uuid.to_string(), name.to_string()).map_err(|e| format!("{}", e).into())
    }

    fn set_value_for(uuid: &Uuid, variable: &str, value: &str) -> glue::Result<()> {
        PANOPTICON.lock().set_value_for(uuid.to_string(), variable.to_string(), value.to_string()).map_err(|e| format!("{}", e).into())
    }

    fn undo() -> glue::Result<()> {
        PANOPTICON.lock().undo().map_err(|e| format!("{}", e).into())
    }

    fn redo() -> glue::Result<()> {
        PANOPTICON.lock().redo().map_err(|e| format!("{}", e).into())
    }
}
