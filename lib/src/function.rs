use basic_block::BasicBlock;
use guard::Guard;
use graph_algos::{AdjacencyList,GraphTrait,MutableGraphTrait};
use graph_algos::adjacency_list::{AdjacencyListEdgeDescriptor,AdjacencyListVertexDescriptor};
use graph_algos::{VertexListGraphTrait,EdgeListGraphTrait};
use disassembler::{Disassembler,State,Token};
use layer::LayerIter;
use value::Rvalue;
use std::collections::{HashMap,BTreeMap,BinaryHeap,Bound,BTreeSet};
use mnemonic::Mnemonic;
use num::traits::NumCast;
use std::fmt::{Display,Debug};
use std::ops::{BitAnd,BitOr,Shl,Shr,Not};
use std::rc::Rc;

use disassembler;

#[derive(RustcDecodable,RustcEncodable,Debug)]
pub enum ControlFlowTarget {
    Resolved(BasicBlock),
    Unresolved(Rvalue),
}

pub type ControlFlowGraph = AdjacencyList<ControlFlowTarget,Guard>;
pub type ControlFlowRef = AdjacencyListVertexDescriptor;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Function {
    pub name: String,
    pub cflow_graph: ControlFlowGraph,
    pub entry_point: Option<ControlFlowRef>
}

impl Function {
    pub fn new(a: String) -> Function {
        Function{
            name: a,
            cflow_graph: AdjacencyList::new(),
            entry_point: None,
        }
    }

    fn index_cflow_graph(g: ControlFlowGraph) -> (BTreeMap<u64,Vec<Mnemonic>>,HashMap<u64,Vec<(Option<u64>,Guard)>>,HashMap<u64,Vec<(Option<u64>,Guard)>>) {
        let mut mnemonics = BTreeMap::new();
        let mut by_source = HashMap::<u64,Vec<(Option<u64>,Guard)>>::new();
        let mut by_destination = HashMap::<u64,Vec<(Option<u64>,Guard)>>::new();

        for v in g.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = g.vertex_label(v) {
                for mne in &bb.mnemonics {
                    mnemonics.entry(mne.area.start).or_insert(Vec::new()).push(mne.clone());
                }
            }
        }

        for e in g.edges() {
            let gu = g.edge_label(e).unwrap().clone();
            let src = g.vertex_label(g.source(e));
            let tgt = g.vertex_label(g.target(e));

            match (src,tgt) {
                (Some(&ControlFlowTarget::Resolved(ref src_bb)),Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    by_source.entry(src_bb.area.start).or_insert(Vec::new()).push((Some(tgt_bb.area.start),gu.clone()));
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((Some(src_bb.area.start),gu));
                },
                (Some(&ControlFlowTarget::Resolved(ref src_bb)),Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c)))) => {
                    by_source.entry(src_bb.area.start).or_insert(Vec::new()).push((Some(*c),gu.clone()));
                    by_destination.entry(*c).or_insert(Vec::new()).push((Some(src_bb.area.start),gu));
                },
                (Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))),Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    by_source.entry(*c).or_insert(Vec::new()).push((Some(tgt_bb.area.start),gu.clone()));
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((Some(*c),gu));
                },
                (Some(&ControlFlowTarget::Resolved(ref src_bb)),Some(&ControlFlowTarget::Unresolved(_))) => {
                    by_source.entry(src_bb.area.start).or_insert(Vec::new()).push((None,gu));
                },
                (Some(&ControlFlowTarget::Unresolved(_)),Some(&ControlFlowTarget::Resolved(ref tgt_bb))) => {
                    by_destination.entry(tgt_bb.area.start).or_insert(Vec::new()).push((None,gu));
                },
                _ => {}
            }
        }

        (mnemonics,by_source,by_destination)
    }

    fn assemble_cflow_graph(mnemonics: BTreeMap<u64,Vec<Mnemonic>>,
                            by_source: HashMap<u64,Vec<(Option<u64>,Guard)>>,
                            by_destination: HashMap<u64,Vec<(Option<u64>,Guard)>>,
                            start: u64) -> ControlFlowGraph
    {
        let mut ret = ControlFlowGraph::new();
        let mut bblock = Vec::<Mnemonic>::new();

        for (off,mnes) in mnemonics.iter() {
            for mne in mnes {
                let last_pos = if mne.area.start == mne.area.end {
                    mne.area.start
                } else {
                    mne.area.end - 1
                };

                if bblock.len() > 0 {
                    println!("start");
                    // if next mnemonics aren't adjacent
                    let mut new_bb = bblock.last().unwrap().area.end != mne.area.start;
                    println!("1: {} ({} vs {}, {})",new_bb,bblock.last().unwrap().area.end,mne.area.start,off);

					// or any following jumps aren't to adjacent mnemonics
                    new_bb |= by_source.get(&mne.area.start).unwrap_or(&Vec::new()).iter().any(|&(ref opt_dest,ref gu)| {
                        opt_dest.is_some() && opt_dest.unwrap() != mne.area.end });
                    println!("2: {}",new_bb);

					// or any jumps pointing to the next that aren't from here
                    new_bb |= by_destination.get(&mne.area.start).unwrap_or(&Vec::new()).iter().any(|&(ref opt_src,ref gu)| {
                        opt_src.is_some() && opt_src.unwrap() != bblock.last().unwrap().area.start });
                    println!("3: {}",new_bb);

                    // or the entry point does not point here
                    new_bb |= mne.area.start == start;
                    println!("4: {}",new_bb);

                    if new_bb {
                        ret.add_vertex(ControlFlowTarget::Resolved(BasicBlock::from_vec(bblock.clone())));
                        bblock.clear();
                    }
                }

                bblock.push(mne.clone());
            }
        }

        // last basic block
        ret.add_vertex(ControlFlowTarget::Resolved(BasicBlock::from_vec(bblock)));

        // connect basic blocks
        for (src_off,tgts) in by_source.iter() {
            for &(ref opt_tgt,ref gu) in tgts {
                if opt_tgt.is_some() {
                    let from_bb = ret.vertices().find(|&t| {
                        match ret.vertex_label(t) {
                           Some(&ControlFlowTarget::Resolved(ref bb)) => bb.area.start == *src_off,
                            _ => false
                        }
                    });
                    let to_bb = ret.vertices().find(|&t| {
                        match ret.vertex_label(t) {
                            Some(&ControlFlowTarget::Resolved(ref bb)) => bb.area.start == opt_tgt.unwrap(),
                            _ => false
                        }
                    });

                    match (from_bb,to_bb) {
                        (Some(from),Some(to)) => { ret.add_edge(gu.clone(),from,to); },
                        (None,Some(to)) => {
                            let vx = ret.add_vertex(ControlFlowTarget::Unresolved(Rvalue::Constant(*src_off)));
                            ret.add_edge(gu.clone(),vx,to);
                        },
                        (Some(from),None) => {
                            let vx = ret.add_vertex(ControlFlowTarget::Unresolved(Rvalue::Constant(opt_tgt.unwrap())));
                            ret.add_edge(gu.clone(),from,vx);
                        },
                        _ => error!("jump from {} to {} doesn't hit any blocks",src_off,opt_tgt.unwrap()),
                    }
                }
            }
        }

        ret
    }

    pub fn disassemble<I: Token>(cont: Option<Function>, dec: Rc<Disassembler<I>>, init: State<I>, data: LayerIter, start: u64) -> Function
    where <I as Not>::Output: NumCast,
          <I as BitAnd>::Output: NumCast,
          <I as BitOr>::Output: NumCast,
          <I as Shl<usize>>::Output: NumCast,
          <I as Shr<usize>>::Output: NumCast,
          I: Eq + PartialEq + Display
    {
        let name = cont.as_ref().map_or(format!("func_{}",start),|x| x.name.clone());
        let (mut mnemonics,mut by_source,mut by_destination) = cont.map_or(
            (BTreeMap::new(),HashMap::new(),HashMap::new()),|x| Self::index_cflow_graph(x.cflow_graph));
        let mut todo = BTreeSet::<u64>::new();

        todo.insert(start);

        while !todo.is_empty() {
            let addr = todo.iter().next().unwrap().clone();
            let maybe_mnes = mnemonics.iter().find(|x| *x.0 >= addr).map(|x| x.1.clone());

            todo.remove(&addr);

            if let Some(mnes) = maybe_mnes {
                if !mnes.is_empty() {
                    let a = mnes.first().unwrap().area.clone();
                    if a.start < addr && a.end > addr {
                        println!("jump inside mnemonic at {}",addr);
                    } else if a.start == addr {
                        // else: already disassembled
                        continue;
                    }
                }
            }

            let mut st = init.clone();
            let mut i = data.seek(addr);

            st.address = addr;

            let maybe_match = dec.next_match(&mut i,st);

            if let Some(match_st) = maybe_match {
                let mut last_mne_start = 0;

                println!("match with {} mnemonics",match_st.mnemonics.len());

                for mne in match_st.mnemonics {
                    last_mne_start = mne.area.start;
                    println!("match {} at {}..{}",mne.opcode,mne.area.start,mne.area.end);
                    mnemonics.entry(mne.area.start).or_insert(Vec::new()).push(mne);
                }

                for (tgt,gu) in match_st.jumps {
                    match tgt {
                        Rvalue::Constant(ref c) => {
                            by_source.entry(last_mne_start).or_insert(Vec::new()).push((Some(*c),gu.clone()));
                            by_destination.entry(*c).or_insert(Vec::new()).push((Some(last_mne_start),gu.clone()));
                            todo.insert(*c);
                        },
                        _ => {
                            by_source.entry(last_mne_start).or_insert(Vec::new()).push((None,gu.clone()));
                        }
                    }
                }
            } else {
                println!("failed to match anything at {}",addr);
            }
        }

        let cfg = Self::assemble_cflow_graph(mnemonics,by_source,by_destination,start);
        let e = cfg.vertices().find(|&vx| {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg.vertex_label(vx) {
                bb.area.start == start
            } else {
                false
            }
        });

        Function{
            name: name,
            cflow_graph: cfg,
            entry_point: e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph_algos::{VertexListGraphTrait,EdgeListGraphTrait};
    use graph_algos::{AdjacencyList,GraphTrait,MutableGraphTrait};
    use guard::Guard;
    use mnemonic::{Mnemonic,Bound};
    use basic_block::BasicBlock;
    use value::Rvalue;
    use layer::OpaqueLayer;
    use disassembler::{ToExpr,State};

    #[test]
    fn new() {
        let f = Function::new("test".to_string());

        assert_eq!(f.name, "test".to_string());
        assert_eq!(f.cflow_graph.num_vertices(), 0);
        assert_eq!(f.cflow_graph.num_edges(), 0);
        assert_eq!(f.entry_point, None);
    }

    #[test]
    fn index_resolved() {
        let mut cfg = ControlFlowGraph::new();

        let bb0 = BasicBlock::from_vec(vec!(
                Mnemonic::dummy(0..1),
                Mnemonic::dummy(1..2),
                Mnemonic::dummy(2..5),
                Mnemonic::dummy(5..6)));
        let bb1 = BasicBlock::from_vec(vec!(
                Mnemonic::dummy(10..11),
                Mnemonic::dummy(11..12),
                Mnemonic::dummy(12..15),
                Mnemonic::dummy(15..16)));
        let bb2 = BasicBlock::from_vec(vec!(
                Mnemonic::dummy(6..10)));

        let vx0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let vx1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let vx2 = cfg.add_vertex(ControlFlowTarget::Resolved(bb2));

        cfg.add_edge(Guard::new(),vx0,vx1);
        cfg.add_edge(Guard::new(),vx1,vx1);
        cfg.add_edge(Guard::new(),vx1,vx2);
        cfg.add_edge(Guard::new(),vx2,vx0);

        let (mnes,src,dest) = Function::index_cflow_graph(cfg);

        assert_eq!(mnes.len(),9);
        assert_eq!(src.values().fold(0,|acc,x| acc + x.len()),4);
        assert_eq!(dest.values().fold(0,|acc,x| acc + x.len()),4);

        let cfg_re = Function::assemble_cflow_graph(mnes,src,dest,0);

        println!("{:?}",cfg_re.vertices().map(|x| cfg_re.vertex_label(x)).collect::<Vec<_>>());
        assert_eq!(cfg_re.num_vertices(), 3);
        assert_eq!(cfg_re.num_edges(), 4);

        for vx in cfg_re.vertices() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = cfg_re.vertex_label(vx) {
                assert!(
                    (bb.area.start == 0 && bb.area.end == 6) ||
                    (bb.area.start == 10 && bb.area.end == 16) ||
                    (bb.area.start == 6 && bb.area.end == 10)
                );
            } else {
                unreachable!();
            }
        }

        for e in cfg_re.edges() {
            if let Some(&ControlFlowTarget::Resolved(ref from)) = cfg_re.vertex_label(cfg_re.source(e)) {
                if let Some(&ControlFlowTarget::Resolved(ref to)) = cfg_re.vertex_label(cfg_re.target(e)) {
                    assert!(
                        (from.area.start == 0 && to.area.start == 10) ||
                        (from.area.start == 10 && to.area.start == 10) ||
                        (from.area.start == 10 && to.area.start == 6) ||
                        (from.area.start == 6 && to.area.start == 0)
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

        let bb0 = BasicBlock::from_vec(vec!(
                Mnemonic::dummy(0..1)));
        let bb1 = BasicBlock::from_vec(vec!(
                Mnemonic::dummy(10..11)));

        let vx0 = cfg.add_vertex(ControlFlowTarget::Resolved(bb0));
        let vx1 = cfg.add_vertex(ControlFlowTarget::Resolved(bb1));
        let vx2 = cfg.add_vertex(ControlFlowTarget::Unresolved(Rvalue::Constant(42)));
        let vx3 = cfg.add_vertex(ControlFlowTarget::Unresolved(Rvalue::Constant(23)));
        let vx4 = cfg.add_vertex(ControlFlowTarget::Unresolved(Rvalue::Variable{ name: "a".to_string(), width:8, subscript: None }));

        cfg.add_edge(Guard::new(),vx0,vx1);
        cfg.add_edge(Guard::new(),vx2,vx1);
        cfg.add_edge(Guard::new(),vx3,vx0);
        cfg.add_edge(Guard::new(),vx4,vx3);

        let (mnes,src,dest) = Function::index_cflow_graph(cfg);

        assert_eq!(mnes.len(),2);
        assert_eq!(src.values().fold(0,|acc,x| acc + x.len()),3);
        assert_eq!(dest.values().fold(0,|acc,x| acc + x.len()),3);

        let cfg_re = Function::assemble_cflow_graph(mnes,src,dest,0);

        println!("{:?}",cfg_re.vertices().map(|x| cfg_re.vertex_label(x)).collect::<Vec<_>>());
        assert_eq!(cfg_re.num_vertices(), 4);
        assert_eq!(cfg_re.num_edges(), 3);

        for vx in cfg_re.vertices() {
            match cfg_re.vertex_label(vx) {
                Some(&ControlFlowTarget::Resolved(ref bb)) => {
                    assert!(
                        (bb.area.start == 0 && bb.area.end == 1) ||
                        (bb.area.start == 10 && bb.area.end == 11)
                    );
                },
                Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))) => {
                    assert!(*c == 42 || *c == 23);
                },
                _ => { unreachable!(); }
            }
        }
    }

    #[test]
    fn add_single() {
        let main = new_disassembler!(u8 =>
            [ 0 ] = |st: &mut State<u8>| {
                let next = st.address;
                st.mnemonic(1,"A","",vec!(),|_| {});
                true
            }
		);
        let data = OpaqueLayer::wrap(vec!(0));
        let init = State::new(0);
        let func = Function::disassemble(None,main,init,data.iter(),0);

        assert_eq!(func.cflow_graph.num_vertices(), 1);
        assert_eq!(func.cflow_graph.num_edges(), 0);

        if let Some(vx) = func.cflow_graph.vertices().next() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
                assert_eq!(bb.mnemonics.len(), 1);
                assert_eq!(bb.mnemonics[0].opcode, "A".to_string());
                assert_eq!(bb.mnemonics[0].area, Bound::new(0,1));
                assert_eq!(bb.area, Bound::new(0,1));
            } else {
                unreachable!();
            }
         } else {
            unreachable!();
        }

        assert_eq!(func.entry_point, func.cflow_graph.vertices().next());
        assert_eq!(func.name, "func_0".to_string());
    }

    /*
    TEST(procedure,continuous)
    {
        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,3,4,5});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, const std::string &n) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(1,n);
            st.jump(p+1);
            states.insert(std::make_pair(p,st));
        };
        auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_EQ(m.operands.size(), 0u);
            ASSERT_EQ(m.instructions.size(), 0u);
            ASSERT_EQ(m.area, po::bound(p,p+1));
        };

        add(0,"test0");
        add(1,"test1");
        add(2,"test2");
        add(3,"test3");
        add(4,"test4");
        add(5,"test5");

        disassembler_mockup mockup(states);
        boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
        ASSERT_TRUE(!!maybe_proc);
        proc_loc proc = *maybe_proc;

        ASSERT_TRUE(!!proc->entry);
        ASSERT_EQ(proc->rev_postorder().size(), 1u);

        po::bblock_loc bb = *proc->rev_postorder().begin();

        ASSERT_EQ(bb->mnemonics().size(), 6u);

        check(bb->mnemonics()[0],"test0",0);
        check(bb->mnemonics()[1],"test1",1);
        check(bb->mnemonics()[2],"test2",2);
        check(bb->mnemonics()[3],"test3",3);
        check(bb->mnemonics()[4],"test4",4);
        check(bb->mnemonics()[5],"test5",5);

        auto ep = edges(proc->control_transfers);
        using edge_descriptor = boost::graph_traits<decltype(procedure::control_transfers)>::edge_descriptor;
        ASSERT_TRUE(std::all_of(ep.first,ep.second,[&](edge_descriptor e) { try { get_edge(e,proc->control_transfers); return true; } catch(...) { return false; } }));

        auto in_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);
        auto out_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in_p.first,in_p.second), 0);
        ASSERT_EQ(distance(out_p.first,out_p.second), 1);
        ASSERT_TRUE(get_edge(*out_p.first,proc->control_transfers).relations.empty());
        ASSERT_TRUE(is_constant(get<rvalue>(get_vertex(target(*out_p.first,proc->control_transfers),proc->control_transfers))));
        ASSERT_EQ(to_constant(get<rvalue>(get_vertex(target(*out_p.first,proc->control_transfers),proc->control_transfers))).content(), 6u);
        ASSERT_EQ(bb->area(), po::bound(0,6));
        ASSERT_EQ(bb, *(proc->entry));
        ASSERT_NE(proc->name, "");
    }

    TEST(procedure,branch)
    {
        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, const std::string &n, po::offset b1, boost::optional<po::offset> b2) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(1,n);
            st.jump(b1);
            if(b2)
                st.jump(*b2);
            states.insert(std::make_pair(p,st));
        };
        auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_TRUE(m.operands.empty());
            ASSERT_TRUE(m.instructions.empty());
            ASSERT_EQ(m.area, po::bound(p,p+1));
        };

        add(0,"test0",1,2);
        add(1,"test1",3,boost::none);
        add(2,"test2",1,boost::none);

        disassembler_mockup mockup(states);
        boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
        ASSERT_TRUE(!!maybe_proc);
        proc_loc proc = *maybe_proc;

        ASSERT_TRUE(!!proc->entry);
        ASSERT_EQ(proc->rev_postorder().size(), 3u);

        auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
        auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });
        auto i2 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 2; });

        ASSERT_NE(i0, proc->rev_postorder().end());
        ASSERT_NE(i1, proc->rev_postorder().end());
        ASSERT_NE(i2, proc->rev_postorder().end());

        po::bblock_loc bb0 = *i0;
        po::bblock_loc bb1 = *i1;
        po::bblock_loc bb2 = *i2;

        ASSERT_EQ(bb0->mnemonics().size(), 1u);
        ASSERT_EQ(bb1->mnemonics().size(), 1u);
        ASSERT_EQ(bb2->mnemonics().size(), 1u);

        auto in0_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);
        auto out0_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in0_p.first,in0_p.second), 0);
        check(bb0->mnemonics()[0],"test0",0);
        ASSERT_EQ(distance(out0_p.first,out0_p.second), 2);

        auto in1_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);
        auto out1_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in1_p.first,in1_p.second), 2);
        check(bb1->mnemonics()[0],"test1",1);
        ASSERT_EQ(distance(out1_p.first,out1_p.second), 1);

        auto in2_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb2),proc->control_transfers),proc->control_transfers);
        auto out2_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb2),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in2_p.first,in2_p.second), 1);
        check(bb2->mnemonics()[0],"test2",2);
        ASSERT_EQ(distance(out2_p.first,out2_p.second), 1);
    }

    TEST(procedure,loop)
    {
        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, const std::string &n, po::offset b1) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(1,n);
            st.jump(b1);
            states.insert(std::make_pair(p,st));
        };
        auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_TRUE(m.operands.empty());
            ASSERT_TRUE(m.instructions.empty());
            ASSERT_EQ(m.area, po::bound(p,p+1));
        };

        add(0,"test0",1);
        add(1,"test1",2);
        add(2,"test2",0);

        disassembler_mockup mockup(states);
        boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
        ASSERT_TRUE(!!maybe_proc);
        proc_loc proc = *maybe_proc;

        ASSERT_EQ(proc->rev_postorder().size(), 1u);

        po::bblock_loc bb = *proc->rev_postorder().begin();

        ASSERT_EQ(bb->mnemonics().size(), 3u);

        check(bb->mnemonics()[0],"test0",0);
        check(bb->mnemonics()[1],"test1",1);
        check(bb->mnemonics()[2],"test2",2);

        auto in_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);
        auto out_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in_p.first,in_p.second), 1);
        ASSERT_EQ(distance(out_p.first,out_p.second), 1);
    }

    TEST(procedure,empty)
    {
        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        disassembler_mockup mockup(states);
        boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
        ASSERT_TRUE(!maybe_proc);
    }

    TEST(procedure,refine)
    {
        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,3});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, size_t l, const std::string &n, po::offset b1) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(l,n);
            st.jump(b1);
            states.insert(std::make_pair(p,st));
        };
        /*auto check = [&](const po::mnemonic &m, const std::string &n, po::bound p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_TRUE(m.operands.empty());
            ASSERT_TRUE(m.instructions.empty());
            ASSERT_EQ(m.area, p);
        };*/

        /*
         * test0
         *  -"-  test1
         * test2
         */
        add(0,2,"test0",2);
        add(2,1,"test2",1);
        add(1,1,"test1",2);

        disassembler_mockup mockup(states);
        boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
        ASSERT_TRUE(!!maybe_proc);
        proc_loc proc = *maybe_proc;
        boost::write_graphviz(std::cout,proc->control_transfers,proc_writer(proc));

        // XXX: Disabled until functionality is needed
        /*
        ASSERT_EQ(proc->rev_postorder().size(), 2);
        auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
        auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });
        ASSERT_NE(i0, proc->rev_postorder().end());
        ASSERT_NE(i1, proc->rev_postorder().end());
        po::bblock_loc bb0 = *i0;
        po::bblock_loc bb1 = *i1;
        ASSERT_EQ(bb0->mnemonics().size(), 1);
        ASSERT_EQ(bb1->mnemonics().size(), 2);
        check(bb0->mnemonics()[0],"test0",po::bound(0,2));
        check(bb1->mnemonics()[0],"test1",po::bound(0,2));
        check(bb1->mnemonics()[1],"test2",po::bound(2,3));
        auto in0_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);
        auto out0_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);
        ASSERT_EQ(distance(in0_p.first,in0_p.second), 0);
        ASSERT_EQ(distance(out0_p.first,out0_p.second), 1);
        auto in1_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);
        auto out1_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);
        ASSERT_EQ(distance(in1_p.first,in1_p.second), 2);
        ASSERT_EQ(distance(out1_p.first,out1_p.second), 1);*/
    }

    TEST(procedure,continue)
    {
        rdf::storage store;
        po::proc_loc proc(new po::procedure(""));
        po::mnemonic mne0(po::bound(0,1),"test0","",{},{});
        po::mnemonic mne1(po::bound(1,2),"test1","",{},{});
        po::mnemonic mne2(po::bound(2,3),"test2","",{},{});
        po::mnemonic mne3(po::bound(6,7),"test6","",{},{});
        po::bblock_loc bb0(new po::basic_block());
        po::bblock_loc bb1(new po::basic_block());
        po::bblock_loc bb2(new po::basic_block());

        insert_vertex(variant<bblock_loc,rvalue>(bb0),proc.write().control_transfers);
        insert_vertex(variant<bblock_loc,rvalue>(bb1),proc.write().control_transfers);
        insert_vertex(variant<bblock_loc,rvalue>(bb2),proc.write().control_transfers);

        save_point(store);

        find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers);
        find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers);
        find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers);

        bb0.write().mnemonics().push_back(mne0);
        bb0.write().mnemonics().push_back(mne1);
        bb1.write().mnemonics().push_back(mne2);
        bb2.write().mnemonics().push_back(mne3);

        unconditional_jump(proc,bb2,po::constant(40));
        unconditional_jump(proc,bb0,bb1);
        unconditional_jump(proc,bb0,bb2);

        proc.write().entry = bb0;

        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,40,41,42});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, const std::string &n, boost::optional<po::offset> b1, boost::optional<po::offset> b2) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(1,n);
            if(b1)
                st.jump(*b1);
            if(b2)
                st.jump(*b2);

            states.insert(std::make_pair(p,st));
        };
        auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_TRUE(m.operands.empty());
            ASSERT_TRUE(m.instructions.empty());
            ASSERT_EQ(m.area, po::bound(p,p+1));
        };

        add(0,"test0",1,boost::none);
        add(1,"test1",2,6);
        add(2,"test2",boost::none,boost::none);
        add(6,"test6",40,boost::none);

        add(40,"test40",41,boost::none);
        add(41,"test41",42,boost::none);
        add(42,"test42",55,make_optional<offset>(0));

        disassembler_mockup mockup(states);
        ASSERT_TRUE(!!proc->entry);

        boost::optional<proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(proc,mockup,'a',slab(bytes.data(),bytes.size()),40);
        ASSERT_TRUE(!!maybe_proc);

        proc = *maybe_proc;

        ASSERT_TRUE(!!proc->entry);
        ASSERT_EQ(proc->rev_postorder().size(), 4u);

        auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
        auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 2; });
        auto i2 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 6; });
        auto i3 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 40; });

        ASSERT_NE(i0, proc->rev_postorder().end());
        ASSERT_NE(i1, proc->rev_postorder().end());
        ASSERT_NE(i2, proc->rev_postorder().end());
        ASSERT_NE(i3, proc->rev_postorder().end());

        po::bblock_loc bbo0 = *i0;
        po::bblock_loc bbo1 = *i1;
        po::bblock_loc bbo2 = *i2;
        po::bblock_loc bbo3 = *i3;
        auto ct = proc->control_transfers;

        auto in0_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo0),proc->control_transfers),proc->control_transfers);
        auto out0_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo0),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in0_p.first,in0_p.second), 1);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in0_p.first,ct),ct)) == bbo3);
        ASSERT_EQ(bbo0->mnemonics().size(), 2u);
        check(bbo0->mnemonics()[0],"test0",0);
        check(bbo0->mnemonics()[1],"test1",1);
        ASSERT_EQ(distance(out0_p.first,out0_p.second), 2);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(target(*out0_p.first,ct),ct)) == bbo1 || get<bblock_loc>(get_vertex(target(*out0_p.first,ct),ct)) == bbo2);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(target(*(out0_p.first+1),ct),ct)) == bbo1 || get<bblock_loc>(get_vertex(target(*(out0_p.first+1),ct),ct)) == bbo2);

        auto in1_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo1),proc->control_transfers),proc->control_transfers);
        auto out1_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo1),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in1_p.first,in1_p.second), 1);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in1_p.first,ct),ct)) == bbo0);
        ASSERT_EQ(bbo1->mnemonics().size(), 1u);
        check(bbo1->mnemonics()[0],"test2",2);
        ASSERT_EQ(distance(out1_p.first,out1_p.second), 0);

        auto in2_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo2),proc->control_transfers),proc->control_transfers);
        auto out2_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo2),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in2_p.first,in2_p.second), 1);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in2_p.first,ct),ct)) == bbo0);
        ASSERT_EQ(bbo2->mnemonics().size(), 1u);
        check(bbo2->mnemonics()[0],"test6",6);
        ASSERT_EQ(distance(out2_p.first,out2_p.second), 1);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(target(*out2_p.first,ct),ct)) == bbo3);

        auto in3_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo3),proc->control_transfers),proc->control_transfers);
        auto out3_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo3),proc->control_transfers),proc->control_transfers);

        ASSERT_EQ(distance(in3_p.first,in3_p.second), 1);
        ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in3_p.first,ct),ct)) == bbo2);
        ASSERT_EQ(bbo3->mnemonics().size(), 3u);
        check(bbo3->mnemonics()[0],"test40",40);
        check(bbo3->mnemonics()[1],"test41",41);
        check(bbo3->mnemonics()[2],"test42",42);
        ASSERT_EQ(distance(out3_p.first,out3_p.second), 2);
        ASSERT_TRUE((get<bblock_loc>(&get_vertex(target(*out3_p.first,ct),ct)) && get<bblock_loc>(get_vertex(target(*out3_p.first,ct),ct)) == bbo0) ||
                                (get<rvalue>(&get_vertex(target(*out3_p.first,ct),ct)) && to_constant(get<rvalue>(get_vertex(target(*out3_p.first,ct),ct))).content() == 55));
        ASSERT_TRUE((get<bblock_loc>(&get_vertex(target(*(out3_p.first+1),ct),ct)) && get<bblock_loc>(get_vertex(target(*(out3_p.first+1),ct),ct)) == bbo0) ||
                                (get<rvalue>(&get_vertex(target(*(out3_p.first+1),ct),ct)) && to_constant(get<rvalue>(get_vertex(target(*(out3_p.first+1),ct),ct))).content() == 55));

        ASSERT_EQ(*(proc->entry), bbo0);
    }

    TEST(procedure,entry_split)
    {
        po::proc_loc proc(new po::procedure(""));
        po::mnemonic mne0(po::bound(0,1),"test0","",{},{});
        po::mnemonic mne1(po::bound(1,2),"test1","",{},{});
        po::bblock_loc bb0(new po::basic_block());

        insert_vertex(variant<bblock_loc,rvalue>(bb0),proc.write().control_transfers);

        bb0.write().mnemonics().push_back(mne0);
        bb0.write().mnemonics().push_back(mne1);

        unconditional_jump(proc,bb0,po::constant(2));

        proc.write().entry = bb0;

        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, const std::string &n, boost::optional<po::offset> b1, boost::optional<po::offset> b2) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(1,n);
            if(b1)
                st.jump(*b1);
            if(b2)
                st.jump(*b2);

            states.insert(std::make_pair(p,st));
        };
        auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_TRUE(m.operands.empty());
            ASSERT_TRUE(m.instructions.empty());
            ASSERT_EQ(m.area, po::bound(p,p+1));
        };

        add(0,"test0",1,boost::none);
        add(1,"test1",2,boost::none);

        add(2,"test2",1,boost::none);

        disassembler_mockup mockup(states);
        boost::optional<proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(proc,mockup,'a',slab(bytes.data(),bytes.size()),2);
        ASSERT_TRUE(!!maybe_proc);

        proc = *maybe_proc;

        ASSERT_EQ(proc->rev_postorder().size(), 3u);

        auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
        auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });
        auto i2 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 2; });

        ASSERT_NE(i0, proc->rev_postorder().end());
        ASSERT_NE(i1, proc->rev_postorder().end());
        ASSERT_NE(i2, proc->rev_postorder().end());

        po::bblock_loc bbo0 = *i0;
        po::bblock_loc bbo1 = *i1;
        po::bblock_loc bbo2 = *i2;

        ASSERT_EQ(*(proc->entry), bbo0);
        ASSERT_EQ(bbo0->mnemonics().size(), 1u);
        check(bbo0->mnemonics()[0],"test0",0);
        ASSERT_EQ(bbo1->mnemonics().size(), 1u);
        check(bbo1->mnemonics()[0],"test1",1);
        ASSERT_EQ(bbo2->mnemonics().size(), 1u);
        check(bbo2->mnemonics()[0],"test2",2);
    }

    /*
     *   bb0 ----+
     *    |  \   |
     *   bb1  a  |
     *   /  \    |
     *   bb2 \   |
     *   \   /   |
     * +-bb3---2 |
     * +/ |      |
     *    bb4----+
     */
    TEST(procedure,marshal)
    {
        bblock_loc bb0(new basic_block({mnemonic(bound(0,5),"test","",{},{})}));
        bblock_loc bb1(new basic_block({mnemonic(bound(5,10),"test","",{},{})}));
        bblock_loc bb2(new basic_block({mnemonic(bound(10,12),"test","",{},{})}));
        bblock_loc bb3(new basic_block({mnemonic(bound(12,20),"test","",{},{})}));
        bblock_loc bb4(new basic_block({mnemonic(bound(20,21),"test","",{},{})}));
        rvalue rv1 = variable("a",8);
        rvalue rv2 = constant(42);
        proc_loc proc(new procedure("p1"));

        auto vx0 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb0,proc.write().control_transfers);
        auto vx1 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb1,proc.write().control_transfers);
        auto vx2 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb2,proc.write().control_transfers);
        auto vx3 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb3,proc.write().control_transfers);
        auto vx4 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb4,proc.write().control_transfers);
        auto vx5 = insert_vertex<variant<bblock_loc,rvalue>,guard>(rv1,proc.write().control_transfers);
        auto vx6 = insert_vertex<variant<bblock_loc,rvalue>,guard>(rv2,proc.write().control_transfers);

        insert_edge(guard(),vx0,vx1,proc.write().control_transfers);
        insert_edge(guard(),vx0,vx5,proc.write().control_transfers);
        insert_edge(guard(),vx1,vx2,proc.write().control_transfers);
        insert_edge(guard(),vx2,vx3,proc.write().control_transfers);
        insert_edge(guard(),vx1,vx3,proc.write().control_transfers);
        insert_edge(guard(),vx3,vx3,proc.write().control_transfers);
        insert_edge(guard(),vx3,vx6,proc.write().control_transfers);
        insert_edge(guard(),vx3,vx4,proc.write().control_transfers);
        insert_edge(guard(),vx4,vx0,proc.write().control_transfers);

        proc.write().entry = bb0;

        rdf::storage st;
        save_point(st);

        for(auto s: st.all())
            std::cout << s << std::endl;

        proc_loc p2(proc.tag(),unmarshal<procedure>(proc.tag(),st));

        ASSERT_EQ(proc->name, p2->name);
        ASSERT_TRUE(**proc->entry == **p2->entry);
        ASSERT_EQ(num_vertices(p2->control_transfers), num_vertices(proc->control_transfers));
        ASSERT_EQ(num_edges(p2->control_transfers), num_edges(proc->control_transfers));
        ASSERT_EQ(proc->rev_postorder().size(), p2->rev_postorder().size());
    }

    using sw = po::sem_state<wtest_tag>&;
    TEST(procedure,wide_token)
    {
        std::vector<uint8_t> _buf = {0x22,0x11, 0x44,0x33, 0x44,0x55, 0x44,0x55};
        po::slab buf(_buf.data(),_buf.size());
        po::disassembler<wtest_tag> dec;

        dec[0x1122] = [](sw s)
        {
            s.mnemonic(2,"A");
            s.jump(s.address + 2);
            return true;
        };

        dec[0x3344] = [](sw s)
        {
            s.mnemonic(2,"B");
            s.jump(s.address + 2);
            s.jump(s.address + 4);
            return true;
        };

        dec[0x5544] = [](sw s)
        {
            s.mnemonic(2, "C");
            return true;
        };

        boost::optional<proc_loc> maybe_proc = procedure::disassemble<wtest_tag,po::disassembler<wtest_tag>>(boost::none, dec,'a', buf, 0);
        ASSERT_TRUE(!!maybe_proc);
        proc_loc proc = *maybe_proc;

        EXPECT_EQ(num_vertices(proc->control_transfers), 3u);
        EXPECT_EQ(num_edges(proc->control_transfers), 2u);

        using vx_desc = digraph<boost::variant<bblock_loc,rvalue>,guard>::vertex_descriptor;
        auto p = vertices(proc->control_transfers);
        size_t sz = std::count_if(p.first,p.second,[&](const vx_desc& v)
        {
            try
            {
                bblock_loc bb = boost::get<bblock_loc>(get_vertex(v,proc->control_transfers));
                return bb->area() == po::bound(0,4) && bb->mnemonics().size() == 2;
            }
            catch(const boost::bad_get&)
            {
                return false;
            }
        });
        EXPECT_EQ(sz, 1u);
        sz = std::count_if(p.first,p.second,[&](const vx_desc& v)
        {
            try
            {
                bblock_loc bb = boost::get<bblock_loc>(get_vertex(v,proc->control_transfers));
                return bb->area() == po::bound(4,6) && bb->mnemonics().size() == 1;
            }
            catch(const boost::bad_get&)
            {
                return false;
            }
        });
        EXPECT_EQ(sz, 1u);
        sz = std::count_if(p.first,p.second,[&](const vx_desc& v)
        {
            try
            {
                bblock_loc bb = boost::get<bblock_loc>(get_vertex(v,proc->control_transfers));
                return bb->area() == po::bound(6,8) && bb->mnemonics().size() == 1;
            }
            catch(const boost::bad_get&)
            {
                return false;
            }
        });
        EXPECT_EQ(sz, 1u);
    }

    TEST(procedure,issue_51_treat_entry_point_as_incoming_edge)
    {
        std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
        std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
        auto add = [&](po::offset p, const std::string &n, po::offset b1) -> void
        {
            po::sem_state<test_tag> st(p,'a');
            st.mnemonic(1,n);
            st.jump(b1);

            states.insert(std::make_pair(p,st));
        };
        auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
        {
            ASSERT_EQ(m.opcode, n);
            ASSERT_TRUE(m.operands.empty());
            ASSERT_TRUE(m.instructions.empty());
            ASSERT_EQ(m.area, po::bound(p,p+1));
        };

        add(0,"test0",1);
        add(1,"test1",2);

        add(2,"test2",0);

        disassembler_mockup mockup(states);
        boost::optional<proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),1);
        ASSERT_TRUE(!!maybe_proc);

        proc_loc proc = *maybe_proc;

        ASSERT_EQ(proc->rev_postorder().size(), 2u);

        auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
        auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });

        ASSERT_NE(i0, proc->rev_postorder().end());
        ASSERT_NE(i1, proc->rev_postorder().end());

        po::bblock_loc bbo0 = *i0;
        po::bblock_loc bbo1 = *i1;

        ASSERT_EQ(*(proc->entry), bbo1);
        ASSERT_EQ(bbo0->mnemonics().size(), 1u);
        check(bbo0->mnemonics()[0],"test0",0);
        ASSERT_EQ(bbo1->mnemonics().size(), 2u);
    }
*/
}
