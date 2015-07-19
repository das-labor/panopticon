use disassembler::*;
use program::Program;
use layer::LayerIter;
use value::{Lvalue,Rvalue,Endianess};
use codegen::CodeGen;
use guard::Guard;
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

fn sram(off: &Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.clone()),
        name: "sram".to_string(),
        endianess: Endianess::Big,
        bytes: 1
    }
}

fn flash(off: &Rvalue) -> Lvalue {
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
    let main = new_disassembler!(u16 =>
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

            st.mnemonic(2,"swap","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
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

            st.mnemonic(2,"xch","{{8}}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
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

            st.mnemonic(2,"ser","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
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
                cg.xor_i(S.clone(),N.to_rv(),V.to_rv());
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
                cg.xor_i(S.clone(),V.to_rv(),N.to_rv());
                cg.xor_i(V.clone(),N.to_rv(),C.to_rv());
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

                //cg.less_i(H,constant(16),(rd % 0x10) + (rr % 0x10));
                /*m.or_b(V,
                    m.and_b(m.less_i(rr,constant(0x80),m.and_b(m.less_i(rd,constant(0x80)),m.less_i(constant(0x7f),R))),
                    m.and_b(m.less_i(constant(0x7f),rr),m.and_b(m.less_i(constant(0x7f),rd),m.less_i(R,constant(0x80))))));
                m.less_i(N,R,constant(0x7f));
                m.equal_i(Z,constant(0),R);
                m.less_i(C,constant(0x100),R);
                m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
                m.assign(rd,R % 0x100);*/
            });

            //XXX
            true
        },

        /*
        // byte-level arithmetic and logic
        [ "000111 r@. d@..... r@...." ] = binary_reg("adc",[](cg &m, const variable &rd, const variable &rr)
        {
            rvalue Cr = m.lift_b(C);
            rvalue R = rd + rr + Cr;

            m.less_i(H,constant(16),(rd % 0x10) + (rr % 0x10));
            m.or_b(V,
                m.and_b(m.less_i(rr,constant(0x80),m.and_b(m.less_i(rd,constant(0x80)),m.less_i(constant(0x7f),R))),
                m.and_b(m.less_i(constant(0x7f),rr),m.and_b(m.less_i(constant(0x7f),rd),m.less_i(R,constant(0x80))))));
            m.less_i(N,R,constant(0x7f));
            m.equal_i(Z,constant(0),R);
            m.less_i(C,constant(0x100),R);
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
            m.assign(rd,R % 0x100);
        });
        [ "000011 r@. d@..... r@...." ] = binary_reg("add",[](cg &m, const variable &rd, const variable &rr)
        {
            rvalue R = rd + rr;

            m.less_i(H,constant(16),(rd % 0x10) + (rr % 0x10));
            m.or_b(V,
                m.and_b(m.less_i(rr,constant(0x80),m.and_b(m.less_i(rd,constant(0x80)),m.less_i(constant(0x7f),R))),
                m.and_b(m.less_i(constant(0x7f),rr),m.and_b(m.less_i(constant(0x7f),rd),m.less_i(R,constant(0x80))))));
            m.less_i(N,R,constant(0x7f));
            m.equal_i(Z,constant(0),R);
            m.less_i(C,constant(0x100),R);
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
            m.assign(rd,R % 0x100);
        });
        [ "001000 r@. d@..... r@...." ] = binary_reg("and",[](cg &m, const variable &rd, const variable &rr)
        {
            m.and_i(rd,rd & rr);

            m.assign(V,constant(0));										// V: 0
            m.less_i(N,rd,constant(0x7f));
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
            m.equal_i(Z,constant(0),rd);
        });
        [ "0111 K@.... d@.... K@...." ] = binary_regconst("andi",[&](cg &m, const variable &rd, const constant &K)
        {
            m.and_i(rd,rd & K);

            m.assign(V,constant(0));										// V: 0
            m.less_i(N,rd,constant(0x7f));
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
            m.equal_i(Z,constant(0),rd);
        });

        [ "001001 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            variable rd = decode_reg(st.capture_groups["d"]);
            variable rr = decode_reg(st.capture_groups["r"]);

            if(rd == rr)
            {
                st.mnemonic(st.tokens.size() * 2,"clr","",rd,[&](cg &m)
                {
                    m.assign(rd,constant(0));
                    m.assign(V,constant(0));
                    m.assign(N,constant(0));
                    m.assign(S,constant(0));
                    m.assign(Z,constant(0));
                });
                st.jump(st.address + st.tokens.size() * 2);
            }
            else
            {
                st.mnemonic(st.tokens.size() * 2,"eor","",rd,rr,[&](cg &m)
                {
                    m.xor_i(rd,rd,rr);
                    m.assign(V,constant(0));										// V: 0
                    m.less_i(N,rd,constant(0x7f));
                    m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
                    m.equal_i(Z,constant(0),rd);
                });
                st.jump(st.address + st.tokens.size() * 2);
            }
            return true;
        });
        [ "1001010 d@..... 0001" ] = unary_reg("neg",[](cg &m, const variable &rd)
        {
            //TODO: m.assign(rd,rd ^ 0xff);
        });

        [ "001010 r@. d@..... r@...." ] = binary_reg("or",[](cg &m, const variable &rd, const variable &rr)
        {
            // TODO
        });
        [ "0110 K@.... d@.... K@...." ] = binary_regconst("ori",[&](cg &m, const variable &rd, const constant &K)
        {
            //m.or_b(rd,rd,K);
        });

        [ "000110 r@. d@..... r@...." ] = binary_reg("sub",[&](cg &m, const variable &rd, const variable &rr)
        {
            rvalue R = (rd - rr) % 0x100;

            m.less_i(H,rd % 0x10, rr % 0x10);
            m.less_i(C,rd, rr);
            m.equal_i(Z,R,constant(0));
            m.less_i(N,constant(0x7f), R);
            m.not_b(V,C);
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
            m.assign(rd,R);
        });
        [ "0101 K@.... d@.... K@...." ] = binary_regconst("subi",[&](cg &m, const variable &rd, const constant &K)
        {
            rvalue Cr = m.lift_b(C);
            rvalue R = rd - K - Cr;

            m.less_i(H,rd % 0x10, K % 0x10);
            m.less_i(C,rd, K);
            m.and_b(Z,Z,m.equal_i(R,constant(0)));
            m.less_i(N,constant(0x7f), R);
            m.not_b(V,C);
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
            m.assign(rd,R);
        });

        [ "1001010 d@..... 0101" ] = unary_reg("asr");
        [ "000111 d@.........." ] = unary_reg("rol");
        [ "1001010 d@..... 0111" ] = unary_reg("ror");
        [ "1001010 d@..... 1010" ] = unary_reg("dec");
        [ "1001010 d@..... 0011" ] = unary_reg("inc");
        [ "000010 r@. d@..... r@...." ] = binary_reg("sbc",[](cg &m, const variable &rd, const variable &rr)
        {
            rvalue Cr = m.lift_b(C);
            rvalue R = rd - rr - Cr;

            m.less_i(H,rd % 0x10, rr % 0x10);
            m.less_i(C,rd,rr);
            m.and_b(Z,Z,m.equal_i(R,constant(0)));
            m.less_i(N,constant(0x7f),R);
            m.not_b(V,C);
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));

            m.assign(rd,R % 0x100);
        });

        [ "0100 K@.... d@.... K@...." ] = binary_regconst("sbci",[&](cg &m, const variable &rd, const constant &K)
        {
            rvalue Cr = m.lift_b(C);
            rvalue R = rd - K - Cr;

            m.less_i(H,rd % 0x10, K % 0x10);
            m.less_i(C,rd,K);
            m.and_b(Z,Z,m.equal_i(R,constant(0)));
            m.less_i(N,constant(0x7f), R);
            m.not_b(V,C);
            m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));

            m.assign(rd,R % 0x100);
        });

        [ "1001010 d@..... 0000" ] = unary_reg("com");

        // word-level arithmetic and logic
        [ "10010110 K@.. d@.. K@...." ] = |st: &mut State<u16>| {
        {
            constant K = constant((unsigned int)st.capture_groups["K"]);
            unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
            variable rd1 = decode_reg(d);
            variable rd2 = decode_reg(d+1);

            st.mnemonic(st.tokens.size() * 2,"adiw","{8}:{8}, {16}",{rd2,rd1,K},[&](cg &c)
            {
                rvalue R = rd2 * 0x100 + rd1 + K;

                // V: !rdh7•R15
                c.and_b(V,c.less_i(rd2,constant(0x80)),c.not_b(c.less_i(R,constant(0x8000))));

                // N: R15
                c.less_i(N,R,constant(0x8000));

                // Z: !R15•!R14•!R13•!R12•!R11•!R10•!R9•!R8•!R7•R6•!R5•!R4•!R3•!R2•!R1•!R0
                c.equal_i(Z,constant(0),R);

                // C: !R15•rdh7
                c.and_b(V,c.not_b(c.less_i(rd2,constant(0x80))),c.less_i(R,constant(0x8000)));

                // S: N ⊕ V
                c.or_b(S,c.and_b(c.not_b(N),V),c.and_b(N,c.not_b(V)));

                c.assign(rd2,R / 0x100);
                c.assign(rd1,R % 0x100);
            });
            st.jump(st.address + st.tokens.size() * 2);
            return true;
        });
        [ "10010111 K@.. d@.. K@...." ] = |st: &mut State<u16>| {
        {
            unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
            constant K = constant((unsigned int)st.capture_groups["K"]);
            variable rd1 = decode_reg(d);
            variable rd2 = decode_reg(d+1);

            st.mnemonic(st.tokens.size() * 2,"sbiw","{8}:{8}, {16}",{rd1,rd2,K});
            st.jump(st.address + st.tokens.size() * 2);
            return true;
        });
        [ "0000 0011 0 d@... 1 r@..." ] = binary_reg("fmul",[](cg &m, const variable &rd, const variable &rr)
        {
            // TODO
        });
        [ "000000111 d@... 0 r@..." ] = binary_reg("fmuls",[](cg &m, const variable &rd, const variable &rr)
        {
            // TODO
        });
        [ "000000111 d@... 1 r@..." ] = binary_reg("fmulsu",[](cg &m, const variable &rd, const variable &rr)
        {
            // TODO
        });
        [ "100111 r@. d@..... r@...." ] = binary_reg("mul",[](cg &m, const variable &rd, const variable &rr)
        {
            // TODO
        });
        [ "00000010 d@.... r@...." ] = binary_reg("muls",[](cg &m, const variable &rd, const variable &rr)
        {
            // TODO
        });*/
        // MULS
        [ "000000110 d@... 0 r@..." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let rd = reg(st,"d");
            let cur = st.address;

            st.mnemonic(2,"muls","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                let res = new_temp(16);

                cg.mul_i(res.clone(),rr.to_rv(),rd.to_rv());
                cg.div_i(R1.clone(),res.to_rv(),Rvalue::Constant(0x100));
                cg.mod_i(R0.clone(),res.to_rv(),Rvalue::Constant(0x100));
                cg.less_i(C.clone(),res.to_rv(),Rvalue::Constant(16383));

            });

            st.jump(Rvalue::Constant(cur + 2),Guard::new());
            true
        },
        /*
        // branches
        // [ "111101 k@....... s@..." ] = simple("brbc");
        // [ "111100 k@....... s@..." ] = [](sm &st)  { st.mnemonic(st.tokens.size() * 2,"brbs"; });
        [ "111101 k@....... 000" ] = branch("brcc",C,false);
        [ "111100 k@....... 000" ] = branch("brcs",C,true);
        [ "111100 k@....... 001" ] = branch("breq",Z,true);
        [ "111101 k@....... 100" ] = branch("brge",S,false);
        [ "111101 k@....... 101" ] = branch("brhc",H,false);
        [ "111100 k@....... 101" ] = branch("brhs",H,true);
        [ "111101 k@....... 111" ] = branch("brid",I,false);
        [ "111100 k@....... 111" ] = branch("brie",I,true);
        [ "111100 k@....... 000" ] = branch("brlo",C,true);
        [ "111100 k@....... 100" ] = branch("brlt",S,true);
        [ "111100 k@....... 010" ] = branch("brmi",N,true);
        [ "111101 k@....... 001" ] = branch("brne",Z,false);
        [ "111101 k@....... 010" ] = branch("brpl",N,false);
        [ "111101 k@....... 000" ] = branch("brsh",C,false);
        [ "111101 k@....... 110" ] = branch("brtc",T,false);
        [ "111100 k@....... 110" ] = branch("brts",T,true);
        [ "111101 k@....... 011" ] = branch("brvc",V,false);
        [ "111100 k@....... 011" ] = branch("brvs",V,true);
        */
        /*
        // SBRC
        [ "1111 110 r@..... 0 b@..." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let b = Rvalue::Constant(st.get_group("b"));
            let masked = new_temp(8);
            let mask = 1 << st.get_group("b");
            let cur = st.address;

            st.mnemonic(2,"sbrc","{8}, {3}",vec!(a,b),|cg: &mut CodeGen| {
                cg.and_i(masked,a.to_rv(),mask);
            });

            let g = Guard::from_rel(Relation::NotEqual(Rvalue::Constant(0),masked));
            st.jump(Rvalue::Constant(cur + 2,g.negation()));
            st.jump(Rvalue::Constant(cur + 4,g));

            true
        },
        // SBRS
        [ "1111 111 r@..... 0 b@..." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let b = Rvalue::Constant(st.get_group("b"));
            let masked = new_temp(8);
            let mask = 1 << st.get_group("b");
            let cur = st.address;

            st.mnemonic(2,"sbrs","{8}, {3}",vec!(a,b),|cg: &mut CodeGen| {
                cg.and_i(masked,a.to_rv(),mask);
            });

            let g = Guard::from_rel(Relation::NotEqual(Rvalue::Constant(0),masked));
            st.jump(Rvalue::Constant(cur + 4,g.negation()));
            st.jump(Rvalue::Constant(cur + 2,g));

            true
        },
        // CPSE
        [ "000100 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let rd = reg(st,"d");
            let cur = st.address;
            let g = Guard::from_rel(Relation::Equal(rr,rd));

            st.mnemonic(2,"cpse","{8}, {8}",vec!(rd,rr),|cg: &mut CodeGen| {});
            st.jump(cur + 2,g.negation());
            st.jump(cur + 4,g);

            true
        },
        // SBIC
        [ "1001 1001 A@..... b@..." ] = |st: &mut State<u16>| {
            let a = decode_ioreg(st,"A");
            let b = Rvalue::Constant(st.get_group("b"));
            let masked = new_temp(8);
            let mask = 1 << st.get_group("b");
            let cur = st.address;

            st.mnemonic(2,"sbic","{8}, {3}",vec!(a,b),|cg: &mut CodeGen| {
                cg.and_i(masked,a.to_rv(),mask);
            });

            let g = Guard::from_rel(Relation::NotEqual(Rvalue::Constant(0),masked));
            st.jump(Rvalue::Constant(cur + 2,g.negation()));
            st.jump(Rvalue::Constant(cur + 4,g));

            true
        },
        // SBIS
        [ "1001 1011 A@..... b@..." ] = |st: &mut State<u16>| {
            let a = decode_ioreg(st,"A");
            let b = Rvalue::Constant(st.get_group("b"));
            let masked = new_temp(8);
            let mask = 1 << st.get_group("b");
            let cur = st.address;

            st.mnemonic(2,"sbis","{8}, {3}",vec!(a,b),|cg: &mut CodeGen| {
                cg.and_i(masked,a.to_rv(),mask);
            });

            let g = Guard::from_rel(Relation::NotEqual(Rvalue::Constant(0),masked));
            st.jump(Rvalue::Constant(cur + 4,g.negation()));
            st.jump(Rvalue::Constant(cur + 2,g));

            true
        },
        // CALL
        [ "1001010 k@..... 111 k@.", "k@................" ] = |st: &mut State<u16>| {
            let k = Rvalue::Constant((st.get_group("k") * 2));// % (st.state.flash_sz));
            let next = st.address + 4;

            st.mnemonic(2,"call","{24}",vec!(k),|cg: &mut CodeGen| {
                cg.call_i(Lvalue::Undefined,k);
            });
            st.jump(next);
            true
        },
        // JMP
        [ "1001010 k@..... 110 k@.", "k@................" ] = |st: &mut State<u16>| {
            let k = Rvalue::Constant((st.get_group("k") * 2));// % (st.state.flash_sz));

            st.mnemonic(2,"jmp","{24}",vec!(k),|cg: &mut CodeGen| {});
            st.jump(k);
            true
        },
        // RCALL
        [ "1101 k@............" ] = |st: &mut State<u16>| {
        {
            let _k = st.get_groups("k");
            let k = Rvalue::Constant(((if _k <= 2047 { _k } else { _k - 4096 }) * 2 + 2 + st.address));// % st.state.flash_sz);
            let next = st.address + 4;

            st.mnemonic(st.tokens.size() * 2,"rcall","{24}",vec!(k),|cg: &mut CodeGen| {
                cg.call_i(Rvalue::Undefined,k);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // RJMP
        [ "1100 k@............" ] = |st: &mut State<u16>| {
            let _k = st.get_group("k");
            let k = Rvalue::Constant(((if _k <= 2047 { _k } else { _k - 4096 }) * 2 + 2 + st.address));// % st.state.flash_sz);

            st.mnemonic(2,"rjmp","{24}",vec!(k),|cg: &mut CodeGen| {});
            st.jump(k);
            true
        },
        // RET
        [ 0x9518 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"ret","",vec!(),|cg: &mut CodeGen| {});
            st.jump(j,Guard::new());
            true
        },
        // RETI
        [ 0x9509 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"reti","",vec!(),|cg: &mut CodeGen| {});
            st.jump(j,Guard::new());
            true
        },
        // IJMP
        [ 0x9509 ] = |st: &mut State<u16>| {
            let next = st.address + 1;
            let j = new_temp(24);

            st.mnemonic(2,"ijmp","",vec!(),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());
                //cg.mod_i(z.clone(),Rvalue::Constant(st.flash_sz));

                cg.assign(j,sram(z.to_rv()));
            });
            st.jump(j,Guard::new());
            true
        },
        // ICALL
        [ 0x9509 ] = |st: &mut State<u16>| {
            let next = st.address + 1;

            st.mnemonic(2,"icall","",vec!(),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());
                //cg.mod_i(z.clone(),Rvalue::Constant(st.flash_sz));

                cg.call_i(Lvalue::Undefined,sram(z.to_rv()));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        */
        /*
        // store and load with x,y,z
        [ "1001 001r@. r@.... 1100" ] = binary_st(r26,r27,false,false);
        [ "1001 001r@. r@.... 1101" ] = binary_st(r26,r27,false,true);
        [ "1001 001r@. r@.... 1110" ] = binary_st(r26,r27,true,false);

        [ "1000 001r@. r@.... 1000" ] = binary_st(r28,r29,false,false);
        [ "1001 001r@. r@.... 1001" ] = binary_st(r28,r29,false,true);
        [ "1001 001r@. r@.... 1010" ] = binary_st(r28,r29,true,false);
        [ "10q@.0 q@..1r@. r@.... 1q@..." ] = binary_stq(r28,r29);

        [ "1000 001r@. r@.... 0000" ] = binary_st(r30,r31,false,false);
        [ "1001 001r@. r@.... 0001" ] = binary_st(r30,r31,false,true);
        [ "1001 001r@. r@.... 0010" ] = binary_st(r30,r31,true,false);
        [ "10q@.0 q@..1r@. r@.... 0q@..." ] = binary_stq(r30,r31);

        [ "1001 000d@. d@.... 1100" ] = binary_ld(r26,r27,false,false);
        [ "1001 000d@. d@.... 1101" ] = binary_ld(r26,r27,false,true);
        [ "1001 000d@. d@.... 1110" ] = binary_ld(r26,r27,true,false);

        [ "1000 000d@. d@.... 1000" ] = binary_ld(r28,r29,false,false);
        [ "1001 000d@. d@.... 1001" ] = binary_ld(r28,r29,false,true);
        [ "1001 000d@. d@.... 1010" ] = binary_ld(r28,r29,true,false);
        [ "10 q@. 0 q@.. 0 d@..... 1 q@..." ] = binary_ldq(r28,r29);

        [ "1000 000d@. d@.... 0000" ] = binary_ld(r30,r31,false,false);
        [ "1001 000 d@..... 0001" ] = binary_ld(r30,r31,false,true);
        [ "1001 000d@. d@.... 0010" ] = binary_ld(r30,r31,true,false);
        [ "10q@.0 q@..0d@. d@.... 0q@..." ] = binary_ldq(r30,r31);
        */
        /*
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
                cg.assign(R0,Rvalue::Undefined);
                cg.assign(R1,Rvalue::Undefined);
                cg.assign(R2,Rvalue::Undefined);
                cg.assign(R3,Rvalue::Undefined);
                cg.assign(R4,Rvalue::Undefined);
                cg.assign(R5,Rvalue::Undefined);
                cg.assign(R6,Rvalue::Undefined);
                cg.assign(R7,Rvalue::Undefined);
                cg.assign(R8,Rvalue::Undefined);
                cg.assign(R9,Rvalue::Undefined);
                cg.assign(R10,Rvalue::Undefined);
                cg.assign(R11,Rvalue::Undefined);
                cg.assign(R12,Rvalue::Undefined);
                cg.assign(R13,Rvalue::Undefined);
                cg.assign(R14,Rvalue::Undefined);
                cg.assign(R15,Rvalue::Undefined);
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
        },*/
        // catch all
        _ = |st: &mut State<u16>| {
            let next = st.address + 1;
            let rd = reg(st,"d");

            st.mnemonic(1,"unk","",vec!(),|cg: &mut CodeGen| {});
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        }
    );

    Program::disassemble(None,main,State::<u16>::new(0),data,0)
}
