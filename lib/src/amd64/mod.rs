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

#[derive(Clone,PartialEq)]
pub enum AddressSize
{
    SixtyFour,
    ThirtyTwo,
    Sixteen,
}

#[derive(Clone,PartialEq)]
pub enum OperandSize
{
    SixtyFour,
    ThirtyTwo,
    Sixteen,
    Eight,
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
    static ref bl: Lvalue = Lvalue::Variable{ name: "bl".to_string(), width: 8, subscript: None };
    static ref cl: Lvalue = Lvalue::Variable{ name: "cl".to_string(), width: 8, subscript: None };
    static ref dl: Lvalue = Lvalue::Variable{ name: "dl".to_string(), width: 8, subscript: None };
    static ref r8l: Lvalue = Lvalue::Variable{ name: "r8l".to_string(), width: 8, subscript: None };
    static ref r9l: Lvalue = Lvalue::Variable{ name: "r9l".to_string(), width: 8, subscript: None };
    static ref r10: Lvalue = Lvalue::Variable{ name: "r10l".to_string(), width: 8, subscript: None };
    static ref r11l: Lvalue = Lvalue::Variable{ name: "r11l".to_string(), width: 8, subscript: None };
    static ref r12l: Lvalue = Lvalue::Variable{ name: "r12l".to_string(), width: 8, subscript: None };
    static ref r13l: Lvalue = Lvalue::Variable{ name: "r13l".to_string(), width: 8, subscript: None };
    static ref r14l: Lvalue = Lvalue::Variable{ name: "r14l".to_string(), width: 8, subscript: None };
    static ref r15l: Lvalue = Lvalue::Variable{ name: "r15l".to_string(), width: 8, subscript: None };
    static ref spl: Lvalue = Lvalue::Variable{ name: "spl".to_string(), width: 8, subscript: None };
    static ref bpl: Lvalue = Lvalue::Variable{ name: "bpl".to_string(), width: 8, subscript: None };
    static ref sil: Lvalue = Lvalue::Variable{ name: "sill".to_string(), width: 8, subscript: None };
    static ref dil: Lvalue = Lvalue::Variable{ name: "dil".to_string(), width: 8, subscript: None };
    static ref ah: Lvalue = Lvalue::Variable{ name: "ah".to_string(), width: 8, subscript: None };
    static ref bh: Lvalue = Lvalue::Variable{ name: "bh".to_string(), width: 8, subscript: None };
    static ref ch: Lvalue = Lvalue::Variable{ name: "ch".to_string(), width: 8, subscript: None };
    static ref dh: Lvalue = Lvalue::Variable{ name: "dh".to_string(), width: 8, subscript: None };
}



/*
        // 16 bit gp registers
        extern const rvalue ax,bx,cx,dx,
                                     r8w,r9w,r10w,r11w,r12w,r13w,r14w,r15w,
                                     si,di,sp,bp;
        // 32 bit gp registers
        extern const rvalue eax,ebx,ecx,edx,
                                     esi,edi,
                                     r8d,r9d,r10d,r11d,r12d,r13d,r14d,r15d;
        // 64 bit gp registers
        extern const rvalue rax,rbx,rcx,rdx,
                                     rsi,rdi,
                                     r4,r5,r6,r7,r8,r9,r10,r11,r12,r13,r14,r15;

        // 16 bit management registers
        extern const rvalue sp,bp,ip/*,eflags*/;

        // 32 bit management registers
        extern const rvalue esp,ebp,eip,/*eflags,*/CF,PF,AF,ZF,SF,TF,IF,DF,OF,IOPL,NT,RF,VM,AC,VIF,VIP,ID;

        // 64 bit management registers
        extern const rvalue rsp,rbp,rip,rflags;

        // segment registers
        extern const rvalue cs, ds, fs, ss, gs, es;

        // control registers
        extern const rvalue cr0, cr1, cr2, cr3, cr4, cr8, ldtr, gdtr, idtr;

        // debug registers
        extern const rvalue dr0, dr1, dr2, dr3, dr4, dr5, dr6, dr7;*/

pub fn disassembler(bits: u8) -> Rc<Disassembler<Amd64>> {
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

            st.mnemonic(0,"internal-rm","",vec!(),|cg: &mut CodeGen| {
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

    let (main, mainrep, mainrepx) = generic::add_generic(
        bits,
        lock_prfx,
        imm8, imm16, imm32, imm48, imm64, imm,
        moffs8, moffs,
        sib,
        rm, rm0, rm1, rm2, rm3, rm4, rm5, rm6, rm7,
        rmbyte, rmbyte0, rmbyte1, rmbyte2, rmbyte3,
        rmbyte4, rmbyte5, rmbyte6, rmbyte7,
        rmlong,
        disp8, disp16, disp32, disp64);

    if(bits == 64)
    {
        new_disassembler!(Amd64 =>
            [ opt!(opsize_prfx), opt!(rex_prfx), main ] = |_: &mut State<Amd64>| { true },
            [ opt!(rep_prfx), opt!(opsize_prfx), opt!(rep_prfx), opt!(rex_prfx), mainrep ] = |_: &mut State<Amd64>| { true },
            [ opt!(rep_prfx), opt!(opsize_prfx), opt!(repx_prfx), opt!(rex_prfx), mainrepx ] = |_: &mut State<Amd64>| { true })
    }
    else
    {
        new_disassembler!(Amd64 =>
            [ opt!(rep_prfx), opt!(opsize_prfx), opt!(rep_prfx), mainrep ] = |_: &mut State<Amd64>| { true },
            [ opt!(rep_prfx), opt!(opsize_prfx), opt!(repx_prfx), mainrepx ] = |_: &mut State<Amd64>| { true },
            [ opt!(opsize_prfx), main ] = |_: &mut State<Amd64>| { true })
    }
}
