use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;
use panopticon_core::neo::{Value, Variable, Statement, Function, Result};

/// Abstract Domain. Models both under- and over-approximation.
pub trait Avalue: Debug + Default + From<Value> + Clone + PartialEq + Eq {
    /// Execute the abstract version of the operation, yielding the result.
    fn execute(&Statement<Proxy<Self>>) -> Result<Self>;
    /// Widens `self` with the argument.
    fn widen(&self, other: &Self) -> Self;
    /// Computes the lowest upper bound of self and the argument.
    fn combine(&self, &Self) -> Self;
    /// Returns true if `self` <= `other`.
    fn is_better(&self, other: &Self) -> bool;
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proxy<A: Avalue> {
    Constant(A),
    Variable(usize),
}

pub struct Approximation<A: Avalue> {
    variables: HashMap<Variable,usize>,
    values: Vec<A>,
}

impl<A: Avalue> Approximation<A> {
    pub fn new(cap: usize) -> Self {
        Approximation{
            variables: HashMap::<Variable,usize>::with_capacity(cap),
            values: vec![A::default(); cap],
        }
    }

    pub fn set(&mut self, index: usize, value: A) -> bool {
        if value.is_better(&self.values[index]) {
            self.values[index] = value;
            true
        } else {
            false
        }
    }

    pub fn get<'a>(&'a self, var: &Variable) -> Option<&'a A> {
        if let Some(idx) = self.variables.get(var) {
            Some(&self.values[*idx])
        } else {
            None
        }
    }

    pub fn insert(&mut self, idx: usize, var: Variable) {
        self.variables.insert(var,idx);
    }

    pub fn index(&self, var: &Variable) -> Option<usize> {
        self.variables.get(var).cloned()
    }
}

fn lift_value<A: Avalue>(value: &Value, approx: &mut Approximation<A>) -> Proxy<A> {
    match value {
        &Value::Constant(_) => Proxy::Constant(A::from(value.clone())),
        &Value::Variable(ref v) => Proxy::Variable(approx.index(v).unwrap()),
        &Value::Undefined => Proxy::Constant(A::from(Value::Undefined)),
    }
}

fn lift_function<A: Avalue>(func: &Function) -> Result<(Approximation<A>,Vec<Statement<Proxy<A>>>)> {
    let num_stmts = func.statements(..).count();
    let mut approx = Approximation::new(num_stmts);

    // index variables
    for (idx,stmt) in func.statements(..).enumerate() {
        match &stmt {
            &Statement::Expression{ ref result,.. } => {
                approx.insert(idx,result.clone())
            }
            _ => { /* skip */ }
        }
    }

    let mut stmts = Vec::with_capacity(num_stmts);
    for stmt in func.statements(..) {
        use panopticon_core::neo::Statement::*;
        use panopticon_core::neo::Operation::*;

        match &stmt {
            &Expression{ op: Add(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: Add(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: Subtract(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: Subtract(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: Multiply(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: Multiply(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: DivideUnsigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: DivideUnsigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: DivideSigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: DivideSigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: Modulo(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: Modulo(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: And(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: And(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: InclusiveOr(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: InclusiveOr(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: ExclusiveOr(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: ExclusiveOr(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: ShiftLeft(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: ShiftLeft(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: ShiftRightUnsigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: ShiftRightUnsigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: ShiftRightSigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: ShiftRightSigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: Equal(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: Equal(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: LessOrEqualUnsigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: LessOrEqualUnsigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: LessOrEqualSigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: LessOrEqualSigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: LessUnsigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: LessUnsigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: LessSigned(ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: LessSigned(a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: SignExtend(sz, ref a), ref result } => {
                let a = lift_value(a,&mut approx);

                stmts.push(Expression{
                    op: SignExtend(sz,a),
                    result: result.clone()
                });
            }
            &Expression{ op: ZeroExtend(sz, ref a), ref result } => {
                let a = lift_value(a,&mut approx);

                stmts.push(Expression{
                    op: ZeroExtend(sz,a),
                    result: result.clone()
                });
            }
            &Expression{ op: Move(ref a), ref result } => {
                let a = lift_value(a,&mut approx);

                stmts.push(Expression{
                    op: Move(a),
                    result: result.clone()
                });
            }
            &Expression{ op: Initialize(ref name, sz), ref result } => {
                stmts.push(Expression{
                    op: Initialize(name.clone(),sz),
                    result: result.clone()
                });
            }
            &Expression{ op: Select(sz, ref a, ref b), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);

                stmts.push(Expression{
                    op: Select(sz,a,b),
                    result: result.clone()
                });
            }
            &Expression{ op: Load(ref reg, end, sz, ref a), ref result } => {
                let a = lift_value(a,&mut approx);

                stmts.push(Expression{
                    op: Load(reg.clone(),end,sz,a),
                    result: result.clone()
                });
            }
            &Expression{ op: Phi(ref a, ref b, ref c), ref result } => {
                let a = lift_value(a,&mut approx);
                let b = lift_value(b,&mut approx);
                let c = lift_value(c,&mut approx);

                stmts.push(Expression{
                    op: Phi(a,b,c),
                    result: result.clone()
                });
            }
            &Call{ ref function } => {
                stmts.push(Call{ function: function.clone() });
            }
            &IndirectCall{ ref target } => {
                let target = lift_value(target,&mut approx);

                stmts.push(IndirectCall{ target: target });
            }
            &Return => {
                stmts.push(Return);
            }
            &Store{ ref region, endianess, bytes, ref address, ref value } => {
                let address = lift_value(address,&mut approx);
                let value = lift_value(value,&mut approx);

                stmts.push(Store{
                    region: region.clone(),
                    endianess: endianess,
                    bytes: bytes,
                    address: address,
                    value: value,
                });
            }
        }
    }

    Ok((approx,stmts))
}

pub fn approximate<A: Avalue>(func: &Function) -> Result<Approximation<A>> {
    let (mut approx, stmts) = lift_function(func)?;
    let mut fixedpoint = false;

    while !fixedpoint {
        fixedpoint = true;

        for (idx,astmt) in stmts.iter().enumerate() {
            let avalue = A::execute(astmt)?;

            fixedpoint &= !approx.set(idx,avalue);
        }
    }

    Ok(approx)
}
