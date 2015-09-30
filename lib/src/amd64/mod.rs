use disassembler::*;
use program::Program;
use layer::LayerIter;
use value::{Lvalue,Rvalue,Endianess,ToRvalue};
use codegen::CodeGen;
use guard::Guard;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::rc::Rc;

pub mod decode;
pub mod generic;
pub mod semantic;

#[derive(Clone)]
pub enum Amd64 {}

#[derive(Clone,PartialEq,Copy)]
pub enum AddressSize
{
    SixtyFour,
    ThirtyTwo,
    Sixteen,
}

#[derive(Clone,PartialEq,Copy)]
pub enum OperandSize
{
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

#[derive(Clone,PartialEq)]
pub enum Mode
{
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
}

impl Config {
    pub fn new(m: Mode) -> Config {
        match m {
            Mode::Real => Config{
                address_size: AddressSize::Sixteen,
                operand_size: OperandSize::Sixteen,
                mode: m, rex: false, reg: None, rm: None,
                imm: None, disp: None, moffs: None,
            },
            // assumes CS.d == 1
            Mode::Protected => Config{
                address_size: AddressSize::ThirtyTwo,
                operand_size: OperandSize::ThirtyTwo,
                mode: m, rex: false, reg: None, rm: None,
                imm: None, disp: None, moffs: None,
            },
            // assumes REX.W == 0
            Mode::Long => Config{
                address_size: AddressSize::SixtyFour,
                operand_size: OperandSize::ThirtyTwo,
                mode: m, rex: false, reg: None, rm: None,
                imm: None, disp: None, moffs: None,
            },
        }
    }
}

impl Architecture for Amd64 {
    type Token = u8;
    type Configuration = Config;
}

// 8 bit gp registers
lazy_static! {
    pub static ref al: Lvalue = Lvalue::Variable{ name: "al".to_string(), width: 8, subscript: None };
    pub static ref bl: Lvalue = Lvalue::Variable{ name: "bl".to_string(), width: 8, subscript: None };
    pub static ref cl: Lvalue = Lvalue::Variable{ name: "cl".to_string(), width: 8, subscript: None };
    pub static ref dl: Lvalue = Lvalue::Variable{ name: "dl".to_string(), width: 8, subscript: None };
    pub static ref r8l: Lvalue = Lvalue::Variable{ name: "r8l".to_string(), width: 8, subscript: None };
    pub static ref r9l: Lvalue = Lvalue::Variable{ name: "r9l".to_string(), width: 8, subscript: None };
    pub static ref r10l: Lvalue = Lvalue::Variable{ name: "r10l".to_string(), width: 8, subscript: None };
    pub static ref r11l: Lvalue = Lvalue::Variable{ name: "r11l".to_string(), width: 8, subscript: None };
    pub static ref r12l: Lvalue = Lvalue::Variable{ name: "r12l".to_string(), width: 8, subscript: None };
    pub static ref r13l: Lvalue = Lvalue::Variable{ name: "r13l".to_string(), width: 8, subscript: None };
    pub static ref r14l: Lvalue = Lvalue::Variable{ name: "r14l".to_string(), width: 8, subscript: None };
    pub static ref r15l: Lvalue = Lvalue::Variable{ name: "r15l".to_string(), width: 8, subscript: None };
    pub static ref spl: Lvalue = Lvalue::Variable{ name: "spl".to_string(), width: 8, subscript: None };
    pub static ref bpl: Lvalue = Lvalue::Variable{ name: "bpl".to_string(), width: 8, subscript: None };
    pub static ref sil: Lvalue = Lvalue::Variable{ name: "sil".to_string(), width: 8, subscript: None };
    pub static ref dil: Lvalue = Lvalue::Variable{ name: "dil".to_string(), width: 8, subscript: None };
    pub static ref ah: Lvalue = Lvalue::Variable{ name: "ah".to_string(), width: 8, subscript: None };
    pub static ref bh: Lvalue = Lvalue::Variable{ name: "bh".to_string(), width: 8, subscript: None };
    pub static ref ch: Lvalue = Lvalue::Variable{ name: "ch".to_string(), width: 8, subscript: None };
    pub static ref dh: Lvalue = Lvalue::Variable{ name: "dh".to_string(), width: 8, subscript: None };
}

// 16 bit gp registers
lazy_static! {
    pub static ref ax: Lvalue = Lvalue::Variable{ name: "ax".to_string(), width: 16, subscript: None };
    pub static ref bx: Lvalue = Lvalue::Variable{ name: "bx".to_string(), width: 16, subscript: None };
    pub static ref cx: Lvalue = Lvalue::Variable{ name: "cx".to_string(), width: 16, subscript: None };
    pub static ref dx: Lvalue = Lvalue::Variable{ name: "dx".to_string(), width: 16, subscript: None };
    pub static ref r8w: Lvalue = Lvalue::Variable{ name: "r8w".to_string(), width: 16, subscript: None };
    pub static ref r9w: Lvalue = Lvalue::Variable{ name: "r9w".to_string(), width: 16, subscript: None };
    pub static ref r10w: Lvalue = Lvalue::Variable{ name: "r10w".to_string(), width: 16, subscript: None };
    pub static ref r11w: Lvalue = Lvalue::Variable{ name: "r11w".to_string(), width: 16, subscript: None };
    pub static ref r12w: Lvalue = Lvalue::Variable{ name: "r12w".to_string(), width: 16, subscript: None };
    pub static ref r13w: Lvalue = Lvalue::Variable{ name: "r13w".to_string(), width: 16, subscript: None };
    pub static ref r14w: Lvalue = Lvalue::Variable{ name: "r14w".to_string(), width: 16, subscript: None };
    pub static ref r15w: Lvalue = Lvalue::Variable{ name: "r15w".to_string(), width: 16, subscript: None };
    pub static ref sp: Lvalue = Lvalue::Variable{ name: "sp".to_string(), width: 16, subscript: None };
    pub static ref bp: Lvalue = Lvalue::Variable{ name: "bp".to_string(), width: 16, subscript: None };
    pub static ref si: Lvalue = Lvalue::Variable{ name: "si".to_string(), width: 16, subscript: None };
    pub static ref di: Lvalue = Lvalue::Variable{ name: "di".to_string(), width: 16, subscript: None };
    pub static ref ip: Lvalue = Lvalue::Variable{ name: "ip".to_string(), width: 16, subscript: None };
}

// 32 bit gp registers
lazy_static! {
    pub static ref eax: Lvalue = Lvalue::Variable{ name: "eax".to_string(), width: 32, subscript: None };
    pub static ref ebx: Lvalue = Lvalue::Variable{ name: "ebx".to_string(), width: 32, subscript: None };
    pub static ref ecx: Lvalue = Lvalue::Variable{ name: "ecx".to_string(), width: 32, subscript: None };
    pub static ref edx: Lvalue = Lvalue::Variable{ name: "edx".to_string(), width: 32, subscript: None };
    pub static ref r8d: Lvalue = Lvalue::Variable{ name: "r8d".to_string(), width: 32, subscript: None };
    pub static ref r9d: Lvalue = Lvalue::Variable{ name: "r9d".to_string(), width: 32, subscript: None };
    pub static ref r10d: Lvalue = Lvalue::Variable{ name: "r10d".to_string(), width: 32, subscript: None };
    pub static ref r11d: Lvalue = Lvalue::Variable{ name: "r11d".to_string(), width: 32, subscript: None };
    pub static ref r12d: Lvalue = Lvalue::Variable{ name: "r12d".to_string(), width: 32, subscript: None };
    pub static ref r13d: Lvalue = Lvalue::Variable{ name: "r13d".to_string(), width: 32, subscript: None };
    pub static ref r14d: Lvalue = Lvalue::Variable{ name: "r14d".to_string(), width: 32, subscript: None };
    pub static ref r15d: Lvalue = Lvalue::Variable{ name: "r15d".to_string(), width: 32, subscript: None };
    pub static ref esp: Lvalue = Lvalue::Variable{ name: "esp".to_string(), width: 32, subscript: None };
    pub static ref ebp: Lvalue = Lvalue::Variable{ name: "ebp".to_string(), width: 32, subscript: None };
    pub static ref esi: Lvalue = Lvalue::Variable{ name: "esi".to_string(), width: 32, subscript: None };
    pub static ref edi: Lvalue = Lvalue::Variable{ name: "edi".to_string(), width: 32, subscript: None };
    pub static ref eip: Lvalue = Lvalue::Variable{ name: "eip".to_string(), width: 32, subscript: None };
}

// 64 bit gp registers
lazy_static! {
    pub static ref rax: Lvalue = Lvalue::Variable{ name: "rax".to_string(), width: 64, subscript: None };
    pub static ref rbx: Lvalue = Lvalue::Variable{ name: "rbx".to_string(), width: 64, subscript: None };
    pub static ref rcx: Lvalue = Lvalue::Variable{ name: "rcx".to_string(), width: 64, subscript: None };
    pub static ref rdx: Lvalue = Lvalue::Variable{ name: "rdx".to_string(), width: 64, subscript: None };
    pub static ref r8: Lvalue = Lvalue::Variable{ name: "r8".to_string(), width: 64, subscript: None };
    pub static ref r9: Lvalue = Lvalue::Variable{ name: "r9".to_string(), width: 64, subscript: None };
    pub static ref r10: Lvalue = Lvalue::Variable{ name: "r10".to_string(), width: 64, subscript: None };
    pub static ref r11: Lvalue = Lvalue::Variable{ name: "r11".to_string(), width: 64, subscript: None };
    pub static ref r12: Lvalue = Lvalue::Variable{ name: "r12".to_string(), width: 64, subscript: None };
    pub static ref r13: Lvalue = Lvalue::Variable{ name: "r13".to_string(), width: 64, subscript: None };
    pub static ref r14: Lvalue = Lvalue::Variable{ name: "r14".to_string(), width: 64, subscript: None };
    pub static ref r15: Lvalue = Lvalue::Variable{ name: "r15".to_string(), width: 64, subscript: None };
    pub static ref rsp: Lvalue = Lvalue::Variable{ name: "rsp".to_string(), width: 64, subscript: None };
    pub static ref rbp: Lvalue = Lvalue::Variable{ name: "rbp".to_string(), width: 64, subscript: None };
    pub static ref rsi: Lvalue = Lvalue::Variable{ name: "rsi".to_string(), width: 64, subscript: None };
    pub static ref rdi: Lvalue = Lvalue::Variable{ name: "rdi".to_string(), width: 64, subscript: None };
    pub static ref rip: Lvalue = Lvalue::Variable{ name: "rip".to_string(), width: 64, subscript: None };
}

// flags
lazy_static! {
    pub static ref CF: Lvalue = Lvalue::Variable{ name: "CF".to_string(), width: 1, subscript: None };
    pub static ref PF: Lvalue = Lvalue::Variable{ name: "PF".to_string(), width: 1, subscript: None };
    pub static ref AF: Lvalue = Lvalue::Variable{ name: "AF".to_string(), width: 1, subscript: None };
    pub static ref ZF: Lvalue = Lvalue::Variable{ name: "ZF".to_string(), width: 1, subscript: None };
    pub static ref SF: Lvalue = Lvalue::Variable{ name: "SF".to_string(), width: 1, subscript: None };
    pub static ref TF: Lvalue = Lvalue::Variable{ name: "TF".to_string(), width: 1, subscript: None };
    pub static ref IF: Lvalue = Lvalue::Variable{ name: "IF".to_string(), width: 1, subscript: None };
    pub static ref DF: Lvalue = Lvalue::Variable{ name: "DF".to_string(), width: 1, subscript: None };
    pub static ref OF: Lvalue = Lvalue::Variable{ name: "OF".to_string(), width: 1, subscript: None };
    pub static ref RF: Lvalue = Lvalue::Variable{ name: "RF".to_string(), width: 1, subscript: None };
    pub static ref IOPL: Lvalue = Lvalue::Variable{ name: "IOPL".to_string(), width: 0, subscript: None };
    pub static ref NT: Lvalue = Lvalue::Variable{ name: "NT".to_string(), width: 0, subscript: None };
    pub static ref VM: Lvalue = Lvalue::Variable{ name: "VM".to_string(), width: 0, subscript: None };
    pub static ref AC: Lvalue = Lvalue::Variable{ name: "AC".to_string(), width: 0, subscript: None };
    pub static ref VIF: Lvalue = Lvalue::Variable{ name: "VIF".to_string(), width: 0, subscript: None };
    pub static ref VIP: Lvalue = Lvalue::Variable{ name: "VIP".to_string(), width: 0, subscript: None };
    pub static ref ID: Lvalue = Lvalue::Variable{ name: "ID".to_string(), width: 0, subscript: None };
}

// segment registers
lazy_static! {
    pub static ref cs: Lvalue = Lvalue::Variable{ name: "cs".to_string(), width: 0, subscript: None };
    pub static ref ds: Lvalue = Lvalue::Variable{ name: "ds".to_string(), width: 0, subscript: None };
    pub static ref fs: Lvalue = Lvalue::Variable{ name: "fs".to_string(), width: 0, subscript: None };
    pub static ref ss: Lvalue = Lvalue::Variable{ name: "ss".to_string(), width: 0, subscript: None };
    pub static ref gs: Lvalue = Lvalue::Variable{ name: "gs".to_string(), width: 0, subscript: None };
    pub static ref es: Lvalue = Lvalue::Variable{ name: "es".to_string(), width: 0, subscript: None };
}

// control registers
lazy_static! {
    pub static ref cr0: Lvalue = Lvalue::Variable{ name: "cr0".to_string(), width: 0, subscript: None };
    pub static ref cr1: Lvalue = Lvalue::Variable{ name: "cr1".to_string(), width: 0, subscript: None };
    pub static ref cr2: Lvalue = Lvalue::Variable{ name: "cr2".to_string(), width: 0, subscript: None };
    pub static ref cr3: Lvalue = Lvalue::Variable{ name: "cr3".to_string(), width: 0, subscript: None };
    pub static ref cr4: Lvalue = Lvalue::Variable{ name: "cr4".to_string(), width: 0, subscript: None };
    pub static ref cr8: Lvalue = Lvalue::Variable{ name: "cr8".to_string(), width: 0, subscript: None };
    pub static ref ldtr: Lvalue = Lvalue::Variable{ name: "ldtr".to_string(), width: 0, subscript: None };
    pub static ref gdtr: Lvalue = Lvalue::Variable{ name: "gdtr".to_string(), width: 0, subscript: None };
    pub static ref idtr: Lvalue = Lvalue::Variable{ name: "idtr".to_string(), width: 0, subscript: None };
}

// debug registers
lazy_static! {
    pub static ref dr0: Lvalue = Lvalue::Variable{ name: "dr0".to_string(), width: 0, subscript: None };
    pub static ref dr1: Lvalue = Lvalue::Variable{ name: "dr1".to_string(), width: 0, subscript: None };
    pub static ref dr2: Lvalue = Lvalue::Variable{ name: "dr2".to_string(), width: 0, subscript: None };
    pub static ref dr3: Lvalue = Lvalue::Variable{ name: "dr3".to_string(), width: 0, subscript: None };
    pub static ref dr4: Lvalue = Lvalue::Variable{ name: "dr4".to_string(), width: 0, subscript: None };
    pub static ref dr5: Lvalue = Lvalue::Variable{ name: "dr5".to_string(), width: 0, subscript: None };
    pub static ref dr6: Lvalue = Lvalue::Variable{ name: "dr6".to_string(), width: 0, subscript: None };
    pub static ref dr7: Lvalue = Lvalue::Variable{ name: "dr7".to_string(), width: 0, subscript: None };
}

static GLOBAL_AMD64_TEMPVAR_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn new_temp(bits: usize) -> Lvalue {
    Lvalue::Variable{
        name: format!("__temp{}",GLOBAL_AMD64_TEMPVAR_COUNT.fetch_add(1, Ordering::SeqCst)),
        width: bits as u16,
        subscript: None
    }
}

pub fn disassembler(bits: Mode) -> Rc<Disassembler<Amd64>> {
    let opsize_prfx = new_disassembler!(Amd64 =>
        [ 0x66 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.operand_size = OperandSize::ThirtyTwo,
                Mode::Long => st.configuration.operand_size = OperandSize::Sixteen,
                Mode::Protected => st.configuration.operand_size = OperandSize::Sixteen,
            }
            true
        });

    let addrsize_prfx = new_disassembler!(Amd64 =>
        [ 0x67 ] = |st: &mut State<Amd64>| {
            match st.configuration.mode {
                Mode::Real => st.configuration.address_size = AddressSize::ThirtyTwo,
                Mode::Long => st.configuration.address_size = AddressSize::ThirtyTwo,
                Mode::Protected => st.configuration.address_size = AddressSize::Sixteen,
            }
            true
        });

    let rep_prfx = new_disassembler!(Amd64 =>
        [ 0xf3 ] = |st: &mut State<Amd64>| { true });

    let lock_prfx = new_disassembler!(Amd64 =>
        [ 0xf0 ] = |st: &mut State<Amd64>| { true });

    let repx_prfx = new_disassembler!(Amd64 =>
        [ 0xf3 ] = |st: &mut State<Amd64>| { true },
        [ 0xf2 ] = |st: &mut State<Amd64>| { true });

    let rex_prfx = new_disassembler!(Amd64 =>
        [ "0100 w@. r@. x@. b@." ] = |st: &mut State<Amd64>| {
            st.configuration.rex = true;
            if st.get_group("w") == 1 {
                st.configuration.operand_size = OperandSize::SixtyFour;
            }
            true
        });

    let imm8 = new_disassembler!(Amd64 =>
        [ "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let imm16 = new_disassembler!(Amd64 =>
        [ imm8, "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let imm32 = new_disassembler!(Amd64 =>
        [ imm16, "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let imm48 = new_disassembler!(Amd64 =>
        [ imm32, "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            // XXX
            //uint64_t a = st.capture_groups.at("imm") & 0xffff;
            //st.state.imm = constant((a << 32) | st.capture_groups.at("imm") >> 16);
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let imm64 = new_disassembler!(Amd64 =>
        [ imm32, "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let imm = new_disassembler!(Amd64 =>
        [ "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Eight
        },
        [ "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Sixteen
        },
        [ "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::ThirtyTwo || st.configuration.operand_size == OperandSize::SixtyFour
        });

    let immlong = new_disassembler!(Amd64 =>
        [ "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Eight
        },
        [ "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::Sixteen
        },
        [ "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::ThirtyTwo
        },
        [ "imm@........", "imm@........", "imm@........", "imm@........",
          "imm@........", "imm@........", "imm@........", "imm@........" ] = |st: &mut State<Amd64>| {
            st.configuration.imm = Some(Rvalue::Constant(st.get_group("imm")));
            st.configuration.operand_size == OperandSize::SixtyFour
        });

    let moffs = new_disassembler!(Amd64 =>
        [ "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::Constant(st.get_group("moffs")));
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::Constant(st.get_group("moffs")));
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........",
          "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::Constant(st.get_group("moffs")));
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let moffs8 = new_disassembler!(Amd64 =>
        [ "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::Constant(st.get_group("moffs")));
            st.configuration.operand_size = OperandSize::Eight;
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::Constant(st.get_group("moffs")));
            st.configuration.operand_size = OperandSize::Eight;
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "moffs@........", "moffs@........", "moffs@........", "moffs@........",
          "moffs@........", "moffs@........", "moffs@........", "moffs@........" ] = |st: &mut State<Amd64>| {
            st.configuration.moffs = Some(Rvalue::Constant(st.get_group("moffs")));
            st.configuration.operand_size = OperandSize::Eight;
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let m64 = new_disassembler!(Amd64 =>
        [ "mq@........", "mq@........" ] = |st: &mut State<Amd64>| {
            st.configuration.rm = Some(decode::select_mem(&OperandSize::SixtyFour,Rvalue::Constant(st.get_group("mq"))));
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "mq@........", "mq@........", "mq@........", "mq@........" ] = |st: &mut State<Amd64>| {
            st.configuration.rm = Some(decode::select_mem(&OperandSize::SixtyFour,Rvalue::Constant(st.get_group("mq"))));
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "mq@........", "mq@........", "mq@........", "mq@........",
          "mq@........", "mq@........", "mq@........", "mq@........" ] = |st: &mut State<Amd64>| {
            st.configuration.rm = Some(decode::select_mem(&OperandSize::SixtyFour,Rvalue::Constant(st.get_group("mq"))));
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let m128 = new_disassembler!(Amd64 =>
        [ "mdq@........", "mdq@........" ] = |st: &mut State<Amd64>| {
            st.configuration.rm = Some(decode::select_mem(&OperandSize::HundredTwentyEight,Rvalue::Constant(st.get_group("mdq"))));
            st.configuration.address_size == AddressSize::Sixteen
        },
        [ "mdq@........", "mdq@........", "mdq@........", "mdq@........" ] = |st: &mut State<Amd64>| {
            st.configuration.rm = Some(decode::select_mem(&OperandSize::HundredTwentyEight,Rvalue::Constant(st.get_group("mdq"))));
            st.configuration.address_size == AddressSize::ThirtyTwo
        },
        [ "mdq@........", "mdq@........", "mdq@........", "mdq@........",
          "mdq@........", "mdq@........", "mdq@........", "mdq@........" ] = |st: &mut State<Amd64>| {
            st.configuration.rm = Some(decode::select_mem(&OperandSize::HundredTwentyEight,Rvalue::Constant(st.get_group("mdq"))));
            st.configuration.address_size == AddressSize::SixtyFour
        });

    let disp8 = new_disassembler!(Amd64 =>
        [ "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::Constant(st.get_group("disp")));
            true
        });

    let disp16 = new_disassembler!(Amd64 =>
        [ disp8, "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::Constant(st.get_group("disp")));
            true
        });

    let disp32 = new_disassembler!(Amd64 =>
        [ disp16, "disp@........", "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::Constant(st.get_group("disp")));
            true
        });

    let disp64 = new_disassembler!(Amd64 =>
        [ disp32, "disp@........", "disp@........", "disp@........", "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::Constant(st.get_group("disp")));
            true
        });

    let sib = new_disassembler!(Amd64 =>
        [ "scale@.. index@... base@101", "disp@........", "disp@........", "disp@........", "disp@........" ] = |st: &mut State<Amd64>| {
            st.configuration.disp = Some(Rvalue::Constant(st.get_group("disp")));
            true
        },
        [ "scale@.. index@... base@..." ] = |st: &mut State<Amd64>| { true });

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
                //st.configuration.reg = select_reg(st.configuration.operand_size,reg,st.configuration.rex);
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

            st.mnemonic(0,"internal-rm","",vec!(),&|cg: &mut CodeGen| {
                /*st.configuration.rm = decode_modrm(st.get_group("mod"),
                    b_rm,
                    st.configuration.disp,
                    sib,
                    st.configuration.operand_size,
                    st.configuration.address_size,
                    st.configuration.mode,
                    st.configuration.rex,
                    c);*/
                });
                true
            })
    }

    let rm = new_disassembler!(Amd64 =>
        [ "mod@00 reg@... rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 reg@... rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 reg@... rm@..."              ] = rm_semantic(None),
        [ "mod@01 reg@... rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@01 reg@... rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 reg@... rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 reg@... rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 reg@... rm@..."              ] = rm_semantic(None));

    let rmlong = new_disassembler!(Amd64 =>
        [ "mod@00 reg@... rm@101", disp32      ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@00 reg@... rm@100", sib         ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@00 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@01 reg@... rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@01 reg@... rm@...", disp8       ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@10 reg@... rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@10 reg@... rm@...", disp32      ] = rm_semantic(Some(OperandSize::SixtyFour)),
        [ "mod@11 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::SixtyFour)));

    let rmbyte = new_disassembler!(Amd64 =>
        [ "mod@00 reg@... rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 reg@... rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 reg@... rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 reg@... rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 reg@... rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 reg@... rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@11 reg@... rm@..."              ] = rm_semantic(Some(OperandSize::Eight)));

    // w/ extension opcode
    let rm0 = new_disassembler!(Amd64 =>
        [ "mod@00 000 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 000 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 000 rm@..."              ] = rm_semantic(None),
        [ "mod@01 000 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 000 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 000 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 000 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 000 rm@..."              ] = rm_semantic(None));

    let rmbyte0 = new_disassembler!(Amd64 =>
        [ "mod@00 000 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 000 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 000 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 000 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm1 = new_disassembler!(Amd64 =>
        [ "mod@00 001 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 001 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 001 rm@..."              ] = rm_semantic(None),
        [ "mod@01 001 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 001 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 001 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 001 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 001 rm@..."              ] = rm_semantic(None));

    let rmbyte1 = new_disassembler!(Amd64 =>
        [ "mod@00 001 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 001 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 001 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 001 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm2 = new_disassembler!(Amd64 =>
        [ "mod@00 010 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 010 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 010 rm@..."              ] = rm_semantic(None),
        [ "mod@01 010 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 010 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 010 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 010 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 010 rm@..."              ] = rm_semantic(None));

    let rmbyte2 = new_disassembler!(Amd64 =>
        [ "mod@00 010 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 010 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 010 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 010 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm3 = new_disassembler!(Amd64 =>
        [ "mod@00 011 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 011 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 011 rm@..."              ] = rm_semantic(None),
        [ "mod@01 011 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 011 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 011 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 011 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 011 rm@..."              ] = rm_semantic(None));

    let rmbyte3 = new_disassembler!(Amd64 =>
        [ "mod@00 011 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 011 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 011 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 011 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm4 = new_disassembler!(Amd64 =>
        [ "mod@00 100 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 100 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 100 rm@..."              ] = rm_semantic(None),
        [ "mod@01 100 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 100 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 100 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 100 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 100 rm@..."              ] = rm_semantic(None));

    let rmbyte4 = new_disassembler!(Amd64 =>
        [ "mod@00 100 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 100 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 100 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 100 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm5 = new_disassembler!(Amd64 =>
        [ "mod@00 101 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 101 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 101 rm@..."              ] = rm_semantic(None),
        [ "mod@01 101 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 101 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 101 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 101 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 101 rm@..."              ] = rm_semantic(None));

    let rmbyte5 = new_disassembler!(Amd64 =>
        [ "mod@00 101 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 101 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 101 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 101 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm6 = new_disassembler!(Amd64 =>
        [ "mod@00 110 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 110 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 110 rm@..."              ] = rm_semantic(None),
        [ "mod@01 110 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 110 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 110 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 110 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 110 rm@..."              ] = rm_semantic(None));

    let rmbyte6 = new_disassembler!(Amd64 =>
        [ "mod@00 110 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 110 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 110 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 110 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    let rm7 = new_disassembler!(Amd64 =>
        [ "mod@00 111 rm@101", disp32      ] = rm_semantic(None),
        [ "mod@00 111 rm@100", sib         ] = rm_semantic(None),
        [ "mod@00 111 rm@..."              ] = rm_semantic(None),
        [ "mod@01 111 rm@100", sib, disp8  ] = rm_semantic(None),
        [ "mod@01 111 rm@...", disp8       ] = rm_semantic(None),
        [ "mod@10 111 rm@100", sib, disp32 ] = rm_semantic(None),
        [ "mod@10 111 rm@...", disp32      ] = rm_semantic(None),
        [ "mod@11 111 rm@..."              ] = rm_semantic(None));

    let rmbyte7 = new_disassembler!(Amd64 =>
        [ "mod@00 111 rm@101", disp32      ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@100", sib         ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@00 111 rm@..."              ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@100", sib, disp8  ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@01 111 rm@...", disp8       ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@100", sib, disp32 ] = rm_semantic(Some(OperandSize::Eight)),
        [ "mod@10 111 rm@...", disp32      ] = rm_semantic(Some(OperandSize::Eight)));

    generic::integer_instructions(
        bits,
        lock_prfx, rep_prfx, repx_prfx, opsize_prfx,
        imm8, imm16, imm32, imm48, imm64, imm, immlong,
        moffs8, moffs,
        sib,
        rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7,
        rmbyte, rmbyte0, rmbyte1, rmbyte2, rmbyte3,
        rmbyte4, rmbyte5, rmbyte6, rmbyte7,
        rmlong, m64, m128,
        disp8, disp16, disp32, disp64)
}
