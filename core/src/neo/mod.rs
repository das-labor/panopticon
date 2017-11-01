#![allow(missing_docs)]

mod value;
mod il;
mod bitcode;
mod function;
mod errors {
    error_chain!{
        foreign_links {
            Fmt(::std::fmt::Error);
            Io(::std::io::Error);
            Leb128(::leb128::read::Error);
        }
    }
}
pub use neo::errors::*;

pub use self::il::{Operation,Statement,Endianess,CallTarget};
pub use self::value::{Variable,Constant,Value};
pub use self::bitcode::{Bitcode,BitcodeIter};
pub use self::function::{Function, CfgNode, Mnemonic, MnemonicIndex, MnemonicIterator, BasicBlockIterator, BasicBlock, BasicBlockIndex};

use std::borrow::Cow;
pub type Str = Cow<'static,str>;
