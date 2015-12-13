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

use value::Rvalue;
use codegen::CodeGen;
use mos::*;

/*
fn do_push(v: &Rvalue, cg: &mut CodeGen<Mos>) {
    if let &Rvalue::Variable{ width: w, ..} = v {
        cg.assign(&Lvalue::Memory{
            offset: Box::new(SP.to_rv() + 0x100),
            bytes: w / 8,
            endianess: Endianess::Little,
            name: "ram".to_string()
        },v);

	cg.sub_i(&*SP,&SP.to_rv(),&Rvalue::Constant(w as u64));
        cg.mod_i(&*SP,&SP.to_rv(),&Rvalue::Constant(0x100));
    } else {
        unreachable!()
    }
}

*/

pub fn nop(_: &mut CodeGen<Mos>) {}

pub fn nop_r(_: &mut CodeGen<Mos>, _: Rvalue) {}

pub fn nop_rr(_: &mut CodeGen<Mos>, _: Rvalue, _: Rvalue) {}
