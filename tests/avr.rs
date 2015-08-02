extern crate panopticon;

use panopticon::region::Region;
use panopticon::avr::{Mcu,disassembler,Avr};
use panopticon::disassembler::State;
use std::path::Path;

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
