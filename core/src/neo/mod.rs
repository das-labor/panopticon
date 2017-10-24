
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
//pub use self::function::{Function, CfgNode, Mnemonic, MnemonicIndex, MnemonicIterator, BasicBlockIterator, BasicBlock, BasicBlockIndex};
pub use self::function::{Function, CfgNode, Mnemonic, MnemonicIndex, BasicBlock, BasicBlockIndex};

use std::borrow::Cow;
pub type Str = Cow<'static,str>;

impl<'a> From<&'a super::Statement> for Statement {
    fn from(statement: &'a super::Statement) -> Self {
        function::to_statement(statement)
    }
}

impl From<super::Statement> for Statement {
    fn from(statement: super::Statement) -> Self {
        function::to_statement(&statement)
    }
}

pub use self::function::Language;