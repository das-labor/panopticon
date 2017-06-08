/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone,Copy,Debug)]
enum Argument {
    Literal,
    LiteralWidth,
    NoOffset,
    Offset,
    Constant,
    Undefined,
}

impl Argument {
    pub fn match_expr(&self,pos: &'static str) -> String {
        match self {
            &Argument::Literal => format!("( ${}:expr )",pos),
            &Argument::LiteralWidth => format!("( ${}:expr ) : ${}_w:tt",pos,pos),
            &Argument::NoOffset => format!("${}:tt : ${}_w:tt",pos,pos),
            &Argument::Offset => format!("${}:tt : ${}_w:tt / ${}_o:tt",pos,pos,pos),
            &Argument::Constant => format!("[ ${}:tt ] : ${}_w:tt",pos,pos),
            &Argument::Undefined => "?".to_string(),
        }
    }

    pub fn arg_expr(&self,pos: &'static str) -> String {
        match self {
            &Argument::Literal => format!("( ${} )",pos),
            &Argument::LiteralWidth => format!("( ${} ) : ${}_w",pos,pos),
            &Argument::NoOffset => format!("${} : ${}_w",pos,pos),
            &Argument::Offset => format!("${} : ${}_w / ${}_o",pos,pos,pos),
            &Argument::Constant => format!("[ ${} ] : ${}_w",pos,pos),
            &Argument::Undefined => "?".to_string(),
        }
    }
}

const BOILERPLATE: &'static str = "
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
    Ok(()) => {
        let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
        match tail {
            Ok(ref mut other) => {
                stmt.extend(other.drain(..));
                Ok(stmt)
            }
            Err(e) => Err(e),
        }
    }
    Err(e) => Err(e).into(),
};

ret
";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rreil.rs");
    let mut f = File::create(&dest_path).unwrap();
    let lvalues = &[
        Argument::Literal,
        Argument::LiteralWidth,
        Argument::NoOffset,
        Argument::Undefined,
    ];
    let rvalues = &[
        Argument::Literal,
        Argument::LiteralWidth,
        Argument::NoOffset,
        Argument::Offset,
        Argument::Constant,
        Argument::Undefined,
    ];

    // binary
    f.write_all(b"
#[macro_export]
macro_rules! rreil_binop {
    ").unwrap();

    for a in lvalues.iter() {
        for b in rvalues.iter() {
            for c in rvalues.iter() {
                f.write_fmt(format_args!("
    // {:?} := {:?}, {:?}
    ( $op:ident # {}, {} , {} ; $($cdr:tt)*) => {{{{
	    let mut stmt = vec![$crate::Statement{{ op: $crate::Operation::$op(rreil_rvalue!({}),rreil_rvalue!({})), assignee: rreil_lvalue!({})}}];
        {}
    }}}};
                ",a,b,c,
                a.match_expr("a"),b.match_expr("b"),c.match_expr("c"),
                b.arg_expr("b"),c.arg_expr("c"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();
            }
        }
    }
    f.write_all(b"}
    ").unwrap();

    // unary
    f.write_all(b"
#[macro_export]
macro_rules! rreil_unop {
    ").unwrap();

    for a in lvalues.iter() {
        for b in rvalues.iter() {
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( $op:ident # {}, {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{ op: $crate::Operation::$op(rreil_rvalue!({})), assignee: rreil_lvalue!({})}}];
        {}
    }}}};
                ",a,b,
                a.match_expr("a"),b.match_expr("b"),
                b.arg_expr("b"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();
        }
    }
    f.write_all(b"}
    ").unwrap();

    // memop
    f.write_all(b"
#[macro_export]
macro_rules! rreil_memop {
    ").unwrap();

    for a in lvalues.iter() {
        for b in rvalues.iter() {
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( $op:ident # $bank:ident # {} , {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!({})), assignee: rreil_lvalue!({})}}];
        {}
    }}}};
                ",a,b,
                a.match_expr("a"),b.match_expr("b"),
                b.arg_expr("b"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();
        }
    }
    f.write_all(b"}
    ").unwrap();

    // extop
    f.write_all(b"
#[macro_export]
macro_rules! rreil_extop {
    ").unwrap();

    for a in lvalues.iter() {
        for b in rvalues.iter() {
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( $op:ident # $sz:tt # {}, {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!({})), assignee: rreil_lvalue!({})}}];
        {}
    }}}};
                ",a,b,
                a.match_expr("a"),b.match_expr("b"),
                b.arg_expr("b"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();
        }
    }
    f.write_all(b"}
    ").unwrap();

    // selop
    f.write_all(b"
#[macro_export]
macro_rules! rreil_selop {
    ").unwrap();

    for a in lvalues.iter() {
        for b in rvalues.iter() {
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( $op:ident # $sz:tt # {}, {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!({}),rreil_rvalue!({})), assignee: rreil_lvalue!({})}}];
        {}
    }}}};
                ",a,b,
                a.match_expr("a"),b.match_expr("b"),
                a.arg_expr("a"),b.arg_expr("b"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();
        }
    }
    f.write_all(b"}
    ").unwrap();
}

