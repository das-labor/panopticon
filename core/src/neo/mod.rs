
mod value;
mod il;
mod bitcode;
mod function;
pub use result::*;

pub use self::il::{Operation,Statement,Endianess,CallTarget};
pub use self::value::{Variable,Constant,Value};
pub use self::bitcode::{Bitcode,BitcodeIter};
//pub use self::function::{Function, CfgNode, Mnemonic, MnemonicIndex, MnemonicIterator, BasicBlockIterator, BasicBlock, BasicBlockIndex};
pub use self::function::{Function, CfgNode, Mnemonic, MnemonicIndex, BasicBlock, BasicBlockIndex};

use std::borrow::Cow;
pub type Str = Cow<'static,str>;

pub use self::function::{Language, StatementIterator, RREIL};