use std::fmt::Debug;
use uuid::Uuid;

use neo::value::{Value,Variable};
use neo::Str;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Endianess {
    Little,
    Big,
}

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
    Load(Str,Endianess,usize,V),

    /// SSA Phi function
    Phi(V,V,V),
}

impl<V> Operation<V> where V: Clone + PartialEq + Eq + Debug {
    pub fn reads<'x>(&'x self) -> Vec<&'x V> {
        match self {
            &Operation::Add(ref a, ref b) => vec![a,b],
            &Operation::Subtract(ref a, ref b) => vec![a,b],
            &Operation::Multiply(ref a, ref b) => vec![a,b],
            &Operation::DivideUnsigned(ref a, ref b) => vec![a,b],
            &Operation::DivideSigned(ref a, ref b) => vec![a,b],
            &Operation::ShiftLeft(ref a, ref b) => vec![a,b],
            &Operation::ShiftRightUnsigned(ref a, ref b) => vec![a,b],
            &Operation::ShiftRightSigned(ref a, ref b) => vec![a,b],
            &Operation::Modulo(ref a, ref b) => vec![a,b],
            &Operation::And(ref a, ref b) => vec![a,b],
            &Operation::InclusiveOr(ref a, ref b) => vec![a,b],
            &Operation::ExclusiveOr(ref a, ref b) => vec![a,b],

            &Operation::Equal(ref a, ref b) => vec![a,b],
            &Operation::LessOrEqualUnsigned(ref a, ref b) => vec![a,b],
            &Operation::LessOrEqualSigned(ref a, ref b) => vec![a,b],
            &Operation::LessUnsigned(ref a, ref b) => vec![a,b],
            &Operation::LessSigned(ref a, ref b) => vec![a,b],

            &Operation::ZeroExtend(_, ref a) => vec![a],
            &Operation::SignExtend(_, ref a) => vec![a],
            &Operation::Move(ref a) => vec![a],
            &Operation::Initialize(_,_) => vec![],
            &Operation::Select(_, ref a, ref b) => vec![a,b],

            &Operation::Load(_, _, _, ref a) => vec![a],

            &Operation::Phi(ref a, ref b, ref c) => vec![a, b, c],
        }
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
    Expression{
        /// Value that the operation result is assigned to
        result: Variable,
        /// Operation and its arguments
        op: Operation<Value>,
    },
    /// Function call
    Call{
        function: CallTarget,
    },
    IndirectCall{
        /// Call target
        target: Value,
    },
    Return,
    /// Writes a memory cell
    Store{
        region: Str,
        endianess: Endianess,
        bytes: usize,
        address: Value,
        value: Value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary,Gen};

    impl Arbitrary for Endianess {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            match g.gen_range(0, 1) {
                0 => Endianess::Little,
                1 => Endianess::Big,
                _ => unreachable!(),
            }
        }
    }

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

                    22 => Operation::Load(g.gen_ascii_chars().take(1).collect(), Endianess::arbitrary(g), g.gen(), Value::arbitrary(g)),

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
                        endianess: Endianess::arbitrary(g),
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
