use std::borrow::Cow;
use std::error;
use std::result;
use std::sync::{
    PoisonError,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};
use std::convert::From;
use std::fmt;
use std::io;

use rustc_serialize::json::{
    EncoderError,
    DecoderError,
};

#[derive(Debug)]
pub struct Error(pub Cow<'static,str>);
pub type Result<T> = result::Result<T,Error>;

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

impl From<Cow<'static,str>> for Error {
    fn from(s: Cow<'static,str>) -> Error {
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
        Error(Cow::Owned(format!("I/O error: {:?}",e)))
    }
}

impl From<DecoderError> for Error {
    fn from(e: DecoderError) -> Error {
        Error(Cow::Owned(format!("JSON decoder error: {}",e)))
    }
}

impl From<EncoderError> for Error {
    fn from(e: EncoderError) -> Error {
        Error(Cow::Owned(format!("JSON encoder error: {}",e)))
    }
}
