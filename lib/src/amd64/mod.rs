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

use std::rc::Rc;
use std::borrow::Cow;

use disassembler::*;
use {
    Lvalue,
    Rvalue,
    CodeGen,
    Result,
    LayerIter,
};

pub mod decode;
pub mod semantic;
pub mod integer;
pub mod vector;
pub mod extensions;

#[derive(Clone)]
pub enum Amd64 {}

#[derive(Clone,PartialEq,Copy)]
pub enum AddressSize {
    SixtyFour,
    ThirtyTwo,
    Sixteen,
}

#[derive(Clone,PartialEq,Copy)]
pub enum OperandSize {
    HundredTwentyEight,
    SixtyFour,
    ThirtyTwo,
    Sixteen,
    Eight,
}

impl OperandSize {
    fn num_bits(&self) -> usize {
        match self {
            &OperandSize::HundredTwentyEight => 128,
            &OperandSize::SixtyFour => 64,
            &OperandSize::ThirtyTwo => 32,
            &OperandSize::Sixteen => 16,
            &OperandSize::Eight => 8,
        }
    }
}

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

#[derive(Clone,PartialEq,Copy)]
pub enum Mode {
    Real,       // Real mode / Virtual 8086 mode
    Protected,  // Protected mode / Long compatibility mode
    Long,       // Long 64-bit mode
}

#[derive(Clone)]
pub struct Config {
    pub address_size: AddressSize,
    pub operand_size: OperandSize,
    pub mode: Mode,
    pub rex: bool,
    pub reg: Option<Lvalue>,
    pub rm: Option<Lvalue>,
    pub imm: Option<Rvalue>,
    pub disp: Option<Rvalue>,
    pub moffs: Option<Rvalue>,
    pub vvvv: Option<Lvalue>,
}

impl Config {
    pub fn new(m: Mode) -> Config {
        match m {
            Mode::Real => Config{
                address_size: AddressSize::Sixteen,
                operand_size: OperandSize::Sixteen,
                mode: m, rex: false, reg: None, rm: None,
                imm: None, disp: None, moffs: None,
                vvvv: None,
            },
            // assumes CS.d == 1
            Mode::Protected => Config{
                address_size: AddressSize::ThirtyTwo,
                operand_size: OperandSize::ThirtyTwo,
                mode: m, rex: false, reg: None, rm: None,
                imm: None, disp: None, moffs: None,
                vvvv: None,
            },
            // assumes REX.W == 0
            Mode::Long => Config{
                address_size: AddressSize::SixtyFour,
                operand_size: OperandSize::ThirtyTwo,
                mode: m, rex: false, reg: None, rm: None,
                imm: None, disp: None, moffs: None,
                vvvv: None,
            },
        }
    }
}

impl Architecture for Amd64 {
    type Token = u8;
    type Configuration = Config;

    fn prepare(_: LayerIter,cfg: &Self::Configuration) -> Result<Vec<(&'static str,u64,&'static str)>> {
        match cfg.mode {
            Mode::Real => Ok(vec![("RESET",0xFFFF0,"Reset vector")]),
            Mode::Protected => Ok(vec![("RESET",0xFFFFFFF0,"Reset vector")]),
            Mode::Long => Ok(vec![("RESET",0xFFFFFFF0,"Reset vector")]),
        }
    }
    fn disassembler(cfg: &Self::Configuration) -> Rc<Disassembler<Self>> {
        disassembler(cfg.mode)
    }
}

// 8 bit gp registers
lazy_static! {
    pub static ref AL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("al"), size: 8, offset: 0, subscript: None };
    pub static ref BL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("bl"), size: 8, offset: 0, subscript: None };
    pub static ref CL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cl"), size: 8, offset: 0, subscript: None };
    pub static ref DL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dl"), size: 8, offset: 0, subscript: None };
    pub static ref R8L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r8l"), size: 8, offset: 0, subscript: None };
    pub static ref R9L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r9l"), size: 8, offset: 0, subscript: None };
    pub static ref R10L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r10l"), size: 8, offset: 0, subscript: None };
    pub static ref R11L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r11l"), size: 8, offset: 0, subscript: None };
    pub static ref R12L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r12l"), size: 8, offset: 0, subscript: None };
    pub static ref R13L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r13l"), size: 8, offset: 0, subscript: None };
    pub static ref R14L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r14l"), size: 8, offset: 0, subscript: None };
    pub static ref R15L: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r15l"), size: 8, offset: 0, subscript: None };
    pub static ref SPL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("spl"), size: 8, offset: 0, subscript: None };
    pub static ref BPL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("bpl"), size: 8, offset: 0, subscript: None };
    pub static ref SIL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("sil"), size: 8, offset: 0, subscript: None };
    pub static ref DIL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dil"), size: 8, offset: 0, subscript: None };
    pub static ref AH: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ah"), size: 8, offset: 0, subscript: None };
    pub static ref BH: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("bh"), size: 8, offset: 0, subscript: None };
    pub static ref CH: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ch"), size: 8, offset: 0, subscript: None };
    pub static ref DH: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dh"), size: 8, offset: 0, subscript: None };
}

// 16 bit gp registers
lazy_static! {
    pub static ref AX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ax"), size: 16, offset: 0, subscript: None };
    pub static ref BX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("bx"), size: 16, offset: 0, subscript: None };
    pub static ref CX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cx"), size: 16, offset: 0, subscript: None };
    pub static ref DX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dx"), size: 16, offset: 0, subscript: None };
    pub static ref R8W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r8w"), size: 16, offset: 0, subscript: None };
    pub static ref R9W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r9w"), size: 16, offset: 0, subscript: None };
    pub static ref R10W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r10w"), size: 16, offset: 0, subscript: None };
    pub static ref R11W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r11w"), size: 16, offset: 0, subscript: None };
    pub static ref R12W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r12w"), size: 16, offset: 0, subscript: None };
    pub static ref R13W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r13w"), size: 16, offset: 0, subscript: None };
    pub static ref R14W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r14w"), size: 16, offset: 0, subscript: None };
    pub static ref R15W: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r15w"), size: 16, offset: 0, subscript: None };
    pub static ref SP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("sp"), size: 16, offset: 0, subscript: None };
    pub static ref BP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("bp"), size: 16, offset: 0, subscript: None };
    pub static ref SI: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("si"), size: 16, offset: 0, subscript: None };
    pub static ref DI: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("di"), size: 16, offset: 0, subscript: None };
    pub static ref IP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ip"), size: 16, offset: 0, subscript: None };
}

// 32 bit gp registers
lazy_static! {
    pub static ref EAX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("eax"), size: 32, offset: 0, subscript: None };
    pub static ref EBX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ebx"), size: 32, offset: 0, subscript: None };
    pub static ref ECX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ecx"), size: 32, offset: 0, subscript: None };
    pub static ref EDX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("edx"), size: 32, offset: 0, subscript: None };
    pub static ref R8D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r8d"), size: 32, offset: 0, subscript: None };
    pub static ref R9D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r9d"), size: 32, offset: 0, subscript: None };
    pub static ref R10D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r10d"), size: 32, offset: 0, subscript: None };
    pub static ref R11D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r11d"), size: 32, offset: 0, subscript: None };
    pub static ref R12D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r12d"), size: 32, offset: 0, subscript: None };
    pub static ref R13D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r13d"), size: 32, offset: 0, subscript: None };
    pub static ref R14D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r14d"), size: 32, offset: 0, subscript: None };
    pub static ref R15D: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r15d"), size: 32, offset: 0, subscript: None };
    pub static ref ESP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("esp"), size: 32, offset: 0, subscript: None };
    pub static ref EBP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ebp"), size: 32, offset: 0, subscript: None };
    pub static ref ESI: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("esi"), size: 32, offset: 0, subscript: None };
    pub static ref EDI: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("edi"), size: 32, offset: 0, subscript: None };
    pub static ref EIP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("eip"), size: 32, offset: 0, subscript: None };
}

// 64 bit gp registers
lazy_static! {
    pub static ref RAX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rax"), size: 64, offset: 0, subscript: None };
    pub static ref RBX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rbx"), size: 64, offset: 0, subscript: None };
    pub static ref RCX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rcx"), size: 64, offset: 0, subscript: None };
    pub static ref RDX: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rdx"), size: 64, offset: 0, subscript: None };
    pub static ref R8: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r8"), size: 64, offset: 0, subscript: None };
    pub static ref R9: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r9"), size: 64, offset: 0, subscript: None };
    pub static ref R10: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r10"), size: 64, offset: 0, subscript: None };
    pub static ref R11: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r11"), size: 64, offset: 0, subscript: None };
    pub static ref R12: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r12"), size: 64, offset: 0, subscript: None };
    pub static ref R13: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r13"), size: 64, offset: 0, subscript: None };
    pub static ref R14: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r14"), size: 64, offset: 0, subscript: None };
    pub static ref R15: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("r15"), size: 64, offset: 0, subscript: None };
    pub static ref RSP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rsp"), size: 64, offset: 0, subscript: None };
    pub static ref RBP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rbp"), size: 64, offset: 0, subscript: None };
    pub static ref RSI: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rsi"), size: 64, offset: 0, subscript: None };
    pub static ref RDI: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rdi"), size: 64, offset: 0, subscript: None };
    pub static ref RIP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("rip"), size: 64, offset: 0, subscript: None };
}

// flags
lazy_static! {
    pub static ref CF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("CF"), size: 1, offset: 0, subscript: None };
    pub static ref PF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("PF"), size: 1, offset: 0, subscript: None };
    pub static ref AF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("AF"), size: 1, offset: 0, subscript: None };
    pub static ref ZF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ZF"), size: 1, offset: 0, subscript: None };
    pub static ref SF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("SF"), size: 1, offset: 0, subscript: None };
    pub static ref TF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("TF"), size: 1, offset: 0, subscript: None };
    pub static ref IF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("IF"), size: 1, offset: 0, subscript: None };
    pub static ref DF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("DF"), size: 1, offset: 0, subscript: None };
    pub static ref OF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("OF"), size: 1, offset: 0, subscript: None };
    pub static ref RF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("RF"), size: 1, offset: 0, subscript: None };
    pub static ref IOPL: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("IOPL"), size: 0, offset: 0, subscript: None };
    pub static ref NT: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("NT"), size: 0, offset: 0, subscript: None };
    pub static ref VM: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("VM"), size: 0, offset: 0, subscript: None };
    pub static ref AC: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("AC"), size: 0, offset: 0, subscript: None };
    pub static ref VIF: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("VIF"), size: 0, offset: 0, subscript: None };
    pub static ref VIP: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("VIP"), size: 0, offset: 0, subscript: None };
    pub static ref ID: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ID"), size: 0, offset: 0, subscript: None };
}

// segment registers
lazy_static! {
    pub static ref CS: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cs"), size: 16, offset: 0, subscript: None };
    pub static ref DS: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ds"), size: 16, offset: 0, subscript: None };
    pub static ref FS: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("fs"), size: 16, offset: 0, subscript: None };
    pub static ref SS: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ss"), size: 16, offset: 0, subscript: None };
    pub static ref GS: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("gs"), size: 16, offset: 0, subscript: None };
    pub static ref ES: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("es"), size: 16, offset: 0, subscript: None };
}

// control registers
lazy_static! {
    pub static ref CR0: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cr0"), size: 64, offset: 0, subscript: None };
    pub static ref CR1: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cr1"), size: 64, offset: 0, subscript: None };
    pub static ref CR2: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cr2"), size: 64, offset: 0, subscript: None };
    pub static ref CR3: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cr3"), size: 64, offset: 0, subscript: None };
    pub static ref CR4: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cr4"), size: 64, offset: 0, subscript: None };
    pub static ref CR8: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("cr8"), size: 64, offset: 0, subscript: None };
    pub static ref LDTR: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("ldtr"), size: 64, offset: 0, subscript: None };
    pub static ref GDTR: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("gdtr"), size: 64, offset: 0, subscript: None };
    pub static ref IDTR: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("idtr"), size: 64, offset: 0, subscript: None };
}

// debug registers
lazy_static! {
    pub static ref DR0: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr0"), size: 32, offset: 0, subscript: None };
    pub static ref DR1: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr1"), size: 32, offset: 0, subscript: None };
    pub static ref DR2: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr2"), size: 32, offset: 0, subscript: None };
    pub static ref DR3: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr3"), size: 32, offset: 0, subscript: None };
    pub static ref DR4: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr4"), size: 32, offset: 0, subscript: None };
    pub static ref DR5: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr5"), size: 32, offset: 0, subscript: None };
    pub static ref DR6: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr6"), size: 32, offset: 0, subscript: None };
    pub static ref DR7: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("dr7"), size: 32, offset: 0, subscript: None };
}

// fpu register stack
lazy_static! {
    pub static ref ST0: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st0"), size: 80, offset: 0, subscript: None };
    pub static ref ST1: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st1"), size: 80, offset: 0, subscript: None };
    pub static ref ST2: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st2"), size: 80, offset: 0, subscript: None };
    pub static ref ST3: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st3"), size: 80, offset: 0, subscript: None };
    pub static ref ST4: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st4"), size: 80, offset: 0, subscript: None };
    pub static ref ST5: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st5"), size: 80, offset: 0, subscript: None };
    pub static ref ST6: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st6"), size: 80, offset: 0, subscript: None };
    pub static ref ST7: Lvalue = Lvalue::Variable{ name: Cow::Borrowed("st7"), size: 80, offset: 0, subscript: None };
}

pub fn disassembler(bits: Mode) -> Rc<Disassembler<Amd64>> {
    let opsize_prfx = new_disassembler!(Amd64 =>
        [ 0x66, 0x66, 0x66, 0x66, 0x66, 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        },
        [ 0x66, 0x66, 0x66, 0x66, 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        },
        [ 0x66, 0x66, 0x66, 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        },
        [ 0x66, 0x66, 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        },

        [ 0x66, 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        },
        [ 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        });

    let addrsz_prfx = new_disassembler!(Amd64 =>
        [ 0x67 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.address_size = AddressSize::ThirtyTwo,
                Mode::Long => st.configuration.address_size = AddressSize::ThirtyTwo,
                Mode::Protected => st.configuration.address_size = AddressSize::Sixteen,
            }
            true
        });

    let rep_prfx = new_disassembler!(Amd64 =>
        [ 0xf3 ] = |_: &mut State<Amd64>| { true });

    let lock_prfx = new_disassembler!(Amd64 =>
        [ 0xf0 ] = |_: &mut State<Amd64>| { true });

    let repx_prfx = new_disassembler!(Amd64 =>
        [ 0xf3 ] = |_: &mut State<Amd64>| { true },
        [ 0xf2 ] = |_: &mut State<Amd64>| { true });

    fn rex_semantic(st: &mut State<Amd64>) -> bool {
        st.configuration.rex = true;
        if st.get_group("w") == 1 {
            st.configuration.operand_size = OperandSize::SixtyFour;
        }
        if st.has_group("vvvv") {
            st.configuration.vvvv = Some(decode::select_reg(&st.configuration.operand_size,15 ^ st.get_group("vvvv"),st.configuration.rex));
        }
        true
    }

    let rex_prfx = new_disassembler!(Amd64 =>
        [ "0100 w@. r@. x@. b@." ] = rex_semantic);

    let rexw_prfx = new_disassembler!(Amd64 =>
        [ "0100 w@1 r@. x@. b@." ] = rex_semantic);

    let vex_0f_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 01", "w@. vvvv@.... 1 00", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00001", "w@. vvvv@.... L@. 00" ] = rex_semantic,
        [ "11000101", "w@. vvvv@.... L@. 00" ] = rex_semantic);

    let vex_660f_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 01", "w@. vvvv@.... 1 01", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00001", "w@. vvvv@.... L@. 01" ] = rex_semantic,
        [ "11000101", "w@. vvvv@.... L@. 01" ] = rex_semantic);

    let vex_f20f_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 01", "w@. vvvv@.... 1 11", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00001", "w@. vvvv@.... L@. 11" ] = rex_semantic,
        [ "11000101", "w@. vvvv@.... L@. 11" ] = rex_semantic);

    let vex_f30f_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 01", "w@. vvvv@.... 1 10", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00001", "w@. vvvv@.... L@. 10" ] = rex_semantic,
        [ "11000101", "w@. vvvv@.... L@. 10" ] = rex_semantic);

    let vex_0f38_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 10", "w@. vvvv@.... 1 00", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00010", "w@. vvvv@.... L@. 00" ] = rex_semantic);

    let vex_660f38_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 10", "w@. vvvv@.... 1 01", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00010", "w@. vvvv@.... L@. 01" ] = rex_semantic);

    let vex_f20f38_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 10", "w@. vvvv@.... 1 11", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00010", "w@. vvvv@.... L@. 11" ] = rex_semantic);

    let vex_f30f38_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 10", "w@. vvvv@.... 1 10", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00010", "w@. vvvv@.... L@. 10" ] = rex_semantic);

    let vex_0f3a_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 11", "w@. vvvv@.... 1 00", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00011", "w@. vvvv@.... L@. 00" ] = rex_semantic);

    let vex_660f3a_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 11", "w@. vvvv@.... 1 01", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00011", "w@. vvvv@.... L@. 01" ] = rex_semantic);

    let vex_f20f3a_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 11", "w@. vvvv@.... 1 11", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00011", "w@. vvvv@.... L@. 11" ] = rex_semantic);

    let vex_f30f3a_prfx = new_disassembler!(Amd64 =>
        [ "01100010", "r@. x@. b@. rr@. 00 11", "w@. vvvv@.... 1 10", "z@. LL@. L@. b@. Vb@. aaa@..." ] = rex_semantic,
        [ "11000100", "r@. x@. b@. 00011", "w@. vvvv@.... L@. 10" ] = rex_semantic);


    let seg_prfx = new_disassembler!(Amd64 =>
        [ 0x2e ] = |_: &mut State<Amd64>| { true },
        [ 0x36 ] = |_: &mut State<Amd64>| { true },
        [ 0x3e ] = |_: &mut State<Amd64>| { true },
        [ 0x26 ] = |_: &mut State<Amd64>| { true },
        [ 0x64 ] = |_: &mut State<Amd64>| { true },
        [ 0x65 ] = |_: &mut State<Amd64>| { true });

    let imm8 = new_disassembler!(Amd64 =>
        [ "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            true
        });

    let imm16 = new_disassembler!(Amd64 =>
        [ imm8, "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            true
        });

    let imm32 = new_disassembler!(Amd64 =>
        [ imm16, "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            true
        });

    let imm48 = new_disassembler!(Amd64 =>
        [ imm32, "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            // XXX
            //uint64_t a = st.capture_groups.at("imm") & 0xffff;
            //st.state.imm = constant((a << 32) | st.capture_groups.at("imm") >> 16);
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            true
        });

    let imm64 = new_disassembler!(Amd64 =>
        [ imm32, "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            true
        });

    let imm = new_disassembler!(Amd64 =>
        [ "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Eight
        },
        [ "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Sixteen
        },
        [ "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::ThirtyTwo || st.configuration.operand_size == OperandSize::SixtyFour
        });

    let immlong = new_disassembler!(Amd64 =>
        [ "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Eight
        },
        [ "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Sixteen
        },
        [ "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::ThirtyTwo
        },
        [ "imm@........", "imm@........", "imm@........", "imm@........",
          "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::new_u64(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::SixtyFour
        });

    let moffs = new_disassembler!(Amd64 =>
        [ "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::new_u64(st.get_group("moffs")));
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::new_u64(st.get_group("moffs")));
            st.configuration.address_size == AddressSize::ThirtyTwo || st.configuration.address_size == AddressSize::SixtyFour
        });

    let moffs8 = new_disassembler!(Amd64 =>
        [ "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::new_u64(st.get_group("moffs")));
            st.configuration.operand_size = OperandSize::Eight;
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::new_u64(st.get_group("moffs")));
            st.configuration.operand_size = OperandSize::Eight;
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........",
          "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::new_u64(st.get_group("moffs")));
            st.configuration.operand_size = OperandSize::Eight;
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let m64 = new_disassembler!(Amd64 =>
        [ "mq@........", "mq@........" ] = |st: &mut State<Amd64>| {
            let md = Rvalue::new_u64(st.get_group("md"));
            st.mnemonic(0,"__decode_m64","",vec![],&move |cg: &mut CodeGen<Amd64>| {
                cg.configuration.rm = Some(decode::select_mem(&OperandSize::SixtyFour,md.clone(),cg))
            });
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "mq@........", "mq@........", "mq@........", "mq@........" ] = |st: &mut State<Amd64>| {
            let md = Rvalue::new_u64(st.get_group("md"));
            st.mnemonic(0,"__decode_m64","",vec![],&move |cg: &mut CodeGen<Amd64>| {
                cg.configuration.rm = Some(decode::select_mem(&OperandSize::SixtyFour,md.clone(),cg))
            });
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "mq@........", "mq@........", "mq@........", "mq@........",
          "mq@........", "mq@........", "mq@........", "mq@........" ] = |st: &mut State<Amd64>| {
            let md = Rvalue::new_u64(st.get_group("md"));
            st.mnemonic(0,"__decode_m64","",vec![],&move |cg: &mut CodeGen<Amd64>| {
                cg.configuration.rm = Some(decode::select_mem(&OperandSize::SixtyFour,md.clone(),cg))
            });
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let m128 = new_disassembler!(Amd64 =>
        [ "mdq@........", "mdq@........" ] = |st: &mut State<Amd64>| {
            let mdq = Rvalue::new_u64(st.get_group("mdq"));
            st.mnemonic(0,"__decode_m128","",vec![],&move |cg: &mut CodeGen<Amd64>| {
                cg.configuration.rm = Some(decode::select_mem(&OperandSize::HundredTwentyEight,mdq.clone(),cg));
            });
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "mdq@........", "mdq@........", "mdq@........", "mdq@........" ] = |st: &mut State<Amd64>| {
            let mdq = Rvalue::new_u64(st.get_group("mdq"));
            st.mnemonic(0,"__decode_m128","",vec![],&move |cg: &mut CodeGen<Amd64>| {
                cg.configuration.rm = Some(decode::select_mem(&OperandSize::HundredTwentyEight,mdq.clone(),cg));
            });
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "mdq@........", "mdq@........", "mdq@........", "mdq@........",
          "mdq@........", "mdq@........", "mdq@........", "mdq@........" ] = |st: &mut State<Amd64>| {
            let mdq = Rvalue::new_u64(st.get_group("mdq"));
            st.mnemonic(0,"__decode_m128","",vec![],&move |cg: &mut CodeGen<Amd64>| {
                cg.configuration.rm = Some(decode::select_mem(&OperandSize::HundredTwentyEight,mdq.clone(),cg));
            });
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let disp8 = new_disassembler!(Amd64 =>
        [ "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::new_u8(st.get_group("disp") as u8));
            true
        });

    let disp16 = new_disassembler!(Amd64 =>
        [ disp8, "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::new_u16(st.get_group("disp") as u16));
            true
        });

    let disp32 = new_disassembler!(Amd64 =>
        [ disp16, "disp@........", "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::new_u32(st.get_group("disp") as u32));
            true
        });

    let disp64 = new_disassembler!(Amd64 =>
        [ disp32, "disp@........", "disp@........", "disp@........", "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::new_u64(st.get_group("disp")));
            true
        });

    let sib = new_disassembler!(Amd64 =>
        [ "scale@.. index@... base@101", "sd@........", "sd@........", "sd@........", "sd@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::new_u64(st.get_group("sd")));
            st.get_group("mod") == 0
        },
        [ "scale@.. index@... base@..." ] = |st: &mut State<Amd64>| {
            st.get_group("mod") != 0 || st.get_group("base") != 5
        });

    let is4 = new_disassembler!(Amd64 =>
        [ "isfour@........" ] = |_: &mut State<Amd64>| {
            true
        },
        [ "scale@.. index@... base@..." ] = |st: &mut State<Amd64>| {
            st.get_group("mod") != 0 || st.get_group("base") != 5
        });

    fn rm_semantic(_os: Option<OperandSize>) -> Box<Fn(&mut State<Amd64>) -> bool> {
        Box::new(move |st: &mut State<Amd64>| -> bool {
            assert!(st.configuration.reg.is_none() && st.configuration.rm.is_none());

            if let Some(ref os) = _os {
                if *os == OperandSize::SixtyFour {
                    st.configuration.operand_size = if st.configuration.mode == Mode::Long {
                        os.clone()
                    } else {
                        OperandSize::ThirtyTwo
                    };
                } else {
                    st.configuration.operand_size = os.clone();
                }
            }

            if st.has_group("reg") {
                let reg = if st.has_group("r") && st.get_group("r") == 1 { 8 } else { 0 } + st.get_group("reg");
                st.configuration.reg = Some(decode::select_reg(&st.configuration.operand_size,reg,st.configuration.rex));
            }

            let b_rm = if st.has_group("b") && st.get_group("b") > 0 { 1 << 3 } else { 0 } + st.get_group("rm");

            let sib = if st.has_group("scale") && st.has_group("index") && st.has_group("base") {
                let scale = st.get_group("scale");
                let x_index = if st.configuration.rex && st.has_group("x") { 1 << 3 } else { 0 } + st.get_group("index");
                let b_base = if st.configuration.rex && st.has_group("b") { 1 << 3 } else { 0 } + st.get_group("base");

                Some((scale,x_index,b_base))
            } else {
                None
            };

            let _mod = st.get_group("mod");
            st.mnemonic(0,"internal-rm","",vec!(),&mut |cg: &mut CodeGen<Amd64>| {
                let maybe_modrm = decode::decode_modrm(_mod,
                                                       b_rm,
                                                       cg.configuration.disp.clone(),
                                                       sib,
                                                       cg.configuration.operand_size,
                                                       cg.configuration.address_size,
                                                       cg.configuration.mode,
                                                       cg.configuration.rex,
                                                       cg);
                cg.configuration.rm = maybe_modrm;
            });

            true
        })
    }

    let rm = new_disassembler!(Amd64 =>
        [ "mod@00 reg@... rm@000"         ] = rm_semantic(None),
        [ "mod@00 reg@... rm@001"         ] = rm_semantic(None),
        [ "mod@00 reg@... rm@010"         ] = rm_semantic(None),
        [ "mod@00 reg@... rm@011"         ] = rm_semantic(None),
        [ "mod@00 reg@... rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 reg@... rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 reg@... rm@110"         ] = rm_semantic(None),
        [ "mod@00 reg@... rm@111"         ] = rm_semantic(None),

        [ "mod@01 reg@... rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 reg@... rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 reg@... rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 reg@... rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 reg@... rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 reg@... rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 reg@... rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 reg@... rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 reg@... rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 reg@... rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 reg@... rm@..."              ] = rm_semantic(None));

    let rmlong = new_disassembler!(Amd64 =>
        [ "mod@00 reg@... rm@101", disp32      ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@00 reg@... rm@100", sib         ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@00 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@01 reg@... rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@01 reg@... rm@...", disp8       ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@10 reg@... rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@10 reg@... rm@...", disp32      ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@11 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::SixtyFour)));

    let rmbyte = new_disassembler!(Amd64 =>
        [ "mod@00 reg@... rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 reg@... rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 reg@... rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 reg@... rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 reg@... rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 reg@... rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@11 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    // w/ extension opcode
    let rm0 = new_disassembler!(Amd64 =>
        [ "mod@00 000 rm@000"         ] = rm_semantic(None),
        [ "mod@00 000 rm@001"         ] = rm_semantic(None),
        [ "mod@00 000 rm@010"         ] = rm_semantic(None),
        [ "mod@00 000 rm@011"         ] = rm_semantic(None),
        [ "mod@00 000 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 000 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 000 rm@110"         ] = rm_semantic(None),
        [ "mod@00 000 rm@111"         ] = rm_semantic(None),

        [ "mod@01 000 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 000 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 000 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 000 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 000 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 000 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 000 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 000 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 000 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 000 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 000 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 000 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 000 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 000 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 000 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 000 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 000 rm@..."              ] = rm_semantic(None));

    let rmbyte0 = new_disassembler!(Amd64 =>
        [ "mod@00 000 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 000 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 000 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 000 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm1 = new_disassembler!(Amd64 =>
        [ "mod@00 001 rm@000"         ] = rm_semantic(None),
        [ "mod@00 001 rm@001"         ] = rm_semantic(None),
        [ "mod@00 001 rm@010"         ] = rm_semantic(None),
        [ "mod@00 001 rm@011"         ] = rm_semantic(None),
        [ "mod@00 001 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 001 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 001 rm@110"         ] = rm_semantic(None),
        [ "mod@00 001 rm@111"         ] = rm_semantic(None),

        [ "mod@01 001 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 001 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 001 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 001 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 001 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 001 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 001 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 001 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 001 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 001 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 001 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 001 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 001 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 001 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 001 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 001 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 001 rm@..."              ] = rm_semantic(None));

    let rmbyte1 = new_disassembler!(Amd64 =>
        [ "mod@00 001 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 001 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 001 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 001 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm2 = new_disassembler!(Amd64 =>
        [ "mod@00 010 rm@000"         ] = rm_semantic(None),
        [ "mod@00 010 rm@001"         ] = rm_semantic(None),
        [ "mod@00 010 rm@010"         ] = rm_semantic(None),
        [ "mod@00 010 rm@011"         ] = rm_semantic(None),
        [ "mod@00 010 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 010 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 010 rm@110"         ] = rm_semantic(None),
        [ "mod@00 010 rm@111"         ] = rm_semantic(None),

        [ "mod@01 010 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 010 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 010 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 010 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 010 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 010 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 010 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 010 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 010 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 010 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 010 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 010 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 010 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 010 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 010 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 010 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 010 rm@..."              ] = rm_semantic(None));

    let rmbyte2 = new_disassembler!(Amd64 =>
        [ "mod@00 010 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 010 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 010 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 010 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm3 = new_disassembler!(Amd64 =>
        [ "mod@00 011 rm@000"         ] = rm_semantic(None),
        [ "mod@00 011 rm@001"         ] = rm_semantic(None),
        [ "mod@00 011 rm@010"         ] = rm_semantic(None),
        [ "mod@00 011 rm@011"         ] = rm_semantic(None),
        [ "mod@00 011 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 011 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 011 rm@110"         ] = rm_semantic(None),
        [ "mod@00 011 rm@111"         ] = rm_semantic(None),

        [ "mod@01 011 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 011 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 011 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 011 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 011 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 011 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 011 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 011 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 011 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 011 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 011 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 011 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 011 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 011 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 011 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 011 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 011 rm@..."              ] = rm_semantic(None));

    let rmbyte3 = new_disassembler!(Amd64 =>
        [ "mod@00 011 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 011 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 011 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 011 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm4 = new_disassembler!(Amd64 =>
        [ "mod@00 100 rm@000"         ] = rm_semantic(None),
        [ "mod@00 100 rm@001"         ] = rm_semantic(None),
        [ "mod@00 100 rm@010"         ] = rm_semantic(None),
        [ "mod@00 100 rm@011"         ] = rm_semantic(None),
        [ "mod@00 100 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 100 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 100 rm@110"         ] = rm_semantic(None),
        [ "mod@00 100 rm@111"         ] = rm_semantic(None),

        [ "mod@01 100 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 100 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 100 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 100 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 100 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 100 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 100 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 100 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 100 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 100 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 100 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 100 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 100 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 100 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 100 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 100 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 100 rm@..."              ] = rm_semantic(None));

    let rmbyte4 = new_disassembler!(Amd64 =>
        [ "mod@00 100 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 100 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 100 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 100 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm5 = new_disassembler!(Amd64 =>
        [ "mod@00 101 rm@000"         ] = rm_semantic(None),
        [ "mod@00 101 rm@001"         ] = rm_semantic(None),
        [ "mod@00 101 rm@010"         ] = rm_semantic(None),
        [ "mod@00 101 rm@011"         ] = rm_semantic(None),
        [ "mod@00 101 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 101 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 101 rm@110"         ] = rm_semantic(None),
        [ "mod@00 101 rm@111"         ] = rm_semantic(None),

        [ "mod@01 101 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 101 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 101 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 101 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 101 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 101 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 101 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 101 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 101 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 101 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 101 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 101 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 101 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 101 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 101 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 101 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 101 rm@..."              ] = rm_semantic(None));

    let rmbyte5 = new_disassembler!(Amd64 =>
        [ "mod@00 101 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 101 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 101 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 101 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm6 = new_disassembler!(Amd64 =>
        [ "mod@00 110 rm@000"         ] = rm_semantic(None),
        [ "mod@00 110 rm@001"         ] = rm_semantic(None),
        [ "mod@00 110 rm@010"         ] = rm_semantic(None),
        [ "mod@00 110 rm@011"         ] = rm_semantic(None),
        [ "mod@00 110 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 110 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 110 rm@110"         ] = rm_semantic(None),
        [ "mod@00 110 rm@111"         ] = rm_semantic(None),

        [ "mod@01 110 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 110 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 110 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 110 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 110 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 110 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 110 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 110 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 110 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 110 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 110 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 110 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 110 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 110 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 110 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 110 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 110 rm@..."              ] = rm_semantic(None));

    let rmbyte6 = new_disassembler!(Amd64 =>
        [ "mod@00 110 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 110 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 110 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 110 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let rm7 = new_disassembler!(Amd64 =>
        [ "mod@00 111 rm@000"         ] = rm_semantic(None),
        [ "mod@00 111 rm@001"         ] = rm_semantic(None),
        [ "mod@00 111 rm@010"         ] = rm_semantic(None),
        [ "mod@00 111 rm@011"         ] = rm_semantic(None),
        [ "mod@00 111 rm@100", sib    ] = rm_semantic(None),
        [ "mod@00 111 rm@101", disp32 ] = rm_semantic(None),
        [ "mod@00 111 rm@110"         ] = rm_semantic(None),
        [ "mod@00 111 rm@111"         ] = rm_semantic(None),

        [ "mod@01 111 rm@000", disp8       ] = rm_semantic(None),
        [ "mod@01 111 rm@001", disp8       ] = rm_semantic(None),
        [ "mod@01 111 rm@010", disp8       ] = rm_semantic(None),
        [ "mod@01 111 rm@011", disp8       ] = rm_semantic(None),
        [ "mod@01 111 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 111 rm@101", disp8       ] = rm_semantic(None),
        [ "mod@01 111 rm@110", disp8       ] = rm_semantic(None),
        [ "mod@01 111 rm@111", disp8       ] = rm_semantic(None),

        [ "mod@10 111 rm@000", disp32       ] = rm_semantic(None),
        [ "mod@10 111 rm@001", disp32       ] = rm_semantic(None),
        [ "mod@10 111 rm@010", disp32       ] = rm_semantic(None),
        [ "mod@10 111 rm@011", disp32       ] = rm_semantic(None),
        [ "mod@10 111 rm@100", sib, disp32  ] = rm_semantic(None),
        [ "mod@10 111 rm@101", disp32       ] = rm_semantic(None),
        [ "mod@10 111 rm@110", disp32       ] = rm_semantic(None),
        [ "mod@10 111 rm@111", disp32       ] = rm_semantic(None),

        [ "mod@11 111 rm@..."              ] = rm_semantic(None));

    let rmbyte7 = new_disassembler!(Amd64 =>
        [ "mod@00 111 rm@000"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@001"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@010"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@011"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@100", sib    ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@101", disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@110"         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@111"         ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@01 111 rm@000", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@001", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@010", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@011", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@101", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@110", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@111", disp8       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@10 111 rm@000", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@001", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@010", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@011", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@100", sib, disp32  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@101", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@110", disp32       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@111", disp32       ] = rm_semantic(Some(OperandSize::Eight)),

        [ "mod@11 111 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    let main = integer::integer_universial(
        imm8.clone(),
        imm16.clone(), imm32.clone(),
        imm48.clone(), imm64.clone(),
        imm.clone(), immlong.clone(),
        moffs8.clone(), moffs.clone(),
        sib.clone(), rm.clone(),
        rm0.clone(), rm1.clone(),
        rm2.clone(), rm3.clone(),
        rm4.clone(), rm5.clone(),
        rm6.clone(), rm7.clone(),
        rmbyte.clone(), rmbyte0.clone(),
        rmbyte1.clone(), rmbyte2.clone(),
        rmbyte3.clone(), rmbyte4.clone(),
        rmbyte5.clone(), rmbyte6.clone(),
        rmbyte7.clone(), rmlong.clone(),
        m64.clone(), disp8.clone(),
        disp16.clone(), disp32.clone(),
        disp64.clone());

     let lockable = integer::integer_lockable(
        imm8.clone(),
        imm16.clone(), imm32.clone(),
        imm48.clone(), imm64.clone(),
        imm.clone(), immlong.clone(),
        moffs8.clone(), moffs.clone(),
        sib.clone(), rm.clone(),
        rm0.clone(), rm1.clone(),
        rm2.clone(), rm3.clone(),
        rm4.clone(), rm5.clone(),
        rm6.clone(), rm7.clone(),
        rmbyte.clone(), rmbyte0.clone(),
        rmbyte1.clone(), rmbyte2.clone(),
        rmbyte3.clone(), rmbyte4.clone(),
        rmbyte5.clone(), rmbyte6.clone(),
        rmbyte7.clone(), rmlong.clone(),
        m64.clone(), disp8.clone(),
        disp16.clone(), disp32.clone(),
        disp64.clone());

    match bits {
        Mode::Real => {
            let main16 = integer::integer_16bit(imm16.clone(), imm32.clone(),
                moffs.clone(),
                rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone());
            let main16_or_32 = integer::integer_32bit_or_less(
                imm8.clone(), imm48.clone(),
                rm.clone(), rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone());
            let lockable16_or_32 = integer::lockable_32bit_or_less(
                imm8, imm48,
                rm.clone(), rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone());
            let x87 = extensions::fpu(rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7);

            new_disassembler!(Amd64 =>
                [ main ] = |_: &mut State<Amd64>| { true },
                [ x87 ] = |_: &mut State<Amd64>| { true },
                [ opt!(lock_prfx), lockable ] = |_: &mut State<Amd64>| { true },
                [ main16 ] = |_: &mut State<Amd64>| { true },
                [ main16_or_32 ] = |_: &mut State<Amd64>| { true },
                [ opt!(lock_prfx), lockable16_or_32 ] = |_: &mut State<Amd64>| { true },
                [ main16_or_32 ] = |_: &mut State<Amd64>| { true })
        },
        Mode::Protected => {
            let main32 = integer::integer_32bit(
                imm8.clone(), imm48.clone(),
                moffs.clone(),
                rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone());
            let main16_or_32 = integer::integer_32bit_or_less(
                imm8.clone(), imm48.clone(),
                rm.clone(), rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone());
            let lockable16_or_32 = integer::lockable_32bit_or_less(
                imm8, imm48,
                rm.clone(), rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone());
            let (rep,repx) = integer::integer_rep();
            let mpx = extensions::mpx(rm.clone());
            let x87 = extensions::fpu(rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7);

            new_disassembler!(Amd64 =>
                [ x87 ] = |_: &mut State<Amd64>| { true },
                [ mpx ] = |_: &mut State<Amd64>| { true },
                [ opt!(seg_prfx), opt!(opsize_prfx), opt!(addrsz_prfx), main ] = |_: &mut State<Amd64>| { true },
                [ opt!(lock_prfx), opt!(seg_prfx), opt!(opsize_prfx), opt!(addrsz_prfx),  lockable ] = |_: &mut State<Amd64>| { true },
                [ opt!(seg_prfx), opt!(opsize_prfx), opt!(addrsz_prfx), main32 ] = |_: &mut State<Amd64>| { true },
                [ opt!(seg_prfx), opt!(opsize_prfx), opt!(addrsz_prfx), main16_or_32 ] = |_: &mut State<Amd64>| { true },
                [ opt!(lock_prfx), opt!(seg_prfx), opt!(opsize_prfx), opt!(addrsz_prfx), lockable16_or_32 ] = |_: &mut State<Amd64>| { true },
                [ opt!(opsize_prfx), opt!(seg_prfx), opt!(addrsz_prfx), opt!(lock_prfx), main16_or_32 ] = |_: &mut State<Amd64>| { true },
                [ opt!(rep_prfx), opt!(seg_prfx), opt!(opsize_prfx), opt!(rep_prfx), rep ] = |_: &mut State<Amd64>| { true },
                [ opt!(rep_prfx), opt!(seg_prfx), opt!(opsize_prfx), opt!(repx_prfx), repx ] = |_: &mut State<Amd64>| { true })
        },
        Mode::Long => {
            let main64 = integer::integer_64bit(
                imm8.clone(),
                moffs.clone(),
                rm.clone(), rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone(),
                m128.clone());
            let lockable64 = integer::lockable_64bit(
                imm8.clone(),
                moffs.clone(),
                rm.clone(), rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone(),
                m128.clone());
            let (rep,repx) = integer::integer_rep();
            let sse4 = vector::sse4(
                rm.clone(),imm8.clone(),rex_prfx.clone(),rexw_prfx.clone());
            let sse3 = vector::sse3(
                rm.clone(),imm8.clone(),rex_prfx.clone(),rexw_prfx.clone());
            let sse2 = vector::sse2(
                rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone(),
                rm.clone(),imm8.clone(),rex_prfx.clone(),rexw_prfx.clone());
            let avx = vector::avx(
                vex_0f_prfx.clone(), vex_660f_prfx.clone(), vex_f20f_prfx.clone(),
                vex_f30f_prfx.clone(), vex_0f38_prfx.clone(), vex_660f38_prfx.clone(),
                vex_f20f38_prfx.clone(), vex_f30f38_prfx.clone(), vex_0f3a_prfx.clone(),
                vex_660f3a_prfx.clone(), vex_f20f3a_prfx.clone(), vex_f30f3a_prfx.clone(),
                rm.clone(),
                rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone(),
                imm8.clone(),is4.clone());
            let sse1 = vector::sse1(
                rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone(),
                rm.clone(),imm8.clone(),rex_prfx.clone(),rexw_prfx.clone());
            let mmx = vector::mmx(
                rm0.clone(), rm1.clone(), rm2.clone(), rm3.clone(), rm4.clone(), rm5.clone(), rm6.clone(), rm7.clone(),
                rm.clone(),imm8.clone());
            let mpx = extensions::mpx(rm.clone());
            let x87 = extensions::fpu(rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7);

            new_disassembler!(Amd64 =>
                [ opt!(rex_prfx), x87 ] = |_: &mut State<Amd64>| { true },
                [ opt!(rex_prfx), mpx ] = |_: &mut State<Amd64>| { true },
                [ opt!(opsize_prfx), opt!(addrsz_prfx), opt!(repx_prfx), opt!(seg_prfx), opt!(rex_prfx), main ] = |_: &mut State<Amd64>| { true },
                [ opt!(opsize_prfx), opt!(addrsz_prfx), opt!(repx_prfx), opt!(lock_prfx), opt!(seg_prfx), opt!(rex_prfx),  lockable ] = |_: &mut State<Amd64>| { true },
                [ opt!(opsize_prfx), opt!(addrsz_prfx), opt!(rex_prfx), main64 ] = |_: &mut State<Amd64>| { true },
                [ opt!(opsize_prfx), opt!(addrsz_prfx), opt!(lock_prfx),  opt!(rex_prfx), lockable64 ] = |_: &mut State<Amd64>| { true },
                [ mmx ] = |_: &mut State<Amd64>| { true },
                [ sse1 ] = |_: &mut State<Amd64>| { true },
                [ sse2 ] = |_: &mut State<Amd64>| { true },
                [ sse3 ] = |_: &mut State<Amd64>| { true },
                [ sse4 ] = |_: &mut State<Amd64>| { true },
                [ avx ] = |_: &mut State<Amd64>| { true },
                [ opt!(rep_prfx), opt!(opsize_prfx), opt!(rep_prfx), opt!(repx_prfx), opt!(rex_prfx), rep ] = |_: &mut State<Amd64>| { true },
                [ opt!(rep_prfx), opt!(opsize_prfx), opt!(repx_prfx), opt!(rex_prfx), repx ] = |_: &mut State<Amd64>| { true })
        }
    }
}
