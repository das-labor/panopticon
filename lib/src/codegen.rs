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

    pub fn and_b(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::LogicAnd(op1,op2),a) }
    pub fn or_b(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::LogicOr(op1,op2),a) }
    pub fn lift_b(&mut self,a: Lvalue, op: Rvalue) -> Lvalue { self.named(Operation::LogicLift(op),a) }
    pub fn not_b(&mut self,a: Lvalue, op: Rvalue) -> Lvalue { self.named(Operation::LogicNegation(op),a) }
    pub fn assign(&mut self,a: Lvalue, op: Rvalue) -> Lvalue { self.named(Operation::Nop(op),a) }
    pub fn and_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntAnd(op1,op2),a) }
    pub fn or_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntInclusiveOr(op1,op2),a) }
    pub fn xor_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntExclusiveOr(op1,op2),a) }
    pub fn add_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntAdd(op1,op2),a) }
    pub fn sub_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntSubtract(op1,op2),a) }
    pub fn mul_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntMultiply(op1,op2),a) }
    pub fn div_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntDivide(op1,op2),a) }
    pub fn mod_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntModulo(op1,op2),a) }
    pub fn equal_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntEqual(op1,op2),a) }
    pub fn less_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntLess(op1,op2),a) }
    pub fn call_i(&mut self,a: Lvalue, op: Rvalue) -> Lvalue { self.named(Operation::IntCall(op),a) }
    pub fn rshift_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntRightShift(op1,op2),a) }
    pub fn lshift_i(&mut self,a: Lvalue, op1: Rvalue, op2: Rvalue) -> Lvalue { self.named(Operation::IntLeftShift(op1,op2),a) }

    fn named(&mut self,op: Operation, assign: Lvalue) -> Lvalue {
        let mut ret = Instr{ op: op, assignee: assign.clone() };

        fn sanity_check(v: &Rvalue) -> bool {
            match v {
                &Rvalue::Constant(_) => true,
                &Rvalue::Undefined => true,
                &Rvalue::Variable{ width: ref w, name: ref n, subscript: ref s} => *w > 0 && s.is_none() && n.len() > 0,
                &Rvalue::Memory{ offset: ref o, bytes: ref b, endianess: ref e, name: ref n} => sanity_check(o) && *b > 0 && n.len() > 0,
            }
        };

        assert!(ret.operands().iter().cloned().all(sanity_check) && sanity_check(&Rvalue::from_lvalue(&assign)));

        self.instructions.push(ret);
        assign
    }

    pub fn temporary() -> Lvalue {
        let t = unsafe {
            temporary_variable_counter += 1;
            temporary_variable_counter
        };

        Lvalue::Variable{
            name: format!("__internal_tmp{}",t),
            width: 64,
            subscript: None
        }
    }

    pub fn anonymous(&mut self,op: Operation) -> Lvalue {
        self.named(op,Self::temporary())
    }
}
/*
#![feature(trace_macros)]

trace_macros!(true);

macro_rules! pil_expr {
    ($e:expr)                         => { Rvalue::Constant($e) };
    ($bank:ident[$off:expr ; $len:expr])  => { Rvalue::Memory(stringify!($bank),pil_expr!($off),$len) };
    ($id:ident:$l:expr)                         => { Rvalue::Variable(stringify!($id),$l) }
}

macro_rules! pil_stmt {
    ($r:ident,add $a:expr, $b:expr)   => { cg.add_i(pil_expr!($a),pil_expr!($b)) };
    ($r:ident,add $a:expr, $bi:ident:$bl:expr)   => { cg.add_i(pil_expr!($a),pil_expr!($bi:$bl)) };
    ($r:ident,($a:expr) + $b:ident)  => { cg.add_i($r,Rvalue::Constant($a),$b) };
    ($r:ident,$a:ident + ($b:expr))  => { cg.add_i($r,pil_expr!($a),pil_expr!($b)) };
}

macro_rules! pil {
    ($cg:ident,$r:ident <- add $a:tt) => {{
        let mut cg = $cg;
        pil_stmt!($r,$rhs);
        cg
    }};
}

fn la() {
    pil_expr!(a:33);
    pil_expr!(33);
    //pil_expr!(ram[33;2]);
   pil_stmt!(a,add 1, a:44);
   pil!(cg,
         a <- add 1, a:44
    );
}*/
