/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015  Panopticon authors
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

//! Functions are a graph of `BasicBlock`s connected with conditional jumps.
//!
//! Functions have an entry point, a (non-unique) name and an unique identifier. Functions
//! do not share basic blocks. In case functions overlap in the binary, the basic blocks are copied
//! by the disassembler.
//!
//! Functions have the concept of unresolved basic blocks. These are inserted into the graph if a
//! indirect branch could not be resolved. If disassembly failes for example because an unknown
//! instruction was found, an error node is inserted into the graph to allow displaying a message
//! on the front-end.


use {Architecture, BasicBlock, Guard, Mnemonic, Operation, Region, Result, Rvalue, Statement, Program};

use panopticon_graph_algos::{AdjacencyList, EdgeListGraphTrait, GraphTrait, MutableGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::adjacency_list::{AdjacencyListEdgeDescriptor, AdjacencyListVertexDescriptor, VertexLabelIterator};
use panopticon_graph_algos::search::{TraversalOrder, TreeIterator};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use uuid::Uuid;

/// An iterator over every BasicBlock in a Function
pub struct BasicBlockIterator<'a> {
    iter: VertexLabelIterator<'a, ControlFlowTarget, Guard, AdjacencyList<ControlFlowTarget, Guard>>
}

impl<'a> BasicBlockIterator<'a> {
    /// Create a new statement iterator from `mnemonics`
    pub fn new(cfg: &'a ControlFlowGraph) -> Self {
        BasicBlockIterator {
            iter: cfg.into_iter(),
        }
    }
}

impl<'a> Iterator for BasicBlockIterator<'a> {
    type Item = &'a BasicBlock;
    fn next(&mut self) ->  Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(&ControlFlowTarget::Resolved(ref bb)) => return Some(bb),
                _ => ()
            }
        }
    }
}

/// Node of the function graph.
#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum ControlFlowTarget {
    /// A basic block
    Resolved(BasicBlock),
    /// An unresolved indirect jump
    Unresolved(Rvalue),
    /// An error occured while disassembling
    Failed(u64, Cow<'static, str>),
}

/// Graph of basic blocks and jumps
pub type ControlFlowGraph = AdjacencyList<ControlFlowTarget, Guard>;
/// Stable reference to a node in the `ControlFlowGraph`
pub type ControlFlowRef = AdjacencyListVertexDescriptor;
/// Stable reference to an edge in the `ControlFlowGraph`
pub type ControlFlowEdge = AdjacencyListEdgeDescriptor;

/// A set of basic blocks connected by conditional jumps
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Function {
    /// Display name of the function.
    pub name: String,
    /// Unique, immutable identifier for this function.
    uuid: Uuid,
    /// Graph of basic blocks and jumps
    cflow_graph: ControlFlowGraph,
    /// The function's entry point
    entry_point: ControlFlowRef,
    /// Name of the memory region the function is part of
    region: String,
    /// The size of this function, in bytes (only counts the number of instructions, not padding bytes, or gaps for non-contiguous functions)
    size: usize,
}

#[derive(Clone,PartialEq,Eq,Debug)]
enum MnemonicOrError {
    Mnemonic(Mnemonic),
    Error(u64, Cow<'static, str>),
}

impl Function {
    /// Create an undefined Function. This function has undefined behavior. Creating an undefined Function always succeeds, and is usually a bad idea. Don't do it unless you know what you're doing.
    pub fn undefined(start: u64, uuid: Option<Uuid>, region: &Region, name: Option<String>) -> Function {
        let mut cflow_graph = AdjacencyList::new();
        let entry_point = ControlFlowTarget::Unresolved(Rvalue::new_u64(start));
        let entry_point = cflow_graph.add_vertex(entry_point);
        Function {
            name: name.unwrap_or(format!("func_{:#x}", start)),
            uuid: uuid.unwrap_or(Uuid::new_v4()),
            cflow_graph,
            entry_point,
            region: region.name().clone(),
            size: 0,
        }
    }
    /// Create and start disassembling a new function with `name`, inside memory `region`, starting at entry point `start`, with a random UUID.
    pub fn new<A: Architecture>(start: u64, region: &Region, name: Option<String>, init: A::Configuration) -> Result<Function> {
        let mut cflow_graph = AdjacencyList::new();
        let entry_point = ControlFlowTarget::Unresolved(Rvalue::new_u64(start));
        cflow_graph.add_vertex(entry_point);
        let mut size = 0;
        let (mut mnemonics, mut by_source, mut by_destination) = Self::index_cflow_graph(&mut cflow_graph, start);

        let mut todo = cflow_graph.into_iter().filter_map(|lb| {
            if let &ControlFlowTarget::Unresolved(Rvalue::Constant{ value,.. }) = lb {
                Some(value)
            } else {
                None
            }
        }).collect::<HashSet<u64>>();

        todo.insert(start);

        while let Some(addr) = todo.iter().next().cloned() {
            let maybe_mnes = mnemonics.iter().find(|x| *x.0 >= addr).map(|x| x.1.clone());

            assert!(todo.remove(&addr));

            if let Some(mnes) = maybe_mnes {
                if !mnes.is_empty() {
                    match mnes.first() {
                        Some(&MnemonicOrError::Mnemonic(ref mne)) => {
                            if mne.area.start < addr && mne.area.end > addr {
                                mnemonics.entry(addr).or_insert(Vec::new()).push(MnemonicOrError::Error(addr, "Jump inside instruction".into()));
                                continue;
                            } else if mne.area.start == addr {
                                size += mne.size();
                                continue;
                            }
                        }
                        Some(&MnemonicOrError::Error(ref pos, _)) => {
                            if *pos == addr {
                                continue;
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }

            let maybe_match = A::decode(region, addr, &init);

            match maybe_match {
                Ok(match_st) => {
                    if match_st.mnemonics.is_empty() {
                        mnemonics.entry(addr).or_insert(Vec::new()).push(MnemonicOrError::Error(addr, "Unrecognized instruction".into()));
                    } else {
                        for mne in match_st.mnemonics {
                            debug!(
                                "{:x}: {} ({:?})",
                                mne.area.start,
                                mne.opcode,
                                match_st.tokens
                            );
                            size += mne.size();
                            mnemonics.entry(mne.area.start).or_insert(Vec::new()).push(MnemonicOrError::Mnemonic(mne));
                        }
                    }

                    for (origin, tgt, gu) in match_st.jumps {
                        debug!("jump to {:?}", tgt);
                        match tgt {
                            Rvalue::Constant { value: ref c, .. } => {
                                by_source.entry(origin).or_insert(Vec::new()).push((tgt.clone(), gu.clone()));
                                by_destination.entry(*c).or_insert(Vec::new()).push((Rvalue::new_u64(origin), gu.clone()));
                                todo.insert(*c);
                            }
                            _ => {
                                by_source.entry(origin).or_insert(Vec::new()).push((tgt, gu.clone()));
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("failed to disassemble: {}", e);
                    mnemonics.entry(addr).or_insert(Vec::new()).push(MnemonicOrError::Error(addr, "Unrecognized instruction".into()));
                }
            }
        }

        let cfg = Self::assemble_cflow_graph(mnemonics, by_source, by_destination, start);
        let name = name.unwrap_or(format!("func_{:#x}", start));
        let uuid = Uuid::new_v4();
        let ep = cfg
            .vertices()
            .find(
                |&x| match cfg.vertex_label(x) {
                    Some(&ControlFlowTarget::Resolved(ref bb)) => bb.area.start == start && bb.area.end > start,
                    _ => false,
                }
            );

        let entry_point = match ep {
            Some(entry_point) => {
                entry_point
            },
            None => {
                return Err(format!("function ({}) {} has no entry point", name, uuid).into());
            }
        };
        Ok(Function {
            name,
            uuid,
            cflow_graph: cfg,
            entry_point,
            region: region.name().clone(),
            size,
        })
    }

    /// Returns the start address of the first basic block in this function
    pub fn start(&self) -> u64 {
        self.entry_point().area.start
    }

    /// New function starting at `start`, with name `name`, inside memory region `region` and UUID `uuid`.
    pub fn with_uuid<A: Architecture>(start: u64, uuid: &Uuid, region: &Region, name: Option<String>, init: A::Configuration) -> Result<Function> {
        let mut f = Function::new::<A>(start, region, name, init)?;
        f.uuid = uuid.clone();
        Ok(f)
    }

    /// Returns the UUID of this function
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// The size of this function, in bytes (only counts the number of instructions, not padding bytes, or gaps for non-contiguous functions)
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a reference to this functions control flow graph
    pub fn cfg(&self) -> &ControlFlowGraph {
        &self.cflow_graph
    }

    /// Returns a mutable reference to this functions control flow graph; **WARNING** this can cause instability if the entry point is not correctly updated
    pub fn cfg_mut(&mut self) -> &mut ControlFlowGraph {
        &mut self.cflow_graph
    }

    /// Returns a reference to the entry point vertex in the cfg
    pub fn entry_point_ref(&self) -> ControlFlowRef {
        self.entry_point
    }

    /// Sets the functions entry point vertex in the cfg to `vx` (this is primarily for use in tests).
    ///
    /// **WARNING** Make sure the vertex descriptor actually is the entry point _and_ points to a _resolved_ basic block, otherwise subsequent operations on this function will be undefined.
    pub fn set_entry_point_ref(&mut self, vx: ControlFlowRef) {
        self.entry_point = vx;
    }

    /// Returns a reference to the BasicBlock entry point of this function.
    pub fn entry_point(&self) -> &BasicBlock {
        match self.cflow_graph.vertex_label(self.entry_point).unwrap() {
            &ControlFlowTarget::Resolved(ref bb) => bb,
            _ => panic!("Function {} has an unresolved entry point - this is a bug, dumping the cfg: {:?}", self.name, self.cflow_graph)
        }
    }

    /// Returns a mutable reference to the BasicBlock entry point of this function.
    pub fn entry_point_mut(&mut self) -> &mut BasicBlock {
        match self.cflow_graph.vertex_label_mut(self.entry_point).unwrap() {
            &mut ControlFlowTarget::Resolved(ref mut bb) => bb,
            _ => panic!("Function {} has an unresolved entry point - this is a bug!", self.name) // can't dump cfg here because borrowed mutable ;)
        }
    }

    fn index_cflow_graph(
        g: &ControlFlowGraph,
        entry: u64,
    ) -> (BTreeMap<u64, Vec<MnemonicOrError>>, HashMap<u64, Vec<(Rvalue, Guard)>>, HashMap<u64, Vec<(Rvalue, Guard)>>) {
        let mut mnemonics = BTreeMap::new();
        let mut by_source = HashMap::<u64, Vec<(Rvalue, Guard)>>::new();
        let mut by_destination = HashMap::<u64, Vec<(Rvalue, Guard)>>::new();

        by_destination.insert(entry, vec![(Rvalue::Undefined, Guard::always())]);

        for cft in g {
            match cft {
                &ControlFlowTarget::Resolved(ref bb) => {
                    let mut prev_mne = None;

                    for mne in &bb.mnemonics {
                        mnemonics.entry(mne.area.start).or_insert(Vec::new()).push(MnemonicOrError::Mnemonic(mne.clone()));

                        if let Some(prev) = prev_mne {
                            by_source.entry(prev).or_insert(Vec::new()).push((Rvalue::new_u64(mne.area.start), Guard::always()));
                            by_destination.entry(mne.area.start).or_insert(Vec::new()).push((Rvalue::new_u64(prev), Guard::always()));
                        }
                        prev_mne = Some(mne.area.start);
                    }
                }
                &ControlFlowTarget::Failed(ref pos, ref msg) => {
                    mnemonics.entry(*pos).or_insert(Vec::new()).push(MnemonicOrError::Error(*pos, msg.clone()));
                }
                &ControlFlowTarget::Unresolved(_) => {}
            }
        }

        for e in g.edges() {
            let gu = g.edge_label(e).unwrap().clone();
            let src = g.vertex_label(g.source(e));
            let tgt = g.vertex_label(g.target(e));

            match (src, tgt) {
                // Resolved -> Resolved
                (Some(&ControlFlowTarget::Resolved(ref src_bb)), Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    let last = src_bb.mnemonics.last().map_or(src_bb.area.start, |mne| mne.area.start);
                    by_source.entry(last).or_insert(Vec::new()).push((Rvalue::new_u64(tgt_bb.area.start), gu.clone()));
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((Rvalue::new_u64(last), gu));
                }
                // Resolved -> Unresolved(Constant)
                (Some(&ControlFlowTarget::Resolved(ref src_bb)), Some(&ControlFlowTarget::Unresolved(Rvalue::Constant { value: ref c, .. }))) => {
                    let last = src_bb.mnemonics.last().map_or(src_bb.area.start, |mne| mne.area.start);
                    by_source.entry(last).or_insert(Vec::new()).push((Rvalue::new_u64(*c), gu.clone()));
                    by_destination.entry(*c).or_insert(Vec::new()).push((Rvalue::new_u64(last), gu));
                }
                // Unresolved(Constant) -> Resolved
                (Some(&ControlFlowTarget::Unresolved(Rvalue::Constant { value: ref c, .. })), Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    by_source.entry(*c).or_insert(Vec::new()).push((Rvalue::new_u64(tgt_bb.area.start), gu.clone()));
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((Rvalue::new_u64(*c), gu));
                }
                // Resolved -> Unresolved
                (Some(&ControlFlowTarget::Resolved(ref src_bb)), Some(&ControlFlowTarget::Unresolved(ref r))) => {
                    by_source.entry(src_bb.area.start).or_insert(Vec::new()).push((r.clone(), gu));
                }
                // Unresolved -> Resolved
                (Some(&ControlFlowTarget::Unresolved(ref t)), Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((t.clone(), gu));
                }
                // Failed -> Resolved
                (Some(&ControlFlowTarget::Failed(ref src_pos, _)), Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    by_source.entry(*src_pos).or_insert(Vec::new()).push((Rvalue::new_u64(tgt_bb.area.start), gu.clone()));
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((Rvalue::new_u64(*src_pos), gu));
                }
                // Resolved -> Failed
                (Some(&ControlFlowTarget::Resolved(ref src_bb)), Some(&ControlFlowTarget::Failed(ref tgt_pos, _))) => {
                    let last = src_bb.mnemonics.last().map_or(src_bb.area.start, |mne| mne.area.start);
                    by_source.entry(last).or_insert(Vec::new()).push((Rvalue::new_u64(*tgt_pos), gu.clone()));
                    by_destination.entry(*tgt_pos).or_insert(Vec::new()).push((Rvalue::new_u64(last), gu));
                }
                // Failed -> Failed
                (Some(&ControlFlowTarget::Failed(ref src_pos, _)), Some(&ControlFlowTarget::Failed(ref tgt_pos, _))) => {
                    by_source.entry(*src_pos).or_insert(Vec::new()).push((Rvalue::new_u64(*tgt_pos), gu.clone()));
                    by_destination.entry(*tgt_pos).or_insert(Vec::new()).push((Rvalue::new_u64(*src_pos), gu));
                }
                // Failed -> Unresolved(Constant)
                (Some(&ControlFlowTarget::Failed(ref src_pos, _)), Some(&ControlFlowTarget::Unresolved(Rvalue::Constant { value: ref c, .. }))) => {
                    by_source.entry(*src_pos).or_insert(Vec::new()).push((Rvalue::new_u64(*c), gu.clone()));
                    by_destination.entry(*c).or_insert(Vec::new()).push((Rvalue::new_u64(*src_pos), gu));
                }
                // Unresolved(Constant) -> Failed
                (Some(&ControlFlowTarget::Unresolved(Rvalue::Constant { value: ref c, .. })), Some(&ControlFlowTarget::Failed(ref tgt_pos, _))) => {
                    by_source.entry(*c).or_insert(Vec::new()).push((Rvalue::new_u64(*tgt_pos), gu.clone()));
                    by_destination.entry(*tgt_pos).or_insert(Vec::new()).push((Rvalue::new_u64(*c), gu));
                }
                // Failed -> Unresolved
                (Some(&ControlFlowTarget::Failed(ref src_pos, _)), Some(&ControlFlowTarget::Unresolved(ref r))) => {
                    by_source.entry(*src_pos).or_insert(Vec::new()).push((r.clone(), gu));
                }
                // Unresolved -> Failed
                (Some(&ControlFlowTarget::Unresolved(ref t)), Some(&ControlFlowTarget::Failed(ref tgt_pos, _))) => {
                    by_destination.entry(*tgt_pos).or_insert(Vec::new()).push((t.clone(), gu));
                }
                // Unresolved -> Unresolved
                (Some(&ControlFlowTarget::Unresolved(_)), Some(&ControlFlowTarget::Unresolved(_))) => {}
                (None, _) | (_, None) => unreachable!(),
            }
        }

        (mnemonics, by_source, by_destination)
    }

    fn assemble_cflow_graph(
        mut mnemonics: BTreeMap<u64, Vec<MnemonicOrError>>,
        by_source: HashMap<u64, Vec<(Rvalue, Guard)>>,
        by_destination: HashMap<u64, Vec<(Rvalue, Guard)>>,
        start: u64,
    ) -> ControlFlowGraph {
        let mut ret = ControlFlowGraph::new();
        let mut bblock = Vec::<Mnemonic>::new();

        for (_, mnes) in mnemonics.iter_mut() {
            if !bblock.is_empty() && !mnes.is_empty() {
                if let Some(&MnemonicOrError::Mnemonic(ref mne)) = mnes.first() {
                    let last_mne = &bblock.last().unwrap().clone();

                    // if next mnemonics aren't adjacent
                    let mut new_bb = bblock.last().unwrap().area.end != mne.area.start;

                    // or any following jumps aren't to adjacent mnemonics
                    new_bb |= by_source
                        .get(&last_mne.area.start)
                        .unwrap_or(&Vec::new())
                        .iter()
                        .any(
                            |&(ref opt_dest, _)| if let &Rvalue::Constant { value, .. } = opt_dest {
                                value != mne.area.start
                            } else {
                                false
                            }
                        );

                    // or any jumps pointing to the next that aren't from here
                    new_bb |= by_destination
                        .get(&mne.area.start)
                        .unwrap_or(&Vec::new())
                        .iter()
                        .any(
                            |&(ref opt_src, _)| if let &Rvalue::Constant { value, .. } = opt_src {
                                value != last_mne.area.start
                            } else {
                                false
                            }
                        );

                    // or the entry point does not point here
                    new_bb |= mne.area.start == start;

                    if new_bb {
                        let bb = BasicBlock::from_vec(bblock.clone());

                        bblock.clear();
                        ret.add_vertex(ControlFlowTarget::Resolved(bb));
                    }
                }
            }

            for moe in mnes.drain(..) {
                match moe {
                    MnemonicOrError::Mnemonic(mne) => {
                        bblock.push(mne);
                    }
                    MnemonicOrError::Error(pos, msg) => {
                        ret.add_vertex(ControlFlowTarget::Failed(pos, msg));
                    }
                }
            }
        }

        // last basic block
        if !bblock.is_empty() {
            ret.add_vertex(ControlFlowTarget::Resolved(BasicBlock::from_vec(bblock)));
        }

        // connect basic blocks
        for (src_off, tgts) in by_source.iter() {
            for &(ref tgt, ref gu) in tgts {

                let from_bb = ret.vertices()
                    .find(
                        |&t| match ret.vertex_label(t) {
                            Some(&ControlFlowTarget::Resolved(ref bb)) => bb.mnemonics.last().map_or(false, |x| x.area.start == *src_off),
                            Some(&ControlFlowTarget::Unresolved(Rvalue::Constant { value: v, .. })) => v == *src_off,
                            Some(&ControlFlowTarget::Unresolved(_)) => false,
                            Some(&ControlFlowTarget::Failed(pos, _)) => pos == *src_off,
                            None => unreachable!(),
                        }
                    );
                let to_bb = ret.vertices()
                    .find(
                        |&t| match (tgt, ret.vertex_label(t)) {
                            (&Rvalue::Constant { value, .. }, Some(&ControlFlowTarget::Resolved(ref bb))) => bb.area.start == value,
                            (&Rvalue::Constant { value, .. }, Some(&ControlFlowTarget::Failed(pos, _))) => pos == value,
                            (rv, Some(&ControlFlowTarget::Unresolved(ref v))) => *v == *rv,
                            (_, None) => unreachable!(),
                            _ => false,
                        }
                    );

                match (from_bb, to_bb) {
                    (Some(from), Some(to)) => {
                        ret.add_edge(gu.clone(), from, to);
                    }
                    (None, Some(to)) => {
                        if let Some(&ControlFlowTarget::Resolved(ref bb)) = ret.vertex_label(to) {
                            if bb.area.start <= *src_off && bb.area.end > *src_off {
                                continue;
                            }
                        }

                        let vx = ret.add_vertex(ControlFlowTarget::Unresolved(Rvalue::new_u64(*src_off)));
                        ret.add_edge(gu.clone(), vx, to);
                    }
                    (Some(from), None) => {
                        if let Some(&ControlFlowTarget::Resolved(ref bb)) = ret.vertex_label(from) {
                            if let &Rvalue::Constant { value, .. } = tgt {
                                if bb.area.start <= value && bb.area.end > value {
                                    continue;
                                }
                            }
                        }

                        let vx = ret.add_vertex(ControlFlowTarget::Unresolved(tgt.clone()));
                        ret.add_edge(gu.clone(), from, vx);
                    }
                    _ => {
                        trace!(
                            "jump from 0x{:x} to {} doesn't hit any blocks",
                            src_off,
                            tgt
                        )
                    }
                }
            }
        }

        ret
    }

    /// Returns an iterator over this functions `BasicBlock`s
    pub fn basic_blocks(&self) -> BasicBlockIterator {
        BasicBlockIterator::new(&self.cflow_graph)
    }

    /// Returns the address of every function this function calls
    pub fn collect_call_addresses(&self) -> Vec<u64> {
        let mut ret = Vec::new();
        for bb in self.basic_blocks() {
            for statement in bb.statements() {
                match statement {
                    &Statement { op: Operation::Call(Rvalue::Constant{ value, .. }), .. } => ret.push(value),
                    _ => ()
                }
            }
        }
        debug!("collected calls: {:?}", ret);
        ret
    }

    /// Returns all call targets.
    pub fn collect_calls(&self) -> Vec<Rvalue> {
        let mut ret = Vec::new();
        for bb in self.basic_blocks() {
            for statement in bb.statements() {
                match statement {
                    &Statement { op: Operation::Call(ref t), .. } => ret.push(t.clone()),
                    _ => ()
                }
            }
        }
        debug!("collected calls: {:?}", ret);
        ret
    }

    /// Returns the basic block that begins at `a`.
    pub fn find_basic_block_by_start(&self, a: u64) -> Option<ControlFlowRef> {
        self.cflow_graph
            .vertices()
            .find(
                |&x| match self.cflow_graph.vertex_label(x) {
                    Some(&ControlFlowTarget::Resolved(ref bb)) => bb.area.start == a && bb.area.end > a,
                    _ => false,
                }
            )
    }

    /// Returns the basic block that contains `a`.
    pub fn find_basic_block_at(&self, a: u64) -> Option<&BasicBlock> {
        self.basic_blocks().find(|&bb| bb.area.start <= a && bb.area.end > a)
    }

    /// Returns all nodes in the graph of this function in post order.
    pub fn postorder(&self) -> Vec<ControlFlowRef> {
        TreeIterator::new(
            self.entry_point,
            TraversalOrder::Postorder,
            &self.cflow_graph,
        )
                .collect()
    }

    /// Returns the functions basic block graph in graphivz's DOT format. Useful for debugging.
    pub fn to_dot(&self) -> String {
        let mut ret = "digraph G {".to_string();

        for v in self.cflow_graph.vertices() {
            match self.cflow_graph.vertex_label(v) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    ret = format!(
                        "{}\n{} [label=<<table border=\"0\"><tr><td>{}:{}</td></tr>",
                        ret,
                        v.0,
                        bb.area.start,
                        bb.area.end
                    );

                    for mne in bb.mnemonics.iter() {
                        ret = format!("{}<tr><td align=\"left\">{}</td></tr>", ret, mne.opcode);
                        for i in mne.instructions.iter() {
                            ret = format!(
                                "{}<tr><td align=\"left\">&nbsp;&nbsp;&nbsp;&nbsp;{}</td></tr>",
                                ret,
                                i
                            );
                        }
                    }

                    ret = format!("{}</table>>,shape=record];", ret);
                }
                Some(&ControlFlowTarget::Unresolved(ref c)) => {
                    ret = format!("{}\n{} [label=\"{:?}\",shape=circle];", ret, v.0, c);
                }
                _ => {
                    ret = format!("{}\n{} [label=\"?\",shape=circle];", ret, v.0);
                }
            }
        }

        for e in self.cflow_graph.edges() {
            ret = format!(
                "{}\n{} -> {} [label=\"{}\"];",
                ret,
                self.cflow_graph.source(e).0,
                self.cflow_graph.target(e).0,
                self.cflow_graph.edge_label(e).unwrap()
            );
        }

        format!("{}\n}}", ret)
    }

    /// Displays the function in a human readable format, using `program`
    pub fn display_with(&self, program: &Program) -> String {
        let mut bbs = self.cflow_graph.vertices().filter_map(|v| {
            match self.cflow_graph.vertex_label(v) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    Some (bb)
                },
                _ => None
            }
        }).collect::<Vec<&BasicBlock>>();
        bbs.sort_by(|bb1, bb2| bb1.area.start.cmp(&bb2.area.start));
        let mut fmt = if let Some(bb) = bbs.first() {
            format!("{:0>8x} <{}>:", bb.area.start, self.name)
        } else {
            format!("{}", self.name)
        };
        for bb in bbs {
            fmt = format!("{}{}", fmt, bb.display_with(program));
        }
        fmt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Architecture, BasicBlock, Bound, Disassembler, Guard, Match, Mnemonic, OpaqueLayer, Region, Result, Rvalue, State};
    use panopticon_graph_algos::{AdjacencyMatrixGraphTrait, EdgeListGraphTrait, VertexListGraphTrait};
    use panopticon_graph_algos::{GraphTrait, MutableGraphTrait};
    use std::borrow::Cow;
    use std::sync::Arc;

    #[derive(Clone,Debug)]
    enum TestArchShort {}
    impl Architecture for TestArchShort {
        type Token = u8;
        type Configuration = Arc<Disassembler<TestArchShort>>;

        fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
            unimplemented!()
        }

        fn decode(reg: &Region, addr: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
            if let Some(s) = cfg.next_match(&mut reg.iter().seek(addr), addr, cfg.clone()) {
                Ok(s.into())
            } else {
                Err("No match".into())
            }
        }
    }

    #[derive(Clone,Debug)]
    enum TestArchWide {}
    impl Architecture for TestArchWide {
        type Token = u16;
        type Configuration = Arc<Disassembler<TestArchWide>>;

        fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
            unimplemented!()
        }

        fn decode(reg: &Region, addr: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
            if let Some(s) = cfg.next_match(&mut reg.iter().seek(addr), addr, cfg.clone()) {
                Ok(s.into())
            } else {
                Err("No match".into())
            }
        }
    }

    #[test]
    fn new() {
        let f = Function::undefined(100, None, &Region::undefined("ram".to_owned(), 100), Some("test".to_owned()));

        assert_eq!(f.name, "test".to_string());
        assert_eq!(f.cflow_graph.num_vertices(), 1);
        assert_eq!(f.cflow_graph.num_edges(), 0);
        match f.cflow_graph.vertex_label(f.entry_point_ref()).unwrap() {
            &ControlFlowTarget::Unresolved(Rvalue::Constant{ value, size }) => {
                assert_eq!(value, 100);
                assert_eq!(size, 64);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn index_resolved() {
        let mut cfg = ControlFlowGraph::new();

        let bb0 = BasicBlock::from_vec(
            vec![
                Mnemonic::dummy(0..1),
                Mnemonic::dummy(1..2),
                Mnemonic::dummy(2..5),
                Mnemonic::dummy(5..6),
            ]
        );
        let bb1 = BasicBlock::from_vec(
            vec![
                Mnemonic::dummy(10..11),
                Mnemonic::dummy(11..12),
                Mnemonic::dummy(12..15),
                Mnemonic::dummy(15..16),
            ]
        );
        let bb2 = BasicBlock::from_vec(vec![Mnemonic::dummy(6..10)]);

        let vx0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let vx1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let vx2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));

        cfg.add_edge(Guard::always(), vx0, vx1);
        cfg.add_edge(Guard::always(), vx1, vx1);
        cfg.add_edge(Guard::always(), vx1, vx2);
        cfg.add_edge(Guard::always(), vx2, vx0);

        let (mnes, src, dest) = Function::index_cflow_graph(&cfg, 0);

        assert_eq!(mnes.len(), 9);
        assert_eq!(src.values().fold(0, |acc, x| acc + x.len()), 10);
        assert_eq!(dest.values().fold(0, |acc, x| acc + x.len()), 11); // because index_cflow_graph adds the start/entry value

        let cfg_re = Function::assemble_cflow_graph(mnes, src, dest, 0);

        assert_eq!(cfg_re.num_vertices(), 3);
        assert_eq!(cfg_re.num_edges(), 4);

        for vx in cfg_re.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg_re.vertex_label(vx) {
                assert!((bb.area.start == 0 && bb.area.end == 6) || (bb.area.start == 10 && bb.area.end == 16) || (bb.area.start == 6 && bb.area.end == 10));
            } else {
                unreachable!();
            }
        }

        for e in cfg_re.edges() {
            if let Some(&ControlFlowTarget::Resolved(ref from)) = cfg_re.vertex_label(cfg_re.source(e)) {
                if let Some(&ControlFlowTarget::Resolved(ref to)) = cfg_re.vertex_label(cfg_re.target(e)) {
                    assert!(
                        (from.area.start == 0 && to.area.start == 10) || (from.area.start == 10 && to.area.start == 10) ||
                        (from.area.start == 10 && to.area.start == 6) || (from.area.start == 6 && to.area.start == 0)
                    );
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }
    }

    #[test]
    fn index_unresolved() {
        let mut cfg = ControlFlowGraph::new();

        let bb0 = BasicBlock::from_vec(vec![Mnemonic::dummy(0..1)]);
        let bb1 = BasicBlock::from_vec(vec![Mnemonic::dummy(10..11)]);

        let vx0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let vx1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let vx2 = cfg.add_vertex(ControlFlowTarget::Unresolved(Rvalue::new_u32(42)));
        let vx3 = cfg.add_vertex(ControlFlowTarget::Unresolved(Rvalue::new_u32(23)));
        let vx4 = cfg.add_vertex(
            ControlFlowTarget::Unresolved(
                Rvalue::Variable {
                    name: Cow::Borrowed("a"),
                    size: 8,
                    offset: 0,
                    subscript: None,
                }
            )
        );

        cfg.add_edge(Guard::always(), vx0, vx1);
        cfg.add_edge(Guard::always(), vx2, vx1);
        cfg.add_edge(Guard::always(), vx3, vx0);
        cfg.add_edge(Guard::always(), vx4, vx3);

        let (mnes, src, dest) = Function::index_cflow_graph(&cfg, 0);

        assert_eq!(mnes.len(), 2);
        assert_eq!(src.values().fold(0, |acc, x| acc + x.len()), 3);
        assert_eq!(dest.values().fold(0, |acc, x| acc + x.len()), 4); // because index_cflow_graph automatically adds the functions start entry

        let cfg_re = Function::assemble_cflow_graph(mnes, src, dest, 0);

        assert_eq!(cfg_re.num_vertices(), 4);
        assert_eq!(cfg_re.num_edges(), 3);

        for vx in cfg_re.vertices() {
            match cfg_re.vertex_label(vx) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    assert!((bb.area.start == 0 && bb.area.end == 1) || (bb.area.start == 10 && bb.area.end == 11));
                }
                Some(&ControlFlowTarget::Unresolved(Rvalue::Constant { value: ref c, size: 64 })) => {
                    assert!(*c == 42 || *c == 23);
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    #[test]
    fn add_single() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"A","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );
        let data = OpaqueLayer::wrap(vec![0]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(0, &reg, None, main).unwrap();

        assert_eq!(func.cflow_graph.num_vertices(), 1);
        assert_eq!(func.cflow_graph.num_edges(), 0);

        if let Some(vx) = func.cflow_graph.vertices().next() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                assert_eq!(bb.mnemonics.len(), 1);
                assert_eq!(bb.mnemonics[0].opcode, "A".to_string());
                assert_eq!(bb.mnemonics[0].area, Bound::new(0, 1));
                assert_eq!(bb.area, Bound::new(0, 1));
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }

        assert_eq!(func.entry_point_ref(), func.cflow_graph.vertices().next().unwrap());
        assert_eq!(func.name, "func_0x0".to_string());
    }

    #[test]
    fn continuous() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"test0","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            [ 3 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"test3","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            [ 4 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"test4","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            [ 5 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"test5","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2, 3, 4, 5]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(0, &reg, None, main).unwrap();

        assert_eq!(func.cflow_graph.num_vertices(), 2);
        assert_eq!(func.cflow_graph.num_edges(), 1);

        let mut bb_vx = None;
        let mut ures_vx = None;

        for vx in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                assert_eq!(bb.mnemonics.len(), 6);
                assert_eq!(bb.mnemonics[0].opcode, "test0".to_string());
                assert_eq!(bb.mnemonics[0].area, Bound::new(0, 1));
                assert_eq!(bb.mnemonics[1].opcode, "test1".to_string());
                assert_eq!(bb.mnemonics[1].area, Bound::new(1, 2));
                assert_eq!(bb.mnemonics[2].opcode, "test2".to_string());
                assert_eq!(bb.mnemonics[2].area, Bound::new(2, 3));
                assert_eq!(bb.mnemonics[3].opcode, "test3".to_string());
                assert_eq!(bb.mnemonics[3].area, Bound::new(3, 4));
                assert_eq!(bb.mnemonics[4].opcode, "test4".to_string());
                assert_eq!(bb.mnemonics[4].area, Bound::new(4, 5));
                assert_eq!(bb.mnemonics[5].opcode, "test5".to_string());
                assert_eq!(bb.mnemonics[5].area, Bound::new(5, 6));
                assert_eq!(bb.area, Bound::new(0, 6));
                bb_vx = Some(vx);
            } else if let Some(&ControlFlowTarget::Failed(c, _)) = func.cflow_graph.vertex_label(vx) {
                assert_eq!(c, 6);
                ures_vx = Some(vx);
            } else {
                unreachable!();
            }
        }

        assert!(ures_vx.is_some() && bb_vx.is_some());
        assert_eq!(Some(func.entry_point_ref()), bb_vx);
        assert_eq!(func.name, "func_0x0".to_string());
        assert!(func.cflow_graph.edge(bb_vx.unwrap(), ures_vx.unwrap()).is_some());
    }

    #[test]
    fn branch() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test0","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(3),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(0, &reg, None, main).unwrap();

        assert_eq!(func.cflow_graph.num_vertices(), 4);
        assert_eq!(func.cflow_graph.num_edges(), 4);

        let mut bb0_vx = None;
        let mut bb1_vx = None;
        let mut bb2_vx = None;
        let mut ures_vx = None;

        for vx in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                if bb.area.start == 0 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.mnemonics[0].opcode, "test0".to_string());
                    assert_eq!(bb.mnemonics[0].area, Bound::new(0, 1));
                    assert_eq!(bb.area, Bound::new(0, 1));
                    bb0_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.mnemonics[0].opcode, "test1".to_string());
                    assert_eq!(bb.mnemonics[0].area, Bound::new(1, 2));
                    assert_eq!(bb.area, Bound::new(1, 2));
                    bb1_vx = Some(vx);
                } else if bb.area.start == 2 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.mnemonics[0].opcode, "test2".to_string());
                    assert_eq!(bb.mnemonics[0].area, Bound::new(2, 3));
                    assert_eq!(bb.area, Bound::new(2, 3));
                    bb2_vx = Some(vx);
                } else {
                    unreachable!();
                }
            } else if let Some(&ControlFlowTarget::Failed(c, _)) = func.cflow_graph.vertex_label(vx) {
                assert_eq!(c, 3);
                ures_vx = Some(vx);
            } else {
                unreachable!();
            }
        }

        assert!(ures_vx.is_some() && bb0_vx.is_some() && bb1_vx.is_some() && bb2_vx.is_some());
        assert_eq!(Some(func.entry_point_ref()), bb0_vx);
        assert_eq!(func.name, "func_0x0".to_string());
        assert!(func.cflow_graph.edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb0_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb1_vx.unwrap(), ures_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb2_vx.unwrap(), bb1_vx.unwrap()).is_some());
    }

    #[test]
    fn function_loop() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test0","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(0),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(0, &reg, None, main).unwrap();

        assert_eq!(func.cflow_graph.num_vertices(), 1);
        assert_eq!(func.cflow_graph.num_edges(), 1);

        let vx = func.cflow_graph.vertices().next().unwrap();
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
            if bb.area.start == 0 {
                assert_eq!(bb.mnemonics.len(), 3);
                assert_eq!(bb.mnemonics[0].opcode, "test0".to_string());
                assert_eq!(bb.mnemonics[0].area, Bound::new(0, 1));
                assert_eq!(bb.mnemonics[1].opcode, "test1".to_string());
                assert_eq!(bb.mnemonics[1].area, Bound::new(1, 2));
                assert_eq!(bb.mnemonics[2].opcode, "test2".to_string());
                assert_eq!(bb.mnemonics[2].area, Bound::new(2, 3));
                assert_eq!(bb.area, Bound::new(0, 3));
            } else {
                unreachable!();
            }
        }

        assert_eq!(func.name, "func_0x0".to_string());
        assert_eq!(Some(func.entry_point_ref()), Some(vx));
        assert!(func.cflow_graph.edge(vx, vx).is_some());
    }

    #[test]
    fn empty() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test0","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(0),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(0, &reg, None, main);
        assert!(func.is_err());
        // these tests have been rendered somewhat moot now since the entry point must be present
        // assert_eq!(func.cflow_graph.num_vertices(), 1);
        // assert_eq!(func.cflow_graph.num_edges(), 0);
        // assert_eq!(func.name, "func_0x0".to_string());

        // let vx = func.cflow_graph.vertices().next().unwrap();
        // if let Some(&ControlFlowTarget::Failed(v, _)) = func.cflow_graph.vertex_label(vx) {
        //     assert_eq!(v, 0);
        // }
    }

    #[test]
    fn entry_split() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test0","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2]);
        let reg = Region::new("".to_string(), data);
        let bb = BasicBlock::from_vec(vec![Mnemonic::dummy(0..1), Mnemonic::dummy(1..2)]);
        let mut func = Function::new::<TestArchShort>(0, &reg, Some("test".to_owned()), main).unwrap();
        let vx0 = func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb));
        let vx1 = func.cflow_graph.add_vertex(ControlFlowTarget::Unresolved(Rvalue::new_u32(2)));

        func.set_entry_point_ref(vx0);
        func.cflow_graph.add_edge(Guard::always(), vx0, vx1);

        assert_eq!(func.cflow_graph.num_vertices(), 2);
        assert_eq!(func.cflow_graph.num_edges(), 2);
        assert_eq!(func.name, "test".to_string());

        let mut bb0_vx = None;
        let mut bb1_vx = None;

        for vx in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                if bb.area.start == 0 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.mnemonics[0].opcode, "dummy".to_string());
                    assert_eq!(bb.mnemonics[0].area, Bound::new(0, 1));
                    assert_eq!(bb.area, Bound::new(0, 1));
                    bb0_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(bb.mnemonics.len(), 2);
                    assert_eq!(bb.mnemonics[0].opcode, "dummy".to_string());
                    assert_eq!(bb.mnemonics[0].area, Bound::new(1, 2));
                    assert_eq!(bb.mnemonics[1].opcode, "test2".to_string());
                    assert_eq!(bb.mnemonics[1].area, Bound::new(2, 3));
                    assert_eq!(bb.area, Bound::new(1, 3));
                    bb1_vx = Some(vx);
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some());
        assert_eq!(Some(func.entry_point_ref()), bb0_vx);
        assert!(func.cflow_graph.edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb1_vx.unwrap(), bb1_vx.unwrap()).is_some());
    }

    #[test]
    fn wide_token() {
        let def = OpaqueLayer::wrap(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x44]);
        let reg = Region::new("".to_string(), def);
        let dec = new_disassembler!(TestArchWide =>
            [0x2211] = |s: &mut State<TestArchWide>|
            {
                let a = s.address;
                s.mnemonic(2,"A","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                s.jump(Rvalue::new_u64(a + 2),Guard::always()).unwrap();
                true
            },

            [0x4433] = |s: &mut State<TestArchWide>|
            {
                let a = s.address;
                s.mnemonic(2,"B","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                s.jump(Rvalue::new_u64(a + 2),Guard::always()).unwrap();
                s.jump(Rvalue::new_u64(a + 4),Guard::always()).unwrap();
                true
            },

            [0x4455] = |s: &mut State<TestArchWide>|
            {
                s.mnemonic(2, "C","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );

        let func = Function::new::<TestArchWide>(0, &reg, None, dec).unwrap();
        assert_eq!(func.cflow_graph.num_vertices(), 3);
        assert_eq!(func.cflow_graph.num_edges(), 2);

        let mut bb0_vx = None;
        let mut bb1_vx = None;

        for vx in func.cflow_graph.vertices() {
            match func.cflow_graph.vertex_label(vx) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    if bb.area.start == 0 {
                        assert_eq!(bb.mnemonics.len(), 2);
                        assert_eq!(bb.area, Bound::new(0, 4));
                        bb0_vx = Some(vx);
                    } else if bb.area.start == 4 {
                        assert_eq!(bb.mnemonics.len(), 1);
                        assert_eq!(bb.area, Bound::new(4, 6));
                        bb1_vx = Some(vx);
                    } else {
                        unreachable!();
                    }
                }
                Some(&ControlFlowTarget::Failed(6, _)) => {}
                _ => unreachable!(),
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some());
        assert_eq!(Some(func.entry_point_ref()), bb0_vx);
    }

    #[test]
    fn issue_51_treat_entry_point_as_incoming_edge() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test0","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(1),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(0),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(1, &reg, None, main).unwrap();
        assert_eq!(func.cflow_graph.num_vertices(), 2);
        assert_eq!(func.cflow_graph.num_edges(), 2);

        let mut bb0_vx = None;
        let mut bb1_vx = None;

        for vx in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                if bb.area.start == 0 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.area, Bound::new(0, 1));
                    bb0_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(bb.mnemonics.len(), 2);
                    assert_eq!(bb.area, Bound::new(1, 3));
                    bb1_vx = Some(vx);
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some());
        assert_eq!(Some(func.entry_point_ref()), bb1_vx);
        assert!(func.cflow_graph.edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb1_vx.unwrap(), bb0_vx.unwrap()).is_some());
    }

    #[test]
    fn issue_232_overlap_with_entry_point() {
        let main = new_disassembler!(TestArchShort =>
            [ 0, 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(2,"test01","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(2),Guard::always()).unwrap();
                true
            },
            [ 2 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test2","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u32(0),Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(1, &reg, None, main).unwrap();
        assert_eq!(func.cflow_graph.num_vertices(), 3);
        assert_eq!(func.cflow_graph.num_edges(), 3);

        let mut bb01_vx = None;
        let mut bb1_vx = None;
        let mut bb2_vx = None;

        for vx in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                if bb.area.start == 0 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.area, Bound::new(0, 2));
                    bb01_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.area, Bound::new(1, 2));
                    bb1_vx = Some(vx);
                } else if bb.area.start == 2 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.area, Bound::new(2, 3));
                    bb2_vx = Some(vx);
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }

        assert!(bb01_vx.is_some());
        assert!(bb1_vx.is_some());
        assert!(bb2_vx.is_some());
        assert_eq!(Some(func.entry_point_ref()), bb1_vx);
        assert!(func.cflow_graph.edge(bb01_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb1_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb2_vx.unwrap(), bb01_vx.unwrap()).is_some());
    }
}
