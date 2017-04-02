/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2016,2017 Panopticon Authors
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

//! Bounded Address Tracking.
//!
//! TODO

use std::borrow::Cow;
use std::ops::Range;
use std::collections::HashMap;

use {
    Rvalue,
    Avalue,
    Constraint,
    ProgramPoint,
    Operation,
    il,
    Region,
};

pub const VERSION_LIMIT: usize = 10;

// None == global region
#[derive(Debug,PartialEq,Eq,Clone,Hash,RustcDecodable,RustcEncodable)]
pub enum BoundedAddrTrack {
    Join,
    Region{ region: Option<(Cow<'static,str>,usize)> },
    Offset{ region: Option<(Cow<'static,str>,usize)>, offset: u64, offset_size: usize },
    Meet,
}

macro_rules! addrtrack_op {
    ($t:path: $r1:ident,$r2:ident,$val1:ident,$val2:ident,$sz1:ident,$sz2:ident) => {{
        let r1 = $r1; let r2 = $r2; let val1 = $val1; let val2 = $val2;
        let sz1 = $sz1; let sz2 = $sz2;
        if r1.is_some() || r2.is_some() {
            let (rX,verX) = if r1.is_some() { r1.clone().unwrap() } else { r2.clone().unwrap() };

            if verX < VERSION_LIMIT {
                let tmp = il::execute($t(Rvalue::Constant{ value: *val1, size: *sz1 },
                                                    Rvalue::Constant{ value: *val2, size: *sz2 }));
                if let Rvalue::Constant{ ref value, ref size } = tmp {
                    BoundedAddrTrack::Offset{ region: Some((rX.clone(),verX + 1)), offset: *value, offset_size: *size }
                } else {
                    BoundedAddrTrack::Join
                }
            } else {
                BoundedAddrTrack::Join
            }
        } else {
            BoundedAddrTrack::Join
        }
    }}
}

impl Avalue for BoundedAddrTrack {
    fn abstract_value(v: &Rvalue) -> Self {
        if let &Rvalue::Constant{ ref value, ref size } = v {
            BoundedAddrTrack::Offset{ region: None, offset: *value, offset_size: *size }
        } else {
            BoundedAddrTrack::Join
        }
    }

    fn abstract_constraint(constr: &Constraint) -> Self {
        if let &Constraint::Equal(Rvalue::Constant{ ref value, ref size }) = constr {
            BoundedAddrTrack::Offset{ region: None, offset: *value, offset_size: *size }
        } else {
            BoundedAddrTrack::Join
        }
    }

    fn execute(pp: &ProgramPoint, op: &Operation<Self>/*, reg: Option<&Region>,
               symbolic: &HashMap<Range<u64>,Cow<'static,str>>, initial: &HashMap<(Cow<'static,str>,usize),Self>*/) -> Self {
        fn execute(op: Operation<Rvalue>) -> BoundedAddrTrack {
            let tmp = il::execute(op);
            if let Rvalue::Constant{ ref value, ref size } = tmp {
                BoundedAddrTrack::Offset{ region: None, offset: *value, offset_size: *size }
            } else {
                BoundedAddrTrack::Join
            }
        }


        match *op {
            // EvalOp
            Operation::And(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::And(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::And(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::And: r1,r2,val1,val2,sz1,sz2),

            Operation::InclusiveOr(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::InclusiveOr(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::InclusiveOr(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::InclusiveOr: r1,r2,val1,val2,sz1,sz2),

            Operation::ExclusiveOr(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::ExclusiveOr(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::ExclusiveOr(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::ExclusiveOr: r1,r2,val1,val2,sz1,sz2),

            Operation::Add(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::Add(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),

            Operation::Add(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 })
            if r1.is_some() || r2.is_some() => {
                let (rX,verX) = if r1.is_some() { r1.clone().unwrap() } else { r2.clone().unwrap() };
                let tmp = il::execute(Operation::Add(Rvalue::Constant{ value: *val1, size: *sz1 },
                                                     Rvalue::Constant{ value: *val2, size: *sz2 }));
                if let Rvalue::Constant{ ref value, ref size } = tmp {
                    BoundedAddrTrack::Offset{ region: Some((rX.clone(),verX)), offset: *value, offset_size: *size }
                } else {
                    BoundedAddrTrack::Join
                }
            }

            Operation::Subtract(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::Subtract(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::Subtract(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::Subtract: r1,r2,val1,val2,sz1,sz2),

            Operation::Multiply(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::Multiply(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::Multiply(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::Multiply: r1,r2,val1,val2,sz1,sz2),

            Operation::DivideUnsigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::DivideUnsigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::DivideUnsigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::DivideUnsigned: r1,r2,val1,val2,sz1,sz2),

            Operation::DivideSigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::DivideSigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::DivideSigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::DivideSigned: r1,r2,val1,val2,sz1,sz2),

            Operation::Modulo(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::Modulo(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::Modulo(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::Modulo: r1,r2,val1,val2,sz1,sz2),

            Operation::ShiftLeft(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::ShiftLeft(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::ShiftLeft(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::ShiftLeft: r1,r2,val1,val2,sz1,sz2),

            Operation::ShiftRightUnsigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::ShiftRightUnsigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::ShiftRightUnsigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::ShiftRightUnsigned: r1,r2,val1,val2,sz1,sz2),

            Operation::ShiftRightSigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::ShiftRightSigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::ShiftRightSigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::ShiftRightSigned: r1,r2,val1,val2,sz1,sz2),

            Operation::LessOrEqualSigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::LessOrEqualSigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::LessOrEqualSigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::LessOrEqualSigned: r1,r2,val1,val2,sz1,sz2),

            Operation::LessOrEqualUnsigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::LessOrEqualUnsigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::LessOrEqualUnsigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::LessOrEqualUnsigned: r1,r2,val1,val2,sz1,sz2),

            Operation::LessUnsigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::LessUnsigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::LessUnsigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::LessUnsigned: r1,r2,val1,val2,sz1,sz2),

            Operation::LessSigned(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::LessSigned(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::LessSigned(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::LessSigned: r1,r2,val1,val2,sz1,sz2),

            Operation::Equal(BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::Equal(Rvalue::Constant{ value: *val1, size: *sz1 },
                                       Rvalue::Constant{ value: *val2, size: *sz2 })),
            Operation::Equal(BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 },
                           BoundedAddrTrack::Offset{ region: ref r2, offset: ref val2, offset_size: ref sz2 }) =>
                addrtrack_op!(Operation::Equal: r1,r2,val1,val2,sz1,sz2),

            Operation::Move(ref a) => a.clone(),

            Operation::ZeroExtend(ref sz, BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 }) => {
                let tmp = il::execute(Operation::ZeroExtend(*sz,Rvalue::Constant{ value: *val1, size: *sz1 }));
                if let Rvalue::Constant{ ref value, ref size } = tmp {
                    BoundedAddrTrack::Offset{ region: r1.clone(), offset: *value, offset_size: *size }
                } else {
                    BoundedAddrTrack::Join
                }
            },
            Operation::SignExtend(ref sz, BoundedAddrTrack::Offset{ region: ref r1, offset: ref val1, offset_size: ref sz1 }) => {
                let tmp = il::execute(Operation::SignExtend(*sz,Rvalue::Constant{ value: *val1, size: *sz1 }));
                if let Rvalue::Constant{ ref value, ref size } = tmp {
                    BoundedAddrTrack::Offset{ region: r1.clone(), offset: *value, offset_size: *size }
                } else {
                    BoundedAddrTrack::Join
                }
            },
            Operation::Select(ref off,
                              BoundedAddrTrack::Offset{ region: None, offset: ref val1, offset_size: ref sz1 },
                              BoundedAddrTrack::Offset{ region: None, offset: ref val2, offset_size: ref sz2 }) =>
                execute(Operation::Select(*off,
                                          Rvalue::Constant{ value: *val1, size: *sz1 },
                                          Rvalue::Constant{ value: *val2, size: *sz2 })),

            /*
            // EvalMem
            Operation::Load(ref r,ref endian,ref sz,ref a) => {
                if let &BoundedAddrTrack::Offset{ region: ref reg, offset: ref val,.. } = a {
                    if reg.is_none() {
                        if let Some(ref sym) = symbolic.iter().find(|&(k,v)| k.start == *val && k.end == *val + (*sz / 8) as u64) {
                            BoundedAddrTrack::Offset{ region: Some((sym.1.clone(),0)), offset: 0, offset_size: *sz }
                        } else {
                            BoundedAddrTrack::Join
                        }
                    } else {
                        BoundedAddrTrack::Join
                    }
                } else {
                    BoundedAddrTrack::Join
                }
            }

            // AssignMem
            Operation::Store(ref r,ref endian,ref sz,ref a) => BoundedAddrTrack::Join,
            */

            Operation::Phi(ref ops) => {
                match ops.len() {
                    0 => unreachable!("Phi function w/o arguments"),
                    1 => ops[0].clone(),
                    _ => ops.iter().fold(BoundedAddrTrack::Meet,|acc,x| acc.combine(&x))
                }
            }

            //Operation::Initialize(ref name, ref size) =>
            //    initial.get(&(name.clone(),*size)).unwrap_or(&BoundedAddrTrack::Meet).clone(),

            _ => BoundedAddrTrack::Join,
        }
    }

    fn narrow(&self, a: &Self) -> Self {
        a.clone()
    }

    fn combine(&self,a: &Self) -> Self {
        if *a == *self {
            a.clone()
        } else {
            match (self,a) {
                (&BoundedAddrTrack::Join,_) => BoundedAddrTrack::Join,
                (_,&BoundedAddrTrack::Join) => BoundedAddrTrack::Join,
                (&BoundedAddrTrack::Region{ region: ref a },&BoundedAddrTrack::Region{ region: ref b }) => {
                    if a == b {
                        BoundedAddrTrack::Region{ region: a.clone() }
                    } else {
                        BoundedAddrTrack::Join
                    }
                }
                (&BoundedAddrTrack::Offset{ region: ref a,.. },&BoundedAddrTrack::Region{ region: ref b }) => {
                    if a == b {
                        BoundedAddrTrack::Region{ region: a.clone() }
                    } else {
                        BoundedAddrTrack::Join
                    }
                }
                (&BoundedAddrTrack::Region{ region: ref a },&BoundedAddrTrack::Offset{ region: ref b,.. }) => {
                    if a == b {
                        BoundedAddrTrack::Region{ region: a.clone() }
                    } else {
                        BoundedAddrTrack::Join
                    }
                }
                (&BoundedAddrTrack::Offset{ region: ref a,.. },&BoundedAddrTrack::Offset{ region: ref b,.. }) => {
                    if a == b {
                        BoundedAddrTrack::Region{ region: a.clone() }
                    } else {
                        BoundedAddrTrack::Join
                    }
                }
                (&BoundedAddrTrack::Meet,b) => b.clone(),
                (a,&BoundedAddrTrack::Meet) => a.clone(),
            }
        }
    }

    fn widen(&self,s: &Self) -> Self {
        s.clone()
    }

    fn initial() -> Self {
        BoundedAddrTrack::Meet
    }

    fn more_exact(&self, a: &Self) -> bool {
        if self == a {
            false
        } else {
            match (self,a) {
                (&BoundedAddrTrack::Join,_) => true,
                (_,&BoundedAddrTrack::Join) => false,
                (_,&BoundedAddrTrack::Meet) => true,
                (&BoundedAddrTrack::Meet,_) => false,
                (&BoundedAddrTrack::Region{ .. },&BoundedAddrTrack::Offset{ .. }) => true,
                (&BoundedAddrTrack::Offset{ .. },&BoundedAddrTrack::Region{ .. }) => false,
                _ => unreachable!(),
            }
        }
    }

    fn extract(&self,size: usize,offset: usize) -> Self {
        match self {
            &BoundedAddrTrack::Join => BoundedAddrTrack::Join,
            &BoundedAddrTrack::Meet => BoundedAddrTrack::Meet,
            &BoundedAddrTrack::Region{ region: ref r } => BoundedAddrTrack::Region{ region: r.clone() },
            &BoundedAddrTrack::Offset{ region: ref r, offset: ref v,.. } =>
                BoundedAddrTrack::Offset{ region: r.clone(), offset: (v >> offset) % (1 << (size - 1)), offset_size: size },
        }
    }
}
