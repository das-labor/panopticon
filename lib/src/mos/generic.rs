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
use mos::decode::*;
use mos::semantic::*;
use mos::*;

use std::rc::Rc;

/* use value::{Lvalue,Rvalue,ToRvalue};
use guard::Guard;
use std::num::Wrapping;
use super::*;
*/

/* Tests:
   http://visual6502.org/wiki/index.php?title=6502TestPrograms
   specifically:
   https://github.com/kingcons/cl-6502/blob/b0087903428ec2a3794ba4219494005174d1b09f/tests/6502_functional_test.a65
   http://www.6502.org/tutorials/decimal_mode.html
   http://6502org.wikidot.com/errata-other-decmode
*/

#[allow(overflowing_literals)]
pub fn disassembler() -> Rc<Disassembler<Mos>> {
    /* izx/y addressing can't be represented as an Rvalue for several
       reasons: we can't use addition for the offset, and we can't
       represent the wrap around the zero page both when adding X and
       when reading the word at page boundary.  */
    let izx = new_disassembler!(Mos =>
        [ "izx@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("izx")));
            true
        });

    let izy = new_disassembler!(Mos =>
        [ "izy@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("izy")));
            true
        });

    let imm = new_disassembler!(Mos =>
        [ "imm@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("imm")));
            true
        });

    let zpg = new_disassembler!(Mos =>
        [ "zpg@........" ] = |st: &mut State<Mos>| {
            let off = st.get_group("zpg");
	    st.configuration.arg0 = Some(Rvalue::Memory{
	        offset: Box::new(Rvalue::Constant(off)),
	        name: "ram".to_string(),
	        endianess: Endianess::Little,
	        bytes: 1
	    });
	    true
        });

    let imm16 = new_disassembler!(Mos =>
        [ "lo@........", "hi@........" ] = |st: &mut State<Mos>| {
	    let addr = ((st.get_group("hi") as u16) << 8) + st.get_group("lo") as u16;
	    st.configuration.arg0 = Some(Rvalue::Constant(addr as u64));
	    true
        });

    let abs = new_disassembler!(Mos =>
        [ "lo@........", "hi@........" ] = |st: &mut State<Mos>| {
	    let addr = ((st.get_group("hi") as u16) << 8) + st.get_group("lo") as u16;
	    st.configuration.arg0 = Some(Rvalue::Memory{
	        offset: Box::new(Rvalue::Constant(addr as u64)),
	        name: "ram".to_string(),
	        endianess: Endianess::Little,
	        bytes: 1
	    });
	    true
        });

    let rel = new_disassembler!(Mos =>
        [ "rel@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg0 = Some(Rvalue::Constant(st.get_group("rel")));
	    true
        });

    /* FIXME: Implement indirect addressing by wrapping this in a memory rvalue.  */
    let ind = new_disassembler!(Mos =>
        [ "lo@........", "hi@........" ] = |st: &mut State<Mos>| {
	    let addr = ((st.get_group("hi") as u16) << 8) + st.get_group("lo") as u16;
	    st.configuration.arg0 = Some(Rvalue::Memory{
	        offset: Box::new(Rvalue::Constant(addr as u64)),
	        name: "ram".to_string(),
	        endianess: Endianess::Little,
	        bytes: 1
	    });
            true
        });

    // FIXME: Add illegal opcodes.
    new_disassembler!(Mos =>
        // ADC
	[ 0x61, izx ] = unary_izx("adc", adc_izx),	// 011 000 01 xxxx xxxx
	[ 0x65, zpg ] = unary_z("adc", adc),		// 011 001 01 zzzz zzzz
	[ 0x69, imm ] = unary_i("adc", adc),		// 011 010 01 iiii iiii
	[ 0x6d, abs ] = unary_a("adc", adc),		// 011 011 01 aaaa aaaa
	[ 0x71, izy ] = unary_izy("adc", adc_izy),	// 011 100 01 yyyy yyyy
	[ 0x75, zpg ] = unary_zr("adc", &*X, adc_zpi),	// 011 101 01 zzzz zzzz,X
	[ 0x79, abs ] = unary_ar("adc", &*Y, adc_idx),	// 011 110 01 aaaa aaaa,Y
	[ 0x7d, abs ] = unary_ar("adc", &*X, adc_idx),	// 011 111 01 aaaa aaaa,X

	// AND
	[ 0x21, izx ] = unary_izx("and", and_izx),	// 001 000 01 xxxx xxxx
	[ 0x25, zpg ] = unary_z("and", and),		// 001 001 01 zzzz zzzz
        [ 0x29, imm ] = unary_i("and", and),		// 001 010 01 iiii iiii
	[ 0x2d, abs ] = unary_a("and", and),		// 001 011 01 aaaa aaaa
	[ 0x31, izy ] = unary_izy("and", and_izy),	// 001 100 01 yyyy yyyy
	[ 0x35, zpg ] = unary_zr("and", &*X, and_zpi),	// 001 101 01 zzzz zzzz,X
	[ 0x39, abs ] = unary_ar("and", &*Y, and_idx),	// 001 110 01 aaaa aaaa,X
	[ 0x3d, abs ] = unary_ar("and", &*X, and_idx),	// 001 111 01 aaaa aaaa,Y

	// ASL
	[ 0x02 ] = nonary_ret("kil!", nop),		// 000 00 010*
	[ 0x0a ] = unary_r("asl", &*A, asl),		// 000 01 010  A
	[ 0x12 ] = nonary_ret("kil!", nop),		// 000 10 010*
	[ 0x1a ] = nonary("nop!", nop),	     		// 000 11 010*
	// ASL arg
	[ 0x06, zpg ] = unary_z("asl", asl),		// 000 00 110  zzzz zzzz
	[ 0x0e, abs ] = unary_a("asl", asl),		// 000 01 110  aaaa aaaa
	[ 0x16, zpg ] = unary_zr("asl", &*X, asl_zpi),	// 000 10 110  zzzz zzzz,X
	[ 0x1e, abs ] = unary_ar("asl", &*X, asl_idx),   // 000 11 110  aaaa aaaa,X

	// BCx
	[ 0x90, rel ] = unary_b("bcc", &*C, false, nop_r),	// 1001 0000 rrrr rrrr
	[ 0xb0, rel ] = unary_b("bcs", &*C, true, nop_r),	// 1010 0000 rrrr rrrr

	// BEQ, BNE
	[ 0xf0, rel ] = unary_b("beq", &*Z, true, nop_r),	// 1111 0000 rrrr rrrr
	[ 0xd0, rel ] = unary_b("bne", &*Z, false, nop_r), 	// 1100 0000 rrrr rrrr

	// BIT
	[ 0x24, zpg ] = unary_z("bit", bit),			// 0010 0 100 zzzz zzzz
	[ 0x2c, abs ] = unary_a("bit", bit),			// 0010 1 100 aaaa aaaa

	// BMI, BPL
	[ 0x30, rel ] = unary_b("bmi", &*N, true, nop_r),	// 0011 0000
	[ 0x10, rel ] = unary_b("bpl", &*N, false, nop_r),	// 0001 0000

	// BVC, BVS
	[ 0x50, rel ] = unary_b("bvc", &*V, false, nop_r),	// 0101 0000
	[ 0x70, rel ] = unary_b("bvs", &*V, true, nop_r),	// 0111 0000

	// BRK
	[ 0x00, imm ] = unary_i("brk", brk),			// 0000 0000 FIXME: Maybe clobber the registers (otherwise ROM is needed)

	// CLx
	[ 0x18 ] = nonary("clc", clc),				// 00 011000
	[ 0x58 ] = nonary("cli", cli),				// 01 011000
	       	   		 				// 10 011000 is tya!!!
	[ 0xd8 ] = nonary("cld", cld),				// 11 011000

	// SEx (no SEV!)
	[ 0x38 ] = nonary("sec", sec),				// 00 111000
	[ 0x78 ] = nonary("sei", sei),				// 01 111000
	[ 0xb8 ] = nonary("clv", clv),				// 10 111000 Odd one out! Should logically be sev, but that doesn't exist.  switched place with tya
	[ 0xf8 ] = nonary("sed", sed),				// 11 111000

	// CMP
	[ 0xc1, izx ] = unary_izx("cmp", cmp_izx),		// 110 000 01
	[ 0xc5, zpg ] = unary_z("cmp", cmp),			// 110 001 01
	[ 0xc9, imm ] = unary_i("cmp", cmp),			// 110 010 01
	[ 0xcd, abs ] = unary_a("cmp", cmp),			// 110 011 01
	[ 0xd1, izy ] = unary_izy("cmp", cmp_izy),		// 110 100 01
	[ 0xd5, zpg ] = unary_zr("cmp", &*X, cmp_zpi),		// 110 101 01
	[ 0xd9, abs ] = unary_ar("cmp", &*Y, cmp_idx),		// 110 110 01
	[ 0xdd, abs ] = unary_ar("cmp", &*X, cmp_idx),		// 110 111 01

	// CPx
	[ 0xc0, imm ] = unary_i("cpy", cpy),			// 11 0 0 00 00
	[ 0xc4, zpg ] = unary_z("cpy", cpy),			// 11 0 0 01 00
	  	      		       				// 11 0 0 10 00 Odd one out 0xc8 iny
	[ 0xcc, abs ] = unary_a("cpy", cpy),			// 11 0 0 11 00
	[ 0xe0, imm ] = unary_i("cpx", cpx),			// 11 1 0 00 00
	[ 0xe4, zpg ] = unary_z("cpx", cpx),			// 11 1 0 01 00
	  	      		       				// 11 1 0 10 00 Odd one out 0xe8 inx
	[ 0xec, abs ] = unary_a("cpx", cpx),			// 11 1 0 11 00

	// DEC
	[ 0xc6, zpg ] = unary_z("dec", dec),
	[ 0xd6, zpg ] = unary_zr("dec", &*X, dec_zpi),
	[ 0xce, abs ] = unary_a("dec", dec),
	[ 0xde, abs ] = unary_ar("dec", &*X, dec_idx),

	// DEr
	[ 0xca ] = nonary("dex", dex),
	[ 0x88 ] = nonary("dey", dey),

	// EOR
	[ 0x41, izx ] = unary_izx("eor", eor_izx),		// 010 000 01
	[ 0x45, zpg ] = unary_z("eor", eor),			// 010 001 01
	[ 0x49, imm ] = unary_i("eor", eor),			// 010 010 01
	[ 0x4d, abs ] = unary_a("eor", eor),			// 010 011 01
	[ 0x51, izy ] = unary_izy("eor", eor_izy),		// 010 100 01
	[ 0x55, zpg ] = unary_zr("eor", &*X, eor_zpi),		// 010 101 01
	[ 0x59, abs ] = unary_ar("eor", &*Y, eor_idx),		// 010 110 01
	[ 0x5d, abs ] = unary_ar("eor", &*X, eor_idx),		// 010 111 01

	// INC
	[ 0xe6, zpg ] = unary_z("inc", inc),
	[ 0xf6, zpg ] = unary_zr("inc", &*X, inc_zpi),
	[ 0xee, abs ] = unary_a("inc", inc),
	[ 0xfe, abs ] = unary_ar("inc", &*X, inc_idx),

	// INr
	[ 0xe8 ] = nonary("inx", inx),
	[ 0xc8 ] = nonary("iny", iny),

	// JMP
	[ 0x4c, abs ] = unary_goto_a("jmp", nop_r),
	// Note that this wraps around the page when address is last byte on it.
 	[ 0x6c, ind ] = unary_goto_ind("jmp", nop_r), // FIXME: semantics

	// JSR
	[ 0x20, abs ] = unary_call_a("jsr", jsr),

	// LDA
	[ 0xa1, izx ] = unary_izx("lda", lda_izx),	// 101 000 01
	[ 0xa5, zpg ] = unary_z("lda", lda),		// 101 001 01
	[ 0xa9, imm ] = unary_i("lda", lda),		// 101 010 01
	[ 0xad, abs ] = unary_a("lda", lda),		// 101 011 01
	[ 0xb1, izy ] = unary_izy("lda", lda_izy),	// 101 100 01
	[ 0xb5, zpg ] = unary_zr("lda", &*X, lda_zpi),	// 101 101 01
	[ 0xb9, abs ] = unary_ar("lda", &*Y, lda_idx),	// 101 110 01
	[ 0xbd, abs ] = unary_ar("lda", &*X, lda_idx),	// 101 111 01

	// LDX
	[ 0xa2, imm ] = unary_i("ldx", ldx),		// 101 000 10
	[ 0xa6, zpg ] = unary_z("ldx", ldx),		// 101 001 10
	  	      		       			// 101 010 10 0xaa is tax
	[ 0xae, abs ] = unary_a("ldx", ldx),		// 101 011 10
	[ 0xb2 ] = nonary_ret("kil!", nop),		// 101 100 10*
	[ 0xb6, zpg ] = unary_zr("ldx", &*Y, ldx_zpi),	// 101 101 10
	  	      			     		// 101 110 10 0xba is tsx
	[ 0xbe, abs ] = unary_ar("ldx", &*Y, ldx_idx),	// 101 111 10

	// LDY
	[ 0xa4, zpg ] = unary_z("ldy", ldy),
	[ 0xa0, imm ] = unary_i("ldy", ldy),
	[ 0xb4, zpg ] = unary_zr("ldy", &*X, ldy_zpi),
	[ 0xac, abs ] = unary_a("ldy", ldy),
	[ 0xbc, abs ] = unary_ar("ldy", &*X, ldy_idx),

	// LSR
	[ 0x42 ] = nonary_ret("kil!", nop),		// 010 00 0 10*
	[ 0x4a ] = unary_r("lsr", &*A, lsr),		// 010 01 0 10
	[ 0x52 ] = nonary_ret("kil!", nop),		// 010 10 0 10*
	[ 0x5a ] = nonary("nop!", nop),	     		// 010 11 0 10
	[ 0x46, zpg ] = unary_z("lsr", lsr),		// 010 00 1 10 zzzz zzzz
	[ 0x4e, abs ] = unary_a("lsr", lsr),		// 010 01 1 10 aaaa aaaa
	[ 0x56, zpg ] = unary_zr("lsr", &*X, lsr_zpi),	// 010 10 1 10 zzzz zzzz
	[ 0x5e, abs ] = unary_ar("lsr", &*X, lsr_idx),	// 010 11 1 10 aaaa aaaa

	// NOP
	[ 0xea ] = nonary("nop", nop),

	// ORA
	[ 0x01, izx ] = unary_izx("ora", ora_izx),	// 000 000 01
	[ 0x05, zpg ] = unary_z("ora", ora),		// 000 001 01
	[ 0x09, imm ] = unary_i("ora", ora),		// 000 010 01
	[ 0x0d, abs ] = unary_a("ora", ora),		// 000 011 01
	[ 0x11, izy ] = unary_izy("ora", ora_izy),	// 000 100 01
	[ 0x15, zpg ] = unary_zr("ora", &*X, ora_zpi),	// 000 101 01
	[ 0x19, abs ] = unary_ar("ora", &*Y, ora_idx),	// 000 110 01
	[ 0x1d, abs ] = unary_ar("ora", &*X, ora_idx),	// 000 111 01

	// PHx, PLx
	[ 0x48 ] = nonary("pha", pha),
	[ 0x08 ] = nonary("php", php),
	[ 0x68 ] = nonary("pla", pla),
	[ 0x28 ] = nonary("plp", plp),

	// ROx
	[ 0x22 ] = nonary_ret("kil!", nop),    		// 0 0 1 00 0 10*
	[ 0x2a ] = unary_r("rol", &*A, rol),		// 0 0 1 01 0 10
	[ 0x32 ] = nonary_ret("kil!", nop),		// 0 0 1 10 0 10*
	[ 0x3a ] = nonary("nop!", nop),			// 0 0 1 11 0 10*
	[ 0x26, zpg ] = unary_z("rol", rol),		// 0 0 1 00 1 10
	[ 0x2e, abs ] = unary_a("rol", rol),		// 0 0 1 01 1 10
	[ 0x36, zpg ] = unary_zr("rol", &*X, rol_zpi),	// 0 0 1 10 1 10
	[ 0x3e, abs ] = unary_ar("rol", &*X, rol_idx),	// 0 0 1 11 1 10
	[ 0x62 ] = nonary_ret("kil!", nop),    		// 0 1 1 00 0 10*
	[ 0x6a ] = unary_r("ror", &*A, ror),		// 0 1 1 01 0 10
	[ 0x72 ] = nonary_ret("kil!", nop),		// 0 1 1 10 0 10*
	[ 0x7a ] = nonary("nop!", nop),			// 0 1 1 11 0 10*
	[ 0x66, zpg ] = unary_z("ror", ror),		// 0 1 1 00 1 10
	[ 0x6e, abs ] = unary_a("ror", ror),		// 0 1 1 01 1 10
	[ 0x76, zpg ] = unary_zr("ror", &*X, ror_zpi),	// 0 1 1 10 1 10
	[ 0x7e, abs ] = unary_ar("ror", &*X, ror_idx),	// 0 1 1 11 1 10

	// RTI
	[ 0x40 ] = nonary_ret("rti", nop),		// 0100 0000

	// RTS
	[ 0x60 ] = nonary_ret("rts", rts),		// 0110 0000

	// SBC
	[ 0xe1, izx ] = unary_izx("sbc", sbc_izx),	// 111 000 01
	[ 0xe5, zpg ] = unary_z("sbc", sbc),		// 111 001 01
	[ 0xe9, imm ] = unary_i("sbc", sbc),		// 111 010 01
	[ 0xed, abs ] = unary_a("sbc", sbc),		// 111 011 01
	[ 0xf1, izy ] = unary_izy("sbc", sbc_izy),	// 111 100 01
	[ 0xf5, zpg ] = unary_zr("sbc", &*X, sbc_zpi),	// 111 101 01
	[ 0xf9, abs ] = unary_ar("sbc", &*Y, sbc_idx),	// 111 110 01
	[ 0xfd, abs ] = unary_ar("sbc", &*X, sbc_idx),	// 111 111 01

	// STA
	[ 0x81, izx ] = unary_izx("sta", sta_izx),	// 100 000 01
	[ 0x85, zpg ] = unary_z("sta", sta),		// 100 001 01
	[ 0x89, imm ] = unary_i("nop!", nop_r),		// 100 010 01* illegal nop imm
	[ 0x8d, abs ] = unary_a("sta", sta),		// 100 011 01
	[ 0x91, izy ] = unary_izy("sta", sta_izy),	// 100 100 01
	[ 0x95, zpg ] = unary_zr("sta", &*X, sta_zpi),	// 100 101 01
	[ 0x99, abs ] = unary_ar("sta", &*Y, sta_idx),	// 100 110 01
	[ 0x9d, abs ] = unary_ar("sta", &*X, sta_idx),	// 100 111 01

	// STx
	[ 0x86, zpg ] = unary_z("stx", stx),		// 100 00 1 1 0
	[ 0x96, zpg ] = unary_zr("stx", &*Y, stx_zpi),	// 100 10 1 1 0
	[ 0x8e, abs ] = unary_a("stx", stx),		// 100 01 1 1 0
	[ 0x9e, abs ] = unary_ar("shx!", &*Y, nop_rr),  // 100 11 1 1 0* ill shx absy
	[ 0x84, zpg ] = unary_z("sty", sty),		// 100 00 1 0 0
	[ 0x8c, abs ] = unary_a("sty", sty),		// 100 01 1 0 0
	[ 0x94, zpg ] = unary_zr("sty", &*X, sty_zpi),	// 100 10 1 0 0
	[ 0x9c, abs ] = unary_ar("shy!", &*X, nop_rr),  // 100 11 1 0 0* ill shx absy

	// Txy - no pattern :-/
	[ 0xaa ] = nonary("tax", tax),			// 10 1 0 10 1 0
	[ 0xa8 ] = nonary("tay", tay),			// 10 1 0 10 0 0
	       	   		 			// 10 1 1 10 0 0  0xb8 clv !!!
	[ 0xba ] = nonary("tsx", tsx),			// 10 1 1 10 1 0
	[ 0x8a ] = nonary("txa", txa),			// 10 0 0 10 1 0
	       	   		 			// 10 0 0 10 0 0  0x88 dey
	[ 0x9a ] = nonary("txs", txs),			// 10 0 1 10 1 0
	[ 0x98 ] = nonary("tya", tya),			// 10 0 1 10 0 0

	// Illegal diverse.
	[ 0x80, imm ] = unary_i("nop!", nop_r),		// 1000 0000* iiii iiii
	[ 0x82, imm ] = unary_i("nop!", nop_r),		// 1000 0010* iiii iiii
	[ 0xc2, imm ] = unary_i("nop!", nop_r),		// 1100 0010* iiii iiii
	[ 0xd2 ] = nonary_ret("kil!", nop),		// 1101 0010*
	[ 0xe2, imm ] = unary_i("nop!", nop_r),		// 1100 0010* iiii iiii
	[ 0xf2 ] = nonary_ret("kil!", nop),		// 1111 0010*
	[ 0xd4, zpg ] = unary_zr("nop!", &*X, nop_rr),	// 1101 0100*
	[ 0xf4, zpg ] = unary_zr("nop!", &*X, nop_rr),	// 1101 0100*
	[ 0xda ] = nonary("nop!", nop),	     		// 110 11 010*
	[ 0xfa ] = nonary("nop!", nop),	     		// 111 11 010*
	[ 0xdc, abs ] = unary_ar("nop!", &*X, nop_rr),	// 010 111 11*
	[ 0xfc, abs ] = unary_ar("nop!", &*X, nop_rr),	// 010 111 11*
	[ 0x04, zpg ] = unary_z("nop!", nop_r),
	[ 0x0c, abs ] = unary_a("nop!", nop_r),
	[ 0x14, zpg ] = unary_zr("nop!", &*X, nop_rr),
	[ 0x1c, abs ] = unary_ar("nop!", &*X, nop_rr),
	[ 0x34, zpg ] = unary_zr("nop!", &*X, nop_rr),
	[ 0x3c, abs ] = unary_ar("nop!", &*X, nop_rr),
	[ 0x44, zpg ] = unary_z("nop!", nop_r),
	[ 0x54, zpg ] = unary_zr("nop!", &*X, nop_rr),
	[ 0x5c, abs ] = unary_ar("nop!", &*X, nop_rr),
	[ 0x64, zpg ] = unary_z("nop!", nop_r),
	[ 0x74, zpg ] = unary_zr("nop!", &*X, nop_rr),
	[ 0x7c, abs ] = unary_ar("nop!", &*X, nop_rr),
	[ 0x92 ] = nonary_ret("kil!", nop),


	// SLO (ASL + ORA), ANC (AND + [ASL]carry only)
	[ 0x03, izx ] = unary_izx("slo!", nop_r),	// 000 000 11
	[ 0x07, zpg ] = unary_z("slo!", nop_r),		// 000 001 11
	[ 0x0b, imm ] = unary_i("anc!", nop_r),		// 000 010 11 ANC!
	[ 0x0f, abs ] = unary_a("slo!", nop_r),		// 000 011 11
	[ 0x13, izy ] = unary_izy("slo!", nop_r),	// 000 100 11
	[ 0x17, zpg ] = unary_zr("slo!", &*X, nop_rr),	// 000 101 11
	[ 0x1b, abs ] = unary_ar("slo!", &*Y, nop_rr),	// 000 110 11
	[ 0x1f, abs ] = unary_ar("slo!", &*X, nop_rr),	// 000 111 11

	// RLA (ROL + AND), ANC (AND + [ROL]carry only)
	[ 0x23, izx ] = unary_izx("rla!", nop_r),	// 001 000 11
	[ 0x27, zpg ] = unary_z("rla!", nop_r),		// 001 001 11
	[ 0x2b, imm ] = unary_i("anc!", nop_r),		// 001 010 11 ANC!
	[ 0x2f, abs ] = unary_a("rla!", nop_r),		// 001 011 11
	[ 0x33, izy ] = unary_izy("rla!", nop_r),	// 001 100 11
	[ 0x37, zpg ] = unary_zr("rla!", &*X, nop_rr),	// 001 101 11
	[ 0x3b, abs ] = unary_ar("rla!", &*Y, nop_rr),	// 001 110 11
	[ 0x3f, abs ] = unary_ar("rla!", &*X, nop_rr),	// 001 111 11

	// SRE (ASR + EOR), ALR (AND + LSR)
	[ 0x43, izx ] = unary_izx("sre!", nop_r),	// 010 000 11
	[ 0x47, zpg ] = unary_z("sre!", nop_r),		// 010 001 11
	[ 0x4b, imm ] = unary_i("alr!", nop_r),		// 010 010 11 ALR!
	[ 0x4f, abs ] = unary_a("sre!", nop_r),		// 010 011 11
	[ 0x53, izy ] = unary_izy("sre!", nop_r),	// 010 100 11
	[ 0x57, zpg ] = unary_zr("sre!", &*X, nop_rr),	// 010 101 11
	[ 0x5b, abs ] = unary_ar("sre!", &*Y, nop_rr),	// 010 110 11
	[ 0x5f, abs ] = unary_ar("sre!", &*X, nop_rr),	// 010 111 11

	// RRA (ROR + ADC), ARR (AND + ROR)
	// note to ARR: part of this command are some ADC mechanisms.
	// following effects appear after AND but before ROR: the V-Flag
	// is set according to (A and #{imm})+#{imm}, bit 0 does NOT go
	// into carry, but bit 7 is exchanged with the carry.
	[ 0x63, izx ] = unary_izx("rra!", nop_r),	// 011 000 11
	[ 0x67, zpg ] = unary_z("rra!", nop_r),		// 011 001 11
	[ 0x6b, imm ] = unary_i("arr!", nop_r),		// 011 010 11 ARR!
	[ 0x6f, abs ] = unary_a("rra!", nop_r),		// 011 011 11
	[ 0x73, izy ] = unary_izy("rra!", nop_r),	// 011 100 11
	[ 0x77, zpg ] = unary_zr("rra!", &*X, nop_rr),	// 011 101 11
	[ 0x7b, abs ] = unary_ar("rra!", &*Y, nop_rr),	// 011 110 11
	[ 0x7f, abs ] = unary_ar("rra!", &*X, nop_rr),	// 011 111 11

	// SAX (store A&X into {adr})
	// AHX stores A&X&H into {adr}
	// XAA? TXA + AND #{imm}
	// TAS stores A&X into S and A&X&H into {adr}
	[ 0x83, izx ] = unary_izx("sax!", nop_r),	// 100 000 11
	[ 0x87, zpg ] = unary_z("sax!", nop_r),		// 100 001 11
	[ 0x8b, imm ] = unary_i("xaa?", nop_r),		// 100 010 11 XAA!
	[ 0x8f, abs ] = unary_a("sax!", nop_r),		// 100 011 11
	[ 0x93, izy ] = unary_izy("ahx!", nop_r),	// 100 100 11
	[ 0x97, zpg ] = unary_zr("sax!", &*Y, nop_rr),	// 100 101 11
	[ 0x9b, abs ] = unary_ar("tas!", &*Y, nop_rr),	// 100 110 11
	[ 0x9f, abs ] = unary_ar("ahx!", &*Y, nop_rr),	// 100 111 11

	// LAX (LDA + TAX), LAS (stores {adr}&S into A, X and S)
	[ 0xa3, izx ] = unary_izx("lax!", nop_r),	// 101 000 11
	[ 0xa7, zpg ] = unary_z("lax!", nop_r),		// 101 001 11
	[ 0xab, imm ] = unary_i("lax?", nop_r),		// 101 010 11
	[ 0xaf, abs ] = unary_a("lax!", nop_r),		// 101 011 11
	[ 0xb3, izy ] = unary_izy("lax!", nop_r),	// 101 100 11
	[ 0xb7, zpg ] = unary_zr("lax!", &*Y, nop_rr),	// 101 101 11
	[ 0xbb, abs ] = unary_ar("las!", &*Y, nop_rr),	// 101 110 11 LAS
	[ 0xbf, abs ] = unary_ar("lax!", &*Y, nop_rr),	// 101 111 11

	// DCP, AXS
	[ 0xc3, izx ] = unary_izx("dcp!", nop_r),	// 110 000 11
	[ 0xc7, zpg ] = unary_z("dcp!", nop_r),		// 110 001 11
	[ 0xcb, imm ] = unary_i("axs!", nop_r),		// 110 010 11 AXS!
	[ 0xcf, abs ] = unary_a("dcp!", nop_r),		// 110 011 11
	[ 0xd3, izy ] = unary_izy("dcp!", nop_r),	// 110 100 11
	[ 0xd7, zpg ] = unary_zr("dcp!", &*X, nop_rr),	// 110 101 11
	[ 0xdb, abs ] = unary_ar("dcp!", &*Y, nop_rr),	// 110 110 11
	[ 0xdf, abs ] = unary_ar("dcp!", &*X, nop_rr),	// 110 111 11

	// ISC, SBC
	[ 0xe3, izx ] = unary_izx("isc!", nop_r),	// 111 000 11
	[ 0xe7, zpg ] = unary_z("isc!", nop_r),		// 111 001 11
	[ 0xeb, imm ] = unary_i("sbc!", nop_r),		// 111 010 11 SBC!
	[ 0xef, abs ] = unary_a("isc!", nop_r),		// 111 011 11
	[ 0xf3, izy ] = unary_izy("isc!", nop_r),	// 111 100 11
	[ 0xf7, zpg ] = unary_zr("isc!", &*X, nop_rr),	// 111 101 11
	[ 0xfb, abs ] = unary_ar("isc!", &*Y, nop_rr),	// 111 110 11
	[ 0xff, abs ] = unary_ar("isc!", &*X, nop_rr),	// 111 111 11

 	// catch all, FIXME: Add at least the args for illegal opcodes.
        _ = nonary("unk", nop)
    )
}
