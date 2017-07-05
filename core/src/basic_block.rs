/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015  Panopticon authors
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

//! A basic block is a sequence of Mnemonics that aren't interrupted by incoming or outgoing
//! jumps/branches.
//!
//! Basic blocks always occupy a continuous byte range.


use {Bound, Mnemonic, Statement, Program};
use std::cmp::{max, min};
use std::slice::Iter;
use std::ops::{Deref, DerefMut};

/// An iterator over every Statement in every Mnemonic in a BasicBlock
pub struct StatementIterator<'a> {
    mnemonics: Iter<'a, Mnemonic>,
    statements: Option<Iter<'a, Statement>>,
}

impl<'a> StatementIterator<'a> {
    /// Create a new statement iterator from `mnemonics`
    pub fn new(mnemonics: &'a[Mnemonic]) -> Self {
        StatementIterator {
            mnemonics: mnemonics.iter(),
            statements: None,
        }
    }
    fn get_next(&mut self) -> Option<&'a Statement> {
        let mut statement = None;
        while statement.is_none() {
            let mnemonic = self.mnemonics.next();
            match mnemonic {
                // termination
                None => return None,
                Some(mnemonic) => {
                    let mut statements = mnemonic.instructions.iter();
                    statement = statements.next();
                    self.statements = Some(statements);
                }
            }
        }
        return statement
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = &'a Statement;
    fn next(&mut self) ->  Option<Self::Item> {
        match self.statements {
            None => (),
            Some(ref mut iter) => {
                match iter.next() {
                    None => (),
                    some => return some
                }
            }
        }
        self.get_next()
    }
}

/// A basic block: a continiuous sequence of mnemonics without any branches in between.
#[derive(PartialEq,Eq,Debug,Serialize,Deserialize,Clone)]
pub struct BasicBlock {
    /// Area the basic block occupies in memory.
    pub area: Bound,
    /// List of mnemonics in to order of execution.
    pub mnemonics: Vec<Mnemonic>,
}

// this gets us iteration over mnemonics from a basic block for free
impl Deref for BasicBlock {
    type Target = [Mnemonic];
    fn deref(&self) -> &Self::Target {
        self.mnemonics.as_slice()
    }
}

impl DerefMut for BasicBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mnemonics.as_mut_slice()
    }
}

impl BasicBlock {
    /// Returns a new, empty basic block.
    pub fn new() -> BasicBlock {
        BasicBlock { area: Bound::new(0, 0), mnemonics: Vec::new() }
    }

    /// Moves `ms` into a new basic block. Panics if the mnemonics do not occupy a continuous
    /// address range.
    pub fn from_vec(ms: Vec<Mnemonic>) -> BasicBlock {
        let a = ms.iter()
            .fold(
                None, |acc: Option<Bound>, m| if acc == None {
                    return Some(m.area.clone());
                } else {
                    let r1 = &acc.unwrap();
                    let r2 = &m.area;
                    return Some(Bound::new(min(r1.start, r2.start), max(r1.end, r2.end)));
                }
            );
        return BasicBlock { area: a.unwrap_or(Bound::new(0, 0)), mnemonics: ms };
    }

    /// Calls `f` on all RREIL instructions starting from the last.
    pub fn execute_backwards<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a Statement),
    {
        for mne in self.mnemonics.iter().rev() {
            for i in mne.instructions.iter().rev() {
                f(&i);
            }
        }
    }

    /// Calls `f` on all RREIL instructions starting from the first.
    pub fn execute<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a Statement),
    {
        for mne in self.mnemonics.iter() {
            for i in mne.instructions.iter() {
                f(&i);
            }
        }
    }

    /// Calls `f` on all RREIL instructions starting from the first.
    pub fn rewrite<'a, F>(&'a mut self, mut f: F)
    where
        F: FnMut(&'a mut Statement),
    {
        for mne in self.mnemonics.iter_mut() {
            for i in mne.instructions.iter_mut() {
                f(i);
            }
        }
    }
    /// Displays the basic block in disassembly order, in human readable form, and looks up any functions calls in `program`
    pub fn display_with(&self, program: &Program) -> String {
        let seed = String::new();
        let display = self.mnemonics.iter().filter_map(|x| {
            if x.opcode.starts_with("__") {
                None
            } else {
                Some(x)
            }
        }).collect::<Vec<_>>();
        display.iter().fold(seed, |acc, ref m| -> String {
            format!("{}\n{}", acc, m.display_with(program))
        })
    }

    /// Returns an iterator over every statement in every mnemonic in this basic block
    pub fn statements(&self) -> StatementIterator {
        StatementIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Bound, Lvalue, Mnemonic, Operation, Rvalue, Statement};
    use std::borrow::Cow;

    #[test]
    fn construct() {
        let ops1 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
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

        let ops2 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
                subscript: None,
            },
        ];
        let i2 = vec![
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
                            subscript: Some(1),
                        },
                    ]
                ),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(3) },
            },
        ];
        let mne2 = Mnemonic::new(
            10..13,
            "op3".to_string(),
            "{s} nog".to_string(),
            ops2.iter(),
            i2.iter(),
        )
                .ok()
                .unwrap();

        let ops3 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
                subscript: None,
            },
        ];
        let i3 = vec![
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
                            subscript: Some(1),
                        },
                    ]
                ),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(3) },
            },
        ];
        let mne3 = Mnemonic::new(
            13..20,
            "op3".to_string(),
            "{s} nog".to_string(),
            ops3.iter(),
            i3.iter(),
        )
                .ok()
                .unwrap();

        let ms = vec![mne1, mne2, mne3];
        let bb1 = BasicBlock::from_vec(ms);

        assert_eq!(bb1.area, Bound::new(0, 20));

        let bb2 = BasicBlock::new();
        assert!(bb1 != bb2);

        let bb3 = BasicBlock::new();
        assert_eq!(bb3, bb2);
    }

    #[test]
    fn execute() {
        let ops1 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
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

        let ops2 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
                subscript: None,
            },
        ];
        let i2 = vec![
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
                            subscript: Some(1),
                        },
                    ]
                ),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(3) },
            },
        ];
        let mne2 = Mnemonic::new(
            10..13,
            "op3".to_string(),
            "{s} nog".to_string(),
            ops2.iter(),
            i2.iter(),
        )
                .ok()
                .unwrap();

        let ms = vec![mne1, mne2];
        let bb = BasicBlock::from_vec(ms);
        let mut vs2 = i1;
        let mut vs1 = Vec::<Statement>::new();

        bb.execute(|i| vs1.push(i.clone()));

        for i in i2.iter() {
            vs2.push(i.clone());
        }

        assert_eq!(vs1, vs2);
    }

    #[test]
    fn rewrite() {
        let ops1 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
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

        let ops2 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
                subscript: None,
            },
        ];
        let i2 = vec![
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
                            subscript: Some(1),
                        },
                    ]
                ),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(3) },
            },
        ];
        let mne2 = Mnemonic::new(
            10..13,
            "op3".to_string(),
            "{s} nog".to_string(),
            ops2.iter(),
            i2.iter(),
        )
                .ok()
                .unwrap();

        let ms = vec![mne1, mne2];
        let mut bb = BasicBlock::from_vec(ms);

        bb.rewrite(
            |i| {
                *i = match &i.assignee {
                    &Lvalue::Variable { name: ref n, size: ref w, subscript: _ } => {
                        Statement {
                            op: i.op.clone(),
                            assignee: Lvalue::Variable { name: n.clone(), size: *w, subscript: None },
                        }
                    }
                    _ => i.clone(),
                };
            }
        );

        let mut ok = true;

        bb.execute(
            |i| match i.assignee {
                Lvalue::Variable { subscript: None, .. } => ok &= true,
                _ => ok = false,
            }
        );

        assert!(ok);
    }

    #[test]
    fn statement_iterator() {
        let ops1 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
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

        let ops2 = vec![
            Rvalue::new_u8(1),
            Rvalue::Variable {
                name: Cow::Borrowed("a"),
                offset: 0,
                size: 3,
                subscript: None,
            },
        ];
        let i2 = vec![
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
                            offset: 0,
                            size: 8,
                            subscript: Some(2),
                        },
                        Rvalue::Variable {
                            name: Cow::Borrowed("a"),
                            offset: 0,
                            size: 8,
                            subscript: Some(1),
                        },
                    ]
                ),
                assignee: Lvalue::Variable { name: Cow::Borrowed("a"), size: 8, subscript: Some(3) },
            },
        ];
        let mne2 = Mnemonic::new(
            10..13,
            "op3".to_string(),
            "{s} nog".to_string(),
            ops2.iter(),
            i2.iter(),
        )
                .ok()
                .unwrap();

        let nstatements = i2.len() + i1.len();
        let ms = vec![mne1, mne2];
        let bb1 = BasicBlock::from_vec(ms);

        assert_eq!(bb1.area, Bound::new(0, 13));

        let statements = bb1.statements().collect::<Vec<_>>();
        assert_eq!(statements.len(), nstatements);
    }
}
