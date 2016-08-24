/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016 Kai Michaelis
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

use std::fmt::{Formatter,Display,Error,Debug};
use std::convert::From;
use std::borrow::Cow;
use std::num::Wrapping;
use std::u64;
use std::str::{SplitWhitespace,FromStr};
use std::result;
use std::cmp;

use Result;

use rustc_serialize::{Encodable,Decodable};

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable,Hash)]
pub enum Rvalue {
    Undefined,
    Variable{ name: Cow<'static,str>, subscript: Option<usize>, offset: usize, size: usize },
    Constant{ value: u64, size: usize },
}

impl Rvalue {
    pub fn new_bit(v: usize) -> Rvalue {
        Rvalue::Constant{ value: v as u64, size: 1 }
    }

    pub fn new_u8(v: u8) -> Rvalue {
        Rvalue::Constant{ value: v as u64, size: 8 }
    }

    pub fn new_u16(v: u16) -> Rvalue {
        Rvalue::Constant{ value: v as u64, size: 16 }
    }

    pub fn new_u32(v: u32) -> Rvalue {
        Rvalue::Constant{ value: v as u64, size: 32 }
    }

    pub fn new_u64(v: u64) -> Rvalue {
        Rvalue::Constant{ value: v, size: 64 }
    }

    pub fn size(&self) -> Option<usize> {
        match self {
            &Rvalue::Constant{ ref size,.. } => Some(*size),
            &Rvalue::Variable{ ref size,.. } => {
                Some(*size)
            },
            &Rvalue::Undefined => None,
        }
    }

    pub fn extract(&self,s: usize,o: usize) -> Result<Rvalue> {
        if s <= 0 { return Err("can't extract zero bits".into()) }

        match self {
            &Rvalue::Constant{ ref size, ref value } => {
                if *size >= s + o {
                    Ok(Rvalue::Constant{ size: s, value: (*value >> o) % (1 << (s - 1)) })
                } else {
                    Err("Rvalue::extract: invalid argument".into())
                }
            },
            &Rvalue::Variable{ ref size, ref offset, ref name, ref subscript } => {
                if *size >= s + o {
                    Ok(Rvalue::Variable{
                        name: name.clone(),
                        subscript: subscript.clone(),
                        size: s,
                        offset: *offset + o,
                    })
                } else {
                    Err("Rvalue::extract: invalid argument".into())
                }
            },
            &Rvalue::Undefined => Ok(Rvalue::Undefined),
        }
    }
}

impl From<Lvalue> for Rvalue {
    fn from(lv: Lvalue) -> Rvalue {
        match lv {
            Lvalue::Undefined => Rvalue::Undefined,
            Lvalue::Variable{ name, subscript, size } => Rvalue::Variable{
                name: name,
                subscript: subscript,
                size: size,
                offset: 0
            },
        }
    }
}

impl FromStr for Rvalue {
    type Err = ();

    fn from_str<'a>(s: &'a str) -> result::Result<Rvalue,()> {
        if s == "?" {
            Ok(Rvalue::Undefined)
        } else if let Ok(n) = u64::from_str(s) {
            Ok(Rvalue::Constant{ value: n, size: 0 })
        } else {
            let mut ws: SplitWhitespace<'a> = s.split_whitespace();
            let maybe_chr = ws.next();
            match maybe_chr {
                Some(s) => {
                    Ok(Rvalue::Variable{ name: Cow::Owned(s.to_string()), subscript: None, offset: 0, size: 0 })
                },
                None => Err(()),
            }
        }
    }
}

impl Display for Rvalue {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self {
            &Rvalue::Undefined => f.write_str("?"),
            &Rvalue::Constant{ value: v, size: s } => f.write_fmt(format_args!("0x{:x}:{}",v,s)),
            &Rvalue::Variable{ ref name, ref subscript, ref offset, ref size } => {
                try!(f.write_str(name));
                if let &Some(ss) = subscript {
                    try!(f.write_fmt(format_args!("_{}",ss)));
                }
                try!(f.write_fmt(format_args!(":{}",size)));
                if *offset > 0 {
                    try!(f.write_fmt(format_args!("/{}",offset)));
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable,Hash)]
pub enum Lvalue {
    Undefined,
    Variable{ name: Cow<'static,str>, subscript: Option<usize>, size: usize },
}

impl Lvalue {
    pub fn from_rvalue(rv: Rvalue) -> Option<Lvalue> {
        match rv {
            Rvalue::Undefined => Some(Lvalue::Undefined),
            Rvalue::Variable{ name, subscript, size, offset: 0 } =>
                Some(Lvalue::Variable{
                    name: name,
                    subscript: subscript,
                    size: size
                }),
            _ => None,
        }
    }

    pub fn extract(&self,s: usize,o: usize) -> Result<Rvalue> {
        if s <= 0 { return Err("can't extract zero bits".into()) }

        match self {
            &Lvalue::Variable{ ref size, ref name, ref subscript } => {
                if *size >= s + o {
                    Ok(Rvalue::Variable{
                        name: name.clone(),
                        subscript: subscript.clone(),
                        size: s,
                        offset: o,
                    })
                } else {
                    Err("Rvalue::extract: invalid argument".into())
                }
            },
            &Lvalue::Undefined => Ok(Rvalue::Undefined),
        }
    }

    pub fn size(&self) -> Option<usize> {
        match self {
            &Lvalue::Variable{ ref size,.. } => {
                Some(*size)
            },
            &Lvalue::Undefined => None,
        }
    }
}

impl Display for Lvalue {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        f.write_fmt(format_args!("{}",Rvalue::from(self.clone())))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub enum Guard {
    True,
    False,
    Predicate{ flag: Rvalue, expected: bool }
}

impl Guard {
    pub fn from_flag(f: &Rvalue) -> Result<Guard> {
        match f {
            &Rvalue::Undefined => Ok(Guard::Predicate{ flag: f.clone(), expected: true }),
            &Rvalue::Constant{ size: 1, value: 0 } => Ok(Guard::False),
            &Rvalue::Constant{ size: 1, value: 1 } => Ok(Guard::True),
            &Rvalue::Variable{ size: 1,.. } => Ok(Guard::Predicate{ flag: f.clone(), expected: true }),
            _ => Err("Not a flag".into()),
        }
    }

    pub fn never() -> Guard {
        Guard::False
    }

    pub fn always() -> Guard {
        Guard::True
    }

    pub fn negation(&self) -> Guard {
        match self {
            &Guard::True => Guard::False,
            &Guard::False => Guard::True,
            &Guard::Predicate{ ref flag, ref expected } =>
                Guard::Predicate{ flag: flag.clone(), expected: !*expected },
        }
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self {
            &Guard::True => f.write_str("true"),
            &Guard::False => f.write_str("false"),
            &Guard::Predicate{ ref flag, ref expected } => f.write_fmt(format_args!("({} == {})",flag,expected))
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub enum Operation<V: Clone + PartialEq + Eq + Debug + Encodable + Decodable> {
    Add(V,V),
    Subtract(V,V),
    Multiply(V,V),
    DivideUnsigned(V,V),
    DivideSigned(V,V),
    ShiftLeft(V,V),
    ShiftRightUnsigned(V,V),
    ShiftRightSigned(V,V),
    Modulo(V,V),
    And(V,V),
    InclusiveOr(V,V),
    ExclusiveOr(V,V),

    Equal(V,V),
    LessOrEqualUnsigned(V,V),
    LessOrEqualSigned(V,V),
    LessUnsigned(V,V),
    LessSigned(V,V),

    ZeroExtend(usize,V),
    SignExtend(usize,V),
    Move(V),
    Call(V),
    Select(usize,V,V),

    Load(Cow<'static,str>,V),
    Store(Cow<'static,str>,V),

    Phi(Vec<V>),
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub struct Statement {
    pub assignee: Lvalue,
    pub op: Operation<Rvalue>,
}

impl Statement {
    pub fn sanity_check(&self) -> Result<()> {
        // check that argument sizes match
        let typecheck_binop = |a: &Rvalue,b: &Rvalue,assignee: &Lvalue| -> Result<()> {
            if !(a.size() == None || b.size() == None || a.size() == b.size()) {
                return Err("Argument sizes mismatch".into())
            }

            if !(assignee.size() == None || Some(cmp::max(a.size().unwrap_or(0),b.size().unwrap_or(0))) == assignee.size()) {
                return Err("Operation result and assingnee sizes mismatch".into())
            }

            Ok(())
        };
        let typecheck_cmpop = |a: &Rvalue,b: &Rvalue,assignee: &Lvalue| -> Result<()> {
            if !(a.size() == None || b.size() == None || a.size() == b.size()) {
                return Err("Argument sizes mismatch".into())
            }

            if !(assignee.size() == None || assignee.size() == Some(1)) {
                return Err("Compare operation assingnee not a flag".into())
            }

            Ok(())
        };
        let typecheck_unop = |a: &Rvalue,sz: Option<usize>,assignee: &Lvalue| -> Result<()> {
            if sz.is_none() {
                // zext?
                if !(a.size() == None || assignee.size() == None || assignee.size() <= a.size()) {
                    return Err("Operation result and assingnee sizes mismatch".into())
                }
            } else {
                if !(a.size() == None || assignee.size() == None || assignee.size() == sz) {
                    return Err("Operation result and assingnee sizes mismatch".into())
                }
            }
            Ok(())
        };

        try!(match self {
            &Statement{ op: Operation::Add(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::Subtract(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::Multiply(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::DivideUnsigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::DivideSigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ShiftLeft(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ShiftRightUnsigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ShiftRightSigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::Modulo(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::And(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ExclusiveOr(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::InclusiveOr(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),

            &Statement{ op: Operation::Equal(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessOrEqualUnsigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessOrEqualSigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessUnsigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessSigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),

            &Statement{ op: Operation::SignExtend(ref a,ref b), ref assignee } => typecheck_unop(b,Some(*a),assignee),
            &Statement{ op: Operation::ZeroExtend(ref a,ref b), ref assignee } => typecheck_unop(b,Some(*a),assignee),
            &Statement{ op: Operation::Move(ref a), ref assignee } => typecheck_unop(a,None,assignee),
            &Statement{ op: Operation::Select(ref off,ref a,ref b), ref assignee } =>
                if !(assignee.size() == a.size() && *off + b.size().unwrap_or(0) <= a.size().unwrap_or(0)) {
                    return Err("Ill-sized Select operation".into());
                } else {
                    Ok(())
                },

            &Statement{ op: Operation::Call(_), ref assignee } =>
                if !(assignee == &Lvalue::Undefined) {
                    return Err("Call operation can only be assigned to Undefined".into());
                } else {
                    Ok(())
                },

            &Statement{ op: Operation::Load(_,_), ref assignee } =>
                if !(assignee.size().is_some()) {
                    return Err("Memory operation with undefined size".into());
                } else {
                    Ok(())
                },
            &Statement{ op: Operation::Store(_,_), ref assignee } =>
                if !(assignee.size().is_some()) {
                    return Err("Memory operation with undefined size".into());
                } else {
                    Ok(())
                },

            &Statement{ op: Operation::Phi(ref vec), ref assignee } =>
                if !(vec.iter().all(|rv| rv.size() == assignee.size()) && assignee.size() != None) {
                    return Err("Phi arguments must have equal sizes and can't be Undefined".into());
                } else {
                    Ok(())
                },
        });

        if !(self.op.operands().iter().all(|rv| rv.size() != Some(0)) && self.assignee.size() != Some(0)) {
            return Err("Operation argument and/or assignee has size 0".into());
        }

        Ok(())
    }
}

pub fn execute(op: Operation<Rvalue>) -> Rvalue {
	match op {
        Operation::Add(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant{ value: ((a + b) & mask).0, size: s }
        }
        Operation::Add(Rvalue::Constant{ value: 0,.. },ref b) =>
            b.clone(),
        Operation::Add(ref a,Rvalue::Constant{ value: 0,.. }) =>
            a.clone(),
        Operation::Add(_,_) =>
            Rvalue::Undefined,
        Operation::Subtract(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant{ value: ((a - b) & mask).0, size: s }
        }
        Operation::Subtract(ref a,Rvalue::Constant{ value: 0,.. }) =>
            a.clone(),
        Operation::Subtract(_,_) =>
            Rvalue::Undefined,

        Operation::Multiply(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant{ value: ((a * b) & mask).0, size: s }
        }
        Operation::Multiply(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::Multiply(_,Rvalue::Constant{ value: 0, size: s }) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::Multiply(Rvalue::Constant{ value: 1,.. },ref b) =>
            b.clone(),
        Operation::Multiply(ref a,Rvalue::Constant{ value: 1,.. }) =>
            a.clone(),
        Operation::Multiply(_,_) =>
            Rvalue::Undefined,

        Operation::DivideUnsigned(_,Rvalue::Constant{ value: 0,.. }) =>
            Rvalue::Undefined,
        Operation::DivideUnsigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant{ value: ((a * b) & mask).0, size: s }
        }
        Operation::DivideUnsigned(ref a,Rvalue::Constant{ value: 1,.. }) =>
            a.clone(),
        Operation::DivideUnsigned(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::DivideUnsigned(_,_) =>
            Rvalue::Undefined,

        Operation::DivideSigned(_,Rvalue::Constant{ value: 0,.. }) =>
            Rvalue::Undefined,
        Operation::DivideSigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << (s - 1)) - 1 } else { u64::MAX });
            let sign_mask = Wrapping(if s < 64 { 1u64 << s } else { 0u64 });
            Rvalue::Constant{ value: (((a * b) & mask) | ((a ^ b) & sign_mask)).0 , size: s }
        }
        Operation::DivideSigned(ref a,Rvalue::Constant{ value: 1,.. }) =>
            a.clone(),
        Operation::DivideSigned(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::DivideSigned(_,_) =>
            Rvalue::Undefined,

        Operation::Modulo(_,Rvalue::Constant{ value: 0,.. }) =>
            Rvalue::Undefined,
        Operation::Modulo(Rvalue::Constant{ value: a, size: s },Rvalue::Constant{ value: b, size: _s }) => {
            debug_assert!(s == _s);

            let mask = if s < 64 { (1u64 << s) - 1 } else { u64::MAX };
            Rvalue::Constant{ value: (a % b) & mask, size: s }
        }
        Operation::Modulo(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::Modulo(_,Rvalue::Constant{ value: 1, size: s }) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::Modulo(_,_) =>
            Rvalue::Undefined,

        Operation::ShiftLeft(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant{ value: ((a << (b as usize)) & mask).0, size: s }
        },
        Operation::ShiftLeft(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::ShiftLeft(ref a,Rvalue::Constant{ value: 0,.. }) =>
            a.clone(),
        Operation::ShiftLeft(_,_) =>
            Rvalue::Undefined,

        Operation::ShiftRightUnsigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            Rvalue::Constant{ value: ((a >> (b as usize)) & mask).0, size: s }
        },
        Operation::ShiftRightUnsigned(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::ShiftRightUnsigned(ref a,Rvalue::Constant{ value: 0,.. }) =>
            a.clone(),
        Operation::ShiftRightUnsigned(_,_) =>
            Rvalue::Undefined,

        Operation::ShiftRightSigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let mask = Wrapping(if s < 64 { (1u64 << s) - 1 } else { u64::MAX });
            let sign = Wrapping(if s < 64 { 1u64 << (s - 1) } else { 0 });
            Rvalue::Constant{ value: ((((a & mask) >> (b as usize)) & mask) | (a & sign)).0, size: s }
        },
        Operation::ShiftRightSigned(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::ShiftRightSigned(ref a,Rvalue::Constant{ value: 0,.. }) =>
            a.clone(),
        Operation::ShiftRightSigned(_,_) =>
            Rvalue::Undefined,

        Operation::And(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            Rvalue::Constant{ value: (a & b).0, size: s }
        },
        Operation::And(_,Rvalue::Constant{ value: 0, size: s }) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::And(Rvalue::Constant{ value: 0, size: s },_) =>
            Rvalue::Constant{ value: 0, size: s },
        Operation::And(_,_) =>
            Rvalue::Undefined,

        Operation::InclusiveOr(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            Rvalue::Constant{ value: (a | b).0, size: s }
        },
        Operation::InclusiveOr(ref a,Rvalue::Constant{ value: 0,.. }) =>
            a.clone(),
        Operation::InclusiveOr(Rvalue::Constant{ value: 0,.. },ref b) =>
            b.clone(),
        Operation::InclusiveOr(_,_) =>
            Rvalue::Undefined,

        Operation::ExclusiveOr(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            Rvalue::Constant{ value: (a ^ b).0, size: s }
        },
        Operation::ExclusiveOr(_,_) =>
            Rvalue::Undefined,

        Operation::Equal(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            if a == b {
                Rvalue::Constant{ value: 1, size: 1 }
            } else {
                Rvalue::Constant{ value: 0, size: 1 }
            }
        },
        Operation::Equal(_,_) =>
            Rvalue::Undefined,

        Operation::LessOrEqualUnsigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            if a <= b {
                Rvalue::Constant{ value: 1, size: 1 }
            } else {
                Rvalue::Constant{ value: 0, size: 1 }
            }
        },
        Operation::LessOrEqualUnsigned(Rvalue::Constant{ value: 0,.. },_) =>
            Rvalue::Constant{ value: 1, size: 1 },
        Operation::LessOrEqualUnsigned(_,_) =>
            Rvalue::Undefined,

        Operation::LessOrEqualSigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << (s - 1)) - 1 } else { u64::MAX });
            let sign_mask = Wrapping(if s < 64 { 1u64 << (s - 1) } else { 0 });
            if (a & sign_mask) ^ (b & sign_mask) != Wrapping(0) {
                Rvalue::Constant{ value: if a & sign_mask != Wrapping(0) { 1 } else { 0 }, size: 1 }
            } else {
                Rvalue::Constant{ value: if (a & mask) <= (b & mask) { 1 } else { 0 }, size: 1 }
            }
        },
        Operation::LessOrEqualSigned(_,_) =>
            Rvalue::Undefined,

        Operation::LessUnsigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            if a < b {
                Rvalue::Constant{ value: 1, size: 1 }
            } else {
                Rvalue::Constant{ value: 0, size: 1 }
            }
        },
        Operation::LessUnsigned(_,_) =>
            Rvalue::Undefined,

        Operation::LessSigned(Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(s == _s);

            let a = Wrapping(_a);
            let b = Wrapping(_b);
            let mask = Wrapping(if s < 64 { (1u64 << (s - 1)) - 1 } else { u64::MAX });
            let sign_mask = Wrapping(if s < 64 { 1u64 << (s - 1) } else { 0 });
            if (a & sign_mask) ^ (b & sign_mask) != Wrapping(0) {
                Rvalue::Constant{ value: if a & sign_mask != Wrapping(0) { 1 } else { 0 }, size: 1 }
            } else {
                Rvalue::Constant{ value: if (a & mask) < (b & mask) { 1 } else { 0 }, size: 1 }
            }
        },
        Operation::LessSigned(_,_) =>
            Rvalue::Undefined,

        Operation::ZeroExtend(s,Rvalue::Constant{ value: v,.. }) => {
            let mask = if s < 64 { (1u64 << s) - 1 } else { u64::MAX };
            Rvalue::Constant{ value: v & mask, size: s }
        },
        Operation::ZeroExtend(s,Rvalue::Variable{ ref name, ref subscript,.. }) =>
            Rvalue::Variable{ name: name.clone(), subscript: subscript.clone(), offset: 0, size: s },
        Operation::ZeroExtend(_,Rvalue::Undefined) =>
            Rvalue::Undefined,

        Operation::SignExtend(t,Rvalue::Constant{ value: v, size: s,.. }) => {
            let mask = if s < 64 { (1u64 << t) - 1 } else { u64::MAX };
            let sign_mask = if s < 64 { 1u64 << (s - 1) } else { 0 };
            let sign = if t < 64 { 1u64 << (t - 1) } else { 0 };
            Rvalue::Constant{ value: (v & mask) | (if v & sign_mask != 0 { sign } else { 0 }) , size: s }
        },
        Operation::SignExtend(s,Rvalue::Variable{ ref name, ref subscript,.. }) =>
            Rvalue::Variable{ name: name.clone(), subscript: subscript.clone(), offset: 0, size: s },
        Operation::SignExtend(_,Rvalue::Undefined) =>
            Rvalue::Undefined,

        Operation::Move(ref a) =>
            a.clone(),

        Operation::Call(_) =>
            Rvalue::Undefined,

        Operation::Select(off,Rvalue::Constant{ value: _a, size: s },Rvalue::Constant{ value: _b, size: _s }) => {
            debug_assert!(off + _s <= s);

            let hi = _a >> (off + _s);
            let lo = _a % (1 << off);

            Rvalue::Constant{ value: lo | (_b << off) | (hi << (off + _s)), size: s }
        },
        Operation::Select(_,_,_) =>
            Rvalue::Undefined,

        Operation::Load(_,_) =>
            Rvalue::Undefined,

        Operation::Store(_,_) =>
            Rvalue::Undefined,

        Operation::Phi(ref vec) =>
            match vec.len() {
                0 => Rvalue::Undefined,
                1 => vec[0].clone(),
                _ => if vec.iter().all(|x| vec.first().unwrap() == x) { vec[0].clone() } else { Rvalue::Undefined }
            },
    }
}

pub fn lift<V: Clone + PartialEq + Eq + Debug + Encodable + Decodable,W: Clone + PartialEq + Eq + Debug + Encodable + Decodable,F: Fn(&V) -> W>(op: &Operation<V>, m: &F) -> Operation<W> {
    let args = op.operands().iter().cloned().map(m).collect::<Vec<_>>();
    match op {
        &Operation::Phi(_) => Operation::Phi(args),
        &Operation::Load(ref s,_) => Operation::Load(s.clone(),args[0].clone()),
        &Operation::Store(ref s,_) => Operation::Store(s.clone(),args[0].clone()),
        &Operation::Add(_,_) => Operation::Add(args[0].clone(),args[1].clone()),
        &Operation::Subtract(_,_) => Operation::Subtract(args[0].clone(),args[1].clone()),
        &Operation::Multiply(_,_) => Operation::Multiply(args[0].clone(),args[1].clone()),
        &Operation::DivideUnsigned(_,_) => Operation::DivideUnsigned(args[0].clone(),args[1].clone()),
        &Operation::DivideSigned(_,_) => Operation::DivideSigned(args[0].clone(),args[1].clone()),
        &Operation::ShiftLeft(_,_) => Operation::ShiftLeft(args[0].clone(),args[1].clone()),
        &Operation::ShiftRightUnsigned(_,_) => Operation::ShiftRightUnsigned(args[0].clone(),args[1].clone()),
        &Operation::ShiftRightSigned(_,_) => Operation::ShiftRightSigned(args[0].clone(),args[1].clone()),
        &Operation::Modulo(_,_) => Operation::Modulo(args[0].clone(),args[1].clone()),
        &Operation::And(_,_) => Operation::And(args[0].clone(),args[1].clone()),
        &Operation::InclusiveOr(_,_) => Operation::InclusiveOr(args[0].clone(),args[1].clone()),
        &Operation::ExclusiveOr(_,_) => Operation::ExclusiveOr(args[0].clone(),args[1].clone()),
        &Operation::Equal(_,_) => Operation::Equal(args[0].clone(),args[1].clone()),
        &Operation::LessUnsigned(_,_) => Operation::LessUnsigned(args[0].clone(),args[1].clone()),
        &Operation::LessSigned(_,_) => Operation::LessSigned(args[0].clone(),args[1].clone()),
        &Operation::LessOrEqualUnsigned(_,_) => Operation::LessOrEqualUnsigned(args[0].clone(),args[1].clone()),
        &Operation::LessOrEqualSigned(_,_) => Operation::LessOrEqualSigned(args[0].clone(),args[1].clone()),
        &Operation::Call(_) => Operation::Call(args[0].clone()),
        &Operation::Move(_) => Operation::Move(args[0].clone()),
        &Operation::Select(ref off, _, _) => Operation::Select(*off,args[0].clone(),args[1].clone()),
        &Operation::ZeroExtend(ref sz, _) => Operation::ZeroExtend(*sz,args[0].clone()),
        &Operation::SignExtend(ref sz,_) => Operation::SignExtend(*sz,args[0].clone()),
    }
}

impl<'a,V> Operation<V> where V: Clone + PartialEq + Eq + Debug + Encodable + Decodable {
    pub fn operands(&'a self) -> Vec<&'a V> {
        match *self {
            Operation::Add(ref a,ref b) => return vec!(a,b),
            Operation::Subtract(ref a,ref b) => return vec!(a,b),
            Operation::Multiply(ref a,ref b) => return vec!(a,b),
            Operation::DivideUnsigned(ref a,ref b) => return vec!(a,b),
            Operation::DivideSigned(ref a,ref b) => return vec!(a,b),
            Operation::ShiftLeft(ref a,ref b) => return vec!(a,b),
            Operation::ShiftRightUnsigned(ref a,ref b) => return vec!(a,b),
            Operation::ShiftRightSigned(ref a,ref b) => return vec!(a,b),
            Operation::Modulo(ref a,ref b) => return vec!(a,b),
            Operation::And(ref a,ref b) => return vec!(a,b),
            Operation::InclusiveOr(ref a,ref b) => return vec!(a,b),
            Operation::ExclusiveOr(ref a,ref b) => return vec!(a,b),

            Operation::Equal(ref a,ref b) => return vec!(a,b),
            Operation::LessOrEqualUnsigned(ref a,ref b) => return vec!(a,b),
            Operation::LessOrEqualSigned(ref a,ref b) => return vec!(a,b),
            Operation::LessUnsigned(ref a,ref b) => return vec!(a,b),
            Operation::LessSigned(ref a,ref b) => return vec!(a,b),

            Operation::ZeroExtend(_,ref a) => return vec!(a),
            Operation::SignExtend(_,ref a) => return vec!(a),
            Operation::Move(ref a) => return vec!(a),
            Operation::Call(ref a) => return vec!(a),
            Operation::Select(_,ref a,ref b) => return vec!(a,b),

            Operation::Load(_,ref b) => return vec!(b),
            Operation::Store(_,ref b) => return vec!(b),

            Operation::Phi(ref vec) => return vec.iter().collect(),
        }
    }

    pub fn operands_mut(&'a mut self) -> Vec<&'a mut V> {
        match self {
            &mut Operation::Add(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::Subtract(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::Multiply(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::DivideUnsigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::DivideSigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::ShiftLeft(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::ShiftRightUnsigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::ShiftRightSigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::Modulo(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::And(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::InclusiveOr(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::ExclusiveOr(ref mut a,ref mut b) => return vec!(a,b),

            &mut Operation::Equal(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LessOrEqualUnsigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LessOrEqualSigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LessUnsigned(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LessSigned(ref mut a,ref mut b) => return vec!(a,b),

            &mut Operation::ZeroExtend(_,ref mut a) => return vec!(a),
            &mut Operation::SignExtend(_,ref mut a) => return vec!(a),
            &mut Operation::Move(ref mut a) => return vec!(a),
            &mut Operation::Call(ref mut a) => return vec!(a),
            &mut Operation::Select(_,ref mut a,ref mut b) => return vec!(a,b),

            &mut Operation::Load(_,ref mut b) => return vec!(b),
            &mut Operation::Store(_,ref mut b) => return vec!(b),

            &mut Operation::Phi(ref mut vec) => return vec.iter_mut().collect(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self.op {
            Operation::Add(ref a,ref b) => f.write_fmt(format_args!("add {}, {}, {}",self.assignee,a,b)),
            Operation::Subtract(ref a,ref b) => f.write_fmt(format_args!("sub {}, {}, {}",self.assignee,a,b)),
            Operation::Multiply(ref a,ref b) => f.write_fmt(format_args!("mul {}, {}, {}",self.assignee,a,b)),
            Operation::DivideUnsigned(ref a,ref b) => f.write_fmt(format_args!("divu {}, {}, {}",self.assignee,a,b)),
            Operation::DivideSigned(ref a,ref b) => f.write_fmt(format_args!("divs {}, {}, {}",self.assignee,a,b)),
            Operation::ShiftLeft(ref a,ref b) => f.write_fmt(format_args!("shl {}, {}, {}",self.assignee,a,b)),
            Operation::ShiftRightUnsigned(ref a,ref b) => f.write_fmt(format_args!("shru {}, {}, {}",self.assignee,a,b)),
            Operation::ShiftRightSigned(ref a,ref b) => f.write_fmt(format_args!("shrs {}, {}, {}",self.assignee,a,b)),
            Operation::Modulo(ref a,ref b) => f.write_fmt(format_args!("mod {}, {}, {}",self.assignee,a,b)),
            Operation::And(ref a,ref b) => f.write_fmt(format_args!("and {}, {}, {}",self.assignee,a,b)),
            Operation::InclusiveOr(ref a,ref b) => f.write_fmt(format_args!("or {}, {}, {}",self.assignee,a,b)),
            Operation::ExclusiveOr(ref a,ref b) => f.write_fmt(format_args!("xor {}, {}, {}",self.assignee,a,b)),

            Operation::Equal(ref a,ref b) => f.write_fmt(format_args!("cmpeq {}, {}, {}",self.assignee,a,b)),
            Operation::LessOrEqualUnsigned(ref a,ref b) => f.write_fmt(format_args!("cmpleu {}, {}, {}",self.assignee,a,b)),
            Operation::LessOrEqualSigned(ref a,ref b) => f.write_fmt(format_args!("cmples {}, {}, {}",self.assignee,a,b)),
            Operation::LessUnsigned(ref a,ref b) => f.write_fmt(format_args!("cmplu {}, {}, {}",self.assignee,a,b)),
            Operation::LessSigned(ref a,ref b) => f.write_fmt(format_args!("cmpls {}, {}, {}",self.assignee,a,b)),

            Operation::ZeroExtend(s,ref a) => f.write_fmt(format_args!("convert_{} {}, {}",s,self.assignee,a)),
            Operation::SignExtend(s,ref a) => f.write_fmt(format_args!("sign-extend_{} {}, {}",s,self.assignee,a)),
            Operation::Select(s,ref a,ref b) => f.write_fmt(format_args!("select_{} {}, {}, {}",s,self.assignee,a,b)),
            Operation::Move(ref a) => f.write_fmt(format_args!("mov {}, {}",self.assignee,a)),
            Operation::Call(ref a) => f.write_fmt(format_args!("call {}, {}",self.assignee,a)),

            Operation::Load(ref r,ref b) => f.write_fmt(format_args!("load_{} {}, {}",r,self.assignee,b)),
            Operation::Store(ref r,ref b) => f.write_fmt(format_args!("store_{} {}, {}",r,self.assignee,b)),

            Operation::Phi(ref vec) => {
                try!(f.write_fmt(format_args!("phi {}",self.assignee)));
                for (i,x) in vec.iter().enumerate() {
                    try!(f.write_fmt(format_args!("{}",x)));
                    if i < vec.len() - 1 { try!(f.write_str(", ")); }
                }
                Ok(())
            },
        }
    }
}

macro_rules! rreil {
    ( ) => {vec![]};
    ( add $($cdr:tt)* ) => { rreil_binop!(Add # $($cdr)*); };
    ( sub $($cdr:tt)* ) => { rreil_binop!(Subtract # $($cdr)*); };
    ( mul $($cdr:tt)* ) => { rreil_binop!(Multiply # $($cdr)*); };
    ( div $($cdr:tt)* ) => { rreil_binop!(DivideUnsigned # $($cdr)*); };
    ( divs $($cdr:tt)* ) => { rreil_binop!(DivideSigned # $($cdr)*); };
    ( shl $($cdr:tt)* ) => { rreil_binop!(ShiftLeft # $($cdr)*); };
    ( shr $($cdr:tt)* ) => { rreil_binop!(ShiftRightUnsigned # $($cdr)*); };
    ( shrs $($cdr:tt)* ) => { rreil_binop!(ShiftRightSigned # $($cdr)*); };
    ( mod $($cdr:tt)* ) => { rreil_binop!(Modulo # $($cdr)*); };
    ( and $($cdr:tt)* ) => { rreil_binop!(And # $($cdr)*); };
    ( xor $($cdr:tt)* ) => { rreil_binop!(ExclusiveOr # $($cdr)*); };
    ( or $($cdr:tt)* ) => { rreil_binop!(InclusiveOr # $($cdr)*); };

    ( cmpeq $($cdr:tt)* ) => { rreil_binop!(Equal # $($cdr)*); };
    ( cmpleu $($cdr:tt)* ) => { rreil_binop!(LessOrEqualUnsigned # $($cdr)*); };
    ( cmples $($cdr:tt)* ) => { rreil_binop!(LessOrEqualSigned # $($cdr)*); };
    ( cmpltu $($cdr:tt)* ) => { rreil_binop!(LessUnsigned # $($cdr)*); };
    ( cmplts $($cdr:tt)* ) => { rreil_binop!(LessSigned # $($cdr)*); };

    ( sel / $off:tt $($cdr:tt)* ) => { rreil_selop!(Select # $off # $($cdr)*); };
    ( sext / $sz:tt $($cdr:tt)* ) => { rreil_extop!(SignExtend # $sz # $($cdr)*); };
    ( zext / $sz:tt $($cdr:tt)* ) => { rreil_extop!(ZeroExtend # $sz # $($cdr)*); };
    ( mov $($cdr:tt)* ) => { rreil_unop!(Move # $($cdr)*); };
    ( call $($cdr:tt)* ) => { rreil_unop!(Call # $($cdr)*); };

    ( load / $r:ident   $($cdr:tt)* ) => { rreil_memop!(Load # $r # $($cdr)*); };
    ( store / $r:ident $($cdr:tt)* ) => { rreil_memop!(Store # $r # $($cdr)*); };
}

include!("rreil.rs");

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

macro_rules! rreil_imm {
    ($x:expr) => ($x as usize);
}

#[cfg(test)]
mod tests {
    use super::*;
    use {
        Rvalue,
        Lvalue,
        Architecture,
        LayerIter,
        Result,
        Disassembler,
    };
    use std::sync::Arc;
    use std::borrow::Cow;

    #[derive(Clone)]
    enum TestArchShort {}
    impl Architecture for TestArchShort {
        type Token = u8;
        type Configuration = ();

        fn prepare(_: LayerIter,_: &Self::Configuration) -> Result<Vec<(&'static str,u64,&'static str)>> {
            unimplemented!()
        }

        fn disassembler(_: &Self::Configuration) -> Arc<Disassembler<Self>> {
            unimplemented!()
        }
    }

    #[test]
    fn rreil_macro() {
        let mut cg = CodeGen::<TestArchShort>::new(0,&());
        let t0 = Lvalue::Variable{ name: Cow::Borrowed("t0"), subscript: None, size: 12 };
        let eax = Rvalue::Variable{ name: Cow::Borrowed("eax"), subscript: None, offset: 0, size: 12 };
        let val = Rvalue::Constant{ value: 1223, size: 12 };

        rreil!{
            cg:
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
        }

        rreil!{
            cg:
            sub t0:32, eax:32, ebx:32;
            cmpltu CF:1, eax:32, ebx:32;
            cmpleu CForZF:1, eax:32, ebx:32;
            cmplts SFxorOF:1, eax:32, ebx:32;
            cmples SFxorOForZF:1, eax:32, ebx:32;
            cmpeq  ZF:1, eax:32, ebx:32;
            cmplts SF:1, t0:32, [0]:32;
            xor OF:1, SFxorOF:1, SF:1;
        }

        rreil!{
            cg:
            sub rax:32, rax:32, [1]:32;
            mov rax:32, [0]:32;
        }

        rreil!{
            cg:
            store/ram rax:32, [0]:32;
            load/ram rax:32, [0]:32;
        }

        rreil!{
            cg:
            sext/32 rax:32, ax:16;
            zext/32 rax:32, ax:16;
            mov rax:32, tbx:32;
        }
    }

    fn setup() -> Vec<Statement> {
        vec![
            Statement{ op: Operation::Add(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Subtract(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Multiply(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::DivideUnsigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::DivideSigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::ShiftLeft(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::ShiftRightUnsigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::ShiftRightSigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Modulo(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::InclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::ExclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::And(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },

            Statement{ op: Operation::Equal(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::LessOrEqualUnsigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::LessOrEqualSigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::LessUnsigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::LessSigned(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },

            Statement{ op: Operation::ZeroExtend(32,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::SignExtend(32,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Select(8,Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Move(Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Call(Rvalue::Undefined), assignee: Lvalue::Undefined },

            Statement{ op: Operation::Load(Cow::Borrowed("ram"),Rvalue::Undefined), assignee: Lvalue::Undefined },
            Statement{ op: Operation::Store(Cow::Borrowed("ram"),Rvalue::Undefined), assignee: Lvalue::Undefined },

            Statement{ op: Operation::Phi(vec![Rvalue::Undefined,Rvalue::Undefined]), assignee: Lvalue::Undefined },
        ]
    }

    #[test]
    fn display() {
        for x in setup() {
            println!("{}",x);
        }
    }

    #[test]
    fn operands() {
        for mut x in setup() {
            let Statement{ ref mut op,.. } = x;
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
}
