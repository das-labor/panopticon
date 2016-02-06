/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
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

use value::{Lvalue,Rvalue};
use rustc_serialize::{Encodable,Decodable};
use std::hash::Hash;
use std::fmt::Debug;
use std::fmt::{Formatter,Display,Error};

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub enum Operation<Value: Clone + PartialEq + Eq + Debug + Encodable + Decodable> {
    LogicAnd(Value,Value),
    LogicInclusiveOr(Value,Value),
    LogicExclusiveOr(Value,Value),
    LogicNegation(Value),
    LogicLift(Value),

    IntAnd(Value,Value),
    IntInclusiveOr(Value,Value),
    IntExclusiveOr(Value,Value),
    IntAdd(Value,Value),
    IntSubtract(Value,Value),
    IntMultiply(Value,Value),
    IntDivide(Value,Value),
    IntModulo(Value,Value),
    IntLess(Value,Value),
    IntEqual(Value,Value),
    IntCall(Value),
    IntRightShift(Value,Value),
    IntLeftShift(Value,Value),

    Phi(Vec<Value>),
    Nop(Value),
}

pub fn execute(op: &Operation<Rvalue>) -> Rvalue {
	match op {
        &Operation::LogicAnd(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            if a > 0 && b > 0 { Rvalue::Constant(1) } else { Rvalue::Constant(0) },
        &Operation::LogicAnd(Rvalue::Constant(a),ref b) =>
            if a > 0 { b.clone() } else { Rvalue::Constant(0) },
        &Operation::LogicAnd(ref a,Rvalue::Constant(b)) =>
            if b > 0 { a.clone() } else { Rvalue::Constant(0) },
        &Operation::LogicAnd(_,_) =>
            Rvalue::Undefined,

        &Operation::LogicInclusiveOr(Rvalue::Constant(a),Rvalue::Constant(b)) =>
             if a > 0 || b > 0 { Rvalue::Constant(1) } else { Rvalue::Constant(0) },
        &Operation::LogicInclusiveOr(Rvalue::Constant(a),ref b) =>
             if a > 0 { Rvalue::Constant(1) } else { b.clone() },
        &Operation::LogicInclusiveOr(ref a,Rvalue::Constant(b)) =>
             if b > 0 { Rvalue::Constant(1) } else { a.clone() },
        &Operation::LogicInclusiveOr(_,_) =>
            Rvalue::Undefined,

        &Operation::LogicExclusiveOr(Rvalue::Constant(a),Rvalue::Constant(b)) =>
             if (a > 0) ^ (b > 0) { Rvalue::Constant(1) } else { Rvalue::Constant(0) },
        &Operation::LogicExclusiveOr(ref a,ref b) =>
            if a != &Rvalue::Undefined {
                if a == b { Rvalue::Constant(0) } else { Rvalue::Constant(1) }
            } else {
                Rvalue::Undefined
            },

        &Operation::LogicNegation(Rvalue::Constant(a)) =>
            if a > 0 { Rvalue::Constant(0) } else { Rvalue::Constant(1) },
        &Operation::LogicNegation(_) =>
             Rvalue::Undefined,

        &Operation::LogicLift(Rvalue::Constant(a)) =>
            if a > 0 { Rvalue::Constant(1) } else { Rvalue::Constant(0) },
        &Operation::LogicLift(_) =>
             Rvalue::Undefined,

        &Operation::IntAnd(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a & b),
        &Operation::IntAnd(Rvalue::Constant(0),_) =>
            Rvalue::Constant(0),
        &Operation::IntAnd(_,Rvalue::Constant(0)) =>
            Rvalue::Constant(0),
        &Operation::IntAnd(_,_) =>
            Rvalue::Undefined,

        &Operation::IntInclusiveOr(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a | b),
        &Operation::IntInclusiveOr(Rvalue::Constant(0),ref b) =>
            b.clone(),
        &Operation::IntInclusiveOr(ref a,Rvalue::Constant(0)) =>
            a.clone(),
        &Operation::IntInclusiveOr(_,_) =>
            Rvalue::Undefined,

        &Operation::IntExclusiveOr(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a ^ b),
        &Operation::IntExclusiveOr(ref a,ref b) =>
            if a != &Rvalue::Undefined {
                if a == b { Rvalue::Constant(0) } else { Rvalue::Constant(1) }
            } else {
                Rvalue::Undefined
            },

        &Operation::IntAdd(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a + b),
        &Operation::IntAdd(Rvalue::Constant(0),ref b) =>
            b.clone(),
        &Operation::IntAdd(ref a,Rvalue::Constant(0)) =>
            a.clone(),
        &Operation::IntAdd(_,_) =>
            Rvalue::Undefined,

        &Operation::IntSubtract(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a - b),
        &Operation::IntSubtract(ref a,Rvalue::Constant(0)) =>
            a.clone(),
        &Operation::IntSubtract(_,_) =>
            Rvalue::Undefined,

        &Operation::IntMultiply(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a * b),
        &Operation::IntMultiply(Rvalue::Constant(0),ref b) =>
            Rvalue::Constant(0),
        &Operation::IntMultiply(ref a,Rvalue::Constant(0)) =>
            Rvalue::Constant(0),
        &Operation::IntMultiply(Rvalue::Constant(1),ref b) =>
            b.clone(),
        &Operation::IntMultiply(ref a,Rvalue::Constant(1)) =>
            a.clone(),
        &Operation::IntMultiply(_,_) =>
            Rvalue::Undefined,

        &Operation::IntDivide(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            if b != 0 { Rvalue::Constant(a / b) } else { Rvalue::Undefined },
        &Operation::IntDivide(Rvalue::Constant(0),_) =>
            Rvalue::Constant(0),
        &Operation::IntDivide(_,Rvalue::Constant(0)) =>
            Rvalue::Undefined,
        &Operation::IntDivide(ref a,Rvalue::Constant(1)) =>
            a.clone(),
        &Operation::IntDivide(_,_) =>
            Rvalue::Undefined,

        &Operation::IntModulo(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            if b != 0 { Rvalue::Constant(a % b) } else { Rvalue::Undefined },
        &Operation::IntModulo(Rvalue::Constant(0),_) =>
            Rvalue::Constant(0),
        &Operation::IntModulo(_,Rvalue::Constant(0)) =>
            Rvalue::Undefined,
        &Operation::IntModulo(_,Rvalue::Constant(1)) =>
            Rvalue::Constant(0),
        &Operation::IntModulo(_,_) =>
            Rvalue::Undefined,

        &Operation::IntLess(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            if a < b { Rvalue::Constant(1) } else { Rvalue::Constant(0) },
        &Operation::IntLess(_,Rvalue::Constant(0)) =>
            Rvalue::Constant(0),
        &Operation::IntLess(_,_) =>
            Rvalue::Undefined,

        &Operation::IntEqual(ref a,ref b) =>
            if a != &Rvalue::Undefined {
                if a == b { Rvalue::Constant(1) } else { Rvalue::Constant(0) }
            } else {
                Rvalue::Undefined
            },

        &Operation::IntCall(_) =>
            Rvalue::Undefined,

        &Operation::IntRightShift(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a >> b),
        &Operation::IntRightShift(Rvalue::Constant(0),_) =>
            Rvalue::Constant(0),
        &Operation::IntRightShift(ref a,Rvalue::Constant(0)) =>
            a.clone(),
        &Operation::IntRightShift(_,_) =>
            Rvalue::Undefined,

        &Operation::IntLeftShift(Rvalue::Constant(a),Rvalue::Constant(b)) =>
            Rvalue::Constant(a << b),
        &Operation::IntLeftShift(Rvalue::Constant(0),_) =>
            Rvalue::Constant(0),
        &Operation::IntLeftShift(ref a,Rvalue::Constant(0)) =>
            a.clone(),
        &Operation::IntLeftShift(_,_) =>
            Rvalue::Undefined,

        &Operation::Phi(ref vec) =>
            match vec.len() {
                0 => Rvalue::Undefined,
                1 => vec[0].clone(),
                _ => if vec.iter().all(|x| vec.first().unwrap() == x) { vec[0].clone() } else { Rvalue::Undefined }
            },
        &Operation::Nop(ref a) =>
            a.clone(),
    }
}

impl<'a,Value> Operation<Value> where Value: Clone + PartialEq + Eq + Debug + Encodable + Decodable {
    pub fn operands(&'a self) -> Vec<&'a Value> {
        match self {
            &Operation::LogicAnd(ref a,ref b) => return vec!(a,b),
            &Operation::LogicInclusiveOr(ref a,ref b) => return vec!(a,b),
            &Operation::LogicExclusiveOr(ref a,ref b) => return vec!(a,b),
            &Operation::LogicNegation(ref a) => return vec!(a),
            &Operation::LogicLift(ref a) => return vec!(a),

            &Operation::IntAnd(ref a,ref b) => return vec!(a,b),
            &Operation::IntInclusiveOr(ref a,ref b) => return vec!(a,b),
            &Operation::IntExclusiveOr(ref a,ref b) => return vec!(a,b),
            &Operation::IntAdd(ref a,ref b) => return vec!(a,b),
            &Operation::IntSubtract(ref a,ref b) => return vec!(a,b),
            &Operation::IntMultiply(ref a,ref b) => return vec!(a,b),
            &Operation::IntDivide(ref a,ref b) => return vec!(a,b),
            &Operation::IntModulo(ref a,ref b) => return vec!(a,b),
            &Operation::IntLess(ref a,ref b) => return vec!(a,b),
            &Operation::IntEqual(ref a,ref b) => return vec!(a,b),
            &Operation::IntCall(ref a) => return vec!(a),
            &Operation::IntRightShift(ref a,ref b) => return vec!(a,b),
            &Operation::IntLeftShift(ref a,ref b) => return vec!(a,b),

            &Operation::Phi(ref vec) => return vec.iter().collect(),
            &Operation::Nop(ref a) => return vec!(a),
        }
    }

    pub fn operands_mut(&'a mut self) -> Vec<&'a mut Value> {
        match self {
            &mut Operation::LogicAnd(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LogicInclusiveOr(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LogicExclusiveOr(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::LogicNegation(ref mut a) => return vec!(a),
            &mut Operation::LogicLift(ref mut a) => return vec!(a),

            &mut Operation::IntAnd(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntInclusiveOr(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntExclusiveOr(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntAdd(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntSubtract(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntMultiply(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntDivide(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntModulo(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntLess(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntEqual(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntCall(ref mut a) => return vec!(a),
            &mut Operation::IntRightShift(ref mut a,ref mut b) => return vec!(a,b),
            &mut Operation::IntLeftShift(ref mut a,ref mut b) => return vec!(a,b),

            &mut Operation::Phi(ref mut vec) => return vec.iter_mut().collect(),
            &mut Operation::Nop(ref mut a) => return vec!(a),
        }
    }
}


#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub struct Instr {
    pub op: Operation<Rvalue>,
    pub assignee: Lvalue,
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{} ≔ ",self.assignee.to_rv()));
        match &self.op {
            &Operation::LogicAnd(ref a,ref b) => f.write_fmt(format_args!("{} ∧ {}",a,b)),
            &Operation::LogicInclusiveOr(ref a,ref b) => f.write_fmt(format_args!("{} ∨ {}",a,b)),
            &Operation::LogicExclusiveOr(ref a,ref b) => f.write_fmt(format_args!("{} ⊕ {}",a,b)),
            &Operation::LogicNegation(ref a) => f.write_fmt(format_args!("¬{}",a)),
            &Operation::LogicLift(ref a) => f.write_fmt(format_args!("(bool)({})",a)),

            &Operation::IntAnd(ref a,ref b) => f.write_fmt(format_args!("{} ∧ {}",a,b)),
            &Operation::IntInclusiveOr(ref a,ref b) => f.write_fmt(format_args!("{} ∨ {}",a,b)),
            &Operation::IntExclusiveOr(ref a,ref b) => f.write_fmt(format_args!("{} ⊕ {}",a,b)),
            &Operation::IntAdd(ref a,ref b) => f.write_fmt(format_args!("{} + {}",a,b)),
            &Operation::IntSubtract(ref a,ref b) => f.write_fmt(format_args!("{} - {}",a,b)),
            &Operation::IntMultiply(ref a,ref b) => f.write_fmt(format_args!("{} * {}",a,b)),
            &Operation::IntDivide(ref a,ref b) => f.write_fmt(format_args!("{} / {}",a,b)),
            &Operation::IntModulo(ref a,ref b) => f.write_fmt(format_args!("{} % {}",a,b)),
            &Operation::IntLess(ref a,ref b) => f.write_fmt(format_args!("{} < {}",a,b)),
            &Operation::IntEqual(ref a,ref b) => f.write_fmt(format_args!("{} = {}",a,b)),
            &Operation::IntCall(ref a) => f.write_fmt(format_args!("call({})",a)),
            &Operation::IntRightShift(ref a,ref b) => f.write_fmt(format_args!("{} >> {}",a,b)),
            &Operation::IntLeftShift(ref a,ref b) => f.write_fmt(format_args!("{} << {}",a,b)),

            &Operation::Phi(ref vec) => {
                f.write_str("Φ(");
                for (i,x) in vec.iter().enumerate() {
                    f.write_fmt(format_args!("{}",x));
                    if i < vec.len() - 1 { f.write_str(", "); }
                }
                f.write_str(")")
            },
            &Operation::Nop(ref a) => f.write_fmt(format_args!("{}",a)),
        }
    }
}

mod tests {
    use super::*;
    use value::{Rvalue,Lvalue};

    #[test]
    fn construct() {
        let logic_and = Instr{ op: Operation::LogicAnd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_or = Instr{ op: Operation::LogicInclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_xor = Instr{ op: Operation::LogicExclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_neg = Instr{ op: Operation::LogicNegation(Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_lift = Instr{ op: Operation::LogicLift(Rvalue::Undefined), assignee: Lvalue::Undefined };

        let int_and = Instr{ op: Operation::IntAnd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_or = Instr{ op: Operation::IntInclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_xor = Instr{ op: Operation::IntExclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_add = Instr{ op: Operation::IntAdd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_sub = Instr{ op: Operation::IntSubtract(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_mul = Instr{ op: Operation::IntMultiply(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_div = Instr{ op: Operation::IntDivide(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_mod = Instr{ op: Operation::IntModulo(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_less = Instr{ op: Operation::IntLess(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_equal = Instr{ op: Operation::IntEqual(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_call = Instr{ op: Operation::IntCall(Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_rs = Instr{ op: Operation::IntRightShift(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_ls = Instr{ op: Operation::IntLeftShift(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };

        let phi = Instr{ op: Operation::Phi(Vec::new()), assignee: Lvalue::Undefined };
        let nop = Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: Lvalue::Undefined };

        assert_eq!(logic_and.clone(),logic_and);
        assert_eq!(logic_or.clone(),logic_or);
        assert_eq!(logic_xor.clone(),logic_xor);
        assert_eq!(logic_neg.clone(),logic_neg);
        assert_eq!(logic_lift.clone(),logic_lift);

        assert_eq!(int_and.clone(),int_and);
        assert_eq!(int_or.clone(),int_or);
        assert_eq!(int_xor.clone(),int_xor);
        assert_eq!(int_add.clone(),int_add);
        assert_eq!(int_sub.clone(),int_sub);
        assert_eq!(int_mul.clone(),int_mul);
        assert_eq!(int_div.clone(),int_div);
        assert_eq!(int_mod.clone(),int_mod);
        assert_eq!(int_less.clone(),int_less);
        assert_eq!(int_equal.clone(),int_equal);
        assert_eq!(int_call.clone(),int_call);
        assert_eq!(int_rs.clone(),int_rs);
        assert_eq!(int_ls.clone(),int_ls);

        assert_eq!(phi.clone(),phi);
        assert_eq!(nop.clone(),nop);

        println!("{:?}",logic_and);
        println!("{:?}",logic_or);
        println!("{:?}",logic_xor);
        println!("{:?}",logic_neg);
        println!("{:?}",logic_lift);

        println!("{:?}",int_and);
        println!("{:?}",int_or);
        println!("{:?}",int_xor);
        println!("{:?}",int_add);
        println!("{:?}",int_sub);
        println!("{:?}",int_mul);
        println!("{:?}",int_div);
        println!("{:?}",int_mod);
        println!("{:?}",int_less);
        println!("{:?}",int_equal);
        println!("{:?}",int_call);
        println!("{:?}",int_rs);
        println!("{:?}",int_ls);

        println!("{:?}",phi);
        println!("{:?}",nop);
    }

    #[test]
    fn operands() {
        let logic_and = Instr{ op: Operation::LogicAnd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_or = Instr{ op: Operation::LogicInclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_xor = Instr{ op: Operation::LogicExclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_neg = Instr{ op: Operation::LogicNegation(Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_lift = Instr{ op: Operation::LogicLift(Rvalue::Undefined), assignee: Lvalue::Undefined };

        let int_and = Instr{ op: Operation::IntAnd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_or = Instr{ op: Operation::IntInclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_xor = Instr{ op: Operation::IntExclusiveOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_add = Instr{ op: Operation::IntAdd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_sub = Instr{ op: Operation::IntSubtract(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_mul = Instr{ op: Operation::IntMultiply(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_div = Instr{ op: Operation::IntDivide(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_mod = Instr{ op: Operation::IntModulo(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_less = Instr{ op: Operation::IntLess(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_equal = Instr{ op: Operation::IntEqual(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_call = Instr{ op: Operation::IntCall(Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_rs = Instr{ op: Operation::IntRightShift(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let int_ls = Instr{ op: Operation::IntLeftShift(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };

        let phi = Instr{ op: Operation::Phi(Vec::new()), assignee: Lvalue::Undefined };
        let nop = Instr{ op: Operation::Nop(Rvalue::Undefined), assignee: Lvalue::Undefined };

        assert_eq!(logic_and.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_or.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_xor.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_neg.op.operands(),vec!(&Rvalue::Undefined));
        assert_eq!(logic_lift.op.operands(),vec!(&Rvalue::Undefined));

        assert_eq!(int_and.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_or.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_xor.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_add.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_sub.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_mul.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_div.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_mod.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_less.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_equal.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_rs.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_ls.op.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_call.op.operands(),vec!(&Rvalue::Undefined));

        assert_eq!(phi.op.operands(),Vec::<&Rvalue>::new());
        assert_eq!(nop.op.operands(),vec!(&Rvalue::Undefined));
    }
}
