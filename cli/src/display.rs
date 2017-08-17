use std::io::Write;
use termcolor::WriteColor;
use termcolor::Color::*;

use panopticon_core::{Function, BasicBlock, Mnemonic, MnemonicFormatToken, Operation, Program, Rvalue, Result, Statement};

macro_rules! color_bold {
    ($fmt:ident, $color:ident, $str:expr) => ({
    $fmt.set_color(::termcolor::ColorSpec::new().set_bold(true).set_fg(Some($color)))?;
    write!($fmt, "{}", $str)?;
    $fmt.reset()
    })
}

macro_rules! color {
    ($fmt:ident, $color:ident, $str:expr) => ({
        $fmt.set_color(::termcolor::ColorSpec::new().set_fg(Some($color)))?;
        write!($fmt, "{}", $str)?;
        $fmt.reset()
    })
}

/// Prints a sorted-by-start list of the RREIL implementing each mnemonic in a basic block, as well as phi functions and init code
pub fn print_rreil<W: Write + WriteColor>(fmt: &mut W, bbs: &[&BasicBlock]) -> Result<()> {
    color_bold!(fmt, White, "RREIL")?;
    writeln!(fmt, ":")?;
    for bb in bbs {
        for mnemonic in bb.mnemonics() {
            print_address_and_mnemonic(fmt, mnemonic)?;
            for statement in &mnemonic.instructions {
                print_statement(fmt, statement)?;
            }
        }
    }
    Ok(())
}

/// Prints an address and its corresponding mnemonic at that address
pub fn print_address_and_mnemonic<W: Write + WriteColor>(fmt: &mut W, mnemonic: &Mnemonic) -> Result<()> {
    color_bold!(fmt, White, format!("{:8x}", mnemonic.area.start as usize))?;
    write!(fmt, ": (")?;
    print_mnemonic(fmt, mnemonic, None)?;
    writeln!(fmt, ")")?;
    Ok(())
}

/// Print colored RREIL statement
pub fn print_statement<W: Write + WriteColor>(fmt: &mut W, statement: &Statement) -> Result<()> {
    write!(fmt, "{: <8}  ", "")?;
    match statement.op {
        Operation::Add(ref a, ref b) => {
            color_bold!(fmt, White, "add")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Subtract(ref a, ref b) => {
            color_bold!(fmt, White, "sub")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Multiply(ref a, ref b) => {
            color_bold!(fmt, White, "mul")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::DivideUnsigned(ref a, ref b) => {
            color_bold!(fmt, White, "divu")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::DivideSigned(ref a, ref b) => {
            color_bold!(fmt, White, "divs")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::ShiftLeft(ref a, ref b) => {
            color_bold!(fmt, White, "shl")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::ShiftRightUnsigned(ref a, ref b) => {
            color_bold!(fmt, White, "shru")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::ShiftRightSigned(ref a, ref b) => {
            color_bold!(fmt, White, "shrs")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Modulo(ref a, ref b) => {
            color_bold!(fmt, White, "mod")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::And(ref a, ref b) => {
            color_bold!(fmt, White, "and")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::InclusiveOr(ref a, ref b) => {
            color_bold!(fmt, White, "or")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::ExclusiveOr(ref a, ref b) => {
            color_bold!(fmt, White, "xor")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Equal(ref a, ref b) => {
            color_bold!(fmt, White, "cmpeq")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::LessOrEqualUnsigned(ref a, ref b) => {
            color_bold!(fmt, White, "cmpleu")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::LessOrEqualSigned(ref a, ref b) => {
            color_bold!(fmt, White, "cmples")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::LessUnsigned(ref a, ref b) => {
            color_bold!(fmt, White, "cmplu")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::LessSigned(ref a, ref b) => {
            color_bold!(fmt, White, "cmpls")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Move(ref a) => {
            color_bold!(fmt, White, "mov")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
        },
        Operation::Call(ref a) => {
            color_bold!(fmt, White, "call")?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
        },
        Operation::ZeroExtend(s, ref a) => {
            color_bold!(fmt, White, format!("convert_{}", s))?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
        },
        Operation::SignExtend(s, ref a) => {
            color_bold!(fmt, White, format!("sign-extend_{}", s))?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
        },
        Operation::Select(s, ref a, ref b) => {
            color_bold!(fmt, White, format!("select_{}", s))?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, a)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Load(ref r, ref b) => {
            color_bold!(fmt, White, format!("load_{}", r))?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Store(ref r, ref b) => {
            color_bold!(fmt, White, format!("store_{}", r))?;
            write!(fmt, " ")?;
            color!(fmt, White, statement.assignee)?;
            color_bold!(fmt, Green, ",")?;
            write!(fmt, " ")?;
            color!(fmt, White, b)?;
        },
        Operation::Phi(ref vec) => {
            color_bold!(fmt, White, format!("phi"))?;
            write!(fmt, " ")?;
            for (i, x) in vec.iter().enumerate() {
                color!(fmt, White, format!("{}", x))?;
                if i < vec.len() - 1 {
                    color_bold!(fmt, Green, ",")?;
                    write!(fmt, " ")?;
                }
            }
        }
    }
    writeln!(fmt, "")?;
    Ok(())
}

/// Prints the function in a human readable format, using `program`, with colors
pub fn print_function<W: Write + WriteColor>(fmt: &mut W, function: &Function, bbs: &[&BasicBlock], program: &Program) -> Result<()> {
    write!(fmt, "{:0>8x} <", function.start())?;
    color_bold!(fmt, Yellow, function.name)?;
    writeln!(fmt, ">:")?;
    for bb in bbs {
        print_basic_block(fmt, &bb, program)?;
    }
    Ok(())
}

/// Prints the basic block into `fmt`, in disassembly order, in human readable form, and looks up any functions calls in `program`
pub fn print_basic_block<W: Write + WriteColor>(fmt: &mut W, basic_block: &BasicBlock, program: &Program) -> Result<()> {
    for mnemonic in basic_block.mnemonics.iter() {
        if !mnemonic.opcode.starts_with("__") {
            write!(fmt, "{:8x}: ", mnemonic.area.start)?;
            print_mnemonic(fmt, &mnemonic, Some(program))?;
            writeln!(fmt)?;
        }
    }
    Ok(())
}

/// Prints the mnemonic into `fmt`, in human readable form, and looks up any functions calls in `program`
pub fn print_mnemonic<W: Write + WriteColor>(fmt: &mut W, mnemonic: &Mnemonic, program: Option<&Program>) -> Result<()> {
    let mut ops = mnemonic.operands.iter();
    color_bold!(fmt, Blue, mnemonic.opcode)?;
    write!(fmt, " ")?;
    for token in &mnemonic.format_string {
        match token {
            &MnemonicFormatToken::Literal(ref s) => {
                color_bold!(fmt, Green, s)?;
            },
            &MnemonicFormatToken::Variable{ ref has_sign } => {
                match ops.next() {
                    Some(&Rvalue::Constant{ value: c, size: s }) => {
                        let val =
                            if s < 64 {
                                let res = 1u64 << s;
                                c % res
                            } else { c };
                        let sign_bit = if s < 64 { 1u64 << (s - 1) } else { 0x8000000000000000 };
                        if !has_sign || val & sign_bit == 0 {
                            color!(fmt, Red, format!("{:x}", val))?;
                        } else {
                            color!(fmt, White, format!("{:x}", (val as i64).wrapping_neg()))?;
                        }
                    },
                    Some(&Rvalue::Variable{ ref name, subscript: Some(ref _subscript),.. }) => {
                        color_bold!(fmt, White, &name.to_lowercase())?;
                    },
                    _ => {
                        color!(fmt, Black, "?")?;
                    }
                }
            },
            &MnemonicFormatToken::Pointer{ is_code,.. } => {
                match ops.next() {
                    Some(&Rvalue::Constant{ value: c, size: s }) => {
                        let val =
                            if s < 64 {
                                let res = 1u64 << s;
                                c % res
                            } else { c };
                        if is_code {
                            if let Some(program) = program {
                                if let Some(function) = program.find_function_by(|f| { f.start() == val }) {
                                    color!(fmt, Red, format!("{:x}",val))?;
                                    write!(fmt, " <", )?;
                                    color_bold!(fmt, Yellow, function.name)?;
                                    write!(fmt, ">")?;
                                } else {
                                    color_bold!(fmt, Magenta, format!("{:x}",val))?;
                                }
                            } else {
                                write!(fmt, "{}", format!("{:#x}",val))?;
                            }
                        } else {
                            write!(fmt, "{}", format!("{:#x}",val))?;
                        }
                    },
                    Some(&Rvalue::Variable{ ref name, subscript: Some(_),.. }) => {
                        color!(fmt, Yellow, name.to_lowercase())?;
                    },
                    Some(&Rvalue::Variable{ ref name, .. }) => {
                        color!(fmt, Yellow, name.to_lowercase())?;
                    },
                    Some(&Rvalue::Undefined) => {
                        color_bold!(fmt, Red, "undefined")?;
                    },
                    None => {
                        color!(fmt, Black, "?")?;
                    }
                }
            }
        }
    }
    Ok(())
}
