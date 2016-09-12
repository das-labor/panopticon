/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016 Panopticon Authors
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
    Mnemonic,
    Statement,
    Guard,
};

#[macro_use]
mod tables;
mod semantic;

#[derive(Clone,Debug)]
pub enum Amd64 {}

#[derive(Clone,PartialEq,Copy)]
pub enum Condition {
    Below,
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

    fn decode(reg: &Region,start: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
        let mut data = reg.iter();
        let mut buf: Vec<u8> = vec![];
        let mut i = data.seek(start);
        let mut p = start;

        while let Some(Some(b)) = i.next() {
            buf.push(b);
            if buf.len() == 15 {
                break;
            }
        }

        info!("disass @ {:x}: {:?}",p,buf);

        let ret = ::amd64::read(*cfg,&buf,p).and_then(|(len,mne,mut jmp)| {
            Ok(Match::<Amd64> {
                tokens: buf[0..len as usize].to_vec(),
                mnemonics: vec![mne],
                jumps: jmp.drain(..).map(|x| (p,x.0,x.1)).collect::<Vec<_>>(),
                configuration: cfg.clone(),
            })
        });

        info!("    res: {:?}",ret);

        ret
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
    pub simd_size: usize,
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
        let s = match *self {
            Register::RAX => "RAX",
            Register::RBX => "RBX",
            Register::RCX => "RCX",
            Register::RDX => "RDX",
            Register::RDI => "RDI",
            Register::RSI => "RSI",
            Register::RSP => "RSP",
            Register::RBP => "RBP",
            Register::RIP => "RIP",
            Register::R8 => "R8",
            Register::R9 => "R9",
            Register::R10 => "R10",
            Register::R11 => "R11",
            Register::R12 => "R12",
            Register::R13 => "R13",
            Register::R14 => "R14",
            Register::R15 => "R15",

            Register::EAX => "EAX",
            Register::EBX => "EBX",
            Register::ECX => "ECX",
            Register::EDX => "EDX",
            Register::EDI => "EDI",
            Register::ESI => "ESI",
            Register::ESP => "ESP",
            Register::EBP => "EBP",
            Register::EIP => "EIP",
            Register::R8D => "R8D",
            Register::R9D => "R9D",
            Register::R10D => "R10D",
            Register::R11D => "R11D",
            Register::R12D => "R12D",
            Register::R13D => "R13D",
            Register::R14D => "R14D",
            Register::R15D => "R15D",

            Register::AX => "AX",
            Register::BX => "BX",
            Register::CX => "CX",
            Register::DX => "DX",
            Register::DI => "DI",
            Register::SI => "SI",
            Register::SP => "SP",
            Register::BP => "BP",
            Register::IP => "IP",
            Register::R8W => "R8W",
            Register::R9W => "R9W",
            Register::R10W => "R10W",
            Register::R11W => "R11W",
            Register::R12W => "R12W",
            Register::R13W => "R13W",
            Register::R14W => "R14W",
            Register::R15W => "R15W",

            Register::AL => "AL",
            Register::BL => "BL",
            Register::CL => "CL",
            Register::DL => "DL",
            Register::R8L => "R8L",
            Register::R9L => "R9L",
            Register::R10L => "R10L",
            Register::R11L => "R11L",
            Register::R12L => "R12L",
            Register::R13L => "R13L",
            Register::R14L => "R14L",
            Register::R15L => "R15L",
            Register::DIL => "DIL",
            Register::SIL => "SIL",
            Register::SPL => "SPL",
            Register::BPL => "BPL",

            Register::AH => "AH",
            Register::BH => "BH",
            Register::CH => "CH",
            Register::DH => "DH",

            Register::ES => "ES",
            Register::FS => "FS",
            Register::GS => "GS",
            Register::SS => "SS",
            Register::CS => "CS",
            Register::DS => "DS",

            Register::ST0 => "ST0",
            Register::ST1 => "ST1",
            Register::ST2 => "ST2",
            Register::ST3 => "ST3",
            Register::ST4 => "ST4",
            Register::ST5 => "ST5",
            Register::ST6 => "ST6",
            Register::ST7 => "ST7",

            Register::MM0 => "MM0",
            Register::MM1 => "MM1",
            Register::MM2 => "MM2",
            Register::MM3 => "MM3",
            Register::MM4 => "MM4",
            Register::MM5 => "MM5",
            Register::MM6 => "MM6",
            Register::MM7 => "MM7",

            Register::MMX0 => "MMX0",
            Register::MMX1 => "MMX1",
            Register::MMX2 => "MMX2",
            Register::MMX3 => "MMX3",
            Register::MMX4 => "MMX4",
            Register::MMX5 => "MMX5",
            Register::MMX6 => "MMX6",
            Register::MMX7 => "MMX7",

            Register::XMM0 => "XMM0",
            Register::XMM1 => "XMM1",
            Register::XMM2 => "XMM2",
            Register::XMM3 => "XMM3",
            Register::XMM4 => "XMM4",
            Register::XMM5 => "XMM5",
            Register::XMM6 => "XMM6",
            Register::XMM7 => "XMM7",
            Register::XMM8 => "XMM8",
            Register::XMM9 => "XMM9",
            Register::XMM10 => "XMM10",
            Register::XMM11 => "XMM11",
            Register::XMM12 => "XMM12",
            Register::XMM13 => "XMM13",
            Register::XMM14 => "XMM14",
            Register::XMM15 => "XMM15",

            Register::YMM0 => "YMM0",
            Register::YMM1 => "YMM1",
            Register::YMM2 => "YMM2",
            Register::YMM3 => "YMM3",
            Register::YMM4 => "YMM4",
            Register::YMM5 => "YMM5",
            Register::YMM6 => "YMM6",
            Register::YMM7 => "YMM7",
            Register::YMM8 => "YMM8",
            Register::YMM9 => "YMM9",
            Register::YMM10 => "YMM10",
            Register::YMM11 => "YMM11",
            Register::YMM12 => "YMM12",
            Register::YMM13 => "YMM13",
            Register::YMM14 => "YMM14",
            Register::YMM15 => "YMM15",

            Register::CR0 => "CR0",
            Register::CR1 => "CR1",
            Register::CR2 => "CR2",
            Register::CR3 => "CR3",
            Register::CR4 => "CR4",
            Register::CR5 => "CR5",
            Register::CR6 => "CR6",
            Register::CR7 => "CR7",
            Register::CR8 => "CR8",
            Register::CR9 => "CR9",
            Register::CR10 => "CR10",
            Register::CR11 => "CR11",
            Register::CR12 => "CR12",
            Register::CR13 => "CR13",
            Register::CR14 => "CR14",
            Register::CR15 => "CR15",

            Register::DR0 => "DR0",
            Register::DR1 => "DR1",
            Register::DR2 => "DR2",
            Register::DR3 => "DR3",
            Register::DR4 => "DR4",
            Register::DR5 => "DR5",
            Register::DR6 => "DR6",
            Register::DR7 => "DR7",
            Register::DR8 => "DR8",
            Register::DR9 => "DR9",
            Register::DR10 => "DR10",
            Register::DR11 => "DR11",
            Register::DR12 => "DR12",
            Register::DR13 => "DR13",
            Register::DR14 => "DR14",
            Register::DR15 => "DR15",

            Register::None => "",
        };

        if !f.alternate() {
            f.write_str(&s.to_lowercase())
        } else {
            f.write_str(&s.to_uppercase())
        }
    }
}

impl Register {
    pub fn width(&self) -> usize {
        match *self {
            Register::RAX => 64,
            Register::RBX => 64,
            Register::RCX => 64,
            Register::RDX => 64,
            Register::RDI => 64,
            Register::RSI => 64,
            Register::RSP => 64,
            Register::RBP => 64,
            Register::RIP => 64,
            Register::R8 => 64,
            Register::R9 => 64,
            Register::R10 => 64,
            Register::R11 => 64,
            Register::R12 => 64,
            Register::R13 => 64,
            Register::R14 => 64,
            Register::R15 => 64,

            Register::EAX => 32,
            Register::EBX => 32,
            Register::ECX => 32,
            Register::EDX => 32,
            Register::EDI => 32,
            Register::ESI => 32,
            Register::ESP => 32,
            Register::EBP => 32,
            Register::EIP => 32,
            Register::R8D => 32,
            Register::R9D => 32,
            Register::R10D => 32,
            Register::R11D => 32,
            Register::R12D => 32,
            Register::R13D => 32,
            Register::R14D => 32,
            Register::R15D => 32,

            Register::AX => 16,
            Register::BX => 16,
            Register::CX => 16,
            Register::DX => 16,
            Register::DI => 16,
            Register::SI => 16,
            Register::SP => 16,
            Register::BP => 16,
            Register::IP => 16,
            Register::R8W => 16,
            Register::R9W => 16,
            Register::R10W => 16,
            Register::R11W => 16,
            Register::R12W => 16,
            Register::R13W => 16,
            Register::R14W => 16,
            Register::R15W => 16,

            Register::AL => 8,
            Register::BL => 8,
            Register::CL => 8,
            Register::DL => 8,
            Register::R8L => 8,
            Register::R9L => 8,
            Register::R10L => 8,
            Register::R11L => 8,
            Register::R12L => 8,
            Register::R13L => 8,
            Register::R14L => 8,
            Register::R15L => 8,
            Register::DIL => 8,
            Register::SIL => 8,
            Register::SPL => 8,
            Register::BPL => 8,

            Register::AH => 8,
            Register::BH => 8,
            Register::CH => 8,
            Register::DH => 8,

            Register::ES => 16,
            Register::FS => 16,
            Register::GS => 16,
            Register::SS => 16,
            Register::CS => 16,
            Register::DS => 16,

            Register::ST0 => 80,
            Register::ST1 => 80,
            Register::ST2 => 80,
            Register::ST3 => 80,
            Register::ST4 => 80,
            Register::ST5 => 80,
            Register::ST6 => 80,
            Register::ST7 => 80,

            Register::MM0 => 32,
            Register::MM1 => 32,
            Register::MM2 => 32,
            Register::MM3 => 32,
            Register::MM4 => 32,
            Register::MM5 => 32,
            Register::MM6 => 32,
            Register::MM7 => 32,

            Register::MMX0 => 64,
            Register::MMX1 => 64,
            Register::MMX2 => 64,
            Register::MMX3 => 64,
            Register::MMX4 => 64,
            Register::MMX5 => 64,
            Register::MMX6 => 64,
            Register::MMX7 => 64,

            Register::XMM0 => 128,
            Register::XMM1 => 128,
            Register::XMM2 => 128,
            Register::XMM3 => 128,
            Register::XMM4 => 128,
            Register::XMM5 => 128,
            Register::XMM6 => 128,
            Register::XMM7 => 128,
            Register::XMM8 => 128,
            Register::XMM9 => 128,
            Register::XMM10 => 128,
            Register::XMM11 => 128,
            Register::XMM12 => 128,
            Register::XMM13 => 128,
            Register::XMM14 => 128,
            Register::XMM15 => 128,

            Register::YMM0 => 256,
            Register::YMM1 => 256,
            Register::YMM2 => 256,
            Register::YMM3 => 256,
            Register::YMM4 => 256,
            Register::YMM5 => 256,
            Register::YMM6 => 256,
            Register::YMM7 => 256,
            Register::YMM8 => 256,
            Register::YMM9 => 256,
            Register::YMM10 => 256,
            Register::YMM11 => 256,
            Register::YMM12 => 256,
            Register::YMM13 => 256,
            Register::YMM14 => 256,
            Register::YMM15 => 256,

            Register::CR0 => 32,
            Register::CR1 => 32,
            Register::CR2 => 32,
            Register::CR3 => 32,
            Register::CR4 => 32,
            Register::CR5 => 32,
            Register::CR6 => 32,
            Register::CR7 => 32,
            Register::CR8 => 32,
            Register::CR9 => 32,
            Register::CR10 => 32,
            Register::CR11 => 32,
            Register::CR12 => 32,
            Register::CR13 => 32,
            Register::CR14 => 32,
            Register::CR15 => 32,

            Register::DR0 => 32,
            Register::DR1 => 32,
            Register::DR2 => 32,
            Register::DR3 => 32,
            Register::DR4 => 32,
            Register::DR5 => 32,
            Register::DR6 => 32,
            Register::DR7 => 32,
            Register::DR8 => 32,
            Register::DR9 => 32,
            Register::DR10 => 32,
            Register::DR11 => 32,
            Register::DR12 => 32,
            Register::DR13 => 32,
            Register::DR14 => 32,
            Register::DR15 => 32,

            Register::None => 0,
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
    ALR8L, BLR11L, CLR9L, DLR10L,
    AHR12L, BHR15L, CHR13L, DHR14L,
    rAX, rBX, rCX, rDX, rDI, rSI, rSP, rBP,
    rAXr8, rCXr9, rDXr10, rBXr11, rSPr12, rBPr13, rSIr14, rDIr15,
    eAX, eBX, eCX, eDX, eDI, eSI, eSP, eBP,
    ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7,
    ES, GS, DS, SS, CS, FS,
    one,
    NTA, T0, T1, T2,
}

fn read_spec_register(op: OperandType,opsz: usize,rex_b: bool) -> Result<Operand> {
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

        OperandType::ALR8L => Ok(Operand::Register(if rex_b { Register::R8L } else { Register::AL })),
        OperandType::CLR9L => Ok(Operand::Register(if rex_b { Register::R9L } else { Register::CL })),
        OperandType::DLR10L => Ok(Operand::Register(if rex_b { Register::R10L } else { Register::DL })),
        OperandType::BLR11L => Ok(Operand::Register(if rex_b { Register::R11L } else { Register::BL })),
        OperandType::AHR12L => Ok(Operand::Register(if rex_b { Register::R12L } else { Register::AH })),
        OperandType::CHR13L => Ok(Operand::Register(if rex_b { Register::R13L } else { Register::CH })),
        OperandType::DHR14L => Ok(Operand::Register(if rex_b { Register::R14L } else { Register::DH })),
        OperandType::BHR15L => Ok(Operand::Register(if rex_b { Register::R15L } else { Register::BH })),

        OperandType::ES => Ok(Operand::Register(Register::ES)),
        OperandType::FS => Ok(Operand::Register(Register::FS)),
        OperandType::GS => Ok(Operand::Register(Register::GS)),
        OperandType::SS => Ok(Operand::Register(Register::SS)),
        OperandType::CS => Ok(Operand::Register(Register::CS)),
        OperandType::DS => Ok(Operand::Register(Register::DS)),

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

        OperandType::rAXr8 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R8 } else { Register::RAX })),
        OperandType::rCXr9 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R9 } else { Register::RCX })),
        OperandType::rDXr10 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R10 } else { Register::RDX })),
        OperandType::rBXr11 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R11 } else { Register::RBX })),
        OperandType::rSPr12 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R12 } else { Register::RSP })),
        OperandType::rBPr13 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R13 } else { Register::RBP })),
        OperandType::rSIr14 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R14 } else { Register::RSI })),
        OperandType::rDIr15 if opsz == 64 => Ok(Operand::Register(if rex_b { Register::R15 } else { Register::RDI })),

        OperandType::rAXr8 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R8D } else { Register::EAX })),
        OperandType::rCXr9 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R9D } else { Register::ECX })),
        OperandType::rDXr10 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R10D } else { Register::EDX })),
        OperandType::rBXr11 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R11D } else { Register::EBX })),
        OperandType::rSPr12 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R12D } else { Register::ESP })),
        OperandType::rBPr13 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R13D } else { Register::EBP })),
        OperandType::rSIr14 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R14D } else { Register::ESI })),
        OperandType::rDIr15 if opsz == 32 => Ok(Operand::Register(if rex_b { Register::R15D } else { Register::EDI })),

        OperandType::rAXr8 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R8W } else { Register::AX })),
        OperandType::rCXr9 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R9W } else { Register::CX })),
        OperandType::rDXr10 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R10W } else { Register::DX })),
        OperandType::rBXr11 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R11W } else { Register::BX })),
        OperandType::rSPr12 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R12W } else { Register::SP })),
        OperandType::rBPr13 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R13W } else { Register::BP })),
        OperandType::rSIr14 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R14W } else { Register::SI })),
        OperandType::rDIr15 if opsz == 16 => Ok(Operand::Register(if rex_b { Register::R15W } else { Register::DI })),

        _ => Err("Invalid OperandType value".into()),
    }
}

#[derive(Clone,Debug)]
pub enum OperandSpec {
    None,
    Present(AddressingMethod,OperandType),
}

#[derive(Clone,Debug)]
enum Operand {
    Register(Register),
    Immediate(u64,usize), // Value, Width (Bits)
    Indirect(SegmentOverride,Register,Register,usize,(u64,usize),usize), // Segment Override, Base, Index, Scale, Disp, Width (Bits)
    Optional,
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(),Error> {
        match *self {
            Operand::Register(ref name) => if f.alternate() {
                write!(f,"{:#}",name)
            } else {
                write!(f,"{}",name)
            },
            Operand::Immediate(ref value,ref width) => if *width < 64 {
                f.write_str(&format!("{:#x}",value % (1 << *width)))
            } else {
                f.write_str(&format!("{:#x}",value))
            },
            Operand::Indirect(ref seg, ref base,ref index,ref scale,ref disp,ref width) => {
                write!(f,"{} PTR ",match *width {
                    8 => "BYTE",
                    16 => "WORD",
                    32 => "DWORD",
                    64 => "QWORD",
                    _ => "UNK",
                });

                let _ = try!(match *seg {
                    SegmentOverride::None => write!(f,"["),
                    SegmentOverride::Cs => write!(f,"cs:["),
                    SegmentOverride::Ds => write!(f,"ds:["),
                    SegmentOverride::Es => write!(f,"es:["),
                    SegmentOverride::Fs => write!(f,"fs:["),
                    SegmentOverride::Gs => write!(f,"gs:["),
                    SegmentOverride::Ss => write!(f,"ss:["),
                });

                if *base != Register::None {
                    if f.alternate() {
                        write!(f,"{:#}",base);
                    } else {
                        write!(f,"{}",base);
                    }
                }

                if *scale > 0 && *index != Register::None {
                    if *base != Register::None {
                        write!(f,"+");
                    }

                    if f.alternate() {
                        write!(f,"{:#}*{}",index,scale);
                    } else {
                        write!(f,"{}*{}",index,scale);
                    }
                }

                if disp.0 > 0 {
                    if disp.0 & 0x8000_0000_0000_0000 != 0 {
                        if disp.1 < 64 {
                            write!(f,"-{:#x}",(disp.0 ^ 0xFFFF_FFFF_FFFF_FFFF).wrapping_add(1) % (1 << disp.1));
                        } else {
                            write!(f,"-{:#x}",(disp.0 ^ 0xFFFF_FFFF_FFFF_FFFF).wrapping_add(1));
                        }
                    } else {
                        if *base != Register::None || (*scale > 0 && *index != Register::None) {
                            write!(f,"+");
                        }
                        if disp.1 < 64 {
                            write!(f,"{:#x}",disp.0 % (1 << disp.1));
                        } else {
                            write!(f,"{:#x}",disp.0);
                        }
                    }

                }

                write!(f,"]")
            },
            Operand::Optional => write!(f,"(Opt)"),
        }
    }
}

fn read_operand(spec: &OperandSpec, tail: &mut Tail,
                mode: Mode, seg: SegmentOverride, vvvv: Option<u8>, rex: Option<(bool,bool,bool,bool)>,
                opsz: usize, addrsz: usize, simdsz: usize, addr: u64) -> Result<Operand> {
    match (spec,opsz) {
        (&OperandSpec::Present(AddressingMethod::None,ref reg),_) =>
            read_spec_register(reg.clone(),opsz,rex.unwrap_or((false,false,false,false)).3),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::v),16) =>
            Ok(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::v),32) =>
            Ok(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::v),64) =>
            Ok(Operand::Immediate(tail.read_u64().ok().unwrap(),64)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::p),16) =>
            Ok(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::A,OperandType::p),32) => {
            let imm16 = tail.read_u16().ok().unwrap() as u64;
            let imm32 = tail.read_u32().ok().unwrap() as u64;
            Ok(Operand::Immediate((imm16 << 32) | imm32,48))
        }
        (&OperandSpec::Present(AddressingMethod::A,OperandType::p),64) => {
            // XXX
            let _ = tail.read_u16().ok().unwrap();
            let imm64 = tail.read_u64().ok().unwrap();
            Ok(Operand::Immediate(imm64 as u64,64))
        }
        (&OperandSpec::Present(AddressingMethod::B,OperandType::y),opsz) if vvvv.is_some() =>
            read_register(vvvv.unwrap(),rex.is_some(),cmp::max(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::C,OperandType::d),_) =>
            read_ctrl_register(tail.modrm(rex).ok().unwrap().1,32),
        (&OperandSpec::Present(AddressingMethod::D,OperandType::d),_) =>
            read_debug_register(tail.modrm(rex).ok().unwrap().1,32),

        // E
        (&OperandSpec::Present(AddressingMethod::E,OperandType::v),opsz) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::z),_) =>
            read_effective_address(mode,seg,tail,rex,cmp::min(32,opsz),addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::y),opsz) =>
            read_effective_address(mode,seg,tail,rex,cmp::max(32,opsz),addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::b),_) =>
            read_effective_address(mode,seg,tail,rex,8,addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::w),_) =>
            read_effective_address(mode,seg,tail,rex,16,addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::d),_) =>
            read_effective_address(mode,seg,tail,rex,32,addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::E,OperandType::dq),_) =>
            read_effective_address(mode,seg,tail,rex,64,addrsz,addr,opsz),

        // G
        (&OperandSpec::Present(AddressingMethod::G,OperandType::dq),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::d),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),32),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::w),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),16),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::b),_) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),8),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::v),opsz) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),opsz),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::z),opsz) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),cmp::min(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::G,OperandType::y),opsz) =>
            read_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),cmp::max(32,opsz)),

        // H
        (&OperandSpec::Present(AddressingMethod::H,OperandType::x),opsz) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),opsz),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::qq),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),256),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::dq),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::ps),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::pd),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::ss),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::H,OperandType::sd),_) if vvvv.is_some() =>
            read_simd_register(vvvv.unwrap(),rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::H,_),_) if vvvv.is_none() =>
            Ok(Operand::Optional),

        (&OperandSpec::Present(AddressingMethod::I,OperandType::z),16) =>
            Ok(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::z),_) =>
            Ok(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::b),_) =>
            Ok(Operand::Immediate(((tail.read_u8().ok().unwrap() as i8) as i64) as u64,opsz)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::one),opsz) =>
            Ok(Operand::Immediate(1,opsz)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::w),_) =>
            Ok(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),16) =>
            Ok(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,16)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),32) =>
            Ok(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,32)),
        (&OperandSpec::Present(AddressingMethod::I,OperandType::v),64) =>
            Ok(Operand::Immediate(tail.read_u64().ok().unwrap() as u64,64)),
        (&OperandSpec::Present(AddressingMethod::J,OperandType::b),_) =>
            Ok(Operand::Immediate(addr.wrapping_add(((tail.read_u8().ok().unwrap() as i8) as i64) as u64).wrapping_add(1),addrsz)),
        (&OperandSpec::Present(AddressingMethod::J,OperandType::z),16) =>
            Ok(Operand::Immediate(addr.wrapping_add(((tail.read_u16().ok().unwrap() as i16) as i64) as u64).wrapping_add(2),addrsz)),
        (&OperandSpec::Present(AddressingMethod::J,OperandType::z),_) =>
            Ok(Operand::Immediate(addr.wrapping_add(((tail.read_u32().ok().unwrap() as i32) as i64) as u64).wrapping_add(4),addrsz)),
        (&OperandSpec::Present(AddressingMethod::L,OperandType::x),32) =>
            read_simd_register(tail.read_u8().ok().unwrap() & 0b0111,rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::L,OperandType::x),_) =>
            read_simd_register(tail.read_u8().ok().unwrap() & 0b1111,rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::p),16) =>
            read_effective_address(mode,seg,tail,rex,16,addrsz,addr,32),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::p),32) =>
            read_effective_address(mode,seg,tail,rex,32,addrsz,addr,48),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::p),64) =>
            read_effective_address(mode,seg,tail,rex,64,addrsz,addr,80),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::w),opsz) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,16),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::d),opsz) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,32),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::q),opsz) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,64),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::s),64) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,80),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::s),_) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,48),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::b),_) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,8),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::None),opsz) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,opsz),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::a),32) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,64),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::a),16) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,32),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::y),opsz) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,cmp::min(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::x),32) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,128),
        (&OperandSpec::Present(AddressingMethod::M,OperandType::x),64) =>
            read_effective_address(mode,seg,tail,rex,opsz,addrsz,addr,256),
        (&OperandSpec::Present(AddressingMethod::N,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 16 =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),seg,addrsz,8),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 32 =>
            read_memory(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,addrsz),seg,addrsz,8),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 64 =>
            read_memory(Operand::Immediate(tail.read_u64().ok().unwrap() as u64,addrsz),seg,addrsz,8),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 16 =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),seg,addrsz,opsz),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 32 =>
            read_memory(Operand::Immediate(tail.read_u32().ok().unwrap() as u64,addrsz),seg,addrsz,opsz),
        (&OperandSpec::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 64 =>
            read_memory(Operand::Immediate(tail.read_u64().ok().unwrap() as u64,addrsz),seg,addrsz,opsz),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::pi),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::ps),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::P,OperandType::d),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),32),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::d),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,32),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::pi),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,simdsz),
        (&OperandSpec::Present(AddressingMethod::Q,OperandType::q),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,32),
        (&OperandSpec::Present(AddressingMethod::S,OperandType::w),_) =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),seg,addrsz,16),
        (&OperandSpec::Present(AddressingMethod::R,OperandType::d),_) =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),seg,addrsz,32),
        (&OperandSpec::Present(AddressingMethod::R,OperandType::q),_) =>
            read_memory(Operand::Immediate(tail.read_u16().ok().unwrap() as u64,addrsz),seg,addrsz,64),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::ps),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::pi),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::x),32) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::x),64) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),256),
        (&OperandSpec::Present(AddressingMethod::U,OperandType::dq),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().2,rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::pi),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::ps),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::pd),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),simdsz),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::ss),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::x),32) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::x),64) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),256),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::dq),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::q),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),64),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::sd),_) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),128),
        (&OperandSpec::Present(AddressingMethod::V,OperandType::y),opsz) =>
            read_simd_register(tail.modrm(rex).ok().unwrap().1,rex.is_some(),cmp::min(32,opsz)),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::pd),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,simdsz),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::ps),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,simdsz),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::q),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,64),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::dq),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,128),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::x),32) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,128),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::x),64) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,256),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::sd),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,128),
        (&OperandSpec::Present(AddressingMethod::W,OperandType::ss),_) =>
            read_effective_simd_address(mode,seg,tail,rex,opsz,addrsz,addr,128),
        _ => {
            error!("can't decode {:?}/{}",spec,opsz);
            Err(format!("can't decode {:?}/{}",spec,opsz).into())
        }
    }
}

fn sign_ext_u8(val: u8, w: usize) -> u64 {
    match w {
        8 => val as u64,
        16 => ((val as i8) as i16) as u64,
        32 => ((val as i8) as i32) as u64,
        _ => ((val as i8) as i64) as u64,
    }
}

fn sign_ext_u32(val: u32, w: usize) -> u64 {
    match w {
        8 => (val & 0xFF) as u64,
        16 => (val & 0xFFFF) as u64,
        32 => val as u64,
        _ => ((val as i32) as i64) as u64,
    }
}

fn read_effective_simd_address(mode: Mode, seg: SegmentOverride, tail: &mut Tail,
                               rex: Option<(bool,bool,bool,bool)>,
                               opsz: usize, addrsz: usize, ip: u64, simdsz: usize) -> Result<Operand> {
    let (mod_,reg,rm) = try!(tail.modrm(rex));

    match (mod_,rm & 0b111) {
        // mod = 00
        (0b00,0b000) | (0b00,0b001) | (0b00,0b010) |
        (0b00,0b011) | (0b00,0b110) | (0b00,0b111) =>
            read_memory(try!(read_register(rm,rex.is_some(),addrsz)),seg,addrsz,simdsz),
        (0b00,0b100) =>
            tail.sib(mod_,seg,rex,addrsz,opsz),
        (0b00,0b101) if mode == Mode::Long =>
            Ok(Operand::Indirect(seg,Register::RIP,Register::None,0,(sign_ext_u32(try!(tail.read_u32()),addrsz),32),opsz)),
        (0b00,0b101) if mode != Mode::Long =>
            Ok(Operand::Indirect(seg,Register::None,Register::None,0,(sign_ext_u32(try!(tail.read_u32()),addrsz),32),opsz)),

        // mod = 01
        (0b01,0b000) | (0b01,0b001) | (0b01,0b010) | (0b01,0b011) |
        (0b01,0b101) | (0b01,0b110) | (0b01,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),addrsz) {
                Ok(Operand::Indirect(seg,reg,Register::None,0,(sign_ext_u8(try!(tail.read_u8()),addrsz),8),simdsz))
            } else {
                error!("Failed to decode SIB byte");
                Err("Failed to decode SIB byte".into())
            },
        (0b01,0b100) =>
            if let Operand::Indirect(e,b,i,s,_,w) = try!(tail.sib(mod_,seg,rex,addrsz,opsz)) {
                let d = sign_ext_u8(try!(tail.read_u8()),opsz);
                Ok(Operand::Indirect(e,b,i,s,(d as u64,8),w))
            } else {
                error!("Internal error: read_sib did not return indirect operand");
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 10
        (0b10,0b000) | (0b10,0b001) | (0b10,0b010) | (0b10,0b011) |
        (0b10,0b101) | (0b10,0b110) | (0b10,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),addrsz) {
                Ok(Operand::Indirect(seg,reg,Register::None,0,(sign_ext_u32(try!(tail.read_u32()),addrsz),32),simdsz))
            } else {
                error!("Failed to decode SIB byte");
                Err("Failed to decode SIB byte".into())
            },
       (0b10,0b100) =>
            if let Operand::Indirect(e,b,i,s,_,w) = try!(tail.sib(mod_,seg,rex,addrsz,opsz)) {
                let d = sign_ext_u32(try!(tail.read_u32()),addrsz);
                Ok(Operand::Indirect(e,b,i,s,(d as u64,32),w))
            } else {
                error!("Internal error: read_sib did not return indirect operand");
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 11
        (0b11,_) => read_simd_register(rm,rex.is_some(),simdsz),

        _ => {
            error!("Invalid mod value: {:b}",mod_);
            Err("Invalid mod value".into())
        }
    }
}

fn read_effective_address(mode: Mode, seg: SegmentOverride, tail: &mut Tail,
                          rex: Option<(bool,bool,bool,bool)>,
                          opsz: usize, addrsz: usize, ip: u64, simdsz: usize) -> Result<Operand> {
    let (mod_,reg,rm) = try!(tail.modrm(rex));

    match (mod_,rm & 0b111) {
        // mod = 00
        (0b00,0b000) | (0b00,0b001) | (0b00,0b010) |
        (0b00,0b011) | (0b00,0b110) | (0b00,0b111) =>
            read_memory(try!(read_register(rm,rex.is_some(),addrsz)),seg,addrsz,opsz),
        (0b00,0b100) =>
            tail.sib(mod_,seg,rex,addrsz,opsz),
        (0b00,0b101) if mode == Mode::Long =>
            Ok(Operand::Indirect(seg,Register::RIP,Register::None,0,(sign_ext_u32(try!(tail.read_u32()),addrsz),32),opsz)),
        (0b00,0b101) if mode != Mode::Long =>
            Ok(Operand::Indirect(seg,Register::None,Register::None,0,(sign_ext_u32(try!(tail.read_u32()),addrsz),32),opsz)),

        // mod = 01
        (0b01,0b000) | (0b01,0b001) | (0b01,0b010) | (0b01,0b011) |
        (0b01,0b101) | (0b01,0b110) | (0b01,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),addrsz) {
                Ok(Operand::Indirect(seg,reg,Register::None,0,(sign_ext_u8(try!(tail.read_u8()),addrsz),8),opsz))
            } else {
                error!("Failed to decode r/m byte");
                Err("Failed to decode r/m byte".into())
            },
        (0b01,0b100) =>
            match tail.sib(mod_,seg,rex,addrsz,opsz) {
                Ok(Operand::Indirect(e,b,i,s,_,w)) => {
                    let d = sign_ext_u8(try!(tail.read_u8()),addrsz);
                    Ok(Operand::Indirect(e,b,i,s,(d as u64,8),w))
                }
                Ok(_) => {
                    error!("Failed to decode SIB byte: No Indirect");
                    Err("Failed to decode SIB byte: No Indirect".into())
                }
                Err(e) => {
                    error!("Failed to decode SIB byte: {}",e);
                    Err("Failed to decode SIB byte".into())
                }
            },

        // mod = 10
        (0b10,0b000) | (0b10,0b001) | (0b10,0b010) | (0b10,0b011) |
        (0b10,0b101) | (0b10,0b110) | (0b10,0b111) =>
            if let Ok(Operand::Register(reg)) = read_register(rm,rex.is_some(),addrsz) {
                Ok(Operand::Indirect(seg,reg,Register::None,0,(sign_ext_u32(try!(tail.read_u32()),addrsz),32),opsz))
            } else {
                error!("Failed to decode SIB byte");
                Err("Failed to decode SIB byte".into())
            },
        (0b10,0b100) =>
            if let Operand::Indirect(e,b,i,s,_,w) = try!(tail.sib(mod_,seg,rex,addrsz,opsz)) {
                let d = sign_ext_u32(try!(tail.read_u32()),addrsz);
                Ok(Operand::Indirect(e,b,i,s,(d as u64,32),w))
            } else {
                error!("Internal error: read_sib did not return indirect operand");
                Err("Internal error: read_sib did not return indirect operand".into())
            },

        // mod = 11
        (0b11,_) => read_register(rm,rex.is_some(),opsz),

        _ => {
            error!("Invalid mod value: {:b}",mod_);
            Err("Invalid mod value".into())
        }
    }
}

fn read_memory(op: Operand, seg: SegmentOverride, addrsz: usize, width: usize) -> Result<Operand> {
    match op {
        Operand::Register(reg) => Ok(Operand::Indirect(seg,reg,Register::None,0,(0,0),width)),
        Operand::Immediate(imm,w) => Ok(Operand::Indirect(seg,Register::None,Register::None,0,(imm,w),width)),
        Operand::Indirect(_,_,_,_,_,_) => {
            error!("Tried to contruct doubly indirect operand");
            Err("Tried to contruct doubly indirect operand".into())
        }
        Operand::Optional => {
            error!("Tried to use and optional operand as address");
            Err("Tried to use and optional operand as address".into())
        }
    }
}

fn read_sib<R: ReadBytesExt>(fd: &mut R, mod_: u8, seg: SegmentOverride, rex: Option<(bool,bool,bool,bool)>,
            addrsz: usize,width: usize) -> Result<Operand> {
    let sib = try!(fd.read_u8());
    let scale = sib >> 6;
    let mut index = (sib >> 3) & 0b111;
    let mut base = sib & 0b111;

    trace!("read sib 0x{:02x} ({:02b},{:03b},{:03b})",sib,scale,index,base);

    if mod_ != 0b11 {
        if let Some((_,_,x,b)) = rex {
            if x { index |= 0b1000 };
            if b { base |= 0b1000 };
        }
    }

    let ret_scale = 1 << scale;
    let (ret_base,ret_disp) = if mod_ != 0b11 && base & 0b111 == 0b101 {
        match mod_ {
            0b00 => (Register::None,(sign_ext_u32(try!(fd.read_u32::<LittleEndian>()),addrsz),32)),
            0b01 => (Register::EBP,(0,0)),
            0b10 => (Register::EBP,(0,0)),
            _ => {
                error!("read_sib: invalid mod value");
                return Err("Internal error".into())
            }
        }
    } else {
        if let Ok(Operand::Register(r)) = read_register(base,rex.is_some(),addrsz) {
            (r,(0,0))
        } else {
            error!("read_sib: Failed to decode base register");
            return Err("Failed to decode base register".into());
        }
    };
    let ret_index = if index & 0b111 == 0b100 {
        Register::None
    } else {
        if let Ok(Operand::Register(r)) = read_register(index,rex.is_some(),addrsz) {
            r
        } else {
            error!("read_sib: Failed to decode index register");
            return Err("Failed to decode index register".into());
        }
    };

    // disp handled by calling function
    Ok(Operand::Indirect(seg,ret_base,ret_index,ret_scale,ret_disp,width))
}

fn read_modrm<R: ReadBytesExt>(fd: &mut R,rex: Option<(bool,bool,bool,bool)>) -> Result<(u8,u8,u8)> {
    let modrm = try!(fd.read_u8());
    let mod_ = modrm >> 6;
    let mut reg = (modrm >> 3) & 0b111;
    let mut rm = modrm & 0b111;
    let sib_present = mod_ != 0b11 && rm == 0b100;

    if let Some((w,r,x,b)) = rex {
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
        (0b0100,8) => if !rex_present { Ok(Operand::Register(Register::AH)) } else { Ok(Operand::Register(Register::SPL)) },
        (0b0101,8) => if !rex_present { Ok(Operand::Register(Register::CH)) } else { Ok(Operand::Register(Register::BPL)) },
        (0b0110,8) => if !rex_present { Ok(Operand::Register(Register::DH)) } else { Ok(Operand::Register(Register::SIL)) },
        (0b0111,8) => if !rex_present { Ok(Operand::Register(Register::BH)) } else { Ok(Operand::Register(Register::DIL)) },
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

        _ => {
            error!("Invalid reg value {:b} ({} bits)",reg,opsz);
            Err(format!("Invalid reg value {:b} ({} bits)",reg,opsz).into())
        }
    }
}
fn read_simd_register(reg: u8, rex_present: bool, opsz: usize) -> Result<Operand> {
    match (reg,opsz) {
       (0b0000,32) => Ok(Operand::Register(Register::MM0)),
       (0b0001,32) => Ok(Operand::Register(Register::MM1)),
       (0b0010,32) => Ok(Operand::Register(Register::MM2)),
       (0b0011,32) => Ok(Operand::Register(Register::MM3)),
       (0b0100,32) => Ok(Operand::Register(Register::MM4)),
       (0b0101,32) => Ok(Operand::Register(Register::MM5)),
       (0b0110,32) => Ok(Operand::Register(Register::MM6)),
       (0b0111,32) => Ok(Operand::Register(Register::MM7)),
       (0b1000,32) => Ok(Operand::Register(Register::MM0)),
       (0b1001,32) => Ok(Operand::Register(Register::MM1)),
       (0b1010,32) => Ok(Operand::Register(Register::MM2)),
       (0b1011,32) => Ok(Operand::Register(Register::MM3)),
       (0b1100,32) => Ok(Operand::Register(Register::MM4)),
       (0b1101,32) => Ok(Operand::Register(Register::MM5)),
       (0b1110,32) => Ok(Operand::Register(Register::MM6)),
       (0b1111,32) => Ok(Operand::Register(Register::MM7)),

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
pub enum MnemonicSpec {
    Undefined,
    Escape,
    Single(&'static str,),
    ModRM(isize),
}

#[derive(Clone,Debug)]
pub enum JumpSpec {
    DeadEnd,
    FallThru,
    Branch(Rvalue,Guard),
    Jump(Rvalue),
}

#[derive(Clone,Debug)]
pub enum Opcode {
    Nonary(MnemonicSpec,OpcodeOption,fn() -> Result<(Vec<Statement>,JumpSpec)>),
    Unary(MnemonicSpec,OpcodeOption,fn(Rvalue) -> Result<(Vec<Statement>,JumpSpec)>,OperandSpec),
    Binary(MnemonicSpec,OpcodeOption,fn(Rvalue,Rvalue) -> Result<(Vec<Statement>,JumpSpec)>,OperandSpec,OperandSpec),
    Trinary(MnemonicSpec,OpcodeOption,fn(Rvalue,Rvalue,Rvalue) -> Result<(Vec<Statement>,JumpSpec)>,OperandSpec,OperandSpec,OperandSpec),
    Quaternary(MnemonicSpec,OpcodeOption,fn(Rvalue,Rvalue,Rvalue,Rvalue) -> Result<(Vec<Statement>,JumpSpec)>,OperandSpec,OperandSpec,OperandSpec,OperandSpec),
}

impl Opcode {
    pub fn operands<'a>(&'a self) -> Vec<&'a OperandSpec> {
        match *self {
            Opcode::Nonary(_,_,_) => vec![],
            Opcode::Unary(_,_,_,ref a) => vec![a],
            Opcode::Binary(_,_,_,ref a,ref b) => vec![a,b],
            Opcode::Trinary(_,_,_,ref a,ref b,ref c) => vec![a,b,c],
            Opcode::Quaternary(_,_,_,ref a,ref b,ref c,ref d) => vec![a,b,c,d],
        }
    }

    pub fn call(&self,a: &Option<Rvalue>, b: &Option<Rvalue>, c: &Option<Rvalue>, d: &Option<Rvalue>) -> Result<(Vec<Statement>,JumpSpec)> {
        match *self {
            Opcode::Nonary(_,_,ref f) => f(),
            Opcode::Unary(_,_,ref f,_) => if let &Some(ref a) = a {
                f(a.clone())
            } else {
                Err("Internal error. Called 1-ary function with 0 arguments".into())
            },
            Opcode::Binary(_,_,ref f,_,_) => if let (&Some(ref a),&Some(ref b)) = (a,b) {
                f(a.clone(),b.clone())
            } else {
                Err("Internal error. Called 2-ary function less than 2 arguments".into())
            },
            Opcode::Trinary(_,_,ref f,_,_,_) => if let (&Some(ref a),&Some(ref b),&Some(ref c)) = (a,b,c) {
                f(a.clone(),b.clone(),c.clone())
            } else {
                Err("Internal error. Called 3-ary function less than 3 arguments".into())
            },
            Opcode::Quaternary(_,_,ref f,_,_,_,_) => if let (&Some(ref a),&Some(ref b),&Some(ref c),&Some(ref d)) = (a,b,c,d) {
                f(a.clone(),b.clone(),c.clone(),d.clone())
            } else {
                Err("Internal error. Called 4-ary function less than 4 arguments".into())
            },
        }
    }

    pub fn mnemonic<'a>(&'a self) -> &'a MnemonicSpec {
        match *self {
            Opcode::Nonary(ref mne,_,_) => mne,
            Opcode::Unary(ref mne,_,_,_) => mne,
            Opcode::Binary(ref mne,_,_,_,_) => mne,
            Opcode::Trinary(ref mne,_,_,_,_,_) => mne,
            Opcode::Quaternary(ref mne,_,_,_,_,_,_) => mne,
        }
    }

    pub fn option<'a>(&'a self) -> &'a OpcodeOption {
        match *self {
            Opcode::Nonary(_,ref opo,_) => opo,
            Opcode::Unary(_,ref opo,_,_) => opo,
            Opcode::Binary(_,ref opo,_,_,_) => opo,
            Opcode::Trinary(_,ref opo,_,_,_,_) => opo,
            Opcode::Quaternary(_,ref opo,_,_,_,_,_) => opo,
        }
    }
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
            simd_size: 0,
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

    pub fn sib(&mut self,mod_: u8,seg: SegmentOverride,rex: Option<(bool,bool,bool,bool)>,
               addrsz: usize, width: usize) -> Result<Operand> {
        if self.sib.is_none() {
            self.sib = Some(try!(read_sib(&mut self.fd,mod_,seg,rex,addrsz,width)));
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
            (0b000,0b001) => opcode!(vmcall; ),
            (0b000,0b010) => opcode!(vmlaunch; ),
            (0b000,0b011) => opcode!(vmresume; ),
            (0b000,0b100) => opcode!(vmxoff; ),
            (0b000,_) => unused!(),
            (0b001,0b000) => opcode!(monitor; ),
            (0b001,0b001) => opcode!(mwait; ),
            (0b001,0b010) => opcode!(clac; ),
            (0b001,0b011) => opcode!(stac; ),
            (0b001,0b111) => opcode!(encls; ),
            (0b001,_) => unused!(),
            (0b010,0b000) => opcode!(xgetbv; E/v),
            (0b010,0b001) => opcode!(xsetbv; ),
            (0b010,0b100) => opcode!(vmfunc; ),
            (0b010,0b101) => opcode!(xend; ),
            (0b010,0b110) => opcode!(xtest; ),
            (0b010,0b111) => opcode!(enclu; ),
            (0b010,_) => unused!(),
            (0b011,_) => unused!(),
            (0b100,_) => GROUP7_OPC01_MEM[reg].clone(),
            (0b101,_) => unused!(),
            (0b110,_) => GROUP7_OPC01_MEM[reg].clone(),
            (0b111,0b000) => opcode!(swapgs; ),
            (0b111,0b001) => opcode!(rdtscp; ),
            (0b111,_) => unused!(),
            (_,_) => unreachable!(),
        },
        (7,_,SimdPrefix::None,0x01) => GROUP7_OPC01_MEM[reg].clone(),

        // GROUP8
        (8,_,SimdPrefix::None,0xba) => GROUP8_OPCBA[reg].clone(),

        // GROUP9
        (9,_,SimdPrefix::None,0xc7) => match (mo,pfx,reg) {
            (0b11,SimdPrefix::None,0b110) => opcode!(rdrand; R/v),
            (0b11,SimdPrefix::None,0b111) => opcode!(rdseed; R/v),
            (0b11,SimdPrefix::PrefixF3,0b111) => opcode!(rdpid; R/dq),
            (_,SimdPrefix::None,0b001,) => {
                if mode == Mode::Long {
                    opcode!(cmpxch8b; M/q)
                } else {
                    opcode!(cmpxchg16b; M/dq)
                }
            }
            (_,SimdPrefix::None,0b110) => opcode!(vmptrld; M/q),
            (_,SimdPrefix::None,0b111) => opcode!(vmptrst; M/q),
            (_,SimdPrefix::Prefix66,0b110) => opcode!(vmclear; M/q),
            (_,SimdPrefix::PrefixF3,0b110) => opcode!(vmxon; M/q),
            _ => unused!(),
        },

        // GROUP10
        (10,_,SimdPrefix::None,0xb9) => GROUP10_OPCB9[reg].clone(),

        // GROUP11
        (11,_,SimdPrefix::None,0xc6) => match (mo,reg,rm) {
            (0b11,0b111,0b000) => opcode!(xabort; I/b),
            (_,_,_) => GROUP11_OPCC6[reg].clone(),
        },
        (11,_,SimdPrefix::None,0xc7) => match (mo,reg,rm) {
            (0b11,0b111,0b000) => opcode!(xbegin; I/b),
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
            0b101 => opcode!(lfence; ),
            0b110 => opcode!(mfence; ),
            0b111 => opcode!(sfence; ),
            _ => unused!(),
        },
        (15,0b11,SimdPrefix::PrefixF3,0xae) => match reg {
            0b000 => opcode!(rdfsbase; R/y),
            0b001 => opcode!(rdgsbase; R/y),
            0b010 => opcode!(wrfsbase; R/y),
            0b011 => opcode!(wrgsbase; R/y),
            _ => unused!(),
        },
        (15,_,SimdPrefix::None,0xae) => match reg {
            0b000 => opcode!(fxsave; ),
            0b001 => opcode!(fxstor; ),
            0b010 => opcode!(ldmxcsr; ),
            0b011 => opcode!(stmxcsr; ),
            0b100 => opcode!(xsave; ),
            0b101 => opcode!(xrstor; ),
            0b110 => opcode!(xsaveopt; ),
            0b111 => opcode!(clflush; ),
            _ => unreachable!(),
        },

        // GROUP16
        (16,0b11,SimdPrefix::None,0x18) => unused!(),
        (16,_,SimdPrefix::None,0x18) => match reg {
            0b000 => opcode!(prefetch; M/None),
            0b001 => opcode!(prefetch; M/None),
            0b010 => opcode!(prefetch; M/None),
            0b011 => opcode!(prefetch; M/None),
            _ => unused!(),
        },

        // GROUP17
        (17,_,_,0xf3) if vexxop_present => match reg {
            0b001 => opcode!(blsr; B/y, E/y),
            0b010 => opcode!(blsmsk; B/y, E/y),
            0b011 => opcode!(blsi; B/y, E/y),
            _ => unused!(),
        },

        // GROUPX
        (102,_,SimdPrefix::None,0x01) => GROUP102_OPC01[reg].clone(),

        _ => return Err("Unknown instruction".into()),
    })
}

pub fn read(mode: Mode, buf: &[u8], addr: u64) -> Result<(u64,Mnemonic,Vec<(Rvalue,Guard)>)> {
    let mut i = 0;
    let mut prefix = Prefix::default();
    let mut vexxop_present = false;
    let mut rex_present = false;

    match mode {
        Mode::Real => {
            prefix.address_size = 16;
            prefix.operand_size = 16;
            prefix.simd_size = 128;
        }
        Mode::Protected => {
            prefix.address_size = 32;
            prefix.operand_size = 32;
            prefix.simd_size = 128;
        }
        Mode::Long => {
            prefix.address_size = 64;
            prefix.operand_size = 32;
            prefix.simd_size = 128;
        }
    }

    while i < 15 {
        match buf.get(i) {
            // Group 1: LOCK, REPE, REPNE
            Some(&0xf0) | Some(&0xf2) | Some(&0xf3) =>
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
            Some(&0x2e) | Some(&0x36) | Some(&0x3e) | Some(&0x26) | Some(&0x64) | Some(&0x65) =>
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
            Some(&0x66) => {
                match mode {
                    Mode::Long | Mode::Protected => prefix.operand_size = 16,
                    Mode::Real => prefix.operand_size = 32,
                }
                if i == 0 { prefix.simd_prefix = SimdPrefix::Prefix66; }
            }
            // Group 4: Address size override
            Some(&0x67) =>
                if prefix.address_size == 1 {
                    error!("Multiple address-size override prefixes");
                } else {
                    prefix.address_size = 1;
                },
            // 2 byte VEX
            Some(&0xc5) if i == 0 && mode == Mode::Long => {
                let vex = buf[i + 1];

                prefix.simd_prefix = match vex & 0b00000011 {
                    0 => SimdPrefix::None,
                    1 => SimdPrefix::Prefix66,
                    2 => SimdPrefix::PrefixF3,
                    3 => SimdPrefix::PrefixF2,
                    _ => unreachable!(),
                };

                prefix.opcode_escape = OpcodeEscape::Escape0F;
                if vex & 0b100 != 0 { prefix.simd_size = 256 }
                prefix.vvvv = Some(&0xFF ^ ((vex >> 3) & 0b1111));
                prefix.rex_r = vex & 0b1000000 == 0;

                vexxop_present = true;
                rex_present = true;
                i += 2;

                break;
            }

            // 3 byte VEX
            Some(&0xc4) if i == 0 && mode == Mode::Long => {
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

                prefix.vvvv = Some(&0xFF ^ ((vex2 >> 3) & 0b1111));
                if vex2 & 0b100 != 0 { prefix.simd_size = 256 }
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
            Some(&0x62) if i == 0 && mode == Mode::Long => {
                unimplemented!()
            },

            // XOP
            Some(&0x8f) if i == 0 && mode == Mode::Long => {
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
            Some(&0x40) | Some(&0x41) | Some(&0x42) | Some(&0x43) | Some(&0x44) |
            Some(&0x45) | Some(&0x46) | Some(&0x47) | Some(&0x48) | Some(&0x49) |
            Some(&0x4a) | Some(&0x4b) | Some(&0x4c) | Some(&0x4d) | Some(&0x4e) |
            Some(&0x4f) if mode == Mode::Long => {
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
    if !vexxop_present && buf.get(i) == Some(&0x0f) {
        match buf.get(i + 1) {
            Some(&0x38) => {
                prefix.opcode_escape = OpcodeEscape::Escape0F38;
                i += 2;
            }
            Some(&0x3A) => {
                prefix.opcode_escape = OpcodeEscape::Escape0F3A;
                i += 2;
            }
            Some(&0x0f) => {
                prefix.opcode_escape = OpcodeEscape::Escape0F0F;
                i += 2;
            }
            Some(_) => {
                prefix.opcode_escape = OpcodeEscape::Escape0F;
                i += 1;
            }
            None => return Err("Premature buffer end".into()),
        }
    }

    use amd64::tables::*;

    trace!("prefix: {:?}, opcode: {:?}",prefix,buf.get(i).cloned());

    if prefix.opcode_escape == OpcodeEscape::Escape0F0F {
        // XXX: 3DNow!
        return Err("Unknown instruction".into())
    } else {
        // remove non-mandatory prefixes
        let rm_pfx = match prefix.opcode_escape {
            OpcodeEscape::Escape0F => {
                if let Some(b) = buf.get(i) {
                    match *b {
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
                    }
                } else {
                    return Err("Premature buffer end".into());
                }
            }
            _ => false,
        };

        if rm_pfx {
            prefix.simd_prefix = SimdPrefix::None;
        } else {
            prefix.lock = false;
            prefix.repe = false;
            prefix.repne = false;
        }

        trace!("tbl lookup: ({:?},{:?})",prefix.opcode_escape,prefix.simd_prefix);

        let b = match buf.get(i) {
            Some(b) => *b as usize,
            None => return Err("Premature buffer end".into()),
        };

        let opc = match (prefix.opcode_escape,prefix.simd_prefix) {
            (OpcodeEscape::None,_) if b == 0x63 && mode == Mode::Long =>
                Opcode::Binary(
                    MnemonicSpec::Single("movsxd"),
                    OpcodeOption::Only64,
                    semantic::movsxd,
                    OperandSpec::Present(AddressingMethod::G,OperandType::v),
                    OperandSpec::Present(AddressingMethod::E,OperandType::z),
                ),
            (OpcodeEscape::None,_) => ONEBYTE_TABLE[b].clone(),
            (OpcodeEscape::Escape0F,SimdPrefix::None) => TWOBYTE_TABLE[b].clone(),
            (OpcodeEscape::Escape0F,SimdPrefix::Prefix66) => {
                prefix.operand_size = match mode {
                    Mode::Real => 16,
                    Mode::Protected => 32,
                    Mode::Long => 64,
                };
                TWOBYTE_66_TABLE[b].clone()
            }
            (OpcodeEscape::Escape0F,SimdPrefix::PrefixF2) => TWOBYTE_F2_TABLE[b].clone(),
            (OpcodeEscape::Escape0F,SimdPrefix::PrefixF3) => TWOBYTE_F3_TABLE[b].clone(),
            (OpcodeEscape::Escape0F3A,SimdPrefix::None) => THREEBYTE_3A_TABLE[b].clone(),
            (OpcodeEscape::Escape0F3A,SimdPrefix::Prefix66) => {
                prefix.operand_size = match mode {
                    Mode::Real => 16,
                    Mode::Protected => 32,
                    Mode::Long => 64,
                };
                THREEBYTE_3A66_TABLE[b].clone()
            }
            (OpcodeEscape::Escape0F3A,SimdPrefix::PrefixF3) => return Err("Unknown instruction".into()),
            (OpcodeEscape::Escape0F3A,SimdPrefix::PrefixF2) => THREEBYTE_3AF2_TABLE[b].clone(),
            (OpcodeEscape::Escape0F38,SimdPrefix::None) => THREEBYTE_38_TABLE[b].clone(),
            (OpcodeEscape::Escape0F38,SimdPrefix::Prefix66) => {
                prefix.operand_size = match mode {
                    Mode::Real => 16,
                    Mode::Protected => 32,
                    Mode::Long => 64,
                };
                THREEBYTE_3866_TABLE[b].clone()
            }
            (OpcodeEscape::Escape0F38,SimdPrefix::PrefixF3) => THREEBYTE_38F3_TABLE[b].clone(),
            (OpcodeEscape::Escape0F38,SimdPrefix::PrefixF2) => THREEBYTE_38F2_TABLE[b].clone(),
            (OpcodeEscape::Xop8,SimdPrefix::None) => XOP8_TABLE[b].clone(),
            (OpcodeEscape::Xop8,_) => return Err("Unknown instruction".into()),
            (OpcodeEscape::Xop9,SimdPrefix::None) => XOP9_TABLE[b].clone(),
            (OpcodeEscape::Xop9,_) => return Err("Unknown instruction".into()),
            (OpcodeEscape::XopA,SimdPrefix::None) => XOPA_TABLE[b].clone(),
            (OpcodeEscape::XopA,_) => return Err("Unknown instruction".into()),
            (OpcodeEscape::Escape0F0F,_) => unreachable!(),
        };

        trace!("res: {:?}",opc);

        let opc = match opc.mnemonic() {
            &MnemonicSpec::Single(s) => {
                opc
            }
            &MnemonicSpec::Undefined => return Err("Unknown instruction".into()),
            &MnemonicSpec::Escape => {
                let (esc,modrm) = match (buf.get(i),buf.get(i + 1)) {
                    (Some(b1),Some(b2)) => (*b1 as usize,*b2 as usize),
                    _ => return Err("Premature buffer end".into()),
                };

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
            &MnemonicSpec::ModRM(grp) => {
                let pfx = if prefix.opcode_escape != OpcodeEscape::None {
                    prefix.simd_prefix
                } else {
                    SimdPrefix::None
                };
                let (opc,modrm) = match (buf.get(i),buf.get(i + 1)) {
                    (Some(b1),Some(b2)) => (*b1 as usize,*b2 as usize),
                    _ => return Err("Premature buffer end".into()),
                };

                try!(select_opcode_ext(grp,opc, modrm, pfx,mode,vexxop_present)).clone()
            }
        };

        match opc.option() {
            &OpcodeOption::Default64 if mode == Mode::Long && prefix.operand_size == 32 =>
                prefix.operand_size = 64,
            &OpcodeOption::Force64 if mode == Mode::Long =>
                prefix.operand_size = 64,
            &OpcodeOption::Only64 if mode != Mode::Long =>
                return Err("Unknown instruction".into()),
            &OpcodeOption::Invalid64 if mode == Mode::Long =>
                return Err("Unknown instruction".into()),
            _ => {}
        }

        trace!("prefix after fixup: {:?}",prefix);

        trace!("opcode len: {}",i + 1);

        match opc.mnemonic() {
            &MnemonicSpec::Single(s) => {
                use std::io::Cursor;

                let mut tail = Tail::new(Cursor::new(&buf[i+1..]));
                let rex = if rex_present {
                    Some((prefix.rex_w,prefix.rex_r,prefix.rex_x,prefix.rex_b))
                } else {
                    None
                };
                let ip = addr + i as u64 + 1;
                let mut stmts = vec![];
                let mut ops = vec![];

                for op in opc.operands().iter() {
                   let maybe_op = read_operand(op,&mut tail,mode,prefix.seg_override,prefix.vvvv,rex,
                                               prefix.operand_size,prefix.address_size,prefix.simd_size,ip).
                                  and_then(|x| to_rreil(x));

                   match maybe_op {
                       Ok((rv,mut st)) => {
                           stmts.append(&mut st);
                           ops.push(rv);
                       },
                       Err(e) => {
                           error!("error while decoding operands of '{}': {:?}",s,e);
                           return Err(e);
                       }
                   }
                }

                //if prefix.lock { print!("lock "); }
                //if prefix.repe { print!("repz "); }
                //if prefix.repne { print!("repnz "); }

                debug!("call {} with {:?}",s,ops);
                let res = opc.call(&ops.get(0).cloned(),
                                   &ops.get(1).cloned(),
                                   &ops.get(2).cloned(),
                                   &ops.get(3).cloned());
                let (mut op_stmts,jmp_spec) = match res {
                    Ok(o) => o,
                    Err(e) => {
                        error!("Semantic function for '{}' with {:?} failed: {}",s,ops,e);
                        return Err(e);
                    }
                };
                stmts.append(&mut op_stmts);

                let len = tail.fd.position() + i as u64 + 1;
                let mne = try!(match ops.len() {
                    0 => Mnemonic::new((addr..addr+len),format!("{}",s),"".to_string(),ops.iter(),stmts.iter()),
                    1 => Mnemonic::new((addr..addr+len),format!("{}",s),"{u}".to_string(),ops.iter(),stmts.iter()),
                    2 => Mnemonic::new((addr..addr+len),format!("{}",s),"{u}, {u}".to_string(),ops.iter(),stmts.iter()),
                    3 => Mnemonic::new((addr..addr+len),format!("{}",s),"{u}, {u}, {u}".to_string(),ops.iter(),stmts.iter()),
                    4 => Mnemonic::new((addr..addr+len),format!("{}",s),"{u}, {u}, {u}, {u}".to_string(),ops.iter(),stmts.iter()),
                    _ => unreachable!(),
                });
                let next = match jmp_spec {
                    JumpSpec::DeadEnd => vec![],
                    JumpSpec::FallThru => vec![(Rvalue::Constant{ value: addr + len, size: 64 },Guard::always())],
                    JumpSpec::Jump(ref v) => vec![(v.clone(),Guard::always())],
                    JumpSpec::Branch(ref v,ref g) => vec![
                        (Rvalue::Constant{ value: addr + len, size: 64 },Guard::always()),
                        (v.clone(),g.clone())
                    ],
                };


                debug!("'{:?}' with {} bytes",mne,len as usize);
                trace!("");
                Ok((len,mne,next))
            }
            e => Err("Internal error".into()),
        }
    }
}

fn to_rreil(op: Operand) -> Result<(Rvalue,Vec<Statement>)> {
    match op {
        Operand::Register(ref name) => Ok((Rvalue::Variable{ name: format!("{}",name).into(), size: name.width(), offset: 0, subscript: None },vec![])),
        Operand::Immediate(ref value,ref size) => Ok((Rvalue::Constant{ value: *value, size: *size },vec![])),
        Operand::Indirect(ref seg,ref base,ref index,ref scale,ref disp,ref width) => {
            let mut stmts = vec![];
            let mut ret = Rvalue::Undefined;

            if *base != Register::None {
                ret = Rvalue::Variable{ name: format!("{}",base).into(), size: base.width(), offset: 0, subscript: None };
            }

            if *scale > 0 && *index != Register::None {
                let s = *scale;
                let w = index.width();
                let rw = ret.size().unwrap_or(w);
                let i = Lvalue::Variable{ name: format!("{}",index).into(), size: w, subscript: None };
                if *base != Register::None {
                    stmts = try!(rreil!{
                        mul t:w, [s]:w, (i);
                        zext/rw t1:rw, t:w;
                        add (Lvalue::from_rvalue(ret.clone()).unwrap()), (ret), t1:rw;
                    });
                } else {
                    stmts = try!(rreil!{
                        mul t:w, (i), [s]:w;
                    });
                    ret = Rvalue::Variable{ name: "t".into(), size: w, offset: 0, subscript: None };
                }
            }

            if disp.0 > 0 {
                if ret != Rvalue::Undefined {
                    let d = Rvalue::Constant{ value: disp.0, size: disp.1 };
                    let w = ret.size().unwrap_or(disp.1);
                    stmts.append(&mut try!(rreil!{
                        zext/w d:w, (d);
                        add (Lvalue::from_rvalue(ret.clone()).unwrap()), (ret), d:w;
                    }));
                } else {
                    ret = Rvalue::Constant{ value: disp.0, size: disp.1 };
                }
            }

            let tgt = Lvalue::Variable{ name: format!("{}",op).into(), size: *width, subscript: None };

            stmts.append(&mut try!(rreil!{
                load/ram (tgt), (ret);
            }));
            ret = tgt.into();

            Ok((ret,stmts))
        },
        Operand::Optional => Ok((Rvalue::Undefined,vec![]))
    }
}
