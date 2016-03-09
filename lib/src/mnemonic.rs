/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
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

use std::str::{Chars,FromStr};
use std::ops::Range;
use std::borrow::Cow;

use Rvalue;
use Statement;
use Result;

#[derive(Debug,Clone,PartialEq,Eq,RustcEncodable,RustcDecodable)]
pub struct Bound {
    pub start: u64,
    pub end: u64
}

impl Bound {
    pub fn new(a: u64, b: u64) -> Bound {
        Bound{ start: a, end: b }
    }
}

#[derive(Clone,Debug,PartialEq,Eq,RustcEncodable,RustcDecodable)]
pub enum MnemonicFormatToken {
    Literal(char),
    Variable{ has_sign: bool },
    Pointer{ is_code: bool, bank: String },
}

/// format := '{' type '}'
/// type := 'u' | # unsigned
///         's' | # signed
///         'p' ':' bank |  # data pointer
///         'c' ':' bank |  # code pointer
impl MnemonicFormatToken {
    fn parse_bank<'a>(i: Chars<'a>) -> Result<(String,Chars<'a>)> {
        let p = i.clone().position(|x| x == '}');

        if p.is_none() {
            Err("Mnemonic format string parse error. Bank name is invalid.".into())
        } else {
            let b = i.clone().take(p.unwrap()).collect::<String>();
            let mut j = i.clone();

            match j.nth(p.unwrap()) {
                    Some('}') => Ok((b,j)),
                    _ => Err("Mnemonic format string parse error. Expecting '}'.".into()),
                }
        }
    }

    pub fn parse(mut j: Chars) -> Result<Vec<MnemonicFormatToken>> {
        let mut ret = vec![];

        loop {
            match j.next() {
                None => break,
                Some('{') => {
                    match j.next() {
                        Some('{') => ret.push(MnemonicFormatToken::Literal('{')),
                        Some('u') => {
                            ret.push(MnemonicFormatToken::Variable{ has_sign: false });
                        },
                        Some('s') => {
                            ret.push(MnemonicFormatToken::Variable{ has_sign: true });
                        },
                        Some('p') => {
                            let (bank,k) = try!(Self::parse_bank(j));
                            ret.push(MnemonicFormatToken::Pointer{ is_code: false, bank: bank });
                            j = k;
                        }
                        Some('c') => {
                            let (bank,k) = try!(Self::parse_bank(j));
                            ret.push(MnemonicFormatToken::Pointer{ is_code: true, bank: bank });
                            j = k;
                        }
                        _ => return Err("Mnemonic format string parse error. Unknown format identifier.".into())
                    }
                }
                Some(a) => ret.push(MnemonicFormatToken::Literal(a)),
            }
        }

        Ok(ret)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub struct Mnemonic {
    pub area: Bound,
    pub opcode: String,
    pub operands: Vec<Rvalue>,
    pub instructions: Vec<Statement>,
    pub format_string: Vec<MnemonicFormatToken>,
}

impl Mnemonic {
    pub fn new<'a,I1,I2> (a: Range<u64>, code: String, fmt: String, ops: I1, instr: I2) -> Result<Mnemonic>
        where I1: Iterator<Item=&'a Rvalue>,I2: Iterator<Item=&'a Statement> {
        Ok(Mnemonic{
            area: Bound::new(a.start,a.end),
            opcode: code,
            operands: ops.cloned().collect(),
            instructions: instr.cloned().collect(),
            format_string: try!(MnemonicFormatToken::parse(fmt.chars())),
        })
    }

    #[cfg(test)]
    pub fn dummy(a: Range<u64>) -> Mnemonic {
        Mnemonic{
            area: Bound::new(a.start,a.end),
            opcode: "dummy".to_string(),
            operands: vec!(),
            instructions: vec!(),
            format_string: vec!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use {
        Rvalue,
        Lvalue,
        Operation,
        Statement,
    };

    #[test]
    fn parse_format_string() {
        let fmt = "doe{69::eax3}io{8:-}øiq{88:-:sss}   {9::} sasq {32:}".to_string();
        let val = MnemonicFormatToken::parse(fmt.chars());

        assert_eq!(Some(vec!(
                MnemonicFormatToken::Literal('d'),
                MnemonicFormatToken::Literal('o'),
                MnemonicFormatToken::Literal('e'),
                MnemonicFormatToken::Variable{ has_sign: false },
                MnemonicFormatToken::Literal('i'),
                MnemonicFormatToken::Literal('o'),
                MnemonicFormatToken::Variable{ has_sign: true },
                MnemonicFormatToken::Literal('ø'),
                MnemonicFormatToken::Literal('i'),
                MnemonicFormatToken::Literal('q'),
                MnemonicFormatToken::Variable{ has_sign: true },
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Variable{ has_sign: false },
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Literal('s'),
                MnemonicFormatToken::Literal('a'),
                MnemonicFormatToken::Literal('s'),
                MnemonicFormatToken::Literal('q'),
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Variable{ has_sign: false },
            )),val.ok());

        assert!(MnemonicFormatToken::parse("{69:+}".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{-69:+}".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69::".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{}".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69:".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69:-".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69::".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69:-:".to_string().chars()).is_err());
        assert!(MnemonicFormatToken::parse("{69::ddd".to_string().chars()).is_err());
        assert_eq!(MnemonicFormatToken::parse("{69}".to_string().chars()).ok(),Some(vec!(MnemonicFormatToken::Variable{ has_sign: false })));
    }

    #[test]
    fn construct() {
        let ops1 = vec!(Rvalue::new_u8(1),Rvalue::Variable{ name: Cow::Borrowed("a"), size: 3, offset: 0, subscript: None });
        let i1 = vec!(
            Statement{ op: Operation::Add(Rvalue::new_u8(1),Rvalue::new_u8(2)), assignee: Lvalue::Variable{ name: Cow::Borrowed("a"), size: 8, offset: 0, subscript: Some(2) }},
            Statement{ op: Operation::Add(Rvalue::new_u8(4),Rvalue::new_u8(2)), assignee: Lvalue::Variable{ name: Cow::Borrowed("a"), size: 8, offset: 0, subscript: Some(1) }},
            Statement{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: Cow::Borrowed("a"), size: 8, offset: 0, subscript: Some(2) },
                Rvalue::Variable{ name: Cow::Borrowed("a"), size: 8, offset: 0, subscript: Some(1) })), assignee: Lvalue::Variable{ name: Cow::Borrowed("a"), size: 8, offset: 0, subscript: Some(3) }});
        let mne1 = Mnemonic::new(0..10,"op1".to_string(),"{8:-:eax} nog".to_string(),ops1.iter(),i1.iter()).ok().unwrap();

        assert_eq!(mne1.area, Bound::new(0,10));
        assert_eq!(mne1.opcode, "op1");
        assert_eq!(mne1.operands, ops1);
        assert_eq!(mne1.instructions, i1);
    }
}
