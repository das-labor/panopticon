/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016  Panopticon authors
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

//! Panopticon uses a language called RREIL to model mnemonic semantics.
//!
//! Conventional disassembler translate machine code from its binary representation to into a list
//! of mnemonics similar to the format assemblers accept. The only knowledge the disassembler has
//! of the opcode is its textual form (for example `mov`) and the number and type (constant vs.
//! register) of operands. These information are purely "syntactic" – they are only about
//! shape. Advanced disassembler like distorm or IDA Pro add limited semantic information to an
//! mnemonic like whenever it's a jump or how executing it effects the stack pointer. This ultimately
//! limits the scope and accuracy of analysis a disassembler can do.
//!
//! Reverse engineering is about understanding code. Most of the time the analyst interprets assembler
//! instructions by "executing" them in his or her head. Good reverse engineers can do
//! this faster and more accurately than others. In order to help human analysts in this labours task
//! the disassembler needs to understand the semantics of each mnemonic.
//!
//! Panopticon uses a simple and well defined programming language (called RREIL) to model the
//! semantics of mnemonics in a machine readable manner. This intermediate languages is emitted
//! by the disassembler part of Panopticon and used by all analysis algorithms. This way the analysis
//! implementation is decoupled from the details of the instruction set.
//!
//! Basic structure
//! ---------------
//!
//! A RREIL program modeling the AVR "adc rd, rr" instruction looks as this:
//!
//! ```rreil
//! zext carry:8, C:1
//! add res:8, rd:8, rr:8
//! add res:8, res:8, carry:8
//!
//! // zero flag
//! cmpeq Z:1, res:8, 0:8
//!
//! mov rd:8, res:8
//! ```
//! Each RREIL program is a sequence of instructions. The first argument of each instructions is
//! assigned its result. The remaining arguments are only read. Arguments can be constants, variables
//! of a special undefined value `?`. Except for the undefined value all arguments are integers with
//! a fixed size.
//!
//! Memory in RREIL programs is modeled as an array of memory cells. The are accessed by the `load`
//! and `store` instructions.
//!
//! Control Flow
//! ------------
//!
//! The RREIL programs produced by the disassemblers are sequences of instructions. No jump or optional
//! instructions are allowed inside a mnemonic. After each mnemonic an unlimited number of jumps is allowed.
//! Each jump is associated with a guard which is a one bit large variable or constant. If the guard is `1`,
//! the jump is taken.
//!
//! The RREIL implemented in Panopticon has a `call` instruction. This instruction has a single argument
//! that specifies the address where a new function begins. No "return" instruction exists. Functions
//! terminate after a sequence with no outgoing jumps is reached.
//!
//! Generating Code
//! ---------------
//!
//! Internally, RREIL code is a `Vec<_>` of `Statement` instances while the arguemnts are either
//! `Lvalue` (writeable) or `Rvalue` (read only). To make generating RREIL easier
//! one can use the `rreil!` macro which translates slightly modified RREIL code into a
//! `Result<Vec<Statement>>` instance.
//!
//! The `rreil!` macro expects constants to be delimited by brackets (`[`/`]`). Rust values can be
//! embedded into RREIL code by enclosing them in parens.
//!
//! The following code generates RREIL code that implements the first part of the AVR `adc R0, R1`
//! instruction.
//!
//! ```
//! #[macro_use] extern crate panopticon_core;
//! # use panopticon_core::{Rvalue,Lvalue,State,Statement,Result};
//! # fn main() {
//! # fn inner() -> Result<()> {
//! let rd = Lvalue::Variable{ name: "R0".into(), size: 8, subscript: None };
//! let rr = Rvalue::Variable{ name: "R1".into(), size: 8, subscript: None, offset: 0 };
//! let stmts = try!(rreil!{
//!     zext/8 carry:8, C:1;
//!     add res:8, (rd), (rr);
//!     add res:8, res:8, carry:8;
//!
//!     // zero flag
//!     cmpeq Z:1, res:8, [0]:8;
//! });
//! # Ok(())
//! # }
//! # inner();
//! # }
//! ```

use Result;
use quickcheck::{Arbitrary, Gen};
use serde::{Serialize,Deserialize};

use std::borrow::Cow;
use std::cmp;
use std::convert::From;
use std::fmt::{Display, Error, Formatter, Debug};
use std::num::Wrapping;
use std::result;
use std::str::{FromStr, SplitWhitespace};
use std::u64;

/// A readable RREIL value.
#[derive(Clone,PartialEq,Eq,Debug,Serialize,Deserialize,Hash,PartialOrd,Ord)]
pub enum Rvalue {
    /// Undefined value of unknown length
    Undefined,
    /// Variable reference
    Variable {
        /// Variable name. Names starting with "__" are reserved.
        name: Cow<'static, str>,
        /// SSA subscript. This can be set to None in most cases.
        subscript: Option<usize>,
        /// First bit of the variable we want to read. Can be set to 0 in most cases.
        offset: usize,
        /// Number of bits we want to read.
        size: usize,
    },
    /// Constant
    Constant {
        /// Value
        value: u64,
        /// Size in bits
        size: usize,
    },
}

impl Rvalue {
    /// Returns a new constant value `v` of size 1
    pub fn new_bit(v: usize) -> Rvalue {
        Rvalue::Constant { value: v as u64, size: 1 }
    }

    /// Returns a new constant value `v` of size 8
    pub fn new_u8(v: u8) -> Rvalue {
        Rvalue::Constant { value: v as u64, size: 8 }
    }

    /// Returns a new constant value `v` of size 16
    pub fn new_u16(v: u16) -> Rvalue {
        Rvalue::Constant { value: v as u64, size: 16 }
    }

    /// Returns a new constant value `v` of size 32
    pub fn new_u32(v: u32) -> Rvalue {
        Rvalue::Constant { value: v as u64, size: 32 }
    }

    /// Returns a new constant value `v` of size 64
    pub fn new_u64(v: u64) -> Rvalue {
        Rvalue::Constant { value: v, size: 64 }
    }

    /// Returns the size of the value in bits or None if its undefined.
    pub fn size(&self) -> Option<usize> {
        match self {
            &Rvalue::Constant { ref size, .. } => Some(*size),
            &Rvalue::Variable { ref size, .. } => Some(*size),
            &Rvalue::Undefined => None,
        }
    }

    /// Returns a new Rvalue with the first `s` starting at `o`.
    pub fn extract(&self, s: usize, o: usize) -> Result<Rvalue> {
        if s <= 0 {
            return Err("can't extract zero bits".into());
        }

        match self {
            &Rvalue::Constant { ref size, ref value } => {
                if *size >= s + o {
                    Ok(Rvalue::Constant { size: s, value: (*value >> o) % (1 << s) })
                } else {
                    Err("Rvalue::extract: invalid argument".into())
                }
            }
            &Rvalue::Variable { ref size, ref offset, ref name, ref subscript } => {
                if *size >= s + o {
                    Ok(
                        Rvalue::Variable {
                            name: name.clone(),
                            subscript: subscript.clone(),
                            size: s,
                            offset: *offset + o,
                        }
                    )
                } else {
                    Err("Rvalue::extract: invalid argument".into())
                }
            }
            &Rvalue::Undefined => Ok(Rvalue::Undefined),
        }
    }
}

impl From<Lvalue> for Rvalue {
    fn from(lv: Lvalue) -> Rvalue {
        match lv {
            Lvalue::Undefined => Rvalue::Undefined,
            Lvalue::Variable { name, subscript, size } => Rvalue::Variable { name: name, subscript: subscript, size: size, offset: 0 },
        }
    }
}

impl FromStr for Rvalue {
    type Err = ();

    fn from_str<'a>(s: &'a str) -> result::Result<Rvalue, ()> {
        if s == "?" {
            Ok(Rvalue::Undefined)
        } else if let Ok(n) = u64::from_str(s) {
            Ok(Rvalue::Constant { value: n, size: 0 })
        } else {
            let mut ws: SplitWhitespace<'a> = s.split_whitespace();
            let maybe_chr = ws.next();
            match maybe_chr {
                Some(s) => {
                    Ok(
                        Rvalue::Variable {
                            name: Cow::Owned(s.to_string()),
                            subscript: None,
                            offset: 0,
                            size: 0,
                        }
                    )
                }
                None => Err(()),
            }
        }
    }
}

impl Display for Rvalue {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self {
            &Rvalue::Undefined => f.write_str("?"),
            &Rvalue::Constant { value: v, size: s } => f.write_fmt(format_args!("0x{:x}:{}", v, s)),
            &Rvalue::Variable { ref name, ref subscript, ref offset, ref size } => {
                f.write_str(name)?;
                if let &Some(ss) = subscript {
                    f.write_fmt(format_args!("_{}", ss))?;
                }
                f.write_fmt(format_args!(":{}", size))?;
                if *offset > 0 {
                    f.write_fmt(format_args!("/{}", offset))?;
                }
                Ok(())
            }
        }
    }
}


/// A writeable RREIL value.
#[derive(Clone,PartialEq,Eq,Debug,Serialize,Deserialize,Hash,PartialOrd,Ord)]
pub enum Lvalue {
    /// Undefined value of unknown length
    Undefined,
    /// Variable reference
    Variable {
        /// Variable name. Names starting with "__" are reserved.
        name: Cow<'static, str>,
        /// SSA subscript. This can be set to None in most cases.
        subscript: Option<usize>,
        /// Size of the variable in bits.
        size: usize,
    },
}

impl Lvalue {
    /// Create a new Lvalue from Rvalue `rv`. Returns None if `rv` is a constant.
    pub fn from_rvalue(rv: Rvalue) -> Option<Lvalue> {
        match rv {
            Rvalue::Undefined => Some(Lvalue::Undefined),
            Rvalue::Variable { name, subscript, size, offset: 0 } => Some(Lvalue::Variable { name: name, subscript: subscript, size: size }),
            _ => None,
        }
    }

    /// Returns a new Rvalue with the first `s` starting at `o`.
    pub fn extract(&self, s: usize, o: usize) -> Result<Rvalue> {
        if s <= 0 {
            return Err("can't extract zero bits".into());
        }

        match self {
            &Lvalue::Variable { ref size, ref name, ref subscript } => {
                if *size >= s + o {
                    Ok(
                        Rvalue::Variable {
                            name: name.clone(),
                            subscript: subscript.clone(),
                            size: s,
                            offset: o,
                        }
                    )
                } else {
                    Err("Rvalue::extract: invalid argument".into())
                }
            }
            &Lvalue::Undefined => Ok(Rvalue::Undefined),
        }
    }

    /// Returns the size of the value in bits or None if its undefined.
    pub fn size(&self) -> Option<usize> {
        match self {
            &Lvalue::Variable { ref size, .. } => Some(*size),
            &Lvalue::Undefined => None,
        }
    }
}

impl Display for Lvalue {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        f.write_fmt(format_args!("{}", Rvalue::from(self.clone())))
    }
}

/// Branch condition
#[derive(Clone,PartialEq,Eq,Debug,Serialize,Deserialize)]
pub enum Guard {
    /// Guard is constant true
    True,
    /// Guard is constant false
    False,
    /// Guard depends on a one bit RREIL value.
    Predicate {
        /// Flag value. Must be `0` or `1`.
        flag: Rvalue,
        /// Expected value of `flag`. If `flag` is `1` and `expected` is true or if
        /// `flag` is `1` and `expected` is true the guard is true. Otherwise its false.
        expected: bool,
    },
}

impl Guard {
    /// Create a guard that is true if `f` is `1`.
    pub fn from_flag(f: &Rvalue) -> Result<Guard> {
        match f {
            &Rvalue::Undefined => Ok(Guard::Predicate { flag: f.clone(), expected: true }),
            &Rvalue::Constant { size: 1, value: 0 } => Ok(Guard::False),
            &Rvalue::Constant { size: 1, value: 1 } => Ok(Guard::True),
            &Rvalue::Variable { size: 1, .. } => Ok(Guard::Predicate { flag: f.clone(), expected: true }),
            _ => Err("Not a flag".into()),
        }
    }

    /// Guard that is always false
    pub fn never() -> Guard {
        Guard::False
    }

    /// Guard that is always true
    pub fn always() -> Guard {
        Guard::True
    }

    /// Negation of self
    pub fn negation(&self) -> Guard {
        match self {
            &Guard::True => Guard::False,
            &Guard::False => Guard::True,
            &Guard::Predicate { ref flag, ref expected } => Guard::Predicate { flag: flag.clone(), expected: !*expected },
        }
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self {
            &Guard::True => f.write_str("true"),
            &Guard::False => f.write_str("false"),
            &Guard::Predicate { flag: Rvalue::Variable { ref name, .. }, expected: true } => f.write_fmt(format_args!("{}", name)),
            &Guard::Predicate { flag: Rvalue::Variable { ref name, .. }, expected: false } => f.write_fmt(format_args!("¬{}", name)),
            &Guard::Predicate { ref flag, ref expected } => f.write_fmt(format_args!("({} == {})", flag, expected)),
        }
    }
}

/// Endianess of a memory operation.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub enum Endianess {
    /// Least significant byte first
    Little,
    /// Most significant byte first
    Big,
}

impl Display for Endianess {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self {
            &Endianess::Little => f.write_str("le"),
            &Endianess::Big => f.write_str("be"),
        }
    }
}

/// A RREIL operation.
#[derive(Clone,PartialEq,Eq,Debug,Serialize,Deserialize)]
#[serde(bound(deserialize = "V: Serialize + for<'a> Deserialize<'a> + Clone + PartialEq + Eq + Debug"))]
pub enum Operation<V>
    where V: Serialize + for<'a> Deserialize<'a> + Clone + PartialEq + Eq + Debug
{
    /// Integer addition
    Add(V, V),
    /// Integer subtraction
    Subtract(V, V),
    /// Unsigned integer multiplication
    Multiply(V, V),
    /// Unsigned integer division
    DivideUnsigned(V, V),
    /// Signed integer division
    DivideSigned(V, V),
    /// Bitwise left shift
    ShiftLeft(V, V),
    /// Bitwise logical right shift
    ShiftRightUnsigned(V, V),
    /// Bitwise arithmetic right shift
    ShiftRightSigned(V, V),
    /// Integer modulo
    Modulo(V, V),
    /// Bitwise logical and
    And(V, V),
    /// Bitwise logical or
    InclusiveOr(V, V),
    /// Bitwise logical xor
    ExclusiveOr(V, V),

    /// Compare both operands for equality and returns `1` or `0`
    Equal(V, V),
    /// Returns `1` if the first operand is less than or equal to the second and `0` otherwise.
    /// Comparison assumes unsigned values.
    LessOrEqualUnsigned(V, V),
    /// Returns `1` if the first operand is less than or equal to the second and `0` otherwise.
    /// Comparison assumes signed values.
    LessOrEqualSigned(V, V),
    /// Returns `1` if the first operand is less than the second and `0` otherwise.
    /// Comparison assumes unsigned values.
    LessUnsigned(V, V),
    /// Returns `1` if the first operand is less than the second and `0` otherwise.
    /// Comparison assumes signed values.
    LessSigned(V, V),

    /// Zero extends the operand.
    ZeroExtend(usize, V),
    /// Sign extends the operand.
    SignExtend(usize, V),
    /// Copies the operand without modification.
    Move(V),
    /// Calls the function located at the address pointed to by the operand.
    Call(V),
    /// Initializes a global variable.
    Initialize(Cow<'static,str>,usize),
    /// Copies only a range of bit from the operand.
    Select(usize, V, V),

    /// Reads a memory cell
    Load(Cow<'static,str>,Endianess,usize,V),
    /// Writes a memory cell pointed by 1st V w/ 2nd V, returns Undef
    Store(Cow<'static,str>,Endianess,usize,V,V),

    /// SSA Phi function
    Phi(Vec<V>),
}

/// A single RREIL statement.
#[derive(Clone,PartialEq,Eq,Debug,Serialize,Deserialize)]
pub struct Statement {
    /// Value that the operation result is assigned to
    pub assignee: Lvalue,
    /// Operation and its arguments
    pub op: Operation<Rvalue>,
}

impl Statement {
    /// Does a simple sanity check. The functions returns Err if
    /// - The argument size are not equal
    /// - The result has not the same size as `assignee`
    /// - The select operation arguments are out of range
    pub fn sanity_check(&self) -> Result<()> {
        // check that argument sizes match
        let typecheck_binop = |a: &Rvalue, b: &Rvalue, assignee: &Lvalue| -> Result<()> {
            if !(a.size() == None || b.size() == None || a.size() == b.size()) {
                return Err(format!("Argument sizes mismatch: {} vs. {}", a, b).into());
            }

            if !(assignee.size() == None || Some(cmp::max(a.size().unwrap_or(0), b.size().unwrap_or(0))) == assignee.size()) {
                return Err(format!("Operation result and assingnee sizes mismatch ({:?})",self).into());
            }

            Ok(())
        };
        let typecheck_cmpop = |a: &Rvalue, b: &Rvalue, assignee: &Lvalue| -> Result<()> {
            if !(a.size() == None || b.size() == None || a.size() == b.size()) {
                return Err("Argument sizes mismatch".into());
            }

            if !(assignee.size() == None || assignee.size() == Some(1)) {
                return Err("Compare operation assingnee not a flag".into());
            }

            Ok(())
        };
        let typecheck_unop = |a: &Rvalue, sz: Option<usize>, assignee: &Lvalue| -> Result<()> {
            if sz.is_none() {
                // zext?
                if !(a.size() == None || assignee.size() == None || assignee.size() <= a.size()) {
                    return Err("Operation result and assingnee sizes mismatch".into());
                }
            } else {
                if !(a.size() == None || assignee.size() == None || assignee.size() == sz) {
                    return Err("Operation result and assingnee sizes mismatch".into());
                }
            }
            Ok(())
        };

        match self {
            &Statement { op: Operation::Add(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::Subtract(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::Multiply(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::DivideUnsigned(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::DivideSigned(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::ShiftLeft(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement {
                op: Operation::ShiftRightUnsigned(ref a, ref b),
                ref assignee,
            } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::ShiftRightSigned(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::Modulo(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::And(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::ExclusiveOr(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),
            &Statement { op: Operation::InclusiveOr(ref a, ref b), ref assignee } => typecheck_binop(a, b, assignee),

            &Statement { op: Operation::Equal(ref a, ref b), ref assignee } => typecheck_cmpop(a, b, assignee),
            &Statement {
                op: Operation::LessOrEqualUnsigned(ref a, ref b),
                ref assignee,
            } => typecheck_cmpop(a, b, assignee),
            &Statement { op: Operation::LessOrEqualSigned(ref a, ref b), ref assignee } => typecheck_cmpop(a, b, assignee),
            &Statement { op: Operation::LessUnsigned(ref a, ref b), ref assignee } => typecheck_cmpop(a, b, assignee),
            &Statement { op: Operation::LessSigned(ref a, ref b), ref assignee } => typecheck_cmpop(a, b, assignee),

            &Statement { op: Operation::SignExtend(ref a, ref b), ref assignee } => typecheck_unop(b, Some(*a), assignee),
            &Statement { op: Operation::ZeroExtend(ref a, ref b), ref assignee } => typecheck_unop(b, Some(*a), assignee),
            &Statement { op: Operation::Move(ref a), ref assignee } => typecheck_unop(a, None, assignee),
            &Statement { op: Operation::Select(ref off, ref a, ref b), ref assignee } => {
                if !(assignee.size() == a.size() && *off + b.size().unwrap_or(0) <= a.size().unwrap_or(0)) {
                    return Err("Ill-sized Select operation".into());
                } else {
                    Ok(())
                }
            }

            &Statement{ op: Operation::Initialize(_,ref sz), ref assignee } => {
                if !(assignee.size() == None || assignee.size() == Some(*sz)) {
                    return Err("Operation result and assingnee sizes mismatch".into())
                } else {
                    Ok(())
                }
            }

            &Statement { op: Operation::Call(_), ref assignee } => {
                if !(assignee == &Lvalue::Undefined) {
                    return Err("Call operation can only be assigned to Undefined".into());
                } else {
                    Ok(())
                }
            }

            &Statement{ op: Operation::Load(_,_,ref sz,_), ref assignee } => {
                if !assignee.size().is_none() && assignee.size() != Some(*sz) {
                    return Err(format!("Memory operation with invalid size. Expected {:?} got {:?}",Some(*sz),assignee.size()).into());
                } else if *sz == 0 {
                    return Err("Memory operation of size 0".into());
                } else if *sz % 8 != 0 {
                    return Err("Memory operation not byte aligned".into());
                } else {
                    Ok(())
                }
            }

            &Statement{ op: Operation::Store(_,_,sz,_,ref val), ref assignee } => {
                if val.size().is_some() && assignee.size().is_some() && val.size() != assignee.size() {
                    return Err("Memory store value with inconsitend size".into());
                } else if sz == 0 {
                    return Err("Memory operation of size 0".into());
                } else if val.size().is_some() && val.size() != Some(sz) {
                    return Err(format!("Memory store value with inconsitend size: {:?} != {}",val.size(),sz).into());
                } else if sz % 8 != 0 {
                    return Err("Memory operation not byte aligned".into());
                } else {
                    Ok(())
                }
            }


            &Statement { op: Operation::Phi(ref vec), ref assignee } => {
                if !(vec.iter().all(|rv| rv.size() == assignee.size()) && assignee.size() != None) {
                    return Err("Phi arguments must have equal sizes and can't be Undefined".into());
                } else {
                    Ok(())
                }
            }
        }?;

        if !(self.op.operands().iter().all(|rv| rv.size() != Some(0)) && self.assignee.size() != Some(0)) {
            return Err("Operation argument and/or assignee has size 0".into());
        }

        Ok(())
    }
}

/// Executes a RREIL operation returning the result.
pub fn execute(op: Operation<Rvalue>) -> Rvalue {
    match op {
        Operation::Add(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant { value: ((a + b) & mask).0, size: s }
        }
        Operation::Add(Rvalue::Constant { value: 0, .. }, ref b) => b.clone(),
        Operation::Add(ref a, Rvalue::Constant { value: 0, .. }) => a.clone(),
        Operation::Add(_, _) => Rvalue::Undefined,
        Operation::Subtract(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant { value: ((a - b) & mask).0, size: s }
        }
        Operation::Subtract(ref a, Rvalue::Constant { value: 0, .. }) => a.clone(),
        Operation::Subtract(_, _) => Rvalue::Undefined,

        Operation::Multiply(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant { value: ((a * b) & mask).0, size: s }
        }
        Operation::Multiply(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::Multiply(_, Rvalue::Constant { value: 0, size: s }) => Rvalue::Constant { value: 0, size: s },
        Operation::Multiply(Rvalue::Constant { value: 1, .. }, ref b) => b.clone(),
        Operation::Multiply(ref a, Rvalue::Constant { value: 1, .. }) => a.clone(),
        Operation::Multiply(_, _) => Rvalue::Undefined,

        Operation::DivideUnsigned(_, Rvalue::Constant { value: 0, .. }) => Rvalue::Undefined,
        Operation::DivideUnsigned(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });

            if (b & mask) == Wrapping(0) {
                Rvalue::Undefined
            } else {
                Rvalue::Constant { value: ((a / b) & mask).0, size: s }
            }
        }
        Operation::DivideUnsigned(ref a, Rvalue::Constant { value: 1, .. }) => a.clone(),
        Operation::DivideUnsigned(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::DivideUnsigned(_, _) => Rvalue::Undefined,

        Operation::DivideSigned(_, Rvalue::Constant { value: 0, .. }) => Rvalue::Undefined,
        Operation::DivideSigned(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let mut a = Wrapping(_a as i64);
            let mut b = Wrapping(_b as i64);

            if s < 64 {
                let sign_bit = Wrapping(1 << (s - 1));
                let m = Wrapping(1 << s);

                if sign_bit & a != Wrapping(0) {
                    a = a - m;
                }
                if sign_bit & b != Wrapping(0) {
                    b = b - m;
                }
                a = a % m;
                b = b % m;
            }

            if b == Wrapping(0) {
                Rvalue::Undefined
            } else {
                if s < 64 {
                    let m = 1 << s;
                    Rvalue::Constant { value: (a / b).0 as u64 % m, size: s }
                } else {
                    Rvalue::Constant { value: (a / b).0 as u64, size: s }
                }
            }
        }
        Operation::DivideSigned(ref a, Rvalue::Constant { value: 1, .. }) => a.clone(),
        Operation::DivideSigned(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::DivideSigned(_, _) => Rvalue::Undefined,

        Operation::Modulo(_, Rvalue::Constant { value: 0, .. }) => Rvalue::Undefined,
        Operation::Modulo(Rvalue::Constant { value: a, size: s }, Rvalue::Constant { value: b, size: _s }) => {
            debug_assert!(s == _s);

            let mask = if s < 64 { (1u64 << s) - 1 } else { u64::MAX };
            Rvalue::Constant { value: (a % b) & mask, size: s }
        }
        Operation::Modulo(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::Modulo(_, Rvalue::Constant { value: 1, size: s }) => Rvalue::Constant { value: 0, size: s },
        Operation::Modulo(_, _) => Rvalue::Undefined,

        Operation::ShiftLeft(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant { value: ((a << (b as usize)) & mask).0, size: s }
        }
        Operation::ShiftLeft(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::ShiftLeft(ref a, Rvalue::Constant { value: 0, .. }) => a.clone(),
        Operation::ShiftLeft(_, _) => Rvalue::Undefined,

        Operation::ShiftRightUnsigned(Rvalue::Constant { value: a, size: s }, Rvalue::Constant { value: b, size: _s }) => {
            use std::cmp;
            debug_assert!(s == _s);

            if b >= s as u64 {
                Rvalue::Constant { value: 0, size: s }
            } else {
                let mask = if s < 64 { (1u64 << s) - 1 } else { u64::MAX };
                Rvalue::Constant {
                    value: ((a >> cmp::min(cmp::min(64, s), b as usize)) & mask),
                    size: s,
                }
            }
        }
        Operation::ShiftRightUnsigned(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::ShiftRightUnsigned(ref a, Rvalue::Constant { value: 0, .. }) => a.clone(),
        Operation::ShiftRightUnsigned(_, _) => Rvalue::Undefined,

        Operation::ShiftRightSigned(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: b, size: _s }) => {
            debug_assert!(s == _s);

            let mut a = Wrapping(_a as i64);

            if s < 64 {
                let sign_bit = Wrapping(1 << (s - 1));
                let m = Wrapping(1 << s);

                if sign_bit & a != Wrapping(0) {
                    a = a - m;
                }
                a = a % m;
            }

            if b >= s as u64 {
                return if a < Wrapping(0) {
                           if s < 64 {
                               Rvalue::Constant { value: (1 << s) - 1, size: s }
                           } else {
                               Rvalue::Constant { value: u64::MAX, size: s }
                           }
                       } else {
                           Rvalue::Constant { value: 0, size: s }
                       };
            }

            if s < 64 {
                let m = (1 << s) - 1;
                Rvalue::Constant { value: (a >> (b as usize)).0 as u64 & m, size: s }
            } else {
                Rvalue::Constant { value: (a >> (b as usize)).0 as u64, size: s }
            }
        }
        Operation::ShiftRightSigned(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::ShiftRightSigned(ref a, Rvalue::Constant { value: 0, .. }) => a.clone(),
        Operation::ShiftRightSigned(_, _) => Rvalue::Undefined,

        Operation::And(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = if s < 64 {
                Wrapping(_a & ((1 << s) - 1))
            } else {
                Wrapping(_a)
            };
            let b = if s < 64 {
                Wrapping(_b & ((1 << s) - 1))
            } else {
                Wrapping(_b)
            };
            Rvalue::Constant { value: (a & b).0, size: s }
        }
        Operation::And(_, Rvalue::Constant { value: 0, size: s }) => Rvalue::Constant { value: 0, size: s },
        Operation::And(Rvalue::Constant { value: 0, size: s }, _) => Rvalue::Constant { value: 0, size: s },
        Operation::And(_, _) => Rvalue::Undefined,

        Operation::InclusiveOr(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = if s < 64 {
                Wrapping(_a & ((1 << s) - 1))
            } else {
                Wrapping(_a)
            };
            let b = if s < 64 {
                Wrapping(_b & ((1 << s) - 1))
            } else {
                Wrapping(_b)
            };
            Rvalue::Constant { value: (a | b).0, size: s }
        }
        Operation::InclusiveOr(ref a, Rvalue::Constant { value: 0, .. }) => a.clone(),
        Operation::InclusiveOr(Rvalue::Constant { value: 0, .. }, ref b) => b.clone(),
        Operation::InclusiveOr(_, _) => Rvalue::Undefined,

        Operation::ExclusiveOr(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = if s < 64 {
                Wrapping(_a & ((1 << s) - 1))
            } else {
                Wrapping(_a)
            };
            let b = if s < 64 {
                Wrapping(_b & ((1 << s) - 1))
            } else {
                Wrapping(_b)
            };
            Rvalue::Constant { value: (a ^ b).0, size: s }
        }
        Operation::ExclusiveOr(_, _) => Rvalue::Undefined,

        Operation::Equal(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = if s < 64 {
                Wrapping(_a & ((1 << s) - 1))
            } else {
                Wrapping(_a)
            };
            let b = if s < 64 {
                Wrapping(_b & ((1 << s) - 1))
            } else {
                Wrapping(_b)
            };
            if a == b {
                Rvalue::Constant { value: 1, size: 1 }
            } else {
                Rvalue::Constant { value: 0, size: 1 }
            }
        }
        Operation::Equal(_, _) => Rvalue::Undefined,

        Operation::LessOrEqualUnsigned(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = if s < 64 {
                Wrapping(_a & ((1 << s) - 1))
            } else {
                Wrapping(_a)
            };
            let b = if s < 64 {
                Wrapping(_b & ((1 << s) - 1))
            } else {
                Wrapping(_b)
            };
            if a <= b {
                Rvalue::Constant { value: 1, size: 1 }
            } else {
                Rvalue::Constant { value: 0, size: 1 }
            }
        }
        Operation::LessOrEqualUnsigned(Rvalue::Constant { value: 0, .. }, _) => Rvalue::Constant { value: 1, size: 1 },
        Operation::LessOrEqualUnsigned(_, _) => Rvalue::Undefined,

        Operation::LessOrEqualSigned(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(
                if s < 64 {
                    (1u64 << (s - 1)) - 1
                } else {
                    u64::MAX
                }
            );
            let sign_mask = Wrapping(if s < 64 { 1u64 << (s - 1) } else { 0 });
            if (a & sign_mask) ^ (b & sign_mask) != Wrapping(0) {
                Rvalue::Constant {
                    value: if a & sign_mask != Wrapping(0) { 1 } else { 0 },
                    size: 1,
                }
            } else {
                Rvalue::Constant { value: if (a & mask) <= (b & mask) { 1 } else { 0 }, size: 1 }
            }
        }
        Operation::LessOrEqualSigned(_, _) => Rvalue::Undefined,

        Operation::LessUnsigned(Rvalue::Constant { value: a_, size: sa }, Rvalue::Constant { value: b_, size: sb }) => {
            debug_assert!(sb == sa);

            let a = if sa < 64 {
                Wrapping(a_ & ((1 << sa) - 1))
            } else {
                Wrapping(a_)
            };
            let b = if sb < 64 {
                Wrapping(b_ & ((1 << sa) - 1))
            } else {
                Wrapping(b_)
            };

            if a < b {
                Rvalue::Constant { value: 1, size: 1 }
            } else {
                Rvalue::Constant { value: 0, size: 1 }
            }
        }
        Operation::LessUnsigned(_, _) => Rvalue::Undefined,

        Operation::LessSigned(Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let mut a = Wrapping(_a as i64);
            let mut b = Wrapping(_b as i64);

            if s < 64 {
                let sign_bit = Wrapping(1 << (s - 1));
                let m = Wrapping(1 << s);

                if sign_bit & a != Wrapping(0) {
                    a = a - m;
                }
                if sign_bit & b != Wrapping(0) {
                    b = b - m;
                }
                a = a % m;
                b = b % m;
            }

            if a < b {
                Rvalue::Constant { value: 1, size: 1 }
            } else {
                Rvalue::Constant { value: 0, size: 1 }
            }
        }
        Operation::LessSigned(_, _) => Rvalue::Undefined,

        Operation::ZeroExtend(s1, Rvalue::Constant { value: v, size: s0 }) => {
            let mask1 = if s1 < 64 { (1u64 << s1) - 1 } else { u64::MAX };
            let mask0 = if s0 < 64 { (1u64 << s0) - 1 } else { u64::MAX };
            Rvalue::Constant { value: (v & mask0) & mask1, size: s1 }
        }
        Operation::ZeroExtend(s, Rvalue::Variable { ref name, ref subscript, .. }) => {
            Rvalue::Variable {
                name: name.clone(),
                subscript: subscript.clone(),
                offset: 0,
                size: s,
            }
        }
        Operation::ZeroExtend(_, Rvalue::Undefined) => Rvalue::Undefined,

        Operation::SignExtend(t, Rvalue::Constant { value: v, size: s, .. }) => {
            let mask0 = if s < 64 { (1u64 << s) - 1 } else { u64::MAX };
            let mask1 = if t < 64 { (1u64 << t) - 1 } else { u64::MAX };
            let sign = if s < 64 { 1u64 << (s - 1) } else { 0 };

            if v & sign == 0 {
                Rvalue::Constant { value: (v & mask0) & mask1, size: t }
            } else {
                let mask = mask1 & !mask0;
                Rvalue::Constant { value: (v & mask0) | mask, size: t }
            }
        }
        Operation::SignExtend(s, Rvalue::Variable { ref name, ref subscript, .. }) => {
            Rvalue::Variable {
                name: name.clone(),
                subscript: subscript.clone(),
                offset: 0,
                size: s,
            }
        }
        Operation::SignExtend(_, Rvalue::Undefined) => Rvalue::Undefined,

        Operation::Move(Rvalue::Constant { ref value, ref size }) => {
            if *size < 64 {
                Rvalue::Constant { value: *value & ((1u64 << size) - 1), size: *size }
            } else {
                Rvalue::Constant { value: *value, size: *size }
            }
        }
        Operation::Move(ref a) => a.clone(),

        Operation::Initialize(_, _) => Rvalue::Undefined,

        Operation::Call(_) => Rvalue::Undefined,

        Operation::Select(off, Rvalue::Constant { value: _a, size: s }, Rvalue::Constant { value: _b, size: _s }) => {
            debug_assert!(off + _s <= s);

            if off + _s < 64 {
                let hi = _a >> (off + _s);
                let lo = _a % (1 << off);
                Rvalue::Constant { value: lo | (_b << off) | (hi << (off + _s)), size: s }
            } else {
                Rvalue::Undefined
            }
        }
        Operation::Select(_, _, _) => Rvalue::Undefined,

        Operation::Load(_, _, _, _) => Rvalue::Undefined,

        Operation::Store(_, _, _, _, _) => Rvalue::Undefined,

        Operation::Phi(ref vec) => {
            match vec.len() {
                0 => Rvalue::Undefined,
                1 => vec[0].clone(),
                _ => {
                    if vec.iter().all(|x| vec.first().unwrap() == x) {
                        vec[0].clone()
                    } else {
                        Rvalue::Undefined
                    }
                }
            }
        }
    }
}

/// Maps the function `m` over all operands of `op`.
pub fn lift<A, B, F>(op: &Operation<B>, m: &F) -> Operation<A>
    where A: Serialize + for<'a> Deserialize<'a> + Clone + PartialEq + Eq + Debug,
          B: Serialize + for<'a> Deserialize<'a> + Clone + PartialEq + Eq + Debug,
          F: Fn(&B) -> A
{
    let args = op.operands().iter().cloned().map(m).collect::<Vec<_>>();
    match op {
        &Operation::Phi(_) => Operation::Phi(args),
        &Operation::Load(ref s, e, sz, _) => Operation::Load(s.clone(), e, sz, args[0].clone()),
        &Operation::Store(ref s, e, sz, _, _) => Operation::Store(s.clone(), e, sz, args[0].clone(),args[1].clone()),
        &Operation::Add(_, _) => Operation::Add(args[0].clone(), args[1].clone()),
        &Operation::Subtract(_, _) => Operation::Subtract(args[0].clone(), args[1].clone()),
        &Operation::Multiply(_, _) => Operation::Multiply(args[0].clone(), args[1].clone()),
        &Operation::DivideUnsigned(_, _) => Operation::DivideUnsigned(args[0].clone(), args[1].clone()),
        &Operation::DivideSigned(_, _) => Operation::DivideSigned(args[0].clone(), args[1].clone()),
        &Operation::ShiftLeft(_, _) => Operation::ShiftLeft(args[0].clone(), args[1].clone()),
        &Operation::ShiftRightUnsigned(_, _) => Operation::ShiftRightUnsigned(args[0].clone(), args[1].clone()),
        &Operation::ShiftRightSigned(_, _) => Operation::ShiftRightSigned(args[0].clone(), args[1].clone()),
        &Operation::Modulo(_, _) => Operation::Modulo(args[0].clone(), args[1].clone()),
        &Operation::And(_, _) => Operation::And(args[0].clone(), args[1].clone()),
        &Operation::InclusiveOr(_, _) => Operation::InclusiveOr(args[0].clone(), args[1].clone()),
        &Operation::ExclusiveOr(_, _) => Operation::ExclusiveOr(args[0].clone(), args[1].clone()),
        &Operation::Equal(_, _) => Operation::Equal(args[0].clone(), args[1].clone()),
        &Operation::LessUnsigned(_, _) => Operation::LessUnsigned(args[0].clone(), args[1].clone()),
        &Operation::LessSigned(_, _) => Operation::LessSigned(args[0].clone(), args[1].clone()),
        &Operation::LessOrEqualUnsigned(_, _) => Operation::LessOrEqualUnsigned(args[0].clone(), args[1].clone()),
        &Operation::LessOrEqualSigned(_, _) => Operation::LessOrEqualSigned(args[0].clone(), args[1].clone()),
        &Operation::Initialize(ref a, b) => Operation::Initialize(a.clone(),b),
        &Operation::Move(_) => Operation::Move(args[0].clone()),
        &Operation::Call(_) => Operation::Call(args[0].clone()),
        &Operation::Select(ref off, _, _) => Operation::Select(*off, args[0].clone(), args[1].clone()),
        &Operation::ZeroExtend(ref sz, _) => Operation::ZeroExtend(*sz, args[0].clone()),
        &Operation::SignExtend(ref sz, _) => Operation::SignExtend(*sz, args[0].clone()),
    }
}



impl<V> Operation<V>
    where V: Serialize + for<'a> Deserialize<'a> + Clone + PartialEq + Eq + Debug
{
    /// Returns its operands
    pub fn operands(&self) -> Vec<&V> {
        match *self {
            Operation::Add(ref a, ref b) => return vec![a, b],
            Operation::Subtract(ref a, ref b) => return vec![a, b],
            Operation::Multiply(ref a, ref b) => return vec![a, b],
            Operation::DivideUnsigned(ref a, ref b) => return vec![a, b],
            Operation::DivideSigned(ref a, ref b) => return vec![a, b],
            Operation::ShiftLeft(ref a, ref b) => return vec![a, b],
            Operation::ShiftRightUnsigned(ref a, ref b) => return vec![a, b],
            Operation::ShiftRightSigned(ref a, ref b) => return vec![a, b],
            Operation::Modulo(ref a, ref b) => return vec![a, b],
            Operation::And(ref a, ref b) => return vec![a, b],
            Operation::InclusiveOr(ref a, ref b) => return vec![a, b],
            Operation::ExclusiveOr(ref a, ref b) => return vec![a, b],

            Operation::Equal(ref a, ref b) => return vec![a, b],
            Operation::LessOrEqualUnsigned(ref a, ref b) => return vec![a, b],
            Operation::LessOrEqualSigned(ref a, ref b) => return vec![a, b],
            Operation::LessUnsigned(ref a, ref b) => return vec![a, b],
            Operation::LessSigned(ref a, ref b) => return vec![a, b],

            Operation::ZeroExtend(_, ref a) => return vec![a],
            Operation::SignExtend(_, ref a) => return vec![a],
            Operation::Move(ref a) => return vec![a],
            Operation::Call(ref a) => return vec![a],
            Operation::Initialize(_, _) => return vec![],
            Operation::Select(_, ref a, ref b) => return vec![a, b],

            Operation::Load(_, _, _, ref b) => return vec![b],
            Operation::Store(_, _, _, ref a, ref b) => return vec![a,b],

            Operation::Phi(ref vec) => return vec.iter().collect(),
        }
    }

    /// Returns its operands
    pub fn operands_mut(&mut self) -> Vec<&mut V> {
        match self {
            &mut Operation::Add(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::Subtract(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::Multiply(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::DivideUnsigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::DivideSigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::ShiftLeft(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::ShiftRightUnsigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::ShiftRightSigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::Modulo(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::And(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::InclusiveOr(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::ExclusiveOr(ref mut a, ref mut b) => return vec![a, b],

            &mut Operation::Equal(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::LessOrEqualUnsigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::LessOrEqualSigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::LessUnsigned(ref mut a, ref mut b) => return vec![a, b],
            &mut Operation::LessSigned(ref mut a, ref mut b) => return vec![a, b],

            &mut Operation::ZeroExtend(_, ref mut a) => return vec![a],
            &mut Operation::SignExtend(_, ref mut a) => return vec![a],
            &mut Operation::Move(ref mut a) => return vec![a],
            &mut Operation::Call(ref mut a) => return vec![a],
            &mut Operation::Initialize(_, _) => return vec![],
            &mut Operation::Select(_, ref mut a, ref mut b) => return vec![a, b],

            &mut Operation::Load(_, _, _, ref mut b) => return vec![b],
            &mut Operation::Store(_, _, _, ref mut a, ref mut b) => return vec![a, b],

            &mut Operation::Phi(ref mut vec) => return vec.iter_mut().collect(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self.op {
            Operation::Add(ref a, ref b) => f.write_fmt(format_args!("add {}, {}, {}", self.assignee, a, b)),
            Operation::Subtract(ref a, ref b) => f.write_fmt(format_args!("sub {}, {}, {}", self.assignee, a, b)),
            Operation::Multiply(ref a, ref b) => f.write_fmt(format_args!("mul {}, {}, {}", self.assignee, a, b)),
            Operation::DivideUnsigned(ref a, ref b) => f.write_fmt(format_args!("divu {}, {}, {}", self.assignee, a, b)),
            Operation::DivideSigned(ref a, ref b) => f.write_fmt(format_args!("divs {}, {}, {}", self.assignee, a, b)),
            Operation::ShiftLeft(ref a, ref b) => f.write_fmt(format_args!("shl {}, {}, {}", self.assignee, a, b)),
            Operation::ShiftRightUnsigned(ref a, ref b) => f.write_fmt(format_args!("shru {}, {}, {}", self.assignee, a, b)),
            Operation::ShiftRightSigned(ref a, ref b) => f.write_fmt(format_args!("shrs {}, {}, {}", self.assignee, a, b)),
            Operation::Modulo(ref a, ref b) => f.write_fmt(format_args!("mod {}, {}, {}", self.assignee, a, b)),
            Operation::And(ref a, ref b) => f.write_fmt(format_args!("and {}, {}, {}", self.assignee, a, b)),
            Operation::InclusiveOr(ref a, ref b) => f.write_fmt(format_args!("or {}, {}, {}", self.assignee, a, b)),
            Operation::ExclusiveOr(ref a, ref b) => f.write_fmt(format_args!("xor {}, {}, {}", self.assignee, a, b)),

            Operation::Equal(ref a, ref b) => f.write_fmt(format_args!("cmpeq {}, {}, {}", self.assignee, a, b)),
            Operation::LessOrEqualUnsigned(ref a, ref b) => f.write_fmt(format_args!("cmpleu {}, {}, {}", self.assignee, a, b)),
            Operation::LessOrEqualSigned(ref a, ref b) => f.write_fmt(format_args!("cmples {}, {}, {}", self.assignee, a, b)),
            Operation::LessUnsigned(ref a, ref b) => f.write_fmt(format_args!("cmplu {}, {}, {}", self.assignee, a, b)),
            Operation::LessSigned(ref a, ref b) => f.write_fmt(format_args!("cmpls {}, {}, {}", self.assignee, a, b)),

            Operation::ZeroExtend(s, ref a) => f.write_fmt(format_args!("convert_{} {}, {}", s, self.assignee, a)),
            Operation::SignExtend(s, ref a) => f.write_fmt(format_args!("sign-extend_{} {}, {}", s, self.assignee, a)),
            Operation::Select(s, ref a, ref b) => f.write_fmt(format_args!("select_{} {}, {}, {}", s, self.assignee, a, b)),
            Operation::Move(ref a) => f.write_fmt(format_args!("mov {}, {}", self.assignee, a)),
            Operation::Call(ref a) => f.write_fmt(format_args!("call {}, {}", self.assignee, a)),

            Operation::Initialize(ref name,ref size) => f.write_fmt(format_args!("init {}, {}:{}",self.assignee,name,size)),

            Operation::Load(ref r,Endianess::Little,ref sz,ref b) => f.write_fmt(format_args!("load_{}/le/{} {}, {}",r,sz,self.assignee,b)),
            Operation::Load(ref r,Endianess::Big,ref sz,ref b) => f.write_fmt(format_args!("load_{}/be/{} {}, {}",r,sz,self.assignee,b)),
            Operation::Store(ref r,Endianess::Little,ref sz,ref a, ref b) => f.write_fmt(format_args!("store_{}/le/{} {}, {}, {}",r,sz,self.assignee,a,b)),
            Operation::Store(ref r,Endianess::Big,ref sz,ref a, ref b) => f.write_fmt(format_args!("store_{}/be/{} {}, {}, {}",r,sz,self.assignee,a,b)),

            Operation::Phi(ref vec) => {
                f.write_fmt(format_args!("phi {}", self.assignee))?;
                for (i, x) in vec.iter().enumerate() {
                    f.write_fmt(format_args!("{}", x))?;
                    if i < vec.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Arbitrary for Rvalue {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        match g.gen_range(0, 3) {
            0 => Rvalue::Undefined,
            1 => {
                Rvalue::Variable {
                    name: Cow::Owned(g.gen_ascii_chars().take(2).collect()),
                    size: g.gen_range(1, 513),
                    subscript: Some(g.gen_range(0, 5)),
                    offset: g.gen_range(0, 512),
                }
            }
            2 => Rvalue::Constant { value: g.gen(), size: g.gen_range(1, 513) },
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for Lvalue {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        match g.gen_range(0, 2) {
            0 => Lvalue::Undefined,
            1 => {
                Lvalue::Variable {
                    name: Cow::Owned(g.gen_ascii_chars().take(2).collect()),
                    size: g.gen_range(1, 513),
                    subscript: Some(g.gen_range(0, 5)),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for Operation<Rvalue> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut op = match g.gen_range(0, 25) {
            0 => Operation::Add(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            1 => Operation::Subtract(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            2 => Operation::Multiply(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            3 => Operation::DivideUnsigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            4 => Operation::DivideSigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            5 => Operation::ShiftLeft(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            6 => Operation::ShiftRightUnsigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            7 => Operation::ShiftRightSigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            8 => Operation::Modulo(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            9 => Operation::InclusiveOr(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            10 => Operation::ExclusiveOr(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),

            11 => Operation::Equal(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            12 => Operation::LessOrEqualUnsigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            13 => Operation::LessOrEqualSigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            14 => Operation::LessUnsigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),
            15 => Operation::LessSigned(Rvalue::arbitrary(g), Rvalue::arbitrary(g)),

            16 => Operation::ZeroExtend(g.gen(), Rvalue::arbitrary(g)),
            17 => Operation::SignExtend(g.gen(), Rvalue::arbitrary(g)),

            18 => Operation::Move(Rvalue::arbitrary(g)),
            19 => Operation::Initialize(g.gen_ascii_chars().take(1).collect(),g.gen()),

            20 => Operation::Select(g.gen(), Rvalue::arbitrary(g), Rvalue::arbitrary(g)),

            21 => Operation::Load(g.gen_ascii_chars().take(1).collect(), Endianess::arbitrary(g), g.gen(), Rvalue::arbitrary(g)),
            22 => Operation::Store(g.gen_ascii_chars().take(1).collect(), Endianess::arbitrary(g), g.gen(), Rvalue::arbitrary(g), Rvalue::arbitrary(g)),

            23 => {
                let cnt = g.gen_range(1, 6);
                // XXX: make sizes equal?
                let i = (0..cnt).into_iter().map(|_| Rvalue::arbitrary(g));
                Operation::Phi(i.collect())
            }
            24 => Operation::Call(Rvalue::arbitrary(g)),

            _ => unreachable!(),
        };

        match op {
            Operation::Add(_, _) |
            Operation::Subtract(_, _) |
            Operation::Multiply(_, _) |
            Operation::DivideUnsigned(_, _) |
            Operation::DivideSigned(_, _) |
            Operation::Modulo(_, _) |
            Operation::ShiftLeft(_, _) |
            Operation::ShiftRightUnsigned(_, _) |
            Operation::ShiftRightSigned(_, _) |
            Operation::And(_, _) |
            Operation::InclusiveOr(_, _) |
            Operation::ExclusiveOr(_, _) |
            Operation::Equal(_, _) |
            Operation::LessOrEqualUnsigned(_, _) |
            Operation::LessOrEqualSigned(_, _) |
            Operation::LessUnsigned(_, _) |
            Operation::LessSigned(_, _) => {
                let mut sz = None;
                for o in op.operands_mut() {
                    if sz.is_none() {
                        sz = o.size();
                    } else {
                        match o {
                            &mut Rvalue::Undefined => {}
                            &mut Rvalue::Constant { ref mut size, .. } => *size = sz.unwrap(),
                            &mut Rvalue::Variable { ref mut size, .. } => *size = sz.unwrap(),
                        }
                    }
                }
            }
            Operation::Select(ref mut off, ref mut rv1, ref mut rv2) => {
                if let (Some(sz1), Some(sz2)) = (rv1.size(), rv2.size()) {
                    if sz2 > sz1 {
                        let t2 = rv1.clone();
                        *rv1 = rv2.clone();
                        *rv2 = t2;
                    }
                }
                if let (Some(sz1), Some(sz2)) = (rv1.size(), rv2.size()) {
                    *off = g.gen_range(0, sz1 - sz2 + 1);
                }
            }
            _ => {}
        }

        op
    }
}

impl Arbitrary for Endianess {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        match g.gen_range(0, 1) {
            0 => Endianess::Little,
            1 => Endianess::Big,
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! rreil {
    ( ) => {Ok(vec![])};
    ( add $($cdr:tt)* ) => { rreil_binop!(Add # $($cdr)*) };
    ( sub $($cdr:tt)* ) => { rreil_binop!(Subtract # $($cdr)*) };
    ( mul $($cdr:tt)* ) => { rreil_binop!(Multiply # $($cdr)*) };
    ( div $($cdr:tt)* ) => { rreil_binop!(DivideUnsigned # $($cdr)*) };
    ( divs $($cdr:tt)* ) => { rreil_binop!(DivideSigned # $($cdr)*) };
    ( shl $($cdr:tt)* ) => { rreil_binop!(ShiftLeft # $($cdr)*) };
    ( shr $($cdr:tt)* ) => { rreil_binop!(ShiftRightUnsigned # $($cdr)*) };
    ( shrs $($cdr:tt)* ) => { rreil_binop!(ShiftRightSigned # $($cdr)*) };
    ( mod $($cdr:tt)* ) => { rreil_binop!(Modulo # $($cdr)*) };
    ( and $($cdr:tt)* ) => { rreil_binop!(And # $($cdr)*) };
    ( xor $($cdr:tt)* ) => { rreil_binop!(ExclusiveOr # $($cdr)*) };
    ( or $($cdr:tt)* ) => { rreil_binop!(InclusiveOr # $($cdr)*) };

    ( cmpeq $($cdr:tt)* ) => { rreil_binop!(Equal # $($cdr)*) };
    ( cmpleu $($cdr:tt)* ) => { rreil_binop!(LessOrEqualUnsigned # $($cdr)*) };
    ( cmples $($cdr:tt)* ) => { rreil_binop!(LessOrEqualSigned # $($cdr)*) };
    ( cmpltu $($cdr:tt)* ) => { rreil_binop!(LessUnsigned # $($cdr)*) };
    ( cmplts $($cdr:tt)* ) => { rreil_binop!(LessSigned # $($cdr)*) };

    ( sel / $off:tt $($cdr:tt)* ) => { rreil_selop!(Select # $off # $($cdr)*) };
    ( sext / $sz:tt $($cdr:tt)* ) => { rreil_extop!(SignExtend # $sz # $($cdr)*) };
    ( zext / $sz:tt $($cdr:tt)* ) => { rreil_extop!(ZeroExtend # $sz # $($cdr)*) };
    ( mov $($cdr:tt)* ) => { rreil_unop!(Move # $($cdr)*) };
    ( call $($cdr:tt)* ) => { rreil_callop!($($cdr)*) };
    ( ret $($cdr:tt)* ) => { rreil_retop!($($cdr)*) };

    ( load / $r:ident / $en:ident / $sz:tt $($cdr:tt)* ) => { rreil_memop!(Load # $r # $en # $sz # $($cdr)*) };
    ( store / $r:ident / $en:ident / $sz:tt $($cdr:tt)* ) => { rreil_memop!(Store # $r # $en # $sz # $($cdr)*) };
}

include!(concat!(env!("OUT_DIR"), "/rreil.rs"));

#[macro_export]
macro_rules! rreil_lvalue {
    (?) =>
        { $crate::Lvalue::Undefined };
    ( ( $a:expr ) ) =>
        { ($a).clone().into() };
    ($a:ident : $a_w:tt) => {
        $crate::Lvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            size: rreil_imm!($a_w)
        }
    };
}

#[macro_export]
macro_rules! rreil_rvalue {
    (?) => { $crate::Rvalue::Undefined };
    ( ( $a:expr ) ) => { ($a).clone().into() };
    ( [ $a:tt ] : $a_w:tt ) => {
        $crate::Rvalue::Constant{
            value: rreil_imm!($a) as u64,
            size: rreil_imm!($a_w)
        }
    };
    ($a:ident : $a_w:tt / $a_o:tt) => {
        $crate::Rvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            offset: rreil_imm!($a_o),
            size: rreil_imm!($a_w)
        }
    };
    ($a:ident : $a_w:tt) => {
        $crate::Rvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            offset: 0,
            size: rreil_imm!($a_w)
        }
    };
}

#[macro_export]
macro_rules! rreil_imm {
    ($x:expr) => ($x as usize);
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Architecture, Match, Region, Result};
    use std::borrow::Cow;

    #[derive(Clone)]
    enum TestArchShort {}
    impl Architecture for TestArchShort {
        type Token = u8;
        type Configuration = ();

        fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
            unimplemented!()
        }

        fn decode(_: &Region, _: u64, _: &Self::Configuration) -> Result<Match<Self>> {
            unimplemented!()
        }
    }

    #[test]
    fn rreil_macro() {
        let t0 = Lvalue::Variable { name: Cow::Borrowed("t0"), subscript: None, size: 12 };
        let eax = Rvalue::Variable {
            name: Cow::Borrowed("eax"),
            subscript: None,
            offset: 0,
            size: 12,
        };
        let val = Rvalue::Constant { value: 1223, size: 12 };

        let _ = rreil!{
            add (t0) , (val), (eax);
            and t0 : 32 , [ 2147483648 ]: 32, eax : 32;
            and t1 : 32 , [2147483648] : 32, ebx : 32;
            sub t2 : 32 , ebx : 32 , eax : 32;
            and t3 : 32 , [2147483648]:32, t2 : 32/32;
            shr SF : 8 , [31] : 8 , t3 : 8/24;
            xor t4 : 32 , t1 : 32 , t0 : 32;
            xor t5 : 32 , t3 : 32 , t0 : 32;
            and t6 : 32 , t5 : 32 , t4 : 32;
            shr OF : 8 , [31] : 8 , t6 : 8/24;
            and t7 : 64 , [4294967296] : 64, t2 : 64;
            shr CF : 8 , [32] : 8 , t7 : 8;
            and t8 : 32 , [4294967295] : 32, t2 : 32/32;
            xor t9 : 8 , OF : 8 , SF : 8;
            sel/32 rax:64, ebx:32;
        };

        let _ = rreil!{
            sub t0:32, eax:32, ebx:32;
            cmpltu CF:1, eax:32, ebx:32;
            cmpleu CForZF:1, eax:32, ebx:32;
            cmplts SFxorOF:1, eax:32, ebx:32;
            cmples SFxorOForZF:1, eax:32, ebx:32;
            cmpeq  ZF:1, eax:32, ebx:32;
            cmplts SF:1, t0:32, [0]:32;
            xor OF:1, SFxorOF:1, SF:1;
        };

        let _ = rreil!{
            sub rax:32, rax:32, [1]:32;
            mov rax:32, [0]:32;
        };

        let _ = rreil!{
            store/ram/le/32 rax:32, [0]:32;
            load/ram/le/32 rax:32, [0]:32;
        };

        let _ = rreil!{
            sext/32 rax:32, ax:16;
            zext/32 rax:32, ax:16;
            mov rax:32, tbx:32;
        };
    }

    fn setup() -> Vec<Statement> {
        vec![
            Statement {
                op: Operation::Add(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Subtract(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Multiply(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::DivideUnsigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::DivideSigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::ShiftLeft(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::ShiftRightUnsigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::ShiftRightSigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Modulo(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::InclusiveOr(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::ExclusiveOr(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::And(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },

            Statement {
                op: Operation::Equal(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::LessOrEqualUnsigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::LessOrEqualSigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::LessUnsigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::LessSigned(Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },

            Statement {
                op: Operation::ZeroExtend(32, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::SignExtend(32, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Select(8, Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Move(Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Call(Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },

            Statement {
                op: Operation::Load(Cow::Borrowed("ram"), Endianess::Little, 8, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },
            Statement {
                op: Operation::Store(Cow::Borrowed("ram"), Endianess::Little, 8, Rvalue::Undefined, Rvalue::Undefined),
                assignee: Lvalue::Undefined,
            },

            Statement {
                op: Operation::Phi(vec![Rvalue::Undefined, Rvalue::Undefined]),
                assignee: Lvalue::Undefined,
            },
        ]
    }

    #[test]
    fn display() {
        for x in setup() {
            println!("{}", x);
        }
    }

    #[test]
    fn operands() {
        for mut x in setup() {
            let Statement { ref mut op, .. } = x;
            op.operands();
            op.operands_mut();
        }
    }

    #[test]
    fn construct_guard() {
        Guard::from_flag(&Rvalue::Undefined).ok().unwrap();
        let g1 = Guard::always();
        let g2 = Guard::never();

        assert!(g1 != g2);
    }

    #[test]
    fn guard_negation() {
        let g = Guard::from_flag(&Rvalue::Undefined).ok().unwrap();
        let ng = g.negation();

        assert!(g != ng);
        assert_eq!(g, ng.negation());
        assert_eq!(g.negation(), ng);
    }
}
