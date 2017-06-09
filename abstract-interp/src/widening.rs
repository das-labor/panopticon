/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015, 2017  Panopticon authors
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

use {Avalue, Constraint, ProgramPoint};
use panopticon_core::{Operation, Rvalue, lift};


/// Mihaila et.al. Widening Point inferring cofibered domain. This domain is parameterized with a
/// child domain.
#[derive(Debug,PartialEq,Eq,Clone,Hash,RustcDecodable,RustcEncodable)]
pub struct Widening<A: Avalue> {
   value: A,
   point: Option<ProgramPoint>,
}

impl<A: Avalue> Avalue for Widening<A> {
   fn abstract_value(v: &Rvalue) -> Self {
      Widening { value: A::abstract_value(v), point: None }
   }

   fn abstract_constraint(c: &Constraint) -> Self {
      Widening { value: A::abstract_constraint(c), point: None }
   }

   fn execute(pp: &ProgramPoint, op: &Operation<Self>) -> Self {
      match op {
         &Operation::Phi(ref ops) => {
            let widen = ops.iter().map(|x| x.point.clone().unwrap_or(pp.clone())).max() > Some(pp.clone());

            Widening {
               value: match ops.len() {
                  0 => unreachable!("Phi function w/o arguments"),
                  1 => ops[0].value.clone(),
                  _ => {
                     ops.iter()
                        .map(|x| x.value.clone())
                        .fold(
                           A::initial(), |acc, x| if widen {
                              acc.widen(&x)
                           } else {
                              acc.combine(&x)
                           }
                        )
                  }
               },
               point: Some(pp.clone()),
            }
         }
         _ => {
            Widening {
               value: A::execute(pp, &lift(op, &|x| x.value.clone())),
               point: Some(pp.clone()),
            }
         }
      }
   }

   fn widen(&self, s: &Self) -> Self {
      Widening { value: self.value.widen(&s.value), point: self.point.clone() }
   }

   fn combine(&self, s: &Self) -> Self {
      Widening {
         value: self.value.combine(&s.value),
         point: self.point.clone(),
      }
   }

   fn narrow(&self, _: &Self) -> Self {
      self.clone()
   }

   fn initial() -> Self {
      Widening { value: A::initial(), point: None }
   }

   fn more_exact(&self, a: &Self) -> bool {
      self.value.more_exact(&a.value)
   }

   fn extract(&self, size: usize, offset: usize) -> Self {
      Widening {
         value: self.value.extract(size, offset),
         point: self.point.clone(),
      }
   }
}
