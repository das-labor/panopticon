use neo::{Result,Str};
use {Rvalue,Lvalue};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Variable {
    pub name: Str,
    pub bits: usize,
    pub subscript: Option<usize>,
}

impl Variable {
    pub fn new<N: Into<Str> + Sized>(name: N, bits: usize, subscript: Option<usize>) -> Result<Variable> {
        let name: Str = name.into();

        if bits == 0 { return Err("Variable can't have size 0".into()); }
        if name == "" { return Err("Variable can't have an empty name".into()); }

        Ok(Variable{
            name: name,
            subscript: subscript,
            bits: bits
        })
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Constant {
    pub value: u64,
    pub bits: usize,
}

impl Constant {
    pub fn new(value: u64, bits: usize) -> Result<Constant> {
        if bits == 0 { return Err("Variable can't have size 0".into()); }

        Ok(Constant{
            value: value,
            bits: bits
        })
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Value {
    Undefined,
    Variable(Variable),
    Constant(Constant),
}

impl Value {
    pub fn val(val: u64, bits: usize) -> Result<Value> {
        Ok(Value::Constant(Constant::new(val,bits)?))
    }

    pub fn var<N: Into<Str> + Sized, S: Into<Option<usize>> + Sized>(name: N, bits: usize, subscript: S) -> Result<Value> {
        Ok(Value::Variable(Variable::new(name,bits,subscript.into())?))
    }

    pub fn undef() -> Value {
        Value::Undefined
    }

    pub fn bits(&self) -> Option<usize> {
        match self {
            &Value::Variable(Variable{ bits,.. }) => Some(bits),
            &Value::Constant(Constant{ bits,.. }) => Some(bits),
            &Value::Undefined => None,
        }
    }
}

impl From<Variable> for Value {
    fn from(v: Variable) -> Value {
        Value::Variable(v)
    }
}

impl From<Constant> for Value {
    fn from(v: Constant) -> Value {
        Value::Constant(v)
    }
}

impl From<Rvalue> for Value {
    fn from(v: Rvalue) -> Value {
        match v {
            Rvalue::Undefined => Value::undef(),
            Rvalue::Variable{ name, subscript, size,.. } => Value::var(name,size,subscript).unwrap(),
            Rvalue::Constant{ value, size } => Value::val(value,size).unwrap(),
        }
    }
}

impl From<Lvalue> for Value {
    fn from(v: Lvalue) -> Value {
        match v {
            Lvalue::Undefined => Value::undef(),
            Lvalue::Variable{ name, subscript, size,.. } => Value::var(name,size,subscript).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use quickcheck::{Arbitrary,Gen};

     impl Arbitrary for Variable {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Variable {
                name: Cow::Owned(g.gen_ascii_chars().take(2).collect()),
                bits: 1 << g.gen_range(0, 11),
                subscript: Some(g.gen_range(0, 5)),
            }
        }
    }

    impl Arbitrary for Constant {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Constant{
                value: g.gen(),
                bits: 1 << g.gen_range(0, 11),
            }
        }
    }

    impl Arbitrary for Value {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            match g.gen_range(0, 2) {
                0 => Value::Undefined,
                1 => Value::Variable(Variable::arbitrary(g)),
                2 => Value::Constant(Constant::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }
}
