use std::result;
use std::fmt::{Formatter, Display, Error};

/// Endianess of a memory operation.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub enum Endianness {
    /// Least significant byte first
    Little,
    /// Most significant byte first
    Big,
}

impl Display for Endianness {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), Error> {
        match self {
            &Endianness::Little => f.write_str("le"),
            &Endianness::Big => f.write_str("be"),
        }
    }
}
