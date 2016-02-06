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

use std::fmt::{Formatter,Display,Error};

#[derive(Clone,Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
pub enum Endianess {
    Little,
    Big,
}

#[derive(Clone,Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
pub enum Rvalue {
    Constant(u64),
    Undefined,
    Variable{ width: u16, name: String, subscript: Option<u32> },
    Memory{ offset: Box<Rvalue>, bytes: u16, endianess: Endianess, name: String },
}

#[derive(Clone,Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
pub enum Lvalue {
    Undefined,
    Variable{ width: u16, name: String, subscript: Option<u32> },
    Memory{ offset: Box<Rvalue>, bytes: u16, endianess: Endianess, name: String },
}

impl Rvalue {
    pub fn from_lvalue(rv: &Lvalue) -> Rvalue {
        match rv {
            &Lvalue::Undefined => Rvalue::Undefined,
            &Lvalue::Variable{ width: ref w, name: ref n, subscript: ref s} =>
                Rvalue::Variable{ width: w.clone(), name: n.clone(), subscript: s.clone()},
            &Lvalue::Memory{ offset: ref o, bytes: ref b, endianess: ref e, name: ref n} =>
                Rvalue::Memory{ offset: o.clone(), bytes: b.clone(), endianess: e.clone(), name: n.clone()},
        }
    }
}

impl Display for Rvalue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Rvalue::Constant(c) => f.write_fmt(format_args!("{:x}",c)),
            &Rvalue::Undefined => f.write_str("undef"),
            &Rvalue::Variable{ name: ref n, subscript: Some(ref s),.. } => f.write_fmt(format_args!("{}_{}",n,s)),
            &Rvalue::Variable{ name: ref n,.. } => f.write_str(n),
            &Rvalue::Memory{ offset: ref o, name: ref n,..} => f.write_fmt(format_args!("{}[{}]",n,o)),
        }
    }
}

impl Lvalue {
    pub fn from_rvalue(rv: &Rvalue) -> Option<Lvalue> {
        match rv {
            &Rvalue::Undefined => Some(Lvalue::Undefined),
            &Rvalue::Variable{ width: ref w, name: ref n, subscript: ref s} =>
                Some(Lvalue::Variable{ width: w.clone(), name: n.clone(), subscript: s.clone()}),
            &Rvalue::Memory{ offset: ref o, bytes: ref b, endianess: ref e, name: ref n} =>
                Some(Lvalue::Memory{ offset: o.clone(), bytes: b.clone(), endianess: e.clone(), name: n.clone()}),
            _ => None,
        }
    }

    pub fn to_rv(&self) -> Rvalue {
        Rvalue::from_lvalue(self)
    }
}

pub trait ToRvalue {
    fn to_rv(&self) -> Rvalue;
}

impl ToRvalue for Rvalue {
    fn to_rv(&self) -> Rvalue {
        self.clone()
    }
}

impl ToRvalue for Lvalue {
    fn to_rv(&self) -> Rvalue {
        Rvalue::from_lvalue(self)
    }
}

impl ToRvalue for u64 {
    fn to_rv(&self) -> Rvalue {
        Rvalue::Constant(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let u = Rvalue::Undefined;
        let c = Rvalue::Constant(5);
        let v = Rvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let m = Rvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::Little, name: "ram".to_string() };

        let u2 = u.clone();
        let c2 = c.clone();
        let v2 = v.clone();
        let m2 = m.clone();

        println!("{:?} {:?} {:?} {:?}",u,c,v,m);

        assert_eq!(u,u2);
        assert_eq!(c,c2);
        assert_eq!(v,v2);
        assert_eq!(m,m2);
    }

    #[test]
    fn convert_lvalue_rvalue() {
        let ru = Rvalue::Undefined;
        let rc = Rvalue::Constant(5);
        let rv = Rvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let rm = Rvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::Little, name: "ram".to_string() };

        let lu = Lvalue::Undefined;
        let lv = Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let lm = Lvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::Little, name: "ram".to_string() };

        assert_eq!(Some(lu.clone()), Lvalue::from_rvalue(&ru));
        assert_eq!(Some(lv.clone()), Lvalue::from_rvalue(&rv));
        assert_eq!(Some(lm.clone()), Lvalue::from_rvalue(&rm));
        assert_eq!(None, Lvalue::from_rvalue(&rc));

        assert_eq!(ru, Rvalue::from_lvalue(&lu));
        assert_eq!(rv, Rvalue::from_lvalue(&lv));
        assert_eq!(rm, Rvalue::from_lvalue(&lm));
    }
}
