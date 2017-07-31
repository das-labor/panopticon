/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015  Panopticon authos
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

//! A Mnemonic is a single CPU instruction.
//!
//! The consist of an opcode, a number of arguments, and a sequence of RREIL instructions
//! descibing the mnemonic semantics.
//!
//! Mnemonics are CPU specific and Panopticon models them as simple as possible. Opcode are only
//! strings and the operands only a list of RREIL values. In order to display the mnemonics
//! correctly on the front-end mnemonics come with a format string. These tell Panopticon whenever
//! a operand is a pointer or a value. They look like this: `{c:ram}, {u}`.
//!
//! This formats the first operand as a code pointer into the "ram" and the second as an unsigned
//! value. Other formattings are `{d:<region>}` for data pointer into <region> and `{s}` for
//! signed values.

use Result;

use Rvalue;
use Statement;
use std::ops::Range;
use std::str::Chars;

/// A non-empty address range [start,end).
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct Bound {
    /// Address of the first byte inside the range.
    pub start: u64,
    /// Address of the first byte outside the range.
    pub end: u64,
}

impl Bound {
    /// Returns a `Bound` for [a,b)
    pub fn new(a: u64, b: u64) -> Bound {
        Bound { start: a, end: b }
    }

    /// Size of the range in bytes.
    pub fn len(&self) -> u64 {
        self.end - self.start
    }
}

/// Internal to `Mnemonic`
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
pub enum MnemonicFormatToken {
    /// Internal to `Mnemonic`
    Literal(char),
    /// Internal to `Mnemonic`
    Variable {
        /// Internal to `Mnemonic`
        has_sign: bool,
    },
    /// Internal to `Mnemonic`
    Pointer {
        /// Internal to `Mnemonic`
        is_code: bool,
        /// Internal to `Mnemonic`
        bank: String,
    },
}

impl MnemonicFormatToken {
    fn parse_bank<'a>(mut i: Chars<'a>) -> Result<(String, Chars<'a>)> {
        let mut j = i.clone();
        if i.next() == Some(':') {
            if let Some(p) = i.clone().position(|x| x == '}') {
                j.nth(p + 1);
                Ok((i.clone().take(p).collect::<String>(), j))
            } else {
                Err("Mnemonic format string parse error. Expecting '}'.".into())
            }
        } else {
            Err("Mnemonic format string parse error. Bank name is invalid.".into())
        }
    }

    /// format := '{' type '}'
    /// type := 'u' | # unsigned
    ///         's' | # signed
    ///         'p' ':' bank |  # data pointer
    ///         'c' ':' bank |  # code pointer
    pub fn parse(mut j: Chars) -> Result<Vec<MnemonicFormatToken>> {
        let mut ret = vec![];

        loop {
            match j.next() {
                None => break,
                Some('{') => {
                    match j.next() {
                        Some('{') => ret.push(MnemonicFormatToken::Literal('{')),
                        Some('u') => {
                            ret.push(MnemonicFormatToken::Variable { has_sign: false });
                            j.next();
                        }
                        Some('s') => {
                            ret.push(MnemonicFormatToken::Variable { has_sign: true });
                            j.next();
                        }
                        Some('p') => {

                            let (bank, k) = Self::parse_bank(j)?;
                            ret.push(MnemonicFormatToken::Pointer { is_code: false, bank: bank });
                            j = k;
                        }
                        Some('c') => {
                            let (bank, k) = Self::parse_bank(j)?;
                            ret.push(MnemonicFormatToken::Pointer { is_code: true, bank: bank });
                            j = k;
                        }
                        _ => return Err("Mnemonic format string parse error. Unknown format identifier.".into()),
                    }
                }
                Some(a) => ret.push(MnemonicFormatToken::Literal(a)),
            }
        }

        Ok(ret)
    }
}

/// A single Mnemonic.
#[derive(Clone,PartialEq,Eq,Debug,Serialize,Deserialize)]
pub struct Mnemonic {
    /// Range of bytes the mnemonic occupies
    pub area: Bound,
    /// Opcode part
    pub opcode: String,
    /// Operands
    pub operands: Vec<Rvalue>,
    /// RREIL code implementing the mnemonic
    pub instructions: Vec<Statement>,
    /// Describes how the operands need to be printed
    pub format_string: Vec<MnemonicFormatToken>,
}

impl Mnemonic {
    /// Create a new mnemonic `code`.
    pub fn new<'a, I1, I2>(a: Range<u64>, code: String, fmt: String, ops: I1, instr: I2) -> Result<Mnemonic>
    where
        I1: Iterator<Item = &'a Rvalue>,
        I2: Iterator<Item = &'a Statement>,
    {
        Ok(
            Mnemonic {
                area: Bound::new(a.start, a.end),
                opcode: code,
                operands: ops.cloned().collect(),
                instructions: instr.cloned().collect(),
                format_string: MnemonicFormatToken::parse(fmt.chars())?,
            }
        )
    }

    /// The size of this instruction mnemonic, in bytes
    pub fn size(&self) -> usize {
        self.area.len() as usize
    }

    /// For testing only
    #[cfg(test)]
    pub fn dummy(a: Range<u64>) -> Mnemonic {
        Mnemonic {
            area: Bound::new(a.start, a.end),
            opcode: "dummy".to_string(),
            operands: vec![],
            instructions: vec![],
            format_string: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Lvalue, Operation, Rvalue, Statement};
    use std::borrow::Cow;

    #[test]
    fn parse_format_string() {
        let fmt = "doe{u}io{s}øiq{s}   {p:te33} sasq {c:test}".to_string();
        let val = MnemonicFormatToken::parse(fmt.chars());

        assert_eq!(
            Some(
                vec![
                    MnemonicFormatToken::Literal('d'),
                    MnemonicFormatToken::Literal('o'),
                    MnemonicFormatToken::Literal('e'),
                    MnemonicFormatToken::Variable { has_sign: false },
                    MnemonicFormatToken::Literal('i'),
                    MnemonicFormatToken::Literal('o'),
                    MnemonicFormatToken::Variable { has_sign: true },
                    MnemonicFormatToken::Literal('ø'),
                    MnemonicFormatToken::Literal('i'),
                    MnemonicFormatToken::Literal('q'),
                    MnemonicFormatToken::Variable { has_sign: true },
                    MnemonicFormatToken::Literal(' '),
                    MnemonicFormatToken::Literal(' '),
                    MnemonicFormatToken::Literal(' '),
                    MnemonicFormatToken::Pointer { is_code: false, bank: "te33".to_string() },
                    MnemonicFormatToken::Literal(' '),
                    MnemonicFormatToken::Literal('s'),
                    MnemonicFormatToken::Literal('a'),
                    MnemonicFormatToken::Literal('s'),
                    MnemonicFormatToken::Literal('q'),
                    MnemonicFormatToken::Literal(' '),
                    MnemonicFormatToken::Pointer { is_code: true, bank: "test".to_string() },
                ]
            ),
            val.ok()
        );

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
        assert_eq!(
            MnemonicFormatToken::parse("{u}".to_string().chars()).ok(),
            Some(vec![MnemonicFormatToken::Variable { has_sign: false }])
        );
    }

    #[test]
    fn construct() {
        let ops1 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                size: 3,
                offset: 0,
                subscript: None,
            },
        ];
        let i1 = vec![
            Statement {
                op: Operation::Add(Rvalue::new_u8(1), Rvalue::new_u8(2)),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(2) },
            },
            Statement {
                op: Operation::Add(Rvalue::new_u8(4), Rvalue::new_u8(2)),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(1) },
            },
            Statement {
                op: Operation::Phi(
                    vec![
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            size: 8,
                            offset: 0,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            size: 8,
                            offset: 0,
                            subscript: Some(1),
                        },
                    ]
                ),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(3) },
            },
        ];
        let mne1 = Mnemonic::new(
            0..10,
            "op1".to_string(),
            "{s} nog".to_string(),
            ops1.iter(),
            i1.iter(),
        )
                .ok()
                .unwrap();

        assert_eq!(mne1.area, Bound::new(0, 10));
        assert_eq!(mne1.opcode, "op1");
        assert_eq!(mne1.operands, ops1);
        assert_eq!(mne1.instructions, i1);
    }
}
