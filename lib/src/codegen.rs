/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016  Kai Michaelis
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

#![macro_use]

use std::cmp::max;

use Rvalue;
use Lvalue;
use Statement;
use Operation;
use Architecture;

pub struct CodeGen<A: Architecture> {
    pub statements: Vec<Statement>,
    pub configuration: A::Configuration,
}

impl<C: Architecture> CodeGen<C> {
    pub fn new(cfg: &C::Configuration) -> CodeGen<C> {
        CodeGen{
            statements: Vec::new(),
            configuration: cfg.clone(),
        }
    }

    pub fn push(&mut self, stmt: Statement) {
        // check that argument sizes match
        let typecheck_binop = |a: &Rvalue,b: &Rvalue,assignee: &Lvalue| -> () {
            assert!(a.size() == None || b.size() == None || a.size() == b.size(),"Argument sizes mismatch");
            assert!(assignee.size() == None || Some(max(a.size().unwrap_or(0),b.size().unwrap_or(0))) == assignee.size(),"Operation result and assingnee sizes mismatch");
        };
        let typecheck_cmpop = |a: &Rvalue,b: &Rvalue,assignee: &Lvalue| -> () {
            assert!(a.size() == None || b.size() == None || a.size() == b.size(),"Argument sizes mismatch");
            assert!(assignee.size() == None || assignee.size() == Some(1),"Compare operation assingnee not a flag");
        };
        let typecheck_unop = |a: &Rvalue,sz: Option<usize>,assignee: &Lvalue| -> () {
            if sz.is_none() {
                // zext?
                assert!(a.size() == None || assignee.size() == None || assignee.size() <= a.size(),"Operation result and assingnee sizes mismatch");
            } else {
                assert!(a.size() == None || assignee.size() == None || assignee.size() == sz,"Operation result and assingnee sizes mismatch");
            }
        };

        match &stmt {
            &Statement{ op: Operation::Add(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::Subtract(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::Multiply(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::DivideUnsigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::DivideSigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ShiftLeft(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ShiftRightUnsigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ShiftRightSigned(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::Modulo(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::And(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::ExclusiveOr(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),
            &Statement{ op: Operation::InclusiveOr(ref a,ref b), ref assignee } => typecheck_binop(a,b,assignee),

            &Statement{ op: Operation::Equal(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessOrEqualUnsigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessOrEqualSigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessUnsigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),
            &Statement{ op: Operation::LessSigned(ref a,ref b), ref assignee } => typecheck_cmpop(a,b,assignee),

            &Statement{ op: Operation::SignExtend(ref a,ref b), ref assignee } => typecheck_unop(b,Some(*a),assignee),
            &Statement{ op: Operation::ZeroExtend(ref a,ref b), ref assignee } => typecheck_unop(b,Some(*a),assignee),
            &Statement{ op: Operation::Move(ref a), ref assignee } => typecheck_unop(a,None,assignee),

            &Statement{ op: Operation::Call(_), ref assignee } =>
                assert!(assignee == &Lvalue::Undefined,"Call operation can only be assigned to Undefined"),
            &Statement{ op: Operation::Load(_,_), ref assignee } =>
                assert!(assignee.size().is_some(),"Memory operation with undefined size"),
            &Statement{ op: Operation::Store(_,_), ref assignee } =>
                assert!(assignee.size().is_some(),"Memory operation with undefined size"),

            &Statement{ op: Operation::Phi(ref vec), ref assignee } =>
                assert!(vec.iter().all(|rv| rv.size() == assignee.size()) && assignee.size() != None,"Phi arguments must have equal sizes and can't be Undefined"),
        }

        assert!(stmt.op.operands().iter().all(|rv| rv.size() != Some(0)) && stmt.assignee.size() != Some(0),"Operation argument and/or assignee has size 0");

        self.statements.push(stmt);
    }
}

macro_rules! rreil {
    ( $cg:ident : ) => {};
    ( $cg:ident : add $($cdr:tt)* ) => { rreil_binop!($cg : Add # $($cdr)*); };
    ( $cg:ident : sub $($cdr:tt)* ) => { rreil_binop!($cg : Subtract # $($cdr)*); };
    ( $cg:ident : mul $($cdr:tt)* ) => { rreil_binop!($cg : Multiply # $($cdr)*); };
    ( $cg:ident : div $($cdr:tt)* ) => { rreil_binop!($cg : DivideUnsigned # $($cdr)*); };
    ( $cg:ident : divs $($cdr:tt)* ) => { rreil_binop!($cg : DivideSigned # $($cdr)*); };
    ( $cg:ident : shl $($cdr:tt)* ) => { rreil_binop!($cg : ShiftLeft # $($cdr)*); };
    ( $cg:ident : shr $($cdr:tt)* ) => { rreil_binop!($cg : ShiftRightUnsigned # $($cdr)*); };
    ( $cg:ident : shrs $($cdr:tt)* ) => { rreil_binop!($cg : ShiftRightSigned # $($cdr)*); };
    ( $cg:ident : mod $($cdr:tt)* ) => { rreil_binop!($cg : Modulo # $($cdr)*); };
    ( $cg:ident : and $($cdr:tt)* ) => { rreil_binop!($cg : And # $($cdr)*); };
    ( $cg:ident : xor $($cdr:tt)* ) => { rreil_binop!($cg : ExclusiveOr # $($cdr)*); };
    ( $cg:ident : or $($cdr:tt)* ) => { rreil_binop!($cg : InclusiveOr # $($cdr)*); };

    ( $cg:ident : cmpeq $($cdr:tt)* ) => { rreil_binop!($cg : Equal # $($cdr)*); };
    ( $cg:ident : cmpleu $($cdr:tt)* ) => { rreil_binop!($cg : LessOrEqualUnsigned # $($cdr)*); };
    ( $cg:ident : cmples $($cdr:tt)* ) => { rreil_binop!($cg : LessOrEqualSigned # $($cdr)*); };
    ( $cg:ident : cmpltu $($cdr:tt)* ) => { rreil_binop!($cg : LessUnsigned # $($cdr)*); };
    ( $cg:ident : cmplts $($cdr:tt)* ) => { rreil_binop!($cg : LessSigned # $($cdr)*); };

    ( $cg:ident : sext / $sz:tt $($cdr:tt)* ) => { rreil_extop!($cg : SignExtend # $sz # $($cdr)*); };
    ( $cg:ident : zext / $sz:tt $($cdr:tt)* ) => { rreil_extop!($cg : ZeroExtend # $sz # $($cdr)*); };
    ( $cg:ident : mov $($cdr:tt)* ) => { rreil_unop!($cg : Move # $($cdr)*); };
    ( $cg:ident : call $($cdr:tt)* ) => { rreil_unop!($cg : Call # $($cdr)*); };

    ( $cg:ident : load / $r:ident   $($cdr:tt)* ) => { rreil_memop!($cg : Load # $r # $($cdr)*); };
    ( $cg:ident : store / $r:ident $($cdr:tt)* ) => { rreil_memop!($cg : Store # $r # $($cdr)*); };
}

include!("rreil.rs");

macro_rules! rreil_lvalue {
    (?) =>
        { $crate::Lvalue::Undefined };
    ( ( $a:expr ) ) =>
        { ($a).clone().into() };
    ($a:ident : $a_w:tt / $a_o:tt) => {
        $crate::Lvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            offset: rreil_imm!($a_o),
            size: rreil_imm!($a_w) }
    };
    ($a:ident : $a_w:tt) => {
        $crate::Lvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            offset: 0,
            size: rreil_imm!($a_w)
        }
    };
}

macro_rules! rreil_rvalue {
    (?) => { $crate::Rvalue::Undefined };
    ( ( $a:expr ) ) => { ($a).clone().into() };
    ( [ $a:tt ] : $a_w:tt ) => {
        $crate::Rvalue::Constant{
            value: rreil_imm!($a) as u64,
            size: rreil_imm!($a_w)
        }
    };
    ($a:ident : $a_w:tt / $a_o:tt) => {
        $crate::Rvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            offset: rreil_imm!($a_o),
            size: rreil_imm!($a_w)
        }
    };
    ($a:ident : $a_w:tt) => {
        $crate::Rvalue::Variable{
            name: ::std::borrow::Cow::Borrowed(stringify!($a)),
            subscript: None,
            offset: 0,
            size: rreil_imm!($a_w)
        }
    };
}

macro_rules! rreil_imm {
    ($x:expr) => ($x as usize);
}

#[cfg(test)]
mod tests {
    use super::*;
    use {
        Rvalue,
        Lvalue,
        Architecture,
        LayerIter,
        Result,
        Disassembler,
    };
    use std::rc::Rc;
    use std::borrow::Cow;

    #[derive(Clone)]
    enum TestArchShort {}
    impl Architecture for TestArchShort {
        type Token = u8;
        type Configuration = ();

        fn prepare(_: LayerIter,_: &Self::Configuration) -> Result<Vec<(&'static str,u64,&'static str)>> {
            unimplemented!()
        }

        fn disassembler(_: &Self::Configuration) -> Rc<Disassembler<Self>> {
            unimplemented!()
        }
    }

    #[test]
    fn rreil_macro() {
        let mut cg = CodeGen::<TestArchShort>::new(&());
        let t0 = Lvalue::Variable{ name: Cow::Borrowed("t0"), subscript: None, offset: 0, size: 12 };
        let eax = Rvalue::Variable{ name: Cow::Borrowed("eax"), subscript: None, offset: 0, size: 12 };
        let val = Rvalue::Constant{ value: 1223, size: 12 };

        rreil!{
            cg:
            add (t0) , (val), (eax);
            and t0 : 32 , [ 2147483648 ]: 32, eax : 32;
            and t1 : 32 , [2147483648] : 32, ebx : 32;
            sub t2 : 32 / 32 , ebx : 32 , eax : 32;
            and t3 : 32 , [2147483648]:32, t2 : 32/32;
            shr SF : 8 , [31] : 8 , t3 : 8/24;
            xor t4 : 32 , t1 : 32 , t0 : 32;
            xor t5 : 32 , t3 : 32 , t0 : 32;
            and t6 : 32 , t5 : 32 , t4 : 32;
            shr OF : 8 , [31] : 8 , t6 : 8/24;
            and t7 : 64 , [4294967296] : 64, t2 : 64;
            shr CF : 8 , [32] : 8 , t7 : 8;
            and t8 : 32 , [4294967295] : 32, t2 : 32/32;
            xor t9 : 8 , OF : 8 , SF : 8;
        }

        rreil!{
            cg:
            sub t0:32, eax:32, ebx:32;
            cmpltu CF:1, eax:32, ebx:32;
            cmpleu CForZF:1, eax:32, ebx:32;
            cmplts SFxorOF:1, eax:32, ebx:32;
            cmples SFxorOForZF:1, eax:32, ebx:32;
            cmpeq  ZF:1, eax:32, ebx:32;
            cmplts SF:1, t0:32, [0]:32;
            xor OF:1, SFxorOF:1, SF:1;
        }

        rreil!{
            cg:
            sub rax:32, rax:32, [1]:32;
            mov rax:32/32, [0]:32;
        }

        rreil!{
            cg:
            store/ram rax:32, [0]:32;
            load/ram rax:32, [0]:32;
        }

        rreil!{
            cg:
            sext/32 rax:32, ax:16;
            zext/32 rax:32, ax:16;
            mov rax:32, tbx:32;
        }
    }
}

