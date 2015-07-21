use instr::{Operation,Instr};
use value::{Rvalue,Lvalue,ToRvalue};

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

    pub fn and_b<A: ToRvalue, B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::LogicAnd(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn or_b<A: ToRvalue, B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::LogicInclusiveOr(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn xor_b<A: ToRvalue, B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::LogicExclusiveOr(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn lift_b<A: ToRvalue>(&mut self,a: &Lvalue, op: &A) {
        self.named(Operation::LogicLift(op.to_rv()),a.clone());
    }

    pub fn not_b<A: ToRvalue>(&mut self,a: &Lvalue, op1: &A) {
        self.named(Operation::LogicNegation(op1.to_rv()),a.clone());
    }

    pub fn assign<A: ToRvalue>(&mut self,a: &Lvalue, op: &A) {
        self.named(Operation::Nop(op.to_rv()),a.clone());
    }

    pub fn and_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntAnd(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn or_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntInclusiveOr(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn xor_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntExclusiveOr(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn add_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntAdd(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn sub_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntSubtract(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn mul_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntMultiply(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn div_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntDivide(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn mod_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntModulo(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn equal_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntEqual(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn less_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntLess(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn call_i<A: ToRvalue>(&mut self,a: &Lvalue, op: &A) {
        self.named(Operation::IntCall(op.to_rv()),a.clone());
    }

    pub fn rshift_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntRightShift(op1.to_rv(),op2.to_rv()),a.clone());
    }

    pub fn lshift_i<A: ToRvalue,B: ToRvalue>(&mut self,a: &Lvalue, op1: &A, op2: &B) {
        self.named(Operation::IntLeftShift(op1.to_rv(),op2.to_rv()),a.clone());
    }

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
