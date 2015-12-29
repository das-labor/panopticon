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

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub enum Operation {
    LogicAnd(Rvalue,Rvalue),
    LogicInclusiveOr(Rvalue,Rvalue),
    LogicExclusiveOr(Rvalue,Rvalue),
    LogicNegation(Rvalue),
    LogicLift(Rvalue),

    IntAnd(Rvalue,Rvalue),
    IntInclusiveOr(Rvalue,Rvalue),
    IntExclusiveOr(Rvalue,Rvalue),
    IntAdd(Rvalue,Rvalue),
    IntSubtract(Rvalue,Rvalue),
    IntMultiply(Rvalue,Rvalue),
    IntDivide(Rvalue,Rvalue),
    IntModulo(Rvalue,Rvalue),
    IntLess(Rvalue,Rvalue),
    IntEqual(Rvalue,Rvalue),
    IntCall(Rvalue),
    IntRightShift(Rvalue,Rvalue),
    IntLeftShift(Rvalue,Rvalue),

    Phi(Vec<Rvalue>),
    Nop(Rvalue),
}

impl<'a> Operation {
    pub fn operands(&'a self) -> Vec<&'a Rvalue> {
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
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub struct Instr {
    pub op: Operation,
    pub assignee: Lvalue,
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
