use std::ops::Range;
use std::cell::RefCell;
use uuid::Uuid;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Walker,DfsPostOrder};
use {Architecture,Guard,Region,MnemonicFormatToken,Rvalue,Lvalue};
use neo::{Str,Result,Statement,Bitcode,Value,BitcodeIter,Constant,Operation,Variable,Endianess,CallTarget};

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
    mnemonics: Range<MnemonicIndex>,
    node: NodeIndex,
    area: Range<u64>,
}

#[derive(Debug)]
pub struct Mnemonic {
    area: Range<u64>,
    opcode: Str,
    operands: Vec<Value>,
    format_string: Vec<MnemonicFormatToken>,
    statements: Range<usize>,
}

#[derive(Clone,Copy,Debug)]
pub struct BasicBlockIndex {
    index: usize
}

impl BasicBlockIndex {
    pub fn new(i: usize) -> BasicBlockIndex { BasicBlockIndex{ index: i } }
    pub fn index(&self) -> usize { self.index }
}

#[derive(Clone,Copy,Debug)]
pub struct MnemonicIndex {
    index: usize
}

impl MnemonicIndex {
    pub fn new(i: usize) -> MnemonicIndex { MnemonicIndex{ index: i } }
    pub fn index(&self) -> usize { self.index }
}

#[derive(Debug)]
enum CfgNode {
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
    // sorted by bb idx
    cflow_graph: Graph<CfgNode,Guard>,
}

impl Function {
	// disassembly
    pub fn new<A: Architecture>(init: A::Configuration, start: u64, region: &Region, name: Option<Str>) -> Result<Function>
        where A: Debug, A::Configuration: Debug {
        let (mnemonics, by_source, by_destination) = disassemble::<A>(init,start,region)?;
        assemble_function(start, mnemonics, by_source, by_destination)
    }

    pub fn extend(&mut self,region: &Region) -> Result<bool> { unimplemented!() }

    // getter
    pub fn entry_point(&self) -> BasicBlockIndex { unimplemented!() }
    pub fn mnemonics(&self) -> &[Mnemonic] { unimplemented!() }
    pub fn mnemonics_for_basic_block(&self, idx: BasicBlockIndex) -> &[Mnemonic] { unimplemented!() }
    pub fn basic_blocks(&self) -> &[BasicBlock] { unimplemented!() }
    pub fn uuid(&self) -> &Uuid { unimplemented!() }

    // iters
    pub fn statements(&self) -> BitcodeIter { unimplemented!() }
    pub fn statements_for_block(&self, idx: BasicBlockIndex) -> BitcodeIter { unimplemented!() }
    pub fn statements_for_mnemonic(&self, idx: MnemonicIndex) -> BitcodeIter { unimplemented!() }

    // aux
    pub fn first_address(&self) -> u64 { unimplemented!() }
    pub fn has_unresolved_jumps(&self) -> bool { unimplemented!() }
    pub fn resolve_jump(&self,_: Value,_: u64) -> Result<bool> { unimplemented!() }

    // insert
    pub fn insert_statement(stmt: Statement, mnemonic: MnemonicIndex,
        basic_block: BasicBlockIndex, index: usize) -> Result<()> { unimplemented!() }
    pub fn insert_pseudo_mnemonic(stmts: &[Statement], opcode: Str,
        basic_block: BasicBlock, index: usize) -> Result<()> { unimplemented!() }
}

fn disassemble<A: Architecture>(init: A::Configuration, start: u64, region: &Region)
-> Result<(Vec<core::Mnemonic>,HashMap<u64,Vec<(Value,Guard)>>,HashMap<u64,Vec<(Value,Guard)>>)>
where A: Debug, A::Configuration: Debug {
    let mut todo = HashSet::<u64>::new();
    let mut mnemonics = Vec::<core::Mnemonic>::new();
    let mut by_source = HashMap::<u64,Vec<(Value,Guard)>>::new();
    let mut by_destination = HashMap::<u64,Vec<(Value,Guard)>>::new();

    todo.insert(start);

    while let Some(addr) = todo.iter().next().cloned() {
        assert!(todo.remove(&addr));

        match mnemonics.binary_search_by_key(&addr,|x| x.area.start) {
            // Already disassembled here
            Ok(pos) => {
                let mne = &mnemonics[pos];

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
                                    "{:x}: {} ({:?})",
                                    mne.area.start,
                                    mne.opcode,
                                    match_st.tokens
                                    );
                                mnemonics.insert(pos,mne);
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

    Ok((mnemonics,by_source,by_destination))
}

fn assemble_function(entry: u64, mut mnemonics: Vec<core::Mnemonic>,
                     by_source: HashMap<u64,Vec<(Value,Guard)>>,
                     by_destination: HashMap<u64,Vec<(Value,Guard)>>) -> Result<Function> {

    let mut basic_blocks = Vec::<BasicBlock>::new();
    let mut idx = 0;

    mnemonics.sort_by_key(|x| x.area.start);

    // Split mnemonics into basic blocks
    while idx < mnemonics.len() {
        if mnemonics.len() - idx > 1 {
            let next_bb = mnemonics
                .as_slice()[idx..].windows(2)
                .position(|x| is_basic_block_boundary(&x[0],&x[1],entry,&by_source,&by_destination))
                .map(|x| x + 1 + idx)
                .unwrap_or(mnemonics.len());
            let bb = BasicBlock{
                mnemonics: MnemonicIndex::new(idx)..MnemonicIndex::new(next_bb),
                area: mnemonics[idx].area.start..mnemonics[next_bb - 1].area.end,
                node: NodeIndex::new(0),
            };

            basic_blocks.push(bb);
            idx = next_bb;
        } else {
            let bb = BasicBlock{
                mnemonics: MnemonicIndex::new(idx)..MnemonicIndex::new(mnemonics.len()),
                area: mnemonics[idx].area.start..mnemonics[idx].area.end,
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

    for (i,bb) in basic_blocks.iter().enumerate() {
        if let Some(ct) = by_source.get(&bb.area.start) {
            for &(ref val,ref guard) in ct.iter() {
                match val {
                    &Value::Constant(Constant{ value,.. }) if bb.area.start <= value && bb.area.end > value => {}
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
    let postorder = DfsPostOrder::new(&cfg,basic_blocks[entry_idx].node).iter(&cfg).collect::<Vec<_>>();
    let mut bitcode = Bitcode::default();
    let mut statement_ranges = vec![0..0; mnemonics.len()];

    for &n in postorder.iter() {
        if let Some(&CfgNode::BasicBlock(idx)) = cfg.node_weight(n) {
            let bb = &basic_blocks[idx.index()];
            let sl = bb.mnemonics.start.index()..bb.mnemonics.end.index();
            for (off,mne) in mnemonics.as_slice()[sl].iter().enumerate() {
                let rgn = bitcode.append(mne.instructions.iter().map(|s| to_statement(s)))?;

                statement_ranges[bb.mnemonics.start.index() + off] = rgn;
            }
        }
    }

    let func = Function{
        name: "".into(),
        uuid: Uuid::new_v4(),
        bitcode: bitcode,
        basic_blocks: basic_blocks,
        mnemonics: mnemonics.into_iter().enumerate().map(|(idx,mne)| {
            Mnemonic{
                area: mne.area.start..mne.area.end,
                opcode: mne.opcode.into(),
                operands: mne.operands.iter().map(|x| x.clone().into()).collect(),
                format_string: mne.format_string,
                statements: statement_ranges[idx].clone(),
            }
        }).collect(),
        cflow_graph: cfg,
    };

    Ok(func)
}

fn is_basic_block_boundary(a: &core::Mnemonic, b: &core::Mnemonic, entry: u64,
                           by_source: &HashMap<u64,Vec<(Value,Guard)>>,
                           by_destination: &HashMap<u64,Vec<(Value,Guard)>>) -> bool {
    true
}

fn to_statement(stmt: &core::Statement) -> Statement {
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
            Statement::Expression{ op: Operation::Initialize(s.clone(),a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
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
/*
    #[test]
    fn new() {
        let f = Function::new("test".to_string(), "ram".to_string());

        assert_eq!(f.name, "test".to_string());
        assert_eq!(f.cflow_graph.num_vertices(), 0);
        assert_eq!(f.cflow_graph.num_edges(), 0);
        assert_eq!(f.entry_point, None);
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

        let (mnes, src, dest) = Function::index_cflow_graph(cfg, None);

        assert_eq!(mnes.len(), 9);
        assert_eq!(src.values().fold(0, |acc, x| acc + x.len()), 10);
        assert_eq!(dest.values().fold(0, |acc, x| acc + x.len()), 10);

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

        let (mnes, src, dest) = Function::index_cflow_graph(cfg, None);

        assert_eq!(mnes.len(), 2);
        assert_eq!(src.values().fold(0, |acc, x| acc + x.len()), 3);
        assert_eq!(dest.values().fold(0, |acc, x| acc + x.len()), 3);

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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 0);

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

        assert_eq!(func.entry_point, func.cflow_graph.vertices().next());
        assert_eq!(func.name, "func_0".to_string());
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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 0);

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
        assert_eq!(func.entry_point, bb_vx);
        assert_eq!(func.name, "func_0".to_string());
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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 0);

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
        assert_eq!(func.entry_point, bb0_vx);
        assert_eq!(func.name, "func_0".to_string());
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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 0);

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

        assert_eq!(func.name, "func_0".to_string());
        assert_eq!(func.entry_point, Some(vx));
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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 0);

        assert_eq!(func.cflow_graph.num_vertices(), 1);
        assert_eq!(func.cflow_graph.num_edges(), 0);
        assert_eq!(func.name, "func_0".to_string());
        assert_eq!(func.entry_point, None);

        let vx = func.cflow_graph.vertices().next().unwrap();
        if let Some(&ControlFlowTarget::Failed(v, _)) = func.cflow_graph.vertex_label(vx) {
            assert_eq!(v, 0);
        }
    }

    #[test]
    fn entry_split() {
        let bb = BasicBlock::from_vec(vec![Mnemonic::dummy(0..1), Mnemonic::dummy(1..2)]);
        let mut fun = Function::new("test_func".to_string(), "ram".to_string());
        let vx0 = fun.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb));
        let vx1 = fun.cflow_graph.add_vertex(ControlFlowTarget::Unresolved(Rvalue::new_u32(2)));

        fun.entry_point = Some(vx0);
        fun.cflow_graph.add_edge(Guard::always(), vx0, vx1);

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
        let func = Function::disassemble::<TestArchShort>(Some(fun), main, &reg, 2);

        assert_eq!(func.cflow_graph.num_vertices(), 3);
        assert_eq!(func.cflow_graph.num_edges(), 3);
        assert_eq!(func.name, "test_func".to_string());

        let mut bb0_vx = None;
        let mut bb1_vx = None;
        let mut bb2_vx = None;

        for vx in func.cflow_graph.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                if bb.area.start == 0 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.mnemonics[0].opcode, "dummy".to_string());
                    assert_eq!(bb.mnemonics[0].area, Bound::new(0, 1));
                    assert_eq!(bb.area, Bound::new(0, 1));
                    bb0_vx = Some(vx);
                } else if bb.area.start == 1 {
                    assert_eq!(bb.mnemonics.len(), 1);
                    assert_eq!(bb.mnemonics[0].opcode, "dummy".to_string());
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
            } else {
                unreachable!();
            }
        }

        assert!(bb0_vx.is_some() && bb1_vx.is_some() && bb2_vx.is_some());
        assert_eq!(func.entry_point, bb0_vx);
        assert!(func.cflow_graph.edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb1_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb2_vx.unwrap(), bb1_vx.unwrap()).is_some());
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

        let func = Function::disassemble::<TestArchWide>(None, dec, &reg, 0);

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
        assert_eq!(func.entry_point, bb0_vx);
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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 1);

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
        assert_eq!(func.entry_point, bb1_vx);
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
        let func = Function::disassemble::<TestArchShort>(None, main, &reg, 1);

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
        assert_eq!(func.entry_point, bb1_vx);
        assert!(func.cflow_graph.edge(bb01_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb1_vx.unwrap(), bb2_vx.unwrap()).is_some());
        assert!(func.cflow_graph.edge(bb2_vx.unwrap(), bb01_vx.unwrap()).is_some());
    }*/
}
