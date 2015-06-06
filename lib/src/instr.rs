use value::{Lvalue,Rvalue};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Operation {
    LogicAnd(Rvalue,Rvalue),
    LogicOr(Rvalue,Rvalue),
    LogicNegation(Rvalue),
    LogicImplication(Rvalue,Rvalue),
    LogicEquivalence(Rvalue,Rvalue),
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

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Instr {
    pub op: Operation,
    pub assignee: Lvalue,
}

impl Instr {
    pub fn operands(&self) -> Vec<&Rvalue> {
        match self.op {
            Operation::LogicAnd(ref a,ref b) => return vec!(a,b),
            Operation::LogicOr(ref a,ref b) => return vec!(a,b),
            Operation::LogicNegation(ref a) => return vec!(a),
            Operation::LogicImplication(ref a,ref b) => return vec!(a,b),
            Operation::LogicEquivalence(ref a,ref b) => return vec!(a,b),
            Operation::LogicLift(ref a) => return vec!(a),

            Operation::IntAnd(ref a,ref b) => return vec!(a,b),
            Operation::IntInclusiveOr(ref a,ref b) => return vec!(a,b),
            Operation::IntExclusiveOr(ref a,ref b) => return vec!(a,b),
            Operation::IntAdd(ref a,ref b) => return vec!(a,b),
            Operation::IntSubtract(ref a,ref b) => return vec!(a,b),
            Operation::IntMultiply(ref a,ref b) => return vec!(a,b),
            Operation::IntDivide(ref a,ref b) => return vec!(a,b),
            Operation::IntModulo(ref a,ref b) => return vec!(a,b),
            Operation::IntLess(ref a,ref b) => return vec!(a,b),
            Operation::IntEqual(ref a,ref b) => return vec!(a,b),
            Operation::IntCall(ref a) => return vec!(a),
            Operation::IntRightShift(ref a,ref b) => return vec!(a,b),
            Operation::IntLeftShift(ref a,ref b) => return vec!(a,b),

            Operation::Phi(ref vec) => return vec.iter().collect(),
            Operation::Nop(ref a) => return vec!(a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::{Rvalue,Lvalue};

    #[test]
    fn construct() {
        let logic_and = Instr{ op: Operation::LogicAnd(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_or = Instr{ op: Operation::LogicOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_neg = Instr{ op: Operation::LogicNegation(Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_imp = Instr{ op: Operation::LogicImplication(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_equiv = Instr{ op: Operation::LogicEquivalence(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
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
        assert_eq!(logic_neg.clone(),logic_neg);
        assert_eq!(logic_imp.clone(),logic_imp);
        assert_eq!(logic_equiv.clone(),logic_equiv);
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
        println!("{:?}",logic_neg);
        println!("{:?}",logic_imp);
        println!("{:?}",logic_equiv);
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
        let logic_or = Instr{ op: Operation::LogicOr(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_neg = Instr{ op: Operation::LogicNegation(Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_imp = Instr{ op: Operation::LogicImplication(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
        let logic_equiv = Instr{ op: Operation::LogicEquivalence(Rvalue::Undefined,Rvalue::Undefined), assignee: Lvalue::Undefined };
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

        assert_eq!(logic_and.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_or.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_neg.operands(),vec!(&Rvalue::Undefined));
        assert_eq!(logic_imp.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_equiv.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(logic_lift.operands(),vec!(&Rvalue::Undefined));

        assert_eq!(int_and.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_or.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_xor.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_add.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_sub.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_mul.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_div.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_mod.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_less.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_equal.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_rs.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_ls.operands(),vec!(&Rvalue::Undefined,&Rvalue::Undefined));
        assert_eq!(int_call.operands(),vec!(&Rvalue::Undefined));

        assert_eq!(phi.operands(),Vec::<&Rvalue>::new());
        assert_eq!(nop.operands(),vec!(&Rvalue::Undefined));
    }
}

