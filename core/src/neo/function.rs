use std::ops::{RangeFull,Range};
use std::iter::FromIterator;
use std::cmp;
use std::usize;
use uuid::Uuid;
use petgraph::Graph;
use petgraph::graph::{NodeIndices,NodeIndex};
use petgraph::visit::{Walker,DfsPostOrder};
use {Architecture,Guard,Region,MnemonicFormatToken,Rvalue,Lvalue};
use neo::{Str,Result,Statement,Bitcode,Value,BitcodeIter,Constant,Operation,Variable,Endianess};

mod core {
    pub use ::mnemonic::Mnemonic;
    pub use ::il::Operation;
    pub use ::il::Statement;
    pub use ::il::Endianess;
    pub use ::program::CallTarget;
}

use std::collections::{HashSet,HashMap};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BasicBlock {
    pub mnemonics: Range<MnemonicIndex>,
    pub node: NodeIndex,
    pub area: Range<u64>,
    pub statements: Range<usize>,
}

impl BasicBlock {
    pub fn area(&self) -> Range<u64> { self.area.clone() }
}

#[derive(Clone,Debug)]
pub struct Mnemonic {
    pub area: Range<u64>,
    pub opcode: Str,
    pub operands: Vec<Rvalue>,
    pub format_string: Vec<MnemonicFormatToken>,
    pub num_statements: usize,
}

impl Mnemonic {
    pub fn new<S: Into<Str> + Sized>(a: Range<u64>, s: S) -> Mnemonic {
        Mnemonic{
            area: a,
            opcode: s.into(),
            operands: vec![],
            format_string: vec![],
            num_statements: 0,
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

#[derive(Clone,Copy,Debug,PartialOrd,Ord,PartialEq,Eq)]
pub struct BasicBlockIndex {
    index: usize
}

impl BasicBlockIndex {
    pub fn new(i: usize) -> BasicBlockIndex { BasicBlockIndex{ index: i } }
    pub fn index(&self) -> usize { self.index }
}

pub struct BasicBlockIterator<'a> {
    function: &'a Function,
    index: usize,
    max: usize,
}

impl<'a> Iterator for BasicBlockIterator<'a> {
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

impl<'a> ExactSizeIterator for BasicBlockIterator<'a> {
    fn len(&self) -> usize {
        self.max - self.index + 1
    }
}

impl<'a> DoubleEndedIterator for BasicBlockIterator<'a> {
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

#[derive(Clone,Copy,Debug,PartialOrd,Ord,PartialEq,Eq)]
pub struct MnemonicIndex {
    index: usize
}

impl MnemonicIndex {
    pub fn new(i: usize) -> MnemonicIndex { MnemonicIndex{ index: i } }
    pub fn index(&self) -> usize { self.index }
}

pub struct MnemonicIterator<'a> {
    function: &'a Function,
    index: usize,
    max: usize,
}

impl<'a> Iterator for MnemonicIterator<'a> {
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

impl<'a> ExactSizeIterator for MnemonicIterator<'a> {
    fn len(&self) -> usize {
        self.max - self.index + 1
    }
}

pub trait IntoStatementRange {
    fn into_statement_range(self, func: &Function) -> Range<usize>;
}

impl IntoStatementRange for RangeFull {
    fn into_statement_range(self, func: &Function) -> Range<usize> {
        0..func.bitcode.num_bytes()
    }
}

impl IntoStatementRange for Range<usize> {
    fn into_statement_range(self, _: &Function) -> Range<usize> {
        self
    }
}

impl IntoStatementRange for BasicBlockIndex {
    fn into_statement_range(self, func: &Function) -> Range<usize> {
        debug!("into rgn: {}",self.index());
        let bb = &func.basic_blocks[self.index()];
        bb.into_statement_range(func)
    }
}

impl<'a> IntoStatementRange for &'a BasicBlock {
    fn into_statement_range(self, _: &Function) -> Range<usize> {
        self.statements.clone()
    }
}

pub trait IntoMnemonicRange: Debug {
    fn into_mnemonic_range(self, func: &Function) -> Range<usize>;
}

impl IntoMnemonicRange for RangeFull {
    fn into_mnemonic_range(self, func: &Function) -> Range<usize> {
        0..func.mnemonics.len()
    }
}

impl IntoMnemonicRange for Range<usize> {
    fn into_mnemonic_range(self, _: &Function) -> Range<usize> {
        self
    }
}

impl IntoMnemonicRange for Range<MnemonicIndex> {
    fn into_mnemonic_range(self, _: &Function) -> Range<usize> {
        self.start.index()..self.end.index()
    }
}

impl IntoMnemonicRange for BasicBlockIndex {
    fn into_mnemonic_range(self, func: &Function) -> Range<usize> {
        let bb = &func.basic_blocks[self.index()];
        bb.into_mnemonic_range(func)
    }
}

impl<'a> IntoMnemonicRange for &'a BasicBlock {
    fn into_mnemonic_range(self, _: &Function) -> Range<usize> {
        let start = self.mnemonics.start.index();
        let end = self.mnemonics.end.index();

        start..end
    }
}

impl IntoMnemonicRange for MnemonicIndex {
    fn into_mnemonic_range(self, _: &Function) -> Range<usize> {
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

#[derive(Debug)]
pub enum CfgNode {
    BasicBlock(BasicBlockIndex),
    Value(Value),
}

#[derive(Debug)]
pub struct Function {
    pub name: Str,
    uuid: Uuid,
    // sort by rev. post order
    bitcode: Bitcode,
    // sort by rev. post order
    basic_blocks: Vec<BasicBlock>,
    // sort by area.start
    mnemonics: Vec<Mnemonic>,
    cflow_graph: Graph<CfgNode,Guard>,
    entry_point: BasicBlockIndex,
}

impl Function {
	// disassembly
    pub fn new<A: Architecture>(init: A::Configuration, start: u64, region: &Region, name: Option<Str>) -> Result<Function>
    where A: Debug, A::Configuration: Debug {
        let mut mnemonics = Vec::new();
        let mut by_source = HashMap::new();
        let mut by_destination = HashMap::new();
        let mut func = Function{
            name: name.unwrap_or("(none)".into()),
            uuid: Uuid::new_v4(),
            bitcode: Bitcode::default(),
            basic_blocks: Vec::new(),
            mnemonics: Vec::new(),
            cflow_graph: Graph::new(),
            entry_point: BasicBlockIndex::new(0),
        };

        disassemble::<A>(init,vec![start],region, &mut mnemonics, &mut by_source, &mut by_destination)?;
        assemble_function(&mut func, start, mnemonics, by_source, by_destination)?;

        Ok(func)
    }

    pub fn extend<A: Architecture>(&mut self, init: A::Configuration, region: &Region) -> Result<()>
    where A: Debug, A::Configuration: Debug {
        use petgraph::visit::EdgeRef;

        let mut mnemonics = self.basic_blocks.iter().flat_map(|bb| {
            let mut stmt_idx = bb.statements.start;

            (bb.mnemonics.start.index()..bb.mnemonics.end.index()).map(|mne_idx| {
                let mne = &self.mnemonics[mne_idx];
                let stmt_rgn = stmt_idx..(stmt_idx + mne.num_statements);
                let stmts = self.bitcode.iter_range(stmt_rgn).collect::<Vec<_>>();

                stmt_idx += self.mnemonics[mne_idx].num_statements;
                (mne.clone(),stmts)
            }).collect::<Vec<_>>()
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
        disassemble::<A>(init,starts, region, &mut mnemonics, &mut by_source, &mut by_destination)?;
        assemble_function(self,entry,mnemonics,by_source,by_destination)
    }

    // getter
    pub fn entry_point(&self) -> BasicBlockIndex { self.entry_point }

    pub fn mnemonics<'a,Idx: IntoMnemonicRange + Sized>(&'a self, idx: Idx) -> MnemonicIterator<'a> {
        let idx = idx.into_mnemonic_range(self);
        MnemonicIterator{
            function: self,
            index: idx.start,
            max: idx.end - 1
        }
    }

    pub fn basic_blocks<'a>(&'a self) -> BasicBlockIterator<'a> {
        BasicBlockIterator{
            function: self,
            index: 0,
            max: self.basic_blocks.len() - 1
        }
    }

    pub fn cflow_graph<'a>(&'a self) -> &'a Graph<CfgNode,Guard> {
        &self.cflow_graph
    }

    pub fn basic_block<'a>(&'a self, idx: BasicBlockIndex) -> &'a BasicBlock {
        &self.basic_blocks[idx.index]
    }

    pub fn mnemonic<'a>(&'a self, idx: MnemonicIndex) -> &'a Mnemonic {
        &self.mnemonics[idx.index]
    }

    // iters
    pub fn statements<Idx: IntoStatementRange + Sized>(&self, rgn: Idx) -> BitcodeIter {
        let rgn = rgn.into_statement_range(self);
        debug!("read statements {:?}",rgn);
        self.bitcode.iter_range(rgn)
    }

    // aux
    pub fn first_address(&self) -> u64 {
        self.basic_blocks[0].area().start
    }

    pub fn entry_address(&self) -> u64 {
        let e = self.entry_point().index();
        self.basic_blocks[e].area().start
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

    pub fn rewrite_mnemonics<F: FnMut(&mut Statement<Value>) -> Result<()> + Sized>(&mut self, basic_block: BasicBlockIndex, mut func: F) -> Result<()> {
        debug!("start func rewrite of {:?}",basic_block);

        let mut bb_offset = 0;
        let mut stmt_offset = 0isize;

        {
            let bb = &self.basic_blocks[basic_block.index()];

            for mne_idx in bb.mnemonics.start.index()..bb.mnemonics.end.index() {
                let mne = &mut self.mnemonics[mne_idx];

                debug!("mne {} at {:#x}",mne.opcode,mne.area.start);
                if mne.num_statements == 0 {
                    debug!("skip this mnemonic");
                    continue;
                }

                let stmt_idx = (bb.statements.start as isize + stmt_offset) as usize;
                debug!("from {:?}",stmt_idx..(stmt_idx + mne.num_statements));
                let new_rgn = self.bitcode.rewrite(stmt_idx..(stmt_idx + mne.num_statements),&mut func)?;

                let new_stmt_num = new_rgn.end - new_rgn.start;
                let offset = new_stmt_num as isize - mne.num_statements as isize;

                debug!("...to {:?}",new_rgn);
                bb_offset += offset;
                mne.num_statements = new_stmt_num;
                stmt_offset += new_stmt_num as isize;
            }
        }

        if bb_offset != 0 {
            for bb_idx in (basic_block.index() + 1)..self.basic_blocks.len() {
                let bb = &mut self.basic_blocks[bb_idx];

                bb.statements.start = (bb.statements.start as isize + bb_offset) as usize;
                bb.statements.end = (bb.statements.end as isize + bb_offset) as usize;
            }

            let bb_stmt = &mut self.basic_blocks[basic_block.index()].statements;
            bb_stmt.end = (bb_stmt.end as isize + bb_offset) as usize;
        }

        Ok(())
    }

    pub fn prepend_mnemonic<S: Into<Str> + Sized>(&mut self, basic_block: BasicBlockIndex, opcode: S, stmts: Vec<Statement<Value>>) -> Result<()> {
        let opcode: Str = opcode.into();
        debug!("prepend mne {} in {:?}",opcode,basic_block);

        let stmt_rgn = {
            let bb = &self.basic_blocks[basic_block.index()];

            if bb.mnemonics.end == bb.mnemonics.start {
                return Err("Internal error: empty basic block".into());
            }

            let rgn = bb.into_statement_range(&*self);
            let stmts_pos = rgn.start;/*if rgn.end == rgn.start {
                debug!("{:?} is empty: {:?}",basic_block,rgn);
                unreachable!();
                let mut ret = None;

                for bb in (0..basic_block.index()).rev() {
                    let bb = &self.basic_blocks[bb];
                    let rgn = bb.into_statement_range(&*self);

                    if rgn.end != rgn.start {
                        ret = Some(rgn.end);
                        break;
                    }
                }

                if ret == None {
                    for bb in (basic_block.index() + 1)..self.basic_blocks.len() {
                        let bb = &self.basic_blocks[bb];
                        let rgn = bb.into_statement_range(&*self);

                        if rgn.end != rgn.start {
                            ret = Some(rgn.start);
                            break;
                        }
                    }
                }

                if let Some(s) = ret {
                    s
                } else {
                    0
                }
            } else {
                rgn.start
            };*/

            debug!("prepend mne at {} in {:?} {}: {:?}",stmts_pos,basic_block,opcode,stmts);
            self.bitcode.insert(stmts_pos,stmts)?
        };

        let mne_idx = self.basic_blocks[basic_block.index()].mnemonics.start;
        let stmt_idx = stmt_rgn.start;

        self.shift_mnemonics(mne_idx,1);
        self.shift_statements(BasicBlockIndex::new(basic_block.index() + 1),(stmt_rgn.end - stmt_rgn.start) as isize);

        let bb = &mut self.basic_blocks[basic_block.index()];
        let addr = bb.area.start;
        let mne = Mnemonic{
            opcode: opcode,
            area: addr..addr,
            format_string: Vec::default(),
            operands: Vec::default(),
            num_statements: stmt_rgn.end - stmt_rgn.start,
        };

        self.mnemonics.insert(bb.mnemonics.start.index(),mne);

        bb.mnemonics.end = MnemonicIndex::new(bb.mnemonics.end.index() + 1);
        bb.statements.end += stmt_rgn.end - stmt_rgn.start;

        Ok(())
    }

    pub fn remove_mnemonic(&mut self, basic_block: BasicBlockIndex) -> Result<()> {
        let (stmt_idx,move_by) = {
            let bb = &self.basic_blocks[basic_block.index()];

            if bb.mnemonics.end == bb.mnemonics.start {
                return Err("Internal error: empty basic block".into());
            }

            let addr = bb.area.start;
            let stmt_rgn = bb.statements.start..(bb.statements.start + self.mnemonics[bb.mnemonics.start.index()].num_statements);

            self.bitcode.remove(stmt_rgn.clone());
            self.mnemonics.remove(bb.mnemonics.start.index());
            (stmt_rgn.start,stmt_rgn.end - stmt_rgn.start)
        };

        let mne_idx = self.basic_blocks[basic_block.index()].mnemonics.start;

        self.shift_mnemonics(mne_idx,-1);
        self.shift_statements(BasicBlockIndex::new(basic_block.index() + 1),-1 * move_by as isize);

        let bb = &mut self.basic_blocks[basic_block.index()];
        bb.mnemonics.end = MnemonicIndex::new(bb.mnemonics.end.index() - 1);
        bb.statements.end -= move_by;

        Ok(())
    }

    fn shift_mnemonics(&mut self, start: MnemonicIndex, change: isize) {
        if start.index() >= self.mnemonics.len() { return; }
        for bb in self.basic_blocks.iter_mut() {
            if bb.mnemonics.start.index() > start.index() {
                bb.mnemonics.start = MnemonicIndex::new((bb.mnemonics.start.index() as isize + change) as usize);
                bb.mnemonics.end = MnemonicIndex::new((bb.mnemonics.end.index() as isize + change) as usize);
            }
        }
    }

    fn shift_statements(&mut self, start: BasicBlockIndex, change: isize) {
        if start.index() >= self.basic_blocks.len() { return; }

        let start_index = self.basic_blocks[start.index()].statements.start;

        for bb_idx in start.index()..self.basic_blocks.len() {
            let bb = &mut self.basic_blocks[bb_idx];
            let rgn = bb.statements.clone();
            let after_modification = rgn.start >= start_index;
            let no_underflow = change >= 0 || rgn.start as isize >= -change;

            if after_modification && no_underflow {
                bb.statements.start = (bb.statements.start as isize + change) as usize;
                bb.statements.end = (bb.statements.end as isize + change) as usize;
            }
        }
    }
}

fn disassemble<A: Architecture>(init: A::Configuration, starts: Vec<u64>, region: &Region,
                                mnemonics: &mut Vec<(Mnemonic,Vec<Statement<Value>>)>,
                                by_source: &mut HashMap<u64,Vec<(Value,Guard)>>,
                                by_destination: &mut HashMap<u64,Vec<(Value,Guard)>>) -> Result<()>
where A: Debug, A::Configuration: Debug {
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
                                trace!(
                                    "{:x}: {} ({:?})",
                                    mne.area.start,
                                    mne.opcode,
                                    match_st.tokens
                                    );
                                let this_mne = Mnemonic{
                                    area: mne.area.start..mne.area.end,
                                    opcode: mne.opcode.into(),
                                    operands: mne.operands.iter().map(|x| x.clone().into()).collect(),
                                    format_string: mne.format_string,
                                    num_statements: 0,
                                };
                                let stmts = mne.instructions.iter().map(|s| to_statement(s)).collect::<Vec<_>>();
                                mnemonics.insert(pos,(this_mne,stmts));
                            }
                        }

                        // New control transfers
                        for (origin, tgt, gu) in match_st.jumps {
                            trace!("jump to {:?}", tgt);
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

fn assemble_function(function: &mut Function, entry: u64, mut mnemonics: Vec<(Mnemonic,Vec<Statement<Value>>)>,
                     by_source: HashMap<u64,Vec<(Value,Guard)>>,
                     by_destination: HashMap<u64,Vec<(Value,Guard)>>) -> Result<()> {

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
                statements: 0..0,
            };

            basic_blocks.push(bb);
            idx = next_bb;
        } else {
            let bb = BasicBlock{
                mnemonics: MnemonicIndex::new(idx)..MnemonicIndex::new(mnemonics.len()),
                area: mnemonics[idx].0.area.start..mnemonics[idx].0.area.end,
                node: NodeIndex::new(0),
                statements: 0..0,
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
    let mut postorder = DfsPostOrder::new(&cfg,basic_blocks[entry_idx].node).iter(&cfg).collect::<Vec<_>>();
    let mut bitcode = Bitcode::default();
    let mut postorder_rev = vec![0; basic_blocks.len()];
    let mut po_idx = 0;

    postorder.reverse();

    for &n in postorder.iter() {
        if let Some(&CfgNode::BasicBlock(bb_idx)) = cfg.node_weight(n) {
            let bb = &mut basic_blocks[bb_idx.index()];
            let sl = bb.mnemonics.start.index()..bb.mnemonics.end.index();

            bb.statements = usize::MAX..usize::MIN;

            for (off,&mut (ref mut mne,ref mut instr)) in mnemonics.as_mut_slice()[sl].iter_mut().enumerate() {
                let rgn = bitcode.append(instr.drain(..))?;

                mne.num_statements = rgn.end - rgn.start;
                bb.statements.start = cmp::min(bb.statements.start,rgn.start);
                bb.statements.end = cmp::max(bb.statements.end,rgn.end);
            }

            postorder_rev[bb_idx.index()] = po_idx;
            po_idx += 1;
        }
    }

    basic_blocks.sort_by_key(|x| {
        if let Some(&CfgNode::BasicBlock(bb_idx)) = cfg.node_weight(x.node) {
            postorder_rev[bb_idx.index()]
        } else {
            unreachable!()
        }
    });

    for (idx,bb) in basic_blocks.iter().enumerate() {
        *cfg.node_weight_mut(bb.node).unwrap() = CfgNode::BasicBlock(BasicBlockIndex::new(idx));
    }

    function.name = format!("func_{:x}",entry).into();
    function.bitcode = bitcode;
    function.basic_blocks = basic_blocks;
    function.mnemonics = mnemonics.into_iter().map(|(mne,_)| mne).collect();
    function.cflow_graph = cfg;
    function.entry_point = BasicBlockIndex::new(0);

    //debug!("{:?}",function.entry_point);
    //debug!("{:?}",function.basic_blocks);

    use petgraph::dot::Dot;
    //debug!("{:?}",Dot::new(&function.cflow_graph));

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

fn to_statement(stmt: &core::Statement) -> Statement<Value> {
    match stmt {
        &core::Statement{ op: core::Operation::Add(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Add(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Subtract(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Subtract(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Multiply(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Multiply(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::DivideUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::DivideUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::DivideSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::DivideSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Modulo(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Modulo(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::ShiftLeft(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ShiftLeft(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::ShiftRightUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ShiftRightUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::ShiftRightSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ShiftRightSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::InclusiveOr(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::InclusiveOr(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::And(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::And(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::ExclusiveOr(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ExclusiveOr(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Equal(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Equal(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::LessOrEqualUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessOrEqualUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::LessOrEqualSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessOrEqualSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::LessUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::LessSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::SignExtend(sz,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::SignExtend(sz,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::ZeroExtend(sz,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ZeroExtend(sz,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Move(ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Move(a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Initialize(ref s,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Initialize(s.clone(),a.clone().into()), result: Variable::new(name.clone(),size * 8,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Select(sz,ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Select(sz,a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Load(ref s,core::Endianess::Little,b,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Load(s.clone(),Endianess::Little,b,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Load(ref s,core::Endianess::Big,b,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Load(s.clone(),Endianess::Big,b,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &core::Statement{ op: core::Operation::Store(ref s,core::Endianess::Little,by,ref a,ref b),.. } => {
            Statement::Store{
                region: s.clone(),
                endianess: Endianess::Little,
                bytes: by,
                address: a.clone().into(),
                value: b.clone().into(),
            }
        }
        &core::Statement{ op: core::Operation::Store(ref s,core::Endianess::Big,by,ref a,ref b),.. } => {
            Statement::Store{
                region: s.clone(),
                endianess: Endianess::Big,
                bytes: by,
                address: a.clone().into(),
                value: b.clone().into(),
            }
        }

        //Phi(Vec<V>),
        &core::Statement{ op: core::Operation::Call(ref a),.. } => {
            Statement::IndirectCall{
                target: a.clone().into(),
            }
        }


        _ => unimplemented!("{:?}",stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Architecture, Disassembler, Guard, Match, OpaqueLayer, Region, Result, Rvalue, State, TestArch};
    use std::sync::Arc;
    use petgraph::dot::Dot;
    use env_logger;

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
    fn single_instruction() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"A","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
		);
        let data = OpaqueLayer::wrap(vec![0]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

        assert_eq!(func.cflow_graph().node_count(), 1);
        assert_eq!(func.cflow_graph().edge_count(), 0);

        let node = func.cflow_graph().node_indices().next().unwrap();
        assert!(if let Some(&CfgNode::BasicBlock(_)) = func.cflow_graph().node_weight(node) { true } else { false });

        assert_eq!(func.entry_address(), 0);
        assert_eq!(func.basic_blocks().len(), 1);
        assert_eq!(func.name, "func_0");

        let (bb_idx,bb) = func.basic_blocks().next().unwrap();
        assert_eq!(bb.area(), 0..1);
        assert_eq!(func.mnemonics(bb_idx).len(), 1);

        let (mne_idx,mne) = func.mnemonics(bb_idx).next().unwrap();
        assert_eq!(mne.opcode, "A");

    }

    #[test]
    fn single_block() {
        let _ = env_logger::init();
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
        let func = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

        assert_eq!(func.cflow_graph().node_count(), 2);
        assert_eq!(func.cflow_graph().edge_count(), 1);

        for n in func.cflow_graph().node_indices() {
            match func.cflow_graph.node_weight(n) {
                Some(&CfgNode::BasicBlock(bb)) => {
                    let mnes = func.mnemonics(bb).collect::<Vec<_>>();
                    assert_eq!(mnes.len(), 6);
                    assert_eq!(mnes[0].1.opcode, "test0");
                    assert_eq!(mnes[0].1.area, 0..1);
                    assert_eq!(mnes[1].1.opcode, "test1");
                    assert_eq!(mnes[1].1.area, 1..2);
                    assert_eq!(mnes[2].1.opcode, "test2");
                    assert_eq!(mnes[2].1.area, 2..3);
                    assert_eq!(mnes[3].1.opcode, "test3");
                    assert_eq!(mnes[3].1.area, 3..4);
                    assert_eq!(mnes[4].1.opcode, "test4");
                    assert_eq!(mnes[4].1.area, 4..5);
                    assert_eq!(mnes[5].1.opcode, "test5");
                    assert_eq!(mnes[5].1.area, 5..6);
                    assert_eq!(func.basic_block(bb).area, 0..6);
                }
                Some(&CfgNode::Value(Value::Constant(Constant{ value: 6,.. }))) => {}
                _ => unreachable!()
            }
        }

        assert_eq!(func.entry_address(), 0);
        assert_eq!(func.basic_blocks().len(), 1);
        assert_eq!(func.name, "func_0");
    }

    #[test]
    fn branch() {
        let _ = env_logger::init();
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
        let func = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

        assert_eq!(func.cflow_graph.node_count(), 4);
        assert_eq!(func.cflow_graph.edge_count(), 4);

        let mut bb0_vx = None;
        let mut bb1_vx = None;
        let mut bb2_vx = None;
        let mut ures_vx = None;

        for n in func.cflow_graph.node_indices() {
            match func.cflow_graph().node_weight(n) {
                Some(&CfgNode::BasicBlock(bb_idx)) => {
                    let bb = func.basic_block(bb_idx);
                    let mnes = func.mnemonics(bb_idx).collect::<Vec<_>>();

                    assert_eq!(bb.node, n);

                    if bb.area.start == 0 {
                        assert_eq!(mnes.len(), 1);
                        assert_eq!(mnes[0].1.opcode, "test0");
                        assert_eq!(mnes[0].1.area, 0..1);
                        assert_eq!(bb.area, 0..1);
                        bb0_vx = Some(n);
                    } else if bb.area.start == 1 {
                        assert_eq!(mnes.len(), 1);
                        assert_eq!(mnes[0].1.opcode, "test1");
                        assert_eq!(mnes[0].1.area, 1..2);
                        assert_eq!(bb.area, 1..2);
                        bb1_vx = Some(n);
                    } else if bb.area.start == 2 {
                        assert_eq!(mnes.len(), 1);
                        assert_eq!(mnes[0].1.opcode, "test2");
                        assert_eq!(mnes[0].1.area, 2..3);
                        assert_eq!(bb.area, 2..3);
                        bb2_vx = Some(n);
                    } else {
                        unreachable!();
                    }
                }
                Some(&CfgNode::Value(Value::Constant(Constant{ value,.. }))) => {
                    assert_eq!(value, 3);
                    ures_vx = Some(n);
                }
                _ => { unreachable!(); }
            }
        }

        assert!(ures_vx.is_some() && bb0_vx.is_some() && bb1_vx.is_some() && bb2_vx.is_some());

        debug!("{:?}, {:?}, {:?}, {:?}",ures_vx, bb0_vx, bb1_vx, bb2_vx);

        let entry_bb = func.entry_point();
        assert_eq!(func.basic_block(entry_bb).node, bb0_vx.unwrap());
        assert_eq!(func.name, "func_0");
        assert!(func.cflow_graph().find_edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
        assert!(func.cflow_graph().find_edge(bb0_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph().find_edge(bb1_vx.unwrap(), ures_vx.unwrap()).is_some());
        assert!(func.cflow_graph().find_edge(bb2_vx.unwrap(), bb1_vx.unwrap()).is_some());
    }

    #[test]
    fn single_loop() {
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
        let func = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

        assert_eq!(func.cflow_graph.node_count(), 1);
        assert_eq!(func.cflow_graph.edge_count(), 1);

        let vx = func.cflow_graph.node_indices().next().unwrap();
        if let Some(&CfgNode::BasicBlock(bb_idx)) = func.cflow_graph().node_weight(vx) {
            let bb = func.basic_block(bb_idx);
            let mnes = func.mnemonics(bb_idx).collect::<Vec<_>>();

            if bb.area.start == 0 {
                assert_eq!(mnes.len(), 3);
                assert_eq!(mnes[0].1.opcode, "test0");
                assert_eq!(mnes[0].1.area, 0..1);
                assert_eq!(mnes[1].1.opcode, "test1");
                assert_eq!(mnes[1].1.area, 1..2);
                assert_eq!(mnes[2].1.opcode, "test2");
                assert_eq!(mnes[2].1.area, 2..3);
                assert_eq!(bb.area, 0..3);
            } else {
                unreachable!();
            }
        }

        assert_eq!(func.name, "func_0".to_string());
        let entry_idx = func.entry_point();
        assert_eq!(func.basic_block(entry_idx).node, vx);
        assert!(func.cflow_graph.find_edge(vx, vx).is_some());
    }

    #[test]
    fn empty_function() {
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
        assert!(Function::new::<TestArchShort>(main, 0, &reg, None).is_err());
    }

    #[test]
    fn resolve_indirect() {
        let _ = env_logger::init();
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
                st.jump(Rvalue::Variable{ name: "A".into(), subscript: None, size: 1, offset: 0 },Guard::always()).unwrap();
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
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1, 2, 3, 4, 5]);
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArchShort>(main.clone(), 0, &reg, None).unwrap();

        assert_eq!(func.cflow_graph().node_count(), 2);
        assert_eq!(func.cflow_graph().edge_count(), 1);

        for n in func.cflow_graph().node_indices() {
            match func.cflow_graph.node_weight(n) {
                Some(&CfgNode::BasicBlock(bb)) => {
                    assert_eq!(func.basic_block(bb).area, 0..2);
                }
                Some(&CfgNode::Value(Value::Variable(Variable{ ref name, bits: 1, subscript: None }))) if *name == "A" => {}
                a => unreachable!("got: {:?}",a)
            }
        }

        let unres = func.indirect_jumps().collect::<Vec<_>>();
        assert_eq!(unres.len(), 1);
        assert_eq!(unres[0], Variable{ name: "A".into(), bits: 1, subscript: None });

        assert!(func.resolve_indirect_jump(Variable{ name: "A".into(), bits: 1, subscript: None },Constant::new(2,1).unwrap()));
        assert!(func.extend::<TestArchShort>(main, &reg).is_ok());

        assert_eq!(func.cflow_graph().node_count(), 2);
        assert_eq!(func.cflow_graph().edge_count(), 1);

        for n in func.cflow_graph().node_indices() {
            match func.cflow_graph.node_weight(n) {
                Some(&CfgNode::BasicBlock(bb)) => {
                    assert_eq!(func.basic_block(bb).area, 0..4);
                }
                Some(&CfgNode::Value(Value::Constant(Constant{ value: 4,.. }))) => {}
                _ => unreachable!()
            }
        }

        let unres = func.indirect_jumps().collect::<Vec<_>>();
        assert_eq!(unres.len(), 0);
        assert!(!func.resolve_indirect_jump(Variable{ name: "A".into(), bits: 1, subscript: Some(0) },Constant::new(2,1).unwrap()));
    }

    #[test]
    fn entry_split() {
        let _ = env_logger::init();
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
                st.jump(Rvalue::Variable{ name: "A".into(), subscript: None, size: 1, offset: 0 },Guard::always()).unwrap();
                true
            }
        );

        let data = OpaqueLayer::wrap(vec![0, 1]);
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArchShort>(main.clone(), 0, &reg, None).unwrap();
        let unres = func.indirect_jumps().collect::<Vec<_>>();
        assert_eq!(unres.len(), 1);
        assert_eq!(unres[0], Variable{ name: "A".into(), bits: 1, subscript: None });

        assert!(func.resolve_indirect_jump(Variable{ name: "A".into(), bits: 1, subscript: None },Constant::new(1,1).unwrap()));
        assert!(func.extend::<TestArchShort>(main, &reg).is_ok());

        assert_eq!(func.cflow_graph().node_count(), 2);
        assert_eq!(func.cflow_graph().edge_count(), 1);

        let mut bb0_vx = None;
        let mut bb1_vx = None;

        for n in func.cflow_graph().node_indices() {
            match func.cflow_graph.node_weight(n) {
                Some(&CfgNode::BasicBlock(bb)) => {
                    if func.basic_block(bb).area == (1..2) {
                        bb1_vx = Some(n);
                    } else if func.basic_block(bb).area == (0..1) {
                        bb0_vx = Some(n);
                    } else {
                        unreachable!();
                    }
                }
                _ => unreachable!()
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some());
        let entry_idx = func.entry_point();
        assert_eq!(func.basic_block(entry_idx).node, bb0_vx.unwrap());
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

        let func = Function::new::<TestArchWide>(dec, 0, &reg, None).unwrap();

        assert_eq!(func.cflow_graph.node_count(), 3);
        assert_eq!(func.cflow_graph.edge_count(), 2);

        let mut bb0_vx = None;
        let mut bb1_vx = None;

        for vx in func.cflow_graph().node_indices() {
            match func.cflow_graph().node_weight(vx) {
                Some(&CfgNode::BasicBlock(bb_idx)) => {
                    let bb = func.basic_block(bb_idx);
                    let mnes = func.mnemonics(bb_idx).collect::<Vec<_>>();

                    if bb.area.start == 0 {
                        assert_eq!(mnes.len(), 2);
                        assert_eq!(bb.area, 0..4);
                        bb0_vx = Some(vx);
                    } else if bb.area.start == 4 {
                        assert_eq!(mnes.len(), 1);
                        assert_eq!(bb.area, 4..6);
                        bb1_vx = Some(vx);
                    } else {
                        unreachable!();
                    }
                }
                Some(&CfgNode::Value(Value::Constant(Constant{ value: 6,.. }))) => {}
                _ => unreachable!(),
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some());
        let entry_idx = func.entry_point();
        assert_eq!(func.basic_block(entry_idx).node, bb0_vx.unwrap());
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
        let func = Function::new::<TestArchShort>(main, 1, &reg, None).unwrap();

        assert_eq!(func.cflow_graph.node_count(), 2);
        assert_eq!(func.cflow_graph.edge_count(), 2);

        let mut bb0_vx = None;
        let mut bb1_vx = None;

        for vx in func.cflow_graph.node_indices() {
            if let Some(&CfgNode::BasicBlock(bb_idx)) = func.cflow_graph().node_weight(vx) {
                let bb = func.basic_block(bb_idx);
                let mnes = func.mnemonics(bb_idx).collect::<Vec<_>>();

                if bb.area.start == 0 {
                    assert_eq!(mnes.len(), 1);
                    assert_eq!(bb.area, 0..1);
                    bb0_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(mnes.len(), 2);
                    assert_eq!(bb.area, 1..3);
                    bb1_vx = Some(vx);
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some());
        let entry_idx = func.entry_point();
        assert_eq!(func.basic_block(entry_idx).node, bb1_vx.unwrap());
        assert!(func.cflow_graph.find_edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
        assert!(func.cflow_graph.find_edge(bb1_vx.unwrap(), bb0_vx.unwrap()).is_some());
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
        let func = Function::new::<TestArchShort>(main, 1, &reg, None).unwrap();

        assert_eq!(func.cflow_graph.node_count(), 3);
        assert_eq!(func.cflow_graph.edge_count(), 3);

        let mut bb01_vx = None;
        let mut bb1_vx = None;
        let mut bb2_vx = None;

        for vx in func.cflow_graph().node_indices() {
            if let Some(&CfgNode::BasicBlock(bb_idx)) = func.cflow_graph().node_weight(vx) {
                let bb = func.basic_block(bb_idx);
                let mnes = func.mnemonics(bb_idx).collect::<Vec<_>>();

                if bb.area.start == 0 {
                    assert_eq!(mnes.len(), 1);
                    assert_eq!(bb.area, 0..2);
                    bb01_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(mnes.len(), 1);
                    assert_eq!(bb.area, 1..2);
                    bb1_vx = Some(vx);
                } else if bb.area.start == 2 {
                    assert_eq!(mnes.len(), 1);
                    assert_eq!(bb.area, 2..3);
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
        let entry_idx = func.entry_point();
        assert_eq!(func.basic_block(entry_idx).node, bb1_vx.unwrap());
        assert!(func.cflow_graph.find_edge(bb01_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.find_edge(bb1_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.find_edge(bb2_vx.unwrap(), bb01_vx.unwrap()).is_some());
    }

    #[test]
    fn iter_range() {
        let main = new_disassembler!(TestArchShort =>
            [ 0 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test0","",vec!(),&|_| {
                    rreil!{
                        add a:32, b:32, c:32;
                        sub a:32, b:32, c:32;
                    }
                }).unwrap();
                let addr = st.address;
                st.jump(Rvalue::new_u64(addr + 1),Guard::always()).unwrap();
                true
            },
            [ 1 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1,"test1","",vec!(),&|_| {
                   rreil!{
                        add a:32, b:32, c:32;
                    }
                }).unwrap();
                let addr = st.address;
                st.jump(Rvalue::new_u64(addr + 1),Guard::always()).unwrap();
                true
            },
            [ 2, 3 ] = |st: &mut State<TestArchShort>| {
                st.mnemonic(2,"test2","",vec!(),&|_| {
                   rreil!{
                        sub a:32, b:32, c:32;
                    }
                }).unwrap();
                true
            }
        );
        let data = OpaqueLayer::wrap(vec![0, 1, 2, 3, 0, 0]);
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

        let bb_idx = func.basic_blocks().map(|x| x.0).collect::<Vec<_>>();
        assert_eq!(bb_idx.len(), 1);
        let stmts = func.statements(bb_idx[0]).collect::<Vec<_>>();
        assert_eq!(stmts.len(), 4);

        let bb = func.basic_blocks().map(|x| x.1).collect::<Vec<_>>();
        assert_eq!(bb.len(), 1);
        let stmts = func.statements(bb[0]).collect::<Vec<_>>();
        assert_eq!(stmts.len(), 4);

        let stmts = func.statements(..).collect::<Vec<_>>();
        assert_eq!(stmts.len(), 4);

        let mne_idx = func.mnemonics(..).map(|x| x.0).collect::<Vec<_>>();
        assert_eq!(mne_idx.len(), 3);
        //let stmts = func.statements(mne_idx[1]).collect::<Vec<_>>();
        //assert_eq!(stmts.len(), 1);
        //if let &Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Variable(_)),.. } = &stmts[0] { ; } else { unreachable!() }

        //let stmts = func.statements(mne_idx[0]).collect::<Vec<_>>();
        //assert_eq!(stmts.len(), 2);
        //if let &Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Variable(_)),.. } = &stmts[0] { ; } else { unreachable!() }
        //if let &Statement::Expression{ op: Operation::Subtract(Value::Variable(_),Value::Variable(_)),.. } = &stmts[1] { ; } else { unreachable!() }
    }

    /*
     * (B0)
     * 0:  Mi1  ; mov i 1
     * 3:  Cfi0 ; cmp f i 0
     * 7:  Bf18 ; br f (B2)
     *
     * (B1)
     * 11: Aii3 ; add i i 3
     * 15: J22  ; jmp (B3)
     *
     * (B2)
     * 18: Ai23 ; add i 2 3
     *
     * (B3)
     * 22: Ms3  ; mov s 3
     * 25: R    ; ret
     */
    #[test]
    fn rewrite_increase() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let mut b0_idx = None;
        let mut b1_idx = None;
        let mut b2_idx = None;
        let mut b3_idx = None;

        for (idx,bb) in func.basic_blocks() {
            if bb.area.start == 0 {
                b0_idx = Some(idx);
            } else if bb.area.start == 11 {
                b1_idx = Some(idx);
            } else if bb.area.start == 18 {
                b2_idx = Some(idx);
            } else if bb.area.start == 22 {
                b3_idx = Some(idx);
            } else {
                unreachable!()
            }
        }

        assert!(b0_idx.is_some() && b1_idx.is_some() && b2_idx.is_some() && b3_idx.is_some());

        fn f(stmt: &mut Statement<Value>) -> ::neo::Result<()> {
            match stmt {
                &mut Statement::Expression{ result: Variable{ ref mut name,.. },.. } => {
                    *name = name.to_string().to_uppercase().into();
                }
                _ => {}
            }

            Ok(())
        }

        let _ = func.rewrite_mnemonics(b2_idx.unwrap(),|stmt| {
            match stmt {
                &mut Statement::Expression{ op: Operation::Add(Value::Constant(ref mut a),Value::Constant(ref mut b)),.. } => {
                    *a = Constant::new(0xffffffff,32).unwrap();
                    *b = Constant::new(0x11111111,32).unwrap();
                }
                _ => {}
            }

            Ok(())
        }).unwrap();

        let b0 = func.statements(b0_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
        assert_eq!(b0.len(), 2);

        let b1 = func.statements(b1_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[0] {} else { unreachable!() }
        assert_eq!(b1.len(), 1);

        let b2 = func.statements(b2_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
        assert_eq!(b2.len(), 1);

        let b3 = func.statements(b3_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
        assert_eq!(b3.len(), 1);
    }

    /*
     * (B0)
     * 0:  Mi1  ; mov i 1
     * 3:  Cfi0 ; cmp f i 0
     * 7:  Bf18 ; br f (B2)
     *
     * (B1)
     * 11: Aii3 ; add i i 3
     * 15: J22  ; jmp (B3)
     *
     * (B2)
     * 18: Ai23 ; add i 2 3
     *
     * (B3)
     * 22: Ms3  ; mov s 3
     * 25: R    ; ret
     */
    #[test]
    fn rewrite_rename() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let mut b0_idx = None;
        let mut b1_idx = None;
        let mut b2_idx = None;
        let mut b3_idx = None;

        for (idx,bb) in func.basic_blocks() {
            if bb.area.start == 0 {
                b0_idx = Some(idx);
            } else if bb.area.start == 11 {
                b1_idx = Some(idx);
            } else if bb.area.start == 18 {
                b2_idx = Some(idx);
            } else if bb.area.start == 22 {
                b3_idx = Some(idx);
            } else {
                unreachable!()
            }
        }

        assert!(b0_idx.is_some() && b1_idx.is_some() && b2_idx.is_some() && b3_idx.is_some());
        fn f(stmt: &mut Statement<Value>) -> ::neo::Result<()> {
            match stmt {
                &mut Statement::Expression{ result: Variable{ ref mut name,.. },.. } => {
                    *name = name.to_string().to_uppercase().into();
                }
                _ => {}
            }

            Ok(())
        }

        let _ = func.rewrite_mnemonics(b0_idx.unwrap(),&f).unwrap();
        let _ = func.rewrite_mnemonics(b1_idx.unwrap(),&f).unwrap();
        let _ = func.rewrite_mnemonics(b2_idx.unwrap(),&f).unwrap();
        let _ = func.rewrite_mnemonics(b3_idx.unwrap(),&f).unwrap();

        let b0 = func.statements(b0_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
        assert_eq!(b0.len(), 2);

        let b1 = func.statements(b1_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[0] {} else { unreachable!() }
        assert_eq!(b1.len(), 1);

        let b2 = func.statements(b2_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
        assert_eq!(b2.len(), 1);

        let b3 = func.statements(b3_idx.unwrap()).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
        assert_eq!(b3.len(), 1);

        for stmt in func.statements(..) {
            match stmt {
                Statement::Expression{ result: Variable{ ref name,.. },.. } => {
                    assert!(name.chars().all(|x| x.is_uppercase()));
                }
                _ => {}
            }
        }
    }

    /*
     * (B0)
     * 0:  Mi1  ; mov i 1
     * 3:  Cfi0 ; cmp f i 0
     * 7:  Bf18 ; br f (B2)
     *
     * (B1)
     *          ; test
     * 11: Aii3 ; add i i 3
     * 15: J22  ; jmp (B3)
     *
     * (B2)
     * 18: Ai23 ; add i 2 3
     *
     * (B3)
     * 22: Ms3  ; mov s 3
     * 25: R    ; ret
     */
    #[test]
    fn rewrite_prepend_mnemonic() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let mut b0_idx = None;
        let mut b1_idx = None;
        let mut b2_idx = None;
        let mut b3_idx = None;

        for (idx,bb) in func.basic_blocks() {
            if bb.area.start == 0 {
                b0_idx = Some(idx);
            } else if bb.area.start == 11 {
                b1_idx = Some(idx);
            } else if bb.area.start == 18 {
                b2_idx = Some(idx);
            } else if bb.area.start == 22 {
                b3_idx = Some(idx);
            } else {
                unreachable!()
            }
        }

        assert!(b0_idx.is_some() && b1_idx.is_some() && b2_idx.is_some() && b3_idx.is_some());

        let stmts = vec![
            Statement::Expression{
                op: Operation::And(Value::val(42,32).unwrap(),Value::var("x",32,None).unwrap()),
                result: Variable::new("x",32,None).unwrap()
            },
            Statement::Expression{
                op: Operation::Subtract(Value::val(42,32).unwrap(),Value::var("x",32,None).unwrap()),
                result: Variable::new("x",32,None).unwrap()
            },
        ];
        debug!("{:?}",func);
        let _ = func.prepend_mnemonic(b1_idx.unwrap(),"test",stmts).unwrap();
        debug!("{:?}",func);

        debug!("read bb 0");
        let b0 = func.statements(b0_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b0.len(), 2);
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }

        debug!("read bb 1");
        let b1 = func.statements(b1_idx.unwrap()).collect::<Vec<_>>();
        debug!("{:?}",b1);
        assert_eq!(b1.len(), 3);
        if let Statement::Expression{ op: Operation::And(Value::Constant(_),Value::Variable(_)),.. } = b1[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::Subtract(Value::Constant(_),Value::Variable(_)),.. } = b1[1] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[2] {} else { unreachable!() }

        debug!("read bb 2");
        let b2 = func.statements(b2_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b2.len(), 1);
        if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }

        debug!("read bb 3");
        let b3 = func.statements(b3_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b3.len(), 1);
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
    }

    /*
    #[test]
    fn rewrite_add_statements() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let _ = func.rewrite(|basic_blocks| {
            let stmts = vec![
                Statement::Expression{
                    op: Operation::And(Value::val(42,32).unwrap(),Value::var("x",32,None).unwrap()),
                    result: Variable::new("x",32,None).unwrap()
                },
                Statement::Expression{
                    op: Operation::Subtract(Value::val(42,32).unwrap(),Value::var("x",32,None).unwrap()),
                    result: Variable::new("x",32,None).unwrap()
                },
            ];

            basic_blocks[1][0].1.extend(stmts);
            Ok(())
        }).unwrap();

        let b0 = func.statements(BasicBlockIndex::new(0)).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
        assert_eq!(b0.len(), 2);

        let b1 = func.statements(BasicBlockIndex::new(1)).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::And(Value::Constant(_),Value::Variable(_)),.. } = b1[1] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::Subtract(Value::Constant(_),Value::Variable(_)),.. } = b1[2] {} else { unreachable!() }
        assert_eq!(b1.len(), 3);

        let b2 = func.statements(BasicBlockIndex::new(2)).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
        assert_eq!(b2.len(), 1);

        let b3 = func.statements(BasicBlockIndex::new(3)).collect::<Vec<_>>();
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
        assert_eq!(b3.len(), 1);
    }*/

    #[test]
    fn rewrite_remove_mnemonic() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let mut b0_idx = None;
        let mut b1_idx = None;
        let mut b2_idx = None;
        let mut b3_idx = None;

        for (idx,bb) in func.basic_blocks() {
            if bb.area.start == 0 {
                b0_idx = Some(idx);
            } else if bb.area.start == 11 {
                b1_idx = Some(idx);
            } else if bb.area.start == 18 {
                b2_idx = Some(idx);
            } else if bb.area.start == 22 {
                b3_idx = Some(idx);
            } else {
                unreachable!()
            }
        }

        assert!(b0_idx.is_some() && b1_idx.is_some() && b2_idx.is_some() && b3_idx.is_some());

        debug!("{:?}",func);
        let _ = func.remove_mnemonic(b1_idx.unwrap()).unwrap();
        debug!("{:?}",func);

        let b0 = func.statements(b0_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b0.len(), 2);
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
        if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }

        let b1 = func.statements(b1_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b1.len(), 0);

        let b2 = func.statements(b2_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b2.len(), 1);
        if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }

        let b3 = func.statements(b3_idx.unwrap()).collect::<Vec<_>>();
        assert_eq!(b3.len(), 1);
        if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
    }
}
