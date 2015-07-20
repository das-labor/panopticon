use instr::{Operation,Instr};
use value::{Rvalue,Lvalue};

static mut temporary_variable_counter: usize = 0;

pub struct CodeGen {
    pub instructions: Vec<Instr>,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen{
            instructions: Vec::new(),
        }
    }

    pub fn and_b(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::LogicAnd(op1,op2),a) }
    pub fn or_b(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::LogicInclusiveOr(op1,op2),a) }
    pub fn xor_b(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::LogicExlusiveOr(op1,op2),a) }
    pub fn lift_b(&mut self,a: Lvalue, op: Rvalue) { self.named(Operation::LogicLift(op),a) }
    pub fn not_b(&mut self,a: Lvalue, op: Rvalue) { self.named(Operation::LogicNegation(op),a) }

    pub fn assign(&mut self,a: Lvalue, op: Rvalue) { self.named(Operation::Nop(op),a) }
    pub fn and_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntAnd(op1,op2),a) }
    pub fn or_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntInclusiveOr(op1,op2),a) }
    pub fn xor_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntExclusiveOr(op1,op2),a) }
    pub fn add_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntAdd(op1,op2),a) }
    pub fn sub_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntSubtract(op1,op2),a) }
    pub fn mul_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntMultiply(op1,op2),a) }
    pub fn div_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntDivide(op1,op2),a) }
    pub fn mod_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntModulo(op1,op2),a) }
    pub fn equal_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntEqual(op1,op2),a) }
    pub fn less_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntLess(op1,op2),a) }
    pub fn call_i(&mut self,a: Lvalue, op: Rvalue) { self.named(Operation::IntCall(op),a) }
    pub fn rshift_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntRightShift(op1,op2),a) }
    pub fn lshift_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) { self.named(Operation::IntLeftShift(op1,op2),a) }

    fn named(&mut self,op: Operation, assign: Lvalue) -> Lvalue {
        let ret = Instr{ op: op, assignee: assign.clone() };

        fn sanity_check(v: &Rvalue) -> bool {
            match v {
                &Rvalue::Constant(_) => true,
                &Rvalue::Undefined => true,
                &Rvalue::Variable{ width: ref w, name: ref n, subscript: ref s} => *w > 0 && s.is_none() && n.len() > 0,
                &Rvalue::Memory{ offset: ref o, bytes: ref b, endianess: _, name: ref n} => sanity_check(o) && *b > 0 && n.len() > 0,
            }
        };

        assert!(ret.operands().iter().cloned().all(sanity_check) && sanity_check(&Rvalue::from_lvalue(&assign)));

        self.instructions.push(ret);
        assign
    }
}
