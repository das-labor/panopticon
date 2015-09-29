extern crate panopticon;
extern crate graph_algos;

use panopticon::region::Region;
use panopticon::avr::{Mcu,disassembler,Avr};
use panopticon::function::{ControlFlowTarget,Function};
use panopticon::disassembler::State;
use panopticon::value::Rvalue;

use std::path::Path;
use std::hash::{Hash,Hasher,SipHasher};

use graph_algos::traits::{VertexListGraph,Graph,EdgeListGraph};

#[test]
fn avr_opcodes_01() {
    let reg = Region::open("flash".to_string(),Path::new("tests/data/avr-all-opcodes.bin")).unwrap();
    let main = disassembler();
    let mut addr = 0;

    loop {
        let st = State::<Avr>::new(addr,Mcu::new());
        let mut i = reg.iter().seek(addr);

        let maybe_match = main.next_match(&mut i,st);

        if let Some(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}",mne.area.start,mne.opcode);
                addr = mne.area.end;
            }
        } else if addr < reg.size() {
            unreachable!("failed to match anything at {:x}",addr);
        } else {
            break;
        }
    }
}

#[test]
fn avr_brne() {
    let reg = Region::wrap("flash".to_string(),
        vec!(
            0xde,0x01, //  0: movw
            0x11,0x96, //  2: adiw
            0x88,0xe0, //  4: ldi
            0x0d,0x90, //  6: ld
            0x01,0x92, //  8: st
            0x81,0x50, // 10: subi
            0xe1,0xf7, // 12: brne
            0x81,0xe0, // 14: ldi
            0x01,0xc0, // 16: rjmp
            0x80,0xe0, // 18: ldi
            0x68,0x96, // 20: adiw
            0xe4,0xe0  // 22: ldi
        ));
    let main = disassembler();
    let fun = Function::disassemble::<Avr>(None,main,Mcu::new(),reg.iter(),0,reg.name().to_string());

    for x in fun.cflow_graph.edges() {
        let cg = &fun.cflow_graph;
        let from = cg.source(x);
        let to = cg.target(x);
        let from_ident = to_ident(cg.vertex_label(from));
        let to_ident = to_ident(cg.vertex_label(to));

        if from_ident.is_some() && to_ident.is_some() {
            println!("{} -> {}",from_ident.unwrap(),to_ident.unwrap());
        }
    }
}

fn to_ident(t: Option<&ControlFlowTarget>) -> Option<String> {
    match t {
        Some(&ControlFlowTarget::Resolved(ref bb)) => Some(format!("\"bb{}\"",bb.area.start)),
        Some(&ControlFlowTarget::Unresolved(Rvalue::Constant(ref c))) => Some(format!("\"v{}\"",c)),
        Some(&ControlFlowTarget::Unresolved(ref c)) => {
            let ref mut h = SipHasher::new();
            c.hash::<SipHasher>(h);
            Some(format!("\"c{}\"",h.finish()))
        },
        _ => None,
    }
}

#[test]
fn avr_jmp_overflow() {
    let reg = Region::open("flash".to_string(),Path::new("tests/data/avr-jmp-overflow.bin")).unwrap();
    let main = disassembler();
    let fun = Function::disassemble::<Avr>(None,main,Mcu::atmega88(),reg.iter(),0,reg.name().to_string());

    assert_eq!(fun.cflow_graph.num_vertices(), 2);
    assert_eq!(fun.cflow_graph.num_edges(), 2);

    let mut vxs = fun.cflow_graph.vertices();
    if let Some(&ControlFlowTarget::Resolved(ref bb1)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
        if let Some(&ControlFlowTarget::Resolved(ref bb2)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
            assert!(bb1.area.start == 0 || bb1.area.start == 6000);
            assert!(bb2.area.start == 0 || bb2.area.start == 6000);
            assert!(bb1.area.end == 2 || bb1.area.end == 6004 );
            assert!(bb2.area.end == 2 || bb2.area.end == 6004 );
        }
    }
}

#[test]
fn avr_wrap_around() {
    let reg = Region::open("flash".to_string(),Path::new("tests/data/avr-overflow.bin")).unwrap();
    let main = disassembler();
    let fun = Function::disassemble::<Avr>(None,main,Mcu::atmega88(),reg.iter(),0,reg.name().to_string());

    assert_eq!(fun.cflow_graph.num_vertices(), 2);
    assert_eq!(fun.cflow_graph.num_edges(), 2);

    let mut vxs = fun.cflow_graph.vertices();
    if let Some(&ControlFlowTarget::Resolved(ref bb1)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
        if let Some(&ControlFlowTarget::Resolved(ref bb2)) = fun.cflow_graph.vertex_label(vxs.next().unwrap()) {
            assert!(bb1.area.start == 0 || bb1.area.start == 8190);
            assert!(bb2.area.start == 0 || bb2.area.start == 8190);
            assert!(bb1.area.end == 2 || bb1.area.end == 8192 );
            assert!(bb2.area.end == 2 || bb2.area.end == 8192 );
        }
    }
}
