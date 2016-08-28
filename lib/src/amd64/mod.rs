/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015, 2016 Kai Michaelis
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

use std::sync::Arc;
use std::borrow::Cow;
use std::cmp;
use std::fmt::{Error,Display,Formatter};
use std::result;

use std::io::Cursor;
use byteorder::{ReadBytesExt,LittleEndian};

use {
    Lvalue,
    Rvalue,
    Result,
    LayerIter,
    Match,
    Architecture,
    Region,
    Statement,
};

#[macro_use]
mod tables;

#[derive(Clone,Debug)]
pub enum Amd64 {}

#[derive(Clone,PartialEq,Copy)]
pub enum Condition {
    Overflow,
    NotOverflow,
    Carry,
    AboveEqual,
    Equal,
    NotEqual,
    BelowEqual,
    Above,
    Sign,
    NotSign,
    Parity,
    NotParity,
    Less,
    GreaterEqual,
    LessEqual,
    Greater,
}

#[derive(Clone,PartialEq,Copy,Debug)]
pub enum Mode {
    Real,       // Real mode / Virtual 8086 mode
    Protected,  // Protected mode / Long compatibility mode
    Long,       // Long 64-bit mode
}

impl Mode {
    pub fn alt_bits(&self) -> usize {
        match self {
            &Mode::Real => 32,
            &Mode::Protected => 16,
            &Mode::Long => 16,
        }
    }

    pub fn bits(&self) -> usize {
        match self {
            &Mode::Real => 16,
            &Mode::Protected => 32,
            &Mode::Long => 64,
        }
    }
}

impl Architecture for Amd64 {
    type Token = u8;
    type Configuration = Mode;

    fn prepare(_: &Region,_: &Self::Configuration) -> Result<Vec<(&'static str,u64,&'static str)>> {
        Ok(vec![])
    }

    fn decode(reg: &Region,start: u64,_: &Self::Configuration) -> Result<Match<Self>> {
        let mut data = reg.iter();
        let mut buf: Vec<u8> = vec![];
        let mut i = data.seek(start);
        let mut j = 0;
        let mut p = 0;

        while let Some(Some(b)) = i.next() {
            buf.push(b);

            if buf.len() >= 15 {
                debug!("disass @ {:x}: {:?}",(p+1)-j+(start as usize),buf);
                let l = match ::amd64::read(::amd64::Mode::Long,&buf) {
                    Ok(l) => l,
                    Err(s) => {
                    error!("{} at {:x}",s,p+1-j+start as usize);
                    unreachable!()
                    }
                };

                buf = buf.split_off(l);
            }
            p += 1;
        }

        Err("todo".into())
    }
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum SegmentOverride {
    None, Cs, Ss, Ds, Es, Fs, Gs,
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum BranchHint {
    None, Taken, NotTaken,
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum SimdPrefix {
    None,
    PrefixF2,
    PrefixF3,
    Prefix66,
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum OpcodeEscape {
    None,
    Escape0F,
    Escape0F0F,
    Escape0F38,
    Escape0F3A,
    Xop8,
    Xop9,
    XopA,
}

#[derive(Debug)]
struct Prefix {
    pub lock: bool,
    pub repe: bool,
    pub repne: bool,
    pub seg_override: SegmentOverride,
    pub branch_hint: BranchHint,
    pub operand_size: usize,
    pub address_size: usize,
    pub simd_prefix: SimdPrefix,
    pub opcode_escape: OpcodeEscape,
    pub vvvv: Option<u8>,
    pub rex_r: bool,
    pub rex_b: bool,
    pub rex_x: bool,
    pub rex_w: bool,
}

#[derive(Clone,Debug,PartialEq)]
pub enum Register {
    None,
    RAX, RBX, RCX, RDX, RDI, RSI, RSP, RBP, RIP,
    R8, R9, R10, R11, R12, R13, R14, R15,
    EAX, EBX, ECX, EDX, EDI, ESI, ESP, EBP, EIP,
    R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D,
    AX, BX, CX, DX, DI, SI, SP, BP, IP,
    R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W,
    AL, BL, CL, DL, SPL, BPL, SIL, DIL,
    R8L, R9L, R10L, R11L, R12L, R13L, R14L, R15L,
    AH, BH, CH, DH,
    ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7,
    ES, GS, DS, SS, CS, FS,
    MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7, 
    MMX0, MMX1, MMX2, MMX3, MMX4, MMX5, MMX6, MMX7, 
    XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7, 
    XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15, 
    YMM0, YMM1, YMM2, YMM3, YMM4, YMM5, YMM6, YMM7, 
    YMM8, YMM9, YMM10, YMM11, YMM12, YMM13, YMM14, YMM15, 
    DR0, DR1, DR2, DR3, DR4, DR5, DR6, DR7,
    DR8, DR9, DR10, DR11, DR12, DR13, DR14, DR15, 
    CR0, CR1, CR2, CR3, CR4, CR5, CR6, CR7,
    CR8, CR9, CR10, CR11, CR12, CR13, CR14, CR15,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(),Error> {
        match *self {
            Register::RAX => f.write_str("RAX"),
            Register::RBX => f.write_str("RBX"),
            Register::RCX => f.write_str("RCX"),
            Register::RDX => f.write_str("RDX"),
            Register::RDI => f.write_str("RDI"),
            Register::RSI => f.write_str("RSI"),
            Register::RSP => f.write_str("RSP"),
            Register::RBP => f.write_str("RBP"),
            Register::RIP => f.write_str("RIP"),
            Register::R8 => f.write_str("R8"),
            Register::R9 => f.write_str("R9"),
            Register::R10 => f.write_str("R10"),
            Register::R11 => f.write_str("R11"),
            Register::R12 => f.write_str("R12"),
            Register::R13 => f.write_str("R13"),
            Register::R14 => f.write_str("R14"),
            Register::R15 => f.write_str("R15"),

            Register::EAX => f.write_str("EAX"),
            Register::EBX => f.write_str("EBX"),
            Register::ECX => f.write_str("ECX"),
            Register::EDX => f.write_str("EDX"),
            Register::EDI => f.write_str("EDI"),
            Register::ESI => f.write_str("ESI"),
            Register::ESP => f.write_str("ESP"),
            Register::EBP => f.write_str("EBP"),
            Register::EIP => f.write_str("EIP"),
            Register::R8D => f.write_str("R8D"),
            Register::R9D => f.write_str("R9D"),
            Register::R10D => f.write_str("R10D"),
            Register::R11D => f.write_str("R11D"),
            Register::R12D => f.write_str("R12D"),
            Register::R13D => f.write_str("R13D"),
            Register::R14D => f.write_str("R14D"),
            Register::R15D => f.write_str("R15D"),

            Register::AX => f.write_str("AX"),
            Register::BX => f.write_str("BX"),
            Register::CX => f.write_str("CX"),
            Register::DX => f.write_str("DX"),
            Register::DI => f.write_str("DI"),
            Register::SI => f.write_str("SI"),
            Register::SP => f.write_str("SP"),
            Register::BP => f.write_str("BP"),
            Register::IP => f.write_str("IP"),
            Register::R8W => f.write_str("R8W"),
            Register::R9W => f.write_str("R9W"),
            Register::R10W => f.write_str("R10W"),
            Register::R11W => f.write_str("R11W"),
            Register::R12W => f.write_str("R12W"),
            Register::R13W => f.write_str("R13W"),
            Register::R14W => f.write_str("R14W"),
            Register::R15W => f.write_str("R15W"),

            Register::AL => f.write_str("AL"),
            Register::BL => f.write_str("BL"),
            Register::CL => f.write_str("CL"),
            Register::DL => f.write_str("DL"),
            Register::R8L => f.write_str("R8L"),
            Register::R9L => f.write_str("R9L"),
            Register::R10L => f.write_str("R10L"),
            Register::R11L => f.write_str("R11L"),
            Register::R12L => f.write_str("R12L"),
            Register::R13L => f.write_str("R13L"),
            Register::R14L => f.write_str("R14L"),
            Register::R15L => f.write_str("R15L"),
            Register::DIL => f.write_str("DIL"),
            Register::SIL => f.write_str("SIL"),
            Register::SPL => f.write_str("SPL"),
            Register::BPL => f.write_str("BPL"),

            Register::AH => f.write_str("AH"),
            Register::BH => f.write_str("BH"),
            Register::CH => f.write_str("CH"),
            Register::DH => f.write_str("DH"),

            Register::ES => f.write_str("ES"),
            Register::FS => f.write_str("FS"),
            Register::GS => f.write_str("GS"),
            Register::SS => f.write_str("SS"),
            Register::CS => f.write_str("CS"),
            Register::DS => f.write_str("DS"),

            Register::ST0 => f.write_str("ST0"),
            Register::ST1 => f.write_str("ST1"),
            Register::ST2 => f.write_str("ST2"),
            Register::ST3 => f.write_str("ST3"),
            Register::ST4 => f.write_str("ST4"),
            Register::ST5 => f.write_str("ST5"),
            Register::ST6 => f.write_str("ST6"),
            Register::ST7 => f.write_str("ST7"),

            Register::MM0 => f.write_str("MM0"),
            Register::MM1 => f.write_str("MM1"),
            Register::MM2 => f.write_str("MM2"),
            Register::MM3 => f.write_str("MM3"),
            Register::MM4 => f.write_str("MM4"),
            Register::MM5 => f.write_str("MM5"),
            Register::MM6 => f.write_str("MM6"),
            Register::MM7 => f.write_str("MM7"),

            Register::MMX0 => f.write_str("MMX0"),
            Register::MMX1 => f.write_str("MMX1"),
            Register::MMX2 => f.write_str("MMX2"),
            Register::MMX3 => f.write_str("MMX3"),
            Register::MMX4 => f.write_str("MMX4"),
            Register::MMX5 => f.write_str("MMX5"),
            Register::MMX6 => f.write_str("MMX6"),
            Register::MMX7 => f.write_str("MMX7"),

            Register::XMM0 => f.write_str("XMM0"),
            Register::XMM1 => f.write_str("XMM1"),
            Register::XMM2 => f.write_str("XMM2"),
            Register::XMM3 => f.write_str("XMM3"),
            Register::XMM4 => f.write_str("XMM4"),
            Register::XMM5 => f.write_str("XMM5"),
            Register::XMM6 => f.write_str("XMM6"),
            Register::XMM7 => f.write_str("XMM7"),
            Register::XMM8 => f.write_str("XMM8"),
            Register::XMM9 => f.write_str("XMM9"),
            Register::XMM10 => f.write_str("XMM10"),
            Register::XMM11 => f.write_str("XMM11"),
            Register::XMM12 => f.write_str("XMM12"),
            Register::XMM13 => f.write_str("XMM13"),
            Register::XMM14 => f.write_str("XMM14"),
            Register::XMM15 => f.write_str("XMM15"),

            Register::YMM0 => f.write_str("YMM0"),
            Register::YMM1 => f.write_str("YMM1"),
            Register::YMM2 => f.write_str("YMM2"),
            Register::YMM3 => f.write_str("YMM3"),
            Register::YMM4 => f.write_str("YMM4"),
            Register::YMM5 => f.write_str("YMM5"),
            Register::YMM6 => f.write_str("YMM6"),
            Register::YMM7 => f.write_str("YMM7"),
            Register::YMM8 => f.write_str("YMM8"),
            Register::YMM9 => f.write_str("YMM9"),
            Register::YMM10 => f.write_str("YMM10"),
            Register::YMM11 => f.write_str("YMM11"),
            Register::YMM12 => f.write_str("YMM12"),
            Register::YMM13 => f.write_str("YMM13"),
            Register::YMM14 => f.write_str("YMM14"),
            Register::YMM15 => f.write_str("YMM15"),

            Register::CR0 => f.write_str("CR0"),
            Register::CR1 => f.write_str("CR1"),
            Register::CR2 => f.write_str("CR2"),
            Register::CR3 => f.write_str("CR3"),
            Register::CR4 => f.write_str("CR4"),
            Register::CR5 => f.write_str("CR5"),
            Register::CR6 => f.write_str("CR6"),
            Register::CR7 => f.write_str("CR7"),
            Register::CR8 => f.write_str("CR8"),
            Register::CR9 => f.write_str("CR9"),
            Register::CR10 => f.write_str("CR10"),
            Register::CR11 => f.write_str("CR11"),
            Register::CR12 => f.write_str("CR12"),
            Register::CR13 => f.write_str("CR13"),
            Register::CR14 => f.write_str("CR14"),
            Register::CR15 => f.write_str("CR15"),

            Register::DR0 => f.write_str("DR0"),
            Register::DR1 => f.write_str("DR1"),
            Register::DR2 => f.write_str("DR2"),
            Register::DR3 => f.write_str("DR3"),
            Register::DR4 => f.write_str("DR4"),
            Register::DR5 => f.write_str("DR5"),
            Register::DR6 => f.write_str("DR6"),
            Register::DR7 => f.write_str("DR7"),
            Register::DR8 => f.write_str("DR8"),
            Register::DR9 => f.write_str("DR9"),
            Register::DR10 => f.write_str("DR10"),
            Register::DR11 => f.write_str("DR11"),
            Register::DR12 => f.write_str("DR12"),
            Register::DR13 => f.write_str("DR13"),
            Register::DR14 => f.write_str("DR14"),
            Register::DR15 => f.write_str("DR15"),

            Register::None => f.write_str(""),
        }
    }
}

#[derive(Clone,Debug)]
pub enum AddressingMethod {
    None,
    A, B, C, D, E, F, G, H, I, J, L, M, N, O, P, Q, R, S, U, V, W, X, Y
}

#[derive(Clone,Debug)]
#[allow(non_camel_case_types)]
pub enum OperandType {
    None,
    a, b, c, d, dq, p, pd, pi, ps, q, qq, s, sd, ss, si, v, w, x, y, z,
    RAX, RBX, RCX, RDX, RDI, RSI, RSP, RBP, RIP,
    EAX, EBX, ECX, EDX, EDI, ESI, ESP, EBP, EIP,
    AX, BX, CX, DX, DI, SI, SP, BP, IP,
    AL, BL, CL, DL,
    AH, BH, CH, DH,
    rAX, rBX, rCX, rDX, rDI, rSI, rSP, rBP,
    eAX, eBX, eCX, eDX, eDI, eSI, eSP, eBP,
    ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7,
    ES, GS, DS, SS, CS, FS,
    one,
    NTA, T0, T1, T2,
}

fn read_spec_register(op: OperandType,opsz: usize) -> Result<Operand> {
    match op {
        OperandType::RAX => Ok(Operand::Register(Register::RAX)),
        OperandType::RBX => Ok(Operand::Register(Register::RBX)),
        OperandType::RCX => Ok(Operand::Register(Register::RCX)),
        OperandType::RDX => Ok(Operand::Register(Register::RDX)),
        OperandType::RDI => Ok(Operand::Register(Register::RDI)),
        OperandType::RSI => Ok(Operand::Register(Register::RSI)),
        OperandType::RSP => Ok(Operand::Register(Register::RSP)),
        OperandType::RBP => Ok(Operand::Register(Register::RBP)),
        OperandType::RIP => Ok(Operand::Register(Register::RIP)),

        OperandType::EAX => Ok(Operand::Register(Register::EAX)),
        OperandType::EBX => Ok(Operand::Register(Register::EBX)),
        OperandType::ECX => Ok(Operand::Register(Register::ECX)),
        OperandType::EDX => Ok(Operand::Register(Register::EDX)),
        OperandType::EDI => Ok(Operand::Register(Register::EDI)),
        OperandType::ESI => Ok(Operand::Register(Register::ESI)),
        OperandType::ESP => Ok(Operand::Register(Register::ESP)),
        OperandType::EBP => Ok(Operand::Register(Register::EBP)),
        OperandType::EIP => Ok(Operand::Register(Register::EIP)),

        OperandType::AX => Ok(Operand::Register(Register::AX)),
        OperandType::BX => Ok(Operand::Register(Register::BX)),
        OperandType::CX => Ok(Operand::Register(Register::CX)),
        OperandType::DX => Ok(Operand::Register(Register::DX)),
        OperandType::DI => Ok(Operand::Register(Register::DI)),
        OperandType::SI => Ok(Operand::Register(Register::SI)),
        OperandType::SP => Ok(Operand::Register(Register::SP)),
        OperandType::BP => Ok(Operand::Register(Register::BP)),
        OperandType::IP => Ok(Operand::Register(Register::IP)),

        OperandType::AL => Ok(Operand::Register(Register::AL)),
        OperandType::BL => Ok(Operand::Register(Register::BL)),
        OperandType::CL => Ok(Operand::Register(Register::CL)),
        OperandType::DL => Ok(Operand::Register(Register::DL)),

        OperandType::AH => Ok(Operand::Register(Register::AH)),
        OperandType::BH => Ok(Operand::Register(Register::BH)),
        OperandType::CH => Ok(Operand::Register(Register::CH)),
        OperandType::DH => Ok(Operand::Register(Register::DH)),

        OperandType::ES => Ok(Operand::Register(Register::ES)),
        OperandType::FS => Ok(Operand::Register(Register::FS)),
        OperandType::GS => Ok(Operand::Register(Register::GS)),
        OperandType::SS => Ok(Operand::Register(Register::SS)),
        OperandType::CS => Ok(Operand::Register(Register::CS)),

        OperandType::ST0 => Ok(Operand::Register(Register::ST0)),
        OperandType::ST1 => Ok(Operand::Register(Register::ST1)),
        OperandType::ST2 => Ok(Operand::Register(Register::ST2)),
        OperandType::ST3 => Ok(Operand::Register(Register::ST3)),
        OperandType::ST4 => Ok(Operand::Register(Register::ST4)),
        OperandType::ST5 => Ok(Operand::Register(Register::ST5)),
        OperandType::ST6 => Ok(Operand::Register(Register::ST6)),
        OperandType::ST7 => Ok(Operand::Register(Register::ST7)),

        OperandType::rAX if opsz == 64 => Ok(Operand::Register(Register::RAX)),
        OperandType::rBX if opsz == 64 => Ok(Operand::Register(Register::RBX)),
        OperandType::rCX if opsz == 64 => Ok(Operand::Register(Register::RCX)),
        OperandType::rDX if opsz == 64 => Ok(Operand::Register(Register::RDX)),
        OperandType::rDI if opsz == 64 => Ok(Operand::Register(Register::RDI)),
        OperandType::rSI if opsz == 64 => Ok(Operand::Register(Register::RSI)),
        OperandType::rSP if opsz == 64 => Ok(Operand::Register(Register::RSP)),
        OperandType::rBP if opsz == 64 => Ok(Operand::Register(Register::RBP)),

        OperandType::rAX | OperandType::eAX if opsz == 32 => Ok(Operand::Register(Register::EAX)),
        OperandType::rBX | OperandType::eBX if opsz == 32 => Ok(Operand::Register(Register::EBX)),
        OperandType::rCX | OperandType::eCX if opsz == 32 => Ok(Operand::Register(Register::ECX)),
        OperandType::rDX | OperandType::eDX if opsz == 32 => Ok(Operand::Register(Register::EDX)),
        OperandType::rDI | OperandType::eDI if opsz == 32 => Ok(Operand::Register(Register::EDI)),
        OperandType::rSI | OperandType::eSI if opsz == 32 => Ok(Operand::Register(Register::ESI)),
        OperandType::rSP | OperandType::eSP if opsz == 32 => Ok(Operand::Register(Register::ESP)),
        OperandType::rBP | OperandType::eBP if opsz == 32 => Ok(Operand::Register(Register::EBP)),

        OperandType::rAX | OperandType::eAX if opsz == 16 => Ok(Operand::Register(Register::AX)),
        OperandType::rBX | OperandType::eBX if opsz == 16 => Ok(Operand::Register(Register::BX)),
        OperandType::rCX | OperandType::eCX if opsz == 16 => Ok(Operand::Register(Register::CX)),
        OperandType::rDX | OperandType::eDX if opsz == 16 => Ok(Operand::Register(Register::DX)),
        OperandType::rDI | OperandType::eDI if opsz == 16 => Ok(Operand::Register(Register::DI)),
        OperandType::rSI | OperandType::eSI if opsz == 16 => Ok(Operand::Register(Register::SI)),
        OperandType::rSP | OperandType::eSP if opsz == 16 => Ok(Operand::Register(Register::SP)),
        OperandType::rBP | OperandType::eBP if opsz == 16 => Ok(Operand::Register(Register::BP)),

        _ => Err("Invalid OperandType value".into()),
    }
}

#[derive(Clone,Debug)]
pub enum OperandSpec {
    None,
    Present(AddressingMethod,OperandType),
}

#[derive(Clone)]
enum Operand {
    Register(Register),
    Immediate(u64,usize), // Value, Width (Bits)
    Indirect(Register,Register,usize,u64,usize), // Base, Index, Scale, Disp, Width (Bits)
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(),Error> {
        match *self {
            Operand::Register(ref name) => f.write_str(&format!("{}",name)),
            Operand::Immediate(ref value,_) => f.write_str(&format!("0x{:x}",value)),
            Operand::Indirect(ref base,ref index,ref scale,ref disp,ref width) => {
                let mut s = format!("{} PTR [",match *width {
                    8 => "BYTE",
                    16 => "WORD",
                    32 => "DWORD",
                    64 => "QWORD",
                    _ => "UNK",
                }.to_string());

                if *base != Register::None {
                    s = format!("{}{}",s,base);
                }

                if *scale > 0 && *index != Register::None {
                    if *base != Register::None {
                        s = format!("{} + ",s);
                    }

                    s = format!("{}{}*{}",s,index,scale);
                }

                if *disp > 0 {
                    if *base != Register::None || (*scale > 0 && *index != Register::None) {
                        s = format!("{} + ",s);
                    }

                    s = format!("{}0x{:x}",s,disp);
                }

                f.write_str(&format!("{}]",s))
            },
        }
    }
}

fn read_operand(spec: &OperandSpec, tail: &mut Tail,
                mode: Mode, vvvv: Option<u8>, rex: Option<(bool,bool,bool,bool)>,
                opsz: usize, addrsz: usize, simdsz: usize, addr: u64) -> Option<Operand> {
    match (spec,opsz) {
        (&OperandSpec::None,_) => None,
        (&OperandSpec::Present(AddressingMethod::None,ref reg),_) =>
            read_spec_register(reg.clone(),opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::v),16) =>
            Some(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::v),32) =>
            Some(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::v),64) =>
            Some(Operand::Immediate(tail.read_u64().ok().unwrap(),64)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::p),16) =>
            Some(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::p),32) => {
            let imm16 = tail.read_u16().ok().unwrap() as u64;
            let imm32 = tail.read_u32().ok().unwrap() as u64;
            Some(Operand::Immediate((imm16 << 32) | imm32,48))
        }
        (&OperandSpec::Present(AddressingMethod::A,OperandType::p),64) => {
            // XXX
            let _ = tail.read_u16().ok().unwrap();
            let imm64 = tail.read_u64().ok().unwrap();
            Some(Operand::Immediate(imm64 as u64,64))
        }
        (&OperandSpec::Present(AddressingMethod::B,OperandType::y),opsz) if vvvv.is_some() =>
            read_register(vvvv.unwrap(),rex.is_some(),cmp::max(32,opsz)).ok(),
        (&OperandSpec::Present(AddressingMethod::C,OperandType::d),_) =>
            read_ctrl_register(tail.modrm(rex).ok().unwrap().1,32).ok(),
        (&OperandSpec::Present(AddressingMethod::D,OperandType::d),_) =>
            read_debug_register(tail.modrm(rex).ok().unwrap().1,32).ok(),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::v),opsz) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::y),opsz) =>
            read_effective_address(mode,tail,rex,cmp::max(32,opsz),addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::b),_) =>
            read_effective_address(mode,tail,rex,8,addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::w),_) =>
            read_effective_address(mode,tail,rex,16,addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::d),_) =>
            read_effective_address(mode,tail,rex,32,addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::dq),_) =>
            read_effective_address(mode,tail,rex,64,addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::dq),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::d),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),32).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::w),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),16).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::b),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),8).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::v),opsz) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::z),opsz) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),cmp::min(32,opsz)).ok(),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::y),opsz) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),cmp::max(32,opsz)).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::x),opsz) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::qq),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),256).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::dq),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::ps),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::pd),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::ss),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::sd),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::H,_),_) if vvvv.is_none() =>
            None,
        (&OperandSpec::Present(AddressingMethod::I,OperandType::z),16) =>
            Some(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::z),_) =>
            Some(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::b),_) =>
            Some(Operand::Immediate(tail.read_u8().ok().unwrap() as u64,opsz)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::one),opsz) =>
            Some(Operand::Immediate(1,opsz)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::w),_) =>
            Some(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),16) =>
            Some(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),32) =>
            Some(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),64) =>
            Some(Operand::Immediate(tail.read_u64().ok().unwrap() as u64,64)),
        (&OperandSpec::Present(AddressingMethod::J,OperandType::b),_) =>
            Some(Operand::Immediate(addr + tail.read_u8().ok().unwrap() as u64,addrsz)),
        (&OperandSpec::Present(AddressingMethod::J,OperandType::z),16) =>
            Some(Operand::Immediate(addr + tail.read_u16().ok().unwrap() as u64,addrsz)),
        (&OperandSpec::Present(AddressingMethod::J,OperandType::z),_) =>
            Some(Operand::Immediate(addr + tail.read_u32().ok().unwrap() as u64,addrsz)),
        (&OperandSpec::Present(AddressingMethod::L,OperandType::x),32) =>
            read_simd_register(tail.read_u8().ok().unwrap() & 0b0111,rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::L,OperandType::x),_) =>
            read_simd_register(tail.read_u8().ok().unwrap() & 0b1111,rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::p),16) =>
            read_effective_address(mode,tail,rex,16,addrsz,addr,32).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::p),32) =>
            read_effective_address(mode,tail,rex,32,addrsz,addr,48).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::p),64) =>
            read_effective_address(mode,tail,rex,64,addrsz,addr,80).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::w),opsz) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,16).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::d),opsz) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,32).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::q),opsz) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,64).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::s),64) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,80).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::s),_) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,48).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::b),_) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,8).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::None),opsz) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::a),32) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,64).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::a),16) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,32).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::y),opsz) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,cmp::min(32,opsz)).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::x),32) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,128).ok(),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::x),64) =>
            read_effective_address(mode,tail,rex,opsz,addrsz,addr,256).ok(),
        (&OperandSpec::Present(AddressingMethod::N,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 16 =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),addrsz,8).ok(),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 32 =>
            read_memory(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,addrsz),addrsz,8).ok(),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 16 =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),addrsz,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 32 =>
            read_memory(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,addrsz),addrsz,opsz).ok(),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::pi),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::ps),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::d),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),32).ok(),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::d),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,32).ok(),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::pi),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::q),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,32).ok(),
        (&OperandSpec::Present(AddressingMethod::S,OperandType::w),_) =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),addrsz,16).ok(),
        (&OperandSpec::Present(AddressingMethod::R,OperandType::d),_) =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),addrsz,32).ok(),
        (&OperandSpec::Present(AddressingMethod::R,OperandType::q),_) =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),addrsz,64).ok(),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::ps),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::pi),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::x),32) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::x),64) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),256).ok(),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::dq),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::pi),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::ps),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::pd),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::ss),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::x),32) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::x),64) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),256).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::dq),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::sd),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128).ok(),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::y),opsz) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),cmp::min(32,opsz)).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::pd),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::ps),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,simdsz).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::q),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,64).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::dq),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,128).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::x),32) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,128).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::x),64) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,256).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::sd),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,128).ok(),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::ss),_) =>
            read_effective_simd_address(mode,tail,rex,opsz,addrsz,addr,128).ok(),
        _ => {
            println!("can't decode {:?}/{}",spec,opsz);
            unreachable!();
        }
    }
}


fn read_effective_simd_address(mode: Mode, tail: &mut Tail,
                               rex: Option<(bool,bool,bool,bool)>,
                               opsz: usize, addrsz: usize, ip: u64, simdsz: usize) -> Result<Operand> {
    let (mod_,reg,rm) = try!(tail.modrm(rex));

    match (mod_,rm & 0b111) {
        // mod = 00
        (0b00,0b000) | (0b00,0b001) | (0b00,0b010) |
        (0b00,0b011) | (0b00,0b110) | (0b00,0b111) =>
            read_memory(try!(read_register(rm,rex.is_some(),opsz)),addrsz,simdsz),
        (0b00,0b100) => {
            let val = try!(tail.sib(mod_,rex,opsz,opsz));
            read_memory(val,addrsz,simdsz)
        }
        (0b00,0b101) if mode == Mode::Long =>
            Ok(Operand::Indirect(Register::RIP,Register::None,0,try!(tail.read_u32()) as u64,opsz)),
        (0b00,0b101) if mode == Mode::Long =>
            Ok(Operand::Indirect(Register::None,Register::None,0,try!(tail.read_u32()) as u64,opsz)),

        // mod = 01
        (0b01,0b000) | (0b01,0b001) | (0b01,0b010) | (0b01,0b011) |
        (0b01,0b101) | (0b01,0b110) | (0b01,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),opsz) {
                Ok(Operand::Indirect(reg,Register::None,0,try!(tail.read_u8()) as u64,simdsz))
            } else {
                Err("Failed to decode SIB byte".into())
            },
        (0b01,0b100) =>
            if let Operand::Indirect(b,i,s,_,w) = try!(tail.sib(mod_,rex,opsz,opsz)) {
                let d = try!(tail.read_u8());
                Ok(Operand::Indirect(b,i,s,d as u64,w))
            } else {
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 10
        (0b10,0b000) | (0b10,0b001) | (0b10,0b010) | (0b10,0b011) |
        (0b10,0b101) | (0b10,0b110) | (0b10,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),opsz) {
                Ok(Operand::Indirect(reg,Register::None,0,try!(tail.read_u32()) as u64,simdsz))
            } else {
                Err("Failed to decode SIB byte".into())
            },
       (0b10,0b100) =>
            if let Operand::Indirect(b,i,s,_,w) = try!(tail.sib(mod_,rex,opsz,opsz)) {
                let d = try!(tail.read_u32());
                Ok(Operand::Indirect(b,i,s,d as u64,w))
            } else {
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 11
        (0b11,_) => read_simd_register(rm,rex.is_some(),simdsz),

        _ => Err("Invalid mod value".into()),
    }
}

fn read_effective_address(mode: Mode, tail: &mut Tail,
                          rex: Option<(bool,bool,bool,bool)>,
                          opsz: usize, addrsz: usize, ip: u64, simdsz: usize) -> Result<Operand> {
    let (mod_,reg,rm) = try!(tail.modrm(rex));

    match (mod_,rm & 0b111) {
        // mod = 00
        (0b00,0b000) | (0b00,0b001) | (0b00,0b010) |
        (0b00,0b011) | (0b00,0b110) | (0b00,0b111) =>
            read_memory(try!(read_register(rm,rex.is_some(),opsz)),addrsz,opsz),
        (0b00,0b100) => {
            let val = try!(tail.sib(mod_,rex,opsz,opsz));
            read_memory(val,addrsz,opsz)
        }
        (0b00,0b101) if mode == Mode::Long =>
            Ok(Operand::Indirect(Register::RIP,Register::None,0,try!(tail.read_u32()) as u64,opsz)),
        (0b00,0b101) if mode == Mode::Long =>
            Ok(Operand::Indirect(Register::None,Register::None,0,try!(tail.read_u32()) as u64,opsz)),

        // mod = 01
        (0b01,0b000) | (0b01,0b001) | (0b01,0b010) | (0b01,0b011) |
        (0b01,0b101) | (0b01,0b110) | (0b01,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),opsz) {
                Ok(Operand::Indirect(reg,Register::None,0,try!(tail.read_u8()) as u64,opsz))
            } else {
                Err("Failed to decode SIB byte".into())
            },
        (0b01,0b100) =>
            if let Operand::Indirect(b,i,s,_,w) = try!(tail.sib(mod_,rex,opsz,opsz)) {
                let d = try!(tail.read_u8());
                Ok(Operand::Indirect(b,i,s,d as u64,w))
            } else {
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 10
        (0b10,0b000) | (0b10,0b001) | (0b10,0b010) | (0b10,0b011) |
        (0b10,0b101) | (0b10,0b110) | (0b10,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),opsz) {
                Ok(Operand::Indirect(reg,Register::None,0,try!(tail.read_u32()) as u64,opsz))
            } else {
                Err("Failed to decode SIB byte".into())
            },
        (0b10,0b100) =>
            if let Operand::Indirect(b,i,s,_,w) = try!(tail.sib(mod_,rex,opsz,opsz)) {
                let d = try!(tail.read_u32());
                Ok(Operand::Indirect(b,i,s,d as u64,w))
            } else {
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 11
        (0b11,_) => read_register(rm,rex.is_some(),opsz),

        _ => Err("Invalid mod value".into()),
    }
}

fn read_memory(op: Operand, addrsz: usize, width: usize) -> Result<Operand> {
    match op {
        Operand::Register(reg) => Ok(Operand::Indirect(reg,Register::None,0,0,width)),
        Operand::Immediate(imm,_) => Ok(Operand::Indirect(Register::None,Register::None,0,imm,width)),
        Operand::Indirect(_,_,_,_,_) => Err("Tried to contruct doubly indirect operand".into()),
    }
}

fn read_sib<R: ReadBytesExt>(fd: &mut R, mod_: u8, rex: Option<(bool,bool,bool,bool)>,
            opsz: usize,width: usize) -> Result<Operand> {
    let sib = try!(fd.read_u8());
    let scale = sib >> 6;
    let index = (sib >> 3) & 0b111;
    let base = sib & 0b111;

    let ret_scale = 1 << scale;
    let (ret_base,ret_disp) = if mod_ != 0b11 && base == 0b101 {
        match mod_ {
            0b00 => (Register::None,try!(fd.read_u32::<LittleEndian>()) as u64),
            0b01 => (Register::EBP,0),
            0b10 => (Register::EBP,0),
            _ => return Err("Internal error".into()),
        }
    } else {
        if let Ok(Operand::Register(r)) = read_register(base,rex.is_some(),opsz) {
            (r,0)
        } else {
            return Err("Failed to decode base register".into());
        }
    };
    let ret_index = if index == 0b100 {
        Register::None
    } else {
        if let Ok(Operand::Register(r)) = read_register(index,rex.is_some(),opsz) {
            r
        } else {
            return Err("Failed to decode index register".into());
        }
    };

    // disp handled by calling function
    Ok(Operand::Indirect(ret_base,ret_index,ret_scale,ret_disp,width))
}

fn read_modrm<R: ReadBytesExt>(fd: &mut R,rex: Option<(bool,bool,bool,bool)>) -> Result<(u8,u8,u8)> {
    let modrm = try!(fd.read_u8());
    let mod_ = modrm >> 6;
    let mut reg = (modrm >> 3) & 0b111;
    let mut rm = modrm & 0b111;
    let sib_present = mod_ != 0b11 && rm == 0b100;

    if let Some((b,x,r,w)) = rex {
        if b && !sib_present { rm |= 0b1000 }
        if r { reg |= 0b1000 }
    }

    trace!("read modrm {:x} ({:b},{:b},{:b})",modrm,mod_,reg,rm);
    Ok((mod_,reg,rm))
}

fn read_register(reg: u8, rex_present: bool, opsz: usize) -> Result<Operand> {
    match (reg,opsz) {
        (0b0000,8) => Ok(Operand::Register(Register::AL)),
        (0b0001,8) => Ok(Operand::Register(Register::CL)),
        (0b0010,8) => Ok(Operand::Register(Register::DL)),
        (0b0011,8) => Ok(Operand::Register(Register::BL)),
        (0b0100,8) => if rex_present { Ok(Operand::Register(Register::AH)) } else { Ok(Operand::Register(Register::SPL)) },
        (0b0101,8) => if rex_present { Ok(Operand::Register(Register::CH)) } else { Ok(Operand::Register(Register::BPL)) },
        (0b0110,8) => if rex_present { Ok(Operand::Register(Register::DH)) } else { Ok(Operand::Register(Register::SIL)) },
        (0b0111,8) => if rex_present { Ok(Operand::Register(Register::BH)) } else { Ok(Operand::Register(Register::DIL)) },
        (0b1000,8) => Ok(Operand::Register(Register::R8L)),
        (0b1001,8) => Ok(Operand::Register(Register::R9L)),
        (0b1010,8) => Ok(Operand::Register(Register::R10L)),
        (0b1011,8) => Ok(Operand::Register(Register::R11L)),
        (0b1100,8) => Ok(Operand::Register(Register::R12L)),
        (0b1101,8) => Ok(Operand::Register(Register::R13L)),
        (0b1110,8) => Ok(Operand::Register(Register::R14L)),
        (0b1111,8) => Ok(Operand::Register(Register::R15L)),

        (0b0000,16) => Ok(Operand::Register(Register::AX)),
        (0b0001,16) => Ok(Operand::Register(Register::CX)),
        (0b0010,16) => Ok(Operand::Register(Register::DX)),
        (0b0011,16) => Ok(Operand::Register(Register::BX)),
        (0b0100,16) => Ok(Operand::Register(Register::SP)),
        (0b0101,16) => Ok(Operand::Register(Register::BP)),
        (0b0110,16) => Ok(Operand::Register(Register::SI)),
        (0b0111,16) => Ok(Operand::Register(Register::DI)),
        (0b1000,16) => Ok(Operand::Register(Register::R8W)),
        (0b1001,16) => Ok(Operand::Register(Register::R9W)),
        (0b1010,16) => Ok(Operand::Register(Register::R10W)),
        (0b1011,16) => Ok(Operand::Register(Register::R11W)),
        (0b1100,16) => Ok(Operand::Register(Register::R12W)),
        (0b1101,16) => Ok(Operand::Register(Register::R13W)),
        (0b1110,16) => Ok(Operand::Register(Register::R14W)),
        (0b1111,16) => Ok(Operand::Register(Register::R15W)),

        (0b0000,32) => Ok(Operand::Register(Register::EAX)),
        (0b0001,32) => Ok(Operand::Register(Register::ECX)),
        (0b0010,32) => Ok(Operand::Register(Register::EDX)),
        (0b0011,32) => Ok(Operand::Register(Register::EBX)),
        (0b0100,32) => Ok(Operand::Register(Register::ESP)),
        (0b0101,32) => Ok(Operand::Register(Register::EBP)),
        (0b0110,32) => Ok(Operand::Register(Register::ESI)),
        (0b0111,32) => Ok(Operand::Register(Register::EDI)),
        (0b1000,32) => Ok(Operand::Register(Register::R8D)),
        (0b1001,32) => Ok(Operand::Register(Register::R9D)),
        (0b1010,32) => Ok(Operand::Register(Register::R10D)),
        (0b1011,32) => Ok(Operand::Register(Register::R11D)),
        (0b1100,32) => Ok(Operand::Register(Register::R12D)),
        (0b1101,32) => Ok(Operand::Register(Register::R13D)),
        (0b1110,32) => Ok(Operand::Register(Register::R14D)),
        (0b1111,32) => Ok(Operand::Register(Register::R15D)),

        (0b0000,64) => Ok(Operand::Register(Register::RAX)),
        (0b0001,64) => Ok(Operand::Register(Register::RCX)),
        (0b0010,64) => Ok(Operand::Register(Register::RDX)),
        (0b0011,64) => Ok(Operand::Register(Register::RBX)),
        (0b0100,64) => Ok(Operand::Register(Register::RSP)),
        (0b0101,64) => Ok(Operand::Register(Register::RBP)),
        (0b0110,64) => Ok(Operand::Register(Register::RSI)),
        (0b0111,64) => Ok(Operand::Register(Register::RDI)),
        (0b1000,64) => Ok(Operand::Register(Register::R8)),
        (0b1001,64) => Ok(Operand::Register(Register::R9)),
        (0b1010,64) => Ok(Operand::Register(Register::R10)),
        (0b1011,64) => Ok(Operand::Register(Register::R11)),
        (0b1100,64) => Ok(Operand::Register(Register::R12)),
        (0b1101,64) => Ok(Operand::Register(Register::R13)),
        (0b1110,64) => Ok(Operand::Register(Register::R14)),
        (0b1111,64) => Ok(Operand::Register(Register::R15)),

        (0b0000,80) => Ok(Operand::Register(Register::ST0)),
        (0b0001,80) => Ok(Operand::Register(Register::ST1)),
        (0b0010,80) => Ok(Operand::Register(Register::ST2)),
        (0b0011,80) => Ok(Operand::Register(Register::ST3)),
        (0b0100,80) => Ok(Operand::Register(Register::ST4)),
        (0b0101,80) => Ok(Operand::Register(Register::ST5)),
        (0b0110,80) => Ok(Operand::Register(Register::ST6)),
        (0b0111,80) => Ok(Operand::Register(Register::ST7)),

        _ => Err("Invalid reg value".into()),
    }
}
fn read_simd_register(reg: u8, rex_present: bool, opsz: usize) -> Result<Operand> {
    match (reg,opsz) {
       (0b0000,64) => Ok(Operand::Register(Register::MMX0)),
       (0b0001,64) => Ok(Operand::Register(Register::MMX1)),
       (0b0010,64) => Ok(Operand::Register(Register::MMX2)),
       (0b0011,64) => Ok(Operand::Register(Register::MMX3)),
       (0b0100,64) => Ok(Operand::Register(Register::MMX4)),
       (0b0101,64) => Ok(Operand::Register(Register::MMX5)),
       (0b0110,64) => Ok(Operand::Register(Register::MMX6)),
       (0b0111,64) => Ok(Operand::Register(Register::MMX7)),
       (0b1000,64) => Ok(Operand::Register(Register::MMX0)),
       (0b1001,64) => Ok(Operand::Register(Register::MMX1)),
       (0b1010,64) => Ok(Operand::Register(Register::MMX2)),
       (0b1011,64) => Ok(Operand::Register(Register::MMX3)),
       (0b1100,64) => Ok(Operand::Register(Register::MMX4)),
       (0b1101,64) => Ok(Operand::Register(Register::MMX5)),
       (0b1110,64) => Ok(Operand::Register(Register::MMX6)),
       (0b1111,64) => Ok(Operand::Register(Register::MMX7)),

       (0b0000,128) => Ok(Operand::Register(Register::XMM0)),
       (0b0001,128) => Ok(Operand::Register(Register::XMM1)),
       (0b0010,128) => Ok(Operand::Register(Register::XMM2)),
       (0b0011,128) => Ok(Operand::Register(Register::XMM3)),
       (0b0100,128) => Ok(Operand::Register(Register::XMM4)),
       (0b0101,128) => Ok(Operand::Register(Register::XMM5)),
       (0b0110,128) => Ok(Operand::Register(Register::XMM6)),
       (0b0111,128) => Ok(Operand::Register(Register::XMM7)),
       (0b1000,128) => Ok(Operand::Register(Register::XMM8)),
       (0b1001,128) => Ok(Operand::Register(Register::XMM9)),
       (0b1010,128) => Ok(Operand::Register(Register::XMM10)),
       (0b1011,128) => Ok(Operand::Register(Register::XMM11)),
       (0b1100,128) => Ok(Operand::Register(Register::XMM12)),
       (0b1101,128) => Ok(Operand::Register(Register::XMM13)),
       (0b1110,128) => Ok(Operand::Register(Register::XMM14)),
       (0b1111,128) => Ok(Operand::Register(Register::XMM15)),

       (0b0000,256) => Ok(Operand::Register(Register::YMM0)),
       (0b0001,256) => Ok(Operand::Register(Register::YMM1)),
       (0b0010,256) => Ok(Operand::Register(Register::YMM2)),
       (0b0011,256) => Ok(Operand::Register(Register::YMM3)),
       (0b0100,256) => Ok(Operand::Register(Register::YMM4)),
       (0b0101,256) => Ok(Operand::Register(Register::YMM5)),
       (0b0110,256) => Ok(Operand::Register(Register::YMM6)),
       (0b0111,256) => Ok(Operand::Register(Register::YMM7)),
       (0b1000,256) => Ok(Operand::Register(Register::YMM8)),
       (0b1001,256) => Ok(Operand::Register(Register::YMM9)),
       (0b1010,256) => Ok(Operand::Register(Register::YMM10)),
       (0b1011,256) => Ok(Operand::Register(Register::YMM11)),
       (0b1100,256) => Ok(Operand::Register(Register::YMM12)),
       (0b1101,256) => Ok(Operand::Register(Register::YMM13)),
       (0b1110,256) => Ok(Operand::Register(Register::YMM14)),
       (0b1111,256) => Ok(Operand::Register(Register::YMM15)),

        _ => Err("Invalid reg value".into()),
    }
}

fn read_ctrl_register(reg: u8, opsz: usize) -> Result<Operand> {
    match (reg,opsz) {
       (0b0000,32) => Ok(Operand::Register(Register::CR0)),
       (0b0001,32) => Ok(Operand::Register(Register::CR1)),
       (0b0010,32) => Ok(Operand::Register(Register::CR2)),
       (0b0011,32) => Ok(Operand::Register(Register::CR3)),
       (0b0100,32) => Ok(Operand::Register(Register::CR4)),
       (0b0101,32) => Ok(Operand::Register(Register::CR5)),
       (0b0110,32) => Ok(Operand::Register(Register::CR6)),
       (0b0111,32) => Ok(Operand::Register(Register::CR7)),
       (0b1000,32) => Ok(Operand::Register(Register::CR8)),
       (0b1001,32) => Ok(Operand::Register(Register::CR9)),
       (0b1010,32) => Ok(Operand::Register(Register::CR10)),
       (0b1011,32) => Ok(Operand::Register(Register::CR11)),
       (0b1100,32) => Ok(Operand::Register(Register::CR12)),
       (0b1101,32) => Ok(Operand::Register(Register::CR13)),
       (0b1110,32) => Ok(Operand::Register(Register::CR14)),
       (0b1111,32) => Ok(Operand::Register(Register::CR15)),

        _ => Err("Invalid reg value".into()),
    }
}

fn read_debug_register(reg: u8, opsz: usize) -> Result<Operand> {
    match (reg,opsz) {
       (0b0000,32) => Ok(Operand::Register(Register::DR0)),
       (0b0001,32) => Ok(Operand::Register(Register::DR1)),
       (0b0010,32) => Ok(Operand::Register(Register::DR2)),
       (0b0011,32) => Ok(Operand::Register(Register::DR3)),
       (0b0100,32) => Ok(Operand::Register(Register::DR4)),
       (0b0101,32) => Ok(Operand::Register(Register::DR5)),
       (0b0110,32) => Ok(Operand::Register(Register::DR6)),
       (0b0111,32) => Ok(Operand::Register(Register::DR7)),
       (0b1000,32) => Ok(Operand::Register(Register::DR8)),
       (0b1001,32) => Ok(Operand::Register(Register::DR9)),
       (0b1010,32) => Ok(Operand::Register(Register::DR10)),
       (0b1011,32) => Ok(Operand::Register(Register::DR11)),
       (0b1100,32) => Ok(Operand::Register(Register::DR12)),
       (0b1101,32) => Ok(Operand::Register(Register::DR13)),
       (0b1110,32) => Ok(Operand::Register(Register::DR14)),
       (0b1111,32) => Ok(Operand::Register(Register::DR15)),

        _ => Err("Invalid reg value".into()),
    }
}

#[derive(Clone,Debug)]
pub enum OpcodeOption {
    None,
    Default64,
    Force64,
    Only64,
    Invalid64,
}

#[derive(Clone,Debug)]
pub enum Mnemonic {
    Undefined,
    Escape,
    Single(&'static str),
    ModRM(isize),
}

#[derive(Clone,Debug)]
pub struct Opcode {
    mnemonic: Mnemonic,
    operand_a: OperandSpec,
    operand_b: OperandSpec,
    operand_c: OperandSpec,
    operand_d: OperandSpec,
    option: OpcodeOption,
}

impl Default for Prefix {
    fn default() -> Self {
        Prefix{
            lock: false,
            repe: false,
            repne: false,
            seg_override: SegmentOverride::None,
            branch_hint: BranchHint::None,
            operand_size: 0,
            address_size: 0,
            simd_prefix: SimdPrefix::None,
            opcode_escape: OpcodeEscape::None,
            vvvv: None,
            rex_r: false,
            rex_b: false,
            rex_x: false,
            rex_w: false,
        }
    }
}

struct Tail<'a> {
    fd: Cursor<&'a [u8]>,
    modrm: Option<(u8,u8,u8)>,
    sib: Option<Operand>,
}

impl<'a> Tail<'a> {
    pub fn new(cur: Cursor<&'a [u8]>) -> Tail<'a> {
        Tail{
            fd: cur,
            modrm: None,
            sib: None,
        }
    }

    pub fn modrm(&mut self,rex: Option<(bool,bool,bool,bool)>) -> Result<(u8,u8,u8)> {
        if self.modrm.is_none() {
            self.modrm = Some(try!(read_modrm(&mut self.fd,rex)));
        }
        Ok(self.modrm.unwrap())
    }

    pub fn sib(&mut self,mod_: u8,rex: Option<(bool,bool,bool,bool)>,
               opsz: usize, width: usize) -> Result<Operand> {
        if self.sib.is_none() {
            self.sib = Some(try!(read_sib(&mut self.fd,mod_,rex,opsz,width)));
        }
        Ok(self.sib.clone().unwrap())
    }

    pub fn position(&self) -> usize {
        self.fd.position() as usize
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(try!(self.fd.read_u8()))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(try!(self.fd.read_u16::<LittleEndian>()))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(try!(self.fd.read_u32::<LittleEndian>()))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(try!(self.fd.read_u64::<LittleEndian>()))
    }
}

fn select_opcode_ext(grp: isize, opc: usize, modrm: usize, pfx: SimdPrefix,mode: Mode,vexxop_present: bool) -> Result<Opcode> {
    use amd64::tables::*;

    let reg = (modrm & 0b00111000) >> 3;
    let mo = (modrm & 0b11000000) >> 6;
    let rm = modrm & 0b00000111;

    Ok(match (grp,mo,pfx,opc) {
        // GROUP1
        (1,_,SimdPrefix::None,0x80) => GROUP1_OPC80[reg].clone(),
        (1,_,SimdPrefix::None,0x81) => GROUP1_OPC81[reg].clone(),
        (1,_,SimdPrefix::None,0x82) => GROUP1_OPC82[reg].clone(),
        (1,_,SimdPrefix::None,0x83) => GROUP1_OPC83[reg].clone(),

        // GROUP1A
        (101,_,SimdPrefix::None,0x8f) => GROUP101_OPC8F[reg].clone(),

        // GROUP2
        (2,_,SimdPrefix::None,0xc0) => GROUP2_OPCC0[reg].clone(),
        (2,_,SimdPrefix::None,0xc1) => GROUP2_OPCC1[reg].clone(),
        (2,_,SimdPrefix::None,0xd0) => GROUP2_OPCD0[reg].clone(),
        (2,_,SimdPrefix::None,0xd1) => GROUP2_OPCD1[reg].clone(),
        (2,_,SimdPrefix::None,0xd2) => GROUP2_OPCD2[reg].clone(),
        (2,_,SimdPrefix::None,0xd3) => GROUP2_OPCD3[reg].clone(),

        // GROUP3
        (3,_,SimdPrefix::None,0xf6) => GROUP3_OPCF6[reg].clone(),
        (3,_,SimdPrefix::None,0xf7) => GROUP3_OPCF7[reg].clone(),

        // GROUP4
        (4,_,SimdPrefix::None,0xfe) => GROUP4_OPCFE[reg].clone(),

        // GROUP5
        (5,_,SimdPrefix::None,0xff) => GROUP5_OPCFF[reg].clone(),

        // GROUP6
        (6,_,SimdPrefix::None,0x00) => GROUP6_OPC00[reg].clone(),

        // GROUP7
        (7,0b11,SimdPrefix::None,0x01) => match (reg,rm) {
            (0b000,0b001) => opcode!("vmcall"; ),
            (0b000,0b010) => opcode!("vmlaunch"; ),
            (0b000,0b011) => opcode!("vmresume"; ),
            (0b000,0b100) => opcode!("vmxoff"; ),
            (0b000,_) => unused!(),
            (0b001,0b000) => opcode!("monitor"; ),
            (0b001,0b001) => opcode!("mwait"; ),
            (0b001,0b010) => opcode!("clac"; ),
            (0b001,0b011) => opcode!("stac"; ),
            (0b001,0b111) => opcode!("encls"; ),
            (0b001,_) => unused!(),
            (0b010,0b000) => opcode!("xgetbv"; E/v),
            (0b010,0b001) => opcode!("xsetbv"; ),
            (0b010,0b100) => opcode!("vmfunc"; ),
            (0b010,0b101) => opcode!("xend"; ),
            (0b010,0b110) => opcode!("xtest"; ),
            (0b010,0b111) => opcode!("enclu"; ),
            (0b010,_) => unused!(),
            (0b011,_) => unused!(),
            (0b100,_) => GROUP7_OPC01_MEM[reg].clone(),
            (0b101,_) => unused!(),
            (0b110,_) => GROUP7_OPC01_MEM[reg].clone(),
            (0b111,0b000) => opcode!("swapgs"; ),
            (0b111,0b001) => opcode!("rdtscp"; ),
            (0b111,_) => unused!(),
            (_,_) => unreachable!(),
        },
        (7,_,SimdPrefix::None,0x01) => GROUP7_OPC01_MEM[reg].clone(),

        // GROUP8
        (8,_,SimdPrefix::None,0xba) => GROUP8_OPCBA[reg].clone(),

        // GROUP9
        (9,_,SimdPrefix::None,0xc7) => match (mo,pfx,reg) {
            (0b11,SimdPrefix::None,0b110) => opcode!("rdrand"; R/v),
            (0b11,SimdPrefix::None,0b111) => opcode!("rdseed"; R/v),
            (0b11,SimdPrefix::PrefixF3,0b111) => opcode!("rdpid"; R/dq),
            (_,SimdPrefix::None,0b001,) => {
                if mode == Mode::Long {
                    opcode!("cmpxch8b"; M/q)
                } else {
                    opcode!("cmpxchg16b"; M/dq)
                }
            }
            (_,SimdPrefix::None,0b110) => opcode!("vmptrld"; M/q),
            (_,SimdPrefix::None,0b111) => opcode!("vmptrst"; M/q),
            (_,SimdPrefix::Prefix66,0b110) => opcode!("vmclear"; M/q),
            (_,SimdPrefix::PrefixF3,0b110) => opcode!("vmxon"; M/q),
            _ => unused!(),
        },

        // GROUP10
        (10,_,SimdPrefix::None,0xb9) => GROUP10_OPCB9[reg].clone(),

        // GROUP11
        (11,_,SimdPrefix::None,0xc6) => match (mo,reg,rm) {
            (0b11,0b111,0b000) => opcode!("xabort"; I/b),
            (_,_,_) => GROUP11_OPCC6[reg].clone(),
        },
        (11,_,SimdPrefix::None,0xc7) => match (mo,reg,rm) {
            (0b11,0b111,0b000) => opcode!("xbegin"; I/b),
            (_,_,_) => GROUP11_OPCC7[reg].clone(),
        },

        // GROUP12
        (12,0b11,SimdPrefix::None,0x71) => GROUP12_OPC71[reg].clone(),
        (12,0b11,SimdPrefix::Prefix66,0x71) => GROUP12_OPC6671[reg].clone(),

        // GROUP13
        (13,0b11,SimdPrefix::None,0x72) => GROUP13_OPC72[reg].clone(),
        (13,0b11,SimdPrefix::Prefix66,0x72) => GROUP13_OPC6672[reg].clone(),

        // GROUP14
        (14,0b11,SimdPrefix::None,0x73) => GROUP14_OPC73[reg].clone(),
        (14,0b11,SimdPrefix::Prefix66,0x73) => GROUP14_OPC6673[reg].clone(),

        // GROUP15
        (15,0b11,SimdPrefix::None,0xae) => match reg {
            0b101 => opcode!("lfence"; ),
            0b110 => opcode!("mfence"; ),
            0b111 => opcode!("sfence"; ),
            _ => unused!(),
        },
        (15,0b11,SimdPrefix::PrefixF3,0xae) => match reg {
            0b000 => opcode!("rdfsbase"; R/y),
            0b001 => opcode!("rdgsbase"; R/y),
            0b010 => opcode!("wrfsbase"; R/y),
            0b011 => opcode!("wrgsbase"; R/y),
            _ => unused!(),
        },
        (15,_,SimdPrefix::None,0xae) => match reg {
            0b000 => opcode!("fxsave"; ),
            0b001 => opcode!("fxstor"; ),
            0b010 => opcode!("ldmxcsr"; ),
            0b011 => opcode!("stmxcsr"; ),
            0b100 => opcode!("xsave"; ),
            0b101 => opcode!("xrstor"; ),
            0b110 => opcode!("xsaveopt"; ),
            0b111 => opcode!("clflush"; ),
            _ => unreachable!(),
        },

        // GROUP16
        (16,0b11,SimdPrefix::None,0x18) => unused!(),
        (16,_,SimdPrefix::None,0x18) => match reg {
            0b000 => opcode!("prefetch"; M/None),
            0b001 => opcode!("prefetch"; M/None),
            0b010 => opcode!("prefetch"; M/None),
            0b011 => opcode!("prefetch"; M/None),
            _ => unused!(),
        },

        // GROUP17
        (17,_,_,0xf3) if vexxop_present => match reg {
            0b001 => opcode!("blsr"; B/y, E/y),
            0b010 => opcode!("blsmsk"; B/y, E/y),
            0b011 => opcode!("blsi"; B/y, E/y),
            _ => unused!(),
        },

        // GROUPX
        (102,_,SimdPrefix::None,0x01) => GROUP102_OPC01[reg].clone(),

        _ => return Err("Unknown instruction".into()),
    })
}

pub fn read(mode: Mode, buf: &[u8]) -> Result<usize> {
    let mut i = 0;
    let mut prefix = Prefix::default();
    let mut vexxop_present = false;
    let mut rex_present = false;

    match mode {
        Mode::Real => {
            prefix.address_size = 16;
            prefix.operand_size = 16;
        }
        Mode::Protected => {
            prefix.address_size = 32;
            prefix.operand_size = 32;
        }
        Mode::Long => {
            prefix.address_size = 64;
            prefix.operand_size = 32;
        }
    }

    while i < 15 {
        match buf[i] {
            // Group 1: LOCK, REPE, REPNE
            0xf0 | 0xf2 | 0xf3 =>
                if prefix.lock || prefix.repe || prefix.repne {
                    error!("Multiple group 1 prefixes");
                } else {
                    match buf[i] {
                        0xf0 => prefix.lock = true,
                        0xf2 => {
                            prefix.repne = true;
                            if i == 0 { prefix.simd_prefix = SimdPrefix::PrefixF2; }
                        }
                        0xf3 => {
                            prefix.repe = true;
                            if i == 0 { prefix.simd_prefix = SimdPrefix::PrefixF3; }
                        }
                        _ => unreachable!()
                    }
                },
            // Group 2: Segment overrides and branch hints
            0x2e | 0x36 | 0x3e | 0x26 | 0x64 | 0x65 =>
                if prefix.seg_override != SegmentOverride::None {
                    error!("Multiple group 2 prefixes");
                } else {
                    match buf[i] {
                        0x2e => {
                            prefix.seg_override = SegmentOverride::Cs;
                            prefix.branch_hint = BranchHint::NotTaken;
                        }
                        0x36 => prefix.seg_override = SegmentOverride::Ss,
                        0x3e => {
                            prefix.seg_override = SegmentOverride::Ds;
                            prefix.branch_hint = BranchHint::Taken;
                        }
                        0x26 => prefix.seg_override = SegmentOverride::Es,
                        0x64 => prefix.seg_override = SegmentOverride::Fs,
                        0x65 => prefix.seg_override = SegmentOverride::Gs,
                        _ => unreachable!(),
                    }
                },
            // Group 3: OperandSpec size override and mandatory prefix
            0x66 => {
                match mode {
                    Mode::Long | Mode::Protected => prefix.operand_size = 16,
                    Mode::Real => prefix.operand_size = 32,
                }
                if i == 0 { prefix.simd_prefix = SimdPrefix::Prefix66; }
            }
            // Group 4: Address size override
            0x67 =>
                if prefix.address_size == 1 {
                    error!("Multiple address-size override prefixes");
                } else {
                    prefix.address_size = 1;
                },
            // 2 byte VEX
            0xc5 if i == 0 && mode == Mode::Long => {
                let vex = buf[i + 1];

                prefix.simd_prefix = match vex & 0b00000011 {
                    0 => SimdPrefix::None,
                    1 => SimdPrefix::Prefix66,
                    2 => SimdPrefix::PrefixF3,
                    3 => SimdPrefix::PrefixF2,
                    _ => unreachable!(),
                };

                prefix.opcode_escape = OpcodeEscape::Escape0F;

                prefix.vvvv = Some(0xFF ^ ((vex >> 3) & 0b1111));
                prefix.rex_r = vex & 0b1000000 == 0;

                vexxop_present = true;
                rex_present = true;
                i += 2;

                break;
            }

            // 3 byte VEX
            0xc4 if i == 0 && mode == Mode::Long => {
                let vex1 = buf[i + 1];
                let vex2 = buf[i + 2];

                prefix.simd_prefix = match vex2 & 0b00000011 {
                    0 => SimdPrefix::None,
                    1 => SimdPrefix::Prefix66,
                    2 => SimdPrefix::PrefixF3,
                    3 => SimdPrefix::PrefixF2,
                    _ => unreachable!(),
                };

                prefix.opcode_escape = match vex1 & 0b00001111 {
                    1 => OpcodeEscape::Escape0F,
                    2 => OpcodeEscape::Escape0F38,
                    3 => OpcodeEscape::Escape0F3A,
                    _ => return Err("Unknown instruction".into())
                };

                prefix.vvvv = Some(0xFF ^ ((vex2 >> 3) & 0b1111));
                prefix.rex_r = vex1 & 0b1000000 == 0;
                prefix.rex_x = vex1 & 0b0100000 == 0;
                prefix.rex_b = vex1 & 0b0010000 == 0;
                prefix.rex_w = vex2 & 0b1000000 == 0;

                vexxop_present = true;
                rex_present = true;
                i += 3;

                break;
            }

            // EVEX
            0x62 if i == 0 && mode == Mode::Long => {
                unimplemented!()
            },

            // XOP
            0x8f if i == 0 && mode == Mode::Long => {
                let xop1 = buf[i + 1];
                let xop2 = buf[i + 2];

                prefix.simd_prefix = match xop2 & 0b00000011 {
                    0 => SimdPrefix::None,
                    1 => SimdPrefix::Prefix66,
                    2 => SimdPrefix::PrefixF3,
                    3 => SimdPrefix::PrefixF2,
                    _ => unreachable!(),
                };

                prefix.opcode_escape = match xop1 & 0b00001111 {
                    0b1000 => OpcodeEscape::Xop8,
                    0b1001 => OpcodeEscape::Xop9,
                    0b1010 => OpcodeEscape::XopA,
                    _ => return Err("Unknown instruction".into())
                };

                vexxop_present = true;
                i += 3;

                break;
            }

            // REX
            0x40...0x4f if mode == Mode::Long => {
                let rex = buf[i];

                prefix.rex_w = rex & 0b00001000 != 0;
                prefix.rex_r = rex & 0b00000100 != 0;
                prefix.rex_x = rex & 0b00000010 != 0;
                prefix.rex_b = rex & 0b00000001 != 0;

                if prefix.rex_w && prefix.operand_size == 32 {
                    prefix.operand_size = 64;
                }

                rex_present = true;
                i += 1;

                break;
            }

            _ => break
        }

        i += 1;
    }

    // Opcode escape
    if !vexxop_present && buf[i] == 0x0f {
        match buf[i + 1] {
            0x38 => {
                prefix.opcode_escape = OpcodeEscape::Escape0F38;
                i += 2;
            }
            0x3A => {
                prefix.opcode_escape = OpcodeEscape::Escape0F3A;
                i += 2;
            }
            0x0f => {
                prefix.opcode_escape = OpcodeEscape::Escape0F0F;
                i += 2;
            }
            _ => {
                prefix.opcode_escape = OpcodeEscape::Escape0F;
                i += 1;
            }
        }
    }

    use amd64::tables::*;

    trace!("prefix: {:?}, opcode: 0x{:x}",prefix,buf[i]);

    if prefix.opcode_escape == OpcodeEscape::Escape0F0F {
        // XXX: 3DNow!
        return Err("Unknown instruction".into())
    } else {
        // remove non-mandatory prefixes
        let rm_pfx = match prefix.opcode_escape {
            OpcodeEscape::Escape0F => match buf[i] {
                0x00...0x07 => true,
                0x20...0x27 => true,
                0x30...0x37 => true,
                0x40...0x47 => true,
                0x08...0x0f => true,
                0x18...0x1f => true,
                0x38...0x3f => true,
                0x48...0x4f => true,
                0x80...0x87 => true,
                0x90...0x97 => true,
                0xa0...0xa7 => true,
                0xb0...0xb7 => true,
                0x88...0x8f => true,
                0x98...0x9f => true,
                0xa8...0xaf => true,
                0xb8...0xbf if prefix.simd_prefix != SimdPrefix::PrefixF3 => true,
                0xc0 | 0xc1 => true,
                0xc8...0xcf => true,
                _ => false,
            },
            _ => false,
        };

        if rm_pfx {
            prefix.simd_prefix = SimdPrefix::None;
        }

        trace!("tbl lookup: ({:?},{:?})",prefix.opcode_escape,prefix.simd_prefix);

        let opc = match (prefix.opcode_escape,prefix.simd_prefix) {
            (OpcodeEscape::None,_) if buf[i] == 0x63 =>
                Opcode{
                    mnemonic: Mnemonic::Single("movsxd"),
                    operand_a: OperandSpec::Present(AddressingMethod::G,OperandType::v),
                    operand_b: OperandSpec::Present(AddressingMethod::E,OperandType::v),
                    operand_c: OperandSpec::None,
                    operand_d: OperandSpec::None,
                    option: OpcodeOption::Only64,
                },
            (OpcodeEscape::None,_) => ONEBYTE_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F,SimdPrefix::None) => TWOBYTE_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F,SimdPrefix::Prefix66) => {
                prefix.operand_size = match mode {
                    Mode::Real => 16,
                    Mode::Protected => 32,
                    Mode::Long => 64,
                };
                TWOBYTE_66_TABLE[buf[i] as usize].clone()
            }
            (OpcodeEscape::Escape0F,SimdPrefix::PrefixF2) => TWOBYTE_F2_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F,SimdPrefix::PrefixF3) => TWOBYTE_F3_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F3A,SimdPrefix::None) => THREEBYTE_3A_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F3A,SimdPrefix::Prefix66) => {
                prefix.operand_size = match mode {
                    Mode::Real => 16,
                    Mode::Protected => 32,
                    Mode::Long => 64,
                };
                THREEBYTE_3A66_TABLE[buf[i] as usize].clone()
            }
            (OpcodeEscape::Escape0F3A,SimdPrefix::PrefixF3) => return Err("Unknown instruction".into()),
            (OpcodeEscape::Escape0F3A,SimdPrefix::PrefixF2) => THREEBYTE_3AF2_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F38,SimdPrefix::None) => THREEBYTE_38_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F38,SimdPrefix::Prefix66) => {
                prefix.operand_size = match mode {
                    Mode::Real => 16,
                    Mode::Protected => 32,
                    Mode::Long => 64,
                };
                THREEBYTE_3866_TABLE[buf[i] as usize].clone()
            }
            (OpcodeEscape::Escape0F38,SimdPrefix::PrefixF3) => THREEBYTE_38F3_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Escape0F38,SimdPrefix::PrefixF2) => THREEBYTE_38F2_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Xop8,SimdPrefix::None) => XOP8_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Xop8,_) => return Err("Unknown instruction".into()),
            (OpcodeEscape::Xop9,SimdPrefix::None) => XOP9_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::Xop9,_) => return Err("Unknown instruction".into()),
            (OpcodeEscape::XopA,SimdPrefix::None) => XOPA_TABLE[buf[i] as usize].clone(),
            (OpcodeEscape::XopA,_) => return Err("Unknown instruction".into()),
            (OpcodeEscape::Escape0F0F,_) => unreachable!(),
        };

        trace!("res: {:?}",opc);

        let opc = match opc.mnemonic {
            Mnemonic::Single(s) => {
                opc
            }
            Mnemonic::Undefined => return Err("Unknown instruction".into()),
            Mnemonic::Escape => {
                let modrm = buf[i + 1] as usize;
                let esc = buf[i];

                if modrm < 0xc0 {
                    let ext = (modrm >> 3) & 0b111;
                    match esc {
                        0xd8 => X87_D8_TABLE2[ext].clone(),
                        0xd9 => X87_D9_TABLE2[ext].clone(),
                        0xda => X87_DA_TABLE2[ext].clone(),
                        0xdb => X87_DB_TABLE2[ext].clone(),
                        0xdc => X87_DC_TABLE2[ext].clone(),
                        0xdd => X87_DD_TABLE2[ext].clone(),
                        0xde => X87_DE_TABLE2[ext].clone(),
                        0xdf => X87_DF_TABLE2[ext].clone(),
                        _ => unreachable!(),
                    }
                } else {
                    i += 1;
                    match esc {
                        0xd8 => X87_D8_TABLE[modrm - 0xc0].clone(),
                        0xd9 => X87_D9_TABLE[modrm - 0xc0].clone(),
                        0xda => X87_DA_TABLE[modrm - 0xc0].clone(),
                        0xdb => X87_DB_TABLE[modrm - 0xc0].clone(),
                        0xdc => X87_DC_TABLE[modrm - 0xc0].clone(),
                        0xdd => X87_DD_TABLE[modrm - 0xc0].clone(),
                        0xde => X87_DE_TABLE[modrm - 0xc0].clone(),
                        0xdf => X87_DF_TABLE[modrm - 0xc0].clone(),
                        _ => unreachable!(),
                    }
                }
            }
            Mnemonic::ModRM(grp) => {
                let pfx = if prefix.opcode_escape != OpcodeEscape::None {
                    prefix.simd_prefix
                } else {
                    SimdPrefix::None
                };

                try!(select_opcode_ext(grp,buf[i] as usize, buf[i + 1] as usize, pfx,mode,vexxop_present)).clone()
            }
        };

        match opc.option {
            OpcodeOption::Default64 if mode == Mode::Long && prefix.operand_size == 32 =>
                prefix.operand_size = 64,
            OpcodeOption::Force64 if mode == Mode::Long =>
                prefix.operand_size = 64,
            OpcodeOption::Only64 if mode != Mode::Long =>
                return Err("Unknown instruction".into()),
            OpcodeOption::Invalid64 if mode == Mode::Long =>
                return Err("Unknown instruction".into()),
            _ => {}
        }

        trace!("prefix after fixup: {:?}",prefix);

        trace!("opcode len: {}",i + 1);

        match opc.mnemonic {
            Mnemonic::Single(s) => {
                use std::io::Cursor;

                let mut tail = Tail::new(Cursor::new(&buf[i+1..]));
                let rex = if rex_present {
                    Some((prefix.rex_b,prefix.rex_x,prefix.rex_r,prefix.rex_w))
                } else {
                    None
                };
                let op1 = read_operand(&opc.operand_a,&mut tail,mode,prefix.vvvv,rex,prefix.operand_size,prefix.address_size,prefix.operand_size,0);
                let op2 = read_operand(&opc.operand_b,&mut tail,mode,prefix.vvvv,rex,prefix.operand_size,prefix.address_size,prefix.operand_size,0);
                let op3 = read_operand(&opc.operand_c,&mut tail,mode,prefix.vvvv,rex,prefix.operand_size,prefix.address_size,prefix.operand_size,0);
                let op4 = read_operand(&opc.operand_d,&mut tail,mode,prefix.vvvv,rex,prefix.operand_size,prefix.address_size,prefix.operand_size,0);

                let ops: Vec<String> = vec![op1,op2,op3,op4].iter().filter_map(|x| match x {
                    &Some(ref op) => Some(format!("{}",op)),
                    &None => None,
                }).collect();

                debug!("'{}' with {} bytes",s,tail.fd.position() as usize + i + 1);
                println!("{} {:?}",s,ops);
                trace!("");
                Ok(tail.fd.position() as usize + i + 1)
            }
            e => {
                println!("tried to match {:?}",e);
                Err("Internal error".into())
            }
        }
    }
}
/*
fn to_rreil(op: Operand) -> (Rvalue,Vec<Statement>) {
    match op {
        Operand::Register(ref name) => name.to_string(),
        Operand::Immediate(ref value,_) => format!("0x{:x}",value),
        Operand::Indirect(ref base,ref index,ref scale,ref disp,ref width) => {
            let mut s = format!("{} PTR [",match *width {
                8 => "BYTE",
                16 => "WORD",
                32 => "DWORD",
                64 => "QWORD",
                _ => "UNK",
            }.to_string());

            if *base != "" {
                s = format!("{}{}",s,base);
            }

            if *scale > 0 && *index != "" {
                if *base != "" {
                    s = format!("{} + ",s);
                }

                s = format!("{}{}*{}",s,index,scale);
            }

            if *disp > 0 {
                if *base != "" || (*scale > 0 && *index != "") {
                    s = format!("{} + ",s);
                }

                s = format!("{}0x{:x}",s,disp);
            }

            format!("{}]",s)
        },
    }
}*/
