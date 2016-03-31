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

use value::{Rvalue,ToRvalue};
use rustc_serialize::{Encodable,Decodable};
use std::hash::Hash;
use std::fmt::{Formatter,Display,Error,Debug};

#[derive(Debug,Clone,PartialEq,Eq,Hash,RustcDecodable,RustcEncodable)]
pub enum Relation {
    NotEqual,
    Equal,
    UnsignedLess,
    UnsignedLessOrEqual,
    UnsignedGreater,
    UnsignedGreaterOrEqual,
    SignedLess,
    SignedLessOrEqual,
    SignedGreater,
    SignedGreaterOrEqual,
}

impl Relation {
    pub fn negation(&self) -> Relation {
        match self {
            &Relation::UnsignedLessOrEqual => Relation::UnsignedGreater,
            &Relation::SignedLessOrEqual => Relation::SignedGreater,
            &Relation::UnsignedGreaterOrEqual => Relation::UnsignedLess,
            &Relation::SignedGreaterOrEqual => Relation::SignedLess,
            &Relation::UnsignedLess => Relation::UnsignedGreaterOrEqual,
            &Relation::SignedLess => Relation::SignedGreaterOrEqual,
            &Relation::UnsignedGreater => Relation::UnsignedLessOrEqual,
            &Relation::SignedGreater => Relation::SignedLessOrEqual,
            &Relation::Equal => Relation::NotEqual,
            &Relation::NotEqual => Relation::Equal,
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash,RustcDecodable,RustcEncodable)]
pub enum Constraint {
    Predicate{ relation: Relation, left: Rvalue, right: Rvalue },
    True,
    False,
}


impl Constraint {
    pub fn operands<'a>(&'a self) -> Vec<&'a Rvalue> {
        if let &Constraint::Predicate{ ref left, ref right,.. } = self {
            vec![left,right]
        } else {
            vec![]
        }
    }

    pub fn operands_mut<'a>(&'a mut self) -> Vec<&'a mut Rvalue> {
        if let &mut Constraint::Predicate{ ref mut left, ref mut right,.. } = self {
            vec![left,right]
        } else {
            vec![]
        }
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Constraint::Predicate{ ref relation, ref left, ref right } => match relation {
                &Relation::UnsignedLessOrEqual => f.write_fmt(format_args!("{} ≤ᵤ {}",left,right)),
                &Relation::SignedLessOrEqual => f.write_fmt(format_args!("{} ≤ₛ {}",left,right)),
                &Relation::UnsignedGreaterOrEqual => f.write_fmt(format_args!("{} ≥ᵤ {}",left,right)),
                &Relation::SignedGreaterOrEqual => f.write_fmt(format_args!("{} ≥ₛ {}",left,right)),
                &Relation::UnsignedLess => f.write_fmt(format_args!("{} <ᵤ {}",left,right)),
                &Relation::SignedLess => f.write_fmt(format_args!("{} <ₛ {}",left,right)),
                &Relation::UnsignedGreater => f.write_fmt(format_args!("{} >ᵤ {}",left,right)),
                &Relation::SignedGreater => f.write_fmt(format_args!("{} >ₛ {}",left,right)),
                &Relation::Equal => f.write_fmt(format_args!("{} = {}",left,right)),
                &Relation::NotEqual => f.write_fmt(format_args!("{} ≠ {}",left,right)),
            },
            &Constraint::True => f.write_str("true"),
            &Constraint::False => f.write_str("false"),
        }
    }
}

#[derive(Clone,Debug,PartialEq,RustcDecodable,RustcEncodable)]
pub struct Guard {
    pub constraint: Constraint,
}

impl Guard {
    pub fn new(r: Constraint) -> Guard {
        Guard{ constraint: r }
    }

    pub fn never() -> Guard {
        Guard{ constraint: Constraint::False }
    }

    pub fn always() -> Guard {
        Guard{ constraint: Constraint::True }
    }

    pub fn eq<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::Equal, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn neq<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::NotEqual, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn sign_gt<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::SignedGreater, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn unsi_gt<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::UnsignedGreater, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn sign_less<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::SignedLess, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn unsi_less<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::UnsignedLess, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn sign_geq<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::SignedGreaterOrEqual, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn unsi_geq<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::UnsignedGreaterOrEqual, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn sign_leq<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::SignedLessOrEqual, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn unsi_leq<A: ToRvalue, B: ToRvalue>(a: &A, b: &B) -> Guard {
        Guard{ constraint: Constraint::Predicate{
            relation: Relation::UnsignedLessOrEqual, left: a.to_rv(), right: b.to_rv() }
        }
    }

    pub fn negation(&self) -> Guard {
        Guard::new(match &self.constraint {
            &Constraint::Predicate{ ref relation, ref left, ref right } =>
                Constraint::Predicate{ relation: relation.negation(), left: left.clone(), right: right.clone() },
            &Constraint::True => Constraint::False,
            &Constraint::False => Constraint::True,
        })
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{}",self.constraint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::{Rvalue};

    #[test]
    fn construct() {
        let g = Guard::new(Constraint::Predicate{
            relation: Relation::UnsignedGreater, left: Rvalue::Undefined, right: Rvalue::Undefined
        });
        let g2 = Guard::new(Constraint::Predicate{
            relation: Relation::Equal, left: Rvalue::Undefined, right: Rvalue::Undefined
        });

        assert!(g != g2);
    }

    #[test]
    fn negation() {
        let g1 = Guard::new(Constraint::Predicate{ relation:
            Relation::UnsignedLessOrEqual, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g2 = Guard::new(Constraint::Predicate{ relation:
            Relation::SignedLessOrEqual, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g3 = Guard::new(Constraint::Predicate{ relation:
            Relation::UnsignedGreaterOrEqual, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g4 = Guard::new(Constraint::Predicate{ relation:
            Relation::SignedGreaterOrEqual, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g5 = Guard::new(Constraint::Predicate{ relation:
            Relation::UnsignedLess, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g6 = Guard::new(Constraint::Predicate{ relation:
            Relation::SignedLess, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g7 = Guard::new(Constraint::Predicate{ relation:
            Relation::UnsignedGreater, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g8 = Guard::new(Constraint::Predicate{ relation:
            Relation::SignedGreater, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g9 = Guard::new(Constraint::Predicate{ relation:
            Relation::Equal, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g10 = Guard::new(Constraint::Predicate{ relation:
            Relation::NotEqual, left: Rvalue::Undefined, right: Rvalue::Undefined });
        let g11 = Guard::new(Constraint::True);
        let g12 = Guard::new(Constraint::False);

        let not_g1 = g1.negation();
        let not_g2 = g2.negation();
        let not_g3 = g3.negation();
        let not_g4 = g4.negation();
        let not_g5 = g5.negation();
        let not_g6 = g6.negation();
        let not_g7 = g7.negation();
        let not_g8 = g8.negation();
        let not_g9 = g9.negation();
        let not_g10 = g10.negation();
        let not_g11 = g11.negation();
        let not_g12 = g12.negation();

        assert!(g1 != not_g1);
        assert!(g2 != not_g2);
        assert!(g3 != not_g3);
        assert!(g4 != not_g4);
        assert!(g5 != not_g5);
        assert!(g6 != not_g6);
        assert!(g7 != not_g7);
        assert!(g8 != not_g8);
        assert!(g9 != not_g9);
        assert!(g10 != not_g10);
        assert!(g11 != not_g11);
        assert!(g12 != not_g12);

        assert_eq!(g1,not_g1.negation());
        assert_eq!(g2,not_g2.negation());
        assert_eq!(g3,not_g3.negation());
        assert_eq!(g4,not_g4.negation());
        assert_eq!(g5,not_g5.negation());
        assert_eq!(g6,not_g6.negation());
        assert_eq!(g7,not_g7.negation());
        assert_eq!(g8,not_g8.negation());
        assert_eq!(g9,not_g9.negation());
        assert_eq!(g10,not_g10.negation());
        assert_eq!(g11,not_g11.negation());
        assert_eq!(g12,not_g12.negation());
    }
}
