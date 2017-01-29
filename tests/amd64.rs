/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate panopticon;

extern crate env_logger;
extern crate regex;

#[macro_use]
extern crate quickcheck;

use panopticon::{
    Region,
    Architecture,
    amd64,
    Result,
    Rvalue,
    Lvalue,
    execute,
    lift,
    Statement,
};
use panopticon::amd64::{
    tables,
    semantic,
    Opcode,
    MnemonicSpec,
    OperandSpec,
    AddressingMethod,
    OperandType,
    JumpSpec,
    Operand,
    read_spec_register,
};

use quickcheck::{Arbitrary,Gen,TestResult,Testable};
use std::path::Path;
use std::cmp;
use std::borrow::Cow;

#[test]
fn amd64_opcodes() {
    let reg = Region::open("com".to_string(),Path::new("tests/data/amd64.com")).unwrap();
    let mut addr = 0;

    loop {
        let maybe_match = <amd64::Amd64 as Architecture>::decode(&reg,addr,&amd64::Mode::Long);

        if let Ok(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}",mne.area.start,mne.opcode);
                addr = mne.area.end;

                if addr >= reg.size() {
                    return;
                }
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
    env_logger::init().unwrap();

    let reg = Region::open("com".to_string(),Path::new("tests/data/ia32.com")).unwrap();
    let mut addr = 0;

    loop {
        let maybe_match = amd64::Amd64::decode(&reg,addr,&amd64::Mode::Protected);

        if let Ok(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}",mne.area.start,mne.opcode);
                addr = mne.area.end;

                if addr >= reg.size() {
                    return;
                }
            }
        } else if addr < reg.size() {
            unreachable!("failed to match anything at {:x}",addr);
        } else {
            break;
        }
    }
}

#[derive(Clone,Debug)]
struct Context {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    flags: u8,
}

impl Arbitrary for Context {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Context{
            rax: g.gen(),
            rbx: g.gen(),
            rcx: g.gen(),
            rdx: g.gen(),
            rsi: g.gen(),
            rdi: g.gen(),
            rbp: g.gen(),
            r8: g.gen(),
            r9: g.gen(),
            r10: g.gen(),
            r11: g.gen(),
            r12: g.gen(),
            r13: g.gen(),
            r14: g.gen(),
            r15: g.gen(),
            flags: g.gen(),
        }
    }
}

#[derive(Debug,Clone)]
enum SampledOperand {
    Register(Cow<'static,str>,u64,usize),
    Immediate(u64,usize),
}

fn sample_register<G: Gen>(g: &mut G, opsz: usize) -> Result<SampledOperand> {
    Ok(match opsz {
        8 => {
            SampledOperand::Register(
                g.choose(&[
                    //"AH","BH","CH","DH",
                    "AL","BL","CL","DL","SIL","DIL","BPL",
                    "R8B","R9B","R10B","R11B","R12B","R13B","R14B","R15B"]).unwrap().to_string().into(),
                g.gen::<u8>() as u64,8)
        }
        16 => {
            SampledOperand::Register(
                g.choose(&[
                    "AX","BX","CX","DX","SI","DI","BP",
                    "R8W","R9W","R10W","R11W","R12W","R13W","R14W","R15W"]).unwrap().to_string().into(),
                g.gen::<u16>() as u64,16)
        }
        32 => {
            SampledOperand::Register(
                g.choose(&[
                    "EAX","EBX","ECX","EDX","ESI","EDI","EBP",
                    "R8D","R9D","R10D","R11D","R12D","R13D","R14D","R15D"]).unwrap().to_string().into(),
                g.gen::<u32>() as u64,32)
        }
        64 => {
            SampledOperand::Register(
                g.choose(&[
                    "RAX","RBX","RCX","RDX","RSI","RDI","RBP",
                    "R8","R9","R10","R11","R12","R13","R14","R15"]).unwrap().to_string().into(),
                g.gen::<u64>(),64)
        }
        _ => return Err("Invalid operator size".into())
    })
}

fn sample_simd_register<G: Gen>(g: &mut G, opsz: usize) -> Result<SampledOperand> {
    Ok(match opsz {
        32 => {
            SampledOperand::Register(
                g.choose(&[
                    "MM0","MM1","MM2","MM3","MM4","MM5", "MM6", "MM7"]).unwrap().to_string().into(),
                g.gen::<u32>() as u64,32)
        }
        64 => {
            SampledOperand::Register(
                g.choose(&[
                    "MMX0","MMX1","MMX2","MMX3","MMX4","MMX5", "MMX6", "MMX7"]).unwrap().to_string().into(),
                g.gen::<u64>(),64)
        }
        128 => {
            SampledOperand::Register(
                g.choose(&[
                    "XMM0","XMM1","XMM2","XMM3","XMM4","XMM5","XMM6","XMM7",
                    "XMM8","XMM9","XMM10","XMM11","XMM12","XMM13","XMM14","XMM15"]).unwrap().to_string().into(),
                g.gen::<u64>(),128)
        }
        256 => {
            SampledOperand::Register(
                g.choose(&[
                    "YMM0","YMM1","YMM2","YMM3","YMM4","YMM5","YMM6","YMM7",
                    "YMM8","YMM9","YMM10","YMM11","YMM12","YMM13","YMM14","YMM15"]).unwrap().to_string().into(),
                g.gen::<u64>(),256)
        }
        _ => return Err("Invalid operator size".into())
    })
}

fn sample_register_variant<G: Gen>(reg: &OperandType,opsz: usize, g: &mut G) -> Result<SampledOperand> {
    read_spec_register(reg.clone(),opsz,g.gen::<bool>()).map(|x| {
        if let &Operand::Register(ref reg) = &x {
            SampledOperand::Register(format!("{}",x).into(),g.gen::<u64>(),reg.width())
        } else {
            unreachable!()
        }
    })
}

fn sample_operand<G: Gen>(spec: &OperandSpec, opsz: usize, simdsz: usize, g: &mut G) -> Result<SampledOperand> {
    match (spec,opsz) {
        (&OperandSpec::Present(AddressingMethod::None,ref reg),opsz) =>
            sample_register_variant(reg,opsz,g),
        (&OperandSpec::Present(AddressingMethod::B,OperandType::y),opsz) =>
            sample_register(g,opsz),
        /*(&OperandSpec::Present(AddressingMethod::C,OperandType::d),_) =>
            sample_ctrl_register(g,32),
        (&OperandSpec::Present(AddressingMethod::D,OperandType::d),_) =>
            sample_debug_register(g,32),*/

        // E
        (&OperandSpec::Present(AddressingMethod::E,OperandType::v),opsz) =>
            sample_register(g,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::z),opsz) =>
            sample_register(g,cmp::min(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::y),opsz) =>
            sample_register(g,cmp::max(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::b),_) =>
            sample_register(g,8),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::w),_) =>
            sample_register(g,16),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::d),_) =>
            sample_register(g,32),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::dq),_) =>
            sample_register(g,64),

        // G
        (&OperandSpec::Present(AddressingMethod::G,OperandType::dq),_) =>
            sample_register(g,64),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::d),_) =>
            sample_register(g,32),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::w),_) =>
            sample_register(g,16),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::b),_) =>
            sample_register(g,8),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::v),opsz) =>
            sample_register(g,opsz),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::z),opsz) =>
            sample_register(g,cmp::min(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::y),opsz) =>
            sample_register(g,cmp::max(32,opsz)),

        // H
        (&OperandSpec::Present(AddressingMethod::H,OperandType::x),opsz) =>
            sample_simd_register(g,opsz),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::qq),_) =>
            sample_simd_register(g,256),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::dq),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::ps),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::pd),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::ss),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::sd),_) =>
            sample_simd_register(g,128),

        // I
        (&OperandSpec::Present(AddressingMethod::I,OperandType::z),16) =>
            Ok(SampledOperand::Immediate(g.gen::<i16>() as i64 as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::z),_) =>
            Ok(SampledOperand::Immediate(g.gen::<i32>() as i64 as u64,32)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::b),_) =>
            Ok(SampledOperand::Immediate(g.gen::<i8>() as i64 as u64,8)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::one),opsz) =>
            Ok(SampledOperand::Immediate(1,opsz)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::w),_) =>
            Ok(SampledOperand::Immediate(g.gen::<i16>() as i64 as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),16) =>
            Ok(SampledOperand::Immediate(g.gen::<i16>() as i64 as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),32) =>
            Ok(SampledOperand::Immediate(g.gen::<i32>() as i64 as u64,32)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),64) =>
            Ok(SampledOperand::Immediate(g.gen::<u64>(),64)),

        // L
        (&OperandSpec::Present(AddressingMethod::L,OperandType::x),32) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::L,OperandType::x),_) =>
            sample_simd_register(g,simdsz),

        (&OperandSpec::Present(AddressingMethod::N,OperandType::q),_) =>
            sample_simd_register(g,64),

        // P
        (&OperandSpec::Present(AddressingMethod::P,OperandType::pi),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::ps),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::q),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::d),_) =>
            sample_simd_register(g,32),

        // Q
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::d),_) =>
            sample_simd_register(g,32),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::pi),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::q),_) =>
            sample_simd_register(g,32),

        // U
        (&OperandSpec::Present(AddressingMethod::U,OperandType::ps),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::pi),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::q),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::x),32) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::x),64) =>
            sample_simd_register(g,256),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::dq),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::pi),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::ps),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::pd),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::ss),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::x),32) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::x),64) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::dq),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::q),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::sd),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::y),opsz) =>
            sample_simd_register(g,cmp::min(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::pd),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::ps),_) =>
            sample_simd_register(g,simdsz),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::q),_) =>
            sample_simd_register(g,64),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::dq),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::x),32) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::x),64) =>
            sample_simd_register(g,256),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::sd),_) =>
            sample_simd_register(g,128),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::ss),_) =>
            sample_simd_register(g,128),
        _ => {
            Err(format!("can't decode {:?}/{}",spec,opsz).into())
        }
    }
}

fn operand_specs(mnemonic: &'static str) -> Vec<Vec<&'static OperandSpec>> {
    let tables_8 = &[
        &tables::GROUP1_OPC80, &tables::GROUP1_OPC81, &tables::GROUP1_OPC82, &tables::GROUP1_OPC83,
        &tables::GROUP101_OPC8F, &tables::GROUP2_OPCC0, &tables::GROUP2_OPCC1, &tables::GROUP2_OPCD0,
        &tables::GROUP2_OPCD1, &tables::GROUP2_OPCD2, &tables::GROUP2_OPCD3, &tables::GROUP3_OPCF6,
        &tables::GROUP3_OPCF7, &tables::GROUP4_OPCFE, &tables::GROUP5_OPCFF, &tables::GROUP6_OPC00,
        &tables::GROUP7_OPC01_MEM, &tables::GROUP7_OPC01_MEM, &tables::GROUP7_OPC01_MEM, &tables::GROUP8_OPCBA,
        &tables::GROUP10_OPCB9, &tables::GROUP11_OPCC6, &tables::GROUP11_OPCC7, &tables::GROUP12_OPC71,
        &tables::GROUP12_OPC6671, &tables::GROUP13_OPC72, &tables::GROUP13_OPC6672, &tables::GROUP14_OPC73,
        &tables::GROUP14_OPC6673, &tables::GROUP102_OPC01
    ];
    let tables_256 = &[
        &tables::ONEBYTE_TABLE,
        &tables::TWOBYTE_TABLE, &tables::TWOBYTE_F2_TABLE, &tables::TWOBYTE_F3_TABLE, &tables::TWOBYTE_66_TABLE,
        &tables::THREEBYTE_3A_TABLE, &tables::THREEBYTE_3AF2_TABLE, &tables::THREEBYTE_3A66_TABLE,
        &tables::THREEBYTE_38_TABLE, &tables::THREEBYTE_38F3_TABLE, &tables::THREEBYTE_38F2_TABLE, &tables::THREEBYTE_3866_TABLE
    ];

    let mut ret = vec![];

    fn _impl(cell: &'static Opcode, mnemonic: &'static str, ret: &mut Vec<Vec<&'static OperandSpec>>) {
        match cell {
            &Opcode::Nonary(MnemonicSpec::Single(ref mne),_,_) if *mne == mnemonic => ret.push(vec![]),
            &Opcode::Unary(MnemonicSpec::Single(ref mne),_,_,ref op) if *mne == mnemonic => ret.push(vec![op]),
            &Opcode::Binary(MnemonicSpec::Single(ref mne),_,_,ref op1,ref op2) if *mne == mnemonic => ret.push(vec![op1,op2]),
            &Opcode::Trinary(MnemonicSpec::Single(ref mne),_,_,ref op1,ref op2,ref op3) if *mne == mnemonic => ret.push(vec![op1,op2,op3]),
            &Opcode::Quaternary(MnemonicSpec::Single(ref mne),_,_,ref op1,ref op2,ref op3,ref op4) if *mne == mnemonic => ret.push(vec![op1,op2,op3,op4]),
            _ => {}
        }
    }

    for tbl in tables_8.iter() {
        for cell in tbl.iter() {
            _impl(cell,mnemonic,&mut ret);
        }
    }

    for tbl in tables_256.iter() {
        for cell in tbl.iter() {
            _impl(cell,mnemonic,&mut ret);
        }
    }

    ret
}

fn rappel_xcheck(mnemonic: &str, sem: fn(Rvalue,Rvalue) -> Result<(Vec<Statement>,JumpSpec)>, a: SampledOperand, b: SampledOperand,start: Context) -> Result<bool> {
    use std::process::{Stdio,Command};
    use std::io::{Read,Write};
    use regex::Regex;
    use std::collections::HashMap;
    use std::borrow::Cow;

    println!("{:?}",start);

    let regs_re = Regex::new(r"(rax|rbx|rcx|rdx|rsi|rdi|r8 |r9 |r10|r11|r12|r13|r14|r15): (.......)?(0x................)").unwrap();
    let flags_re = Regex::new(r"(cf|zf|of|sf|pf|af):(.)").unwrap();
    let mut stmts = vec![];
    let mut child = Command::new("rappel")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn().ok().unwrap();
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( AH:8 ),Rvalue::new_u8(start.flags)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::sahf().map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RAX:64 ),Rvalue::new_u64(start.rax)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RBX:64 ),Rvalue::new_u64(start.rbx)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RCX:64 ),Rvalue::new_u64(start.rcx)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RDX:64 ),Rvalue::new_u64(start.rdx)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RSI:64 ),Rvalue::new_u64(start.rsi)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RDI:64 ),Rvalue::new_u64(start.rdi)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RBP:64 ),Rvalue::new_u64(start.rbp)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R8:64 ),Rvalue::new_u64(start.r8)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R9:64 ),Rvalue::new_u64(start.r9)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R10:64 ),Rvalue::new_u64(start.r10)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R11:64 ),Rvalue::new_u64(start.r11)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R12:64 ),Rvalue::new_u64(start.r12)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R13:64 ),Rvalue::new_u64(start.r13)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R14:64 ),Rvalue::new_u64(start.r14)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R15:64 ),Rvalue::new_u64(start.r15)).map(|x| x.0)));
    stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R15:64 ),Rvalue::new_u64(start.r15)).map(|x| x.0)));

    match (&a,&b) {
        (&SampledOperand::Register(ref nam1,ref val1,ref sz1),&SampledOperand::Register(ref nam2,ref val2,ref sz2)) => {
            let a_var = Rvalue::Variable{ name: nam1.clone().into(), size: *sz1, subscript: None, offset: 0 };
            let b_var = Rvalue::Variable{ name: nam2.clone().into(), size: *sz2, subscript: None, offset: 0 };

            stmts.append(&mut try!(semantic::mov(a_var.clone(),Rvalue::Constant{ value: *val1, size: *sz1 }).map(|x| x.0)));
            stmts.append(&mut try!(semantic::mov(b_var.clone(),Rvalue::Constant{ value: *val2, size: *sz2 }).map(|x| x.0)));
            stmts.append(&mut try!(sem(a_var,b_var)).0);
        }
        (&SampledOperand::Register(ref nam1,ref val1,ref sz1),&SampledOperand::Immediate(ref val2,ref sz2)) => {
            let a_var = Rvalue::Variable{ name: nam1.clone().into(), size: *sz1, subscript: None, offset: 0 };
            let b_val = Rvalue::Constant{ value: *val2, size: *sz2 };

            stmts.append(&mut try!(semantic::mov(a_var.clone(),Rvalue::Constant{ value: *val1, size: *sz1 }).map(|x| x.0)));
            stmts.append(&mut try!(sem(a_var,b_val)).0);
        }
        _ => unreachable!()
    }

    if let (&mut Some(ref mut stdin),&Some(_)) = (&mut child.stdin,&child.stdout) {
        let _ = try!(stdin.write(&format!("mov ah, 0x{:x}\n",start.flags).into_bytes()));
        let _ = try!(stdin.write(b"sahf\n"));
        let _ = try!(stdin.write(&format!("mov rax, 0x{:x}\n",start.rax).into_bytes()));
        let _ = try!(stdin.write(&format!("mov rbx, 0x{:x}\n",start.rbx).into_bytes()));
        let _ = try!(stdin.write(&format!("mov rcx, 0x{:x}\n",start.rcx).into_bytes()));
        let _ = try!(stdin.write(&format!("mov rdx, 0x{:x}\n",start.rdx).into_bytes()));
        let _ = try!(stdin.write(&format!("mov rsi, 0x{:x}\n",start.rsi).into_bytes()));
        let _ = try!(stdin.write(&format!("mov rdi, 0x{:x}\n",start.rdi).into_bytes()));
        let _ = try!(stdin.write(&format!("mov rbp, 0x{:x}\n",start.rbp).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r8, 0x{:x}\n",start.r8).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r9, 0x{:x}\n",start.r9).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r10, 0x{:x}\n",start.r10).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r11, 0x{:x}\n",start.r11).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r12, 0x{:x}\n",start.r12).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r13, 0x{:x}\n",start.r13).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r14, 0x{:x}\n",start.r14).into_bytes()));
        let _ = try!(stdin.write(&format!("mov r15, 0x{:x}\n",start.r15).into_bytes()));

        match (&a,&b) {
            (&SampledOperand::Register(ref nam1,ref val1,_),&SampledOperand::Register(ref nam2,ref val2,_)) => {
                let _ = try!(stdin.write(&format!("mov {}, 0x{:x}\n",nam1,val1).into_bytes()));
                let _ = try!(stdin.write(&format!("mov {}, 0x{:x}\n",nam2,val2).into_bytes()));
                let _ = try!(stdin.write(&format!("{} {}, {}\n",mnemonic,nam1,nam2).into_bytes()));

                println!("{} {}, {}",mnemonic,nam1,nam2);
            }
            (&SampledOperand::Register(ref nam1,ref val1,_),&SampledOperand::Immediate(ref val2,_)) => {
                let _ = try!(stdin.write(&format!("mov {}, 0x{:x}\n",nam1,val1).into_bytes()));
                let _ = try!(stdin.write(&format!("{} {}, 0x{:x}\n",mnemonic,nam1,val2).into_bytes()));

                println!("{} {}, 0x{:x}",mnemonic,nam1,val2);
            }
            _ => unreachable!()
        }
    }

    if !try!(child.wait()).success() {
        return Ok(false);
    }

    let mut out = String::new();
    let _ = try!(child.stdout.ok_or("No output")).read_to_string(&mut out);
    //println!("{}",out);
    let regs = regs_re.captures_iter(&out).filter_map(|x| {
        if let (Some(ref nam),Some(ref s)) = (x.at(1),x.at(3)) {
            if let Ok(val) = u64::from_str_radix(&s[2..],16) {
                Some((nam.to_string(),val))
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<_>>();
    let flags = flags_re.captures_iter(&out).filter_map(|x| {
        if let (Some(ref nam),Some(ref s)) = (x.at(1),x.at(2)) {
            Some((nam.to_string(),*s != "0".to_string()))
        } else {
            None
        }
    }).collect::<Vec<_>>();

    assert_eq!(regs.len(), 14);
    assert_eq!(flags.len(), 6);
    println!("regs: {:?}",regs);

    let mut ctx = HashMap::<Cow<'static,str>,u64>::new();

    for stmt in stmts {
        let s = lift(&stmt.op,&|rv| {
            if let &Rvalue::Variable{ ref name, ref offset, ref size,.. } = rv {
                if let Some(val) = ctx.get(name.as_ref()) {
                    if *size < 64 {
                        Rvalue::Constant{ value: (*val >> *offset as usize) % (1 << *size), size: *size }
                    } else {
                        Rvalue::Constant{ value: (*val >> *offset), size: *size }
                    }
                } else {
                    rv.clone()
                }
            } else {
                rv.clone()
            }
        });

        println!("{}",Statement{ assignee: stmt.assignee.clone(), op: s.clone() });

        if let Lvalue::Variable{ ref name,.. } = stmt.assignee {
            let res =  execute(s);
            println!("\t-> {}",res);

            match res {
                Rvalue::Constant{ ref value,.. } => {
                    ctx.insert(name.clone(),*value);
                }
                Rvalue::Undefined => {
                    ctx.remove(name);
                }
                _ => {}
            }
        }
    }

    println!("{:?}",ctx);

    for (name,val) in regs {
        let key = Cow::Owned(name.trim().clone().to_uppercase());

        if Some(val) != ctx.get(&key).map(|x| *x as u64) {
            println!("{}:\n\tHardware = 0x{:x}\n\tSoftware = 0x{:x}",key,val,ctx.get(&key).unwrap_or(&0));
            return Ok(false);
        }
    }

    for (name,val) in flags {
        let key = Cow::Owned(name.trim().clone().to_uppercase());
        let soft = ctx.get(&key).map(|x| *x as u64);

        if soft.is_some() && Some(if val { 1 } else { 0 }) != soft {
            println!("{}:\n\tHardware = {}\n\tSoftware = 0x{:x}",name,val,soft.unwrap());
            return Ok(false);
        }
    }

    Ok(true)
}

macro_rules! rappel_xcheck {
    ($mne:ident,$func:ident,$typ:ident) => { rappel_xcheck!($mne,$func,$typ,$mne); };
    ($mne:ident,$func:ident,$typ:ident,$sem:ident) => {
        struct $typ(Vec<Vec<&'static OperandSpec>>);

        impl Testable for $typ {
            fn result<G: Gen>(&self, g: &mut G) -> TestResult {
                fn size(x: &SampledOperand) -> usize {
                    match x {
                        &SampledOperand::Register(_,_,s) => s,
                        &SampledOperand::Immediate(_,s) => s,
                    }
                }
                let specs = g.choose(&self.0).unwrap();
                println!("{:?}",specs);
                let opsz = *g.choose(&[16,32,64]).unwrap();
                let simdsz = *g.choose(&[32,64,128,256]).unwrap();
                let ops = specs.into_iter().map(|x| sample_operand(x,opsz,simdsz,g).ok()).collect::<Vec<_>>();
                let ctx = Context::arbitrary(g);
                let mut discard = !ops.iter().all(|x| x.is_some());

                if !discard && ops.len() == 2 {
                    let a = ops.get(0).cloned().unwrap().unwrap();
                    let b = ops.get(1).cloned().unwrap().unwrap();

                    discard |= (stringify!($mne) == "movsx" && size(&a) <= size(&b));
                    discard |= (stringify!($mne) == "movzx" && size(&a) <= size(&b));

                    if !discard {
                        let ret = rappel_xcheck(stringify!($mne),semantic::$sem,a,b,ctx);
                        match ret {
                            Ok(b) => TestResult::from_bool(b),
                            Err(s) => panic!(format!("{:?}",s)),
                        }
                    } else {
                        TestResult::discard()
                    }
                } else {
                    TestResult::discard()
                }
            }
        }

        #[test]
        #[cfg_attr(not(feature = "cross_check_amd64"), ignore)]
        fn $func() {
            use quickcheck::QuickCheck;

            QuickCheck::new()
                .tests(100)
                .quickcheck($typ(operand_specs(stringify!($mne))));
        }
    }
}

rappel_xcheck!(adc,xcheck_adc,Adc);
rappel_xcheck!(add,xcheck_add,Add);
rappel_xcheck!(sub,xcheck_sub,Sub);
rappel_xcheck!(sbb,xcheck_sbb,Sbb);
rappel_xcheck!(xor,xcheck_xor,Xor);
rappel_xcheck!(and,xcheck_and,And);
rappel_xcheck!(or,xcheck_or,Or);
rappel_xcheck!(cmp,xcheck_cmp,Cmp);
//rappel_xcheck!(mul,xcheck_mul,Mul,mul2);
rappel_xcheck!(mov,xcheck_mov,Mov);
rappel_xcheck!(movsx,xcheck_movsx,Movsx);
rappel_xcheck!(movzx,xcheck_movzx,Movzx);
//rappel_xcheck!(imul,xcheck_imul,Imul,imul2);
rappel_xcheck!(rol,xcheck_rol,Rol);
rappel_xcheck!(sar,xcheck_sar,Sar);
rappel_xcheck!(shl,xcheck_shl,Shl);
rappel_xcheck!(shr,xcheck_shr,Shr);
rappel_xcheck!(xchg,xcheck_xchg,Xchg);
