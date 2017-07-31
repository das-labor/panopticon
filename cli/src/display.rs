use colored::*;

use panopticon_core::{Function, BasicBlock, Mnemonic, MnemonicFormatToken, Program, Rvalue};

/// Displays the function in a human readable format, using `program`
pub fn display_function(function: &Function, program: &Program) -> String {
    let mut bbs = function.basic_blocks().collect::<Vec<&BasicBlock>>();
    bbs.sort_by(|bb1, bb2| bb1.area.start.cmp(&bb2.area.start));
    let mut fmt = {
        let &BasicBlock { ref area, .. } = function.entry_point();
        format!("{:0>8x} <{}>:", area.start, function.name.bold().yellow())
    };
    for bb in bbs {
        fmt = format!("{}{}", fmt, display_basic_block(&bb, program));
    }
    fmt
}

/// Displays the basic block in disassembly order, in human readable form, and looks up any functions calls in `program`
pub fn display_basic_block(basic_block: &BasicBlock, program: &Program) -> String {
    let seed = String::new();
    let display = basic_block.mnemonics.iter().filter_map(|x| {
        if x.opcode.starts_with("__") {
            None
        } else {
            Some(x)
        }
    }).collect::<Vec<_>>();
    display.iter().fold(seed, |acc, ref m| -> String {
        format!("{}\n{}", acc, display_mnemonic(&m, program))
    })
}

/// Displays the mnemonic in human readable form, and looks up any functions calls in `program`
pub fn display_mnemonic(mnemonic: &Mnemonic, program: &Program) -> String {
    let mut ops = mnemonic.operands.iter();
    let mut fmt = format!( "{:8x}: {} ", mnemonic.area.start, mnemonic.opcode.bold().blue());
    for token in &mnemonic.format_string {
        match token {
            &MnemonicFormatToken::Literal(ref s) => {
                fmt = format!( "{}{}", fmt, format!("{}", s).bold().green());
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
                            fmt = format!( "{}{}", fmt, format!("{:x}",val).red());
                        } else {
                            fmt = format!( "{}{:x}", fmt, (val as i64).wrapping_neg());
                        }
                    },
                    Some(&Rvalue::Variable{ ref name, subscript: Some(ref _subscript),.. }) => {
                        fmt = format!( "{}{}", fmt, &name.to_lowercase().bold().white());
                    },
                    _ => {
                        fmt = format!( "{}?", fmt);
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
                        let display = if is_code {
                            if let Some(function) = program.find_function_by(|f| { f.start() == val }) {
                                format!("{}{} <{}>", fmt, format!("{:x}",val).red(), function.name.yellow().bold())
                            } else {
                                format!("{}{}", fmt, format!("{:x}",val).bold().purple())
                            }
                        } else {
                            format!("{}{:#x}", fmt, val)
                        };
                        fmt = display;
                    },
                    Some(&Rvalue::Variable{ ref name, subscript: Some(_),.. }) => {
                        fmt = format!( "{}{}", fmt, name.to_lowercase().yellow());
                    },
                    Some(&Rvalue::Variable{ ref name, .. }) => {
                        fmt = format!( "{}{}", fmt, name.to_lowercase().yellow());
                    },
                    Some(&Rvalue::Undefined) => {
                        fmt = format!( "{}{}", fmt, "undefined".bold().red());
                    },
                    None => {
                        fmt = format!( "{}{}", fmt, "?".black().dimmed());
                    }
                }
            }
        }
    }
    fmt
}
