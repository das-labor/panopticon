/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
 * Copyright (C) 2015 Marcus Brinkmann
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
use codegen::*;
use value::*;
use mos::decode::*;
use mos::semantic::*;
use mos::*;

use std::rc::Rc;

/* use value::{Lvalue,Rvalue,ToRvalue};
use guard::Guard;
use std::num::Wrapping;
use super::*;
*/


#[allow(overflowing_literals)]
pub fn disassembler() -> Rc<Disassembler<Mos>> {
    let imm = new_disassembler!(Mos =>
        [ "imm@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let zpg = new_disassembler!(Mos =>
        [ "zpg@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("zpg")));
            true
        });

    let rel = new_disassembler!(Mos =>
        [ "rel@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("rel")));
            true
        });

    let xind = new_disassembler!(Mos =>
        [ "xind@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("xind")));
            true
        });

    let indy = new_disassembler!(Mos =>
        [ "indy@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("indy")));
            true
        });

    let abs = new_disassembler!(Mos =>
        [ "lo@........", "hi@........" ] = |st: &mut State<Mos>| {
	    let addr = ((st.get_group("hi") as u16) << 8) + st.get_group("lo") as u16;
            st.configuration.arg0 = Some(Rvalue::Constant(addr as u64));
            true
        });

    let ind = new_disassembler!(Mos =>
        [ "lo@........", "hi@........" ] = |st: &mut State<Mos>| {
	    let addr = ((st.get_group("hi") as u16) << 8) + st.get_group("lo") as u16;
            st.configuration.arg0 = Some(Rvalue::Constant(addr as u64));
            true
        });

    // FIXME: Add illegal opcodes.
    new_disassembler!(Mos =>
        // ADC
	[ 0x65, zpg ] = unary_z("adc", nop_r),
	[ 0x69, imm ] = unary_i("adc", nop_r),
	[ 0x75, zpg ] = unary_zr("adc", &*X, nop_rr),
	[ 0x61, xind ] = unary_xind("adc", nop_r),
	[ 0x71, indy ] = unary_indy("adc", nop_r),
	[ 0x6d, abs ] = unary_a("adc", nop_r),
	[ 0x79, abs ] = unary_ar("adc", &*Y, nop_rr),
	[ 0x7d, abs ] = unary_ar("adc", &*X, nop_rr),

	// AND
	[ 0x25, zpg ] = unary_z("and", nop_r),
        [ 0x29, imm ] = unary_i("and", nop_r),
	[ 0x35, zpg ] = unary_zr("and", &*X, nop_rr),
	[ 0x21, xind ] = unary_xind("and", nop_r),
	[ 0x31, indy ] = unary_indy("and", nop_r),
	[ 0x2d, abs ] = unary_a("and", nop_r),
	[ 0x39, abs ] = unary_ar("and", &*Y, nop_rr),
	[ 0x3d, abs ] = unary_ar("and", &*X, nop_rr),

	// ASL
	[ 0x0a ] = unary_r("asl", &*A, nop_r),
	[ 0x06, zpg ] = unary_z("asl", nop_r),
	[ 0x16, zpg ] = unary_zr("asl", &*X, nop_rr),
	[ 0x0e, abs ] = unary_a("asl", nop_r),
	[ 0x1e, abs ] = unary_ar("asl", &*X, nop_rr),

	// BCx
	[ 0x90, rel ] = unary_b("bcc", &*C, false, nop_r),
	[ 0xa0, rel ] = unary_b("bcs", &*C, true, nop_r),

	// BEQ, BNE
	[ 0xf0, rel ] = unary_b("beq", &*Z, true, nop_r),
	[ 0xc0, rel ] = unary_b("bne", &*Z, false, nop_r),

	// BIT
	[ 0x24, zpg ] = unary_z("bit", nop_r),
	[ 0x2c, abs ] = unary_a("bit", nop_r),

	// BMI, BPL
	[ 0x30, rel ] = unary_b("bmi", &*N, true, nop_r),
	[ 0x10, rel ] = unary_b("bpl", &*N, false, nop_r),

	// BVC, BVS
	[ 0x50, rel ] = unary_b("bvc", &*V, false, nop_r),
	[ 0x70, rel ] = unary_b("bvs", &*V, true, nop_r),

	// BRK
	[ 0x00 ] = nonary("brk", nop),

	// CLx
	[ 0x58 ] = nonary("cli", nop),
	[ 0xb8 ] = nonary("clv", nop),
	[ 0x18 ] = nonary("clc", nop),
	[ 0xd8 ] = nonary("cld", nop),

	// CMP
	[ 0xc5, zpg ] = unary_z("cmp", nop_r),
	[ 0xc9, imm ] = unary_i("cmp", nop_r),
	[ 0xd5, zpg ] = unary_zr("cmp", &*X, nop_rr),
	[ 0xc1, xind ] = unary_xind("cmp", nop_r),
	[ 0xd1, indy ] = unary_indy("cmp", nop_r),
	[ 0xcd, abs ] = unary_a("cmp", nop_r),
	[ 0xd9, abs ] = unary_ar("cmp", &*Y, nop_rr),
	[ 0xdd, abs ] = unary_ar("cmp", &*X, nop_rr),

	// CPx
	[ 0xe4, zpg ] = unary_z("cpx", nop_r),
	[ 0xe0, imm ] = unary_i("cpx", nop_r),
	[ 0xec, abs ] = unary_a("cpx", nop_r),
	[ 0xc4, zpg ] = unary_z("cpy", nop_r),
	[ 0xc0, imm ] = unary_i("cpy", nop_r),
	[ 0xcc, abs ] = unary_a("cpy", nop_r),

	// DEC
	[ 0xc6, zpg ] = unary_z("dec", nop_r),
	[ 0xd6, zpg ] = unary_zr("dec", &*X, nop_rr),
	[ 0xce, abs ] = unary_a("dec", nop_r),
	[ 0xde, abs ] = unary_ar("dec", &*X, nop_rr),

	// DEx
	[ 0xca ] = nonary("dex", nop),
	[ 0x88 ] = nonary("dey", nop),

	// EOR
	[ 0x45, zpg ] = unary_z("eor", nop_r),
	[ 0x55, zpg ] = unary_zr("eor", &*X, nop_rr),
	[ 0x49, imm ] = unary_i("eor", nop_r),
	[ 0x41, xind ] = unary_xind("eor", nop_r),
	[ 0x51, indy ] = unary_indy("eor", nop_r),
	[ 0x4d, abs ] = unary_a("eor", nop_r),
	[ 0x59, abs ] = unary_ar("eor", &*Y, nop_rr),
	[ 0x5d, abs ] = unary_ar("eor", &*X, nop_rr),

	// INC
	[ 0xe6, zpg ] = unary_z("inc", nop_r),
	[ 0xf6, zpg ] = unary_zr("inc", &*X, nop_rr),
	[ 0xee, abs ] = unary_a("inc", nop_r),
	[ 0xfe, abs ] = unary_ar("inc", &*X, nop_rr),

	// INx
	[ 0xe8 ] = nonary("inx", nop),
	[ 0xc8 ] = nonary("iny", nop),

	// JMP
	[ 0x4c, abs ] = unary_goto_a("jmp", nop_r),
	[ 0x6c, ind ] = unary_goto_ind("jmp", nop_r),

	// JSR
	[ 0x20, abs ] = unary_call_a("jsr", nop_r),

	// LDA
	[ 0xa5, zpg ] = unary_z("lda", nop_r),
	[ 0xa9, imm ] = unary_i("lda", nop_r),
	[ 0xb5, zpg ] = unary_zr("lda", &*X, nop_rr),
	[ 0xa1, xind ] = unary_xind("lda", nop_r),
	[ 0xb1, indy ] = unary_indy("lda", nop_r),
	[ 0xad, abs ] = unary_a("lda", nop_r),
	[ 0xb9, abs ] = unary_ar("lda", &*Y, nop_rr),
	[ 0xbd, abs ] = unary_ar("lda", &*X, nop_rr),

	// LDX
	[ 0xa6, zpg ] = unary_z("ldx", nop_r),
	[ 0xa2, imm ] = unary_i("ldx", nop_r),
	[ 0xb6, zpg ] = unary_zr("ldx", &*Y, nop_rr),
	[ 0xae, abs ] = unary_a("ldx", nop_r),
	[ 0xbe, abs ] = unary_ar("ldx", &*Y, nop_rr),

	// LDY
	[ 0xa4, zpg ] = unary_z("ldy", nop_r),
	[ 0xa0, imm ] = unary_i("ldy", nop_r),
	[ 0xb4, zpg ] = unary_zr("ldy", &*X, nop_rr),
	[ 0xac, abs ] = unary_a("ldy", nop_r),
	[ 0xbc, abs ] = unary_ar("ldy", &*X, nop_rr),

	// LSR
	[ 0x4a ] = unary_r("lsr", &*A, nop_r),
	[ 0x46, zpg ] = unary_z("lsr", nop_r),
	[ 0x56, zpg ] = unary_zr("lsr", &*X, nop_rr),
	[ 0x4e, abs ] = unary_a("lsr", nop_r),
	[ 0x5e, abs ] = unary_ar("lsr", &*X, nop_rr),

	// NOP
	[ 0xea ] = nonary("nop", nop),

	// ORA
	[ 0x05, zpg ] = unary_z("ora", nop_r),
	[ 0x15, zpg ] = unary_zr("ora", &*X, nop_rr),
	[ 0x09, imm ] = unary_i("ora", nop_r),
	[ 0x01, xind ] = unary_xind("ora", nop_r),
	[ 0x11, indy ] = unary_indy("ora", nop_r),
	[ 0x0d, abs ] = unary_a("ora", nop_r),
	[ 0x19, abs ] = unary_ar("ora", &*Y, nop_rr),
	[ 0x1d, abs ] = unary_ar("ora", &*X, nop_rr),

	// PHx, PLx
	[ 0x48 ] = nonary("pha", nop),
	[ 0x08 ] = nonary("php", nop),
	[ 0x68 ] = nonary("pla", nop),
	[ 0x28 ] = nonary("plp", nop),

	// ROx
	[ 0x2a ] = unary_r("rol", &*A, nop_r),
	[ 0x26, zpg ] = unary_z("rol", nop_r),
	[ 0x36, zpg ] = unary_zr("rol", &*X, nop_rr),
	[ 0x2e, abs ] = unary_a("rol", nop_r),
	[ 0x3e, abs ] = unary_ar("rol", &*X, nop_rr),
	[ 0x6a ] = unary_r("ror", &*A, nop_r),
	[ 0x66, zpg ] = unary_z("ror", nop_r),
	[ 0x76, zpg ] = unary_zr("ror", &*X, nop_rr),
	[ 0x6e, abs ] = unary_a("ror", nop_r),
	[ 0x7e, abs ] = unary_ar("ror", &*X, nop_rr),

	// RTI
	[ 0x40 ] = nonary_ret("rti", nop),

	// RTS
	[ 0x60 ] = nonary_ret("rts", nop),

	// SBC
	[ 0xe5, zpg ] = unary_z("sbc", nop_r),
	[ 0xe9, imm ] = unary_i("sbc", nop_r),
	[ 0xf5, zpg ] = unary_zr("sbc", &*X, nop_rr),
	[ 0xe1, xind ] = unary_xind("sbc", nop_r),
	[ 0xf1, indy ] = unary_indy("sbc", nop_r),
	[ 0xed, abs ] = unary_a("sbc", nop_r),
	[ 0xf9, abs ] = unary_ar("sbc", &*Y, nop_rr),
	[ 0xfd, abs ] = unary_ar("sbc", &*X, nop_rr),

	// SEx (no SEV!)
	[ 0x38 ] = nonary("sec", nop),
	[ 0xf8 ] = nonary("sed", nop),
	[ 0x78 ] = nonary("sei", nop),

	// STA
	[ 0x85, zpg ] = unary_z("sta", nop_r),
	[ 0x95, zpg ] = unary_zr("sta", &*X, nop_rr),
	[ 0x81, xind ] = unary_xind("sta", nop_r),
	[ 0x91, indy ] = unary_indy("sta", nop_r),
	[ 0x8d, abs ] = unary_a("sta", nop_r),
	[ 0x99, abs ] = unary_ar("sta", &*Y, nop_rr),
	[ 0x9d, abs ] = unary_ar("sta", &*X, nop_rr),

	// STx
	[ 0x86, zpg ] = unary_z("stx", nop_r),
	[ 0x96, zpg ] = unary_zr("stx", &*Y, nop_rr),
	[ 0x8e, abs ] = unary_a("stx", nop_r),
	[ 0x84, zpg ] = unary_z("sty", nop_r),
	[ 0x94, zpg ] = unary_zr("sty", &*X, nop_rr),
	[ 0x8c, abs ] = unary_a("sty", nop_r),

	// Txy
	[ 0xaa ] = nonary("tax", nop),
	[ 0xa8 ] = nonary("tay", nop),
	[ 0xba ] = nonary("tsx", nop),
	[ 0x8a ] = nonary("txa", nop),
	[ 0x9a ] = nonary("txs", nop),
	[ 0x98 ] = nonary("tya", nop),

 	// catch all, FIXME: Add at least the args for illegal opcodes.
        _ = nonary("unk", nop)
    )
}
