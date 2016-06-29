/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015, 2016 Kai Michaelis
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
use amd64::decode::*;
use amd64::semantic::*;
use amd64::*;

use std::sync::Arc;

pub fn mmx(_: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>, rm2: Arc<Disassembler<Amd64>>,
           _: Arc<Disassembler<Amd64>>, rm4: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
           rm6: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
           rm: Arc<Disassembler<Amd64>>, imm8: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
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

pub fn sse1(rm0: Arc<Disassembler<Amd64>>, rm1: Arc<Disassembler<Amd64>>, rm2: Arc<Disassembler<Amd64>>,
            rm3: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
            _: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
            rm: Arc<Disassembler<Amd64>>, imm8: Arc<Disassembler<Amd64>>,
            rex_prfx: Arc<Disassembler<Amd64>>, rexw_prfx: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // ADDPS
        [ opt!(rex_prfx), 0x0f, 0x58, rm ] = binary("addps",decode_rm,addps),

        // ADDSS
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x58, rm ] = binary("addss",decode_rm,addss),

        // ANDNPS
        [ opt!(rex_prfx), 0x0f, 0x55, rm ] = binary("andnps",decode_rm,andnps),

        // ANDPS
        [ opt!(rex_prfx), 0x0f, 0x54, rm ] = binary("andps",decode_rm,andps),

        // CMPPS
        [ opt!(rex_prfx), 0x0f, 0xc2, rm, imm8 ] = trinary("cmpps",decode_rmi,cmpps),

        // CMPSS
        [ 0xf3, opt!(rex_prfx), 0x0f, 0xc2, rm, imm8 ] = trinary("cmpss",decode_rmi,cmpss),

        // COMISS
        [ opt!(rex_prfx), 0x0f, 0x2f, rm ] = binary("comiss",decode_rm,comiss),

        // CVTPI2PS
        [ opt!(rex_prfx), 0x0f, 0x2a, rm ] = binary("cvtpi2ps",decode_rm,cvtpi2ps),

        // CVTPS2PI
        [ opt!(rex_prfx), 0x0f, 0x2d, rm ] = binary("cvtps2pi",decode_rm,cvtps2pi),

        // CVTSI2SS
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x2a, rm ] = binary("cvtsi2ss",decode_rm,cvtsi2ss),

        // CVTSS2SI
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x2d, rm ] = binary("cvtss2si",decode_rm,cvtss2si),

        // CVTTPS2PI
        [ opt!(rex_prfx), 0x0f, 0x2c, rm ] = binary("cvttps2pi",decode_rm,cvttps2pi),

        // CVTTSS2SI
        [ 0xf3, opt!(rexw_prfx), 0x0f, 0x2c, rm ] = binary("cvttss2si",decode_rm,cvttss2si),

        // DIV*S
        [ opt!(rex_prfx), 0x0f, 0x5e, rm ] = binary("divps",decode_rm,divps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5e, rm ] = binary("divss",decode_rm,divss),

        // LDMXCSR
        [ opt!(rex_prfx), 0x0f, 0xae, rm2 ] = binary("ldmxcsr",decode_rm,ldmxcsr),

        // MASKMOVQ
        [ opt!(rex_prfx), 0x0f, 0xf7, rm ] = binary("maskmovq",decode_rm,maskmovq),

        // MAX*S
        [ opt!(rex_prfx), 0x0f, 0x5f, rm ] = binary("maxps",decode_rm,maxps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5f, rm ] = binary("maxss",decode_rm,maxss),

        // MIN*S
        [ opt!(rex_prfx), 0x0f, 0x5d, rm ] = binary("minps",decode_rm,minps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5d, rm ] = binary("minss",decode_rm,minss),

        // MOVAPS
        [ opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movaps",decode_rm,movaps),
        [ opt!(rex_prfx), 0x0f, 0x29, rm ] = binary("movaps",decode_mr,movaps),

        // MOVHPS
        [ opt!(rex_prfx), 0x0f, 0x16, rm ] = binary("minhps",decode_rm,minhps),
        [ opt!(rex_prfx), 0x0f, 0x17, rm ] = binary("minhps",decode_mr,minhps),

        // MOVLPS
        [ opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movlps",decode_rm,movlps),
        [ opt!(rex_prfx), 0x0f, 0x13, rm ] = binary("movlps",decode_mr,movlps),

        // MOVMSKPS
        [ opt!(rex_prfx), 0x0f, 0x50, rm ] = binary("movmskps",decode_rm,movmskps),

        // MOVNTPS
        [ opt!(rex_prfx), 0x0f, 0x2b, rm ] = binary("movntps",decode_mr,movntps),

        // MOVNTQ
        [ opt!(rex_prfx), 0x0f, 0xe7, rm ] = binary("movntq",decode_mr,movntq),

        // MOVSS
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x10, rm ] = binary("movss",decode_rm,movss),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x11, rm ] = binary("movss",decode_mr,movss),

        // MOVUPS
        [ opt!(rex_prfx), 0x0f, 0x10, rm ] = binary("movups",decode_rm,movups),
        [ opt!(rex_prfx), 0x0f, 0x11, rm ] = binary("movups",decode_mr,movups),

        // MUL*S
        [ opt!(rex_prfx), 0x0f, 0x59, rm ] = binary("mulps",decode_rm,mulps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x59, rm ] = binary("mulss",decode_rm,mulss),

        // ORPS
        [ opt!(rex_prfx), 0x0f, 0x56, rm ] = binary("orps",decode_rm,orps),

        // PAVG*
        [ opt!(rex_prfx), 0x0f, 0xe0, rm ] = binary("pavgb",decode_rm,pavgb),
        [ opt!(rex_prfx), 0x0f, 0xe3, rm ] = binary("pavgw",decode_rm,pavgw),

        // PINSRW
        [ opt!(rex_prfx), 0x0f, 0xc4, rm, imm8 ] = trinary("pinsrw",decode_rmi,pinsrw),

        // PMAX*
        [ opt!(rex_prfx), 0x0f, 0xee, rm ] = binary("pmaxsw",decode_rm,pmaxsw),
        [ opt!(rex_prfx), 0x0f, 0xde, rm ] = binary("pmaxub",decode_rm,pmaxub),

        // PMIN*
        [ opt!(rex_prfx), 0x0f, 0xea, rm ] = binary("pminsw",decode_rm,pminsw),
        [ opt!(rex_prfx), 0x0f, 0xda, rm ] = binary("pminub",decode_rm,pminub),

        // PMOVMSKB
        [ opt!(rex_prfx), 0x0f, 0xd7, rm ] = binary("pmovmskb",decode_rm,pmovmskb),

        // PMULHUW
        [ opt!(rex_prfx), 0x0f, 0xe4, rm ] = binary("pmulhuw",decode_rm,pmulhuw),

        // PREFETCH*
        [ opt!(rex_prfx), 0x0f, 0x18, rm0 ] = unary("prefetchnta",decode_m,prefetchnta),
        [ opt!(rex_prfx), 0x0f, 0x18, rm1 ] = unary("prefetcht0",decode_m,prefetcht0),
        [ opt!(rex_prfx), 0x0f, 0x18, rm2 ] = unary("prefetcht1",decode_m,prefetcht1),
        [ opt!(rex_prfx), 0x0f, 0x18, rm3 ] = unary("prefetcht2",decode_m,prefetcht2),

        [ opt!(rex_prfx), 0x0f, 0x0d, rm1 ] = unary("prefetchw",decode_m,prefetchw),
        [ opt!(rex_prfx), 0x0f, 0x0d, rm2 ] = unary("prefetchwt1",decode_m,prefetchwt1),

        // PSADBW
        [ opt!(rex_prfx), 0x0f, 0xf6, rm ] = binary("psadbw",decode_rm,psadbw),

        // PSHUFW
        [ opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufw",decode_rmi,pshufw),

        // RCP*S
        [ opt!(rex_prfx), 0x0f, 0x53, rm ] = binary("rcpps",decode_rm,rcpps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x53, rm ] = binary("rcpss",decode_rm,rcpss),

        // RSQRT*S
        [ opt!(rex_prfx), 0x0f, 0x52, rm ] = binary("rsqrtps",decode_rm,rsqrtps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x52, rm ] = binary("rsqrtss",decode_rm,rsqrtss),

        // SFENCE
        [ opt!(rex_prfx), 0x0f, 0xae, 0xf8 ] = nonary("sfence",sfence),

        // SHUFPS
        [ opt!(rex_prfx), 0x0f, 0xc6, rm, imm8 ] = trinary("shufps",decode_rmi,shufps),

        // SQRT*S
        [ opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtps",decode_rm,sqrtps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtss",decode_rm,sqrtss),

        // STMXCSR
        [ opt!(rex_prfx), 0x0f, 0xae, rm3 ] = unary("stmxcsr",decode_m,stmxcsr),

        // SUB*S
        [ opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subps",decode_rm,subps),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subss",decode_rm,subss),

        // UCOMISS
        [ opt!(rex_prfx), 0x0f, 0x2e, rm ] = binary("ucomiss",decode_rm,ucomiss),

        // UNPCK*PS
        [ opt!(rex_prfx), 0x0f, 0x15, rm ] = binary("unpckhps",decode_rm,unpckhps),
        [ opt!(rex_prfx), 0x0f, 0x14, rm ] = binary("unpcklps",decode_rm,unpcklps),

        // XORPS
        [ opt!(rex_prfx), 0x0f, 0x57, rm ] = binary("unpckhps",decode_rm,xorps))
}

pub fn sse2(_: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>, rm2: Arc<Disassembler<Amd64>>,
            rm3: Arc<Disassembler<Amd64>>, rm4: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
            rm6: Arc<Disassembler<Amd64>>, rm7: Arc<Disassembler<Amd64>>,
            rm: Arc<Disassembler<Amd64>>, imm8: Arc<Disassembler<Amd64>>,
            rex_prfx: Arc<Disassembler<Amd64>>, rexw_prfx: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // MOVAPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movapd",decode_mr,movapd),

        // ADD*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x58, rm ] = binary("addpd",decode_rm,addpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x58, rm ] = binary("addsd",decode_rm,addsd),

        // ANDNPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x52, rm ] = binary("addpd",decode_rm,andnpd),

        // ANDPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x54, rm ] = binary("addpd",decode_rm,andpd),

        // CLFLUSH
        [ 0x0f, 0xad, rm7 ] = unary("addpd",decode_m,cflush),

        // CMP*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0xc2, rm, imm8 ] = trinary("cmppd",decode_rmi,cmppd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xc2, rm, imm8 ] = trinary("cmpsd",decode_rmi,cmpsd),

        // COMISD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2f, rm ] = binary("comisd",decode_rm,comisd),

        // CVTDQ2PD
        [ 0xf3, opt!(rex_prfx), 0x0f, 0xe6, rm ] = binary("cvtdq2pd",decode_rm,cvtdq2pd),

        // CVTDQ2PS
        [ 0x0f, 0x5b, rm ] = binary("cvtdq2ps",decode_rm,cvtdq2ps),

        // CVTPD2DQ
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xe6, rm ] = binary("cvtdq2pd",decode_rm,cvtpd2dq),

        // CVTPD2PI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2d, rm ] = binary("cvtpd2pi",decode_rm,cvtpd2pi),

        // CVTPD2PS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5a, rm ] = binary("cvtpd2ps",decode_rm,cvtpd2ps),

        // CVTPI2PD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2a, rm ] = binary("cvtpi2pd",decode_rm,cvtpi2pd),

        // CVTPS2DQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5b, rm ] = binary("cvtps2dq",decode_rm,cvtps2dq),

        // CVTPS2PD
        [ 0x0f, 0x5a, rm ] = binary("cvtps2pd",decode_rm,cvtps2pd),

        // CVTSD2SI
        [ 0xf2, opt!(rexw_prfx), 0x0f, 0x2d, rm ] = binary("cvtsd2si",decode_rm,cvtsd2si),

        // CVTSD2SS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5a, rm ] = binary("cvtsd2ss",decode_rm,cvtsd2ss),

        // CVTSI2SD
        [ 0xf2, opt!(rexw_prfx), 0x0f, 0x2a, rm ] = binary("cvtsi2sd",decode_rm,cvtsi2sd),

        // CVTSS2SD
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5a, rm ] = binary("cvtss2sd",decode_rm,cvtss2sd),

        // CVTTPD2DQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe6, rm ] = binary("cvttpd2dq",decode_rm,cvttpd2dq),

        // CVTTPD2PI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2c, rm ] = binary("cvttpd2pi",decode_rm,cvttpd2pi),

        // CVTTPS2DQ
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5b, rm ] = binary("cvttps2dq",decode_rm,cvttps2dq),

        // CVTTSD2SI
        [ 0xf2, opt!(rexw_prfx), 0x0f, 0x2c, rm ] = binary("cvttsd2si",decode_rm,cvttsd2si),

        // DIV*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5e, rm ] = binary("divpd",decode_rm,divpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5e, rm ] = binary("divsd",decode_rm,divsd),

        // LFENCE
        [ 0x0f, 0xae, 0xe8 ] = nonary("lfence",lfence),

        // MASKMOVDQU
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf7, rm ] = binary("maskmovdqu",decode_rm,maskmovdqu),

        // MAX*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5f, rm ] = binary("maxpd",decode_rm,maxpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5f, rm ] = binary("maxsd",decode_rm,maxsd),

        // MFENCE
        [ 0x0f, 0xae, 0xf0 ] = nonary("mfence",mfence),

        // MIN*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5d, rm ] = binary("minpd",decode_rm,minpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5d, rm ] = binary("minsd",decode_rm,minsd),

        // MOVAPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x29, rm ] = binary("movapd",decode_mr,movapd),

        // MOVD
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x6e, rm ] = binary("movd",decode_rm,movd),
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x7e, rm ] = binary("movd",decode_mr,movd),

        // MOVDQ2Q
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xd6, rm ] = binary("movdq2q",decode_rm,movdq2q),

        // MOVDQA
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6f, rm ] = binary("movdqa",decode_rm,movdaq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7f, rm ] = binary("movdqa",decode_mr,movdqa),

        // MOVDQU
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x6f, rm ] = binary("movdqu",decode_rm,movdqu),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x7f, rm ] = binary("movdqu",decode_mr,movdqu),

        // MOVHPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x16, rm ] = binary("movhpd",decode_rm,movhpd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x17, rm ] = binary("movhpd",decode_mr,movhpd),

        // MOVLPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movlpd",decode_rm,movlpd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x13, rm ] = binary("movlpd",decode_mr,movlpd),

        // MOVMSKPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x50, rm ] = binary("movmskpd",decode_rm,movmskpd),

        // MOVNTDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe7, rm ] = binary("movntdq",decode_mr,movntdq),

        // MOVNTI
        [ opt!(rexw_prfx), 0x0f, 0xc3, rm ] = binary("movapd",decode_mr,movnti),

        // MOVNTPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2b, rm ] = binary("movntpd",decode_mr,movntpd),

        // MOVQ
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x7e, rm ] = binary("movq",decode_rm,movq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd6, rm ] = binary("movq",decode_mr,movq),

        // MOVQ2DQ
        [ 0xf3, opt!(rex_prfx), 0x0f, 0xd6, rm ] = binary("movq2dq",decode_rm,movq2dq),

        // MOVSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x10, rm ] = binary("movsd",decode_rm,movsd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x11, rm ] = binary("movsd",decode_mr,movsd),

        // MOVUPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x10, rm ] = binary("movupd",decode_rm,movupd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x11, rm ] = binary("movupd",decode_mr,movupd),

        // MULPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x59, rm ] = binary("mulpd",decode_rm,mulpd),

        // MULSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x59, rm ] = binary("mulsd",decode_rm,mulsd),

        // ORPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x56, rm ] = binary("orpd",decode_rm,orpd),

        // PACKSS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x63, rm ] = binary("packsswb",decode_rm,packsswb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6b, rm ] = binary("packssdw",decode_rm,packssdw),

        // PACKUSWB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x67, rm ] = binary("packuswb",decode_rm,packuswb),

        // PADD*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfc, rm ] = binary("paddb",decode_rm,paddb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfd, rm ] = binary("paddw",decode_rm,paddw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfe, rm ] = binary("paddd",decode_rm,paddd),
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xd4, rm ] = binary("paddq",decode_rm,paddq),

        // PADDS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xec, rm ] = binary("paddsb",decode_rm,paddsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xed, rm ] = binary("paddsw",decode_rm,paddsw),

        // PADDUS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdc, rm ] = binary("paddusb",decode_rm,paddusb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdd, rm ] = binary("paddusw",decode_rm,paddusw),

        // PAND
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdb, rm ] = binary("pand",decode_rm,pand),

        // PANDN
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdf, rm ] = binary("pandn",decode_rm,pandn),

        // PAUSE
        [ 0xf3, 0x90 ] = nonary("pause",pause),

        // PCMPEQ*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x74, rm ] = binary("pcmpeqb",decode_rm,pcmpeqb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x75, rm ] = binary("pcmpeqw",decode_rm,pcmpeqw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x76, rm ] = binary("pcmpeqd",decode_rm,pcmpeqd),

        // PCMPGT*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x64, rm ] = binary("pcmpgtb",decode_rm,pcmpgtb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x65, rm ] = binary("pcmpgtw",decode_rm,pcmpgtw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x66, rm ] = binary("pcmpgtd",decode_rm,pcmpgtd),

        // PMADDWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf5, rm ] = binary("pmaddwd",decode_rm,pmadwd),

        // PMUL*W
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe5, rm ] = binary("pmulhw",decode_rm,pmulhw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd5, rm ] = binary("pmullw",decode_rm,pmullw),

        // PMULUDQ
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xf4, rm ] = binary("pcmpgtd",decode_rm,pmuludq),

        // POR
        [ 0x66, opt!(rex_prfx), 0x0f, 0xeb, rm ] = binary("pcmpgtd",decode_rm,por),

        // PSHUFD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufd",decode_rmi,pshufd),

        // PSHUFHW
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufhw",decode_rmi,pshufhw),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshuflw",decode_rmi,pshuflw),

        // PSLLD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf1, rm        ] = binary("psllw",decode_rm,psllw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm6, imm8 ] = binary("psllw",decode_mi,psllw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf2, rm        ] = binary("pslld",decode_rm,pslld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm6, imm8 ] = binary("pslld",decode_mi,pslld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf3, rm        ] = binary("psllq",decode_rm,psllq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm6, imm8 ] = binary("psllq",decode_mi,psllq),

        // PSLLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm7, imm8 ] = binary("pslldq",decode_mi,pslldq),

        // PSRAD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe2, rm        ] = binary("psrad",decode_rm,psrad),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm4, imm8 ] = binary("psrad",decode_mi,psrad),

        // PSRAW
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe1, rm        ] = binary("psraw",decode_rm,psarw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm4, imm8 ] = binary("psraw",decode_mi,psarw),

        // PSRLW
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd1, rm        ] = binary("psrlw",decode_rm,psrlw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm2, imm8 ] = binary("psrlw",decode_mi,psrlw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd2, rm        ] = binary("psrld",decode_rm,psrld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm2, imm8 ] = binary("psrld",decode_mi,psrld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd3, rm        ] = binary("psrlq",decode_rm,psrlq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm2, imm8 ] = binary("psrlq",decode_mi,psrlq),

        // PSRLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm3, imm8 ] = binary("psrldq",decode_mi,psrldq),

        // PSUBB
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf8, rm ] = binary("psubb",decode_rm,psubb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf9, rm ] = binary("psubw",decode_rm,psubw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfa, rm ] = binary("psubd",decode_rm,psubd),
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xfb, rm ] = binary("psubq",decode_rm,psubq),

        // PSUBS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe8, rm ] = binary("psubsb",decode_rm,psubsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe9, rm ] = binary("psubsw",decode_rm,pusbsw),

        // PSUBUS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd8, rm ] = binary("psubusb",decode_rm,psubusb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd9, rm ] = binary("psubusw",decode_rm,psubusw),

        // PMOVMSKB
        [ 0x66, opt!(rex_prfx), opt!(rex_prfx), 0x0f, 0xd7, rm ] = binary("pmovmskb",decode_rm,pmovmskb),

        // PMAX*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xee, rm ] = binary("pmaxsw",decode_rm,pmaxsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xde, rm ] = binary("pmaxub",decode_rm,pmaxub),

        // PMIN*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xea, rm ] = binary("pminsw",decode_rm,pminsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xda, rm ] = binary("pminub",decode_rm,pminub),

        // PUNPCKHBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x68, rm ] = binary("punpckhbw",decode_rm,punpckhbw),

        // PUNPCKHWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x69, rm ] = binary("punpckhwd",decode_rm,punckhwd),

        // PUNPCKHDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6a, rm ] = binary("punpckhwd",decode_rm,punpckhdq),

        // PUNPCKHQDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6d, rm ] = binary("punpckhqdq",decode_rm,punpckhqdq),

        // PUNPCKLBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x60, rm ] = binary("punpcklbw",decode_rm,punpcklbw),

        // PUNPCKLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x61, rm ] = binary("punpckldq",decode_rm,punpckldq),

        // PUNPCKLQDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x62, rm ] = binary("punpcklqdq",decode_rm,puncklqdq),

        // PUNPCKLWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6c, rm ] = binary("punpcklwd",decode_rm,puncklwd),

        // PXOR
        [ 0x66, opt!(rex_prfx), 0x0f, 0xef, rm ] = binary("pxor",decode_rm,pxor),

        // SHUFPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xc6, rm, imm8 ] = trinary("shufpd",decode_rmi,shufpd),

        // SQRTPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtpd",decode_rm,sqrtpd),

        // SQRTSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtsd",decode_rm,sqrtsd),

        // SUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subpd",decode_rm,subpd),

        // SUBSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subsd",decode_rm,subsd),

        // UCOMISD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2e, rm ] = binary("ucomisd",decode_rm,ucomisd),

        // UNPCKHPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x15, rm ] = binary("unpckhpd",decode_rm,unpckhpd),

        // UNPCKLPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x14, rm ] = binary("unpcklpd",decode_rm,unpcklpd),

        // XORPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x57, rm ] = binary("xorpd",decode_rm,xorpd),

        // MOVD/MOVQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov))
}

pub fn sse3(rm: Arc<Disassembler<Amd64>>, imm8: Arc<Disassembler<Amd64>>,
            rex_prfx: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // ADDSUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd0, rm ] = binary("addsubpd",decode_rm,addsubpd),

        // ADDSUBPS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xd0, rm ] = binary("addsubps",decode_rm,addsubps),

        // HADDPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7c, rm ] = binary("haddpd",decode_rm,haddpd),

        // HADDPS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x7c, rm ] = binary("haddps",decode_rm,haddps),

        // HSUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7d, rm ] = binary("hsubpd",decode_rm,hsubpd),

        // HSUBPS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x7d, rm ] = binary("hsubps",decode_rm,hsubps),

        // LDDQU
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xf0, rm ] = binary("lddqu",decode_rm,lddqu),

        // MONITOR
        [ 0x0f, 0x01, 0xc8 ] = nonary("monitor",monitor),

        // MOVDDUP
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movddup",decode_rm,movddup),

        // MOVSHDUP
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x16, rm ] = binary("movshdup",decode_rm,movshdup),

        // MUVSLDUP
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movsldup",decode_rm,movsldup),

        // MWAIT
        [ 0x0f, 0x01, 0xc9 ] = nonary("mwait",mwait),

        // PALIGNR
        [ opt!(rex_prfx), 0x0f, 0x3a, 0x0f, rm, imm8 ] = trinary("palignr",decode_rmi,palignr),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0f, rm, imm8 ] = trinary("palignr",decode_rmi,palignr),

        // PABS*
        [       opt!(rex_prfx), 0x0f, 0x38, 0x1c, rm ] = binary("pabsb",decode_rm,pabsb),
        [       opt!(rex_prfx), 0x0f, 0x38, 0x1d, rm ] = binary("pabsw",decode_rm,pabsw),
        [       opt!(rex_prfx), 0x0f, 0x38, 0x1e, rm ] = binary("pabsd",decode_rm,pabsd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x1c, rm ] = binary("pabsb",decode_rm,pabsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x1d, rm ] = binary("pabsw",decode_rm,pabsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x1e, rm ] = binary("pabsd",decode_rm,pabsd),

        // PHADD*
        [       opt!(rex_prfx), 0x0f, 0x38, 0x01, rm ] = binary("phaddw",decode_rm,phaddw),
        [       opt!(rex_prfx), 0x0f, 0x38, 0x01, rm ] = binary("phaddw",decode_rm,phaddw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x02, rm ] = binary("phaddd",decode_rm,phaddd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x02, rm ] = binary("phaddd",decode_rm,phaddd))
}


pub fn sse4(rm: Arc<Disassembler<Amd64>>, imm8: Arc<Disassembler<Amd64>>,
            rex_prfx: Arc<Disassembler<Amd64>>, rexw_prfx: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // BLENDPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0c, rm, imm8 ] = trinary("blendpd",decode_rmi,blendpd),

        // BLENDPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0d, rm, imm8 ] = trinary("blendps",decode_rmi,blendps),

        // BLENDVP*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x14, rm ] = trinary("blendvps",decode_rm0,blendvps),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x15, rm ] = trinary("blendvpd",decode_rm0,blendvpd),

        // DPPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x41, rm, imm8 ] = trinary("dppd",decode_rmi,dppd),

        // DPPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x40, rm, imm8 ] = trinary("dpps",decode_rmi,dpps),

        // EXTRACTPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x17, rm, imm8 ] = trinary("extractps",decode_rmi,extractps),

        // INSERTPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x21, rm, imm8 ] = trinary("insertps",decode_rmi,insertps),

        // MOVNTDQA
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x0a, rm ] = binary("movntdqa",decode_rm,movntdqa),

        // MPSADBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x42, rm, imm8 ] = trinary("mpsadbw",decode_rmi,mpsadbw),

        // PACKUSDW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x2b, rm ] = binary("packusdw",decode_rm,packusdw),

        // PBLENDVB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x10, rm ] = binary("pblendvb",decode_rm,pblendvb),

        // PBLENDW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0e, rm, imm8 ] = trinary("pblendbw",decode_rmi,pblendbw),

        // PCMPEQQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x29, rm ] = binary("pcmpeqq",decode_rm,pcmpeqq),

        // PCMPESTRI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x61, rm, imm8 ] = trinary("pcmpestri",decode_rmi,pcmpestri),

        // PCMPESTRM
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x60, rm, imm8 ] = trinary("pcmpestrm",decode_rmi,pcmpestrm),

        // PCMPISTRI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x63, rm, imm8 ] = trinary("pcmpistri",decode_rmi,pcmpistri),

        // PCMPISTRM
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x62, rm, imm8 ] = trinary("pcmpistrm",decode_rmi,pcmpistrm),

        // PEXTRB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x14, rm, imm8 ] = trinary("pextrb",decode_mri,pextrb),

        // PEXTRD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x16, rm, imm8 ] = trinary("extrd",decode_mri,pextrd),

        // PEXTRQ
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x16, rm, imm8 ] = trinary("extrq",decode_mri,pextrq),

        // PEXTRW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x15, rm, imm8 ] = trinary("extrw",decode_mri,pextrw),

        // PINSRB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x20, rm, imm8 ] = trinary("pinsrb",decode_rmi,pinsrb),

        // PINSRD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x22, rm, imm8 ] = trinary("pinsrd",decode_rmi,pinsrd),

        // PINSRQ
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x22, rm, imm8 ] = trinary("pinsrq",decode_rmi,pinsrq),

        // PHMINPUSUW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x41, rm ] = binary("phminpusuw",decode_rm,phminpushuw),

        // PMAX*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x3c, rm ] = binary("pmaxsb",decode_rm,pmaxsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x3d, rm ] = binary("pmaxsd",decode_rm,pmaxsd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x3e, rm ] = binary("pmaxuw",decode_rm,pmaxuw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x3f, rm ] = binary("pmaxud",decode_rm,pmaxud),

        // PMIN*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x38, rm ] = binary("pminsb",decode_rm,pminsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x39, rm ] = binary("pminsd",decode_rm,pminsd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x3a, rm ] = binary("pminuw",decode_rm,pminuw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x3b, rm ] = binary("pminud",decode_rm,pminud),

        // PMOVSX
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x20, rm ] = binary("pmovsx",decode_rm,pmovsx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x21, rm ] = binary("pmovsx",decode_rm,pmovsx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x22, rm ] = binary("pmovsx",decode_rm,pmovsx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x23, rm ] = binary("pmovsx",decode_rm,pmovsx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x24, rm ] = binary("pmovsx",decode_rm,pmovsx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x25, rm ] = binary("pmovsx",decode_rm,pmovsx),

        // PMOVZX
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x30, rm ] = binary("pmovzx",decode_rm,pmovzx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x31, rm ] = binary("pmovzx",decode_rm,pmovzx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x32, rm ] = binary("pmovzx",decode_rm,pmovzx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x33, rm ] = binary("pmovzx",decode_rm,pmovzx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x34, rm ] = binary("pmovzx",decode_rm,pmovzx),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x35, rm ] = binary("pmovzx",decode_rm,pmovzx),

        // PMULDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x28, rm ] = binary("pmuldq",decode_rm,pmuldq),

        // PMULLD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x40, rm ] = binary("pmuldq",decode_rm,pmulld),

        // PTEST
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x17, rm ] = binary("ptest",decode_rm,ptest),

        // ROUNDPD
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x09, rm, imm8 ] = trinary("roundpd",decode_rmi,roundpd),

        // ROUNDPS
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x08, rm, imm8 ] = trinary("roundpd",decode_rmi,roundps),

        // ROUNDSD
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x0b, rm, imm8 ] = trinary("roundpd",decode_rmi,roundsd),

        // ROUNDSS
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x0a, rm, imm8 ] = trinary("roundpd",decode_rmi,roundss))
}

pub fn avx(vex_0f_prfx: Arc<Disassembler<Amd64>>, vex_660f_prfx: Arc<Disassembler<Amd64>>,
           vex_f20f_prfx: Arc<Disassembler<Amd64>>, vex_f30f_prfx: Arc<Disassembler<Amd64>>,
           vex_0f38_prfx: Arc<Disassembler<Amd64>>, vex_660f38_prfx: Arc<Disassembler<Amd64>>,
           _: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
           _: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
           vex_0f3a_prfx: Arc<Disassembler<Amd64>>, vex_660f3a_prfx: Arc<Disassembler<Amd64>>,
           rm: Arc<Disassembler<Amd64>>,
           _: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>, rm2: Arc<Disassembler<Amd64>>,
           rm3: Arc<Disassembler<Amd64>>, rm4: Arc<Disassembler<Amd64>>, _: Arc<Disassembler<Amd64>>,
           rm6: Arc<Disassembler<Amd64>>, rm7: Arc<Disassembler<Amd64>>,
           imm8: Arc<Disassembler<Amd64>>, is4: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // VADD*
        [ vex_660f_prfx, 0x58, rm ] = trinary("vaddpd",decode_rvm,vaddpd),
        [ vex_0f_prfx, 0x58, rm ] = trinary("vaddps",decode_rvm,vaddps),
        [ vex_f20f_prfx, 0x58, rm ] = trinary("vaddsd",decode_rvm,vaddsd),
        [ vex_f30f_prfx, 0x58, rm ] = trinary("vaddss",decode_rvm,vaddss),

        // VADDSUBP*
        [ vex_660f_prfx, 0xd0, rm ] = trinary("vaddsubpd",decode_rvm,vaddsubpd),
        [ vex_f20f_prfx, 0xd0, rm ] = trinary("vaddsubps",decode_rvm,vaddsubps),

        // VAES*
        [ vex_660f38_prfx, 0xde, rm ] = trinary("vaesdec",decode_rvm,vaesdec),
        [ vex_660f38_prfx, 0xdf, rm ] = trinary("vaesdeclast",decode_rvm,vaesdeclast),
        [ vex_660f38_prfx, 0xdc, rm ] = trinary("vaesenc",decode_rvm,vaesenc),
        [ vex_660f38_prfx, 0xdd, rm ] = trinary("vaesenclast",decode_rvm,vaesenclast),
        [ vex_660f38_prfx, 0xdb, rm ] = trinary("vaesimc",decode_rvm,vaesimc),
        [ vex_660f3a_prfx, 0xdf, rm ] = trinary("vaeskeygenassist",decode_rvm,vaeskeygenassist),

        // VANDP*
        [ vex_660f_prfx, 0x54, rm ] = trinary("vandpd",decode_rvm,vandpd),
        [ vex_0f_prfx, 0x54, rm ] = trinary("vandps",decode_rvm,vandps),

        // VANDNP*
        [ vex_660f_prfx, 0x55, rm ] = trinary("vandnpd",decode_rvm,vandnpd),
        [ vex_0f_prfx, 0x55, rm ] = trinary("vandnps",decode_rvm,vandnps),

        // VBLENDPD
        [ vex_660f3a_prfx, 0x0d, rm, imm8 ] = quinary("vblendpd",decode_rvmi,vblendpd),
        [ vex_0f3a_prfx, 0x0d, rm, imm8 ] = quinary("vblendpd",decode_rvmi,vblendpd),

        // VBEXTR
        [ vex_0f38_prfx, 0xf7, rm ] = trinary("vbextr",decode_rvm,vandps),

        // VBLENDPS
        [ vex_660f3a_prfx, 0x0c, rm, imm8 ] = quinary("vblendps",decode_rvmi,vblendps),
        [ vex_0f3a_prfx, 0x0c, rm, imm8 ] = quinary("vblendps",decode_rvmi,vblendps),

        // VBLENDVP*
        [ vex_660f3a_prfx, 0x4b, rm, is4 ] = quinary("vblendvpd",decode_rvmr,vblendvpd),
        [ vex_660f3a_prfx, 0x4a, rm, is4 ] = quinary("vblendvps",decode_rvmr,vblendvps),

        // VCMPP*
        [ vex_660f_prfx, 0xc2, rm, imm8 ] = quinary("vcmppd",decode_rvmi,vcmppd),
        [ vex_0f_prfx, 0xc2, rm, imm8 ] = quinary("vcmpps",decode_rvmi,vcmpps),

        // VCMPS*
        [ vex_f20f_prfx, 0xc2, rm, imm8 ] = quinary("vcmpsd",decode_rvmi,vcmpsd),
        [ vex_f30f_prfx, 0xc2, rm, imm8 ] = quinary("vcmpss",decode_rvmi,vcmpss),

        // VCOMIS*
        [ vex_660f_prfx, 0x2f, rm ] = binary("vcomisd",decode_rm,vcomisd),
        [ vex_0f_prfx, 0x2f, rm ] = binary("vcomiss",decode_rm,vcomiss),

        // VCVTDQ2PD
        [ vex_f30f_prfx, 0xe6, rm ] = binary("vcvtdq2pd",decode_rm,vcvtdq2pd),

        // VCVTDQ2PS
        [ vex_0f_prfx, 0x5b, rm ] = binary("vcvtdq2ps",decode_rm,vcvtdq2ps),

        // VCVTPD2DQ
        [ vex_f20f_prfx, 0xe6, rm ] = binary("vcvtdq2pd",decode_rm,vcvtpd2dq),

        // VCVTPD2PS
        [ vex_660f_prfx, 0x5a, rm ] = binary("vcvtpd2ps",decode_rm,vcvtpd2ps),

        // VCVTPS2DQ
        [ vex_660f_prfx, 0x5b, rm ] = binary("vcvtps2dq",decode_rm,vcvtps2dq),

        // VCVTPS2PD
        [ vex_0f_prfx, 0x5a, rm ] = binary("vcvtps2pd",decode_rm,vcvtps2pd),

        // VCVTSD2SI
        [ vex_f20f_prfx, 0x2d, rm ] = binary("vcvtsd2si",decode_rm,vcvtsd2si),

        // VCVTSD2SS
        [ vex_f20f_prfx, 0x5a, rm ] = trinary("vcvtsd2ss",decode_rvm,vcvtsd2ss),

        // VCVTSI2SD
        [ vex_f20f_prfx, 0x2a, rm ] = trinary("vcvtsi2sd",decode_rvm,vcvtsi2sd),

        // VCVTSS2SD
        [ vex_f30f_prfx, 0x5a, rm ] = trinary("vcvtss2sd",decode_rvm,vcvtss2sd),

        // VCVTSI2SD
        [ vex_f20f_prfx, 0x2a, rm ] = trinary("vcvtsi2sd",decode_rvm,vcvtsi2sd),

        // VCVTSI2SS
        [ vex_f30f_prfx, 0x2a, rm ] = trinary("vcvtsi2ss",decode_rvm,vcvtsi2ss),

        // VCVTSS2SI
        [ vex_f20f_prfx, 0x2a, rm ] = trinary("vcvtsi2sd",decode_rvm,vcvtsi2sd),

        // VCVTTPD2DQ
        [ vex_660f_prfx, 0xe6, rm ] = binary("vcvttpd2dq",decode_rm,vcvttpd2dq),

        // VCVTTPS2DQ
        [ vex_f30f_prfx, 0x5b, rm ] = binary("vcvttps2dq",decode_rm,vcvttps2dq),

        // VCVTTSD2SI
        [ vex_f20f_prfx, 0x2c, rm ] = binary("vcvttsd2si",decode_rm,vcvttsd2si),

        // VCVTTSS2SI
        [ vex_f30f_prfx, 0x2c, rm ] = binary("vcvttsd2si",decode_rm,vcvttss2si),

        // VDIV*
        [ vex_0f_prfx, 0x5e, rm ] = trinary("vdivps",decode_rmv,vdivps),
        [ vex_660f_prfx, 0x5e, rm ] = trinary("vdivpd",decode_rmv,vdivpd),
        [ vex_f30f_prfx, 0x5e, rm ] = trinary("vdivss",decode_rmv,vdivss),
        [ vex_f20f_prfx, 0x5e, rm ] = trinary("vdivsd",decode_rmv,vdivsd),

        // VDPPD
        [ vex_660f3a_prfx, 0x41, rm, imm8 ] = quinary("vdppd",decode_vrmi,vdppd),

        // VDPPS
        [ vex_660f3a_prfx, 0x40, rm, imm8 ] = quinary("vdpps",decode_vrmi,vdpps),

        // VEXTRACTPS
        [ vex_660f3a_prfx, 0x17, rm, imm8 ] = quinary("vextractps",decode_vrmi,vextractps),

        // VHADDPD
        [ vex_660f_prfx, 0x7c, rm ] = trinary("vhaddpd",decode_rvm,vhaddpd),

        // VHADDPS
        [ vex_f20f_prfx, 0x7c, rm ] = trinary("vhaddps",decode_rvm,vhaddps),

        // VHSUBPD
        [ vex_660f_prfx, 0x7d, rm ] = trinary("vhsubpd",decode_rvm,vhsubpd),

        // VHSUBPS
        [ vex_f20f_prfx, 0x7d, rm ] = trinary("vhsubps",decode_rvm,vhsubps),

        // VINSERTPS
        [ vex_660f3a_prfx, 0x21, rm, imm8 ] = quinary("vinsertps",decode_vrmi,vinsertps),

        // VLDDQU
        [ vex_f20f_prfx, 0xf0, rm ] = binary("vlddqu",decode_rm,vlddqu),

        // VLDMXCSR
        [ vex_0f_prfx, 0xae, rm2 ] = unary("vldmxcsr",decode_m,vldmxcsr),

        // VMASKMOVDQU
        [ vex_660f_prfx, 0xf7, rm ] = binary("vmaskmovdqu",decode_rm,maskmovdqu),

        // VMAX*
        [ vex_660f_prfx, 0x5f, rm ] = trinary("vmaxpd",decode_rvm,vmaxpd),
        [ vex_f20f_prfx, 0x5f, rm ] = trinary("vmaxsd",decode_rvm,vmaxsd),
        [ vex_0f_prfx, 0x5f, rm       ] = trinary("vmaxps",decode_rvm,vmaxps),
        [ vex_f30f_prfx, 0x5f, rm ] = trinary("vmaxss",decode_rvm,vmaxss),

        // VMIN*
        [ vex_660f_prfx, 0x5d, rm ] = trinary("vminpd",decode_rvm,vminpd),
        [ vex_f20f_prfx, 0x5d, rm ] = trinary("vminsd",decode_rvm,vminsd),
        [ vex_0f_prfx, 0x5d, rm       ] = trinary("vminps",decode_rvm,vminps),
        [ vex_f30f_prfx, 0x5d, rm ] = trinary("vminss",decode_rvm,vminss),

        // VMOVAPD
        [ vex_660f_prfx, 0x28, rm ] = binary("vmovapd",decode_rm,movapd),
        [ vex_660f_prfx, 0x29, rm ] = binary("vmovapd",decode_mr,movapd),

        // VMOVAPS
        [ vex_0f_prfx, 0x28, rm ] = binary("vmovaps",decode_rm,movaps),
        [ vex_0f_prfx, 0x29, rm ] = binary("vmovaps",decode_mr,movaps),

        // VMOVD
        [ vex_660f_prfx, 0x6e, rm ] = binary("vmovd",decode_rm,movd),
        [ vex_660f_prfx, 0x7e, rm ] = binary("vmovd",decode_mr,movd),

        // VMOVDDUP
        [ vex_f20f_prfx, 0x12, rm ] = binary("vmovddup",decode_rm,movddup),

        // VMOVDQA
        [ vex_660f_prfx, 0x6f, rm ] = binary("vmovdqa",decode_rm,movdaq),
        [ vex_660f_prfx, 0x7f, rm ] = binary("vmovdqa",decode_mr,movdqa),

        // VMOVDQU
        [ vex_f30f_prfx, 0x6f, rm ] = binary("vmovdqu",decode_rm,movdqu),
        [ vex_f30f_prfx, 0x7f, rm ] = binary("vmovdqu",decode_mr,movdqu),

        // VMOVHPS
        [ vex_0f_prfx, 0x16, rm ] = binary("vminhps",decode_rm,minhps),
        [ vex_0f_prfx, 0x17, rm ] = binary("vminhps",decode_mr,minhps),

        // VMOVHPD
        [ vex_660f_prfx, 0x16, rm ] = trinary("vmovhpd",decode_rvm,vmovhpd),
        [ vex_660f_prfx, 0x17, rm ] = binary("vmovhpd",decode_mr,movhpd),

        // VMOVHPS
        [ vex_0f_prfx, 0x16, rm ] = trinary("vmovhps",decode_rvm,vmovhps),
        [ vex_0f_prfx, 0x17, rm ] = binary("vmovhps",decode_mr,movhps),

        // VMOVLPD
        [ vex_660f_prfx, 0x12, rm ] = trinary("vmovlpd",decode_rvm,vmovlpd),
        [ vex_660f_prfx, 0x13, rm ] = binary("vmovlpd",decode_mr,movlpd),

        // VMOVLPS
        [ vex_0f_prfx, 0x12, rm ] = trinary("vmovlps",decode_rvm,vmovlps),
        [ vex_0f_prfx, 0x13, rm ] = binary("vmovlps",decode_mr,movlps),

        // VMOVMSKPD
        [ vex_660f_prfx, 0x50, rm ] = binary("vmovmskpd",decode_rm,movmskpd),

        // VMOVMSKPS
        [ vex_0f_prfx, 0x50, rm ] = binary("vmovmskps",decode_rm,movmskps),

        // VMOVNTQA
        [ vex_660f38_prfx, 0x2a, rm ] = binary("vmovntqa",decode_rm,movntdqa),

        // VMOVNTDQ
        [ vex_660f_prfx, 0xe7, rm ] = binary("vmovntq",decode_mr,movntq),

        // VMOVNTP*
        [ vex_660f_prfx, 0x2b, rm ] = binary("vmovntpd",decode_mr,movntpd),
        [ vex_0f_prfx, 0x2b, rm ] = binary("vmovntps",decode_mr,movntps),

        // VMOVDQ2Q
        [ vex_f20f_prfx, 0xd6, rm ] = binary("vmovdq2q",decode_rm,movdq2q),

        // VMOVHPD
        [ vex_660f_prfx, 0x16, rm ] = binary("vmovhpd",decode_rm,movhpd),
        [ vex_660f_prfx, 0x17, rm ] = binary("vmovhpd",decode_mr,movhpd),

        // VMOVQ
        [ vex_f30f_prfx, 0x7e, rm ] = binary("vmovq",decode_rm,movq),
        [ vex_660f_prfx, 0xd6, rm ] = binary("vmovq",decode_mr,movq),

        // VMOVSD
        [ vex_f20f_prfx, 0x10, rm ] = trinary("vmovsd",decode_rvm,vmovsd),
        [ vex_f20f_prfx, 0x11, rm ] = trinary("vmovsd",decode_mvr,vmovsd),

        // VMOVSHDUP
        [ vex_f30f_prfx, 0x16, rm ] = binary("vmovshdup",decode_rm,movshdup),

        // VMUVSLDUP
        [ vex_f30f_prfx, 0x12, rm ] = binary("vmovsldup",decode_rm,movsldup),

        // VMOVSS
        [ vex_f30f_prfx, 0x10, rm ] = trinary("vmovss",decode_rvm,vmovss),
        [ vex_f30f_prfx, 0x11, rm ] = trinary("vmovss",decode_mvr,vmovss),

        // VMOVUPD
        [ vex_660f_prfx, 0x10, rm ] = binary("vmovupd",decode_rm,movupd),
        [ vex_660f_prfx, 0x11, rm ] = binary("vmovupd",decode_mr,movupd),

        // VMOVUPS
        [ vex_0f_prfx, 0x10, rm ] = binary("vmovups",decode_rm,movups),
        [ vex_0f_prfx, 0x11, rm ] = binary("vmovups",decode_mr,movups),

        // VMPSADBW
        [ vex_660f3a_prfx, 0x42, rm, imm8 ] = quinary("vmpsadbw",decode_vrmi,vmpsadbw),

        // VMUL*
        [ vex_0f_prfx, 0x59, rm       ] = trinary("vmulps",decode_rvm,vmulps),
        [ vex_f30f_prfx, 0x59, rm ] = trinary("vmulss",decode_rvm,vmulss),
        [ vex_660f_prfx, 0x59, rm ] = trinary("vmulpd",decode_rvm,vmulpd),
        [ vex_f20f_prfx, 0x59, rm ] = trinary("vmulsd",decode_rvm,vmulsd),

        // VORPD
        [ vex_660f_prfx, 0x56, rm ] = trinary("vorpd",decode_rvm,vorpd),

        // VORPS
        [ vex_0f_prfx, 0x56, rm ] = trinary("vorps",decode_rvm,vorps),

        // VPABS*
        [ vex_660f38_prfx, 0x1c, rm ] = binary("vpabsb",decode_rm,vpabsb),
        [ vex_660f38_prfx, 0x1d, rm ] = binary("vpabsw",decode_rm,vpabsw),
        [ vex_660f38_prfx, 0x1e, rm ] = binary("vpabsd",decode_rm,vpabsd),

        // VPACKSS*
        [ vex_660f_prfx, 0x63, rm ] = trinary("vpacksswb",decode_rvm,vpacksswb),
        [ vex_660f_prfx, 0x6b, rm ] = trinary("vpackssdw",decode_rvm,vpackssdw),

        // VPACKUS*
        [ vex_660f38_prfx, 0x2b, rm ] = trinary("vpackusdw",decode_rvm,vpackusdw),
        [ vex_660f_prfx, 0x67, rm ] = trinary("vpackuswb",decode_rvm,vpackuswb),

        // VPADD*
        [ vex_660f_prfx, 0xfc, rm ] = trinary("vpaddb",decode_rvm,vpaddb),
        [ vex_660f_prfx, 0xfd, rm ] = trinary("vpaddw",decode_rvm,vpaddw),
        [ vex_660f_prfx, 0xfe, rm ] = trinary("vpaddd",decode_rvm,vpaddd),
        [ vex_660f_prfx, 0xd4, rm ] = trinary("vpaddq",decode_rvm,vpaddq),

        // VPADDS*
        [ vex_660f_prfx, 0xec, rm ] = trinary("vpaddsb",decode_rvm,vpaddsb),
        [ vex_660f_prfx, 0xed, rm ] = trinary("vpaddsw",decode_rvm,vpaddsw),

        // VPADDUS*
        [ vex_660f_prfx, 0xdc, rm ] = trinary("vpaddusb",decode_rvm,vpaddusb),
        [ vex_660f_prfx, 0xdd, rm ] = trinary("vpaddusw",decode_rvm,vpaddusw),

        // VPALIGNR
        [ vex_660f3a_prfx, 0x0f, rm, imm8 ] = quinary("vpalignr",decode_rvmi,vpalignr),

        // VPAND
        [ vex_660f_prfx, 0xdb, rm ] = trinary("vpand",decode_rvm,vpand),

        // VPANDN
        [ vex_660f_prfx, 0xdf, rm ] = trinary("vpandn",decode_rvm,vpandn),

        // VPAVG*
        [ vex_0f_prfx, 0xe0, rm ] = trinary("vpavgb",decode_rvm,vpavgb),
        [ vex_0f_prfx, 0xe3, rm ] = trinary("vpavgw",decode_rvm,vpavgw),

        // VPBLENDVB
        [ vex_660f3a_prfx, 0x4c, rm, is4 ] = quinary("vpblendvb",decode_rvmr,vpblendvb),

        // VPBLENDW
        [ vex_660f3a_prfx, 0x0e, rm, imm8 ] = quinary("vpblendw",decode_rvmi,vpblendw),

        // VPCLMULQDQ
        [ vex_660f3a_prfx, 0x44, rm, imm8 ] = quinary("vpclmulqdq",decode_rvmi,vpclmulqdq),

        // VPCMPEQ*
        [ vex_660f_prfx, 0x74, rm ] = trinary("vpcmpeqb",decode_rvm,vpcmpeqb),
        [ vex_660f_prfx, 0x75, rm ] = trinary("vpcmpeqw",decode_rvm,vpcmpeqw),
        [ vex_660f_prfx, 0x76, rm ] = trinary("vpcmpeqd",decode_rvm,vpcmpeqd),
        [ vex_660f38_prfx, 0x29, rm ] = trinary("vpcmpeqd",decode_rvm,vpcmpeqq),

        // VPCMPESTRI
        [ vex_660f3a_prfx, 0x61, rm, imm8 ] = trinary("vpcmpestri",decode_rmi,pcmpestri),

        // VPCMPESTRM
        [ vex_660f3a_prfx, 0x60, rm, imm8 ] = trinary("vpcmpestrm",decode_rmi,pcmpestrm),

        // VPCMPGT*
        [ vex_660f_prfx, 0x64, rm ] = trinary("vpcmpgtb",decode_rvm,vpcmpgtb),
        [ vex_660f_prfx, 0x65, rm ] = trinary("vpcmpgtw",decode_rvm,vpcmpgtw),
        [ vex_660f_prfx, 0x66, rm ] = trinary("vpcmpgtd",decode_rvm,vpcmpgtd),
        [ vex_660f38_prfx, 0x37, rm ] = trinary("vpcmpgtq",decode_rvm,vpcmpgtq),

        // VPCMPISTRI
        [ vex_660f3a_prfx, 0x63, rm, imm8 ] = trinary("vpcmpistri",decode_rmi,pcmpistri),

        // VPCMPISTRM
        [ vex_660f3a_prfx, 0x62, rm, imm8 ] = trinary("vpcmpistrm",decode_rmi,pcmpistrm),

        // VPEXT*
        [ vex_660f3a_prfx, 0x14, rm, imm8 ] = trinary("vpextrb",decode_mri,pextrb),
        [ vex_660f3a_prfx, 0x16, rm, imm8 ] = trinary("vpextrd",decode_mri,pextrd),
        [ vex_660f3a_prfx, 0x15, rm, imm8 ] = trinary("vpextrw",decode_mri,pextrw),
        [ vex_660f_prfx, 0xc5, rm, imm8 ] = trinary("vpextrw",decode_rmi,pextrw),

        // VHADDW
        [ vex_660f38_prfx, 0x01, rm ] = trinary("vphaddw",decode_rvm,vphaddw),

        // VHADDD
        [ vex_660f38_prfx, 0x02, rm ] = trinary("vphaddd",decode_rvm,vphaddd),

        // VHADDSW
        [ vex_660f38_prfx, 0x03, rm ] = trinary("vphaddsw",decode_rvm,vphaddsw),

        // VPHMINPOSUW
        [ vex_660f38_prfx, 0x41, rm ] = trinary("vphminposuw",decode_rvm,vphminposuw),

        // VHSUBW
        [ vex_660f38_prfx, 0x05, rm ] = trinary("vphsubw",decode_rvm,vphsubw),

        // VHSUBD
        [ vex_660f38_prfx, 0x06, rm ] = trinary("vphsubd",decode_rvm,vphsubd),

        // VHSUBSW
        [ vex_660f38_prfx, 0x07, rm ] = trinary("vphsubsw",decode_rvm,vphsubsw),

        // VPINSRB
        [ vex_660f3a_prfx, 0x20, rm, imm8 ] = quinary("vpinsrb",decode_rvmi,vpinsrb),

        // VPINSRD
        [ vex_660f3a_prfx, 0x22, rm, imm8 ] = quinary("vpinsrd",decode_rvmi,vpinsrd),

        // VPINSRW
        [ vex_660f_prfx, 0xc4, rm, imm8 ] = quinary("vpinsrw",decode_rvmi,vpinsrw),

        // VPMADDUBSW
        [ vex_660f38_prfx, 0x04, rm ] = trinary("vpmaddubsw",decode_rvm,vpmaddubsw),

        // VPMADDWD
        [ vex_660f_prfx, 0xf5, rm ] = trinary("vpmaddwd",decode_rvm,vpmadwd),

        // VPMAX*
        [ vex_660f38_prfx, 0x3c, rm ] = trinary("vpmaxsb",decode_rvm,vpmaxsb),
        [ vex_660f38_prfx, 0x3d, rm ] = trinary("vpmaxsd",decode_rvm,vpmaxsd),
        [ vex_660f_prfx, 0xee, rm ] = trinary("vpmaxsw",decode_rvm,vpmaxsw),
        [ vex_660f_prfx, 0xde, rm ] = trinary("vpmaxub",decode_rvm,vpmaxub),
        [ vex_660f38_prfx, 0x3f, rm ] = trinary("vpmaxud",decode_rvm,vpmaxud),
        [ vex_660f38_prfx, 0x3e, rm ] = trinary("vpmaxuw",decode_rvm,vpmaxuw),

        // VPMIN*
        [ vex_660f38_prfx, 0x38, rm ] = trinary("vpminsb",decode_rvm,vpminsb),
        [ vex_660f38_prfx, 0x39, rm ] = trinary("vpminsd",decode_rvm,vpminsd),
        [ vex_660f_prfx, 0xea, rm ] = trinary("vpminsw",decode_rvm,vpminsw),
        [ vex_660f_prfx, 0xda, rm ] = trinary("vpminub",decode_rvm,vpminub),
        [ vex_660f38_prfx, 0x3b, rm ] = trinary("vpminud",decode_rvm,vpminud),
        [ vex_660f38_prfx, 0x3a, rm ] = trinary("vpminuw",decode_rvm,vpminuw),

        // VPMOVMSKB
        [ vex_660f_prfx, 0xd7, rm ] = binary("vpmovmskb",decode_rm,pmovmskb),

        // VPMOVSX
        [ vex_660f38_prfx, 0x20, rm ] = binary("vpmovsx",decode_rm,pmovsx),
        [ vex_660f38_prfx, 0x21, rm ] = binary("vpmovsx",decode_rm,pmovsx),
        [ vex_660f38_prfx, 0x22, rm ] = binary("vpmovsx",decode_rm,pmovsx),
        [ vex_660f38_prfx, 0x23, rm ] = binary("vpmovsx",decode_rm,pmovsx),
        [ vex_660f38_prfx, 0x24, rm ] = binary("vpmovsx",decode_rm,pmovsx),
        [ vex_660f38_prfx, 0x25, rm ] = binary("vpmovsx",decode_rm,pmovsx),

        // VPMOVZX
        [ vex_660f38_prfx, 0x30, rm ] = binary("vpmovzx",decode_rm,pmovzx),
        [ vex_660f38_prfx, 0x31, rm ] = binary("vpmovzx",decode_rm,pmovzx),
        [ vex_660f38_prfx, 0x32, rm ] = binary("vpmovzx",decode_rm,pmovzx),
        [ vex_660f38_prfx, 0x33, rm ] = binary("vpmovzx",decode_rm,pmovzx),
        [ vex_660f38_prfx, 0x34, rm ] = binary("vpmovzx",decode_rm,pmovzx),
        [ vex_660f38_prfx, 0x35, rm ] = binary("vpmovzx",decode_rm,pmovzx),

        // VPMULDQ
        [ vex_660f38_prfx, 0x28, rm ] = trinary("vpmuldq",decode_rvm,vpmuldq),

        // VPMULHRSW
        [ vex_660f38_prfx, 0x0b, rm ] = trinary("vpmulhrsw",decode_rvm,vpmulhrsw),

        // VPMULHUW
        [ vex_660f_prfx, 0xe4, rm ] = trinary("vpmulhuw",decode_rvm,vpmulhuw),

        // VPMULHW
        [ vex_660f_prfx, 0xe5, rm ] = trinary("vpmulhw",decode_rvm,vpmulhw),

        // VPMULLD
        [ vex_660f38_prfx, 0x40, rm ] = trinary("vpmulld",decode_rvm,vpmulld),

        // VPMULLW
        [ vex_660f_prfx, 0xd5, rm ] = trinary("vpmullw",decode_rvm,vpmullw),

        // VPMULUDQ
        [ vex_660f_prfx, 0xf4, rm ] = trinary("vpmulhd",decode_rvm,vpmuludq),

        // VPOR
        [ vex_660f_prfx, 0xeb, rm ] = trinary("vpor",decode_rvm,vpor),

        // VPSADBW
        [ vex_660f_prfx, 0xf6, rm ] = trinary("vpsadbw",decode_rvm,vpsadbw),

        // VPSHUF*
        [ vex_660f38_prfx, 0x00, rm ] = trinary("vpshufb",decode_rvm,pshufb),
        [ vex_660f_prfx, 0x70, rm, imm8 ] = trinary("vpshufd",decode_rmi,pshufd),
        [ vex_f30f_prfx, 0x70, rm, imm8 ] = trinary("vpshufhw",decode_rmi,pshufhw),
        [ vex_f20f_prfx, 0x70, rm, imm8 ] = trinary("vpshuflw",decode_rmi,pshuflw),

        // VPSIGN*
        [ vex_660f38_prfx, 0x08, rm ] = trinary("vpsignb",decode_rvm,vpsignb),
        [ vex_660f38_prfx, 0x09, rm ] = trinary("vpsignw",decode_rvm,vpsignw),
        [ vex_660f38_prfx, 0x0a, rm ] = trinary("vpsignd",decode_rvm,vpsignd),

        // VPSLLDQ
        [ vex_660f_prfx, 0x73, rm7, imm8 ] = trinary("vpslldq",decode_vmi,vpslldq),

        // VPSLL*
        [ vex_660f_prfx, 0xf1, rm        ] = trinary("vpsllw",decode_rvm,vpsllw),
        [ vex_660f_prfx, 0x71, rm6, imm8 ] = trinary("vpsllw",decode_vmi,vpsllw),
        [ vex_660f_prfx, 0xf2, rm        ] = trinary("vpslld",decode_rvm,vpslld),
        [ vex_660f_prfx, 0x72, rm6, imm8 ] = trinary("vpslld",decode_vmi,vpslld),
        [ vex_660f_prfx, 0xf3, rm        ] = trinary("vpsllq",decode_rvm,vpsllq),
        [ vex_660f_prfx, 0x73, rm6, imm8 ] = trinary("vpsllq",decode_vmi,vpsllq),

        // VPSRA*
        [ vex_660f_prfx, 0xe2, rm        ] = trinary("vpsrad",decode_rvm,vpsrad),
        [ vex_660f_prfx, 0x72, rm4, imm8 ] = trinary("vpsrad",decode_vmi,vpsrad),
        [ vex_660f_prfx, 0xe1, rm        ] = trinary("vpsraw",decode_rvm,vpsarw),
        [ vex_660f_prfx, 0x71, rm4, imm8 ] = trinary("vpsraw",decode_vmi,vpsarw),

        // VPSRLDQ
        [ vex_660f_prfx, 0x73, rm3, imm8 ] = trinary("vpsrldq",decode_vmi,vpsrldq),

        // VPSRL*
        [ vex_660f_prfx, 0xd1, rm        ] = trinary("vpsrlw",decode_rvm,vpsrlw),
        [ vex_660f_prfx, 0x71, rm2, imm8 ] = trinary("vpsrlw",decode_vmi,vpsrlw),
        [ vex_660f_prfx, 0xd2, rm        ] = trinary("vpsrld",decode_rvm,vpsrld),
        [ vex_660f_prfx, 0x72, rm2, imm8 ] = trinary("vpsrld",decode_vmi,vpsrld),
        [ vex_660f_prfx, 0xd3, rm        ] = trinary("vpsrlq",decode_rvm,vpsrlq),
        [ vex_660f_prfx, 0x73, rm2, imm8 ] = trinary("vpsrlq",decode_vmi,vpsrlq),

        // VPSUBB
        [ vex_660f_prfx, 0xf8, rm ] = trinary("vpsubb",decode_rvm,vpsubb),
        [ vex_660f_prfx, 0xf9, rm ] = trinary("vpsubw",decode_rvm,vpsubw),
        [ vex_660f_prfx, 0xfa, rm ] = trinary("vpsubd",decode_rvm,vpsubd),
        [ vex_660f_prfx, 0xfb, rm ] = trinary("vpsubq",decode_rvm,vpsubq),

        // VPSUBS*
        [ vex_660f_prfx, 0xe8, rm ] = trinary("vpsubsb",decode_rvm,vpsubsb),
        [ vex_660f_prfx, 0xe9, rm ] = trinary("vpsubsw",decode_rvm,vpusbsw),

        // VPSUBUS*
        [ vex_660f_prfx, 0xd8, rm ] = trinary("vpsubusb",decode_rvm,vpsubusb),
        [ vex_660f_prfx, 0xd9, rm ] = trinary("vpsubusw",decode_rvm,vpsubusw),

        // VPTEST
        [ vex_660f38_prfx, 0x17 ] = nonary("vptest",vptest),

        // VPUNPCKH*
        [ vex_660f_prfx, 0x68, rm ] = trinary("vpunpckhbw",decode_rvm,vpunpckhbw),
        [ vex_660f_prfx, 0x69, rm ] = trinary("vpunpckhwd",decode_rvm,vpunckhwd),
        [ vex_660f_prfx, 0x6a, rm ] = trinary("vpunpckhwd",decode_rvm,vpunpckhdq),
        [ vex_660f_prfx, 0x6d, rm ] = trinary("vpunpckhqdq",decode_rvm,vpunpckhqdq),

        // VPUNPCKL*
        [ vex_660f_prfx, 0x60, rm ] = trinary("vpunpcklbw",decode_rvm,vpunpcklbw),
        [ vex_660f_prfx, 0x61, rm ] = trinary("vpunpckldq",decode_rvm,vpunpckldq),
        [ vex_660f_prfx, 0x62, rm ] = trinary("vpunpcklqdq",decode_rvm,vpuncklqdq),
        [ vex_660f_prfx, 0x6c, rm ] = trinary("vpunpcklwd",decode_rvm,vpuncklwd),

        // VPXOR
        [ vex_660f_prfx, 0xef, rm ] = trinary("vpxor",decode_rvm,vpxor),

        // VRCP*S
        [ vex_0f_prfx, 0x53, rm ] = binary("vrcpps",decode_rm,rcpps),
        [ vex_f30f_prfx, 0x53, rm ] = trinary("vrcpps",decode_rvm,vrcpps),

        // VROUND*
        [ vex_660f3a_prfx, 0x09, rm, imm8 ] = quinary("vroundpd",decode_rvmi,vroundpd),
        [ vex_660f3a_prfx, 0x08, rm, imm8 ] = quinary("vroundps",decode_rvmi,vroundps),
        [ vex_660f3a_prfx, 0x0b, rm, imm8 ] = quinary("vroundsd",decode_rvmi,vroundsd),
        [ vex_660f3a_prfx, 0x0a, rm, imm8 ] = quinary("vroundss",decode_rvmi,vroundss),

        // VRSQRT*
        [ vex_0f_prfx, 0x52, rm ] = trinary("vrsqrtps",decode_rvm,vrsqrtps),
        [ vex_f30f_prfx, 0x52, rm ] = trinary("vrsqrtss",decode_rvm,vrsqrtss),

        // VSHUFP*
        [ vex_0f_prfx, 0xc6, rm, imm8 ] = quinary("vshufps",decode_rvmi,vshufps),
        [ vex_660f_prfx, 0xc6, rm, imm8 ] = quinary("vshufpd",decode_rvmi,vshufpd),

        // VSQRT*
        [ vex_f20f_prfx, 0x51, rm ] = trinary("vsqrtsd",decode_rvm,vsqrtsd),
        [ vex_f30f_prfx, 0x51, rm ] = trinary("vsqrtss",decode_rvm,vsqrtss),
        [ vex_0f_prfx, 0x51, rm ] = binary("vsqrtps",decode_rm,sqrtps),
        [ vex_f30f_prfx, 0x51, rm ] = binary("vsqrtpd",decode_rm,sqrtpd),

        // VSTMXCSR
        [ vex_0f_prfx, 0xae, rm3 ] = unary("vstmxcsr",decode_m,stmxcsr),

        // VSUB*
        [ vex_0f_prfx, 0x5c, rm ] = trinary("vsubps",decode_rvm,vsubps),
        [ vex_f30f_prfx, 0x5c, rm ] = trinary("vsubss",decode_rvm,vsubss),
        [ vex_660f_prfx, 0x5c, rm ] = trinary("vsubpd",decode_rvm,vsubpd),
        [ vex_f20f_prfx, 0x5c, rm ] = trinary("vsubsd",decode_rvm,vsubsd),

        // VUCOMISD
        [ vex_660f_prfx, 0x2e, rm ] = binary("vucomisd",decode_rm,ucomisd),
        [ vex_0f_prfx, 0x2e, rm ] = binary("vucomiss",decode_rm,ucomiss),

        // VUNPCK*PS
        [ vex_0f_prfx, 0x15, rm ] = trinary("vunpckhps",decode_rvm,vunpckhps),
        [ vex_0f_prfx, 0x14, rm ] = trinary("vunpcklps",decode_rvm,vunpcklps),
        [ vex_660f_prfx, 0x15, rm ] = trinary("vunpckhpd",decode_rvm,vunpckhpd),
        [ vex_660f_prfx, 0x14, rm ] = trinary("vunpcklpd",decode_rvm,vunpcklpd),

        // VBROADCAST*
        [ vex_660f38_prfx, 0x18, rm ] = binary("vbroadcastss",decode_rm,vbroadcastss),
        [ vex_660f38_prfx, 0x19, rm ] = binary("vbroadcastsd",decode_rm,vbroadcastsd),
        [ vex_660f38_prfx, 0x1a, rm ] = binary("vbroadcastf128",decode_rm,vbroadcastf128),

        // VEXTRACT*
        [ vex_660f3a_prfx, 0x19, rm, imm8 ] = binary("vextractf128",decode_mr,vextractf128),
        [ vex_660f3a_prfx, 0x39, rm, imm8 ] = trinary("vextracti128",decode_rmi,vextracti128),

        // VGATHER*
        [ vex_660f38_prfx, 0x90, rm ] = trinary("vgatherdd",decode_rmv,vgatherdd),
        [ vex_660f38_prfx, 0x91, rm ] = trinary("vgatherpd",decode_rmv,vgatherdp),
        [ vex_660f38_prfx, 0x92, rm ] = trinary("vgatherdpd",decode_rmv,vgatherpdp),
        [ vex_660f38_prfx, 0x93, rm ] = trinary("vgatherqpd",decode_rmv,vgatherqpd),

        // VINSERT*
        [ vex_660f3a_prfx, 0x18, rm, imm8 ] = trinary("vinsertf128",decode_rvm,vinsertf128),
        [ vex_660f3a_prfx, 0x38, rm, imm8 ] = quinary("vinserti128",decode_rvmi,vinserti128),

        // VMASKMOV*
        [ vex_660f38_prfx, 0x2c, rm ] = trinary("vmaskmovps",decode_rvm,vmaskmovps),
        [ vex_660f38_prfx, 0x2d, rm ] = trinary("vmaskmovpd",decode_rvm,vmaskmovpd),
        [ vex_660f38_prfx, 0x2e, rm ] = trinary("vmaskmovps",decode_rvm,vmaskmovps),
        [ vex_660f38_prfx, 0x2f, rm ] = trinary("vmaskmovpd",decode_rvm,vmaskmovpd),

        // VBLENDD
        [ vex_660f3a_prfx, 0x02, rm, imm8 ] = trinary("vblendd",decode_rvm,vblendd),

        // VPBROADCASTB
        [ vex_660f38_prfx, 0x78, rm ] = binary("vpbroadcastb",decode_rm,vpboradcastb),
        [ vex_660f38_prfx, 0x79, rm ] = binary("vpbroadcastw",decode_rm,vpboradcastw),
        [ vex_660f38_prfx, 0x58, rm ] = binary("vpbroadcastd",decode_rm,vpboradcastd),
        [ vex_660f38_prfx, 0x59, rm ] = binary("vpbroadcastq",decode_rm,vpboradcastq),
        [ vex_660f38_prfx, 0x5a, rm ] = binary("vpbroadcasti128",decode_rm,vpboradcasti128),

        // VPERM*
        [ vex_660f38_prfx, 0x36, rm ] = trinary("vpermd",decode_rvm,vpermd),
        [ vex_660f3a_prfx, 0x01, rm, imm8 ] = trinary("vpermpd",decode_rmi,vpermpd),
        [ vex_660f38_prfx, 0x16, rm ] = trinary("vpermps",decode_rvm,vpermps),
        [ vex_660f3a_prfx, 0x00, rm, imm8 ] = trinary("vpermq",decode_rmi,vpermq),
        [ vex_660f3a_prfx, 0x46, rm, imm8 ] = quinary("vperm2i128",decode_rvmi,vperm2i128),
        [ vex_660f38_prfx, 0x0d, rm ] = trinary("vpermilpd",decode_rvm,vpermilpd),
        [ vex_660f3a_prfx, 0x05, rm, imm8 ] = trinary("vpermilpd",decode_rmi,vpermilpd),
        [ vex_660f38_prfx, 0x0c, rm ] = trinary("vpermilps",decode_rvm,vpermilps),
        [ vex_660f3a_prfx, 0x04, rm, imm8 ] = trinary("vpermilps",decode_rmi,vpermilps),
        [ vex_660f3a_prfx, 0x06, rm, imm8 ] = quinary("vperm2f128",decode_rvmi,vperm2f128),

        // VPMASKMOV*
        [ vex_660f38_prfx, 0x8c, rm ] = trinary("vpmaskmovd",decode_rvm,vpmaskmovd),
        [ vex_660f38_prfx, 0x8e, rm ] = trinary("vpmaskmovq",decode_mvr,vpmaskmovq),

        // VPSLLVD
        [ vex_660f38_prfx, 0x47, rm ] = trinary("vpsllvd",decode_rvm,vpsllvd),
        [ vex_660f38_prfx, 0x46, rm ] = trinary("vpsravd",decode_rvm,vpsravd),
        [ vex_660f38_prfx, 0x45, rm ] = trinary("vpsrlvd",decode_rvm,vpsrlvd),

        // VTEST*
        [ vex_660f38_prfx, 0x0e, rm ] = binary("vtestpd",decode_rm,vtestpd),
        [ vex_660f38_prfx, 0x0f, rm ] = binary("vtestps",decode_rm,vtestps),

        // VZEROALL
        [ vex_0f_prfx, 0x77 ] = nonary("vzeroall",vzeroall),

        // VXOR*
        [ vex_0f_prfx, 0x57, rm ] = trinary("vpxorps",decode_rvm,vxorps),
        [ vex_660f_prfx, 0x57, rm ] = trinary("vpxorpd",decode_rvm,vxorpd))
}
