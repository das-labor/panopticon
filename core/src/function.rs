/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2017  Panopticon authors
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

#![allow(unused_variables, dead_code)]
use std::ops::{RangeFull, Range};
use std::iter::FromIterator;
use std::collections::{HashSet,HashMap};

use uuid::Uuid;
use petgraph::prelude::*;
use petgraph::graph::NodeIndices;
use petgraph::visit::{Walker,DfsPostOrder};
use {Architecture,Guard,Region,MnemonicFormatToken,Rvalue,Result,Constant,Value,Variable,Str,Statement};
use il::{Bitcode,Language,StatementIterator,CallIterator};

/// Graph of basic blocks and jumps
pub type ControlFlowGraph = Graph<CfgNode, Guard>;
/// Stable reference to a node in the `ControlFlowGraph`
pub type ControlFlowRef = NodeIndex<u32>;
/// Stable reference to an edge in the `ControlFlowGraph`
pub type ControlFlowEdge = EdgeIndex<u32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The kind of function this is, to distinguish plt stubs from regular functions.
pub enum FunctionKind {
    /// A regular function
    Regular,
    /// A PLT stub, which is a name  and an address pointing to its PLT table entry
    Stub {
        /// The import name of this stub, as found in the PLT table
        name: String,
        /// The address of this stub in the PLT table
        plt_address: u64
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct BasicBlock {
    pub mnemonics: Range<MnemonicIndex>,
    pub node: NodeIndex,
    pub area: Range<u64>,
}

impl BasicBlock {
    pub fn area(&self) -> Range<u64> { self.area.clone() }
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Mnemonic {
    pub area: Range<u64>,
    pub opcode: Str,
    pub operands: Vec<Rvalue>,
    pub format_string: Vec<MnemonicFormatToken>,
    pub statements: Range<usize>,
}

impl Mnemonic {
    pub fn new<S: Into<Str> + Sized>(a: Range<u64>, s: S) -> Mnemonic {
        Mnemonic{
            area: a,
            opcode: s.into(),
            operands: vec![],
            format_string: vec![],
            statements: 0..0,
        }
    }
}

/// Internal to `Mnemonic`
#[derive(Clone,Debug)]
pub enum Argument {
    /// Internal to `Mnemonic`
    Literal(char),
    /// Internal to `Mnemonic`
    Value {
        /// Internal to `Mnemonic`
        has_sign: bool,
        /// Internal to `Mnemonic`
        value: Value,
    },
    /// Internal to `Mnemonic`
    Pointer {
        /// Internal to `Mnemonic`
        is_code: bool,
        /// Internal to `Mnemonic`
        region: Str,
        /// Internal to `Mnemonic`
        address: Value,
    },
}

/*macro_rules! arg {
    ( { u : $val:expr } $cdr:tt ) => {
        Argument::Value{
            has_sign: false,
            value: ($val).into(),
        }
    }
    ( { s : $val:expr } $cdr:tt ) => {
        Argument::Value{
            has_sign: true,
            value: ($val).into(),
        }
    }
    ( { p : $val:expr : $bank:expr } $cdr:tt ) => {
        Argument::Pointer{
            is_code: false,
            region: ($bank).into(),
            address: ($val).into(),
        }
    }
    ( { c : $val:expr : $bank:expr } $cdr:tt ) => {
        Argument::Pointer{
            is_code: false,
            region: ($bank).into(),
            address: ($val).into(),
        }
    }
    ( ) => {}
}

arg!({ u : Variable::new("test",1,None) } "sss");
arg!({ s : Variable::new("test",1,None) } "sss");

impl Argument {
    /// format := '{' type '}'
    /// type := 'u' ':' value | # unsigned
    ///         's' ':' value | # signed
    ///         'p' ':' value ':' bank |  # data pointer
    ///         'c' ':' value ':' bank |  # code pointer
    /// value := digit+ | xdigit+ | # constant
    ///          alpha alphanum* | # variable
    /// bank := alpha alphanum*
     pub fn parse(mut j: Chars) -> Result<Vec<Argument>> {
        named!(main, tag!("{"*/

///////////////////////////////
// indexes, never constructable
// outside of function
///////////////////////////////
#[derive(Clone,Copy,Debug,PartialOrd,Ord,PartialEq,Eq,Serialize, Deserialize)]
pub struct BasicBlockIndex {
    index: usize
}

impl BasicBlockIndex {
    pub fn new(i: usize) -> BasicBlockIndex { BasicBlockIndex{ index: i } }
    pub fn index(&self) -> usize { self.index }
}

#[derive(Clone,Copy,Debug,PartialOrd,Ord,PartialEq,Eq,Serialize,Deserialize)]
pub struct MnemonicIndex {
    index: usize
}

impl MnemonicIndex {
    // indexes should never be constructable outside of function
    pub fn new(index: usize) -> MnemonicIndex { MnemonicIndex { index } }
    pub fn index(&self) -> usize { self.index }
}
//////////////////////////////////

pub struct BasicBlockIterator<'a, IL: 'a> {
    function: &'a Function<IL>,
    index: usize,
    max: usize,
}

//////////////////////////// easy
pub struct EasyBasicBlockIterator<'a, IL: 'a> {
    function: &'a Function<IL>,
    range: Range<usize>,
}

pub struct EasyMnemonicIterator<'a, IL: 'a> {
    function: &'a Function<IL>,
    pub basic_block: &'a BasicBlock,
    range: Range<usize>,
}

pub struct EasyStatementIterator<'a, IL: Language, Iterator: StatementIterator<IL>> {
    pub mnemonic: &'a Mnemonic,
    statements: Iterator::IntoIter,
}
////////////////////////////

impl<'a, IL: Language> Iterator for EasyMnemonicIterator<'a, IL>
    where &'a IL: StatementIterator<IL::Statement> {
    type Item = (&'a Mnemonic, <&'a IL as StatementIterator<IL::Statement>>::IntoIter);
    fn next(&mut self) -> Option<Self::Item> {
        match self.range.next() {
            Some(idx) => {
                let mnemonic = &self.function.mnemonics[idx];
                let statements = <&'a IL as StatementIterator<IL::Statement>>::iter_statements(&self.function.code, mnemonic.statements.clone());
                Some((mnemonic, statements))
            },
            None => None
        }
    }
}

impl<'a, IL> Iterator for EasyBasicBlockIterator<'a, IL> {
    type Item = EasyMnemonicIterator<'a, IL>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.range.next() {
            Some(idx) => {
                let basic_block = &self.function.basic_blocks[idx];
                let mnes = &basic_block.mnemonics;
                Some(EasyMnemonicIterator {
                    function: self.function,
                    basic_block,
                    range: mnes.start.index..mnes.end.index,
                })
            },
            None => None
        }
     }
}

//// get us:
////
//// for bb in f {
////   for mne in bb {
////     for statement in mne {
////     }
////   }
//// }
impl<'a, IL: Language> IntoIterator for &'a Function<IL> {
    type Item = EasyMnemonicIterator<'a, IL>;
    type IntoIter = EasyBasicBlockIterator<'a, IL>;
    fn into_iter(self) -> Self::IntoIter {
        EasyBasicBlockIterator {
            function: self,
            range: 0..self.basic_blocks().len(),
        }
    }
}

impl<'a, IL> Iterator for BasicBlockIterator<'a, IL> {
    type Item = (BasicBlockIndex,&'a BasicBlock);

    fn next(&mut self) -> Option<(BasicBlockIndex,&'a BasicBlock)> {
        if self.index <= self.max {
            let idx = BasicBlockIndex::new(self.index);
            let bb = &self.function.basic_blocks[self.index];
            self.index += 1;
            Some((idx,bb))
        } else {
            None
        }
    }
}

impl<'a, IL> ExactSizeIterator for BasicBlockIterator<'a, IL> {
    fn len(&self) -> usize {
        self.max - self.index + 1
    }
}

impl<'a, IL> DoubleEndedIterator for BasicBlockIterator<'a, IL> {
    fn next_back(&mut self) -> Option<(BasicBlockIndex,&'a BasicBlock)> {
        if self.max > 0 {
            self.max -= 1;
            let idx = BasicBlockIndex::new(self.max);
            let bb = &self.function.basic_blocks[self.max];

            Some((idx,bb))
        } else {
            None
        }
    }
}

pub struct MnemonicIterator<'a, IL: 'a> {
    function: &'a Function<IL>,
    index: usize,
    max: usize,
}

impl<'a, IL> Iterator for MnemonicIterator<'a, IL> {
    type Item = (MnemonicIndex,&'a Mnemonic);

    fn next(&mut self) -> Option<(MnemonicIndex,&'a Mnemonic)> {
        if self.index <= self.max {
            let idx = MnemonicIndex::new(self.index);
            let mne = &self.function.mnemonics[self.index];

            self.index += 1;
            Some((idx,mne))
        } else {
            None
        }
    }
}

impl<'a, IL> ExactSizeIterator for MnemonicIterator<'a, IL> {
    fn len(&self) -> usize {
        self.max - self.index + 1
    }
}

pub trait IntoStatementRange<IL> {
    fn into_statement_range(self, func: &Function<IL>) -> Range<usize>;
}

impl<IL: Language> IntoStatementRange<IL> for RangeFull {
    fn into_statement_range(self, func: &Function<IL>) -> Range<usize> {
        0..func.code.len()
    }
}

impl<IL> IntoStatementRange<IL> for Range<usize> {
    fn into_statement_range(self, _: &Function<IL>) -> Range<usize> {
        self
    }
}

impl<IL> IntoStatementRange<IL> for BasicBlockIndex {
    fn into_statement_range(self, func: &Function<IL>) -> Range<usize> {
        let bb = &func.basic_blocks[self.index()];
        bb.into_statement_range(func)
    }
}

impl<IL> IntoStatementRange<IL> for MnemonicIndex {
    fn into_statement_range(self, func: &Function<IL>) -> Range<usize> {
        let mne = &func.mnemonics[self.index()];
        mne.into_statement_range(func)
    }
}

impl<'a, IL> IntoStatementRange<IL> for &'a Mnemonic {
    fn into_statement_range(self, _: &Function<IL>) -> Range<usize> {
        self.statements.clone()
    }
}

impl<'a, IL> IntoStatementRange<IL> for &'a BasicBlock {
    fn into_statement_range(self, func: &Function<IL>) -> Range<usize> {
        let start = func.mnemonics[self.mnemonics.start.index()].statements.start;
        let end = func.mnemonics[self.mnemonics.end.index() - 1].statements.end;
        start..end
    }
}

pub trait IntoMnemonicRange<'a, IL> {
    fn into_mnemonic_range(self, func: &'a Function<IL>) -> Range<usize>;
}

impl<'a, IL: 'a> IntoMnemonicRange<'a, IL> for RangeFull {
    fn into_mnemonic_range(self, func: &'a Function<IL>) -> Range<usize> {
        0..func.mnemonics.len()
    }
}

impl<'a, IL: 'a> IntoMnemonicRange<'a, IL> for Range<usize> {
    fn into_mnemonic_range(self, _: &'a Function<IL>) -> Range<usize> {
        self
    }
}

impl<'a, IL: 'a> IntoMnemonicRange<'a, IL> for Range<MnemonicIndex> {
    fn into_mnemonic_range(self, _: &'a Function<IL>) -> Range<usize> {
        self.start.index()..self.end.index()
    }
}

impl<'a, IL> IntoMnemonicRange<'a, IL> for BasicBlockIndex {
    fn into_mnemonic_range(self, func: &'a Function<IL>) -> Range<usize> {
        let bb = &func.basic_blocks[self.index()];
        bb.into_mnemonic_range(func)
    }
}

impl<'a, IL> IntoMnemonicRange<'a, IL> for (BasicBlockIndex, &'a BasicBlock) {
    fn into_mnemonic_range(self, func: &'a Function<IL>) -> Range<usize> {
        let bb = &func.basic_blocks[self.0.index()];
        bb.into_mnemonic_range(func)
    }
}

impl<'a, IL> IntoMnemonicRange<'a, IL> for &'a BasicBlock {
    fn into_mnemonic_range(self, _: &'a Function<IL>) -> Range<usize> {
        let start = self.mnemonics.start.index();
        let end = self.mnemonics.end.index();

        start..end
    }
}

impl<'a, IL> IntoMnemonicRange<'a, IL> for MnemonicIndex {
    fn into_mnemonic_range(self, _: &'a Function<IL>) -> Range<usize> {
        self.index()..(self.index() + 1)
    }
}

pub struct IndirectJumps<'a> {
    pub graph: &'a Graph<CfgNode,Guard>,
    pub iterator: NodeIndices<u32>,
}

impl<'a> Iterator for IndirectJumps<'a> {
    type Item = Variable;

    fn next(&mut self) -> Option<Variable> {
        while let Some(idx) = self.iterator.next() {
            match self.graph.node_weight(idx) {
                Some(&CfgNode::Value(Value::Variable(ref v))) => {
                    return Some(v.clone());
                }
                _ => {}
            }
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CfgNode {
    BasicBlock(BasicBlockIndex),
    Value(Value),
}

/// A function is a generic container for an Intermediate Language lifted from raw machine code
#[derive(Debug, Serialize, Deserialize)]
pub struct Function<IL = Bitcode> {
    /// The name of this function
    pub name: Str,
    uuid: Uuid,
    // sort by rev. post order
    code: IL,
    // sort by rev. post order
    basic_blocks: Vec<BasicBlock>,
    // sort by area.start
    mnemonics: Vec<Mnemonic>,
    cflow_graph: Graph<CfgNode,Guard>,
    entry_point: BasicBlockIndex,
    kind: FunctionKind,
    aliases: Vec<String>,
}

////////////////////////////////////
// Generic Function construction
////////////////////////////////////
impl<IL: Language + Default> Function<IL> {
    /// New function starting at `start`, with name `name`,
    /// inside memory region `region` and UUID `uuid`.
    pub fn with_uuid<A: Architecture>(start: u64, uuid: &Uuid, region: &Region, name: Option<String>, init: A::Configuration) -> Result<Function<IL>> {
        let mut f = Function::<IL>::new::<A>(init, start, region, name.map(|name| ::std::borrow::Cow::Owned(name)))?;
        f.uuid = uuid.clone();
        Ok(f)
    }
    /// Create and start disassembling a new function with `name`,
    /// inside memory `region`, starting at entry point `start`, with a random UUID.
    pub fn new<A: Architecture>(init: A::Configuration, start: u64, region: &Region, name: Option<Str>) -> Result<Self>
    {
        let mut mnemonics: Vec<(Mnemonic, Vec<IL::Statement>)> = Vec::new();
        let mut by_source = HashMap::new();
        let mut by_destination = HashMap::new();
        let mut func: Function<IL> = Function {
            name: name.unwrap_or(format!("func_{:x}", start).into()),
            uuid: Uuid::new_v4(),
            code: IL::default(),
            basic_blocks: Vec::new(),
            mnemonics: Vec::new(),
            cflow_graph: Graph::new(),
            entry_point: BasicBlockIndex::new(0),
            kind: FunctionKind::Regular,
            aliases: vec![],
        };

        disassemble::<A, IL::Statement>(init, vec![start], region, &mut mnemonics, &mut by_source, &mut by_destination)?;
        func.assemble(start, mnemonics, by_source, by_destination)?;

        Ok(func)
    }

    /// FIXME: ditto this clones and allocates the blocks as well
    pub fn rewrite<'a, F>(&'a mut self, f: F) -> Result<()>
        where F: FnOnce(&mut [Vec<(Mnemonic,Vec<IL::Statement>)>]) -> Result<()>,
              for<'b> &'b IL: StatementIterator<IL::Statement>
    {
        let mut blocks = {
            let mut blocks = Vec::new();
            for bb in self.basic_blocks.iter() {
                let mut mnemonics = Vec::new();
                for (_, mne) in self.mnemonics(bb.mnemonics.clone()) {
                    let statements = self.code.iter_statements(mne.statements.clone()).collect();
                    mnemonics.push((mne.clone(), statements));
                }
                blocks.push(mnemonics);
            }
            blocks
        };

        f(blocks.as_mut_slice())?;

        let mut code = IL::default(); //Bitcode::with_capacity(self.code.len(), self.code.number_of_strings().unwrap_or(0));
        let mne_cnt = blocks.iter().map(|x| x.len()).sum();
        let mut mnemonics = Vec::with_capacity(mne_cnt);
        let mut new_mne_ranges = Vec::with_capacity(blocks.len());

        for (bb_idx,mnes) in blocks.into_iter().enumerate() {
            let fst_mne = mnemonics.len();
            let mut prev_addr = None;

            for (mut mne, stmts) in mnes.into_iter() {
                if let Some(s) = prev_addr {
                    if s != mne.area.start {
                        return Err(format!("Non-continuous basic block #{}: gap between {:#x} and {:#x}",bb_idx,s,mne.area.start).into());
                    }
                }

                prev_addr = Some(mne.area.end);
                let start = code.len();
                let mut end = start;
                for statement in stmts {
                    end += code.push(statement)?;
                }
                //mne.statements = code.append(stmts.into_iter())?;
                mne.statements = start..end;
                mnemonics.push(mne);
            }

            new_mne_ranges.push(fst_mne..mnemonics.len());
        }

        for (idx,rgn) in new_mne_ranges.into_iter().enumerate() {
            self.basic_blocks[idx].mnemonics = MnemonicIndex::new(rgn.start)..MnemonicIndex::new(rgn.end);
        }

        self.mnemonics = mnemonics;
        self.code = code;

        Ok(())
    }

    /// FIXME: this clones and allocates the mnemonics for not really good reasons, only to send into disassemble;
    /// refactor both to fix this behavior
    pub fn extend<A: Architecture>(&mut self, init: A::Configuration, region: &Region) -> Result<()>
        where for<'b> &'b IL: StatementIterator<IL::Statement>
    {
        let mut mnemonics = self.mnemonics.iter().map(|mne| {
            let stmts = self.statements(mne.statements.clone()).collect::<Vec<_>>();
            (mne.clone(),stmts)
        }).collect::<Vec<_>>();
        let mut by_source = HashMap::new();
        let mut by_destination = HashMap::new();
        let mut starts = Vec::new();

        for e in self.cflow_graph.edge_references() {
            let src = match self.cflow_graph.node_weight(e.source()) {
                Some(&CfgNode::BasicBlock(bb_idx)) => {
                    let bb = &self.basic_blocks[bb_idx.index()];
                    let mne = &self.mnemonics[bb.mnemonics.end.index() - 1];
                    mne.area.start
                }
                _ => unreachable!()
            };
            let dst = match self.cflow_graph.node_weight(e.target()) {
                Some(&CfgNode::BasicBlock(bb_idx)) => {
                    let bb = &self.basic_blocks[bb_idx.index()];
                    let mne = &self.mnemonics[bb.mnemonics.start.index()];
                    Value::val(mne.area.start,64)?
                }
                Some(&CfgNode::Value(ref val)) => {
                    val.clone()
                }
                None => unreachable!()
            };

            by_source.entry(src).or_insert_with(|| Vec::new()).push((dst.clone(),e.weight().clone()));

            if let Value::Constant(Constant{ value,.. }) = dst {
                by_destination.entry(value).or_insert_with(|| Vec::new()).push((Value::val(src,64)?,e.weight().clone()));
                starts.push(value);
            }
        }

        let entry = self.entry_address();
        disassemble::<A, IL::Statement>(init,starts, region, &mut mnemonics, &mut by_source, &mut by_destination)?;
        Function::assemble(self,entry,mnemonics,by_source,by_destination)
    }

    /////////////////////////////////
    // Private
    /////////////////////////////////
    fn assemble(&mut self, entry: u64,
                mut mnemonics: Vec<(Mnemonic, Vec<IL::Statement>)>,
                by_source: HashMap<u64,Vec<(Value,Guard)>>,
                by_destination: HashMap<u64,Vec<(Value,Guard)>>) -> Result<()>
    {

        let mut basic_blocks = Vec::<BasicBlock>::new();
        let mut idx = 0;

        // Partition mnemonics into basic blocks
        while idx < mnemonics.len() {
            if mnemonics.len() - idx > 1 {
                let next_bb = mnemonics
                    .as_slice()[idx..].windows(2)
                    .position(|x| is_basic_block_boundary(&x[0].0,&x[1].0,entry,&by_source,&by_destination))
                    .map(|x| x + 1 + idx)
                    .unwrap_or(mnemonics.len());
                let bb = BasicBlock{
                    mnemonics: MnemonicIndex::new(idx)..MnemonicIndex::new(next_bb),
                    area: mnemonics[idx].0.area.start..mnemonics[next_bb - 1].0.area.end,
                    node: NodeIndex::new(0),
                };

                basic_blocks.push(bb);
                idx = next_bb;
            } else {
                let bb = BasicBlock{
                    mnemonics: MnemonicIndex::new(idx)..MnemonicIndex::new(mnemonics.len()),
                    area: mnemonics[idx].0.area.start..mnemonics[idx].0.area.end,
                    node: NodeIndex::new(0),
                };

                basic_blocks.push(bb);
                break;
            }
        }

        // Build control flow graph
        let mut cfg = Graph::<CfgNode,Guard>::with_capacity(basic_blocks.len(),3*basic_blocks.len() / 2);

        for (i,bb) in basic_blocks.iter_mut().enumerate() {
            bb.node = cfg.add_node(CfgNode::BasicBlock(BasicBlockIndex::new(i)));
        }

        for bb in basic_blocks.iter() {
            let last_mne = &mnemonics[bb.mnemonics.end.index() - 1].0;
            if let Some(ct) = by_source.get(&last_mne.area.start) {
                for &(ref val,ref guard) in ct.iter() {
                    match val {
                        &Value::Constant(Constant{ value,.. }) => {
                            if let Ok(pos) = basic_blocks.binary_search_by_key(&value,|bb| bb.area.start) {
                                cfg.update_edge(bb.node,basic_blocks[pos].node,guard.clone());
                            } else {
                                let n = cfg.add_node(CfgNode::Value(val.clone()));
                                cfg.update_edge(bb.node,n,guard.clone());
                            }
                        }
                        val => {
                            let n = cfg.add_node(CfgNode::Value(val.clone()));
                            cfg.update_edge(bb.node,n,guard.clone());
                        }
                    }
                }
            }
        }

        let entry_idx = basic_blocks
            .iter().position(|x| x.area.start == entry)
            .ok_or("Internal error: no basic block at the entry point")?;

        // Generate bitcode
        let mut bitcode = IL::default();
        let mut statement_ranges = vec![0..0; mnemonics.len()];
        debug!("Generating bitcode for {}", self.name);
        {
            let postorder = DfsPostOrder::new(&cfg, basic_blocks[entry_idx].node).iter(&cfg);
            for n in postorder {
                if let Some(&CfgNode::BasicBlock(idx)) = cfg.node_weight(n) {
                    let bb = &basic_blocks[idx.index()];
                    let sl = bb.mnemonics.start.index()..bb.mnemonics.end.index();
                    let mnes_and_instrs= &mut mnemonics.as_mut_slice()[sl];
                    for (off, &mut (_, ref mut instr)) in mnes_and_instrs.into_iter().enumerate() {
                        let start = bitcode.len();
                        let nstatements = instr.len();
                        let mut end = start;
                        for statement in instr.drain(..) {
                            end += bitcode.push(statement)?;
                        }
                        debug!("Added {} statements, at range: {:?}", nstatements, start..end);
                        statement_ranges[bb.mnemonics.start.index() + off] = start..end;
                    }
                }
            }
        }

        self.code = bitcode;
        self.basic_blocks = basic_blocks;
        self.mnemonics = mnemonics.into_iter().enumerate().map(|(idx,(mut mne,_))| {
            // we don't need to clone this, we construct the owned vector above
            mne.statements = statement_ranges[idx].clone();
            mne
        }).collect();
        self.cflow_graph = cfg;
        self.entry_point = BasicBlockIndex::new(entry_idx);
        // we erase the functions name this way; need to keep track of whether we actually have a name or not
        // if entry != function.start_address() { function.name = format!("func_{:x}",entry).into() };
        Ok(())
    } // end assemble
} // end Function

fn disassemble<A, S>(init: A::Configuration, starts: Vec<u64>, region: &Region,
                     mnemonics: &mut Vec<(Mnemonic,Vec<S>)>,
                     by_source: &mut HashMap<u64,Vec<(Value,Guard)>>,
                     by_destination: &mut HashMap<u64,Vec<(Value,Guard)>>) -> Result<()>
    where A: Architecture,
          S: From<Statement>,
{
    let mut todo = HashSet::<u64>::from_iter(starts.into_iter());

    while let Some(addr) = todo.iter().next().cloned() {
        assert!(todo.remove(&addr));

        match mnemonics.binary_search_by_key(&addr,|&(ref x,_)| x.area.start) {
            // Already disassembled here
            Ok(pos) => {
                let mne = &mnemonics[pos].0;

                if mne.area.start != addr {
                    error!("{:#x}: Jump inside mnemonic {} at {:#x}",addr,mne.opcode,mne.area.start);
                }
            }
            // New mnemonic
            Err(pos) => {
                let maybe_match = A::decode(region,addr,&init);

                match maybe_match {
                    Ok(match_st) => {
                        // Matched mnemonics
                        if match_st.mnemonics.is_empty() {
                            error!("{:#x}: Unrecognized instruction",addr);
                        } else {
                            for mne in match_st.mnemonics {
                                debug!(
                                    "{:x}: {}",
                                    mne.area.start,
                                    mne.opcode
                                    //match_st.tokens
                                );
                                let this_mne = Mnemonic{
                                    area: mne.area.start..mne.area.end,
                                    opcode: mne.opcode.into(),
                                    operands: mne.operands,
                                    format_string: mne.format_string,
                                    statements: 0..0,
                                };
                                let stmts = mne.instructions.into_iter().map(|s| s.into()).collect::<Vec<S>>();
                                mnemonics.insert(pos,(this_mne,stmts));
                            }
                        }

                        // New control transfers
                        for (origin, tgt, gu) in match_st.jumps {
                            debug!("jump to {:?}", tgt);
                            match tgt {
                                Rvalue::Constant { value, size } => {
                                    by_source.entry(origin).or_insert(Vec::new()).push((Value::val(value,size)?, gu.clone()));
                                    by_destination.entry(value).or_insert(Vec::new()).push((Value::val(origin,64)?, gu));
                                    todo.insert(value);
                                }
                                Rvalue::Variable{ name, size,.. } => {
                                    by_source.entry(origin).or_insert(Vec::new()).push((Value::var(name,size,None)?, gu));
                                }
                                Rvalue::Undefined => {
                                    by_source.entry(origin).or_insert(Vec::new()).push((Value::undef(), gu));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("{:#x} Failed to disassemble: {}",addr, e);
                    }
                }
            }
        }
    }

    Ok(())
}

fn is_basic_block_boundary(a: &Mnemonic, b: &Mnemonic, entry: u64,
                           by_source: &HashMap<u64,Vec<(Value,Guard)>>,
                           by_destination: &HashMap<u64,Vec<(Value,Guard)>>) -> bool {
    // if next mnemonics aren't adjacent
    let mut new_bb = a.area.end != b.area.start;

    // or any following jumps aren't to adjacent mnemonics
    new_bb |= by_source
        .get(&a.area.start)
        .unwrap_or(&Vec::new())
        .iter()
        .any(|&(ref opt_dest, _)| {
            if let &Value::Constant(Constant{ value, .. }) = opt_dest { value != b.area.start } else { false }
        });

    // or any jumps pointing to the next that aren't from here
    new_bb |= by_destination
        .get(&b.area.start)
        .unwrap_or(&Vec::new())
        .iter()
        .any(
            |&(ref opt_src, _)| if let &Value::Constant(Constant{ value, .. }) = opt_src {
                value != a.area.start
            } else {
                false
            }
        );

    // or the entry point does not point here
    new_bb |= b.area.start == entry;

    new_bb
}

//////////////////////////////////////
// Generic IL iteration-based methods
// allows iteration to get inlined!
// (I know its terrifying, but its worth it)
//////////////////////////////////////

impl<IL: Language> Function<IL> {
    /// Iterate every IL statement in the given `range`
    pub fn statements<'a, Idx: IntoStatementRange<IL> + Sized>(&'a self, range: Idx) -> <&'a IL as StatementIterator<IL::Statement>>::IntoIter where &'a IL: StatementIterator<IL::Statement> {
        let rgn = range.into_statement_range(self);
        self.code.iter_statements(rgn)
    }
}

////////////////////////////////////////
// specialized methods for different IL
////////////////////////////////////////

impl<'a, IL: 'a> Function<IL> where &'a IL: CallIterator{
    pub fn iter_calls(&'a self) -> <&'a IL as CallIterator>::Iter {
        self.code.iter_calls()
    }
}

////////////////////////////////////////
// standard methods for every Function
// irrespective of the underlying IL
// i.e. - requires no special knowledge of IL
////////////////////////////////////////
impl<IL> Function<IL> {
    /// Adds the alias `name` to this functions known aliases
    pub fn add_alias(&mut self, name: String) {
        self.aliases.push(name)
    }
    /// Gets the name of this functino
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Gets the uuid of this function
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    /// Sets this functions uuid
    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }

    /// Sets this function's plt stub entry at `plt_address`, as `name`.
    ///
    /// **Note** This will alter the function's kind from `Regular` to `Stub`,
    /// and will also change move its canonical name into aliases.
    pub fn set_plt(&mut self, name: &str, plt_address: u64) {
        let old_name = self.name.clone().to_string();
        self.aliases.push(old_name);
        self.name = format!("{}@plt", name).into();
        self.kind = FunctionKind::Stub { name: name.to_string(), plt_address };
    }

    /// Returns the lowest address contained in this function
    pub fn first_address(&self) -> u64 {
        self.basic_blocks[0].area().start
    }

    /// Returns the end address of the highest basic block in this function
    pub fn last_address(&self) -> u64 {
        let mut end = self.basic_blocks[0].area().end;
        for (_, bb) in self.basic_blocks() {
            end = ::std::cmp::max(bb.area().end, end);
        }
        end
    }

    /// Whether the given address is contained within this function
    pub fn contains(&self, address: u64) -> bool {
        for (_, bb) in self.basic_blocks() {
            if bb.area.start >= address && address < bb.area.end {
                return true
            }
        }
        false
    }

    /// Returns this functions FunctionKind
    pub fn kind(&self) -> &FunctionKind {
        &self.kind
    }

    /// Returns this functions known name aliases (names pointing to the same start address)
    pub fn aliases(&self) -> &[String] {
        self.aliases.as_slice()
    }

    /// Returns the functions basic block graph in graphivz's DOT format. Useful for debugging.
    pub fn to_dot(&self) -> String {
        use petgraph::dot::Dot;
        format!("{:?}", Dot::new(&self.cflow_graph))
    }

    /// Gets the index of the entry point of this function
    pub fn entry_point(&self) -> BasicBlockIndex { self.entry_point }

    /// Get the control flow graph of this function
    pub fn cfg(&self) -> &Graph<CfgNode, Guard> {
        &self.cflow_graph
    }
    // @flanfly due to the way new function works, it bookkeeps internal state w.r.t.
    // graph + mnemonic + basic block vectors; hence mutating the cflow graph directly should be banned completely
    /// Get a mutable reference to this functions control flow graph
    pub fn cfg_mut(&mut self) -> &mut Graph<CfgNode, Guard> {
        &mut self.cflow_graph
    }

    /// Returns the address of this functions entry point
    pub fn entry_address(&self) -> u64 {
        let e = self.entry_point().index();
        self.basic_blocks[e].area().start
    }

    /// Gets the basic block at the given index
    pub fn basic_block(&self, idx: BasicBlockIndex) -> &BasicBlock {
        &self.basic_blocks[idx.index]
    }

    /// Gets the mnemonic at this given index
    pub fn mnemonic(&self, idx: MnemonicIndex) -> &Mnemonic {
        &self.mnemonics[idx.index]
    }

    /// Returns an iterator over this functions mnemonics, using `idx`
    pub fn mnemonics<'a, Idx: IntoMnemonicRange<'a, IL> + Sized>(&'a self, idx: Idx) -> MnemonicIterator<'a, IL> {
        let idx = idx.into_mnemonic_range(self);
        MnemonicIterator {
            function: self,
            index: idx.start,
            max: idx.end - 1
        }
    }

    /// Returns an iterator over every basic block in this function, in post order
    pub fn basic_blocks(&self) -> BasicBlockIterator<IL> {
        BasicBlockIterator {
            function: self,
            index: 0,
            max: self.basic_blocks.len() - 1
        }
    }

    pub fn indirect_jumps<'a>(&'a self) -> IndirectJumps<'a> {
        IndirectJumps{
            graph: &self.cflow_graph,
            iterator: self.cflow_graph.node_indices()
        }
    }

    pub fn resolve_indirect_jump(&mut self, var: Variable, val: Constant) -> bool {
        let var = Value::Variable(var);

        for n in self.cflow_graph.node_indices() {
            match self.cflow_graph.node_weight_mut(n) {
                Some(&mut CfgNode::Value(ref mut value)) if *value == var => {
                    *value = Value::Constant(val);
                    return true;
                }
                _ => {}
            }
        }
        false
    }
}
