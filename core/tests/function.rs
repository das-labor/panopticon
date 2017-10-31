#[macro_use]
extern crate panopticon_core;
extern crate petgraph;
extern crate env_logger;

use std::sync::Arc;
use petgraph::dot::Dot;

//use panopticon_core::*;
use panopticon_core::{Architecture, Bitcode, Disassembler, Guard, Match, Region, Result, Rvalue, State,
                      TestArch, BasicBlockIndex, Variable, Value, Constant,
                      Function, CfgNode,
                      Mnemonic};
use panopticon_core::il::neo::{Statement, Operation};
#[macro_use]
use panopticon_core::il;

#[derive(Clone,Debug)]
enum TestArchShort {}
impl Architecture for TestArchShort {
    type Token = u8;
    type Configuration = Arc<Disassembler<TestArchShort>>;

    fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
        unimplemented!()
    }

    fn decode(reg: &Region, addr: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
        if let Some(s) = cfg.next_match(&mut reg.iter(addr)?, addr, cfg.clone()) {
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
        if let Some(s) = cfg.next_match(&mut reg.iter(addr)?, addr, cfg.clone()) {
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
    let data = vec![0];
    let reg = Region::with("".to_string(), data);
    let func: Function<Bitcode> = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 1);
    assert_eq!(func.cfg().edge_count(), 0);

    let node = func.cfg().node_indices().next().unwrap();
    assert!(if let Some(&CfgNode::BasicBlock(_)) = func.cfg().node_weight(node) { true } else { false });

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

    let data = vec![0, 1, 2, 3, 4, 5];
    let reg = Region::with("".to_string(), data);
    let func: Function = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 2);
    assert_eq!(func.cfg().edge_count(), 1);

    for n in func.cfg().node_indices() {
        match func.cfg().node_weight(n) {
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

    let data = vec![0, 1, 2];
    let reg = Region::with("".to_string(), data);
    let func: Function = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 4);
    assert_eq!(func.cfg().edge_count(), 4);

    let mut bb0_vx = None;
    let mut bb1_vx = None;
    let mut bb2_vx = None;
    let mut ures_vx = None;

    for n in func.cfg().node_indices() {
        match func.cfg().node_weight(n) {
            Some(&CfgNode::BasicBlock(bb_idx)) => {
                let bb = func.basic_block(bb_idx);
                let mnes = func.mnemonics(bb_idx).collect::<Vec<_>>();

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

    let entry_bb = func.entry_point();
    assert_eq!(func.basic_block(entry_bb).node, bb0_vx.unwrap());
    assert_eq!(func.name, "func_0");
    assert!(func.cfg().find_edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
    assert!(func.cfg().find_edge(bb0_vx.unwrap(), bb2_vx.unwrap()).is_some());
    assert!(func.cfg().find_edge(bb1_vx.unwrap(), ures_vx.unwrap()).is_some());
    assert!(func.cfg().find_edge(bb2_vx.unwrap(), bb1_vx.unwrap()).is_some());
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

    let data = vec![0, 1, 2];
    let reg = Region::with("".to_string(), data);
    let func: Function =  Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 1);
    assert_eq!(func.cfg().edge_count(), 1);

    let vx = func.cfg().node_indices().next().unwrap();
    if let Some(&CfgNode::BasicBlock(bb_idx)) = func.cfg().node_weight(vx) {
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
    assert!(func.cfg().find_edge(vx, vx).is_some());
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

    let data = vec![];
    let reg = Region::with("".to_string(), data);
    let func: Result<Function> = Function::new::<TestArchShort>(main, 0, &reg, None);
    assert!(func.is_err());
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

    let data = vec![0, 1, 2, 3, 4, 5];
    let reg = Region::with("".to_string(), data);
    let mut func: Function =  Function::new::<TestArchShort>(main.clone(), 0, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 2);
    assert_eq!(func.cfg().edge_count(), 1);

    for n in func.cfg().node_indices() {
        match func.cfg().node_weight(n) {
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

    assert_eq!(func.cfg().node_count(), 2);
    assert_eq!(func.cfg().edge_count(), 1);

    for n in func.cfg().node_indices() {
        match func.cfg().node_weight(n) {
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

    let data = vec![0, 1];
    let reg = Region::with("".to_string(), data);
    let mut func: Function =  Function::new::<TestArchShort>(main.clone(), 0, &reg, None).unwrap();
    let unres = func.indirect_jumps().collect::<Vec<_>>();
    assert_eq!(unres.len(), 1);
    assert_eq!(unres[0], Variable{ name: "A".into(), bits: 1, subscript: None });

    assert!(func.resolve_indirect_jump(Variable{ name: "A".into(), bits: 1, subscript: None },Constant::new(1,1).unwrap()));
    assert!(func.extend::<TestArchShort>(main, &reg).is_ok());

    assert_eq!(func.cfg().node_count(), 2);
    assert_eq!(func.cfg().edge_count(), 1);

    let mut bb0_vx = None;
    let mut bb1_vx = None;

    for n in func.cfg().node_indices() {
        match func.cfg().node_weight(n) {
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
    let def = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x44];
    let reg = Region::with("".to_string(), def);
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

    let func: Function =  Function::new::<TestArchWide>(dec, 0, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 3);
    assert_eq!(func.cfg().edge_count(), 2);

    let mut bb0_vx = None;
    let mut bb1_vx = None;

    for vx in func.cfg().node_indices() {
        match func.cfg().node_weight(vx) {
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

    let data = vec![0, 1, 2];
    let reg = Region::with("".to_string(), data);
    let func: Function =  Function::new::<TestArchShort>(main, 1, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 2);
    assert_eq!(func.cfg().edge_count(), 2);

    let mut bb0_vx = None;
    let mut bb1_vx = None;

    for vx in func.cfg().node_indices() {
        if let Some(&CfgNode::BasicBlock(bb_idx)) = func.cfg().node_weight(vx) {
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
    assert!(func.cfg().find_edge(bb0_vx.unwrap(), bb1_vx.unwrap()).is_some());
    assert!(func.cfg().find_edge(bb1_vx.unwrap(), bb0_vx.unwrap()).is_some());
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

    let data = vec![0, 1, 2];
    let reg = Region::with("".to_string(), data);
    let func: Function =  Function::new::<TestArchShort>(main, 1, &reg, None).unwrap();

    assert_eq!(func.cfg().node_count(), 3);
    assert_eq!(func.cfg().edge_count(), 3);

    let mut bb01_vx = None;
    let mut bb1_vx = None;
    let mut bb2_vx = None;

    for vx in func.cfg().node_indices() {
        if let Some(&CfgNode::BasicBlock(bb_idx)) = func.cfg().node_weight(vx) {
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
    assert!(func.cfg().find_edge(bb01_vx.unwrap(), bb2_vx.unwrap()).is_some());
    assert!(func.cfg().find_edge(bb1_vx.unwrap(), bb2_vx.unwrap()).is_some());
    assert!(func.cfg().find_edge(bb2_vx.unwrap(), bb01_vx.unwrap()).is_some());
}

#[test]
fn iter_statementsrange() {
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
    let data = vec![0, 1, 2, 3, 0, 0];
    let reg = Region::with("".to_string(), data);
    let func: Function<Bitcode> = Function::new::<TestArchShort>(main, 0, &reg, None).unwrap();

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
    let stmts = func.statements(mne_idx[1]).collect::<Vec<_>>();
    assert_eq!(stmts.len(), 1);
    if let &Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Variable(_)),.. } = &stmts[0] { ; } else { unreachable!() }

    let stmts = func.statements(mne_idx[0]).collect::<Vec<_>>();
    assert_eq!(stmts.len(), 2);
    if let &Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Variable(_)),.. } = &stmts[0] { ; } else { unreachable!() }
    if let &Statement::Expression{ op: Operation::Subtract(Value::Variable(_),Value::Variable(_)),.. } = &stmts[1] { ; } else { unreachable!() }
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
    let data = b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec();
    let reg = Region::with("".to_string(), data);
    let mut func: Function = Function::new::<TestArch>((), 0, &reg, None).unwrap();
    let _ = func.rewrite(|basic_blocks| {
        for bb in basic_blocks {
            for &mut (_,ref mut stmts) in bb {
                for stmt in stmts {
                    match stmt {
                        &mut Statement::Expression{ result: Variable{ ref mut name,.. },.. } => {
                            *name = name.to_string().to_uppercase().into();
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }).unwrap();

    let b0 = func.statements(BasicBlockIndex::new(0)).collect::<Vec<_>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
    assert_eq!(b0.len(), 2);

    let b1 = func.statements(BasicBlockIndex::new(1)).collect::<Vec<_>>();
    if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[0] {} else { unreachable!() }
    assert_eq!(b1.len(), 1);

    let b2 = func.statements(BasicBlockIndex::new(2)).collect::<Vec<_>>();
    if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
    assert_eq!(b2.len(), 1);

    let b3 = func.statements(BasicBlockIndex::new(3)).collect::<Vec<_>>();
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

#[test]
fn rewrite_add_mnemonic() {
    let _ = env_logger::init();
    let data = b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec();
    let reg = Region::with("".to_string(), data);
    let mut func: Function = Function::new::<TestArch>((), 0, &reg, None).unwrap();
    let _ = func.rewrite(|basic_blocks| {
        let start = basic_blocks[1][0].0.area.start;
        let mne = Mnemonic::new(start..start,"test");
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

        basic_blocks[1].insert(0,(mne,stmts));
        Ok(())
    }).unwrap();

    let b0 = func.statements(BasicBlockIndex::new(0)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
    assert_eq!(b0.len(), 2);

    let b1 = func.statements(BasicBlockIndex::new(1)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::And(Value::Constant(_),Value::Variable(_)),.. } = b1[0] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::Subtract(Value::Constant(_),Value::Variable(_)),.. } = b1[1] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[2] {} else { unreachable!() }
    assert_eq!(b1.len(), 3);

    let b2 = func.statements(BasicBlockIndex::new(2)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
    assert_eq!(b2.len(), 1);

    let b3 = func.statements(BasicBlockIndex::new(3)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
    assert_eq!(b3.len(), 1);
}

#[test]
fn rewrite_add_statements() {
    let _ = env_logger::init();
    let data = b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec();
    let reg = Region::with("".to_string(), data);
    let mut func: Function<Bitcode> = Function::new::<TestArch>((), 0, &reg, None).unwrap();
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

    let b0 = func.statements(BasicBlockIndex::new(0)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
    assert_eq!(b0.len(), 2);

    let b1 = func.statements(BasicBlockIndex::new(1)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Add(Value::Variable(_),Value::Constant(_)),.. } = b1[0] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::And(Value::Constant(_),Value::Variable(_)),.. } = b1[1] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::Subtract(Value::Constant(_),Value::Variable(_)),.. } = b1[2] {} else { unreachable!() }
    assert_eq!(b1.len(), 3);

    let b2 = func.statements(BasicBlockIndex::new(2)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
    assert_eq!(b2.len(), 1);

    let b3 = func.statements(BasicBlockIndex::new(3)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
    assert_eq!(b3.len(), 1);
}

#[test]
fn rewrite_remove_mnemonic() {
    let _ = env_logger::init();
    let data = b"Mi1Cfi0Bf18Aii3J22Ai23Ms3R".to_vec();
    let reg = Region::with("".to_string(), data);
    let mut func: Function<Bitcode> = Function::new::<TestArch>((), 0, &reg, None).unwrap();
    let _ = func.rewrite(|basic_blocks| {
        basic_blocks[1].remove(0);
        Ok(())
    }).unwrap();

    let b0 = func.statements(BasicBlockIndex::new(0)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b0[0] {} else { unreachable!() }
    if let Statement::Expression{ op: Operation::LessOrEqualUnsigned(Value::Variable(_),Value::Constant(_)),.. } = b0[1] {} else { unreachable!() }
    assert_eq!(b0.len(), 2);

    let b1 = func.statements(BasicBlockIndex::new(1)).collect::<Vec<Statement>>();
    assert_eq!(b1.len(), 0);

    let b2 = func.statements(BasicBlockIndex::new(2)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Add(Value::Constant(_),Value::Constant(_)),.. } = b2[0] {} else { unreachable!() }
    assert_eq!(b2.len(), 1);

    let b3 = func.statements(BasicBlockIndex::new(3)).collect::<Vec<Statement>>();
    if let Statement::Expression{ op: Operation::Move(Value::Constant(_)),.. } = b3[0] {} else { unreachable!() }
    assert_eq!(b3.len(), 1);
}
