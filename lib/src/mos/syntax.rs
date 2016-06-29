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

use mos::semantic::*;
use mos::*;

use {
    State,
    Disassembler,
    Rvalue,
};

use std::sync::Arc;

/* use value::{Lvalue,Rvalue,ToRvalue};
use guard::Guard;
use std::num::Wrapping;
use super::*;
*/

/* Tests:
   http://visual6502.org/wiki/index.php?title=6502TestPrograms
   specifically:
   https://github.com/kingcons/cl-6502/blob/b0087903428ec2a3794ba4219494005174d1b09f/tests/6502_functional_test.a65
   http://www.6502.org/tutorials/deaimal_mode.html
   http://6502org.wikidot.com/errata-other-deamode
*/

#[allow(overflowing_literals)]
pub fn disassembler() -> Arc<Disassembler<Mos>> {
    let imm8 = new_disassembler!(Mos =>
        [ "imm@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg = Some(Rvalue::new_u8(st.get_group("imm") as u8));
            true
        });

    let imm16 = new_disassembler!(Mos =>
        [ "immlo@........", "immhi@........" ] = |st: &mut State<Mos>| {
            st.configuration.arg = Some(Rvalue::new_u16(st.get_group("immlo") as u16 | ((st.get_group("immhi") as u16) << 8)));
            true
        });

    let rel = new_disassembler!(Mos =>
        [ "imm@........" ] = |st: &mut State<Mos>| {
            let rel = st.get_group("imm") as i8;

            st.configuration.rel = Some(if rel >= 0 { rel as i16 } else { rel as i16 | 0xff00 });
            true
        });

    // FIXME: Add illegal opcodes.
    new_disassembler!(Mos =>
        // ADC
        [ 0x61, imm8 ] = zpage_index("adc", rreil_lvalue!{ X:8 }, adc),	// 011 000 01 xxxx xxxx
        [ 0x65, imm8 ] = zpage("adc", adc),		// 011 001 01 zzzz zzzz
        [ 0x69, imm8 ] = immediate("adc", adc),		// 011 010 01 iiii iiii
        [ 0x6d, imm16 ] = absolute("adc", adc),		// 011 011 01 aaaa aaaa
        [ 0x71, imm8 ] = zpage_index("adc", rreil_lvalue!{ Y:8 }, adc),	// 011 100 01 yyyy yyyy
        [ 0x75, imm8 ] = zpage_offset("adc", &*X, adc),	// 011 101 01 zzzz zzzz,X
        [ 0x79, imm16 ] = absolute_offset("adc", &*Y, adc),	// 011 110 01 aaaa aaaa,Y
        [ 0x7d, imm16 ] = absolute_offset("adc", &*X, adc),	// 011 111 01 aaaa aaaa,X

        // AND
        [ 0x21, imm8 ] = zpage_index("and", rreil_lvalue!{ X:8 }, and),	// 001 000 01 xxxx xxxx
        [ 0x25, imm8 ] = zpage("and", and),		// 001 001 01 zzzz zzzz
        [ 0x29, imm8 ] = immediate("and", and),		// 001 010 01 iiii iiii
        [ 0x2d, imm16 ] = absolute("and", and),		// 001 011 01 aaaa aaaa
        [ 0x31, imm8 ] = zpage_index("and", rreil_lvalue!{ Y:8 }, and),	// 001 100 01 yyyy yyyy
        [ 0x35, imm8 ] = zpage_offset("and", &*X, and),	// 001 101 01 zzzz zzzz,X
        [ 0x39, imm16 ] = absolute_offset("and", &*Y, and),	// 001 110 01 aaaa aaaa,X
        [ 0x3d, imm16 ] = absolute_offset("and", &*X, and),	// 001 111 01 aaaa aaaa,Y

        // ASL
        [ 0x02 ] = ret("kil!"),		// 000 00 010*
        [ 0x0a ] = implied("asl", &*A, asl),		// 000 01 010  A
        [ 0x12 ] = ret("kil!"),		// 000 10 010*
        [ 0x1a ] = nonary("nop!", nop),	     		// 000 11 010*
        // ASL arg
        [ 0x06, imm8 ] = zpage("asl", asl),		// 000 00 110  zzzz zzzz
        [ 0x0e, imm16 ] = absolute("asl", asl),		// 000 01 110  aaaa aaaa
        [ 0x16, imm8 ] = zpage_offset("asl", &*X, asl),	// 000 10 110  zzzz zzzz,X
        [ 0x1e, imm16 ] = absolute_offset("asl", &*X, asl),   // 000 11 110  aaaa aaaa,X

        // BCx
        [ 0x90, rel ] = branch("bcc", &*C, false),	// 1001 0000 rrrr rrrr
        [ 0xb0, rel ] = branch("bcs", &*C, true),	// 1010 0000 rrrr rrrr

        // BEQ, BNE
        [ 0xf0, rel ] = branch("beq", &*Z, true),	// 1111 0000 rrrr rrrr
        [ 0xd0, rel ] = branch("bne", &*Z, false), 	// 1100 0000 rrrr rrrr

        // BIT
        [ 0x24, imm8 ] = zpage("bit", bit),			// 0010 0 100 zzzz zzzz
        [ 0x2c, imm16 ] = absolute("bit", bit),			// 0010 1 100 aaaa aaaa

        // BMI, BPL
        [ 0x30, rel ] = branch("bmi", &*N, true),	// 0011 0000
        [ 0x10, rel ] = branch("bpl", &*N, false),	// 0001 0000

        // BVC, BVS
        [ 0x50, rel ] = branch("bvc", &*V, false),	// 0101 0000
        [ 0x70, rel ] = branch("bvs", &*V, true),	// 0111 0000

        // BRK
        [ 0x00, imm8 ] = nonary("brk", brk),			// 0000 0000 FIXME: Maybe clobber the registers (otherwise ROM is needed)

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
        [ 0xc1, imm8 ] = zpage_index("cmp", rreil_lvalue!{ X:8 }, cpa),		// 110 000 01
        [ 0xc5, imm8 ] = zpage("cmp", cpa),			// 110 001 01
        [ 0xc9, imm8 ] = immediate("cmp", cpa),			// 110 010 01
        [ 0xcd, imm16 ] = absolute("cmp", cpa),			// 110 011 01
        [ 0xd1, imm8 ] = zpage_index("cmp", rreil_lvalue!{ Y:8 }, cpa),		// 110 100 01
        [ 0xd5, imm8 ] = zpage_offset("cmp", &*X, cpa),		// 110 101 01
        [ 0xd9, imm16 ] = absolute_offset("cmp", &*Y, cpa),		// 110 110 01
        [ 0xdd, imm16 ] = absolute_offset("cmp", &*X, cpa),		// 110 111 01

        // CPx
        [ 0xc0, imm8 ] = immediate("cpy", cpy),			// 11 0 0 00 00
        [ 0xc4, imm8 ] = zpage("cpy", cpy),			// 11 0 0 01 00
        // 11 0 0 10 00 Odd one out 0xc8 iny
        [ 0xcc, imm16 ] = absolute("cpy", cpy),			// 11 0 0 11 00
        [ 0xe0, imm8 ] = immediate("cpx", cpx),			// 11 1 0 00 00
        [ 0xe4, imm8 ] = zpage("cpx", cpx),			// 11 1 0 01 00
        // 11 1 0 10 00 Odd one out 0xe8 inx
        [ 0xec, imm16 ] = absolute("cpx", cpx),			// 11 1 0 11 00

        // DEC
        [ 0xc6, imm8 ] = zpage("dec", dea),
        [ 0xd6, imm8 ] = zpage_offset("dec", &*X, dea),
        [ 0xce, imm16 ] = absolute("dec", dea),
        [ 0xde, imm16 ] = absolute_offset("dec", &*X, dea),

        // DEr
        [ 0xca ] = nonary("dex", dex),
        [ 0x88 ] = nonary("dey", dey),

        // EOR
        [ 0x41, imm8 ] = zpage_index("eor", rreil_lvalue!{ X:8 }, eor),		// 010 000 01
        [ 0x45, imm8 ] = zpage("eor", eor),			// 010 001 01
        [ 0x49, imm8 ] = immediate("eor", eor),			// 010 010 01
        [ 0x4d, imm16 ] = absolute("eor", eor),			// 010 011 01
        [ 0x51, imm8 ] = zpage_index("eor", rreil_lvalue!{ Y:8 }, eor),		// 010 100 01
        [ 0x55, imm8 ] = zpage_offset("eor", &*X, eor),		// 010 101 01
        [ 0x59, imm16 ] = absolute_offset("eor", &*Y, eor),		// 010 110 01
        [ 0x5d, imm16 ] = absolute_offset("eor", &*X, eor),		// 010 111 01

        // INC
        [ 0xe6, imm8 ] = zpage("inc", ina),
        [ 0xf6, imm8 ] = zpage_offset("inc", &*X, ina),
        [ 0xee, imm16 ] = absolute("inc", ina),
        [ 0xfe, imm16 ] = absolute_offset("inc", &*X, ina),

        // INr
        [ 0xe8 ] = nonary("inx", inx),
        [ 0xc8 ] = nonary("iny", iny),

        // JMP
        [ 0x4c, imm16 ] = jmp_direct,
        // FIXME: Note that this wraps around the page when address is last byte on it.
        [ 0x6c, imm16 ] = jmp_indirect, // FIXME: semantics

        // JSR
        [ 0x20, imm16 ] = jsr,

        // LDA
        [ 0xa1, imm8 ] = zpage_index("lda", rreil_lvalue!{ X:8 }, lda),	// 101 000 01
        [ 0xa5, imm8 ] = zpage("lda", lda),		// 101 001 01
        [ 0xa9, imm8 ] = immediate("lda", lda),		// 101 010 01
        [ 0xad, imm16 ] = absolute("lda", lda),		// 101 011 01
        [ 0xb1, imm8 ] = zpage_index("lda", rreil_lvalue!{ Y:8 }, lda),	// 101 100 01
        [ 0xb5, imm8 ] = zpage_offset("lda", &*X, lda),	// 101 101 01
        [ 0xb9, imm16 ] = absolute_offset("lda", &*Y, lda),	// 101 110 01
        [ 0xbd, imm16 ] = absolute_offset("lda", &*X, lda),	// 101 111 01

        // LDX
        [ 0xa2, imm8 ] = immediate("ldx", ldx),		// 101 000 10
        [ 0xa6, imm8 ] = zpage("ldx", ldx),		// 101 001 10
        // 101 010 10 0xaa is tax
        [ 0xae, imm16 ] = absolute("ldx", ldx),		// 101 011 10
        [ 0xb2 ] = ret("kil!"),		// 101 100 10*
        [ 0xb6, imm8 ] = zpage_offset("ldx", &*Y, ldx),	// 101 101 10
        // 101 110 10 0xba is tsx
        [ 0xbe, imm16 ] = absolute_offset("ldx", &*Y, ldx),	// 101 111 10

        // LDY
        [ 0xa4, imm8 ] = zpage("ldy", ldy),
        [ 0xa0, imm8 ] = immediate("ldy", ldy),
        [ 0xb4, imm8 ] = zpage_offset("ldy", &*X, ldy),
        [ 0xac, imm16 ] = absolute("ldy", ldy),
        [ 0xbc, imm16 ] = absolute_offset("ldy", &*X, ldy),

        // LSR
        [ 0x42 ] = ret("kil!"),		// 010 00 0 10*
        [ 0x4a ] = implied("lsr", &*A, lsr),		// 010 01 0 10
        [ 0x52 ] = ret("kil!"),		// 010 10 0 10*
        [ 0x5a ] = nonary("nop!", nop),	     		// 010 11 0 10
        [ 0x46, imm8 ] = zpage("lsr", lsr),		// 010 00 1 10 zzzz zzzz
        [ 0x4e, imm16 ] = absolute("lsr", lsr),		// 010 01 1 10 aaaa aaaa
        [ 0x56, imm8 ] = zpage_offset("lsr", &*X, lsr),	// 010 10 1 10 zzzz zzzz
        [ 0x5e, imm16 ] = absolute_offset("lsr", &*X, lsr),	// 010 11 1 10 aaaa aaaa

        // NOP
        [ 0xea ] = nonary("nop", nop),

        // ORA
        [ 0x01, imm8 ] = zpage_index("ora", rreil_lvalue!{ X:8 }, ora),	// 000 000 01
        [ 0x05, imm8 ] = zpage("ora", ora),		// 000 001 01
        [ 0x09, imm8 ] = immediate("ora", ora),		// 000 010 01
        [ 0x0d, imm16 ] = absolute("ora", ora),		// 000 011 01
        [ 0x11, imm8 ] = zpage_index("ora", rreil_lvalue!{ Y:8 }, ora),	// 000 100 01
        [ 0x15, imm8 ] = zpage_offset("ora", &*X, ora),	// 000 101 01
        [ 0x19, imm16 ] = absolute_offset("ora", &*Y, ora),	// 000 110 01
        [ 0x1d, imm16 ] = absolute_offset("ora", &*X, ora),	// 000 111 01

        // PHx, PLx
        [ 0x48 ] = nonary("pha", pha),
        [ 0x08 ] = nonary("php", php),
        [ 0x68 ] = nonary("pla", pla),
        [ 0x28 ] = nonary("plp", plp),

        // ROx
        [ 0x22 ] = ret("kil!"),    		// 0 0 1 00 0 10*
        [ 0x2a ] = implied("rol", &*A, rol),		// 0 0 1 01 0 10
        [ 0x32 ] = ret("kil!"),		// 0 0 1 10 0 10*
        [ 0x3a ] = nonary("nop!", nop),			// 0 0 1 11 0 10*
        [ 0x26, imm8 ] = zpage("rol", rol),		// 0 0 1 00 1 10
        [ 0x2e, imm16 ] = absolute("rol", rol),		// 0 0 1 01 1 10
        [ 0x36, imm8 ] = zpage_offset("rol", &*X, rol),	// 0 0 1 10 1 10
        [ 0x3e, imm16 ] = absolute_offset("rol", &*X, rol),	// 0 0 1 11 1 10
        [ 0x62 ] = ret("kil!"),    		// 0 1 1 00 0 10*
        [ 0x6a ] = implied("ror", &*A, ror),		// 0 1 1 01 0 10
        [ 0x72 ] = ret("kil!"),		// 0 1 1 10 0 10*
        [ 0x7a ] = nonary("nop!", nop),			// 0 1 1 11 0 10*
        [ 0x66, imm8 ] = zpage("ror", ror),		// 0 1 1 00 1 10
        [ 0x6e, imm16 ] = absolute("ror", ror),		// 0 1 1 01 1 10
        [ 0x76, imm8 ] = zpage_offset("ror", &*X, ror),	// 0 1 1 10 1 10
        [ 0x7e, imm16 ] = absolute_offset("ror", &*X, ror),	// 0 1 1 11 1 10

        // RTI
        [ 0x40 ] = ret("rti"),		// 0100 0000

        // RTS
        [ 0x60 ] = ret("rts"),		// 0110 0000

        // SBC
        [ 0xe1, imm8 ] = zpage_index("sbc", rreil_lvalue!{ X:8 }, sbc),	// 111 000 01
        [ 0xe5, imm8 ] = zpage("sbc", sbc),		// 111 001 01
        [ 0xe9, imm8 ] = immediate("sbc", sbc),		// 111 010 01
        [ 0xed, imm16 ] = absolute("sbc", sbc),		// 111 011 01
        [ 0xf1, imm8 ] = zpage_index("sbc", rreil_lvalue!{ Y:8 }, sbc),	// 111 100 01
        [ 0xf5, imm8 ] = zpage_offset("sbc", &*X, sbc),	// 111 101 01
        [ 0xf9, imm16 ] = absolute_offset("sbc", &*Y, sbc),	// 111 110 01
        [ 0xfd, imm16 ] = absolute_offset("sbc", &*X, sbc),	// 111 111 01

        // STA
        [ 0x81, imm8 ] = zpage_index("sta", rreil_lvalue!{ X:8 }, sta),	// 100 000 01
        [ 0x85, imm8 ] = zpage("sta", sta),		// 100 001 01
        [ 0x89, imm8 ] = immediate("nop!", nop_r),		// 100 010 01* illegal nop imm
        [ 0x8d, imm16 ] = absolute("sta", sta),		// 100 011 01
        [ 0x91, imm8 ] = zpage_index("sta", rreil_lvalue!{ Y:8 }, sta),	// 100 100 01
        [ 0x95, imm8 ] = zpage_offset("sta", &*X, sta),	// 100 101 01
        [ 0x99, imm16 ] = absolute_offset("sta", &*Y, sta),	// 100 110 01
        [ 0x9d, imm16 ] = absolute_offset("sta", &*X, sta),	// 100 111 01

        // STx
        [ 0x86, imm8 ] = zpage("stx", stx),		// 100 00 1 1 0
        [ 0x96, imm8 ] = zpage_offset("stx", &*Y, stx),	// 100 10 1 1 0
        [ 0x8e, imm16 ] = absolute("stx", stx),		// 100 01 1 1 0
        [ 0x9e, imm16 ] = absolute_offset("shx!", &*Y, nop_r),  // 100 11 1 1 0* ill shx imm16y
        [ 0x84, imm8 ] = zpage("sty", sty),		// 100 00 1 0 0
        [ 0x8c, imm16 ] = absolute("sty", sty),		// 100 01 1 0 0
        [ 0x94, imm8 ] = zpage_offset("sty", &*X, sty),	// 100 10 1 0 0
        [ 0x9c, imm16 ] = absolute_offset("shy!", &*X, nop_r),  // 100 11 1 0 0* ill shx imm16y

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
        [ 0x80, imm8 ] = immediate("nop!", nop_r),		// 1000 0000* iiii iiii
        [ 0x82, imm8 ] = immediate("nop!", nop_r),		// 1000 0010* iiii iiii
        [ 0xc2, imm8 ] = immediate("nop!", nop_r),		// 1100 0010* iiii iiii
        [ 0xd2 ] = ret("kil!"),		// 1101 0010*
        [ 0xe2, imm8 ] = immediate("nop!", nop_r),		// 1100 0010* iiii iiii
        [ 0xf2 ] = ret("kil!"),		// 1111 0010*
        [ 0xd4, imm8 ] = zpage_offset("nop!", &*X, nop_r),	// 1101 0100*
        [ 0xf4, imm8 ] = zpage_offset("nop!", &*X, nop_r),	// 1101 0100*
        [ 0xda ] = nonary("nop!", nop),	     		// 110 11 010*
        [ 0xfa ] = nonary("nop!", nop),	     		// 111 11 010*
        [ 0xdc, imm16 ] = absolute_offset("nop!", &*X, nop_r),	// 010 111 11*
        [ 0xfc, imm16 ] = absolute_offset("nop!", &*X, nop_r),	// 010 111 11*
        [ 0x04, imm8 ] = zpage("nop!", nop_r),
        [ 0x0c, imm16 ] = absolute("nop!", nop_r),
        [ 0x14, imm8 ] = zpage_offset("nop!", &*X, nop_r),
        [ 0x1c, imm16 ] = absolute_offset("nop!", &*X, nop_r),
        [ 0x34, imm8 ] = zpage_offset("nop!", &*X, nop_r),
        [ 0x3c, imm16 ] = absolute_offset("nop!", &*X, nop_r),
        [ 0x44, imm8 ] = zpage("nop!", nop_r),
        [ 0x54, imm8 ] = zpage_offset("nop!", &*X, nop_r),
        [ 0x5c, imm16 ] = absolute_offset("nop!", &*X, nop_r),
        [ 0x64, imm8 ] = zpage("nop!", nop_r),
        [ 0x74, imm8 ] = zpage_offset("nop!", &*X, nop_r),
        [ 0x7c, imm16 ] = absolute_offset("nop!", &*X, nop_r),
        [ 0x92 ] = ret("kil!"),


        // SLO (ASL + ORA), ANC (AND + [ASL]carry only)
        [ 0x03, imm8 ] = zpage_index("slo!", rreil_lvalue!{ X:8 }, nop_r),	// 000 000 11
        [ 0x07, imm8 ] = zpage("slo!", nop_r),		// 000 001 11
        [ 0x0b, imm8 ] = immediate("anc!", nop_r),		// 000 010 11 ANC!
        [ 0x0f, imm16 ] = absolute("slo!", nop_r),		// 000 011 11
        [ 0x13, imm8 ] = zpage_index("slo!", rreil_lvalue!{ Y:8 }, nop_r),	// 000 100 11
        [ 0x17, imm8 ] = zpage_offset("slo!", &*X, nop_r),	// 000 101 11
        [ 0x1b, imm16 ] = absolute_offset("slo!", &*Y, nop_r),	// 000 110 11
        [ 0x1f, imm16 ] = absolute_offset("slo!", &*X, nop_r),	// 000 111 11

        // RLA (ROL + AND), ANC (AND + [ROL]carry only)
        [ 0x23, imm8 ] = zpage_index("rla!", rreil_lvalue!{ X:8 }, nop_r),	// 001 000 11
        [ 0x27, imm8 ] = zpage("rla!", nop_r),		// 001 001 11
        [ 0x2b, imm8 ] = immediate("anc!", nop_r),		// 001 010 11 ANC!
        [ 0x2f, imm16 ] = absolute("rla!", nop_r),		// 001 011 11
        [ 0x33, imm8 ] = zpage_index("rla!", rreil_lvalue!{ Y:8 }, nop_r),	// 001 100 11
        [ 0x37, imm8 ] = zpage_offset("rla!", &*X, nop_r),	// 001 101 11
        [ 0x3b, imm16 ] = absolute_offset("rla!", &*Y, nop_r),	// 001 110 11
        [ 0x3f, imm16 ] = absolute_offset("rla!", &*X, nop_r),	// 001 111 11

        // SRE (ASR + EOR), ALR (AND + LSR)
        [ 0x43, imm8 ] = zpage_index("sre!", rreil_lvalue!{ X:8 }, nop_r),	// 010 000 11
        [ 0x47, imm8 ] = zpage("sre!", nop_r),		// 010 001 11
        [ 0x4b, imm8 ] = immediate("alr!", nop_r),		// 010 010 11 ALR!
        [ 0x4f, imm16 ] = absolute("sre!", nop_r),		// 010 011 11
        [ 0x53, imm8 ] = zpage_index("sre!", rreil_lvalue!{ Y:8 }, nop_r),	// 010 100 11
        [ 0x57, imm8 ] = zpage_offset("sre!", &*X, nop_r),	// 010 101 11
        [ 0x5b, imm16 ] = absolute_offset("sre!", &*Y, nop_r),	// 010 110 11
        [ 0x5f, imm16 ] = absolute_offset("sre!", &*X, nop_r),	// 010 111 11

        // RRA (ROR + ADC), ARR (AND + ROR)
        // note to ARR: part of this command are some ADC mechanisms.
        // following effects appear after AND but before ROR: the V-Flag
        // is set according to (A and #{imm})+#{imm}, bit 0 does NOT go
        // into carry, but bit 7 is exchanged with the carry.
        [ 0x63, imm8 ] = zpage_index("rra!", rreil_lvalue!{ X:8 }, nop_r),	// 011 000 11
        [ 0x67, imm8 ] = zpage("rra!", nop_r),		// 011 001 11
        [ 0x6b, imm8 ] = immediate("arr!", nop_r),		// 011 010 11 ARR!
        [ 0x6f, imm16 ] = absolute("rra!", nop_r),		// 011 011 11
        [ 0x73, imm8 ] = zpage_index("rra!", rreil_lvalue!{ Y:8 }, nop_r),	// 011 100 11
        [ 0x77, imm8 ] = zpage_offset("rra!", &*X, nop_r),	// 011 101 11
        [ 0x7b, imm16 ] = absolute_offset("rra!", &*Y, nop_r),	// 011 110 11
        [ 0x7f, imm16 ] = absolute_offset("rra!", &*X, nop_r),	// 011 111 11

        // SAX (store A&X into {adr})
        // AHX stores A&X&H into {adr}
        // XAA? TXA + AND #{imm}
        // TAS stores A&X into S and A&X&H into {adr}
        [ 0x83, imm8 ] = zpage_index("sax!", rreil_lvalue!{ X:8 }, nop_r),	// 100 000 11
        [ 0x87, imm8 ] = zpage("sax!", nop_r),		// 100 001 11
        [ 0x8b, imm8 ] = immediate("xaa?", nop_r),		// 100 010 11 XAA!
        [ 0x8f, imm16 ] = absolute("sax!", nop_r),		// 100 011 11
        [ 0x93, imm8 ] = zpage_index("ahx!", rreil_lvalue!{ Y:8 }, nop_r),	// 100 100 11
        [ 0x97, imm8 ] = zpage_offset("sax!", &*Y, nop_r),	// 100 101 11
        [ 0x9b, imm16 ] = absolute_offset("tas!", &*Y, nop_r),	// 100 110 11
        [ 0x9f, imm16 ] = absolute_offset("ahx!", &*Y, nop_r),	// 100 111 11

        // LAX (LDA + TAX), LAS (stores {adr}&S into A, X and S)
        [ 0xa3, imm8 ] = zpage_index("lax!", rreil_lvalue!{ X:8 }, nop_r),	// 101 000 11
        [ 0xa7, imm8 ] = zpage("lax!", nop_r),		// 101 001 11
        [ 0xab, imm8 ] = immediate("lax?", nop_r),		// 101 010 11
        [ 0xaf, imm16 ] = absolute("lax!", nop_r),		// 101 011 11
        [ 0xb3, imm8 ] = zpage_index("lax!", rreil_lvalue!{ Y:8 }, nop_r),	// 101 100 11
        [ 0xb7, imm8 ] = zpage_offset("lax!", &*Y, nop_r),	// 101 101 11
        [ 0xbb, imm16 ] = absolute_offset("las!", &*Y, nop_r),	// 101 110 11 LAS
        [ 0xbf, imm16 ] = absolute_offset("lax!", &*Y, nop_r),	// 101 111 11

        // DCP, AXS
        [ 0xc3, imm8 ] = zpage_index("dcp!", rreil_lvalue!{ X:8 }, nop_r),	// 110 000 11
        [ 0xc7, imm8 ] = zpage("dcp!", nop_r),		// 110 001 11
        [ 0xcb, imm8 ] = immediate("axs!", nop_r),		// 110 010 11 AXS!
        [ 0xcf, imm16 ] = absolute("dcp!", nop_r),		// 110 011 11
        [ 0xd3, imm8 ] = zpage_index("dcp!", rreil_lvalue!{ Y:8 }, nop_r),	// 110 100 11
        [ 0xd7, imm8 ] = zpage_offset("dcp!", &*X, nop_r),	// 110 101 11
        [ 0xdb, imm16 ] = absolute_offset("dcp!", &*Y, nop_r),	// 110 110 11
        [ 0xdf, imm16 ] = absolute_offset("dcp!", &*X, nop_r),	// 110 111 11

        // ISC, SBC
        [ 0xe3, imm8 ] = zpage_index("isc!", rreil_lvalue!{ X:8 }, nop_r),	// 111 000 11
        [ 0xe7, imm8 ] = zpage("isc!", nop_r),		// 111 001 11
        [ 0xeb, imm8 ] = immediate("sbc!", nop_r),		// 111 010 11 SBC!
        [ 0xef, imm16 ] = absolute("isc!", nop_r),		// 111 011 11
        [ 0xf3, imm8 ] = zpage_index("isc!", rreil_lvalue!{ Y:8 }, nop_r),	// 111 100 11
        [ 0xf7, imm8 ] = zpage_offset("isc!", &*X, nop_r),	// 111 101 11
        [ 0xfb, imm16 ] = absolute_offset("isc!", &*Y, nop_r),	// 111 110 11
        [ 0xff, imm16 ] = absolute_offset("isc!", &*X, nop_r),	// 111 111 11

        // catch all, FIXME: Add at least the args for illegal opcodes.
        _ = nonary("unk", nop)
    )
}
