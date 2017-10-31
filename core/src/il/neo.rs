use std::fmt::Debug;
use uuid::Uuid;
use smallvec::SmallVec;

use il::{Value,Variable,Endianness};
use Statement as RREILStatement;
use Operation as RREILOperation;
use {Lvalue, Str, LoadStatement};

/// A RREIL operation.
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Operation<V>
    where V: Clone + PartialEq + Eq + Debug
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
    /// Initializes a global variable.
    Initialize(Str,usize),
    /// Copies only a range of bit from the operand.
    Select(usize, V, V),

    /// Reads a memory cell
    Load(Str, Endianness, usize, V),

    /// SSA Phi function
    Phi(V,V,V),
}

impl<V> Operation<V> where V: Clone + PartialEq + Eq + Debug {
    pub fn name(&self) -> &'static str {
        use self::Operation::*;
        match self {
            &Add(..) => "add",
            &Subtract(..) => "sub",
            &Multiply(..) => "mul",
            &DivideUnsigned(..) => "div",
            &DivideSigned(..) => "divs",
            &ShiftLeft(..) => "shl",
            &ShiftRightUnsigned(..) => "shr",
            &ShiftRightSigned(..) => "shrs",
            &Modulo(..) => "mod",
            &And(..) => "and",
            &InclusiveOr(..) => "or",
            &ExclusiveOr(..) => "xor",
            &Equal(..) => "eq",
            &LessOrEqualUnsigned(..) => "leq",
            &LessOrEqualSigned(..) => "leqs",
            &LessUnsigned(..) => "less",
            &LessSigned(..) => "less_signed",
            &ZeroExtend(..) => "zero_extend",
            &SignExtend(..) => "sign_extend",
            &Move(..) => "move",
            &Initialize(..) => "init",
            &Select(..) => "select",
            &Load(..) => "load",
            &Phi(..) => "phi",
        }

    }
    pub fn reads<'x>(&'x self) -> SmallVec<[&'x V; 3]> {
        use neo::Operation::*;

        let mut ret = SmallVec::new();

        match self {
            &Add(ref a, ref b) => { ret.push(a); ret.push(b); }
            &Subtract(ref a, ref b) => { ret.push(a); ret.push(b); }
            &Multiply(ref a, ref b) => { ret.push(a); ret.push(b); }
            &DivideUnsigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &DivideSigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &ShiftLeft(ref a, ref b) => { ret.push(a); ret.push(b); }
            &ShiftRightUnsigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &ShiftRightSigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &Modulo(ref a, ref b) => { ret.push(a); ret.push(b); }
            &And(ref a, ref b) => { ret.push(a); ret.push(b); }
            &InclusiveOr(ref a, ref b) => { ret.push(a); ret.push(b); }
            &ExclusiveOr(ref a, ref b) => { ret.push(a); ret.push(b); }

            &Equal(ref a, ref b) => { ret.push(a); ret.push(b); }
            &LessOrEqualUnsigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &LessOrEqualSigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &LessUnsigned(ref a, ref b) => { ret.push(a); ret.push(b); }
            &LessSigned(ref a, ref b) => { ret.push(a); ret.push(b); }

            &ZeroExtend(_, ref a) => { ret.push(a); }
            &SignExtend(_, ref a) => { ret.push(a); }
            &Move(ref a) => { ret.push(a); }
            &Initialize(_,_) => {}
            &Select(_, ref a, ref b) => { ret.push(a); ret.push(b); }

            &Load(_, _, _, ref a) => { ret.push(a); }

            &Phi(ref a, ref b, ref c) => {
                ret.push(a); ret.push(b); ret.push(c);
            }
        }

        ret
    }

    pub fn reads_mut<'x>(&'x mut self) -> SmallVec<[&'x mut V; 3]> {
        use neo::Operation::*;

        let mut ret = SmallVec::new();

        match self {
            &mut Add(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut Subtract(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut Multiply(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut DivideUnsigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut DivideSigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut ShiftLeft(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut ShiftRightUnsigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut ShiftRightSigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut Modulo(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut And(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut InclusiveOr(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut ExclusiveOr(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }

            &mut Equal(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut LessOrEqualUnsigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut LessOrEqualSigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut LessUnsigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }
            &mut LessSigned(ref mut a, ref mut b) => { ret.push(a); ret.push(b); }

            &mut ZeroExtend(_, ref mut a) => { ret.push(a); }
            &mut SignExtend(_, ref mut a) => { ret.push(a); }
            &mut Move(ref mut a) => { ret.push(a); }
            &mut Initialize(_,_) => {}
            &mut Select(_, ref mut a, ref mut b) => { ret.push(a); ret.push(b); }

            &mut Load(_, _, _, ref mut a) => { ret.push(a); }

            &mut Phi(ref mut a, ref mut b, ref mut c) => {
                ret.push(a); ret.push(b); ret.push(c);
            }
        }

        ret
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum CallTarget {
    Function(Uuid),
    External(Str),
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Statement {
    /// A single RREIL statement.
    Expression {
        /// Value that the operation result is assigned to
        result: Variable,
        /// Operation and its arguments
        op: Operation<Value>,
    },
    /// Function call
    Call {
        function: CallTarget,
    },
    IndirectCall {
        /// Call target
        target: Value,
    },
    Return,
    /// Writes a memory cell
    Store {
        region: Str,
        endianness: Endianness,
        bytes: usize,
        address: Value,
        value: Value,
    }
}

impl LoadStatement for Statement {
    fn is_load(&self) -> bool {
        use self::Statement::*;
        use self::Operation::*;
        match self {
            &Expression { op: Load(_,_,_, _), ..} => true,
            _ => false
        }
    }

    fn value(&self) -> Option<Value> {
        use self::Statement::*;
        use self::Operation::*;
        match self {
            &Expression { op: Load(_,_,_, ref value), ..} => Some(value.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary,Gen};

    impl Arbitrary for Operation<Value> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            loop {
                let op = match g.gen_range(0, 25) {
                    0 => Operation::Add(Value::arbitrary(g), Value::arbitrary(g)),
                    1 => Operation::Subtract(Value::arbitrary(g), Value::arbitrary(g)),
                    2 => Operation::Multiply(Value::arbitrary(g), Value::arbitrary(g)),
                    3 => Operation::DivideUnsigned(Value::arbitrary(g), Value::arbitrary(g)),
                    4 => Operation::DivideSigned(Value::arbitrary(g), Value::arbitrary(g)),
                    5 => Operation::ShiftLeft(Value::arbitrary(g), Value::arbitrary(g)),
                    6 => Operation::ShiftRightUnsigned(Value::arbitrary(g), Value::arbitrary(g)),
                    7 => Operation::ShiftRightSigned(Value::arbitrary(g), Value::arbitrary(g)),
                    8 => Operation::Modulo(Value::arbitrary(g), Value::arbitrary(g)),
                    9 => Operation::And(Value::arbitrary(g), Value::arbitrary(g)),
                    10 => Operation::InclusiveOr(Value::arbitrary(g), Value::arbitrary(g)),
                    11 => Operation::ExclusiveOr(Value::arbitrary(g), Value::arbitrary(g)),

                    12 => Operation::Equal(Value::arbitrary(g), Value::arbitrary(g)),
                    13 => Operation::LessOrEqualUnsigned(Value::arbitrary(g), Value::arbitrary(g)),
                    14 => Operation::LessOrEqualSigned(Value::arbitrary(g), Value::arbitrary(g)),
                    15 => Operation::LessUnsigned(Value::arbitrary(g), Value::arbitrary(g)),
                    16 => Operation::LessSigned(Value::arbitrary(g), Value::arbitrary(g)),

                    17 => Operation::ZeroExtend(g.gen(), Value::arbitrary(g)),
                    18 => Operation::SignExtend(g.gen(), Value::arbitrary(g)),

                    19 => Operation::Move(Value::arbitrary(g)),
                    20 => Operation::Initialize(g.gen_ascii_chars().take(1).collect(),g.gen()),

                    21 => Operation::Select(g.gen(), Value::arbitrary(g), Value::arbitrary(g)),

                    22 => Operation::Load(g.gen_ascii_chars().take(1).collect(), Endianness::arbitrary(g), g.gen(), Value::arbitrary(g)),

                    23 => {
                        // XXX: make sizes equal?
                        let a = Value::arbitrary(g);
                        let b = Value::arbitrary(g);
                        Operation::Phi(a,b,Value::undef())
                    }
                    24 => {
                        let a = Value::arbitrary(g);
                        let b = Value::arbitrary(g);
                        let c = Value::arbitrary(g);
                        Operation::Phi(a,b,c)
                    }

                    _ => unreachable!(),
                };

                match op {
                    Operation::Add(Value::Undefined, Value::Undefined) => {}
                    Operation::Subtract(Value::Undefined, Value::Undefined) => {}
                    Operation::Multiply(Value::Undefined, Value::Undefined) => {}
                    Operation::DivideUnsigned(Value::Undefined, Value::Undefined) => {}
                    Operation::DivideSigned(Value::Undefined, Value::Undefined) => {}
                    Operation::Modulo(Value::Undefined, Value::Undefined) => {}
                    Operation::ShiftLeft(Value::Undefined, Value::Undefined) => {}
                    Operation::ShiftRightUnsigned(Value::Undefined, Value::Undefined) => {}
                    Operation::ShiftRightSigned(Value::Undefined, Value::Undefined) => {}
                    Operation::And(Value::Undefined, Value::Undefined) => {}
                    Operation::InclusiveOr(Value::Undefined, Value::Undefined) => {}
                    Operation::ExclusiveOr(Value::Undefined, Value::Undefined) => {}
                    Operation::Equal(Value::Undefined, Value::Undefined) => {}
                    Operation::LessOrEqualUnsigned(Value::Undefined, Value::Undefined) => {}
                    Operation::LessOrEqualSigned(Value::Undefined, Value::Undefined) => {}
                    Operation::LessUnsigned(Value::Undefined, Value::Undefined) => {}
                    Operation::LessSigned(Value::Undefined, Value::Undefined) => {}
                    Operation::ZeroExtend(_, Value::Undefined) => {}
                    Operation::SignExtend(_, Value::Undefined) => {}
                    Operation::Select(_, Value::Undefined, _) => {}
                    Operation::Select(_, _, Value::Undefined) => {}
                    Operation::Phi(Value::Undefined, _, _) => {}
                    Operation::Phi(_, Value::Undefined, _) => {}
                    Operation::Phi(Value::Constant(_), _, _) => {}
                    Operation::Phi(_, Value::Constant(_), _) => {}
                    Operation::Phi(_, _, Value::Constant(_)) => {}

                    _ => { return op; }
                }
            }



            /*
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
               sz = o.bits();
               } else {
               match o {
               &mut Value::Undefined => {}
               &mut Value::Constant(Constant { ref mut bits, .. }) => *bits = sz.unwrap(),
               &mut Value::Variable(Variable { ref mut bits, .. }) => *bits = sz.unwrap(),
               }
               }
               }
               }
               Operation::Select(ref mut off, ref mut rv1, ref mut rv2) => {
               if let (Some(sz1), Some(sz2)) = (rv1.bits(), rv2.bits()) {
               if sz2 > sz1 {
               let t2 = rv1.clone();
             *rv1 = rv2.clone();
             *rv2 = t2;
             }
             }
             if let (Some(sz1), Some(sz2)) = (rv1.bits(), rv2.bits()) {
             *off = g.gen_range(0, sz1 - sz2 + 1);
             }
             }
             _ => {}
             }*/
        }
    }

    impl Arbitrary for CallTarget {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            match g.gen_range(0, 2) {
                0 => CallTarget::Function(Uuid::new_v4()),
                1 => CallTarget::External(g.gen_ascii_chars().take(1).collect()),
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for Statement {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            match g.gen_range(0, 5) {
                0 => Statement::Expression{
                    result: Variable::arbitrary(g),
                    op: Operation::<Value>::arbitrary(g),
                },
                1 => Statement::Call{ function: CallTarget::arbitrary(g) },
                2 => Statement::IndirectCall{ target: Value::arbitrary(g) },
                3 => Statement::Return,
                4 => {
                    let mut addr = Value::arbitrary(g);
                    let mut val = Value::arbitrary(g);

                    while addr == Value::Undefined && val == Value::Undefined {
                        addr = Value::arbitrary(g);
                        val = Value::arbitrary(g);
                    }

                    Statement::Store{
                        region: g.gen_ascii_chars().take(1).collect::<String>().into(),
                        endianness: Endianness::arbitrary(g),
                        bytes: g.gen_range(1,11),
                        address: addr,
                        value: val,
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

//////////////////////////////////
// conversions from standard RREIL
//////////////////////////////////

impl<'a> From<&'a RREILStatement> for Statement {
    fn from(statement: &'a RREILStatement) -> Self {
        to_rreil(statement)
    }
}

impl From<RREILStatement> for Statement {
    fn from(statement: RREILStatement) -> Self {
        to_rreil(&statement)
    }
}

fn to_rreil(stmt: &RREILStatement) -> Statement {
    match stmt {
        &RREILStatement{ op: RREILOperation::Add(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Add(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Subtract(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Subtract(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Multiply(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Multiply(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::DivideUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::DivideUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::DivideSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::DivideSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Modulo(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Modulo(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::ShiftLeft(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ShiftLeft(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::ShiftRightUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ShiftRightUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::ShiftRightSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ShiftRightSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::InclusiveOr(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::InclusiveOr(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::And(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::And(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::ExclusiveOr(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ExclusiveOr(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Equal(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Equal(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::LessOrEqualUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessOrEqualUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::LessOrEqualSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessOrEqualSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::LessUnsigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessUnsigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::LessSigned(ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::LessSigned(a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::SignExtend(sz,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::SignExtend(sz,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::ZeroExtend(sz,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::ZeroExtend(sz,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Move(ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Move(a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Initialize(ref s,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Initialize(s.clone(),a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Select(sz,ref a,ref b), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Select(sz,a.clone().into(),b.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Load(ref s,endianness,b,ref a), assignee: Lvalue::Variable{ ref name, ref subscript, size } } => {
            Statement::Expression{ op: Operation::Load(s.clone(),endianness.clone(),b,a.clone().into()), result: Variable::new(name.clone(),size,subscript.clone()).unwrap() }
        }
        &RREILStatement{ op: RREILOperation::Store(ref s,endianness,by,ref a,ref b),.. } => {
            Statement::Store{
                region: s.clone(),
                endianness: endianness.clone(),
                bytes: by,
                address: a.clone().into(),
                value: b.clone().into(),
            }
        }
        //Phi(Vec<V>),
        &RREILStatement{ op: RREILOperation::Call(ref a),.. } => {
            Statement::IndirectCall{
                target: a.clone().into(),
            }
        }
        _ => {
            error!("Conversion from RREIL not implemented for {:?}", stmt);
            unimplemented!();
        }
    }
}
