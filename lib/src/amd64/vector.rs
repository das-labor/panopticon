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
use codegen::*;
use value::*;
use amd64::decode::*;
use amd64::semantic::*;
use amd64::*;

use std::rc::Rc;

pub fn mmx(rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>, rm2: Rc<Disassembler<Amd64>>,
           rm3: Rc<Disassembler<Amd64>>, rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
           rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
           rm: Rc<Disassembler<Amd64>>, imm8: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // EMMS
        [ 0x0f, 0x77 ] = nonary("emms",emms),

        // PACKSS*
        [ 0x0f, 0x63, rm ] = binary("packsswb",decode_rm,packsswb),
        [ 0x0f, 0x6b, rm ] = binary("packssdw",decode_rm,packssdw),

        // PACKUSWB
        [ 0x0f, 0x67, rm ] = binary("packuswb",decode_rm,packuswb),

        // PADD*
        [ 0x0f, 0xfc, rm ] = binary("paddb",decode_rm,paddb),
        [ 0x0f, 0xfd, rm ] = binary("paddw",decode_rm,paddw),
        [ 0x0f, 0xfe, rm ] = binary("paddd",decode_rm,paddd),

        // PADDS*
        [ 0x0f, 0xec, rm ] = binary("paddsb",decode_rm,paddsb),
        [ 0x0f, 0xed, rm ] = binary("paddsw",decode_rm,paddsw),

        // PADDUS*
        [ 0x0f, 0xdc, rm ] = binary("paddusb",decode_rm,paddusb),
        [ 0x0f, 0xdd, rm ] = binary("paddusw",decode_rm,paddusw),

        // PAND
        [ 0x0f, 0xdb, rm ] = binary("pand",decode_rm,pand),

        // PANDN
        [ 0x0f, 0xdf, rm ] = binary("pandn",decode_rm,pandn),

        // PCMPEQ*
        [ 0x0f, 0x74, rm ] = binary("pcmpeqb",decode_rm,pcmpeqb),
        [ 0x0f, 0x75, rm ] = binary("pcmpeqw",decode_rm,pcmpeqw),
        [ 0x0f, 0x76, rm ] = binary("pcmpeqd",decode_rm,pcmpeqd),

        // PCMPGT*
        [ 0x0f, 0x64, rm ] = binary("pcmpgtb",decode_rm,pcmpgtb),
        [ 0x0f, 0x65, rm ] = binary("pcmpgtw",decode_rm,pcmpgtw),
        [ 0x0f, 0x66, rm ] = binary("pcmpgtd",decode_rm,pcmpgtd),

        // PMADDWD
        [ 0x0f, 0xf5, rm ] = binary("pmadwd",decode_rm,pmadwd),

        // PMULHW
        [ 0x0f, 0xe5, rm ] = binary("pmulhw",decode_rm,pmulhw),

        // PMULLW
        [ 0x0f, 0xd5, rm ] = binary("pmullw",decode_rm,pmullw),

        // POR
        [ 0x0f, 0xeb, rm ] = binary("por",decode_rm,por),

        // PSLLW
        [ 0x0f, 0xf1, rm        ] = binary("psllw",decode_rm,psllw),
        [ 0x0f, 0x71, rm6, imm8 ] = binary("psllw",decode_mi,psllw),

        // PSLLD
        [ 0x0f, 0xf2, rm        ] = binary("pslld",decode_rm,pslld),
        [ 0x0f, 0x72, rm6, imm8 ] = binary("pslld",decode_mi,pslld),

        // PSLLQ
        [ 0x0f, 0xf3, rm        ] = binary("psllq",decode_rm,psllq),
        [ 0x0f, 0x73, rm6, imm8 ] = binary("psllq",decode_mi,psllq),

        // PSRAW
        [ 0x0f, 0xe1, rm        ] = binary("psraw",decode_rm,psraw),
        [ 0x0f, 0x71, rm4, imm8 ] = binary("psraw",decode_mi,psraw),

        // PSRAD
        [ 0x0f, 0xe2, rm        ] = binary("psrad",decode_rm,psrad),
        [ 0x0f, 0x72, rm4, imm8 ] = binary("psrad",decode_mi,psrad),

        // PSRLW
        [ 0x0f, 0xd1, rm        ] = binary("psrlw",decode_rm,psrlw),
        [ 0x0f, 0x71, rm2, imm8 ] = binary("psrlw",decode_mi,psrlw),

        // PSRLD
        [ 0x0f, 0xd2, rm        ] = binary("psrld",decode_rm,psrld),
        [ 0x0f, 0x71, rm2, imm8 ] = binary("psrld",decode_mi,psrld),

        // PSRLQ
        [ 0x0f, 0xd3, rm        ] = binary("psrlq",decode_rm,psrlq),
        [ 0x0f, 0x71, rm2, imm8 ] = binary("psrlq",decode_mi,psrlq),

        // PSUB*
        [ 0x0f, 0xf8, rm ] = binary("psubb",decode_rm,psubb),
        [ 0x0f, 0xf9, rm ] = binary("psubw",decode_rm,psubw),
        [ 0x0f, 0xfa, rm ] = binary("psubd",decode_rm,psubd),

        // PSUBS*
        [ 0x0f, 0xe8, rm ] = binary("psubsb",decode_rm,psubsb),
        [ 0x0f, 0xe9, rm ] = binary("psubsw",decode_rm,psubsw),

        // PSUBUS*
        [ 0x0f, 0xd8, rm ] = binary("psubusb",decode_rm,psubusb),
        [ 0x0f, 0xd9, rm ] = binary("psubusw",decode_rm,psubusw),

        // PUNPCKH*
        [ 0x0f, 0x68, rm ] = binary("punpckhbw",decode_rm,punpckhbw),
        [ 0x0f, 0x69, rm ] = binary("punpckhwd",decode_rm,punpckhwd),
        [ 0x0f, 0x6a, rm ] = binary("punpckhdq",decode_rm,punpckhdq),

        // PUNPCKL*
        [ 0x0f, 0x60, rm ] = binary("punpcklbw",decode_rm,punpcklbw),
        [ 0x0f, 0x61, rm ] = binary("punpcklwd",decode_rm,punpcklwd),
        [ 0x0f, 0x62, rm ] = binary("punpckldq",decode_rm,punpckldq),

        // PXOR
        [ 0x0f, 0xef, rm ] = binary("pxor",decode_rm,pxor),

        // MOVD/MOVQ
        [ 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov))
}

pub fn sse2(rm: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // MOVAPD
        [ 0x66, 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ 0x66, 0x0f, 0x28, rm ] = binary("movapd",decode_mr,movapd),

        // MOVD/MOVQ
        [ 0x66, 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x66, 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov))
}

pub fn avx(vex_prfx: Rc<Disassembler<Amd64>>, rm: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // VZEROUPPER
        [ vex_prfx, 0x77 ] = nonary("vzeroupper",vzeroupper),

        // MOVD/MOVQ
        [ vex_prfx, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ vex_prfx, 0x7e, rm ] = binary("movd",decode_rm,mov),

        // MOVAPD
        [ vex_prfx, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ vex_prfx, 0x29, rm ] = binary("movapd",decode_mr,movapd))
}
