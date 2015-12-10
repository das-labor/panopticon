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

use disassembler::*;
use value::{Lvalue,Rvalue,ToRvalue};
use codegen::CodeGen;
use guard::Guard;
use std::rc::Rc;
use std::num::Wrapping;
use super::*;

#[allow(overflowing_literals)]
pub fn disassembler() -> Rc<Disassembler<Mos>> {
    new_disassembler!(Mos =>
        // NOP
        [ 0xea ] = |st: &mut State<Mos>| {
	    let len = st.tokens.len();
	    let next = st.configuration.wrap(st.address + len as u64);
            st.mnemonic(len, "nop", "", vec!(), &|_: &mut CodeGen<Mos>| {});
            st.jump(next, Guard::always());
            true
        },

 	// catch all
        _ = |st: &mut State<Mos>| {
	    let len = st.tokens.len();
            let next = st.configuration.wrap(st.address + len as u64);
            st.mnemonic(len, "unk", "", vec!(),&|_: &mut CodeGen<Mos>| {});
            st.jump(next, Guard::always());
            true
        }
    )
}
