use std::io::Write;
use termcolor::WriteColor;
use termcolor::Color::*;
use std::ops::Range;

use panopticon_core::{Fun, Function, BasicBlock, Mnemonic, MnemonicFormatToken, Operation, Program, Rvalue, Result, Statement, neo};

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

pub trait PrintableFunction: Sized {
    fn pretty_print<W: WriteColor + Write>(&self, fmt: &mut W, program: &Program<Self>) -> Result<()>;
}

pub trait PrintableStatements<'a>: Sized {
    fn pretty_print_il<IL: PrintableIL, W: WriteColor + Write> (&'a self, fmt: &mut W) -> Result<()>;
}

pub trait PrintableIL {
    fn pretty_print<W: WriteColor + Write>(&self, fmt: &mut W) -> Result<()>;
}

impl PrintableFunction for Function {
    fn pretty_print<W: WriteColor + Write>(&self, fmt: &mut W, program: &Program<Function>) -> Result<()> {
        let mut bbs = self.basic_blocks().collect::<Vec<_>>();
        bbs.sort_by(|bb1, bb2| bb1.area.start.cmp(&bb2.area.start));
        print_function(fmt, self, &bbs, program)
    }
}

impl<'a> PrintableStatements<'a> for Function {
    fn pretty_print_il<IL: PrintableIL, W: WriteColor + Write>(&'a self, fmt: &mut W) -> Result<()> {
        color_bold!(fmt, White, "RREIL")?;
        writeln!(fmt, ":")?;
        for bb in self.basic_blocks() {
            for mnemonic in bb.mnemonics() {
                print_address_and_mnemonic::<Self, &Mnemonic, _>(fmt, &mnemonic)?;
                for statement in &mnemonic.instructions {
                    <Statement as PrintableIL>::pretty_print(statement, fmt)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a, IL: neo::Language<'a> + Default> PrintableStatements<'a> for neo::Function<IL>
    where
        IL::Statement: PrintableIL + Clone,
        Self: Fun
{
    fn pretty_print_il<ILL: PrintableIL, W: WriteColor + Write>(&'a self, fmt: &mut W) -> Result<()> {
        color_bold!(fmt, White, "IL")?;
        writeln!(fmt, ":")?;
        for bb in self.into_iter() {
            for mnemonic in bb {
                print_address_and_mnemonic::<Self, &neo::Mnemonic, W>(fmt, &mnemonic.mnemonic)?;
                for statement in mnemonic {
                    <IL::Statement as PrintableIL>::pretty_print(&statement, fmt)?;
                }
            }
        }
//        for bb in self.basic_blocks() {
//            for (idx, ref mnemonic) in self.mnemonics(bb) {
//                print_address_and_mnemonic::<Self, &neo::Mnemonic, W>(fmt, mnemonic)?;
//                for statement in neo::Function::statements(self,idx) {
//                    let statement: &<IL as neo::Language<'a>>::Statement = ::std::mem::transmute(&statement);
//                    <IL::Statement as PrintableIL>::pretty_print(statement, fmt)?;
//                }
//            }
//        }
        Ok(())
    }
}

//impl PrintableStatements for neo::Function<neo::Bitcode> {
//    fn pretty_print_il<IL: PrintableIL, W: WriteColor + Write>(&self, fmt: &mut W) -> Result<()> {
//        color_bold!(fmt, White, "Bitcode")?;
//        writeln!(fmt, ":")?;
//        for bb in self.into_iter() {
//            for mnemonic in bb {
//                //print_address_and_mnemonic::<Self, &Mnemonic, _>(fmt, &mnemonic)?;
//                for statement in mnemonic {
//                    <neo::Statement as PrintableIL>::pretty_print(&statement, fmt)?;
//                }
//            }
//        }
//        Ok(())
//    }
//}

impl PrintableFunction for neo::Function<Vec<Statement>> {
    fn pretty_print<W: WriteColor + Write>(&self, fmt: &mut W, program: &Program<Self>) -> Result<()> {
        let mut bbs = self.basic_blocks().map(|(_, bb)| NeoFunctionAndBasicBlock { function: self, bb} ).collect::<Vec<_>>();
        bbs.sort_by(|f1, f2| f1.bb.area.start.cmp(&f2.bb.area.start));
        print_function(fmt, self, &bbs, program)?;
        Ok(())
    }
}

impl PrintableFunction for neo::Function<neo::Bitcode> {
    fn pretty_print<W: WriteColor + Write>(&self, fmt: &mut W, program: &Program<neo::Function<neo::Bitcode>>) -> Result<()> {
        let mut bbs = self.basic_blocks().map(|(_, bb)| NeoFunctionAndBasicBlock { function: self, bb} ).collect::<Vec<_>>();
        bbs.sort_by(|f1, f2| f1.bb.area.start.cmp(&f2.bb.area.start));
        print_function(fmt, self, &bbs, program)?;
        Ok(())
    }
}

pub trait PrintableMnemonic {
    fn opcode(&self) -> &str;
    fn operands(&self) -> Vec<Rvalue>;
    fn format_tokens(&self) -> &[MnemonicFormatToken];
    fn area(&self) -> Range<u64>;
}

impl<'a> PrintableMnemonic for &'a Mnemonic {
    fn opcode(&self) -> &str {
        self.opcode.as_str()
    }
    fn operands(&self) -> Vec<Rvalue> {
        self.operands.clone()
    }
    fn format_tokens(&self) -> &[MnemonicFormatToken] {
        &self.format_string
    }
    fn area(&self) -> Range<u64> {
        self.area.start..self.area.end
    }
}

pub trait PrintableBlock<M: PrintableMnemonic> {
    type Iter: Iterator<Item = M>;
    fn mnemonics(&self) -> Self::Iter;
}

impl<'a> PrintableBlock<&'a Mnemonic> for &'a BasicBlock {
    type Iter = ::std::slice::Iter<'a, Mnemonic>;
    fn mnemonics(&self) -> Self::Iter {
        self.mnemonics.as_slice().iter()
    }
}

struct NeoFunctionAndBasicBlock<'a, IL: 'a> {
    function: &'a neo::Function<IL>,
    bb: &'a neo::BasicBlock,
}

impl<'a, IL: neo::Language<'a> + Default> PrintableBlock<&'a neo::Mnemonic> for NeoFunctionAndBasicBlock<'a, IL> {
    type Iter = Box<Iterator<Item = &'a neo::Mnemonic> + 'a>;
    fn mnemonics(&self) -> Self::Iter {
        Box::new(self.function.mnemonics_for(self.bb).map(|(_, m)| m))
    }
}

impl<'a> PrintableMnemonic for &'a neo::Mnemonic {
    fn opcode(&self) -> &str {
        use std::borrow::Borrow;
        self.opcode.borrow()
    }
    fn operands(&self) -> Vec<Rvalue> {
        self.operands.clone()
    }
    fn format_tokens(&self) -> &[MnemonicFormatToken] {
        &self.format_string
    }
    fn area(&self) -> Range<u64> {
        self.area.start..self.area.end
    }
}

///// Prints a sorted-by-start list of the RREIL implementing each mnemonic in a basic block, as well as phi functions and init code
// pub fn print_il<IL: PrintableIL, M: PrintableMnemonic, B: PrintableBlock<M>, W: Write + WriteColor>(fmt: &mut W, bbs: &[&B]) -> Result<()> {
//     color_bold!(fmt, White, "RREIL")?;
//     writeln!(fmt, ":")?;
//     for bb in bbs {
//         for mnemonic in bb.mnemonics() {
//             print_address_and_mnemonic(fmt, mnemonic)?;
//             for statement in mnemonic.instructions() {
//                 statement.pretty_print(fmt)?;
//             }
//         }
//     }
//     Ok(())
// }

/// Prints an address and its corresponding mnemonic at that address
pub fn print_address_and_mnemonic<F: Fun, M: PrintableMnemonic, W: Write + WriteColor>(fmt: &mut W, mnemonic: &M) -> Result<()> {
    color_bold!(fmt, White, format!("{:8x}", mnemonic.area().start as usize))?;
    write!(fmt, ": (")?;
    print_mnemonic(fmt, mnemonic, None::<&Program<F>>)?;
    writeln!(fmt, ")")?;
    Ok(())
}

/// Print colored RREIL statement
impl PrintableIL for Statement {
    fn pretty_print<W: Write + WriteColor>(&self, fmt: &mut W) -> Result<()> {
        write!(fmt, "{: <8}  ", "")?;
        match self.op {
            Operation::Add(ref a, ref b) => {
                color_bold!(fmt, White, "add")?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
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
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, a)?;
            },
            Operation::Call(ref a) => {
                color_bold!(fmt, White, "call")?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, a)?;
            },
            Operation::ZeroExtend(s, ref a) => {
                color_bold!(fmt, White, format!("convert_{}", s))?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, a)?;
            },
            Operation::SignExtend(s, ref a) => {
                color_bold!(fmt, White, format!("sign-extend_{}", s))?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, a)?;
            },
            Operation::Select(s, ref a, ref b) => {
                color_bold!(fmt, White, format!("select_{}", s))?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, a)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, b)?;
            },
            Operation::Initialize(ref r, sz) => {
                color_bold!(fmt, White, "init")?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, r)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, sz)?;
            },
            Operation::Load(ref r, e, sz, ref b) => {
                color_bold!(fmt, White, format!("load/{}/{}/{}", r, e, sz))?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, b)?;
            },
            Operation::Store(ref r, e, sz, ref a, ref b) => {
                color_bold!(fmt, White, format!("store/{}/{}/{}", r, e, sz))?;
                write!(fmt, " ")?;
                color!(fmt, White, self.assignee)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                color!(fmt, White, a)?;
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
}

/// Prints the function in a human readable format, using `program`, with colors
pub fn print_function<Function: Fun, M: PrintableMnemonic, B: PrintableBlock<M>, W: Write + WriteColor>(fmt: &mut W, function: &Function, bbs: &[B], program: &Program<Function>) -> Result<()> {
    write!(fmt, "{:0>8x} <", function.start())?;
    color_bold!(fmt, Yellow, function.name())?;
    writeln!(fmt, ">:")?;
    for bb in bbs {
        print_basic_block(fmt, bb, program)?;
    }
    Ok(())
}

/// Prints the basic block into `fmt`, in disassembly order, in human readable form, and looks up any functions calls in `program`
pub fn print_basic_block<Function: Fun, M: PrintableMnemonic, B: PrintableBlock<M>, W: Write + WriteColor>(fmt: &mut W, basic_block: &B, program: &Program<Function>) -> Result<()> {
    for mnemonic in basic_block.mnemonics() {
        if !mnemonic.opcode().starts_with("__") {
            write!(fmt, "{:8x}: ", mnemonic.area().start)?;
            print_mnemonic(fmt, &mnemonic, Some(program))?;
            writeln!(fmt)?;
        }
    }
    Ok(())
}

/// Prints the mnemonic into `fmt`, in human readable form, and looks up any functions calls in `program`
pub fn print_mnemonic<Function: Fun, M: PrintableMnemonic, W: Write + WriteColor>(fmt: &mut W, mnemonic: &M, program: Option<&Program<Function>>) -> Result<()> {
    let ops = mnemonic.operands();
    let mut ops = ops.iter();
    color_bold!(fmt, Blue, mnemonic.opcode())?;
    write!(fmt, " ")?;
    for token in mnemonic.format_tokens() {
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
                                    color_bold!(fmt, Yellow, function.name())?;
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

fn pp_variable<W: WriteColor + Write>(fmt: &mut W, variable: &neo::Variable) -> Result<()> {
    color!(fmt, White, format!("{}", variable.name))?;
    if let Some(mut subscript ) = variable.subscript {
        while subscript >= 0 {
            match subscript % 10 {
                0 => { color_bold!(fmt, White, "₀")?; },
                1 => { color_bold!(fmt, White, "₁")?; },
                2 => { color_bold!(fmt, White, "₂")?; },
                3 => { color_bold!(fmt, White, "₃")?; },
                4 => { color_bold!(fmt, White, "₄")?; },
                5 => { color_bold!(fmt, White, "₅")?; },
                6 => { color_bold!(fmt, White, "₆")?; },
                7 => { color_bold!(fmt, White, "₇")?; },
                8 => { color_bold!(fmt, White, "₈")?; },
                9 => { color_bold!(fmt, White, "₉")?; },
                _ => unreachable!()
            }
            subscript /= 10;
        }
    }
    write!(fmt, ":{}", variable.bits)?;
    Ok(())
}

fn pp_constant<W: WriteColor + Write>(fmt: &mut W, constant: &neo::Constant) -> Result<()> {
    color_bold!(fmt, Magenta, format!("{:#x}", constant.value))?;
    write!(fmt, ":{}", constant.bits)?;
    Ok(())
}

fn pp_value<W: WriteColor + Write>(fmt: &mut W, variable: &neo::Value) -> Result<()> {
    use neo::Value::*;
    match variable {
        &Undefined => {
            color_bold!(fmt, Red, "undef")?;
        },
        &Variable(ref variable) => {
            pp_variable(fmt, variable)?;
        },
        &Constant(ref constant) => {
            pp_constant(fmt, constant)?;
        }
    }
    Ok(())
}

macro_rules! pp_op {
    ($fmt:ident, $name:expr, $result:expr, $format:expr) => ({
                        color_bold!($fmt, White, $name)?;
                        color_bold!($fmt, White, $format)?;
                        write!($fmt, " ")?;
                        pp_variable($fmt, $result)
        });
    ($fmt:ident, $name:expr, $result:expr) => ({
                        color_bold!($fmt, White, $name)?;
                        write!($fmt, " ")?;
                        pp_variable($fmt, $result)
        });

}

macro_rules! pp_operands {
    ($fmt:ident, $operands:expr) => ({
         for operand in $operands {
           color_bold!($fmt, Green, ",")?;
           write!($fmt, " ")?;
           pp_value($fmt, operand)?;
         }
    });
}

impl PrintableIL for neo::Statement {
    fn pretty_print<W: WriteColor + Write>(&self, fmt: &mut W) -> Result<()> {
        use neo::Statement::*;
        use neo::Operation::*;
        write!(fmt, "{: <8}  ", "")?;
        match self {
            &Expression { ref result, ref op } => {
                let operands = op.reads();
                match op {
                    &ZeroExtend(size, ..) => {
                        pp_op!(fmt, op.name(), result, format!("/{}", size))?;
                        pp_operands!(fmt, operands);
                    },
                    &SignExtend(size, ..) => {
                        pp_op!(fmt, op.name(), result, format!("/{}", size))?;
                        pp_operands!(fmt, operands);
                    },
                    &Initialize(ref name, size) => {
                        color_bold!(fmt, Red, name)?;
                        write!(fmt, "/{}", size)?;
                    },
                    &Select(size, ..)  => {
                        pp_op!(fmt, op.name(), result, format!("/{}", size))?;
                        pp_operands!(fmt, operands);
                    },
                    &Load(ref name, ref endianess, size, _) => {
                        pp_op!(fmt, op.name(), result, format!("/{}/{}/{}", name, if endianess == &neo::Endianess::Little { "le"} else { "big"}, size))?;
                        pp_operands!(fmt, operands);
                    },
                    _ => {
                        pp_op!(fmt, op.name(), result)?;
                        pp_operands!(fmt, operands);
                    }
                }
            },
            &Call { ref function } => {
                color_bold!(fmt, White, "call")?;
                //                color_bold!(fmt, White, "call")?;
//                write!(fmt, " ")?;
//                pp_variable(fmt, &result)?;
//                color_bold!(fmt, Green, ",")?;
//                write!(fmt, " ")?;
//                color!(fmt, White, a)?;

            },
            &IndirectCall { ref target } => {
                color_bold!(fmt, White, "icall")?;
                write!(fmt, " ")?;
                pp_value(fmt, target)?;
            },
            &Return => {
                color_bold!(fmt, White, "ret")?;
            },
            &Store {
                ref region,
                ref endianess,
                ref bytes,
                ref address,
                ref value,
            } => {
                color_bold!(fmt, White, format!("store/{}/{}/{}", region, if endianess == &neo::Endianess::Little { "le"} else { "big"}, bytes))?;
                write!(fmt, " ")?;
                pp_value(fmt, address)?;
                color_bold!(fmt, Green, ",")?;
                write!(fmt, " ")?;
                pp_value(fmt, value)?;

            }
        }
        writeln!(fmt, "")?;
        Ok(())
    }
}


/*
                    op @ Operation::Add( .. ) => {
                        pp_op!("add")?;
                        pp_operands!(op);
                          //color!(fmt, White, a)?;
//                        color_bold!(fmt, Green, ",")?;
//                        write!(fmt, " ")?;
//                        color!(fmt, White, b)?;
                    },
                    Operation::Subtract( a,  b) => {
                        color_bold!(fmt, White, "sub")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::Multiply( a,  b) => {
                        color_bold!(fmt, White, "mul")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::DivideUnsigned( a,  b) => {
                        color_bold!(fmt, White, "divu")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::DivideSigned( a,  b) => {
                        color_bold!(fmt, White, "divs")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::ShiftLeft( a,  b) => {
                        color_bold!(fmt, White, "shl")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::ShiftRightUnsigned( a,  b) => {
                        color_bold!(fmt, White, "shru")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::ShiftRightSigned( a,  b) => {
                        color_bold!(fmt, White, "shrs")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::Modulo( a,  b) => {
                        color_bold!(fmt, White, "mod")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::And( a,  b) => {
                        color_bold!(fmt, White, "and")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::InclusiveOr( a,  b) => {
                        color_bold!(fmt, White, "or")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::ExclusiveOr( a,  b) => {
                        color_bold!(fmt, White, "xor")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::Equal( a,  b) => {
                        color_bold!(fmt, White, "cmpeq")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::LessOrEqualUnsigned( a,  b) => {
                        color_bold!(fmt, White, "cmpleu")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::LessOrEqualSigned( a,  b) => {
                        color_bold!(fmt, White, "cmples")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::LessUnsigned( a,  b) => {
                        color_bold!(fmt, White, "cmplu")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::LessSigned( a,  b) => {
                        color_bold!(fmt, White, "cmpls")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::Move( a) => {
                        color_bold!(fmt, White, "mov")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                    },
                    Operation::ZeroExtend(s,  a) => {
                        color_bold!(fmt, White, format!("convert_{}", s))?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                    },
                    Operation::SignExtend(s,  a) => {
                        color_bold!(fmt, White, format!("sign-extend_{}", s))?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                    },
                    Operation::Select(s,  a,  b) => {
                        color_bold!(fmt, White, format!("select_{}", s))?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, a)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
                    Operation::Initialize( r, sz) => {
                        color_bold!(fmt, White, "init")?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, r)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, sz)?;
                    },
                    Operation::Load( r, e, sz,  b) => {
                        color_bold!(fmt, White, format!("load/{}/{}/{}", r, e, sz))?;
                        write!(fmt, " ")?;
                        pp_variable(fmt, &result)?;
                        color_bold!(fmt, Green, ",")?;
                        write!(fmt, " ")?;
                        color!(fmt, White, b)?;
                    },
//                    Operation::Store( r, e, sz,  a,  b) => {
//                        color_bold!(fmt, White, format!("store/{}/{}/{}", r, e, sz))?;
//                        write!(fmt, " ")?;
//                        pp_variable(fmt, &result)?;
//                        color_bold!(fmt, Green, ",")?;
//                        write!(fmt, " ")?;
//                        color!(fmt, White, a)?;
//                        write!(fmt, " ")?;
//                        color!(fmt, White, b)?;
//                    },
                    phi @ Operation::Phi(..) => {
                        color_bold!(fmt, White, format!("phi"))?;
                        write!(fmt, " ")?;
                        for (i, x) in phi.reads().enumerate() {
                            color!(fmt, White, format!("{}", x))?;
                            if i < 2 {
                                color_bold!(fmt, Green, ",")?;
                                write!(fmt, " ")?;
                            }
                        }
                    }

*/