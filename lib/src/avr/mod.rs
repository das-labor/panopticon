use disassembler::*;
use program::Program;
use layer::LayerIter;
use value::{Lvalue,Rvalue,Endianess};
use codegen::CodeGen;
use guard::{Guard,Relation};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub struct AvrState {
    pc_bits: u16, // width of the program counter in bits (FLASHEND)
}

fn reg(st: &State<u16>, cap: &str) -> Lvalue {
    unimplemented!()
}

fn ioreg(st: &State<u16>, cap: &str) -> Lvalue {
    unimplemented!()
}

fn sram(off: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.clone()),
        name: "sram".to_string(),
        endianess: Endianess::Big,
        bytes: 1
    }
}

fn flash(off: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.clone()),
        name: "flash".to_string(),
        endianess: Endianess::Big,
        bytes: 2
    }
}

fn get_sp(cg: &mut CodeGen) -> Lvalue {
    unimplemented!()
}

fn set_sp(v: &Rvalue, cg: &mut CodeGen) {
    unimplemented!();
}

fn resolv(r: u16) -> Lvalue {
    unimplemented!()
}

static GLOBAL_AVR_TEMPVAR_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

fn new_temp(bits: usize) -> Lvalue {
    Lvalue::Variable{
        name: format!("__temp{}",GLOBAL_AVR_TEMPVAR_COUNT.fetch_add(1, Ordering::SeqCst)),
        width: bits as u16,
        subscript: None
    }
}

lazy_static! {
    static ref R0: Lvalue = Lvalue::Variable{ name: "r0".to_string(), width: 8, subscript: None };
    static ref R1: Lvalue = Lvalue::Variable{ name: "r1".to_string(), width: 8, subscript: None };
    static ref R2: Lvalue = Lvalue::Variable{ name: "r2".to_string(), width: 8, subscript: None };
    static ref R3: Lvalue = Lvalue::Variable{ name: "r3".to_string(), width: 8, subscript: None };
    static ref R4: Lvalue = Lvalue::Variable{ name: "r4".to_string(), width: 8, subscript: None };
    static ref R5: Lvalue = Lvalue::Variable{ name: "r5".to_string(), width: 8, subscript: None };
    static ref R6: Lvalue = Lvalue::Variable{ name: "r6".to_string(), width: 8, subscript: None };
    static ref R7: Lvalue = Lvalue::Variable{ name: "r7".to_string(), width: 8, subscript: None };
    static ref R8: Lvalue = Lvalue::Variable{ name: "r8".to_string(), width: 8, subscript: None };
    static ref R9: Lvalue = Lvalue::Variable{ name: "r9".to_string(), width: 8, subscript: None };
}

lazy_static! {
    static ref R10: Lvalue = Lvalue::Variable{ name: "r10".to_string(), width: 8, subscript: None };
    static ref R11: Lvalue = Lvalue::Variable{ name: "r11".to_string(), width: 8, subscript: None };
    static ref R12: Lvalue = Lvalue::Variable{ name: "r12".to_string(), width: 8, subscript: None };
    static ref R13: Lvalue = Lvalue::Variable{ name: "r13".to_string(), width: 8, subscript: None };
    static ref R14: Lvalue = Lvalue::Variable{ name: "r14".to_string(), width: 8, subscript: None };
    static ref R15: Lvalue = Lvalue::Variable{ name: "r15".to_string(), width: 8, subscript: None };
    static ref R16: Lvalue = Lvalue::Variable{ name: "r16".to_string(), width: 8, subscript: None };
    static ref R17: Lvalue = Lvalue::Variable{ name: "r17".to_string(), width: 8, subscript: None };
    static ref R18: Lvalue = Lvalue::Variable{ name: "r18".to_string(), width: 8, subscript: None };
    static ref R19: Lvalue = Lvalue::Variable{ name: "r19".to_string(), width: 8, subscript: None };
}

lazy_static! {
    static ref R20: Lvalue = Lvalue::Variable{ name: "r20".to_string(), width: 8, subscript: None };
    static ref R21: Lvalue = Lvalue::Variable{ name: "r21".to_string(), width: 8, subscript: None };
    static ref R22: Lvalue = Lvalue::Variable{ name: "r22".to_string(), width: 8, subscript: None };
    static ref R23: Lvalue = Lvalue::Variable{ name: "r23".to_string(), width: 8, subscript: None };
    static ref R24: Lvalue = Lvalue::Variable{ name: "r24".to_string(), width: 8, subscript: None };
    static ref R25: Lvalue = Lvalue::Variable{ name: "r25".to_string(), width: 8, subscript: None };
    static ref R26: Lvalue = Lvalue::Variable{ name: "r26".to_string(), width: 8, subscript: None };
    static ref R27: Lvalue = Lvalue::Variable{ name: "r27".to_string(), width: 8, subscript: None };
    static ref R28: Lvalue = Lvalue::Variable{ name: "r28".to_string(), width: 8, subscript: None };
    static ref R29: Lvalue = Lvalue::Variable{ name: "r29".to_string(), width: 8, subscript: None };
}

lazy_static! {
    static ref R30: Lvalue = Lvalue::Variable{ name: "r30".to_string(), width: 8, subscript: None };
    static ref R31: Lvalue = Lvalue::Variable{ name: "r31".to_string(), width: 8, subscript: None };

    static ref C: Lvalue = Lvalue::Variable{ name: "C".to_string(), width: 1, subscript: None };
    static ref P: Lvalue = Lvalue::Variable{ name: "P".to_string(), width: 1, subscript: None };
    static ref V: Lvalue = Lvalue::Variable{ name: "V".to_string(), width: 1, subscript: None };
    static ref I: Lvalue = Lvalue::Variable{ name: "I".to_string(), width: 1, subscript: None };
    static ref H: Lvalue = Lvalue::Variable{ name: "I".to_string(), width: 1, subscript: None };
    static ref T: Lvalue = Lvalue::Variable{ name: "T".to_string(), width: 1, subscript: None };
    static ref N: Lvalue = Lvalue::Variable{ name: "N".to_string(), width: 1, subscript: None };
    static ref S: Lvalue = Lvalue::Variable{ name: "S".to_string(), width: 1, subscript: None };
    static ref Z: Lvalue = Lvalue::Variable{ name: "Z".to_string(), width: 1, subscript: None };
}

pub fn disassemble(st: AvrState, data: LayerIter) -> Program {
    let simple = new_disassembler!(u16 =>
        // MOV
        [ "001011 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"mov","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd,rr.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // MOVW
        [ "00000001 d@.... r@...." ] = |st: &mut State<u16>| {
            let rd1 = reg(st,"d"); let rd2 = resolv(st.get_group("d") * 2 + 1);
            let rr1 = reg(st,"r"); let rr2 = resolv(st.get_group("r") * 2 + 1);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"mov","{8}, {8}",vec!(rd1.to_rv(),rr1.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd1,rr1.to_rv());
                cg.assign(rd2,rr2.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // IN
        [ "10110 A@.. d@..... A@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let io = ioreg(st,"A");
            let name = if let Lvalue::Variable{ name: n,..} = io { n } else { "(noname)".to_string() };
            let off = Rvalue::Constant(st.get_group("d") as u64);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"in",&format!("{{8}}, {{8::{}}}",name),vec!(rd.to_rv(),off.clone()),|cg: &mut CodeGen| {
                cg.assign(rd,sram(&off).to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // OUT
        [ "10111 A@.. r@..... A@...." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let io = ioreg(st,"A");
            let name = if let Lvalue::Variable{ name: n,..} = io { n } else { "(noname)".to_string() };
            let off = Rvalue::Constant(st.get_group("d") as u64);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"in",&format!("{{8::{}}}, {{8}}",name),vec!(off.clone(),rr.to_rv()),|cg: &mut CodeGen| {
                cg.assign(sram(&off),rr.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // POP
        [ "1001000 d@..... 1111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"pop","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let sp = get_sp(cg);
                cg.sub_i(sp.clone(),sp.to_rv(),Rvalue::Constant(1));
                cg.assign(rd,sram(&sp.to_rv()).to_rv());
                set_sp(&sp.to_rv(),cg);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // PUSH
        [ "1001001 d@..... 1111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"push","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let sp = get_sp(cg);
                cg.sub_i(sp.clone(),sp.to_rv(),Rvalue::Constant(1));
                cg.assign(sram(&sp.to_rv()),rd.to_rv());
                set_sp(&sp.to_rv(),cg);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SWAP
        [ "1001010 d@..... 0010" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"swap","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let lower = new_temp(8);
                let higher = new_temp(8);

                cg.div_i(higher.clone(),rd.to_rv(),Rvalue::Constant(128));
                cg.mod_i(lower.clone(),rd.to_rv(),Rvalue::Constant(127));

                let shifted = new_temp(8);
                cg.mul_i(shifted.clone(),lower.to_rv(),Rvalue::Constant(128));

                cg.add_i(rd,shifted.to_rv(),higher.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // XCH
        [ "1001001 r@..... 0100" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"xch","{8}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                let tmp = new_temp(8);
                cg.assign(tmp.clone(),sram(&z.to_rv()).to_rv());
                cg.assign(sram(&z.to_rv()),rr.to_rv());
                cg.assign(rr,tmp.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SER
        [ "11101111 d@.... 1111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"ser","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd,Rvalue::Constant(0xff));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LDI
        [ "1110 K@.... d@.... K@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let K = st.get_group("K");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"ldi",&format!("{{8}}, {{::{}}}",K),vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd,Rvalue::Constant(K as u64));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LAC
        [ "1001001 r@..... 0110" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"lac","{{8}}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                let comp = new_temp(8);
                cg.sub_i(comp,Rvalue::Constant(0xff),sram(&z.to_rv()).to_rv());

                cg.and_i(sram(&z.to_rv()),rr.to_rv(),comp.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LAS
        [ "1001001 r@..... 0101" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"las","{{8}}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                let tmp = new_temp(8);
                cg.assign(tmp,sram(&z.to_rv()).to_rv());

                cg.or_i(sram(&z.to_rv()),rr.to_rv(),tmp);
                cg.assign(rr,tmp.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LAT
        [ "1001001 r@..... 0111" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"lat","{{8}}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                let tmp = new_temp(8);
                cg.assign(tmp,sram(&z.to_rv()).to_rv());

                cg.xor_i(sram(&z.to_rv()),rr.to_rv(),tmp);
                cg.assign(rr,tmp.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LDS
        [ "1001000 d@..... 0000", "k@................" ] = |st: &mut State<u16>| {
            let k = Rvalue::Constant(st.get_group("k") as u64);
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"lds","{{8}}, {{8}}",vec!(rd.to_rv(),k.clone()),|cg: &mut CodeGen| {
                cg.assign(rd,sram(k).to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LDS
        [ "10100 k@... d@.... k@...." ] = |st: &mut State<u16>| {
            let k_ = st.get_group("k");
            let k = Rvalue::Constant((!k_ & 16) | (k_ & 16) | (k_ & 64) | (k_ & 32) | (k_ & 15) as u64);
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"lds","{{8}}, {{8}}",vec!(rd.to_rv(),k.clone()),|cg: &mut CodeGen| {
                cg.assign(rd,sram(k).to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LPM
        [ 0x95c8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"lpm","",vec!(),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                cg.assign(R1.clone(),flash(z.to_rv()).to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SPM
        [ 0x95e8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic_dynargs(2,"spm","{{16::X}}",|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                cg.assign(flash(z.to_rv()),R1.to_rv());
                vec!(z);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SPM
        [ 0x95f8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic_dynargs(2,"spm","{{16::X+}}",|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                cg.assign(flash(z.to_rv()),R1.to_rv());
                cg.add_i(z.clone(),z.to_rv(),Rvalue::Constant(1));

                cg.div_i(R30.clone(),z.to_rv(),Rvalue::Constant(0x100));
                cg.mod_i(R31.clone(),z.to_rv(),Rvalue::Constant(0x100));

                vec!(z);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // STS
        [ "1001001 d@..... 0000", "k@................" ] = |st: &mut State<u16>| {
            let k = Rvalue::Constant(st.get_group("k"));
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic_dynargs(2,"sts","{{16::X+}}",|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                cg.assign(flash(z.to_rv()),R1.to_rv());
                cg.add_i(z.clone(),z.to_rv(),Rvalue::Constant(1));

                cg.div_i(R30.clone(),z.to_rv(),Rvalue::Constant(0x100));
                cg.mod_i(R31.clone(),z.to_rv(),Rvalue::Constant(0x100));

                vec!(z);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // STS
        [ "10101 k@... d@.... k@...." ] = |st: &mut State<u16>| {
            let k_ = st.get_group("k");
            let k = Rvalue::Constant((!k_ & 16) | (k_ & 16) | (k_ & 64) | (k_ & 32) | (k_ & 15) as u64);
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic_dynargs(2,"sts","{{16::X}}, {{8}}",|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                cg.assign(flash(z.to_rv()),R1.to_rv());
                cg.add_i(z.clone(),z.to_rv(),Rvalue::Constant(1));

                vec!(z,rd);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SBI
        [ "10011010 A@..... b@..." ] = |st: &mut State<u16>| {
            let a = Rvalue::Constant(st.get_group("A"));
            let b = Rvalue::Constant(st.get_group("b") as u64);
            let mask = Rvalue::Constant(1 << (st.get_group("b") - 1));
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sbi","{{8}}, {{8}}",vec!(a.clone(),b.clone()),|cg: &mut CodeGen| {
                cg.or_i(sram(a),sram(a).to_rv(),mask);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CBI
        [ "10011010 A@..... b@..." ] = |st: &mut State<u16>| {
            let a = Rvalue::Constant(st.get_group("A"));
            let b = Rvalue::Constant(st.get_group("b"));
            let mask = Rvalue::Constant((!(1 << (st.get_group("b") - 1))) & 0xff);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sbi","{{8}}, {{8}}",vec!(a.clone(),b.clone()),|cg: &mut CodeGen| {
                cg.and_i(sram(a),sram(a).to_rv(),mask);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SEC
        [ 0x9408 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sec","",vec!(),|cg: &mut CodeGen| {
                cg.assign(C.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SEH
        [ 0x9458 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"seh","",vec!(),|cg: &mut CodeGen| {
                cg.assign(H.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SEI
        [ 0x9478 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sei","",vec!(),|cg: &mut CodeGen| {
                cg.assign(I.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SEN
        [ 0x9428 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sen","",vec!(),|cg: &mut CodeGen| {
                cg.assign(N.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SES
        [ 0x9448 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"ses","",vec!(),|cg: &mut CodeGen| {
                cg.assign(S.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SET
        [ 0x9468 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"set","",vec!(),|cg: &mut CodeGen| {
                cg.assign(T.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SEV
        [ 0x9438 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sev","",vec!(),|cg: &mut CodeGen| {
                cg.assign(V.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SEZ
        [ 0x9418 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sez","",vec!(),|cg: &mut CodeGen| {
                cg.assign(Z.clone(),Rvalue::Constant(1));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLC
        [ 0x9488 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLC","",vec!(),|cg: &mut CodeGen| {
                cg.assign(C.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLH
        [ 0x94d8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLH","",vec!(),|cg: &mut CodeGen| {
                cg.assign(H.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLI
        [ 0x94f8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLI","",vec!(),|cg: &mut CodeGen| {
                cg.assign(I.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLN
        [ 0x94a8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLN","",vec!(),|cg: &mut CodeGen| {
                cg.assign(N.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLS
        [ 0x94c8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLS","",vec!(),|cg: &mut CodeGen| {
                cg.assign(S.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLT
        [ 0x94e8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLT","",vec!(),|cg: &mut CodeGen| {
                cg.assign(T.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLV
        [ 0x94b8 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLV","",vec!(),|cg: &mut CodeGen| {
                cg.assign(V.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CLZ
        [ 0x9498 ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"CLZ","",vec!(),|cg: &mut CodeGen| {
                cg.assign(Z.clone(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CP
        [ "000101 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;
            let rr = reg(st,"r");
            let rd = reg(st,"d");

            st.mnemonic(2,"CP","{{8}}, {{8}}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(8);
                cg.sub_i(r.clone(),rd,rr);

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(half_rd.clone(),rd.to_rv(),Rvalue::Constant(0x10));
                cg.mod_i(half_rr.clone(),rr.to_rv(),Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd,half_rr.to_rv());

                cg.less_i(C.clone(),rd,rr.to_rv());
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r);
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CPC
        [ "000001 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;
            let rr = reg(st,"r");
            let rd = reg(st,"d");

            st.mnemonic(2,"CPC","{{8}}, {{8}}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(8);
                let cr = new_temp(8);

                cg.lift(cr.clone(),C.to_rv());
                cg.sub_i(r.clone(),rd.to_rv(),rr.to_rv());
                cg.sub_i(r.clone(),r.to_rv(),cr.to_rv());

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(half_rd.clone(),rd.to_rv(),Rvalue::Constant(0x10));
                cg.mod_i(half_rr.clone(),rr.to_rv(),Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd.to_rv(),half_rr.to_rv());

                cg.less_i(C.clone(),rd.to_rv(),rr.to_rv());
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r.to_rv());
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // CPI
        [ "0011 K@.... d@.... K@...." ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;
            let k = st.get_group("K");
            let rd = reg(st,"d");

            st.mnemonic(2,"CPI","{{8}}, {{8}}",vec!(rd.to_rv(),k),|cg: &mut CodeGen| {
                let r = new_temp(8);
                cg.sub_i(r.clone(),rd.clone(),k);

                let half_k = new_temp(4);
                let half_rd = new_temp(4);

                cg.mod_i(half_k.clone(),k,Rvalue::Constant(0x10));
                cg.mod_i(half_rd.clone(),rd.to_rv(),Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_k.to_rv(),half_rd.to_rv());

                cg.less_i(C.clone(),k.to_rv,rd.to_rv());
                cg.equal_i(Z.clone(),r.to_rv(),Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r.to_rv());
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LSR
        [ "1001010 d@..... 0110" ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;
            let rd = reg(st,"d");

            st.mnemonic(2,"lsr","",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.mod_i(C.clone(),rd.to_rv(),Rvalue::Constant(2));
                cg.rshift_i(rd.clone(),rd.to_rv(),Rvalue::Constant(1));
                cg.xor_b(S.clone(),V.to_rv(),N.to_rv());
                cg.xor_b(V.clone(),N.to_rv(),C.to_rv());
                cg.assign(N.clone(),Rvalue::Constant(0));
                cg.equal_i(Z.clone(),rd.to_rv(),Rvalue::Constant(0));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // ADC
        [ "000111 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let next = st.address + (st.tokens.len() as u64) * 2;
            let rd = reg(st,"d");
            let rr = reg(st,"r");

            st.mnemonic(2,"adc","{{8}}, {{8}}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let cr = new_temp(1);
                let r = new_temp(8);

                cg.lift_b(cr.clone(),C.to_rv());
                cg.add_i(r.clone(),rd.to_rv(),rr.to_rv());
                cg.add_i(r.clone(),r.to_rv(),cr.to_rv());

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(half_rd.clone(),rd.to_rv(),Rvalue::Constant(0x10));
                cg.mod_i(half_rr.clone(),rr.to_rv(),Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd.to_rv(),half_rr.to_rv());

                //cg.less_i(H.clone(),constant(16),(rd % 0x10) + (rr % 0x10));
                /*m.or_b(V.clone(),
                    m.and_b(m.less_i(rr,constant(0x80),m.and_b(m.less_i(rd,constant(0x80)),m.less_i(constant(0x7f),R))),
                    m.and_b(m.less_i(constant(0x7f),rr),m.and_b(m.less_i(constant(0x7f),rd),m.less_i(R,constant(0x80))))));
                m.less_i(N.clone(),R,constant(0x7f));
                m.equal_i(Z.clone(),constant(0),R);
                m.less_i(C.clone(),constant(0x100),R);
                m.or_b(S.clone(),m.and_b(m.not_b(N.to_rv()),V),m.and_b(N,m.not_b(V)));
                m.assign(rd,R % 0x100);*/
            });

            //XXX
            true
        },
        // SUB
        [ "000110 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + 4;

            st.mnemonic(2,"sub","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(8);

                cg.sub_i(r,rd,rr);

                let half_rr = new_temp(8);
                let half_rd = new_temp(8);

                cg.mod_i(half_rd,rd,Rvalue::Constant(0x10));
                cg.mod_i(half_rr,rr,Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd,half_rr);

                cg.less_i(C.clone(),rd,rr);
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r);
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.assign(rd,r);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SUBI
        [ "0101 K@.... d@.... K@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let k = st.get_group("K");
            let next = st.address + 4;

            st.mnemonic(2,"subi","{8}, {8}",vec!(rd.to_rv(),k.clone()),|cg: &mut CodeGen| {
                let r = new_temp(8);
                let cr = cg.lift_b(C.to_rv());

                cg.sub_i(r,rd,k);
                cg.sub_i(r,r,cr);

                let half_rd = new_temp(8);

                cg.mod_i(half_rd,rd,Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd,Rvalue::Constant(k % 0x10));

                cg.less_i(C.clone(),rd,k);
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r);
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.assign(rd,r);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // ASR
        [ "1001010 d@..... 0101" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"asr","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let low = new_temp(8);

                cg.mod_i(low,rd,Rvalue::Constant(2));
                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.equal_i(C.clone(),low,Rvalue::Constant(0));

                let r = new_temp(8);
                cg.div_i(r,rd,Rvalue::Constant(2));

                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r);
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.assign(rd,r);
                cg.xor_b(V.clone(),N.to_rv(),C);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // ROL
        [ "000111 d@.........." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"rol","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let c = new_temp(1);

                cg.div_i(c,rd,Rvalue::Constant(0x80));
                cg.mul_i(rd,rd,Rvalue::Constant(2));
                cg.add_i(rd,rd,C.to_rv());
                cg.assign_i(C.clone(),c);

                let half_rd = new_temp(8);
                cg.div_i(half_rd,rd,Rvalue::Constant(0x10));
                cg.mod_i(half_rd,half_rd,Rvalue::Constant(2));
                cg.equal_i(H.clone(),half_rd,Rvalue::Constant(1));
                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),rd);
                cg.equal_i(Z.clone(),rd,Rvalue::Constant(0));
                cg.xor_b(V.clone(),N.to_rv(),C);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // ROR
        [ "1001010 d@..... 0111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"ror","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let c = new_temp(1);

                cg.mod_i(c,rd,Rvalue::Constant(2));
                cg.div_i(rd,rd,Rvalue::Constant(2));

                let t = new_temp(8);
                cg.mul_i(t,C.to_rv(),Rvalue::Constant(0x80));
                cg.add_i(rd,rd,t);
                cg.assign_i(C.clone(),c);

                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),rd);
                cg.equal_i(Z.clone(),rd,Rvalue::Constant(0));
                cg.xor_b(V.clone(),N.to_rv(),C);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // DEC
        [ "1001010 d@..... 1010" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"dec","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.sub_i(rd,rd,Rvalue::Constant(1));
                cg.equal_i(C.clone(),rd,Rvalue::Constant(0xff));

                cg.less_i(N.clone(),Rvalue::Constant(0x7f),rd);
                cg.equal_i(Z.clone(),rd,Rvalue::Constant(0));
                cg.equal_i(V.clone(),rd,Rvalue::Constant(0x80));
                cg.xor_b(S.clone(),N.to_rv(),V);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // INC
        [ "1001010 d@..... 0011" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"inc","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.add_i(rd,rd,Rvalue::Constant(1));
                cg.equal_i(C.clone(),rd,Rvalue::Constant(0));

                cg.less_i(N.clone(),Rvalue::Constant(0x7f),rd);
                cg.equal_i(Z.clone(),rd,Rvalue::Constant(0));
                cg.equal_i(V.clone(),rd,Rvalue::Constant(0x80));
                cg.xor_b(S.clone(),N.to_rv(),V);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SBC
        [ "000010 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sbc","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(8);
                let cr = cg.lift_b(C.to_rv());

                cg.sub_i(r,rd,rr);
                cg.sub_i(r,r,cr);

                let half_rd = new_temp(8);
                let half_rr = new_temp(8);

                cg.mod_i(half_rd,rd,Rvalue::Constant(0x10));
                cg.mod_i(half_rr,rr,Rvalue::Constant(0x10));

                cg.less_i(C.clone(),rd,rr);
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r);
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.assign(rd,r);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SBCI
        [ "0100 K@.... d@.... K@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let k = st.get_group("K");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sbci","{8}, {8}",vec!(rd.to_rv(),k.clone()),|cg: &mut CodeGen| {
                let r = new_temp(8);
                let cr = cg.lift_b(C.to_rv());

                cg.sub_i(r,rd,k);
                cg.sub_i(r,r,cr);

                let half_rd = new_temp(8);

                cg.mod_i(half_rd,rd,Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd,Rvalue::Constant(k % 0x10));

                cg.less_i(C.clone(),rd,k);
                cg.equal_i(Z.clone(),r,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r);
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V);
                cg.assign(rd,r);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // COM
        [ "1001010 d@..... 0000" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"com","{8}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.sub_i(rd,Rvalue::Constant(0xff),rd);

                cg.assign(C.clone(),Rvalue::Constant(0));
                cg.equal_i(Z.clone(),rd,Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),rd);
                cg.assign(V.clone(),Rvalue::Constant(0));
                cg.xor_b(S.clone(),N.to_rv(),V);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // ADIW
        [ "10010110 K@.. d@.. K@...." ] = |st: &mut State<u16>| {
            let d = st.get_group("d") * 2 + 24;
            let k = st.get_group("K");
            let rd1 = resolv(d);
            let rd2 = resolv(d + 1);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"adiw","{8}:{8}, {8}",vec!(rd1.to_rv(),rd2.to_rv(),k.clone()),|cg: &mut CodeGen| {
                let r = new_temp(16);

                cg.assign(r,rd2);
                cg.mul_i(r,r,Rvalue::Constant(0x100));
                cg.add_i(r,r,rd1);
                cg.add_i(r,r,k);

                let v1 = new_temp(1);
                let v2 = new_temp(1);
                cg.less_i(v1,rd2,Rvalue::Constant(0x80));
                cg.less_i(v2,r,Rvalue::Constant(0x8000));
                cg.not_b(v1,v1);
                cg.and_b(V.clone(),v1,v2);

                cg.less_i(N.clone(),r,Rvalue::Constant(0x8000));
                cg.equal_i(Z.clone(),Rvalue::Constant(0),r);

                let c1 = new_temp(1);
                let c2 = new_temp(1);
                cg.less_i(c1,rd2,Rvalue::Constant(0x80));
                cg.less_i(c2,r,Rvalue::Constant(0x8000));
                cg.not_b(c2,c2);
                cg.and_b(C.clone(),c1,c2);

                cg.xor_b(S.clone(),N.to_rv(),V);

                cg.div_i(rd2,r,Rvalue::Constant(0x100));
                cg.mod_i(rd1,r,Rvalue::Constant(0x100));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SBIW
        [ "10010111 K@.. d@.. K@...." ] = |st: &mut State<u16>| {
            let d = st.get_group("d") * 2 + 24;
            let k = st.get_group("K");
            let rd1 = resolv(d);
            let rd2 = resolv(d + 1);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"sbiw","{8}:{8}, {8}",vec!(rd1.to_rv(),rd2.to_rv(),k.clone()),|cg: &mut CodeGen| {
                let r = new_temp(16);

                cg.assign(r,rd2);
                cg.mul_i(r,r,Rvalue::Constant(0x100));
                cg.add_i(r,r,rd1);
                cg.sub_i(r,r,k);

                let v1 = new_temp(1);
                let v2 = new_temp(1);
                cg.less_i(v1,rd2,Rvalue::Constant(0x80));
                cg.less_i(v2,r,Rvalue::Constant(0x8000));
                cg.not_b(v2,v2);
                cg.and_b(V.clone(),v1,v2);

                cg.less_i(N.clone(),r,Rvalue::Constant(0x8000));
                cg.equal_i(Z.clone(),Rvalue::Constant(0),r);

                let c1 = new_temp(1);
                let c2 = new_temp(1);
                cg.less_i(c1,rd2,Rvalue::Constant(0x80));
                cg.less_i(c2,r,Rvalue::Constant(0x8000));
                cg.not_b(c1,c1);
                cg.and_b(C.clone(),c1,c2);

                cg.xor_b(S.clone(),N.to_rv(),V);

                cg.div_i(rd2,r,Rvalue::Constant(0x100));
                cg.mod_i(rd1,r,Rvalue::Constant(0x100));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // MULS
        [ "0000 0010 d@.... r@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"muls","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(16);

                cg.mul_i(r,rd,rr);
                cg.less_i(C.clone(),r,Rvalue::Constant(0x8000));
                cg.equal_i(Z.clone(),Rvalue::Constant(0),r);

                cg.div_i(R1,r,Rvalue::Constant(0x100));
                cg.mod_i(R0,r,Rvalue::Constant(0x100));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // MULSU
        [ "0000 0011 0 d@... 0 r@..." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"mulsu","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(16);

                cg.mul_i(r,rd,rr);
                cg.less_i(C.clone(),r,Rvalue::Constant(0x8000));
                cg.equal_i(Z.clone(),Rvalue::Constant(0),r);

                cg.div_i(R1,r,Rvalue::Constant(0x100));
                cg.mod_i(R0,r,Rvalue::Constant(0x100));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // MUL
        [ "1001 11 r@. d@.... r@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"mul","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(16);

                cg.mul_i(r,rd,rr);
                cg.less_i(C.clone(),r,Rvalue::Constant(0x8000));
                cg.equal_i(Z.clone(),Rvalue::Constant(0),r);

                cg.div_i(R1,r,Rvalue::Constant(0x100));
                cg.mod_i(R0,r,Rvalue::Constant(0x100));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // BRCx
        [ "11110 x@. k@....... 000" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(C.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brcs","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brcc","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BREQ/BRNE
        [ "11110 x@. k@....... 001" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(Z.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"breq","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brne","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BRNx
        [ "11110 x@. k@....... 010" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(N.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brns","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brnc","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BRVx
        [ "11110 x@. k@....... 011" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(V.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brvs","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brvc","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BRGE/BTLT
        [ "11110 x@. k@....... 100" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(S.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brlt","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brge","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BRHx
        [ "11110 x@. k@....... 101" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(H.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brhs","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brhc","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BRTx
        [ "11110 x@. k@....... 110" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(T.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brts","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brtc","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // BRIx
        [ "11110 x@. k@....... 111" ] = |st: &mut State<u16>| {
            let d = st.get_group("k");
            let fallthru = st.address + (st.tokens.len() as u64) * 2;
            let g = Guard::from_relation(Relation::Equal(I.to_rc(),Rvalue::Constant(0)));

            if st.get_group("x") == 0 {
                st.mnemonic(2,"brie","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g);
                st.jump(Rvalue::Constant(d),g.negation());
            } else {
                st.mnemonic(2,"brid","{8}",vec!(Rvalue::Constant(d)),|cg: &mut CodeGen| {});
                st.jump(Rvalue::Constant(fallthru),g.negation());
                st.jump(Rvalue::Constant(d),g);
            }
            true
        },
        // CALL
        [ "1001010 k@..... 111 k@.", "k@................" ] = |st: &mut State<u16>| {
            let _k = (st.get_group("k") * 2);// % st.state.flash_sz;
            let k = Rvalue::Constant(_k);

            st.mnemonic(4,"call","{26}",vec!(k),|cg: &mut CodeGen| {
                cg.call_i(Lvalue::Undefined(),k);
            });
            true
        },
        // JMP
        [ "1001010 k@..... 110 k@.", "k@................" ] = |st: &mut State<u16>| {
            let _k = (st.get_group("k") * 2);// % st.state.flash_sz;
            let k = Rvalue::Constant(_k);

            st.mnemonic(4,"jmp","{26}",vec!(k.clone()),|cg: &mut CodeGen| {});
            st.jump(k,Guard::new());
            true
        },
        // RCALL
        [ "1101 k@............" ] = |st: &mut State<u16>| {
            let _k = st.get_group("k");
            let k = if _k <= 2047 {
                Rvalue::Constant((_k * 2 + 2 + st.address))// % st.state.flash_sz);
            } else {
                Rvalue::Constant(((_k - 4096) * 2 + 2 + st.address))// % st.state.flash_sz);
            };

            st.mnemonic(2,"call","{26}",vec!(Rvalue::Constant(k)),|cg: &mut CodeGen| {
                cg.call_i(Lvalue::Undefined(),Rvalue::Constant(k));
            });
            true
        },
        // RJMP
        [ "1100 k@............" ] = |st: &mut State<u16>| {
            let _k = (st.get_group("k") * 2);// % st.state.flash_sz;
            let k = if _k <= 2047 {
                Rvalue::Constant((_k * 2 + 2 + st.address))// % st.state.flash_sz);
            } else {
                Rvalue::Constant(((_k - 4096) * 2 + 2 + st.address))// % st.state.flash_sz);
            };

            st.mnemonic(2,"jmp","{26}",vec!(k.clone()),|cg: &mut CodeGen| {});
            st.jump(k,Guard::new());
            true
        },
        // RET
        [ 0x9508 ] = |st: &mut State<u16>| {
            let next = st.address + 2;
            st.mnemonic(2,"ret","",vec!(),|cg: &mut CodeGen| {});
            st.jump(next,Guard::new());
            true
        },
        // RETI
        [ 0x9518 ] = |st: &mut State<u16>| {
            let next = st.address + 2;
            st.mnemonic(2,"reti","",vec!(),|cg: &mut CodeGen| {});
            st.jump(next,Guard::new());
            true
        },
        // IJMP
        [ 0x9409 ] = |st: &mut State<u16>| {
            let z = new_temp(16);
            let next = st.address + 2;
            st.mnemonic_dynargs(2,"ijmp","{16::Z}",|cg: &mut CodeGen| {
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());
                vec!(z.to_rv())
            });
            st.jump(z,Guard::new());
            true
        },
        // ICALL
        [ 0x9509 ] = |st: &mut State<u16>| {
            let next = st.address + 2;
            let z = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"icall","{16::Z}",|cg: &mut CodeGen| {
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                cg.call_i(Lvalue::Undefined(),z.to_rv());
                vec!(z.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST X
        [ "1001 001 r@. r@.... 1100" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::X}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R26.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R27.to_rv(),x.to_rv());
                cg.assign(sram(x.to_rv()),rr.to_rv());
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST -X
        [ "1001 001 r@. r@.... 1110" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::-X}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R26.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R27.to_rv(),x.to_rv());
                cg.sub_i(x,x.to_rv(),Rvalue::Constant(1));
                cg.assign(sram(x.to_rv()),rr.to_rv());
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST X+
        [ "1001 001 r@. r@.... 1101" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::X+}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R26.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R27.to_rv(),x.to_rv());
                cg.assign(sram(x.to_rv()),rr.to_rv());
                cg.add_i(x,x.to_rv(),Rvalue::Constant(1));
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST Y
        [ "1001 001 r@. r@.... 1000" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::Y}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R28.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R29.to_rv(),x.to_rv());
                    cg.sub_i(x,x.to_rv(),Rvalue::Constant(1));
                cg.assign(sram(x.to_rv()),rr.to_rv());
                    cg.add_i(x,x.to_rv(),Rvalue::Constant(1));
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST -Y
        [ "1001 001 r@. r@.... 1010" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::-Y}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R28.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R29.to_rv(),x.to_rv());
                cg.sub_i(x,x.to_rv(),Rvalue::Constant(1));
                cg.assign(sram(x.to_rv()),rr.to_rv());
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST Y+
        [ "1001 001 r@. r@.... 1001" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::Y+}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R28.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R29.to_rv(),x.to_rv());
                cg.assign(sram(x.to_rv()),rr.to_rv());
                cg.add_i(x,x.to_rv(),Rvalue::Constant(1));
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST Z
        [ "1001 001 r@. r@.... 0000" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::Z}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R31.to_rv(),x.to_rv());
                cg.assign(sram(x.to_rv()),rr).to_rv();
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST -Z
        [ "1001 001 r@. r@.... 0010" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::-Z}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R31.to_rv(),x.to_rv());
                cg.sub_i(x,x.to_rv(),Rvalue::Constant(1));
                cg.assign(sram(x.to_rv()),rr.to_rv());
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST Z+
        [ "1001 001 r@. r@.... 0001" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st","{16::Z+}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R31.to_rv(),x.to_rv());
                cg.assign(sram(x.to_rv()),rr.to_rv());
                cg.add_i(x,x.to_rv(),Rvalue::Constant(1));
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // ST *+q
        [ "10q@.0 q@..1r@. r@.... x@. q@..." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let reg = st.get_group("x") == 1;
            let q = st.get_group("q");
            let x = new_temp(16);
            let t = if reg { "Y" } else { "Z" };
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"st",format!("{{16::{}+{}}}",t,q),|cg: &mut CodeGen| {
                if reg {
                    cg.mul_i(x.clone(),R28.to_rv(),Rvalue::Constant(0x100));
                    cg.add_i(x.clone(),R29.to_rv(),x.to_rv());
                } else {
                    cg.mul_i(x.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                    cg.add_i(x.clone(),R31.to_rv(),x.to_rv());
                }

                cg.assign(sram(x.to_rv()),rr.to_rv());
                cg.add_i(x.clone(),x.to_rv(),Rvalue::Constant(q));
                vec!(x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // LD X
        [ "1001 000 d@..... 1100" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"ld","{8}, {16::X}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R26.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R27.to_rv(),x.to_rv());
                cg.assign(rd.clone(),sram(x.to_rv()).to_rv());
                vec!(rd.to_rv(),x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // LD -X
        [ "1001 000 d@..... 1110" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"ld","{8}, {16::-X}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R26.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R27.to_rv(),x.to_rv());
                cg.sub_i(x.clone(),x.to_rv(),Rvalue::Constant(1));
                cg.assign(rd.clone(),sram(x.to_rv()).to_rv());
                vec!(rd.to_rv(),x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
        // LD X+
        [ "1001 000 d@..... 1101" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.address + 2;

            st.mnemonic_dynargs(2,"ld","{8}, {16::X+}",|cg: &mut CodeGen| {
                cg.mul_i(x.clone(),R26.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(x.clone(),R27.to_rv(),x.to_rv());
                cg.assign(rd.clone(),sram(x.to_rv()).to_rv());
                cg.add_i(x.clone(),x.to_rv(),Rvalue::Constant(1));
                vec!(rd.to_rv(),x.to_rv())
            });
            st.jump(next,Guard::new());
            true
        },
/*
	main[e("1000 000d@. d@.... 1000")] = binary_ld(r28,r29,false,false);
	main[e("1001 000d@. d@.... 1001")] = binary_ld(r28,r29,false,true);
	main[e("1001 000d@. d@.... 1010")] = binary_ld(r28,r29,true,false);
	main[e("10 q@. 0 q@.. 0 d@..... 1 q@...")] = binary_ldq(r28,r29);

	main[e("1000 000d@. d@.... 0000")] = binary_ld(r30,r31,false,false);
	main[e("1001 000 d@..... 0001")] = binary_ld(r30,r31,false,true);
	main[e("1001 000d@. d@.... 0010")] = binary_ld(r30,r31,true,false);
	main[e("10q@.0 q@..0d@. d@.... 0q@...")] = binary_ldq(r30,r31);*/

        // BREAK
        [ 0x9598 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"break","",vec!(),|cg: &mut CodeGen| {});
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // DES
        [ "10010100 K@.... 1011" ] = |st: &mut State<u16>| {
            let next = st.address + 1;
            let k = st.get_group(st,"K");

            st.mnemonic(2,"des","{{4}}",vec!(k.clone()),|cg: &mut CodeGen| {
                cg.assign(R0.clone(),Rvalue::Undefined);
                cg.assign(R1.clone(),Rvalue::Undefined);
                cg.assign(R2.clone(),Rvalue::Undefined);
                cg.assign(R3.clone(),Rvalue::Undefined);
                cg.assign(R4.clone(),Rvalue::Undefined);
                cg.assign(R5.clone(),Rvalue::Undefined);
                cg.assign(R6.clone(),Rvalue::Undefined);
                cg.assign(R7.clone(),Rvalue::Undefined);
                cg.assign(R8.clone(),Rvalue::Undefined);
                cg.assign(R9.clone(),Rvalue::Undefined);
                cg.assign(R10.clone(),Rvalue::Undefined);
                cg.assign(R11.clone(),Rvalue::Undefined);
                cg.assign(R12.clone(),Rvalue::Undefined);
                cg.assign(R13.clone(),Rvalue::Undefined);
                cg.assign(R14.clone(),Rvalue::Undefined);
                cg.assign(R15.clone(),Rvalue::Undefined);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // NOP
        [ 0x0 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"nop","",vec!(),|cg: &mut CodeGen| {});
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SLEEP
        [ 0x9588 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"sleep","",vec!(),|cg: &mut CodeGen| {});
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // WDR
        [ 0x95a8 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"wdr","",vec!(),|cg: &mut CodeGen| {});
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // catch all
        _ = |st: &mut State<u16>| {
            let next = st.address + 1;
            let rd = reg(st,"d");

            st.mnemonic(1,"unk","",vec!(),|cg: &mut CodeGen| {});
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        }
    );

    let main = new_disassembler!(u16 =>
        // SBRC
        [ "1111 110 r@..... 0 b@...", simple ] = |st: &mut State<u16>| {
            let _b = st.get_group("b") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let rr = reg(st,"r");
            let fallthru = st.address + 2;
            let skip = st.address + (st.tokens.len() as u64) * 2;
            let r = new_temp(8);

            st.mnemonic(2,"sbrc","{8}, {3}",vec!(rr.to_rv(),b.clone()),|cg: &mut CodeGen| {
                cg.div_i(r.clone(),rr.to_rv(),mask);
                cg.mod_i(r.clone(),r.to_rv(),Rvalue::Constant(2));
            });

            let g = Guard::from_relation(Relation::Equal(r.to_rv(),Rvalue::Constant(0)));
            st.jump(Rvalue::Constant(fallthru),g.negation());
            st.jump(Rvalue::Constant(skip),g);
            true
        },
        // SBRS
        [ "1111 111 r@..... 0 b@...", simple ] = |st: &mut State<u16>| {
            let _b = st.get_group("b") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let rr = reg(st,"r");
            let fallthru = st.address + 2;
            let skip = st.address + (st.tokens.len() as u64) * 2;
            let r = new_temp(8);

            st.mnemonic(2,"sbrs","{8}, {3}",vec!(rr.to_rv(),b.clone()),|cg: &mut CodeGen| {
                cg.div_i(r.clone(),rr.to_rv(),mask);
                cg.mod_i(r.clone(),r.to_rv(),Rvalue::Constant(2));
            });

            let g = Guard::from_relation(Relation::Equal(r.to_rv(),Rvalue::Constant(0)));
            st.jump(Rvalue::Constant(skip),g.negation());
            st.jump(Rvalue::Constant(fallthru),g);
            true
        },
        // CPSE
        [ "000100 r@. d@..... r@....", simple ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let rd = reg(st,"d");
            let fallthru = st.address + 2;
            let skip = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"cpse","{8}, {8}",vec!(rr.to_rv(),rd.to_rv()),|cg: &mut CodeGen| {
                let r = new_temp(8);
                cg.sub_i(r.clone(),rr.to_rv(),rd.to_rv());

                let half_rr = new_temp(8);
                let half_rd = new_temp(8);

                cg.mod_i(half_rd.clone(),rd.to_rv(),Rvalue::Constant(0x10));
                cg.mod_i(half_rr.clone(),rr.to_rv(),Rvalue::Constant(0x10));
                cg.less_i(H.clone(),half_rd.to_rv(),half_rr.to_rv());

                cg.less_i(C.clone(),rd.to_rv(),rr.to_rv());
                cg.equal_i(Z.clone(),r.to_rv(),Rvalue::Constant(0));
                cg.less_i(N.clone(),Rvalue::Constant(0x7f),r.to_rv());
                cg.not_b(V.clone(),C.to_rv());
                cg.xor_b(S.clone(),N.to_rv(),V.to_rv());
            });

            let g = Guard::from_relation(Relation::Equal(Z.to_rv(),Rvalue::Constant(0)));
            st.jump(Rvalue::Constant(fallthru),g.negation());
            st.jump(Rvalue::Constant(skip),g);
            true
        },
        // SBIC
        [ "1001 1001 A@..... b@...", simple ] = |st: &mut State<u16>| {
            let _b = st.get_group("b") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let a = ioreg(st,"a");
            let fallthru = st.address + 2;
            let skip = st.address + (st.tokens.len() as u64) * 2;
            let r = new_temp(8);

            st.mnemonic(2,"sbic","{8}, {3}",vec!(a.to_rv(),b.clone()),|cg: &mut CodeGen| {
                cg.div_i(r,a.to_rv(),mask);
                cg.mod_i(r,r.to_rv(),Rvalue::Constant(2));
            });

            let g = Guard::from_relation(Relation::Equal(r.to_rv(),Rvalue::Constant(0)));
            st.jump(Rvalue::Constant(fallthru),g.negation());
            st.jump(Rvalue::Constant(skip),g);
            true
        },
        // SBIS
        [ "1001 1011 A@..... b@...", simple ] = |st: &mut State<u16>| {
            let _b = st.get_group("b") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let a = ioreg(st,"a");
            let fallthru = st.address + 2;
            let skip = st.address + (st.tokens.len() as u64) * 2;
            let r = new_temp(8);

            st.mnemonic(2,"sbis","{8}, {3}",vec!(a.to_rv(),b.clone()),|cg: &mut CodeGen| {
                cg.div_i(r,a.to_rv(),mask);
                cg.mod_i(r,r.to_rv(),Rvalue::Constant(2));
            });

            let g = Guard::from_relation(Relation::Equal(r.to_rv(),Rvalue::Constant(0)));
            st.jump(Rvalue::Constant(fallthru),g);
            st.jump(Rvalue::Constant(skip),g.negation());
            true
        }
    );

    Program::disassemble(None,main,State::<u16>::new(0),data,0)
}
