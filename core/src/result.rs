/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2016  Panopticon authors
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

//! Result type used throughout the library.
//!
//! The error type is simply a string.


use std::borrow::Cow;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::sync::PoisonError;
use serde_cbor;
use goblin;

// Error chain is effectively a broken crate until something like this lands:
// https://github.com/rust-lang-nursery/error-chain/pull/163
//error_chain!{
//        foreign_links {
//            Fmt(::std::fmt::Error);
//            Io(::std::io::Error);
//            Leb128(::leb128::read::Error);
//            Goblin(::goblin::error::Error);
//            Serde(::serde_cbor::Error);
//        }
//    }

/// Panopticon error type
#[derive(Debug)]
pub struct Error(pub Cow<'static, str>);
/// Panopticon result type
pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for Error {
    fn description<'a>(&'a self) -> &'a str {
        match &self.0 {
            &Cow::Borrowed(s) => s,
            &Cow::Owned(ref s) => s,
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error(Cow::Owned(s))
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Error {
        Error(Cow::Borrowed(s))
    }
}

impl From<Cow<'static,str>> for Error {
    fn from(s: Cow<'static, str>) -> Error {
        Error(s.into())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error(Cow::Borrowed("Lock poisoned"))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error(Cow::Owned(format!("I/O error: {:?}", e)))
    }
}

impl From<goblin::error::Error> for Error {
    fn from(e: goblin::error::Error) -> Error {
        Error(Cow::Owned(format!("Goblin error: {}", e)))
    }
}

impl From<serde_cbor::Error> for Error {
    fn from(e: serde_cbor::Error) -> Error {
        Error(Cow::Owned(format!("Serde error: {}", e)))
    }
}

impl From<::leb128::read::Error> for Error {
    fn from(e: ::leb128::read::Error) -> Error {
        Error(Cow::Owned(format!("Leb128 error: {}", e)))
    }
}
