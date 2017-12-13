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

// Qt uses camelCase
#![allow(non_snake_case)]

use errors::*;
use futures::{Future, future};
use panopticon_abstract_interp::Kset;
use panopticon_core::{ControlFlowTarget, Function, Guard, Mnemonic, Rvalue};
use panopticon_graph_algos::{EdgeListGraphTrait, GraphTrait, IncidenceGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::adjacency_list::{AdjacencyListEdgeDescriptor, AdjacencyListVertexDescriptor};
use singleton::{AbstractInterpretation, VarName};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use sugiyama;
use uuid::Uuid;

#[derive(Clone)]
pub struct ControlFlowLayout {
    pub node_dimensions: HashMap<AdjacencyListVertexDescriptor, (f32, f32)>,
    pub node_positions: HashMap<AdjacencyListVertexDescriptor, (f32, f32)>,
    pub node_data: HashMap<AdjacencyListVertexDescriptor, (bool, Vec<BasicBlockLine>)>,
    pub edges: HashMap<AdjacencyListEdgeDescriptor, (Vec<(f32, f32, f32, f32)>, (f32, f32), (f32, f32))>,
    pub edge_data: HashMap<AdjacencyListEdgeDescriptor, (&'static str, String)>,
}

#[derive(Clone)]
pub struct BasicBlockLine {
    pub opcode: String,
    pub region: String,
    pub offset: u64,
    pub comment: String,
    pub args: Vec<BasicBlockOperand>,
}

#[derive(Clone)]
pub struct BasicBlockOperand {
    pub kind: &'static str, // constant, variable, function, literal
    pub display: String, // string to display
    pub alt: String, // alternative display string
    pub data: String, // constant: value, variable: ssa var, function: UUID, literal: empty string
}

impl ControlFlowLayout {
    pub fn new_async(
        func: &Function,
        comments: &HashMap<u64, String>,
        values: Option<&AbstractInterpretation>,
        functions: &HashMap<Uuid, Function>,
        char_width: usize,
        padding: usize,
        margin: usize,
        col_padding: usize,
        line_height: usize,
        cmnt_width: usize,
    ) -> Box<future::Future<Item=ControlFlowLayout, Error=Error> + Send + 'static> {
        use std::f32;

        let (vertices, edges, edges_rev) = Self::flatten_cflow_graph(func);

        if vertices.is_empty() {
            println!("{} is empty", func.uuid());
            return Box::new(
                future::ok(
                    ControlFlowLayout {
                        node_data: HashMap::new(),
                        node_positions: HashMap::new(),
                        node_dimensions: HashMap::new(),
                        edges: HashMap::new(),
                        edge_data: HashMap::new(),
                    }
                )
            );
        }

        let data = HashMap::from_iter(
            func.cfg()
                .vertices()
                .filter_map(|vx| func.cfg().vertex_label(vx).map(|lb| (vx, lb)))
                .filter_map(
                    |(vx, lb)| {
                        let maybe_lines = Self::get_node_data(lb, comments, values, functions).ok();
                        let is_entry = func.entry_point_ref() == vx;

                        maybe_lines.map(|v| (vx, (is_entry, v)))
                    }
                )
        );
        let labels = HashMap::from_iter(func.cfg().edges().filter_map(|e| Self::get_edge_data(e, func).ok().map(|x| (e, x))));
        let dims = Self::compute_node_dimensions(
            func,
            char_width,
            padding,
            margin,
            col_padding,
            line_height,
            cmnt_width,
        );
        if dims.is_err() {
            return Box::new(future::err(dims.err().unwrap()));
        }
        let dims = dims.unwrap();
        let vx_vec = vertices.iter().map(|&vx| vx).collect::<Vec<_>>();
        let edges2 = edges.clone();

        Box::new(future::lazy(
            move || -> Result<_> {
                let vx_vec = vx_vec;
                let edges = edges2;
                Ok(sugiyama::linear_layout_start(&vx_vec, &edges, None)?)
            }
        )
                .and_then(move |layout| future::result(sugiyama::linear_layout_rank(layout)))
                .and_then(move |layout| future::result(sugiyama::linear_layout_initial_order(layout)))
                .and_then(
                    move |layout| {
                        future::loop_fn(
                            layout,
                            |layout| if let &sugiyama::LinearLayout::Ordering { iterations_left: 0, .. } = &layout {
                                Ok(future::Loop::Break(layout))
                            } else {
                                sugiyama::linear_layout_order(layout).map(|x| future::Loop::Continue(x))
                            },
                        )
                    }
                )
                .and_then(
                    move |layout| {
                        let vertices = vertices;
                        let edges = edges;
                        let edges_rev = edges_rev;
                        let dims = dims;
                        let data = data;

                        future::lazy(
                            move || {
                                let mut placement = sugiyama::linear_layout_placement(
                                    &vertices.iter().map(|&vx| vx).collect::<Vec<_>>(),
                                    &edges,
                                    &layout,
                                    &dims,
                                    cmnt_width as f32 + 20.,
                                    20.,
                                    50.,
                                    30.,
                                    30.,
                                    8.,
                                )?;
                                let mut positions = HashMap::from_iter(placement.0.into_iter().map(|(idx, pos)| (AdjacencyListVertexDescriptor(idx), pos)));
                                let initial = (f32::INFINITY, f32::INFINITY);
                                let (min_x, min_y) = positions
                                    .iter()
                                    .fold(
                                        initial, |(min_x, min_y), (_, &(mut x, mut y))| {
                                            x -= 10.;
                                            y -= 10.;
                                            let min_x = if min_x > x { x } else { min_x };
                                            let min_y = if min_y > y { y } else { min_y };

                                            (min_x, min_y)
                                        }
                                    );
                                let (min_x, min_y) = placement
                                    .1
                                    .iter()
                                    .fold(
                                        (min_x, min_y), |(min_x, min_y), (_, &(ref trail, _, _))| {
                                            let (x, y) = trail
                                                .iter()
                                                .fold(
                                                    (min_x, min_y),
                                                    |(min_x, min_y), &(mut from_x, mut from_y, mut to_x, mut to_y)| {
                                                        from_x -= 10.;
                                                        from_y -= 10.;
                                                        to_x -= 10.;
                                                        to_y -= 10.;

                                                        let min_x = if min_x > from_x { from_x } else { min_x };
                                                        let min_x = if min_x > to_x { to_x } else { min_x };
                                                        let min_y = if min_y > from_y { from_y } else { min_y };
                                                        let min_y = if min_y > to_y { to_y } else { min_y };

                                                        (min_x, min_y)
                                                    },
                                                );

                                            let min_x = if min_x > x { x } else { min_x };
                                            let min_y = if min_y > y { y } else { min_y };

                                            (min_x, min_y)
                                        }
                                    );

                                for (_, &mut (ref mut x, ref mut y)) in positions.iter_mut() {
                                    *x -= min_x;
                                    *y -= min_y;
                                }

                                for (_, &mut (ref mut trail, (ref mut start_x, ref mut start_y), (ref mut end_x, ref mut end_y))) in placement.1.iter_mut() {
                                    *end_x -= min_x;
                                    *end_y -= min_y;
                                    *start_x -= min_x;
                                    *start_y -= min_y;
                                    for &mut (ref mut x1, ref mut y1, ref mut x2, ref mut y2) in trail.iter_mut() {
                                        *x1 -= min_x;
                                        *y1 -= min_y;
                                        *x2 -= min_x;
                                        *y2 -= min_y;
                                    }
                                }

                                Ok(
                                    ControlFlowLayout {
                                        node_data: data,
                                        node_positions: positions,
                                        node_dimensions: HashMap::from_iter(dims.into_iter().map(|(idx, wh)| (AdjacencyListVertexDescriptor(idx), wh))),
                                        edges: HashMap::from_iter(placement.1.into_iter().map(|(idx, e)| (edges_rev[&idx], e))),
                                        edge_data: labels,
                                    }
                                )
                            }
                        )
                    }
                ))
    }

    pub fn new(
        func: &Function,
        comments: &HashMap<u64, String>,
        values: Option<&AbstractInterpretation>,
        functions: &HashMap<Uuid, Function>,
        char_width: usize,
        padding: usize,
        margin: usize,
        col_padding: usize,
        line_height: usize,
        cmnt_width: usize,
    ) -> Result<ControlFlowLayout> {
        Self::new_async(
            func,
            comments,
            values,
            functions,
            char_width,
            padding,
            margin,
            col_padding,
            line_height,
            cmnt_width,
        )
                .wait()
    }

    pub fn get_all_nodes(&self) -> Vec<(usize, f32, f32, bool, Vec<BasicBlockLine>)> {
        self.node_data
            .iter()
            .map(
                |(vx, data)| {
                    let pos = self.node_positions.get(vx).unwrap();

                    (vx.0, pos.0, pos.1, data.0, data.1.clone())
                }
            )
            .collect()
    }

    pub fn get_all_edges(&self) -> Vec<(usize, &'static str, String, (f32, f32), (f32, f32), Vec<(f32, f32, f32, f32)>)> {
        self.edges
            .iter()
            .map(
                |(k, &(ref segs, ref head, ref tail))| {
                    let &(kind, ref label) = self.edge_data.get(k).unwrap();
                    (k.0, kind, label.clone(), head.clone(), tail.clone(), segs.clone())
                }
            )
            .collect()
    }

    pub fn update_nodes(
        &mut self,
        addresses: Option<&[u64]>,
        func: &Function,
        comments: &HashMap<u64, String>,
        values: Option<&AbstractInterpretation>,
        functions: &HashMap<Uuid, Function>,
    ) -> Result<Vec<i32>> {
        let mut ret = vec![];

        for (&vx, &mut (_, ref mut lines)) in self.node_data.iter_mut() {
            let hit = if let Some(ref addrs) = addresses {
                lines.iter().any(|line| addrs.iter().find(|&&x| x == line.offset).is_some())
            } else {
                false
            };

            if hit {
                let cfg = &func.cfg();
                let lb = cfg.vertex_label(vx).ok_or(::panopticon_core::Error("missing label in cfg".into()))?;
                *lines = Self::get_node_data(lb, comments, values, functions)?;
                ret.push(vx.0 as i32);
            }
        }

        Ok(ret)
    }

    fn flatten_cflow_graph(func: &Function) -> (HashSet<usize>, Vec<(usize, usize)>, HashMap<usize, AdjacencyListEdgeDescriptor>) {
        let mut edges = vec![];
        let cfg = &func.cfg();
        let vertices = HashSet::from_iter(cfg.vertices().map(|x| x.0));
        let edge_iter = cfg.edges().map(|e| (cfg.source(e).0, cfg.target(e).0));

        for (from_idx, to_idx) in edge_iter {
            edges.push((from_idx, to_idx));
        }

        let edges_rev = HashMap::<usize, AdjacencyListEdgeDescriptor>::from_iter(cfg.edges().enumerate());

        (vertices, edges, edges_rev)
    }

    fn compute_node_dimensions(
        func: &Function,
        char_width: usize,
        padding: usize,
        margin: usize,
        col_padding: usize,
        line_height: usize,
        _cmnt_width: usize,
    ) -> Result<HashMap<usize, (f32, f32)>> {
        let mut dims = HashMap::new();
        let cfg = &func.cfg();

        for vx in cfg.vertices() {
            let maybe_lb = cfg.vertex_label(vx);

            match maybe_lb {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    let linew = bb.mnemonics
                        .iter()
                        .filter_map(
                            |mne| if mne.opcode.starts_with("__") {
                                None
                            } else {
                                Some(mne.opcode.len() + mne.operands.iter().map(|a| format!("{}", a).len()).sum::<usize>())
                            }
                        )
                        .max()
                        .unwrap_or(0);
                    let line_count = bb.mnemonics
                        .iter()
                        .filter_map(
                            |mne| if mne.opcode.starts_with("__") {
                                None
                            } else {
                                Some(mne.opcode.len() + mne.operands.iter().map(|a| format!("{}", a).len()).sum::<usize>())
                            }
                        )
                        .count();
                    /*let has_cmnt = bb.mnemonics.iter().any(|mne| {
                      self.control_flow_comments.contains_key(&mne.area.start)
                      });*/
                    let height = line_count * line_height + 2 * margin + 2 * padding;
                    let width = linew * char_width + 2 * margin + 2 * padding + col_padding;
                    //+ if has_cmnt { cmnt_width } else { 0 };

                    dims.insert(vx.0, (width as f32, height as f32));
                }
                Some(&ControlFlowTarget::Unresolved(_)) |
                Some(&ControlFlowTarget::Failed(_, _)) => {
                    dims.insert(vx.0, (1., 1.));
                }
                None => return Err(format!("Unlabeled vertex {}", vx.0).into()),
            }
        }

        Ok(dims)
    }

    fn get_node_data(
        ct: &ControlFlowTarget,
        comments: &HashMap<u64, String>,
        values: Option<&AbstractInterpretation>,
        functions: &HashMap<Uuid, Function>,
    ) -> Result<Vec<BasicBlockLine>> {
        match ct {
            &ControlFlowTarget::Resolved(ref bb) => {
                let i = bb.mnemonics
                    .iter()
                    .filter_map(
                        |mne| if mne.opcode.starts_with("__") {
                            None
                        } else {
                            Some(mne)
                        }
                    )
                    .filter_map(|mne| Self::get_basic_block_line(mne, comments, values, functions).ok());
                Ok(i.collect())
            }
            &ControlFlowTarget::Unresolved(ref rv) => Ok(vec![Self::get_value_line(rv, values)]),
            &ControlFlowTarget::Failed(_, _) => Ok(vec![]),
        }
    }

    fn get_value_line(value: &Rvalue, values: Option<&AbstractInterpretation>) -> BasicBlockLine {
        let arg = Self::rvalue_to_operand(value, false, values);

        BasicBlockLine {
            opcode: "".to_string(),
            region: "".to_string(),
            offset: 0,
            comment: "".to_string(),
            args: vec![arg],
        }
    }

    fn rvalue_to_operand(rv: &Rvalue, has_sign: bool, values: Option<&AbstractInterpretation>) -> BasicBlockOperand {
        match rv {
            &Rvalue::Constant { value: c, size: s } => {
                let val = if s < 64 { c % (1u64 << s) } else { c };
                let sign_bit = if s < 64 {
                    1u64 << (s - 1)
                } else {
                    0x8000000000000000
                };
                let s = if !has_sign || val & sign_bit == 0 {
                    format!("0x{:x}", val)
                } else {
                    format!("0x{:x}", (val as i64).wrapping_neg())
                };
                BasicBlockOperand {
                    kind: "constant",
                    display: s.clone(),
                    alt: "".to_string(),
                    data: s,
                }
            }
            &Rvalue::Variable { ref name, subscript, .. } => {
                let data = if let Some(subscript) = subscript {
                    format!("{}_{}", *name, subscript)
                } else {
                    format!("{}", *name)
                };
                let (display, alt) = values
                    .and_then(
                        |x| {
                            subscript.and_then(
                                |s| {
                                    let nam = VarName { name: name.clone(), subscript: s };
                                    x.output.get(&nam)
                                }
                            )
                        }
                    )
                    .and_then(
                        |val| if val != &Kset::Join && val != &Kset::Meet {
                            Some(val)
                        } else {
                            None
                        }
                    )
                    .map(|x| (format!("{}", x), name.to_string()))
                    .unwrap_or_else(|| (name.to_string(), "".to_string()));

                BasicBlockOperand { kind: "variable", display: display, alt: alt, data: data }
            }
            &Rvalue::Undefined => {
                BasicBlockOperand {
                    kind: "variable",
                    display: "?".to_string(),
                    alt: "".to_string(),
                    data: "".to_string(),
                }
            }
        }
    }

    pub fn get_basic_block_line(
        mnemonic: &Mnemonic,
        comments: &HashMap<u64, String>,
        values: Option<&AbstractInterpretation>,
        functions: &HashMap<Uuid, Function>,
    ) -> Result<BasicBlockLine> {
        use panopticon_core::MnemonicFormatToken;

        let mut ret = BasicBlockLine {
            opcode: mnemonic.opcode.clone(),
            region: "".to_string(),
            offset: mnemonic.area.start,
            comment: comments.get(&mnemonic.area.start).unwrap_or(&"".to_string()).to_string(),
            args: vec![],
        };
        let mut ops = mnemonic.operands.clone();

        ops.reverse();
        ret.args = mnemonic
            .format_string
            .iter()
            .filter_map(
                |x| match x {
                    &MnemonicFormatToken::Literal(ref s) => {
                        Some(
                            BasicBlockOperand {
                                kind: "literal",
                                display: s.to_string(),
                                alt: "".to_string(),
                                data: "".to_string(),
                            }
                        )
                    }
                    &MnemonicFormatToken::Variable { has_sign } => {
                        match ops.pop() {
                            Some(ref rv) => Some(Self::rvalue_to_operand(rv, has_sign, values)),
                            None => {
                                error!(
                                    "mnemonic at {:x} has invalid format string: {:?}",
                                    mnemonic.area.start,
                                    mnemonic
                                );
                                None
                            }
                        }
                    }
                    &MnemonicFormatToken::Pointer { is_code, .. } => {
                        match ops.pop() {
                            Some(Rvalue::Constant { value: c, size: s }) => {
                                let val = if s < 64 { c % (1u64 << s) } else { c };
                                let (display, data) = if is_code {
                                    let maybe_func = functions
                                        .iter()
                                        .find(|&(_, f)| val == f.start());
                                    if let Some((_, called_func)) = maybe_func {
                                        (called_func.name.clone(), format!("{}", called_func.uuid()))
                                    } else {
                                        (format!("0x{:x}", val), "".to_string())
                                    }
                                } else {
                                    (format!("0x{:x}", val), "".to_string())
                                };

                                Some(
                                    BasicBlockOperand {
                                        kind: if data == "" { "pointer" } else { "function" },
                                        display: display,
                                        alt: "".to_string(),
                                        data: data,
                                    }
                                )
                            }
                            Some(Rvalue::Variable { ref name, .. }) => {
                                Some(
                                    BasicBlockOperand {
                                        kind: "pointer",
                                        display: name.to_string(),
                                        alt: "".to_string(),
                                        data: "".to_string(),
                                    }
                                )
                            }
                            Some(Rvalue::Undefined) => {
                                Some(
                                    BasicBlockOperand {
                                        kind: "pointer",
                                        display: "?".to_string(),
                                        alt: "".to_string(),
                                        data: "".to_string(),
                                    }
                                )
                            }
                            None => {
                                error!(
                                    "mnemonic at {:x} has invalid format string: {:?}",
                                    mnemonic.area.start,
                                    mnemonic
                                );
                                None
                            }
                        }
                    }
                }
            )
            .collect();

        Ok(ret)
    }

    fn get_edge_data(edge_desc: AdjacencyListEdgeDescriptor, func: &Function) -> Result<(&'static str, String)> {
        let cfg = &func.cfg();
        let label = cfg.edge_label(edge_desc)
            .map(
                |guard| if *guard != Guard::always() && *guard != Guard::never() {
                    format!("{}", guard)
                } else {
                    "".to_string()
                }
            )
            .unwrap_or("".to_string());
        let from = cfg.source(edge_desc);
        let to = cfg.target(edge_desc);
        let from_addr = cfg.vertex_label(from)
            .and_then(
                |lb| if let &ControlFlowTarget::Resolved(ref bb) = lb {
                    Some(bb.area.end)
                } else {
                    None
                }
            );
        let to_addr = cfg.vertex_label(to)
            .and_then(
                |lb| if let &ControlFlowTarget::Resolved(ref bb) = lb {
                    Some(bb.area.start)
                } else {
                    None
                }
            );
        let kind = if cfg.out_degree(from) >= 2 {
            if let (Some(from), Some(to)) = (from_addr, to_addr) {
                if to == from {
                    "fallthru"
                } else {
                    if from > to {
                        "branch-backedge"
                    } else {
                        "branch"
                    }
                }
            } else {
                "jump"
            }
        } else {
            "jump"
        };

        if kind != "jump" && label == "" {
            error!("{} edge has no label", kind);
        }

        Ok((kind, label))
    }
}
