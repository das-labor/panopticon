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
    pub fn match_expr(&self, pos: &'static str) -> String {
        match self {
            &Argument::Literal => format!("( ${}:expr )", pos),
            &Argument::LiteralWidth => format!("( ${}:expr ) : ${}_w:tt", pos, pos),
            &Argument::NoOffset => format!("${}:tt : ${}_w:tt", pos, pos),
            &Argument::Offset => format!("${}:tt : ${}_w:tt / ${}_o:tt", pos, pos, pos),
            &Argument::Constant => format!("[ ${}:tt ] : ${}_w:tt", pos, pos),
            &Argument::Undefined => "?".to_string(),
        }
    }

    pub fn arg_expr(&self, pos: &'static str) -> String {
        match self {
            &Argument::Literal => format!("( ${} )", pos),
            &Argument::LiteralWidth => format!("( ${} ) : ${}_w", pos, pos),
            &Argument::NoOffset => format!("${} : ${}_w", pos, pos),
            &Argument::Offset => format!("${} : ${}_w / ${}_o", pos, pos, pos),
            &Argument::Constant => format!("[ ${} ] : ${}_w", pos, pos),
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

const LVALUES: &'static [Argument] = &[
    Argument::Literal,
    Argument::LiteralWidth,
    Argument::NoOffset,
    Argument::Undefined,
];

const RVALUES: &'static [Argument] = &[
    Argument::Literal,
    Argument::LiteralWidth,
    Argument::NoOffset,
    Argument::Offset,
    Argument::Constant,
    Argument::Undefined,
];

fn write_binary_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_binop {
    "
        )
        .unwrap();

    for a in LVALUES.iter() {
        for b in RVALUES.iter() {
            for c in RVALUES.iter() {
                f.write_fmt(
                    format_args!(
                        "
    // {:?} := {:?}, {:?}
    ( $op:ident # {}, {} , {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::$op(rreil_rvalue!({}),rreil_rvalue!({})),
            assignee: rreil_lvalue!({})
        }}];
        {}
    }}}};
                ",
                a,
                b,
                c,
                a.match_expr("a"),
                b.match_expr("b"),
                c.match_expr("c"),
                b.arg_expr("b"),
                c.arg_expr("c"),
                a.arg_expr("a"),
                BOILERPLATE
                )
                    )
                    .unwrap();
            }
        }
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn write_unary_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_unop {
    "
        )
        .unwrap();

    for a in LVALUES.iter() {
        for b in RVALUES.iter() {
            f.write_fmt(
                format_args!(
                    "
    // {:?} := {:?}
    ( $op:ident # {}, {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::$op(rreil_rvalue!({})),
            assignee: rreil_lvalue!({})
        }}];
        {}
    }}}};
                ",
                a,
                b,
                a.match_expr("a"),
                b.match_expr("b"),
                b.arg_expr("b"),
                a.arg_expr("a"),
                BOILERPLATE
                )
                )
                .unwrap();
        }
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn write_memory_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_memop {
    "
        )
        .unwrap();

    for a in LVALUES.iter() {
        for b in RVALUES.iter() {
            // Little Endian Load
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( Load # $bank:ident # le # $sz:tt # {} , {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::Load(
                ::std::borrow::Cow::Borrowed(stringify!($bank)),
                $crate::Endianess::Little,
                rreil_imm!($sz),
                rreil_rvalue!({})
            ),
            assignee: rreil_lvalue!({})
        }}];
        {}
    }}}};
                ",a,b,
                a.match_expr("a"),b.match_expr("b"),
                b.arg_expr("b"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();

            // Big Endian Load
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( Load # $bank:ident # be # $sz:tt # {} , {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::Load(
                ::std::borrow::Cow::Borrowed(stringify!($bank)),
                $crate::Endianess::Big,
                rreil_imm!($sz),
                rreil_rvalue!({})
            ),
            assignee: rreil_lvalue!({})
        }}];
        {}
    }}}};
                ",a,b,
                a.match_expr("a"),b.match_expr("b"),
                b.arg_expr("b"),a.arg_expr("a"),
                BOILERPLATE)).unwrap();
          }
    }

    for val in RVALUES.iter() {
        for ptr in RVALUES.iter() {
       // Little Endian Store
            f.write_fmt(format_args!("
    // *({:?}) := {:?}
    ( Store # $bank:ident # le # $sz:tt # {} , {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::Store(
                ::std::borrow::Cow::Borrowed(stringify!($bank)),
                $crate::Endianess::Little,
                rreil_imm!($sz),
                rreil_rvalue!({}),
                rreil_rvalue!({})
            ),
            assignee: Lvalue::Undefined
        }}];
        {}
    }}}};
                ",ptr,val,
                val.match_expr("val"),ptr.match_expr("ptr"),
                ptr.arg_expr("ptr"),val.arg_expr("val"),
                BOILERPLATE)).unwrap();

            // Big Endian Store
            f.write_fmt(format_args!("
    // {:?} := {:?}
    ( Store # $bank:ident # be # $sz:tt # {} , {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::Store(
                ::std::borrow::Cow::Borrowed(stringify!($bank)),
                $crate::Endianess::Big,
                rreil_imm!($sz),
                rreil_rvalue!({}),
                rreil_rvalue!({})
            ),
            assignee: Lvalue::Undefined
        }}];
        {}
    }}}};
                ",ptr,val,
                val.match_expr("val"),ptr.match_expr("ptr"),
                ptr.arg_expr("ptr"),val.arg_expr("val"),
                BOILERPLATE)).unwrap();

        }
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn write_extraction_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_extop {
    "
        )
        .unwrap();

    for a in LVALUES.iter() {
        for b in RVALUES.iter() {
            f.write_fmt(
                format_args!(
                    "
    // {:?} := {:?}
    ( $op:ident # $sz:tt # {}, {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!({})),
            assignee: rreil_lvalue!({})
        }}];
        {}
    }}}};
                ",
                a,
                b,
                a.match_expr("a"),
                b.match_expr("b"),
                b.arg_expr("b"),
                a.arg_expr("a"),
                BOILERPLATE
                )
                )
                .unwrap();
        }
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn write_selection_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_selop {
    "
        )
        .unwrap();

    for a in LVALUES.iter() {
        for b in RVALUES.iter() {
            f.write_fmt(
                format_args!(
                    "
    // {:?} := {:?}
    ( $op:ident # $sz:tt # {}, {} ; $($cdr:tt)*) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!({}),rreil_rvalue!({})),
            assignee: rreil_lvalue!({})
        }}];
        {}
    }}}};
                ",
                a,
                b,
                a.match_expr("a"),
                b.match_expr("b"),
                a.arg_expr("a"),
                b.arg_expr("b"),
                a.arg_expr("a"),
                BOILERPLATE
                )
                )
                .unwrap();
        }
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn write_call_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_callop {
    "
        )
        .unwrap();

    for a in RVALUES.iter() {
        f.write_fmt(
            format_args!(
                "
    // call {:?}
    ( {} ; $($cdr:tt)* ) => {{{{
        let mut stmt = vec![$crate::Statement{{
            op: $crate::Operation::Call(rreil_rvalue!({})),
            assignee: $crate::Lvalue::Undefined
        }}];
        {}
    }}}};
                ",
                a,
                a.match_expr("a"),
                a.arg_expr("a"),
                BOILERPLATE
                )
            )
            .unwrap();
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn write_ret_operations(f: &mut File) {
    f.write_all(
        b"
#[macro_export]
macro_rules! rreil_retop {
    "
        )
        .unwrap();

    for a in RVALUES.iter() {
        f.write_fmt(
            format_args!(
                "
    // ret {:?}
    ( {} ; $($cdr:tt)* ) => {{{{
        let mut stmt = vec![$crate::Statement::Return{{
            stack_effect: rreil_rvalue!({}),
        }}];
        {}
    }}}};
                ",
                a,
                a.match_expr("a"),
                a.arg_expr("a"),
                BOILERPLATE
                )
            )
            .unwrap();
    }
    f.write_all(
        b"}
    "
    )
        .unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rreil.rs");
    let mut f = File::create(&dest_path).unwrap();

    write_binary_operations(&mut f);
    write_unary_operations(&mut f);
    write_memory_operations(&mut f);
    write_call_operations(&mut f);
    write_ret_operations(&mut f);
    write_extraction_operations(&mut f);
    write_selection_operations(&mut f);
}
