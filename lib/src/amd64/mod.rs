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
                    error!("{} at {:x}",s,p);
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

#[derive(Clone,Debug)]
pub enum AddressingMethod {
    None,
    A, B, C, D, E, F, G, H, I, J, L, M, N, O, P, Q, R, S, U, V, W, X, Y
}

impl AddressingMethod {
    pub fn requires_modrm(&self) -> bool {
        match *self {
            AddressingMethod::None => false,
            AddressingMethod::A => false,
            AddressingMethod::B => true,
            AddressingMethod::C => true,
            AddressingMethod::D => true,
            AddressingMethod::E => true,
            AddressingMethod::F => true,
            AddressingMethod::G => true,
            AddressingMethod::H => true,
            AddressingMethod::I => false,
            AddressingMethod::J => false,
            AddressingMethod::L => true,
            AddressingMethod::M => true,
            AddressingMethod::N => true,
            AddressingMethod::O => false,
            AddressingMethod::P => true,
            AddressingMethod::Q => true,
            AddressingMethod::R => true,
            AddressingMethod::S => true,
            AddressingMethod::U => true,
            AddressingMethod::V => true,
            AddressingMethod::W => true,
            AddressingMethod::X => true,
            AddressingMethod::Y => true,
        }
    }

    pub fn requires_immediate(&self) -> bool {
        match *self {
            AddressingMethod::None => false,
            AddressingMethod::A => true,
            AddressingMethod::B => false,
            AddressingMethod::C => false,
            AddressingMethod::D => false,
            AddressingMethod::E => false,
            AddressingMethod::F => false,
            AddressingMethod::G => false,
            AddressingMethod::H => false,
            AddressingMethod::I => true,
            AddressingMethod::J => true,
            AddressingMethod::L => true,
            AddressingMethod::M => false,
            AddressingMethod::N => false,
            AddressingMethod::O => false,
            AddressingMethod::P => false,
            AddressingMethod::Q => false,
            AddressingMethod::R => false,
            AddressingMethod::S => false,
            AddressingMethod::U => false,
            AddressingMethod::V => false,
            AddressingMethod::W => false,
            AddressingMethod::X => false,
            AddressingMethod::Y => false,
        }
    }
}

#[derive(Clone,Debug)]
#[allow(non_camel_case_types)]
pub enum OperandType {
    None,
    a, b, c, d, dq, p, pd, pi, ps, q, qq, s, sd, ss, si, v, w, x, y, z,
    RAX, RBX, RCX, RDX, RDI, RSI, RSP, RBP,
    EAX, EBX, ECX, EDX, EDI, ESI, ESP, EBP,
    AX, BX, CX, DX, DI, SI, SP, BP,
    AL, BL, CL, DL,
    AH, BH, CH, DH,
    rAX, rBX, rCX, rDX, rDI, rSI, rSP, rBP,
    eAX, eBX, eCX, eDX, eDI, eSI, eSP, eBP,
    ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7,
    ES, GS, DS, SS, CS, FS,
    one,
    NTA, T0, T1, T2,
}

impl OperandType {
    pub fn to_rreil(&self,opsz: usize) -> Result<Rvalue> {
        match *self {
            OperandType::RAX => Ok(rreil_rvalue!{ RAX:64 }),
            OperandType::RBX => Ok(rreil_rvalue!{ RBX:64 }),
            OperandType::RCX => Ok(rreil_rvalue!{ RCX:64 }),
            OperandType::RDX => Ok(rreil_rvalue!{ RDX:64 }),
            OperandType::RDI => Ok(rreil_rvalue!{ RDI:64 }),
            OperandType::RSI => Ok(rreil_rvalue!{ RSI:64 }),
            OperandType::RSP => Ok(rreil_rvalue!{ RSP:64 }),
            OperandType::RBP => Ok(rreil_rvalue!{ RBP:64 }),

            OperandType::EAX => Ok(rreil_rvalue!{ EAX:32 }),
            OperandType::EBX => Ok(rreil_rvalue!{ EBX:32 }),
            OperandType::ECX => Ok(rreil_rvalue!{ ECX:32 }),
            OperandType::EDX => Ok(rreil_rvalue!{ EDX:32 }),
            OperandType::EDI => Ok(rreil_rvalue!{ EDI:32 }),
            OperandType::ESI => Ok(rreil_rvalue!{ ESI:32 }),
            OperandType::ESP => Ok(rreil_rvalue!{ ESP:32 }),
            OperandType::EBP => Ok(rreil_rvalue!{ EBP:32 }),

            OperandType::AX => Ok(rreil_rvalue!{ AX:16 }),
            OperandType::BX => Ok(rreil_rvalue!{ BX:16 }),
            OperandType::CX => Ok(rreil_rvalue!{ CX:16 }),
            OperandType::DX => Ok(rreil_rvalue!{ DX:16 }),
            OperandType::DI => Ok(rreil_rvalue!{ DI:16 }),
            OperandType::SI => Ok(rreil_rvalue!{ SI:16 }),
            OperandType::SP => Ok(rreil_rvalue!{ SP:16 }),
            OperandType::BP => Ok(rreil_rvalue!{ BP:16 }),

            OperandType::AL => Ok(rreil_rvalue!{ AL:8 }),
            OperandType::BL => Ok(rreil_rvalue!{ BL:8 }),
            OperandType::CL => Ok(rreil_rvalue!{ CL:8 }),
            OperandType::DL => Ok(rreil_rvalue!{ DL:8 }),

            OperandType::AH => Ok(rreil_rvalue!{ AH:8 }),
            OperandType::BH => Ok(rreil_rvalue!{ BH:8 }),
            OperandType::CH => Ok(rreil_rvalue!{ CH:8 }),
            OperandType::DH => Ok(rreil_rvalue!{ DH:8 }),

            OperandType::ES => Ok(rreil_rvalue!{ ES:16 }),
            OperandType::FS => Ok(rreil_rvalue!{ FS:16 }),
            OperandType::GS => Ok(rreil_rvalue!{ GS:16 }),
            OperandType::SS => Ok(rreil_rvalue!{ SS:16 }),
            OperandType::CS => Ok(rreil_rvalue!{ CS:16 }),

            OperandType::ST0 => Ok(rreil_rvalue!{ ST0:80 }),
            OperandType::ST1 => Ok(rreil_rvalue!{ ST1:80 }),
            OperandType::ST2 => Ok(rreil_rvalue!{ ST2:80 }),
            OperandType::ST3 => Ok(rreil_rvalue!{ ST3:80 }),
            OperandType::ST4 => Ok(rreil_rvalue!{ ST4:80 }),
            OperandType::ST5 => Ok(rreil_rvalue!{ ST5:80 }),
            OperandType::ST6 => Ok(rreil_rvalue!{ ST6:80 }),
            OperandType::ST7 => Ok(rreil_rvalue!{ ST7:80 }),

            OperandType::rAX if opsz == 64 => Ok(rreil_rvalue!{ RAX:64 }),
            OperandType::rBX if opsz == 64 => Ok(rreil_rvalue!{ RBX:64 }),
            OperandType::rCX if opsz == 64 => Ok(rreil_rvalue!{ RCX:64 }),
            OperandType::rDX if opsz == 64 => Ok(rreil_rvalue!{ RDX:64 }),
            OperandType::rDI if opsz == 64 => Ok(rreil_rvalue!{ RDI:64 }),
            OperandType::rSI if opsz == 64 => Ok(rreil_rvalue!{ RSI:64 }),

            OperandType::rAX | OperandType::eAX if opsz == 32 => Ok(rreil_rvalue!{ EAX:32 }),
            OperandType::rBX | OperandType::eBX if opsz == 32 => Ok(rreil_rvalue!{ EBX:32 }),
            OperandType::rCX | OperandType::eCX if opsz == 32 => Ok(rreil_rvalue!{ ECX:32 }),
            OperandType::rDX | OperandType::eDX if opsz == 32 => Ok(rreil_rvalue!{ EDX:32 }),
            OperandType::rDI | OperandType::eDI if opsz == 32 => Ok(rreil_rvalue!{ EDI:32 }),
            OperandType::rSI | OperandType::eSI if opsz == 32 => Ok(rreil_rvalue!{ ESI:32 }),

            OperandType::rAX | OperandType::eAX if opsz == 16 => Ok(rreil_rvalue!{ AX:16 }),
            OperandType::rBX | OperandType::eBX if opsz == 16 => Ok(rreil_rvalue!{ BX:16 }),
            OperandType::rCX | OperandType::eCX if opsz == 16 => Ok(rreil_rvalue!{ CX:16 }),
            OperandType::rDX | OperandType::eDX if opsz == 16 => Ok(rreil_rvalue!{ DX:16 }),
            OperandType::rDI | OperandType::eDI if opsz == 16 => Ok(rreil_rvalue!{ DI:16 }),
            OperandType::rSI | OperandType::eSI if opsz == 16 => Ok(rreil_rvalue!{ SI:16 }),

            _ => Err("Invalid OperandType value".into()),
        }
    }
}

#[derive(Clone,Debug)]
pub enum Operand {
    None,
    Present(AddressingMethod,OperandType),
}

impl Operand {
    pub fn requires_modrm(&self) -> bool {
        match self {
            &Operand::None => false,
            &Operand::Present(ref a,_) => a.requires_modrm(),
        }
    }

    pub fn bit_size(&self, opsz: usize) -> usize {
        use std::cmp;

        match self {
            &Operand::None => 0,
            &Operand::Present(_,ref o) => match *o {
                OperandType::None => 0,

                OperandType::a => 2 * opsz,
                OperandType::b => 8,
                OperandType::c => if opsz == 16 { 8 } else { 16 },
                OperandType::d => 64,
                OperandType::dq => 128,
                OperandType::p => opsz,
                OperandType::pd => 256,
                OperandType::pi => 128,
                OperandType::ps => 256,
                OperandType::q => 64,
                OperandType::qq => 256,
                OperandType::s => 80,
                OperandType::sd => 128,
                OperandType::ss => 128,
                OperandType::si => 32,
                OperandType::v => opsz,
                OperandType::w => 16,
                OperandType::x => if opsz == 32 { 128 } else { 256 },
                OperandType::y => opsz,
                OperandType::z => cmp::min(32,opsz),

                OperandType::RAX | OperandType::RBX => 64,
                OperandType::RCX | OperandType::RDX => 64,
                OperandType::RDI | OperandType::RSI => 64,
                OperandType::RSP | OperandType::RBP => 64,

                OperandType::EAX | OperandType::EBX => 32,
                OperandType::ECX | OperandType::EDX => 32,
                OperandType::EDI | OperandType::ESI => 32,
                OperandType::ESP | OperandType::EBP => 32,

                OperandType::AX | OperandType::BX => 16,
                OperandType::CX | OperandType::DX => 16,
                OperandType::DI | OperandType::SI => 16,
                OperandType::SP | OperandType::BP => 16,

                OperandType::AL | OperandType::BL => 8,
                OperandType::CL | OperandType::DL => 8,

                OperandType::AH | OperandType::BH => 8,
                OperandType::CH | OperandType::DH => 8,

                OperandType::rAX | OperandType::rBX => cmp::min(64,opsz),
                OperandType::rCX | OperandType::rDX => cmp::min(64,opsz),
                OperandType::rDI | OperandType::rSI => cmp::min(64,opsz),
                OperandType::rSP | OperandType::rBP => cmp::min(64,opsz),

                OperandType::eAX | OperandType::eBX => cmp::min(32,opsz),
                OperandType::eCX | OperandType::eDX => cmp::min(32,opsz),
                OperandType::eDI | OperandType::eSI => cmp::min(32,opsz),
                OperandType::eSP | OperandType::eBP => cmp::min(32,opsz),

                OperandType::ST0 | OperandType::ST1 => 80,
                OperandType::ST2 | OperandType::ST3 => 80,
                OperandType::ST4 | OperandType::ST5 => 80,
                OperandType::ST6 | OperandType::ST7 => 80,

                OperandType::ES | OperandType::GS => 16,
                OperandType::DS | OperandType::SS => 16,
                OperandType::CS | OperandType::FS => 16,

                OperandType::one => 0,

                OperandType::NTA | OperandType::T0 => 0,
                OperandType::T1 | OperandType::T2 => 0,
            }
        }
    }

    pub fn requires_immediate(&self) -> bool {
        match self {
            &Operand::None => false,
            &Operand::Present(ref a,_) => a.requires_immediate(),
        }
    }

    pub fn to_rreil(&self, mode: Mode,
                    mod_: Option<u8>, reg: Option<u8>, rm: Option<u8>,
                    scale: Option<u8>, index: Option<u8>, base: Option<u8>,
                    vvvv: Option<u8>,
                    disp8: Option<u8>, disp32: Option<u32>, rex_present: bool, imm8: Option<u8>,
                    imm16: Option<u16>, imm32: Option<u32>, imm48: Option<u64>, imm64: Option<u64>,
                    opsz: usize, addrsz: usize, simdsz: usize, addr: u64) -> Option<(Rvalue,Vec<Statement>)> {
        match (self,opsz) {
            (&Operand::None,_) => None,
            (&Operand::Present(AddressingMethod::None,ref reg),_) => reg.to_rreil(opsz).ok().map(|x| (x,vec![])),
            (&Operand::Present(AddressingMethod::A,OperandType::v),16) if imm16.is_some() =>
                Some((Rvalue::Constant{ value: imm16.unwrap() as u64, size: 16 },vec![])),
            (&Operand::Present(AddressingMethod::A,OperandType::v),32) if imm16.is_some() =>
                Some((Rvalue::Constant{ value: imm32.unwrap() as u64, size: 32 },vec![])),
            (&Operand::Present(AddressingMethod::A,OperandType::v),64) if imm64.is_some() =>
                Some((Rvalue::Constant{ value: imm64.unwrap() as u64, size: 64 },vec![])),
            (&Operand::Present(AddressingMethod::A,OperandType::p),16) if imm32.is_some() =>
                Some((Rvalue::Constant{ value: imm32.unwrap() as u64, size: 32 },vec![])),
            (&Operand::Present(AddressingMethod::A,OperandType::p),32) if imm48.is_some() =>
                Some((Rvalue::Constant{ value: imm48.unwrap() as u64, size: 48 },vec![])),
            (&Operand::Present(AddressingMethod::A,OperandType::p),64) if imm64.is_some() =>
                // XXX
                Some((Rvalue::Constant{ value: imm64.unwrap() as u64, size: 64 },vec![])),
            (&Operand::Present(AddressingMethod::B,OperandType::y),opsz) if vvvv.is_some() =>
                read_register(vvvv.unwrap(),rex_present,cmp::max(32,opsz)).ok(),
            (&Operand::Present(AddressingMethod::C,OperandType::d),_) =>
                read_ctrl_register(reg.unwrap(),32).ok(),
            (&Operand::Present(AddressingMethod::D,OperandType::d),_) =>
                read_debug_register(reg.unwrap(),32).ok(),
            (&Operand::Present(AddressingMethod::E,OperandType::v),opsz) if rm.is_some() && mod_.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::E,OperandType::y),opsz) if rm.is_some() && mod_.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,cmp::max(32,opsz),addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::E,OperandType::b),_) if rm.is_some() && mod_.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,8,addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::E,OperandType::w),_) if rm.is_some() && mod_.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,16,addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::E,OperandType::d),_) if rm.is_some() && mod_.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,32,addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::E,OperandType::dq),_) if rm.is_some() && mod_.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,64,addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::dq),_) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::d),_) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,32).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::w),_) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,16).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::b),_) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,8).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::v),opsz) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,opsz).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::z),opsz) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,cmp::min(32,opsz)).ok(),
            (&Operand::Present(AddressingMethod::G,OperandType::y),opsz) if reg.is_some() =>
                read_register(reg.unwrap(),rex_present,cmp::max(32,opsz)).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::x),opsz) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,opsz).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::qq),_) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,256).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::dq),_) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::ps),_) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::pd),_) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::ss),_) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::H,OperandType::sd),_) if vvvv.is_some() =>
                read_simd_register(vvvv.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::H,_),_) if vvvv.is_none() =>
                None,
            (&Operand::Present(AddressingMethod::I,OperandType::z),16) if imm16.is_some() =>
                Some((Rvalue::Constant{ value: imm16.unwrap() as u64, size: 16 },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::z),_) if imm32.is_some() =>
                Some((Rvalue::Constant{ value: imm32.unwrap() as u64, size: 32 },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::b),_) if imm8.is_some() =>
                Some((Rvalue::Constant{ value: imm8.unwrap() as u64, size: opsz },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::one),opsz) =>
                Some((Rvalue::Constant{ value: 1, size: opsz },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::w),_) if imm16.is_some() =>
                Some((Rvalue::Constant{ value: imm16.unwrap() as u64, size: 16 },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::v),16) if imm16.is_some() =>
                Some((Rvalue::Constant{ value: imm16.unwrap() as u64, size: 16 },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::v),32) if imm32.is_some() =>
                Some((Rvalue::Constant{ value: imm32.unwrap() as u64, size: 32 },vec![])),
            (&Operand::Present(AddressingMethod::I,OperandType::v),64) if imm64.is_some() =>
                Some((Rvalue::Constant{ value: imm64.unwrap() as u64, size: 64 },vec![])),
            (&Operand::Present(AddressingMethod::J,OperandType::b),_) if imm8.is_some() =>
                Some((Rvalue::Constant{ value: addr + imm8.unwrap() as u64, size: addrsz },vec![])),
            (&Operand::Present(AddressingMethod::J,OperandType::z),16) if imm16.is_some() =>
                Some((Rvalue::Constant{ value: addr + imm16.unwrap() as u64, size: addrsz },vec![])),
            (&Operand::Present(AddressingMethod::J,OperandType::z),_) if imm32.is_some() =>
                Some((Rvalue::Constant{ value: addr + imm32.unwrap() as u64, size: addrsz },vec![])),
            (&Operand::Present(AddressingMethod::L,OperandType::x),32) if imm8.is_some() =>
                read_simd_register(imm8.unwrap() & 0b0111,rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::L,OperandType::x),_) if imm8.is_some() =>
                read_simd_register(imm8.unwrap() & 0b1111,rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::p),16) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,16,addrsz,addr,32).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::p),32) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,32,addrsz,addr,48).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::p),64) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,64,addrsz,addr,80).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::w),opsz) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,16).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::d),opsz) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,32).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::q),opsz) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,64).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::s),64) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,80).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::s),_) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,48).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::b),_) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,8).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::None),opsz) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,opsz).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::a),32) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,64).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::a),16) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,32).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::y),opsz) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,cmp::min(32,opsz)).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::x),32) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,128).ok(),
            (&Operand::Present(AddressingMethod::M,OperandType::x),64) if mod_.is_some() && rm.is_some() =>
                read_effective_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,256).ok(),
            (&Operand::Present(AddressingMethod::N,OperandType::q),_) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 16 && imm16.is_some() =>
                read_memory(Rvalue::Constant{ value: imm16.unwrap() as u64, size: addrsz },addrsz,8).ok(),
            (&Operand::Present(AddressingMethod::O,OperandType::b),_) if addrsz == 32 && imm32.is_some() =>
                read_memory(Rvalue::Constant{ value: imm32.unwrap() as u64, size: addrsz },addrsz,8).ok(),
            (&Operand::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 16 && imm16.is_some() =>
                read_memory(Rvalue::Constant{ value: imm16.unwrap() as u64, size: addrsz },addrsz,opsz).ok(),
            (&Operand::Present(AddressingMethod::O,OperandType::v),opsz) if addrsz == 32 && imm32.is_some() =>
                read_memory(Rvalue::Constant{ value: imm32.unwrap() as u64, size: addrsz },addrsz,opsz).ok(),
            (&Operand::Present(AddressingMethod::P,OperandType::pi),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::P,OperandType::ps),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::P,OperandType::q),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::P,OperandType::d),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,32).ok(),
            (&Operand::Present(AddressingMethod::Q,OperandType::d),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,32).ok(),
            (&Operand::Present(AddressingMethod::Q,OperandType::pi),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,simdsz).ok(),
            (&Operand::Present(AddressingMethod::Q,OperandType::q),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,32).ok(),
            (&Operand::Present(AddressingMethod::S,OperandType::w),_) if rm.is_some() =>
                read_memory(Rvalue::Constant{ value: imm16.unwrap() as u64, size: addrsz },addrsz,16).ok(),
            (&Operand::Present(AddressingMethod::R,OperandType::d),_) if rm.is_some() =>
                read_memory(Rvalue::Constant{ value: imm16.unwrap() as u64, size: addrsz },addrsz,32).ok(),
            (&Operand::Present(AddressingMethod::R,OperandType::q),_) if rm.is_some() =>
                read_memory(Rvalue::Constant{ value: imm16.unwrap() as u64, size: addrsz },addrsz,64).ok(),
            (&Operand::Present(AddressingMethod::U,OperandType::ps),_) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::U,OperandType::pi),_) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::U,OperandType::q),_) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::U,OperandType::x),32) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::U,OperandType::x),64) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,256).ok(),
            (&Operand::Present(AddressingMethod::U,OperandType::dq),_) if rm.is_some() =>
                read_simd_register(rm.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::pi),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::ps),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::pd),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,simdsz).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::ss),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::x),32) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::x),64) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,256).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::dq),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::q),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,64).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::sd),_) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,128).ok(),
            (&Operand::Present(AddressingMethod::V,OperandType::y),opsz) if reg.is_some() =>
                read_simd_register(reg.unwrap(),rex_present,cmp::min(32,opsz)).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::pd),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,simdsz).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::ps),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,simdsz).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::q),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,64).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::dq),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,128).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::x),32) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,128).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::x),64) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,256).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::sd),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,128).ok(),
            (&Operand::Present(AddressingMethod::W,OperandType::ss),_) if rm.is_some() && mod_.is_some() =>
                read_effective_simd_address(mod_.unwrap(),rex_present,rm.unwrap(),scale,index,base,disp8,disp32,opsz,addrsz,addr,128).ok(),
            _ => {
                println!("can't decode {:?}/{}",self,opsz);
                unreachable!();
            }
        }
    }
}

fn read_effective_simd_address(mod_: u8,rex_present: bool,rm: u8,
                               scale: Option<u8>,index: Option<u8>,base: Option<u8>,
                               disp8: Option<u8>,disp32: Option<u32>,
                               opsz: usize, addrsz: usize, ip: u64, simdsz: usize) -> Result<(Rvalue,Vec<Statement>)> {
    match (mod_,rm & 0b111) {
        // mod = 00
        (0b00,0b000) | (0b00,0b001) | (0b00,0b010) |
        (0b00,0b011) | (0b00,0b110) | (0b00,0b111) => {
            let (reg,mut stmts1) = try!(read_register(mod_,rex_present,opsz));
            let (ptr,mut stmts2) = try!(read_memory(reg,addrsz,simdsz));
            stmts1.append(&mut stmts2);
            Ok((ptr,stmts1))
        }
        (0b00,0b100) if scale.is_some() && index.is_some() && base.is_some() => {
            let (val,mut stmts1) = try!(read_sib(mod_,rex_present,scale.unwrap(),index.unwrap(),base.unwrap(),disp8,disp32,opsz));
            let (ptr,mut stmts2) = try!(read_memory(val,addrsz,simdsz));
            stmts1.append(&mut stmts2);
            Ok((ptr,stmts1))
        }
        (0b00,0b101) if opsz == 64 && disp32.is_some() =>
            read_memory(Rvalue::Constant{ value: disp32.unwrap() as u64 + ip, size: addrsz },addrsz,64),
        (0b00,0b101) if opsz != 64 && disp32.is_some() =>
            Ok((Rvalue::Constant{ value: disp32.unwrap() as u64, size: simdsz },vec![])),

        // mod = 01
        (0b01,0b000) | (0b01,0b001) | (0b01,0b010) | (0b01,0b011) |
        (0b01,0b101) | (0b01,0b110) | (0b01,0b111) if disp8.is_some() => {
            let (arg,mut stmts) = try!(read_register(mod_,rex_present,opsz));
            let d = disp8.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/simdsz t:simdsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:simdsz },stmts))
        }
        (0b01,0b100) if scale.is_some() && index.is_some() && base.is_some() && disp8.is_some() => {
            let (arg,mut stmts) = try!(read_sib(mod_,rex_present,scale.unwrap(),index.unwrap(),base.unwrap(),disp8,disp32,opsz));
            let d = disp8.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/simdsz t:simdsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:simdsz },stmts))
        }

        // mod = 10
        (0b10,0b000) | (0b10,0b001) | (0b10,0b010) | (0b10,0b011) |
        (0b10,0b101) | (0b10,0b110) | (0b10,0b111) if disp32.is_some() => {
            let (arg,mut stmts) = try!(read_register(mod_,rex_present,opsz));
            let d = disp32.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/simdsz t:simdsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:simdsz },stmts))
        }
        (0b10,0b100) if scale.is_some() && index.is_some() && base.is_some() && disp32.is_some() => {
            let (arg,mut stmts) = try!(read_sib(mod_,rex_present,scale.unwrap(),index.unwrap(),base.unwrap(),disp8,disp32,opsz));
            let d = disp32.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/simdsz t:simdsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:simdsz },stmts))
        }

        // mod = 11
        (0b11,_) => read_simd_register(rm,rex_present,simdsz),

        _ => Err("Invalid mod value".into()),
    }
}

fn read_effective_address(mod_: u8,rex_present: bool,rm: u8,
                          scale: Option<u8>,index: Option<u8>,base: Option<u8>,
                          disp8: Option<u8>,disp32: Option<u32>,
                          opsz: usize, addrsz: usize, ip: u64, width: usize) -> Result<(Rvalue,Vec<Statement>)> {
    match (mod_,rm & 0b111) {
       // mod = 00
        (0b00,0b000) | (0b00,0b001) | (0b00,0b010) |
        (0b00,0b011) | (0b00,0b110) | (0b00,0b111) => {
            let (reg,mut stmts1) = try!(read_register(mod_,rex_present,opsz));
            let (ptr,mut stmts2) = try!(read_memory(reg,addrsz,opsz));
            stmts1.append(&mut stmts2);
            Ok((ptr,stmts1))
        }
        (0b00,0b100) if scale.is_some() && index.is_some() && base.is_some() => {
            let (val,mut stmts1) = try!(read_sib(mod_,rex_present,scale.unwrap(),index.unwrap(),base.unwrap(),disp8,disp32,opsz));
            let (ptr,mut stmts2) = try!(read_memory(val,addrsz,opsz));
            stmts1.append(&mut stmts2);
            Ok((ptr,stmts1))
        }
        (0b00,0b101) if opsz == 64 && disp32.is_some() =>
            read_memory(Rvalue::Constant{ value: disp32.unwrap() as u64 + ip, size: addrsz },addrsz,64),
        (0b00,0b101) if opsz != 64 && disp32.is_some() =>
            Ok((Rvalue::Constant{ value: disp32.unwrap() as u64, size: opsz },vec![])),

        // mod = 01
        (0b01,0b000) | (0b01,0b001) | (0b01,0b010) | (0b01,0b011) |
        (0b01,0b101) | (0b01,0b110) | (0b01,0b111) if disp8.is_some() => {
            let (arg,mut stmts) = try!(read_register(mod_,rex_present,opsz));
            let d = disp8.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/opsz t:opsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:opsz },stmts))
        }
        (0b01,0b100) if scale.is_some() && index.is_some() && base.is_some() && disp8.is_some() => {
            let (arg,mut stmts) = try!(read_sib(mod_,rex_present,scale.unwrap(),index.unwrap(),base.unwrap(),disp8,disp32,opsz));
            let d = disp8.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/opsz t:opsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:opsz },stmts))
        }

        // mod = 10
        (0b10,0b000) | (0b10,0b001) | (0b10,0b010) | (0b10,0b011) |
        (0b10,0b101) | (0b10,0b110) | (0b10,0b111) if disp32.is_some() => {
            let (arg,mut stmts) = try!(read_register(mod_,rex_present,opsz));
            let d = disp32.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/opsz t:opsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:opsz },stmts))
        }
        (0b10,0b100) if scale.is_some() && index.is_some() && base.is_some() && disp32.is_some() => {
            let (arg,mut stmts) = try!(read_sib(mod_,rex_present,scale.unwrap(),index.unwrap(),base.unwrap(),disp8,disp32,opsz));
            let d = disp32.unwrap();
            stmts.append(&mut rreil!{
                add t:opsz, (arg), [d]:opsz;
                load/opsz t:opsz, t:opsz;
            });
            Ok((rreil_rvalue!{ t:opsz },stmts))
        }

        // mod = 11
        (0b11,_) => read_register(rm,rex_present,opsz),

        _ => Err("Invalid mod value".into()),
    }
}

fn read_memory(ptr: Rvalue, addrsz: usize, width: usize) -> Result<(Rvalue,Vec<Statement>)> {
    let mut stmts = if ptr.size().unwrap_or(addrsz) != addrsz {
        rreil!{
            zext/addrsz t:addrsz, (ptr);
        }
    } else {
        rreil!{
            mov t:addrsz, (ptr);
        }
    };
    stmts.append(&mut rreil!{
        load/width s:width, t:addrsz;
    });

    Ok((rreil_rvalue!{ s:width },stmts))
}

fn read_sib(mod_: u8, rex_present: bool, scale: u8, index: u8, base: u8,
            disp8: Option<u8>, disp32: Option<u32>, opsz: usize) -> Result<(Rvalue,Vec<Statement>)> {
    let mut stmts = vec![];
    let s = (1 << scale) >> 1;
    let ss = if scale == 0b100 {
        rreil_rvalue!{ [0]:opsz }
    } else {
        let (t,mut stmts1) = try!(read_register(scale,rex_present,opsz));
        stmts.append(&mut stmts1);
        stmts.append(&mut rreil!{
            mul t:opsz, (t), [s]:opsz;
        });
        rreil_rvalue!{ t:opsz }
    };
    let b = if base == 0b101 {
        match mod_ {
            0b00 if disp32.is_some() => Rvalue::Constant{ value: disp32.unwrap() as u64, size: opsz },
            0b01 if disp8.is_some() => {
                let (bp,mut st) = try!(read_register(0b101,rex_present,opsz));
                let d = disp8.unwrap();
                stmts.append(&mut st);
                stmts.append(&mut rreil!{
                    zext/opsz t:opsz, [d]:opsz;
                    load/opsz x:opsz, (bp);
                    add s:opsz, t:opsz, x:opsz;
                });
                rreil_rvalue!{ s:opsz }
            }
            0b10 if disp32.is_some() => {
                let (bp,mut st) = try!(read_register(0b101,rex_present,opsz));
                let d = disp32.unwrap();
                stmts.append(&mut st);
                stmts.append(&mut rreil!{
                    zext/opsz t:opsz, [d]:opsz;
                    load/opsz x:opsz, (bp);
                    add s:opsz, t:opsz, x:opsz;
                });
                rreil_rvalue!{ s:opsz }
            }

            _ => return Err("Invalid mod value".into()),
        }
    } else {
        let (r,mut st) = try!(read_register(base,rex_present,opsz));
        stmts.append(&mut st);
        r
    };

    stmts.append(&mut rreil!{
        add x:opsz, (ss), (b);
        load/opsz d:opsz, x:opsz;
    });
    Ok((rreil_rvalue!{ d:opsz },stmts))
}

fn read_register(reg: u8, rex_present: bool, opsz: usize) -> Result<(Rvalue,Vec<Statement>)> {
    let ret = match (reg,opsz) {
        (0b0000,8) => rreil_rvalue!{ AL:8 },
        (0b0001,8) => rreil_rvalue!{ CL:8 },
        (0b0010,8) => rreil_rvalue!{ DL:8 },
        (0b0011,8) => rreil_rvalue!{ BL:8 },
        (0b0100,8) => if rex_present { rreil_rvalue!{ AH:8 } } else { rreil_rvalue!{ SPL:8 } },
        (0b0101,8) => if rex_present { rreil_rvalue!{ CH:8 } } else { rreil_rvalue!{ BPL:8 } },
        (0b0110,8) => if rex_present { rreil_rvalue!{ DH:8 } } else { rreil_rvalue!{ SIL:8 } },
        (0b0111,8) => if rex_present { rreil_rvalue!{ BH:8 } } else { rreil_rvalue!{ DIL:8 } },
        (0b1000,8) => rreil_rvalue!{ R8L:8 },
        (0b1001,8) => rreil_rvalue!{ R9L:8 },
        (0b1010,8) => rreil_rvalue!{ R10L:8 },
        (0b1011,8) => rreil_rvalue!{ R11L:8 },
        (0b1100,8) => rreil_rvalue!{ R12L:8 },
        (0b1101,8) => rreil_rvalue!{ R13L:8 },
        (0b1110,8) => rreil_rvalue!{ R14L:8 },
        (0b1111,8) => rreil_rvalue!{ R15L:8 },

        (0b0000,16) => rreil_rvalue!{ AX:16 },
        (0b0001,16) => rreil_rvalue!{ CX:16 },
        (0b0010,16) => rreil_rvalue!{ DX:16 },
        (0b0011,16) => rreil_rvalue!{ BX:16 },
        (0b0100,16) => rreil_rvalue!{ SP:16 },
        (0b0101,16) => rreil_rvalue!{ BP:16 },
        (0b0110,16) => rreil_rvalue!{ SI:16 },
        (0b0111,16) => rreil_rvalue!{ DI:16 },
        (0b1000,16) => rreil_rvalue!{ R8W:16 },
        (0b1001,16) => rreil_rvalue!{ R9W:16 },
        (0b1010,16) => rreil_rvalue!{ R10W:16 },
        (0b1011,16) => rreil_rvalue!{ R11W:16 },
        (0b1100,16) => rreil_rvalue!{ R12W:16 },
        (0b1101,16) => rreil_rvalue!{ R13W:16 },
        (0b1110,16) => rreil_rvalue!{ R14W:16 },
        (0b1111,16) => rreil_rvalue!{ R15W:16 },

        (0b0000,32) => rreil_rvalue!{ EAX:32 },
        (0b0001,32) => rreil_rvalue!{ ECX:32 },
        (0b0010,32) => rreil_rvalue!{ EDX:32 },
        (0b0011,32) => rreil_rvalue!{ EBX:32 },
        (0b0100,32) => rreil_rvalue!{ ESP:32 },
        (0b0101,32) => rreil_rvalue!{ EBP:32 },
        (0b0110,32) => rreil_rvalue!{ ESI:32 },
        (0b0111,32) => rreil_rvalue!{ EDI:32 },
        (0b1000,32) => rreil_rvalue!{ R8D:32 },
        (0b1001,32) => rreil_rvalue!{ R9D:32 },
        (0b1010,32) => rreil_rvalue!{ R10D:32 },
        (0b1011,32) => rreil_rvalue!{ R11D:32 },
        (0b1100,32) => rreil_rvalue!{ R12D:32 },
        (0b1101,32) => rreil_rvalue!{ R13D:32 },
        (0b1110,32) => rreil_rvalue!{ R14D:32 },
        (0b1111,32) => rreil_rvalue!{ R15D:32 },

        (0b0000,64) => rreil_rvalue!{ RAX:64 },
        (0b0001,64) => rreil_rvalue!{ RCX:64 },
        (0b0010,64) => rreil_rvalue!{ RDX:64 },
        (0b0011,64) => rreil_rvalue!{ RBX:64 },
        (0b0100,64) => rreil_rvalue!{ RSP:64 },
        (0b0101,64) => rreil_rvalue!{ RBP:64 },
        (0b0110,64) => rreil_rvalue!{ RSI:64 },
        (0b0111,64) => rreil_rvalue!{ RDI:64 },
        (0b1000,64) => rreil_rvalue!{ R8:64 },
        (0b1001,64) => rreil_rvalue!{ R9:64 },
        (0b1010,64) => rreil_rvalue!{ R10:64 },
        (0b1011,64) => rreil_rvalue!{ R11:64 },
        (0b1100,64) => rreil_rvalue!{ R12:64 },
        (0b1101,64) => rreil_rvalue!{ R13:64 },
        (0b1110,64) => rreil_rvalue!{ R14:64 },
        (0b1111,64) => rreil_rvalue!{ R15:64 },

        (0b0000,80) => rreil_rvalue!{ ST0:80 },
        (0b0001,80) => rreil_rvalue!{ ST1:80 },
        (0b0010,80) => rreil_rvalue!{ ST2:80 },
        (0b0011,80) => rreil_rvalue!{ ST3:80 },
        (0b0100,80) => rreil_rvalue!{ ST4:80 },
        (0b0101,80) => rreil_rvalue!{ ST5:80 },
        (0b0110,80) => rreil_rvalue!{ ST6:80 },
        (0b0111,80) => rreil_rvalue!{ ST7:80 },

        _ => return Err("Invalid reg value".into()),
    };
    Ok((ret,vec![]))
}
fn read_simd_register(reg: u8, rex_present: bool, opsz: usize) -> Result<(Rvalue,Vec<Statement>)> {
    let ret = match (reg,opsz) {
       (0b0000,64) => rreil_rvalue!{ MMX0:64 },
       (0b0001,64) => rreil_rvalue!{ MMX1:64 },
       (0b0010,64) => rreil_rvalue!{ MMX2:64 },
       (0b0011,64) => rreil_rvalue!{ MMX3:64 },
       (0b0100,64) => rreil_rvalue!{ MMX4:64 },
       (0b0101,64) => rreil_rvalue!{ MMX5:64 },
       (0b0110,64) => rreil_rvalue!{ MMX6:64 },
       (0b0111,64) => rreil_rvalue!{ MMX7:64 },
       (0b1000,64) => rreil_rvalue!{ MMX0:64 },
       (0b1001,64) => rreil_rvalue!{ MMX1:64 },
       (0b1010,64) => rreil_rvalue!{ MMX2:64 },
       (0b1011,64) => rreil_rvalue!{ MMX3:64 },
       (0b1100,64) => rreil_rvalue!{ MMX4:64 },
       (0b1101,64) => rreil_rvalue!{ MMX5:64 },
       (0b1110,64) => rreil_rvalue!{ MMX6:64 },
       (0b1111,64) => rreil_rvalue!{ MMX7:64 },

       (0b0000,128) => rreil_rvalue!{ XMM0:128 },
       (0b0001,128) => rreil_rvalue!{ XMM1:128 },
       (0b0010,128) => rreil_rvalue!{ XMM2:128 },
       (0b0011,128) => rreil_rvalue!{ XMM3:128 },
       (0b0100,128) => rreil_rvalue!{ XMM4:128 },
       (0b0101,128) => rreil_rvalue!{ XMM5:128 },
       (0b0110,128) => rreil_rvalue!{ XMM6:128 },
       (0b0111,128) => rreil_rvalue!{ XMM7:128 },
       (0b1000,128) => rreil_rvalue!{ XMM8:128 },
       (0b1001,128) => rreil_rvalue!{ XMM9:128 },
       (0b1010,128) => rreil_rvalue!{ XMM10:128 },
       (0b1011,128) => rreil_rvalue!{ XMM11:128 },
       (0b1100,128) => rreil_rvalue!{ XMM12:128 },
       (0b1101,128) => rreil_rvalue!{ XMM13:128 },
       (0b1110,128) => rreil_rvalue!{ XMM14:128 },
       (0b1111,128) => rreil_rvalue!{ XMM15:128 },

       (0b0000,256) => rreil_rvalue!{ YMM0:256 },
       (0b0001,256) => rreil_rvalue!{ YMM1:256 },
       (0b0010,256) => rreil_rvalue!{ YMM2:256 },
       (0b0011,256) => rreil_rvalue!{ YMM3:256 },
       (0b0100,256) => rreil_rvalue!{ YMM4:256 },
       (0b0101,256) => rreil_rvalue!{ YMM5:256 },
       (0b0110,256) => rreil_rvalue!{ YMM6:256 },
       (0b0111,256) => rreil_rvalue!{ YMM7:256 },
       (0b1000,256) => rreil_rvalue!{ YMM8:256 },
       (0b1001,256) => rreil_rvalue!{ YMM9:256 },
       (0b1010,256) => rreil_rvalue!{ YMM10:256 },
       (0b1011,256) => rreil_rvalue!{ YMM11:256 },
       (0b1100,256) => rreil_rvalue!{ YMM12:256 },
       (0b1101,256) => rreil_rvalue!{ YMM13:256 },
       (0b1110,256) => rreil_rvalue!{ YMM14:256 },
       (0b1111,256) => rreil_rvalue!{ YMM15:256 },

        _ => return Err("Invalid reg value".into()),
    };
    Ok((ret,vec![]))
}

fn read_ctrl_register(reg: u8, opsz: usize) -> Result<(Rvalue,Vec<Statement>)> {
    let ret = match (reg,opsz) {
       (0b0000,32) => rreil_rvalue!{ CR0:32 },
       (0b0001,32) => rreil_rvalue!{ CR1:32 },
       (0b0010,32) => rreil_rvalue!{ CR2:32 },
       (0b0011,32) => rreil_rvalue!{ CR3:32 },
       (0b0100,32) => rreil_rvalue!{ CR4:32 },
       (0b0101,32) => rreil_rvalue!{ CR5:32 },
       (0b0110,32) => rreil_rvalue!{ CR6:32 },
       (0b0111,32) => rreil_rvalue!{ CR7:32 },
       (0b1000,32) => rreil_rvalue!{ CR8:32 },
       (0b1001,32) => rreil_rvalue!{ CR9:32 },
       (0b1010,32) => rreil_rvalue!{ CR10:32 },
       (0b1011,32) => rreil_rvalue!{ CR11:32 },
       (0b1100,32) => rreil_rvalue!{ CR12:32 },
       (0b1101,32) => rreil_rvalue!{ CR13:32 },
       (0b1110,32) => rreil_rvalue!{ CR14:32 },
       (0b1111,32) => rreil_rvalue!{ CR15:32 },

        _ => return Err("Invalid reg value".into()),
    };
    Ok((ret,vec![]))
}

fn read_debug_register(reg: u8, opsz: usize) -> Result<(Rvalue,Vec<Statement>)> {
    let ret = match (reg,opsz) {
       (0b0000,32) => rreil_rvalue!{ DR0:32 },
       (0b0001,32) => rreil_rvalue!{ DR1:32 },
       (0b0010,32) => rreil_rvalue!{ DR2:32 },
       (0b0011,32) => rreil_rvalue!{ DR3:32 },
       (0b0100,32) => rreil_rvalue!{ DR4:32 },
       (0b0101,32) => rreil_rvalue!{ DR5:32 },
       (0b0110,32) => rreil_rvalue!{ DR6:32 },
       (0b0111,32) => rreil_rvalue!{ DR7:32 },
       (0b1000,32) => rreil_rvalue!{ DR8:32 },
       (0b1001,32) => rreil_rvalue!{ DR9:32 },
       (0b1010,32) => rreil_rvalue!{ DR10:32 },
       (0b1011,32) => rreil_rvalue!{ DR11:32 },
       (0b1100,32) => rreil_rvalue!{ DR12:32 },
       (0b1101,32) => rreil_rvalue!{ DR13:32 },
       (0b1110,32) => rreil_rvalue!{ DR14:32 },
       (0b1111,32) => rreil_rvalue!{ DR15:32 },

        _ => return Err("Invalid reg value".into()),
    };
    Ok((ret,vec![]))
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
    operand_a: Operand,
    operand_b: Operand,
    operand_c: Operand,
    operand_d: Operand,
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

fn has_sib(mod_: u8, _: u8, rm: u8) -> bool {
    return mod_ != 0b11 && rm == 0b100;
}

fn read_modrm(modrm: usize, sib: Option<usize>) -> usize {
    let reg = (modrm & 0b00111000) >> 3;
    let mo = (modrm & 0b11000000) >> 6;
    let rm = modrm & 0b00000111;

    match (mo,rm) {
        (0b00,0b000) => 0,
        (0b00,0b001) => 0,
        (0b00,0b010) => 0,
        (0b00,0b011) => 0,
        (0b00,0b100) => 1 + if sib.unwrap_or(0) & 0b111 == 0b101 { 4 } else { 0 },
        (0b00,0b101) => 4,
        (0b00,0b110) => 0,
        (0b00,0b111) => 0,

        (0b01,0b000) => 1,
        (0b01,0b001) => 1,
        (0b01,0b010) => 1,
        (0b01,0b011) => 1,
        (0b01,0b100) => 2,
        (0b01,0b101) => 1,
        (0b01,0b110) => 1,
        (0b01,0b111) => 1,

        (0b10,0b000) => 4,
        (0b10,0b001) => 4,
        (0b10,0b010) => 4,
        (0b10,0b011) => 4,
        (0b10,0b100) => 5,
        (0b10,0b101) => 4,
        (0b10,0b110) => 4,
        (0b10,0b111) => 4,

        _ => 0,
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
            (0b010,0b000) => opcode!("xgetbv"; ),
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
            0b000 => opcode!("prefetch"; NTA),
            0b001 => opcode!("prefetch"; T0),
            0b010 => opcode!("prefetch"; T1),
            0b011 => opcode!("prefetch"; T2),
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
            // Group 3: Operand size override and mandatory prefix
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
                if buf[i] & 0b00001000 != 0 && prefix.operand_size == 32 {
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
                    operand_a: Operand::Present(AddressingMethod::G,OperandType::v),
                    operand_b: Operand::Present(AddressingMethod::E,OperandType::v),
                    operand_c: Operand::None,
                    operand_d: Operand::None,
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

        let (opc,has_modrm) = match opc.mnemonic {
            Mnemonic::Single(s) => {
                let m = opc.operand_a.requires_modrm() ||
                        opc.operand_b.requires_modrm() ||
                        opc.operand_c.requires_modrm() ||
                        opc.operand_d.requires_modrm();
                (opc,m)
            }
            Mnemonic::Undefined => return Err("Unknown instruction".into()),
            Mnemonic::Escape => {
                let modrm = buf[i + 1] as usize;
                let esc = buf[i];

                if modrm < 0xc0 {
                    let ext = (modrm >> 3) & 0b111;
                    (match esc {
                        0xd8 => X87_D8_TABLE2[ext].clone(),
                        0xd9 => X87_D9_TABLE2[ext].clone(),
                        0xda => X87_DA_TABLE2[ext].clone(),
                        0xdb => X87_DB_TABLE2[ext].clone(),
                        0xdc => X87_DC_TABLE2[ext].clone(),
                        0xdd => X87_DD_TABLE2[ext].clone(),
                        0xde => X87_DE_TABLE2[ext].clone(),
                        0xdf => X87_DF_TABLE2[ext].clone(),
                        _ => unreachable!(),
                    },true)
                } else {
                    i += 1;
                    (match esc {
                        0xd8 => X87_D8_TABLE[modrm - 0xc0].clone(),
                        0xd9 => X87_D9_TABLE[modrm - 0xc0].clone(),
                        0xda => X87_DA_TABLE[modrm - 0xc0].clone(),
                        0xdb => X87_DB_TABLE[modrm - 0xc0].clone(),
                        0xdc => X87_DC_TABLE[modrm - 0xc0].clone(),
                        0xdd => X87_DD_TABLE[modrm - 0xc0].clone(),
                        0xde => X87_DE_TABLE[modrm - 0xc0].clone(),
                        0xdf => X87_DF_TABLE[modrm - 0xc0].clone(),
                        _ => unreachable!(),
                    },false)
                }
            }
            Mnemonic::ModRM(grp) => {
                let pfx = if prefix.opcode_escape != OpcodeEscape::None {
                    prefix.simd_prefix
                } else {
                    SimdPrefix::None
                };

                (try!(select_opcode_ext(grp,buf[i] as usize, buf[i + 1] as usize, pfx,mode,vexxop_present)).clone(),true)
            }
        };

        trace!("opcode len: {}",i + 1);

        let mut tail = 0;

        if has_modrm {
            tail += read_modrm(buf[i + 1] as usize,buf.get(i + 2).map(|&x| x as usize)) + 1;
            trace!("w/ modrm: {} bytes tail",tail);
        }

        for op in [&opc.operand_a, &opc.operand_b, &opc.operand_c, &opc.operand_d].iter() {
            if op.requires_immediate() {
                tail += op.bit_size(prefix.operand_size) / 8;
                trace!("w/ imm: {} bytes tail",tail);
                break;
            }
        }

        trace!("{:?} {:?}",prefix,opc);

        match opc.mnemonic {
            Mnemonic::Single(s) => {
                use byteorder::{LittleEndian,ReadBytesExt};
                use std::io::{Read,Cursor};

                let mut mod_: Option<u8> = None;
                let mut reg: Option<u8> = None;
                let mut rm: Option<u8> = None;
                let mut scale: Option<u8> = None;
                let mut index: Option<u8> = None;
                let mut base: Option<u8> = None;
                let mut disp8: Option<u8> = None;
                let mut disp32: Option<u32> = None;
                let mut imm8: Option<u8> = None;
                let mut imm16: Option<u16> = None;
                let mut imm32: Option<u32> = None;
                let mut imm48: Option<u64> = None;
                let mut imm64: Option<u64> = None;
                let modrm = buf.get(i).cloned();
                let sib_present = modrm.and_then(|x| Some(has_sib(x >> 6,(x >> 3) & 0b111,x & 0b111))) == Some(true);

                if tail >= 8 {
                    let mut cur = Cursor::new(&buf[i..]);
                    imm64 = cur.read_u64::<LittleEndian>().ok();
                }

                if tail >= 6 {
                    let mut cur = Cursor::new(&buf[i..]);
                    let a = cur.read_u32::<LittleEndian>().ok();
                    let b = cur.read_u16::<LittleEndian>().ok();
                    if let (Some(a),Some(b)) = (a,b) {
                        imm48 = Some(a as u64 | ((b as u64) << 32));
                    }
                }

                if tail >= 4 {
                    let mut cur = Cursor::new(&buf[i..]);
                    imm32 = cur.read_u32::<LittleEndian>().ok();
                }

                if tail >= 2 {
                    let mut cur = Cursor::new(&buf[i..]);
                    imm16 = cur.read_u16::<LittleEndian>().ok();

                    if sib_present {
                        let sib = buf.get(i + 1).cloned();
                        scale = sib.and_then(|x| Some(x >> 6));
                        index = sib.and_then(|x| Some((x >> 3) & 0b111));
                        base = sib.and_then(|x| Some(x & 0b111));

                        if rex_present {
                            if prefix.rex_x { index = index.and_then(|x| Some(x  | 0b1000)); };
                            if prefix.rex_b { base = base.and_then(|x| Some(x  | 0b1000)); };
                        }
                    }
                }

                if tail >= 1 {

                    imm8 = buf.get(i).cloned();
                    mod_ = modrm.and_then(|x| Some(x >> 6));
                    reg = modrm.and_then(|x| Some((x >> 3) & 0b111));
                    rm = modrm.and_then(|x| Some(x & 0b111));

                    if rex_present {
                        if prefix.rex_r { reg = reg.and_then(|x| Some(x | 0b1000)); };
                        if prefix.rex_x { index = index.and_then(|x| Some(x | 0b1000)); };
                        if prefix.rex_b && !sib_present { rm = rm.and_then(|x| Some(x | 0b1000)); };
                    }
                }

                let op1 = opc.operand_a.to_rreil(mode,mod_,reg,rm,scale,index,base,prefix.vvvv,disp8,disp32,rex_present,imm8,imm16,imm32,imm48,imm64,prefix.operand_size,prefix.address_size,prefix.operand_size,0);
                let op2 = opc.operand_b.to_rreil(mode,mod_,reg,rm,scale,index,base,prefix.vvvv,disp8,disp32,rex_present,imm8,imm16,imm32,imm48,imm64,prefix.operand_size,prefix.address_size,prefix.operand_size,0);
                let op3 = opc.operand_c.to_rreil(mode,mod_,reg,rm,scale,index,base,prefix.vvvv,disp8,disp32,rex_present,imm8,imm16,imm32,imm48,imm64,prefix.operand_size,prefix.address_size,prefix.operand_size,0);
                let op4 = opc.operand_d.to_rreil(mode,mod_,reg,rm,scale,index,base,prefix.vvvv,disp8,disp32,rex_present,imm8,imm16,imm32,imm48,imm64,prefix.operand_size,prefix.address_size,prefix.operand_size,0);

                let ops: Vec<String> = vec![op1,op2,op3,op4].iter().filter_map(|x| match x {
                    &Some((Rvalue::Variable{ ref name,.. },_)) => Some(name.to_string()),
                    &Some((Rvalue::Constant{ ref value,.. },_)) => Some(format!("0x{:x}",value)),
                    &Some((Rvalue::Undefined,_)) => Some("?".to_string()),
                    _ => None,
                }).collect();

                debug!("'{}' with {} bytes",s,tail + i + 1);
                println!("{} {:?}",s,ops);
                trace!("");
                Ok(tail + i + 1)
            }
            e => {
                println!("tried to match {:?}",e);
                Err("Internal error".into())
            }
        }
    }
}
