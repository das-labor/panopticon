use std::io::Write;
use atty;
use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};
use termcolor::Color::*;

use panopticon_core::{Function, BasicBlock, Mnemonic, MnemonicFormatToken, Program, Rvalue, Result};

macro_rules! color_bold {
    ($fmt:ident, $color:ident, $str:expr) => ({
    $fmt.set_color(ColorSpec::new().set_bold(true).set_fg(Some($color)))?;
    write!($fmt, "{}", $str)?;
    $fmt.reset()
    })
}

macro_rules! color {
    ($fmt:ident, $color:ident, $str:expr) => ({
        $fmt.set_color(ColorSpec::new().set_fg(Some($color)))?;
        write!($fmt, "{}", $str)?;
        $fmt.reset()
    })
}

/// Prints the function in a human readable format, using `program`, with colors. If `always_color` is set, it will force printing, even to non ttys.
pub fn print_function(function: &Function, program: &Program, always_color: bool) -> Result<()> {
    let cc = if always_color || atty::is(atty::Stream::Stdout) { ColorChoice::Auto } else { ColorChoice::Never };
    let writer = BufferWriter::stdout(cc);
    let mut fmt = writer.buffer();
    let mut bbs = function.basic_blocks().collect::<Vec<&BasicBlock>>();
    bbs.sort_by(|bb1, bb2| bb1.area.start.cmp(&bb2.area.start));
    write!(fmt, "{:0>8x} <", function.start())?;
    color_bold!(fmt, Yellow, function.name)?;
    writeln!(fmt, ">:")?;
    for bb in bbs {
        display_basic_block(&mut fmt, &bb, program)?;
    }
    writer.print(&fmt)?;
    Ok(())
}

/// Prints the basic block into `fmt`, in disassembly order, in human readable form, and looks up any functions calls in `program`
pub fn display_basic_block<W: Write + WriteColor>(fmt: &mut W, basic_block: &BasicBlock, program: &Program) -> Result<()> {
    for x in basic_block.mnemonics.iter() {
        if !x.opcode.starts_with("__") {
            display_mnemonic(fmt, &x, program)?;
        }
    }
    Ok(())
}

/// Prints the mnemonic into `fmt`, in human readable form, and looks up any functions calls in `program`
pub fn display_mnemonic<W: Write + WriteColor>(fmt: &mut W, mnemonic: &Mnemonic, program: &Program) -> Result<()> {
    let mut ops = mnemonic.operands.iter();
    write!(fmt, "{:8x}: ", mnemonic.area.start)?;
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
    writeln!(fmt)?;
    Ok(())
}
