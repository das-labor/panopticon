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
use std::error;
use std::result;
use std::sync::{
    PoisonError,
};
use std::convert::From;
use std::fmt;
use std::io;

use rustc_serialize::json::{
    EncoderError,
    DecoderError,
};

use goblin;

/// Panopticon error type
#[derive(Debug)]
pub struct Error(pub Cow<'static, str>);
/// Panopticon result type
pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for Error {
    fn description<'a>(&'a self) -> &'a str {
        match &self.0 {
            &Cow::Borrowed(s) => s,
            &Cow::Owned(ref s)=> s,
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

impl From<Cow<'static, str>> for Error {
    fn from(s: Cow<'static, str>) -> Error {
        Error(s)
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

impl From<DecoderError> for Error {
    fn from(e: DecoderError) -> Error {
        Error(Cow::Owned(format!("JSON decoder error: {}", e)))
    }
}

impl From<EncoderError> for Error {
    fn from(e: EncoderError) -> Error {
        Error(Cow::Owned(format!("JSON encoder error: {}", e)))
    }
}

impl From<goblin::error::Error> for Error {
    fn from(e: goblin::error::Error) -> Error {
        Error(Cow::Owned(format!("Goblin error: {}", e)))
    }
}
