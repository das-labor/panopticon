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

use {
    Disassembler,
    State,
    CodeGen,
    Rvalue,
};

use amd64::decode::*;
use amd64::semantic::*;
use amd64::*;

use std::rc::Rc;

pub fn integer_lockable(imm8: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          imm: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, rm: Rc<Disassembler<Amd64>>,
                          rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>,
                          rm2: Rc<Disassembler<Amd64>>, rm3: Rc<Disassembler<Amd64>>,
                          rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
                          rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
                          rmbyte: Rc<Disassembler<Amd64>>, rmbyte0: Rc<Disassembler<Amd64>>,
                          rmbyte1: Rc<Disassembler<Amd64>>, rmbyte2: Rc<Disassembler<Amd64>>,
                          rmbyte3: Rc<Disassembler<Amd64>>, rmbyte4: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          m64: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // ADC
        [ 0x14, imm8          ] = binary_rv("adc",&*AL,decode_imm,adc),
        [ 0x15, imm           ] = binary("adc",decode_i,adc),
        [ 0x80, rmbyte2, imm8 ] = binary("adc",decode_mi,adc),
        [ 0x81, rm2, imm      ] = binary("adc",decode_mi,adc),
        [ 0x83, rm2, imm8     ] = binary("adc",decode_mi,adc),
        [ 0x10, rmbyte        ] = binary("adc",decode_mr,adc),
        [ 0x11, rm            ] = binary("adc",decode_mr,adc),
        [ 0x12, rmbyte        ] = binary("adc",decode_rm,adc),
        [ 0x13, rm            ] = binary("adc",decode_rm,adc),

        // ADD
        [ 0x04, imm8          ] = binary_rv("add",&*AL,decode_imm,add),
        [ 0x05, imm           ] = binary("add",decode_i,add),
        [ 0x80, rmbyte0, imm8 ] = binary("add",decode_mi,add),
        [ 0x81, rm0, imm      ] = binary("add",decode_mi,add),
        [ 0x83, rm0, imm8     ] = binary("add",decode_mi,add),
        [ 0x00, rmbyte        ] = binary("add",decode_mr,add),
        [ 0x01, rm            ] = binary("add",decode_mr,add),
        [ 0x02, rmbyte        ] = binary("add",decode_rm,add),
        [ 0x03, rm            ] = binary("add",decode_rm,add),

        // AND
        [ 0x24, imm8          ] = binary_rv("and",&*AL,decode_imm,and),
        [ 0x25, imm           ] = binary("and",decode_i,and),
        [ 0x80, rmbyte4, imm8 ] = binary("and",decode_mi,and),
        [ 0x81, rm4, imm      ] = binary("and",decode_mi,and),
        [ 0x83, rm4, imm8     ] = binary("and",decode_mi,and),
        [ 0x20, rmbyte        ] = binary("and",decode_mr,and),
        [ 0x21, rm            ] = binary("and",decode_mr,and),
        [ 0x22, rmbyte        ] = binary("and",decode_rm,and),
        [ 0x23, rm            ] = binary("and",decode_rm,and),

        // BTC
        [ 0x0f, 0xbb, rm        ] = binary("btc",decode_rm,btc),
        [ 0x0f, 0xba, rm7, imm8 ] = binary("btc",decode_mi,btc),

        // BTR
        [ 0x0f, 0xb3, rm        ] = binary("btr",decode_rm,btr),
        [ 0x0f, 0xba, rm6, imm8 ] = binary("btr",decode_mi,btr),

        // BTS
        [ 0x0f, 0xab, rm        ] = binary("bts",decode_rm,bts),
        [ 0x0f, 0xba, rm5, imm8 ] = binary("bts",decode_mi,bts),

        // CMPXCHG
        [ 0x0f, 0xb0, rmbyte ] = binary("cmpxchg",decode_mr,cmpxchg),
        [ 0x0f, 0xb1, rm     ] = binary("cmpxchg",decode_mr,cmpxchg),

        // CMPXCHG8B
        [ 0x0f, 0xc7, rm1, m64 ] = unary("cmpxchg8b",decode_m,cmpxchg8b),

        // DEC
        [ 0xfe, rmbyte1 ] = unary("dec",decode_m,dec),
        [ 0xff, rm1     ] = unary("dec",decode_m,dec),

        // INC
        [ 0xfe, rmbyte0 ] = unary("inc",decode_m,inc),
        [ 0xff, rm0     ] = unary("inc",decode_m,inc),

        // NEG
        [ 0xf6, rmbyte3 ] = unary("neg",decode_m,neg),
        [ 0xf7, rm3     ] = unary("neg",decode_m,neg),

        // NOT
        [ 0xf6, rmbyte2 ] = unary("not",decode_m,not),
        [ 0xf7, rm2     ] = unary("not",decode_m,not),

        // OR
        [ 0x0c, imm8          ] = binary_rv("or",&*AL,decode_imm,or),
        [ 0x0d, imm           ] = binary("or",decode_i,or),
        [ 0x80, rmbyte1, imm8 ] = binary("or",decode_mi,or),
        [ 0x81, rm1, imm      ] = binary("or",decode_mi,or),
        [ 0x83, rm1, imm8     ] = binary("or",decode_mi,or),
        [ 0x08, rmbyte        ] = binary("or",decode_mr,or),
        [ 0x09, rm            ] = binary("or",decode_mr,or),
        [ 0x0a, rmbyte        ] = binary("or",decode_rm,or),
        [ 0x0b, rm            ] = binary("or",decode_rm,or),

        // SBB
        [ 0x1c, imm8          ] = binary_rv("sbb",&*AL,decode_imm,sbb),
        [ 0x1d, imm           ] = binary("sbb",decode_i,sbb),
        [ 0x80, rmbyte3, imm8 ] = binary("sbb",decode_mi,sbb),
        [ 0x81, rm3, imm      ] = binary("sbb",decode_mi,sbb),
        [ 0x83, rm3, imm8     ] = binary("sbb",decode_mi,sbb),
        [ 0x18, rmbyte        ] = binary("sbb",decode_mr,sbb),
        [ 0x19, rm            ] = binary("sbb",decode_mr,sbb),
        [ 0x1a, rmbyte        ] = binary("sbb",decode_rm,sbb),
        [ 0x1b, rm            ] = binary("sbb",decode_rm,sbb),

        // SUB
        [ 0x2c, imm8      ] = binary_rv("sub",&*AL,decode_imm,sub),
        [ 0x2d, imm       ] = binary("sub",decode_i,sub),
        [ 0x81, rm5, imm  ] = binary("sub",decode_mi,sub),
        [ 0x83, rm5, imm8 ] = binary("sub",decode_mi,sub),
        [ 0x28, rmbyte    ] = binary("sub",decode_mr,sub),
        [ 0x29, rm        ] = binary("sub",decode_mr,sub),
        [ 0x2a, rmbyte    ] = binary("sub",decode_rm,sub),
        [ 0x2b, rm        ] = binary("sub",decode_rm,sub),

        // XADD
        [ 0x0f, 0xc0, rmbyte ] = binary("xadd",decode_mr,xadd),
        [ 0x0f, 0xc1, rm     ] = binary("xadd",decode_mr,xadd),

        // XCHG
        [ 0x90         ] = binary_vv("xchg",regb_a,regd_a,xchg),
        [ 0x91         ] = binary_vv("xchg",regb_a,regd_c,xchg),
        [ 0x92         ] = binary_vv("xchg",regb_a,regd_d,xchg),
        [ 0x93         ] = binary_vv("xchg",regb_a,regd_b,xchg),
        [ 0x94         ] = binary_vv("xchg",regb_a,regd_sp,xchg),
        [ 0x95         ] = binary_vv("xchg",regb_a,regd_bp,xchg),
        [ 0x96         ] = binary_vv("xchg",regb_a,regd_si,xchg),
        [ 0x97         ] = binary_vv("xchg",regb_a,regd_di,xchg),
        [ 0x86, rmbyte ] = binary("xchg",decode_mr,xchg),
        [ 0x87, rm     ] = binary("xchg",decode_mr,xchg),

        // XOR
        [ 0x34, imm8      ] = binary_rv("xor",&*AL,decode_imm,xor),
        [ 0x35, imm       ] = binary("xor",decode_i,xor),
        [ 0x81, rm6, imm  ] = binary("xor",decode_mi,xor),
        [ 0x83, rm6, imm8 ] = binary("xor",decode_mi,xor),
        [ 0x30, rmbyte    ] = binary("xor",decode_mr,xor),
        [ 0x31, rm        ] = binary("xor",decode_mr,xor),
        [ 0x32, rmbyte    ] = binary("xor",decode_rm,xor),
        [ 0x33, rm        ] = binary("xor",decode_rm,xor))
}

pub fn integer_universial(imm8: Rc<Disassembler<Amd64>>,
                          imm16: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          imm: Rc<Disassembler<Amd64>>, immlong: Rc<Disassembler<Amd64>>,
                          moffs8: Rc<Disassembler<Amd64>>, moffs: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, rm: Rc<Disassembler<Amd64>>,
                          rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>,
                          rm2: Rc<Disassembler<Amd64>>, rm3: Rc<Disassembler<Amd64>>,
                          rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
                          rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
                          rmbyte: Rc<Disassembler<Amd64>>, rmbyte0: Rc<Disassembler<Amd64>>,
                          rmbyte1: Rc<Disassembler<Amd64>>, rmbyte2: Rc<Disassembler<Amd64>>,
                          rmbyte3: Rc<Disassembler<Amd64>>, rmbyte4: Rc<Disassembler<Amd64>>,
                          rmbyte5: Rc<Disassembler<Amd64>>, rmbyte6: Rc<Disassembler<Amd64>>,
                          rmbyte7: Rc<Disassembler<Amd64>>, rmlong: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                          _: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    fn cmovcc(cond: Condition) -> Box<Fn(&mut CodeGen<Amd64>,Rvalue,Rvalue)> {
        Box::new(move |cg: &mut CodeGen<Amd64>,a: Rvalue,b: Rvalue| {
            cmov(cg,a,b,cond)
        })
    }

    fn _setcc(cond: Condition) -> Box<Fn(&mut CodeGen<Amd64>,Rvalue)> {
        Box::new(move |cg: &mut CodeGen<Amd64>,a: Rvalue| {
            setcc(cg,a,cond)
        })
    }


    new_disassembler!(Amd64 =>
        // ADCX
        [ 0x66, 0x0f, 0x38, 0xf6, rm ] = binary("adcx",decode_rm,adcx),

        // BSF
        [ 0x0f, 0xbc, rm ] = binary("bsf",decode_rm,bsf),

        // BSR
        [ 0x0f, 0xbd, rm ] = binary("bsr",decode_rm,bsr),

        // BSWAP
        [ 0x0f, 0xc8 ] = unary("bswap",reg_a,bswap),
        [ 0x0f, 0xc9 ] = unary("bswap",reg_c,bswap),
        [ 0x0f, 0xca ] = unary("bswap",reg_d,bswap),
        [ 0x0f, 0xcb ] = unary("bswap",reg_b,bswap),
        [ 0x0f, 0xcc ] = unary("bswap",reg_sp,bswap),
        [ 0x0f, 0xcd ] = unary("bswap",reg_bp,bswap),
        [ 0x0f, 0xce ] = unary("bswap",reg_si,bswap),
        [ 0x0f, 0xcf ] = unary("bswap",reg_di,bswap),

        // BT
        [ 0x0f, 0xa3, rm        ] = binary("bt",decode_rm,bt),
        [ 0x0f, 0xba, rm4, imm8 ] = binary("bt",decode_mi,bt),

        // CALL
        [ 0xff, rm2   ] = unary("call",decode_m,far_call),
        [ 0xff, rm3   ] = unary("call",decode_m,far_call),
        [ 0xe8, moffs ] = unary("call",decode_moffs,near_rcall),

        // CBW
        [ 0x98 ] = conv,

        [ 0x99 ] = conv2,

        // CLC
        [ 0xf8 ] = nonary("clc",clc),

        // CLD
        [ 0xfc ] = nonary("cld",cld),

        // CLI
        [ 0xfa ] = nonary("cli",cli),

        // CMC
        [ 0xf5 ] = nonary("cmc",cmc),

        // CMOVcc
        [ 0x0f, 0x40, rm ] = binary_box("cmovo",decode_rm,cmovcc(Condition::Overflow)),
        [ 0x0f, 0x41, rm ] = binary_box("cmovno",decode_rm,cmovcc(Condition::NotOverflow)),
        [ 0x0f, 0x42, rm ] = binary_box("cmovc",decode_rm,cmovcc(Condition::Carry)),
        [ 0x0f, 0x43, rm ] = binary_box("cmovae",decode_rm,cmovcc(Condition::AboveEqual)),
        [ 0x0f, 0x44, rm ] = binary_box("cmove",decode_rm,cmovcc(Condition::Equal)),
        [ 0x0f, 0x45, rm ] = binary_box("cmovne",decode_rm,cmovcc(Condition::NotEqual)),
        [ 0x0f, 0x46, rm ] = binary_box("cmovbe",decode_rm,cmovcc(Condition::BelowEqual)),
        [ 0x0f, 0x47, rm ] = binary_box("cmova",decode_rm,cmovcc(Condition::Above)),
        [ 0x0f, 0x48, rm ] = binary_box("cmovs",decode_rm,cmovcc(Condition::Sign)),
        [ 0x0f, 0x49, rm ] = binary_box("cmovns",decode_rm,cmovcc(Condition::NotSign)),
        [ 0x0f, 0x4a, rm ] = binary_box("cmovp",decode_rm,cmovcc(Condition::Parity)),
        [ 0x0f, 0x4b, rm ] = binary_box("cmovnp",decode_rm,cmovcc(Condition::NotParity)),
        [ 0x0f, 0x4c, rm ] = binary_box("cmovl",decode_rm,cmovcc(Condition::Less)),
        [ 0x0f, 0x4d, rm ] = binary_box("cmovge",decode_rm,cmovcc(Condition::GreaterEqual)),
        [ 0x0f, 0x4e, rm ] = binary_box("cmovle",decode_rm,cmovcc(Condition::LessEqual)),
        [ 0x0f, 0x4f, rm ] = binary_box("cmovg",decode_rm,cmovcc(Condition::Greater)),

        // CMP
        [ 0x3c, imm8      ] = binary_rv("cmp",&*AL,decode_imm,cmp),
        [ 0x3d, imm       ] = binary("cmp",decode_i,cmp),
        [ 0x80, rm7, imm8 ] = binary("cmp",decode_mi,cmp),
        [ 0x81, rm7, imm  ] = binary("cmp",decode_mi,cmp),
        [ 0x83, rm7, imm8 ] = binary("cmp",decode_mi,cmp),
        [ 0x38, rmbyte    ] = binary("cmp",decode_mr,cmp),
        [ 0x39, rm        ] = binary("cmp",decode_mr,cmp),
        [ 0x3a, rmbyte    ] = binary("cmp",decode_rm,cmp),
        [ 0x3b, rm        ] = binary("cmp",decode_rm,cmp),

        // CPUID
        [ 0x0f, 0xa2 ] = nonary("cpuid",cpuid),

        // DIV
        [ 0xf6, rmbyte6 ] = unary("div",decode_m,div),
        [ 0xf7, rm6     ] = unary("div",decode_m,div),

        // ENTER
        [ 0xc8, imm16, imm8 ] = binary("enter",decode_ii,enter),

        // HLT
        [ 0xf4 ] = nonary("hlt",hlt),

        // IDIV
        [ 0xf6, rmbyte7 ] = unary("idiv",decode_m,idiv),
        [ 0xf7, rm7     ] = unary("idiv",decode_m,idiv),

        // IMUL
        [ 0xf6, rmbyte5  ] = unary("imul",decode_m,imul1),
        [ 0xf7, rm5      ] = unary("imul",decode_m,imul1),
        [ 0x6b, rm, imm8 ] = trinary("imul",decode_rmi,imul3),
        [ 0x69, rm, imm  ] = trinary("imul",decode_rmi,imul3),
        [ 0x0f, 0xaf, rm ] = binary("imul",decode_rm,imul2),

        // IN
        [ 0xe4, imm8 ] = binary_rv("in",&*AL,decode_imm,in_),
        [ 0xe5, imm8 ] = binary("in",decode_i,in_),
        [ 0xec       ] = binary_rr("in",&*AL,&*DX,in_),
        [ 0xed       ] = binary_vr("in",reg_a,&*DX,in_),

        // INT
        [ 0xcc       ] = unary_c("int",Rvalue::new_u64(3),int),
        [ 0xce       ] = nonary("into",into),
        [ 0xcd, imm8 ] = unary("int",decode_imm,int),

        // ICEBP
        [ 0xf1 ] = nonary("icebp",icebp),

        // IRET*
        [ 0xcf ] = iret,

        // Jcc
        [ 0x70, imm8      ] = branch("jo",decode_imm,Condition::Overflow),
        [ 0x71, imm8      ] = branch("jno",decode_imm,Condition::NotOverflow),
        [ 0x72, imm8      ] = branch("jc",decode_imm,Condition::Carry),
        [ 0x73, imm8      ] = branch("jae",decode_imm,Condition::AboveEqual),
        [ 0x74, imm8      ] = branch("je",decode_imm,Condition::Equal),
        [ 0x75, imm8      ] = branch("jne",decode_imm,Condition::NotEqual),
        [ 0x76, imm8      ] = branch("jbe",decode_imm,Condition::BelowEqual),
        [ 0x77, imm8      ] = branch("ja",decode_imm,Condition::Above),
        [ 0x78, imm8      ] = branch("js",decode_imm,Condition::Sign),
        [ 0x79, imm8      ] = branch("jns",decode_imm,Condition::NotSign),
        [ 0x7a, imm8      ] = branch("jp",decode_imm,Condition::Parity),
        [ 0x7b, imm8      ] = branch("jnp",decode_imm,Condition::NotParity),
        [ 0x7c, imm8      ] = branch("jl",decode_imm,Condition::Less),
        [ 0x7d, imm8      ] = branch("jge",decode_imm,Condition::GreaterEqual),
        [ 0x7e, imm8      ] = branch("jle",decode_imm,Condition::LessEqual),
        [ 0x7f, imm8      ] = branch("jg",decode_imm,Condition::Greater),

        [ 0x0f, 0x80, imm ] = branch("jo",decode_imm,Condition::Overflow),
        [ 0x0f, 0x81, imm ] = branch("jno",decode_imm,Condition::NotOverflow),
        [ 0x0f, 0x82, imm ] = branch("jc",decode_imm,Condition::Carry),
        [ 0x0f, 0x83, imm ] = branch("jae",decode_imm,Condition::AboveEqual),
        [ 0x0f, 0x84, imm ] = branch("je",decode_imm,Condition::Equal),
        [ 0x0f, 0x85, imm ] = branch("jne",decode_imm,Condition::NotEqual),
        [ 0x0f, 0x86, imm ] = branch("jbe",decode_imm,Condition::BelowEqual),
        [ 0x0f, 0x87, imm ] = branch("ja",decode_imm,Condition::Above),
        [ 0x0f, 0x88, imm ] = branch("js",decode_imm,Condition::Sign),
        [ 0x0f, 0x89, imm ] = branch("jns",decode_imm,Condition::NotSign),
        [ 0x0f, 0x8a, imm ] = branch("jp",decode_imm,Condition::Parity),
        [ 0x0f, 0x8b, imm ] = branch("jnp",decode_imm,Condition::NotParity),
        [ 0x0f, 0x8c, imm ] = branch("jl",decode_imm,Condition::Less),
        [ 0x0f, 0x8d, imm ] = branch("jge",decode_imm,Condition::GreaterEqual),
        [ 0x0f, 0x8e, imm ] = branch("jle",decode_imm,Condition::LessEqual),
        [ 0x0f, 0x8f, imm ] = branch("jg",decode_imm,Condition::Greater),

        // JMP
        [ 0xeb, imm8   ] = unary("jmp",decode_d,jmp),

        // LAR
        [ 0x0f, 0x02, rm ] = binary("lar",decode_rm,lar),

        // LEA
        [ 0x8d, rm ] = binary("lea",decode_rm,lea),

        // LEAVE
        [ 0xc9 ] = leave,

        // LFS
        [ 0x0f, 0xb4, rm ] = binary("lfs",decode_rm,lfs),

        // LGS
        [ 0x0f, 0xb5, rm ] = binary("lgs",decode_rm,lgs),

        // LOOP
        [ 0xe2, imm8 ] = loop_,

        // LOOPNE
        [ 0xe0, imm8 ] = loopne,

        // LOOPE
        [ 0xe1, imm8 ] = loope,

        // LSS
        [ 0x0f, 0xb2, rm ] = binary("lss",decode_rm,lss),

        // MOV
        [ 0x88, rmbyte ] = binary("mov",decode_mr,mov),
        [ 0x89, rm     ] = binary("mov",decode_mr,mov),
        [ 0x8a, rmbyte ] = binary("mov",decode_rm,mov),
        [ 0x8b, rm     ] = binary("mov",decode_rm,mov),
        [ 0x8e, rm     ] = binary("mov",decode_msreg,mov),
        [ 0x8c, rm     ] = binary("mov",decode_sregm,mov),
        [ 0xa0, moffs8 ] = binary("mov",decode_fd,mov),
        [ 0xa1, moffs  ] = binary("mov",decode_fd,mov),
        [ 0xa2, moffs8 ] = binary("mov",decode_td,mov),
        [ 0xa3, moffs  ] = binary("mov",decode_td,mov),

        [ 0xb0, imm8 ] = binary_vv("mov",regb_a,decode_imm,mov),
        [ 0xb1, imm8 ] = binary_vv("mov",regb_c,decode_imm,mov),
        [ 0xb2, imm8 ] = binary_vv("mov",regb_d,decode_imm,mov),
        [ 0xb3, imm8 ] = binary_vv("mov",regb_b,decode_imm,mov),
        [ 0xb4, imm8 ] = binary_vv("mov",regb_sp,decode_imm,mov),
        [ 0xb5, imm8 ] = binary_vv("mov",regb_bp,decode_imm,mov),
        [ 0xb6, imm8 ] = binary_vv("mov",regb_si,decode_imm,mov),
        [ 0xb7, imm8 ] = binary_vv("mov",regb_di,decode_imm,mov),

        [ 0xb8, immlong ] = binary_vv("mov",reg_a,decode_imm,mov),
        [ 0xb9, immlong ] = binary_vv("mov",reg_c,decode_imm,mov),
        [ 0xba, immlong ] = binary_vv("mov",reg_d,decode_imm,mov),
        [ 0xbb, immlong ] = binary_vv("mov",reg_b,decode_imm,mov),
        [ 0xbc, immlong ] = binary_vv("mov",reg_sp,decode_imm,mov),
        [ 0xbd, immlong ] = binary_vv("mov",reg_bp,decode_imm,mov),
        [ 0xbe, immlong ] = binary_vv("mov",reg_si,decode_imm,mov),
        [ 0xbf, immlong ] = binary_vv("mov",reg_di,decode_imm,mov),

        [ 0xc6, rmbyte0, imm8 ] = binary("mov",decode_mi,mov),
        [ 0xc7, rm0, imm      ] = binary("mov",decode_mi,movsx),

        [ 0x0f, 0x20, rmlong ] = binary("mov",decode_rmctrl,mov),
        [ 0x0f, 0x22, rmlong ] = binary("mov",decode_ctrlrm,mov),
        [ 0x0f, 0x21, rmlong ] = binary("mov",decode_rmdbg,mov),
        [ 0x0f, 0x23, rmlong ] = binary("mov",decode_dbgrm,mov),

        // MOVBE
        [ 0x0f, 0x38, 0xf0, rm ] = binary("movbe",decode_rm,movbe),
        [ 0x0f, 0x38, 0xf1, rm ] = binary("movbe",decode_mr,movbe),

        // MOVSX*
        [ 0x0f, 0xbe, rm ] = binary("movsx",decode_rm,movsx),
        [ 0x0f, 0xbf, rm ] = binary("movsx",decode_rm,movsx),

        // MOVZX
        [ 0x0f, 0xb6, rm ] = binary("movzx",decode_rm,movzx),
        [ 0x0f, 0xb7, rm ] = binary("movzx",decode_rm,movzx),


        // MUL
        [ 0xf6, rmbyte4 ] = unary("mul",decode_m,mul),
        [ 0xf7, rm4     ] = unary("mul",decode_m,mul),

        // NOP
        [ 0x0f, 0x1f, rm0 ] = nonary("nop",nop),

        // OUT
        [ 0xe6, imm8 ] = binary_rv("out",&*AL,decode_imm,out),
        [ 0xe7, imm8 ] = binary("out",decode_i,out),

        [ 0xee ] = binary_rr("out",&*AL,&*DX,out),
        [ 0xef ] = binary_vr("out",reg_a,&*DX,out),

        [ 0x8f, rm0  ] = pop,
        [ 0x58       ] = pop,
        [ 0x59       ] = pop,
        [ 0x5a       ] = pop,
        [ 0x5b       ] = pop,
        [ 0x5c       ] = pop,
        [ 0x5d       ] = pop,
        [ 0x5e       ] = pop,
        [ 0x5f       ] = pop,
        [ 0x0f, 0xa1 ] = pop,
        [ 0x0f, 0xa9 ] = pop,

        // POPCNT
        [ 0xf3, 0x0f, 0xb8, rm ] = binary("popcnt",decode_rm,popcnt),

        // POPF*
        [ 0x9d ] = unary("popf",decode_m,popf),

        // PUSH
        [ 0xff, rm6  ] = push,
        [ 0x50       ] = push,
        [ 0x51       ] = push,
        [ 0x52       ] = push,
        [ 0x53       ] = push,
        [ 0x54       ] = push,
        [ 0x55       ] = push,
        [ 0x56       ] = push,
        [ 0x57       ] = push,
        [ 0x0f, 0xa0 ] = push,
        [ 0x0f, 0xa8 ] = push,
        [ 0x6a, imm8 ] = push,
        [ 0x68, imm  ] = push,

        // PUSHF*
        [ 0x9d ] = unary("push",decode_m,pushf),

        // RCL
        [ 0xd0, rmbyte2       ] = binary_vc("rcl",decode_m,Rvalue::new_u64(1),rcl),
        [ 0xd1, rm2           ] = binary_vc("rcl",decode_m,Rvalue::new_u64(1),rcl),
        [ 0xd2, rmbyte2       ] = binary_vr("rcl",decode_m,&*CF,rcl),
        [ 0xd3, rm2           ] = binary_vr("rcl",decode_m,&*CF,rcl),
        [ 0xc0, rmbyte2, imm8 ] = binary("rcl",decode_mi,rcl),
        [ 0xc1, rm2, imm8     ] = binary("rcl",decode_mi,rcl),

        // RCR
        [ 0xd0, rmbyte3       ] = binary_vc("rcr",decode_m,Rvalue::new_u64(1),rcr),
        [ 0xd1, rm3           ] = binary_vc("rcr",decode_m,Rvalue::new_u64(1),rcr),
        [ 0xd2, rmbyte3       ] = binary_vr("rcr",decode_m,&*CF,rcr),
        [ 0xd3, rm3           ] = binary_vr("rcr",decode_m,&*CF,rcr),
        [ 0xc0, rmbyte3, imm8 ] = binary("rcr",decode_mi,rcr),
        [ 0xc1, rm3, imm8     ] = binary("rcr",decode_mi,rcr),

        // RET*
        [ 0xc3        ] = unary_c("ret",Rvalue::new_u64(0),ret),
        [ 0xcb        ] = unary_c("retf",Rvalue::new_u64(0),retf),
        [ 0xc2, imm16 ] = unary("ret",decode_imm,ret),
        [ 0xca, imm16 ] = unary("retf",decode_imm,retf),

        // ROL
        [ 0xd0, rmbyte0       ] = binary_vc("rol",decode_m,Rvalue::new_u64(1),rol),
        [ 0xd1, rm0           ] = binary_vc("rol",decode_m,Rvalue::new_u64(1),rol),
        [ 0xd2, rmbyte0       ] = binary_vr("rol",decode_m,&*CF,rol),
        [ 0xd3, rm0           ] = binary_vr("rol",decode_m,&*CF,rol),
        [ 0xc0, rmbyte0, imm8 ] = binary("rol",decode_mi,rol),
        [ 0xc1, rm0, imm8     ] = binary("rol",decode_mi,rol),

        // ROR
        [ 0xd0, rmbyte1       ] = binary_vc("ror",decode_m,Rvalue::new_u64(1),ror),
        [ 0xd1, rm1           ] = binary_vc("ror",decode_m,Rvalue::new_u64(1),ror),
        [ 0xd2, rmbyte1       ] = binary_vr("ror",decode_m,&*CF,ror),
        [ 0xd3, rm1           ] = binary_vr("ror",decode_m,&*CF,ror),
        [ 0xc0, rmbyte1, imm8 ] = binary("ror",decode_mi,ror),
        [ 0xc1, rm1, imm8     ] = binary("ror",decode_mi,ror),

        // RDTSC
        [ 0x0f, 0x31 ] = nonary("rdtsc",rdtsc),

        // SAHF
        [ 0x9e ] = nonary("sahf",sahf),

        // SAL
        [ 0xd0, rmbyte4       ] = binary_vc("sal",decode_m,Rvalue::new_u64(1),sal),
        [ 0xd1, rm4           ] = binary_vc("sal",decode_m,Rvalue::new_u64(1),sal),
        [ 0xd2, rmbyte4       ] = binary_vr("sal",decode_m,&*CF,sal),
        [ 0xd3, rm4           ] = binary_vr("sal",decode_m,&*CF,sal),
        [ 0xc0, rmbyte4, imm8 ] = binary("sal",decode_mi,sal),
        [ 0xc1, rm4, imm8     ] = binary("sal",decode_mi,sal),

        // SALC/SETALC
        [ 0xd6 ] = nonary("salc",salc),

        // SAR
        [ 0xd0, rmbyte7       ] = binary_vc("sar",decode_m,Rvalue::new_u64(1),sar),
        [ 0xd1, rm7           ] = binary_vc("sar",decode_m,Rvalue::new_u64(1),sar),
        [ 0xd2, rmbyte7       ] = binary_vr("sar",decode_m,&*CF,sar),
        [ 0xd3, rm7           ] = binary_vr("sar",decode_m,&*CF,sar),
        [ 0xc0, rmbyte7, imm8 ] = binary("sar",decode_mi,sar),
        [ 0xc1, rm7, imm8     ] = binary("sar",decode_mi,sar),

        // SETcc
        [ 0x0f, 0x90, rmbyte ] = unary_box("seto",decode_m,_setcc(Condition::Overflow)),
        [ 0x0f, 0x91, rmbyte ] = unary_box("setno",decode_m,_setcc(Condition::NotOverflow)),
        [ 0x0f, 0x92, rmbyte ] = unary_box("setc",decode_m,_setcc(Condition::Carry)),
        [ 0x0f, 0x93, rmbyte ] = unary_box("setae",decode_m,_setcc(Condition::AboveEqual)),
        [ 0x0f, 0x94, rmbyte ] = unary_box("sete",decode_m,_setcc(Condition::Equal)),
        [ 0x0f, 0x95, rmbyte ] = unary_box("setne",decode_m,_setcc(Condition::NotEqual)),
        [ 0x0f, 0x96, rmbyte ] = unary_box("setbe",decode_m,_setcc(Condition::BelowEqual)),
        [ 0x0f, 0x97, rmbyte ] = unary_box("seta",decode_m,_setcc(Condition::Above)),
        [ 0x0f, 0x98, rmbyte ] = unary_box("sets",decode_m,_setcc(Condition::Sign)),
        [ 0x0f, 0x99, rmbyte ] = unary_box("setns",decode_m,_setcc(Condition::NotSign)),
        [ 0x0f, 0x9a, rmbyte ] = unary_box("setp",decode_m,_setcc(Condition::Parity)),
        [ 0x0f, 0x9b, rmbyte ] = unary_box("setnp",decode_m,_setcc(Condition::NotParity)),
        [ 0x0f, 0x9c, rmbyte ] = unary_box("setl",decode_m,_setcc(Condition::Less)),
        [ 0x0f, 0x9d, rmbyte ] = unary_box("setge",decode_m,_setcc(Condition::GreaterEqual)),
        [ 0x0f, 0x9e, rmbyte ] = unary_box("setle",decode_m,_setcc(Condition::LessEqual)),
        [ 0x0f, 0x9f, rmbyte ] = unary_box("setg",decode_m,_setcc(Condition::Greater)),

        // SHLD
        [ 0x0f, 0xa4, rm, imm8 ] = trinary("shld",decode_mri,shld),
        [ 0x0f, 0xa5, rm       ] = trinary_vr("shld",decode_mr,&*CF,shld),

        // SHR
        [ 0xd0, rmbyte5       ] = binary_vc("shr",decode_m,Rvalue::new_u64(1),shr),
        [ 0xd1, rm5           ] = binary_vc("shr",decode_m,Rvalue::new_u64(1),shr),
        [ 0xd2, rmbyte5       ] = binary_vr("shr",decode_m,&*CF,shr),
        [ 0xd3, rm5           ] = binary_vr("shr",decode_m,&*CF,shr),
        [ 0xc0, rmbyte5, imm8 ] = binary("shr",decode_mi,shr),
        [ 0xc1, rm5, imm8     ] = binary("shr",decode_mi,shr),

        // SHRD
        [ 0x0f, 0xac, rm, imm8 ] = trinary("shrd",decode_mri,shrd),
        [ 0x0f, 0xad, rm       ] = trinary_vr("shrd",decode_mr,&*CF,shrd),

        // STC
        [ 0xf9 ] = nonary("stc",stc),

        // STD
        [ 0xfd ] = nonary("std",std),

        // STI
        [ 0xfb ] = nonary("sti",sti),

        // TEST
        [ 0xa8, imm8          ] = binary_rv("test",&*AL,decode_imm,test),
        [ 0xa9, imm           ] = binary("test",decode_i,test),
        [ 0xf6, rmbyte0, imm8 ] = binary("test",decode_mi,test),
        [ 0xf7, rm0, imm      ] = binary("test",decode_mi,test),
        [ 0x84, rmbyte        ] = binary("test",decode_mr,test),
        [ 0x85, rm            ] = binary("test",decode_mr,test),

        // UD1
        [ 0x0f, 0xb9 ] = nonary("ud1",ud1),

        // UD2
        [ 0x0f, 0x0b ] = nonary("ud2",ud2),

        // XGETBV
        [ 0x0f, 0x01, 0xd0 ] = nonary("xgetbv",xgetbv))

}

pub fn integer_rep() -> (Rc<Disassembler<Amd64>>,Rc<Disassembler<Amd64>>) {
    let rep = new_disassembler!(Amd64 =>
        // INS*
        [ 0x6c ] = binary_vr("insb",reg_di,&*DX,ins),
        [ 0x6d ] = binary_vr("ins",reg_di,&*DX,ins),

        // OUTS*
        [ 0x6e ] = binary_vr("outsb",reg_di,&*DX,outs),
        [ 0x6f ] = binary_vr("outs",reg_di,&*DX,outs),

        // LODS*
        [ 0xac ] = lodsb,
        [ 0xad ] = lods,

        // MOVS*
        [ 0xa4 ] = movsb,
        [ 0xa5 ] = movs,

        // STOS*
        [ 0xaa ] = stos,
        [ 0xab ] = stos,

        // SCAS*
        [ 0xae ] = scas,
        [ 0xaf ] = scas);

    let repx = new_disassembler!(Amd64 =>
        // CMPS/CMPSW/CMPSD/CMPSQ
        [ 0xa6 ] = binary_vv("cmpsb",reg_di,reg_si,cmps),
        [ 0xa7 ] = binary_vv("cmpsw",reg_di,reg_si,cmps));

    (rep,repx)
}


pub fn integer_16bit(imm8: Rc<Disassembler<Amd64>>, imm32: Rc<Disassembler<Amd64>>,
                     moffs: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // JCXZ
        [ 0xe3, imm8 ] = unary("jcxz",decode_imm,jcxz),

        // JMP
        [ 0xe9, moffs ] = unary("jmp",decode_moffs,jmp),
        [ 0xea, imm32 ] = unary("jmp",decode_d,jmp),
        [ 0xff, rm4   ] = unary("jmp",decode_m,jmp),
        [ 0xff, rm5   ] = unary("jmp",decode_d,jmp))
}

pub fn integer_32bit(imm8: Rc<Disassembler<Amd64>>, imm48: Rc<Disassembler<Amd64>>,
                     moffs: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // JECX
        [ 0xe3, imm8 ] = unary("jecxz",decode_imm,jecxz),

        // JMP
        [ 0xe9, moffs ] = unary("jmp",decode_moffs,jmp),
        [ 0xea, imm48 ] = unary("jmp",decode_d,jmp),
        [ 0xff, rm4   ] = unary("jmp",decode_m,jmp),
        [ 0xff, rm5   ] = unary("jmp",decode_d,jmp))
}

pub fn lockable_64bit(_: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     m128: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // CMPXCHG16B
        [ 0x0f, 0xc7, rm1, m128 ] = unary("cmpxchg16b",decode_m,cmpxchg16b))
}

pub fn integer_64bit(imm8: Rc<Disassembler<Amd64>>,
                     moffs: Rc<Disassembler<Amd64>>,
                     rm: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     m128: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // CMPXCHG16B
        [ 0x0f, 0xc7, rm1, m128 ] = unary("cmpxchg16b",decode_m,cmpxchg16b),

        // JRCXZ
        [ 0xe3, imm8 ] = unary("jrcxz",decode_imm,jrcxz),

        // MOVSXD
        [ 0x63, rm ] = binary("movsxd",decode_rm,movsx),

        // JMP
        [ 0xe9, moffs ] = unary("jmp",decode_moffs,jmp),
        [ 0xff, rm4   ] = unary("jmp",decode_m,jmp),
        [ 0xff, rm5   ] = unary("jmp",decode_d,jmp),

        // SYSCALL
        [ 0x0f, 0x05   ] = nonary("syscall",syscall),

        // SYSCALL
        [ 0x0f, 0x07   ] = nonary("sysret",sysret))
}

pub fn lockable_32bit_or_less(
                     _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // DEC
        [ 0x48 ] = unary("dec",reg_a,dec),
        [ 0x49 ] = unary("dec",reg_c,dec),
        [ 0x4a ] = unary("dec",reg_d,dec),
        [ 0x4b ] = unary("dec",reg_b,dec),
        [ 0x4c ] = unary("dec",reg_sp,dec),
        [ 0x4d ] = unary("dec",reg_bp,dec),
        [ 0x4e ] = unary("dec",reg_si,dec),
        [ 0x4f ] = unary("dec",reg_di,dec),

        // INC
        [ 0x40 ] = unary("inc",reg_a,inc),
        [ 0x41 ] = unary("inc",reg_c,inc),
        [ 0x42 ] = unary("inc",reg_d,inc),
        [ 0x43 ] = unary("inc",reg_b,inc),
        [ 0x44 ] = unary("inc",reg_sp,inc),
        [ 0x45 ] = unary("inc",reg_bp,inc),
        [ 0x46 ] = unary("inc",reg_si,inc),
        [ 0x47 ] = unary("inc",reg_di,inc))
}

pub fn integer_32bit_or_less(imm8: Rc<Disassembler<Amd64>>,
                     imm48: Rc<Disassembler<Amd64>>,
                     rm: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     rm2: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>,
                     _: Rc<Disassembler<Amd64>>, _: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // AAA
        [ 0x37 ] = nonary("aaa",aaa),

        // AAD
        [ 0xd5, imm8 ] = unary("aad",decode_imm,aad),

        // AAM
        [ 0xd4, imm8 ] = unary("aam",decode_imm,aam),

        // AAS
        [ 0x3f ] = nonary("aas",aas),

        // ARPL
        [ 0x63, rm ] = binary("arpl",decode_mr,arpl),

        // BOUND
        [ 0x62, rm ] = binary("bound",decode_rm,bound),

        // CALL
        [ 0xff, rm2   ] = unary("call",decode_m,near_call),
        [ 0x9a, imm48 ] = unary("call",decode_d,far_rcall),

        // DAS
        [ 0x2f ] = nonary("das",das),

        // DAA
        [ 0x27 ] = nonary("daa",daa),

        // LAHF
        [ 0x9f ] = nonary("lahf",lahf),

        // LDS
        [ 0xc5, rm ] = binary("lds",decode_rm,lds),

        // LES
        [ 0xc4, rm ] = binary("les",decode_rm,les),

        // POP
        [ 0x1f ] = pop,
        [ 0x07 ] = pop,
        [ 0x17 ] = pop,

        // POPA*
        [ 0x61 ] = popa,

        // PUSH
        [ 0x0e ] = push,
        [ 0x1e ] = push,
        [ 0x06 ] = push,
        [ 0x16 ] = push,

        // PUSHA*
        [ 0x60 ] = pusha)
}


