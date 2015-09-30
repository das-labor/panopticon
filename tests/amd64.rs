extern crate panopticon;
extern crate graph_algos;

use panopticon::region::Region;
use panopticon::amd64::{disassembler,Amd64,Config,Mode};
use panopticon::function::{ControlFlowTarget,Function};
use panopticon::disassembler::State;
use panopticon::value::Rvalue;

use std::path::Path;
use std::hash::{Hash,Hasher,SipHasher};

use graph_algos::traits::{VertexListGraph,Graph,MutableGraph,IncidenceGraph,EdgeListGraph};

#[test]
fn amd64_opcodes() {
    let reg = Region::open("com".to_string(),Path::new("tests/data/amd64.com")).unwrap();
    let main = disassembler(Mode::Long);
    let mut addr = 0;

    loop {
        let st = State::<Amd64>::new(addr,Config::new(Mode::Long));
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
fn ia32_opcodes() {
    let reg = Region::open("com".to_string(),Path::new("tests/data/ia32.com")).unwrap();
    let main = disassembler(Mode::Protected);
    let mut addr = 0;

    loop {
        let st = State::<Amd64>::new(addr,Config::new(Mode::Long));
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
