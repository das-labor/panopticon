use value::{Lvalue,Rvalue,Endianess};
use disassembler::State;
use amd64::{Amd64,Mode};
use codegen::CodeGen;

fn byte(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 1,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn word(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 2,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn dword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 4,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn qword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 8,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

pub fn decode_m(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    unimplemented!();
}

pub fn decode_d(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    unimplemented!();
}

pub fn decode_imm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    unimplemented!();
}

pub fn decode_moffs(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    unimplemented!();
}

pub fn decode_rm1(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    unimplemented!();
}

pub fn decode_i(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_fd(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_td(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_regms(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_sregm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_dbgrm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmdbg(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_ctrlrm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmctrl(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_mr(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_mi(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_m1(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_mc(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_ii(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rvm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmv(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmi(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_mri(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rvmi(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_reg8(r_reg: usize,rex: bool) -> Lvalue {
    unimplemented!();
}

pub fn decode_reg16(r_reg: usize) -> Lvalue {
    unimplemented!();
}

pub fn decode_reg32(r_reg: usize) -> Lvalue {
    unimplemented!();
}

pub fn decode_reg64(r_reg: usize) -> Lvalue {
    unimplemented!();
}

pub fn binary(opcode: &str,
                  decode: fn(&mut State<Amd64>, &mut CodeGen) -> (Rvalue,Rvalue),
                  sem: fn(cg: &mut CodeGen, a: Rvalue, b: Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary_reg(opcode: &str,
                  a: &Lvalue,
                  decode: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
                  sem: fn(cg: &mut CodeGen, a: Rvalue, b: Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}
