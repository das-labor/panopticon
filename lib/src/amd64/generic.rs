use disassembler::*;
use amd64::decode::*;
use amd64::semantic::*;
use amd64::*;

use std::rc::Rc;

pub fn add_generic(
    bits: u8,
    lock_prfx: Rc<Disassembler<Amd64>>,
    imm8: Rc<Disassembler<Amd64>>, imm16: Rc<Disassembler<Amd64>>, imm32: Rc<Disassembler<Amd64>>, imm48: Rc<Disassembler<Amd64>>, imm64: Rc<Disassembler<Amd64>>, imm: Rc<Disassembler<Amd64>>,
    moffs8: Rc<Disassembler<Amd64>>, moffs: Rc<Disassembler<Amd64>>,
    sib: Rc<Disassembler<Amd64>>,
    rm: Rc<Disassembler<Amd64>>, rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>, rm2: Rc<Disassembler<Amd64>>, rm3: Rc<Disassembler<Amd64>>, rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>, rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
    rmbyte: Rc<Disassembler<Amd64>>, rmbyte0: Rc<Disassembler<Amd64>>, rmbyte1: Rc<Disassembler<Amd64>>, rmbyte2: Rc<Disassembler<Amd64>>, rmbyte3: Rc<Disassembler<Amd64>>,
    rmbyte4: Rc<Disassembler<Amd64>>, rmbyte5: Rc<Disassembler<Amd64>>, rmbyte6: Rc<Disassembler<Amd64>>, rmbyte7: Rc<Disassembler<Amd64>>,
    rmlong: Rc<Disassembler<Amd64>>,
    disp8: Rc<Disassembler<Amd64>>, disp16: Rc<Disassembler<Amd64>>, disp32: Rc<Disassembler<Amd64>>, disp64: Rc<Disassembler<Amd64>>) -> (Rc<Disassembler<Amd64>>, Rc<Disassembler<Amd64>>, Rc<Disassembler<Amd64>>) {
//(main, mainrep, mainrepx)
    // AAA, AAD, AAM and AAS (32 bits only)
    /*if(bits <= 32)
    {
        main[ e(0x37)         ] = nonary("aaa",aaa);
        main[ e(0xd5) >> imm8 ] = unary("aad",decode_imm,aad);
        main[ e(0xd4) >> imm8 ] = unary("aam",decode_imm,aam);
        main[ e(0x3f)         ] = nonary("aas",aas);
    }*/

    let main = new_disassembler!(Amd64 =>
        // ADC
        [ opt!(lock_prfx), 0x14, imm8          ] = binary_reg("adc",&*al,decode_imm,adc),
        [ opt!(lock_prfx), 0x15, imm           ] = binary("adc",decode_i,adc),
        [ opt!(lock_prfx), 0x80, rmbyte2, imm8 ] = binary("adc",decode_mi,adc),
        [ opt!(lock_prfx), 0x81, rm2, imm      ] = binary("adc",decode_mi,adc),
        [ opt!(lock_prfx), 0x83, rm2, imm8     ] = binary("adc",decode_mi,adc),
        [ opt!(lock_prfx), 0x10, rmbyte        ] = binary("adc",decode_mr,adc),
        [ opt!(lock_prfx), 0x11, rm            ] = binary("adc",decode_mr,adc),
        [ opt!(lock_prfx), 0x12, rmbyte        ] = binary("adc",decode_rm,adc),
        [ opt!(lock_prfx), 0x13, rm            ] = binary("adc",decode_rm,adc));

    (main.clone(), main.clone(), main.clone())

    // ADD
    /*main[ *lock_prfx >> e(0x04) >> imm8            ] = binary("add",al,decode_imm,add);
    main[ *lock_prfx >> e(0x05) >> imm             ] = binary("add",decode_i,add);
    main[ *lock_prfx >> e(0x80) >> rmbyte0 >> imm8 ] = binary("add",decode_mi,add);
    main[ *lock_prfx >> e(0x81) >> rm0 >> imm      ] = binary("add",decode_mi,add);
    main[ *lock_prfx >> e(0x83) >> rm0 >> imm8     ] = binary("add",decode_mi,add);
    main[ *lock_prfx >> e(0x00) >> rmbyte          ] = binary("add",decode_mr,add);
    main[ *lock_prfx >> e(0x01) >> rm              ] = binary("add",decode_mr,add);
    main[ *lock_prfx >> e(0x02) >> rmbyte          ] = binary("add",decode_rm,add);
    main[ *lock_prfx >> e(0x03) >> rm              ] = binary("add",decode_rm,add);

    // ADCX
    main[ e(0x66) >> e(0x0f) >> e(0x38) >> e(0xf6) >> rm ] = binary("adcx",decode_rm,adcx);

    // AND
    main[ *lock_prfx >>	e(0x24) >> imm8				] = binary("and",al,decode_imm,and_);
    main[ *lock_prfx >>	e(0x25) >> imm         ] = binary("and",decode_i,and_);
    main[ *lock_prfx >>	e(0x81) >> rm4 >> imm  ] = binary("and",decode_mi,and_);
    main[ *lock_prfx >>	e(0x83) >> rm4 >> imm8 ] = binary("and",decode_mi,and_);
    main[ *lock_prfx >>	e(0x20) >> rmbyte			] = binary("and",decode_mr,and_);
    main[ *lock_prfx >>	e(0x21) >> rm				  ] = binary("and",decode_mr,and_);
    main[ *lock_prfx >>	e(0x22) >> rmbyte			] = binary("and",decode_rm,and_);
    main[ *lock_prfx >> e(0x23) >> rm			   	] = binary("and",decode_rm,and_);

    // ARPL
    if(Bits <= 32)
        main[ e(0x63) >> rm ] = binary("arpl",decode_mr,arpl);

    // BOUND
    if(Bits <= 32)
        main[ e(0x62) >> rm ] = binary("bound",decode_rm,bound);

    // BSF
    main[ e(0x0f) >> e(0xbc) >> rm ] = binary("bsf",decode_rm,bsf);

    // BSR
    main[ e(0x0f) >> e(0xbd) >> rm ] = binary("bsr",decode_rm,bsr);

    // BSWAP
    main[ e(0x0f) >> e(0xc8) ] = unary("bswap",reg_a,bswap);
    main[ e(0x0f) >> e(0xc9) ] = unary("bswap",reg_c,bswap);
    main[ e(0x0f) >> e(0xca) ] = unary("bswap",reg_d,bswap);
    main[ e(0x0f) >> e(0xcb) ] = unary("bswap",reg_b,bswap);
    main[ e(0x0f) >> e(0xcc) ] = unary("bswap",reg_sp,bswap);
    main[ e(0x0f) >> e(0xcd) ] = unary("bswap",reg_bp,bswap);
    main[ e(0x0f) >> e(0xce) ] = unary("bswap",reg_si,bswap);
    main[ e(0x0f) >> e(0xcf) ] = unary("bswap",reg_di,bswap);

    // BT
    main[ e(0x0f) >> e(0xa3) >> rm          ] = binary("bt",decode_rm,bt);
    main[ e(0x0f) >> e(0xba) >> rm4 >> imm8 ] = binary("bt",decode_mi,bt);

    // BTC
    main[ *lock_prfx >> e(0x0f) >> e(0xbb) >> rm          ] = binary("btc",decode_rm,btc);
    main[ *lock_prfx >> e(0x0f) >> e(0xba) >> rm7 >> imm8 ] = binary("btc",decode_mi,btc);

    // BTR
    main[ *lock_prfx >> e(0x0f) >> e(0xb3) >> rm			    ] = binary("btr",decode_rm,btr);
    main[ *lock_prfx >> e(0x0f) >> e(0xba) >> rm6 >> imm8	] = binary("btr",decode_mi,btr);

    // BTS
    main[ *lock_prfx >> e(0x0f) >> e(0xab) >> rm           ] = binary("bts",decode_rm,bts);
    main[ *lock_prfx >> e(0x0f) >> e(0xba) >> rm5 >> imm8  ] = binary("bts",decode_mi,bts);

    // CALL
    if(Bits <= 32)
    {
        main[ e(0xff) >> rm2   ] = unary("call",decode_m,std::bind(near_call,pls::_1,pls::_2,false));
        main[ e(0x9a) >> imm48 ] = unary("call",decode_d,std::bind(far_call,pls::_1,pls::_2,true));
    }

    main[ e(0xff) >> rm3 ] = unary("call",decode_m,std::bind(far_call,pls::_1,pls::_2,false));
    main[ e(0xe8) >> moffs ] = unary("call",decode_moffs,std::bind(near_call,pls::_1,pls::_2,true));

    // CBW
    main[ e(0x98) ] = conv;

    main[ e(0x99) ] = conv2;

    // CLC
    main[ e(0xf8) ] = nonary("clc",std::bind(flagwr,pls::_1,to_variable(CF),false));

    // CLD
    main[ e(0xfc) ] = nonary("cld",std::bind(flagwr,pls::_1,to_variable(DF),false));

    // CLI
    main[ e(0xfa) ] = nonary("cli",std::bind(flagwr,pls::_1,to_variable(IF),false));

    // CMC
    main[ e(0xf5) ] = nonary("cmc",std::bind(flagcomp,pls::_1,to_variable(CF)));

    // CMOVcc
    std::function<void(uint8_t, std::string const&, amd64::condition)> cmovcc = [&](uint8_t op, std::string const& suffix, amd64::condition cond)
    {
        main[                e(0x0f) >> op >> rm ] = binary("cmov" + suffix,decode_rm,std::bind(cmov,pls::_1,pls::_2,pls::_3,cond));
    };

    cmovcc(0x40,"o",Overflow);
    cmovcc(0x41,"no",NotOverflow);
    cmovcc(0x42,"c",Carry);
    cmovcc(0x43,"ae",AboveEqual);
    cmovcc(0x44,"e",Equal);
    cmovcc(0x45,"ne",NotEqual);
    cmovcc(0x46,"be",BelowEqual);
    cmovcc(0x47,"a",Above);
    cmovcc(0x48,"s",Sign);
    cmovcc(0x49,"ns",NotSign);
    cmovcc(0x4a,"p",Parity);
    cmovcc(0x4b,"np",NotParity);
    cmovcc(0x4c,"l",Less);
    cmovcc(0x4d,"ge",GreaterEqual);
    cmovcc(0x4e,"le",LessEqual);
    cmovcc(0x4f,"g",Greater);

    // CMP
    main[ e(0x3c) >> imm8        ] = binary("cmp",al,decode_imm,cmp);
    main[ e(0x3d) >> imm         ] = binary("cmp",decode_i,cmp);
    main[ e(0x81) >> rm7 >> imm  ] = binary("cmp",decode_mi,cmp);
    main[ e(0x83) >> rm7 >> imm8 ] = binary("cmp",decode_mi,cmp);
    main[ e(0x38) >> rmbyte      ] = binary("cmp",decode_mr,cmp);
    main[ e(0x39) >> rm          ] = binary("cmp",decode_mr,cmp);
    main[ e(0x3a) >> rmbyte      ] = binary("cmp",decode_rm,cmp);
    main[ e(0x3b) >> rm          ] = binary("cmp",decode_rm,cmp);

    // CMPS/CMPSW/CMPSD/CMPSQ (rep*)
    mainrepx[ e(0xa6) ] = binary("cmpsb",reg_di,reg_si,cmps);
    mainrepx[ e(0xa7) ] = binary("cmpsw",reg_di,reg_si,cmps);

    // CMPXCHG
    main[ *lock_prfx >> e(0x0f) >> e(0xb0) >> rmbyte ] = binary("cmpxchg",decode_mr,cmpxchg);
    main[ *lock_prfx >> e(0x0f) >> e(0xb1) >> rm ] = binary("cmpxchg",decode_mr,cmpxchg);

    // CMPXCHG8B
    main[ *lock_prfx >> e(0x0f) >> e(0xc7) >> rm1 >> m64 ] = unary("cmpxchg8b",decode_m,std::bind(cmpxchg8b,pls::_1,pls::_2));

    // CMPXCHG16B
    if(Bits == 64)
        main[ *lock_prfx >> e(0x0f) >> e(0xc7) >> rm1 >> m128 ] = unary("cmpxchg16b",decode_m,std::bind(cmpxchg16b,pls::_1,pls::_2));

    // CPUID
    main[ e(0x0f) >> e(0xa2) ] = nonary("cpuid",cpuid);

    // DAS
    if(Bits <= 32)
        main[ e(0x2f) ] = nonary("das",das);

    // DEC
    main[ *lock_prfx >> e(0xfe) >> rmbyte1 ] = unary("dec",decode_m,dec);
    main[ *lock_prfx >> e(0xff) >> rm1 ] = unary("dec",decode_m,dec);

    if(Bits < 64)
    {
        main[ *lock_prfx >> e(0x48) ] = unary("dec",reg_a,dec);
        main[ *lock_prfx >> e(0x49) ] = unary("dec",reg_c,dec);
        main[ *lock_prfx >> e(0x4a) ] = unary("dec",reg_d,dec);
        main[ *lock_prfx >> e(0x4b) ] = unary("dec",reg_b,dec);
        main[ *lock_prfx >> e(0x4c) ] = unary("dec",reg_sp,dec);
        main[ *lock_prfx >> e(0x4d) ] = unary("dec",reg_bp,dec);
        main[ *lock_prfx >> e(0x4e) ] = unary("dec",reg_si,dec);
        main[ *lock_prfx >> e(0x4f) ] = unary("dec",reg_di,dec);
    }

    // DIV
    main[ e(0xf6) >> rmbyte6 ] = unary("div",decode_m,div);
    main[ e(0xf7) >> rm6 ] = unary("div",decode_m,div);

    // DAA
    if(Bits <= 32)
        main[ e(0x27) ] = nonary("daa",daa);

    // ENTER
    main[ e(0xc8) >> imm16 >> imm8 ] = binary("enter",decode_ii,enter);

    // HLT
    main[ e(0xf4) ] = nonary("hlt",hlt);

    // IDIV
    main[ e(0xf6) >> rmbyte7 ] = unary("idiv",decode_m,idiv);
    main[ e(0xf7) >> rm7     ] = unary("idiv",decode_m,idiv);

    // IMUL
    main[ e(0xf6) >> rmbyte5      ] = unary("imul",decode_m,imul1);
    main[ e(0xf7) >> rm5          ] = unary("imul",decode_m,imul1);
    main[ e(0x6b) >> rm >> imm8   ] = trinary("imul",decode_rmi,imul3);
    main[ e(0x69) >> rm >> imm    ] = trinary("imul",decode_rmi,imul3);
    main[ e(0x0f) >> e(0xaf) >> rm ] = binary("imul",decode_rm,imul2);

    // IN
    main[ e(0xe4) >> imm8 ] = binary("in",al,decode_imm,in);
    main[ e(0xe5) >> imm8 ] = binary("in",decode_i,in);
    main[ e(0xec)         ] = binary("in",al,dx,in);
    main[ e(0xed)         ] = binary("in",reg_a,dx,in);

    // INC
    main[ *lock_prfx >> e(0xfe) >> rmbyte0 ] = unary("inc",decode_m,inc);
    main[ *lock_prfx >> e(0xff) >> rm0 ] = unary("inc",decode_m,inc);

    if(Bits < 64)
    {
        main[ *lock_prfx >> e(0x40) ] = unary("inc",reg_a,inc);
        main[ *lock_prfx >> e(0x41) ] = unary("inc",reg_c,inc);
        main[ *lock_prfx >> e(0x42) ] = unary("inc",reg_d,inc);
        main[ *lock_prfx >> e(0x43) ] = unary("inc",reg_b,inc);
        main[ *lock_prfx >> e(0x44) ] = unary("inc",reg_sp,inc);
        main[ *lock_prfx >> e(0x45) ] = unary("inc",reg_bp,inc);
        main[ *lock_prfx >> e(0x46) ] = unary("inc",reg_si,inc);
        main[ *lock_prfx >> e(0x47) ] = unary("inc",reg_di,inc);
    }

    // INS* (rep)
    mainrep[ e(0x6c) ] = binary("insb",reg_di,dx,ins);
    mainrep[ e(0x6d) ] = binary("ins",reg_di,dx,ins);

    // INT
    main[ e(0xcc)         ] = unary("int",rvalue(constant(3)),int_);
    main[ e(0xce)         ] = nonary("into",into);
    main[ e(0xcd) >> imm8 ] = unary("int",decode_imm,int_);

    // ICEBP
    main[ e(0xf1) ] = nonary("icebp",icebp);

    // IRET*
    main[ e(0xcf) ] = iret;

    // J*CXZ
    if(Bits == 16)
    {
        main[ e(0xe3) >> imm8 ] = unary("jcxz",decode_imm,std::bind(jxz,pls::_1,pls::_2,cx));
    }
    else if(Bits == 32)
    {
        main[ e(0xe3) >> imm8 ] = unary("jecxz",decode_imm,std::bind(jxz,pls::_1,pls::_2,ecx));
    }
    else if(Bits == 64)
    {
        main[ e(0xe3) >> imm8 ] = unary("jrcxz",decode_imm,std::bind(jxz,pls::_1,pls::_2,rcx));
    }

    // Jcc
    std::function<void(uint8_t, std::string const&, amd64::condition)> _jcc = [&](uint8_t op, std::string const& suffix, amd64::condition cond)
    {
        main[            op >> imm8        ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));
        main[ e(0x0f) >> (op + 0x10) >> imm ] = unary("j" + suffix,decode_imm,std::bind(jcc,pls::_1,pls::_2,cond));
    };

    _jcc(0x70,"o",Overflow);
    _jcc(0x71,"no",NotOverflow);
    _jcc(0x72,"c",Carry);
    _jcc(0x73,"ae",AboveEqual);
    _jcc(0x74,"e",Equal);
    _jcc(0x75,"ne",NotEqual);
    _jcc(0x76,"be",BelowEqual);
    _jcc(0x77,"a",Above);
    _jcc(0x78,"s",Sign);
    _jcc(0x79,"ns",NotSign);
    _jcc(0x7a,"p",Parity);
    _jcc(0x7b,"np",NotParity);
    _jcc(0x7c,"l",Less);
    _jcc(0x7d,"ge",GreaterEqual);
    _jcc(0x7e,"le",LessEqual);
    _jcc(0x7f,"g",Greater);

    // JMP
    main[ e(0xeb) >> imm8   ] = unary("jmp",decode_d,jmp);

    if(Bits == 16)
    {
        main[ e(0xe9) >> moffs ] = unary("jmp",decode_moffs,jmp);
        main[ e(0xea) >> imm32 ] = unary("jmp",decode_d,jmp);
        main[ e(0xff) >> rm4   ] = unary("jmp",decode_m,jmp);
        main[ e(0xff) >> rm5   ] = unary("jmp",decode_d,jmp);
    }
    else if(Bits == 32)
    {
        main[ e(0xe9) >> moffs ] = unary("jmp",decode_moffs,jmp);
        main[ e(0xea) >> imm48 ] = unary("jmp",decode_d,jmp);
        main[ e(0xff) >> rm4   ] = unary("jmp",decode_m,jmp);
        main[ e(0xff) >> rm5   ] = unary("jmp",decode_d,jmp);
    }
    else if(Bits == 64)
    {
        main[ e(0xe9) >> moffs ] = unary("jmp",decode_moffs,jmp);
        main[ e(0xff) >> rm4   ] = unary("jmp",decode_m,jmp);
        main[ e(0xff) >> rm5   ] = unary("jmp",decode_d,jmp);
    }

    // LAHF
    if(Bits <= 32)
        main[ e(0x9f) ] = nonary("lahf",lahf);

    // LAR
    main[ e(0x0f) >> e(0x02) >> rm ] = binary("lar",decode_rm,lar);

    // LDS
    if(Bits <= 32)
    {
        main[ e(0xc5) >> rm ] = binary("lds",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,ds));
    }

    // LEA
    main[ e(0x8d) >> rm ] = binary("lea",decode_rm,lea);

    // LEAVE
    main[ e(0xc9) ] = leave;

    // LES
    if(Bits <= 32)
        main[ e(0xc4) >> rm ] = binary("les",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,es));

    // LFS
    main[ e(0x0f) >> e(0xb4) >> rm ] = binary("lfs",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,fs));

    // LGS
    main[ e(0x0f) >> e(0xb5) >> rm ] = binary("lgs",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,gs));

    // LODS*
    mainrep[ e(0xac) ] = lodsb;
    mainrep[ e(0xad) ] = lods;

    // LOOP
    main[ e(0xe2) >> imm8 ] = loop;

    // LOOPNE
    main[ e(0xe0) >> imm8 ] = loopne;

    // LOOPE
    main[ e(0xe1) >> imm8 ] = loope;

    // LSS
    main[ e(0x0f) >> e(0xb2) >> rm ] = binary("lss",decode_rm,std::bind(lxs,pls::_1,pls::_2,pls::_3,ss));

    // MOV
    main[ e(0x88) >> rmbyte ] = binary("mov",decode_mr,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x89) >> rm     ] = binary("mov",decode_mr,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x8a) >> rmbyte ] = binary("mov",decode_rm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x8b) >> rm     ] = binary("mov",decode_rm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x8e) >> rm     ] = binary("mov",decode_msreg,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x8c) >> rm     ] = binary("mov",decode_sregm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xa0) >> moffs8 ] = binary("mov",decode_fd,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xa1) >> moffs  ] = binary("mov",decode_fd,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xa2) >> moffs8 ] = binary("mov",decode_td,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xa3) >> moffs  ] = binary("mov",decode_td,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

    main[ e(0xb0) >> imm8 ] = binary("mov",regb_a,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb1) >> imm8 ] = binary("mov",regb_c,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb2) >> imm8 ] = binary("mov",regb_d,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb3) >> imm8 ] = binary("mov",regb_b,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb4) >> imm8 ] = binary("mov",regb_sp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb5) >> imm8 ] = binary("mov",regb_bp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb6) >> imm8 ] = binary("mov",regb_si,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb7) >> imm8 ] = binary("mov",regb_di,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

    main[ e(0xb8) >> immlong ] = binary("mov",reg_a,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xb9) >> immlong ] = binary("mov",reg_c,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xba) >> immlong ] = binary("mov",reg_d,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xbb) >> immlong ] = binary("mov",reg_b,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xbc) >> immlong ] = binary("mov",reg_sp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xbd) >> immlong ] = binary("mov",reg_bp,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xbe) >> immlong ] = binary("mov",reg_si,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xbf) >> immlong ] = binary("mov",reg_di,decode_imm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

    main[ e(0xc6) >> rmbyte0 >> imm8 ] = binary("mov",decode_mi,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0xc7) >> rm0 >> imm      ] = binary("mov",decode_mi,std::bind(mov,pls::_1,pls::_2,pls::_3,true));

    main[ e(0x0f) >> e(0x20) >> rmlong ] = binary("mov",decode_rmctrl,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x0f) >> e(0x22) >> rmlong ] = binary("mov",decode_ctrlrm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x0f) >> e(0x21) >> rmlong ] = binary("mov",decode_rmdbg,std::bind(mov,pls::_1,pls::_2,pls::_3,false));
    main[ e(0x0f) >> e(0x23) >> rmlong ] = binary("mov",decode_dbgrm,std::bind(mov,pls::_1,pls::_2,pls::_3,false));

    // MOVBE
    main[ e(0x0f) >> e(0x38) >> e(0xf0) >> rm ] = binary("movbe",decode_rm,movbe);
    main[ e(0x0f) >> e(0x38) >> e(0xf1) >> rm ] = binary("movbe",decode_mr,movbe);

    // MOVS*
    mainrep[ e(0xa4) ] = movsb;
    mainrep[ e(0xa5) ] = movs;

    // MOVSX*
    main[ e(0x0f) >> e(0xbe) >> rm ] = binary("movsx",decode_rm,movsx);
    main[ e(0x0f) >> e(0xbf) >> rm ] = binary("movsx",decode_rm,movsx);

    if(Bits == 64)
        main[ e(0x63) >> rm ] = binary("movsxd",decode_rm,movsx);

    // MOVZX
    main[ e(0x0f) >> e(0xb6) >> rm ] = binary("movzx",decode_rm,movzx);
    main[ e(0x0f) >> e(0xb7) >> rm ] = binary("movzx",decode_rm,movzx);


    // MUL
    main[ e(0xf6) >> rmbyte4 ] = unary("mul",decode_m,mul);
    main[ e(0xf7) >> rm4     ] = unary("mul",decode_m,mul);

    // NEG
    main[ *lock_prfx >> e(0xf6) >> rmbyte3 ] = unary("neg",decode_m,neg);
    main[ *lock_prfx >> e(0xf7) >> rm3     ] = unary("neg",decode_m,neg);

    // NOP
    main[ e(0x0f) >> e(0x1f) >> rm0 ] = nonary("nop",nop);

    // NOT (lock)
    main[ *lock_prfx >> e(0xf6) >> rmbyte2 ] = unary("not",decode_m,not_);
    main[ *lock_prfx >> e(0xf7) >> rm2     ] = unary("not",decode_m,not_);

    // OR
    main[ *lock_prfx >> e(0x0c) >> imm8        ] = binary("or",al,decode_imm,or_);
    main[ *lock_prfx >> e(0x0d) >> imm         ] = binary("or",decode_i,or_);
    main[ *lock_prfx >> e(0x81) >> rm1 >> imm  ] = binary("or",decode_mi,or_);
    main[ *lock_prfx >> e(0x83) >> rm1 >> imm8 ] = binary("or",decode_mi,or_);
    main[ *lock_prfx >> e(0x08) >> rmbyte      ] = binary("or",decode_mr,or_);
    main[ *lock_prfx >> e(0x09) >> rm          ] = binary("or",decode_mr,or_);
    main[ *lock_prfx >> e(0x0a) >> rmbyte      ] = binary("or",decode_rm,or_);
    main[ *lock_prfx >> e(0x0b) >> rm          ] = binary("or",decode_rm,or_);

    // OUT
    main[ e(0xe6) >> imm8 ] = binary("out",al,decode_imm,out);
    main[ e(0xe7) >> imm8 ] = binary("out",decode_i,out);

    main[ e(0xee) ] = binary("out",al,dx,out);
    main[ e(0xef) ] = binary("out",reg_a,dx,out);

    // OUTS* (rep)
    mainrep[ e(0x6e) ] = outs;
    mainrep[ e(0x6f) ] = outs;

    main[ e(0x8f) >> rm0    ] = pop;
    main[ e(0x58)           ] = pop;
    main[ e(0x59)           ] = pop;
    main[ e(0x5a)           ] = pop;
    main[ e(0x5b)           ] = pop;
    main[ e(0x5c)           ] = pop;
    main[ e(0x5d)           ] = pop;
    main[ e(0x5e)           ] = pop;
    main[ e(0x5f)           ] = pop;
    main[ e(0x0f) >> e(0xa1) ] = pop;
    main[ e(0x0f) >> e(0xa9) ] = pop;

    if(Bits <= 32)
    {
        main[  e(0x1f) ] = pop;
        main[  e(0x07) ] = pop;
        main[  e(0x17) ] = pop;
    }

    // POPA*
    if(Bits != 64)
        main[ e(0x61) ] = popa;

    // POPCNT
    main[ e(0xf3) >> e(0x0f) >> e(0xb8) >> rm ] = binary("popcnt",decode_rm,popcnt);

    // POPF*
    main[ e(0x9d) ] = unary("popf",decode_m,popf);

    // PUSH
    main[ e(0xff) >> rm6    ] = push;
    main[ e(0x50)           ] = push;
    main[ e(0x51)           ] = push;
    main[ e(0x52)           ] = push;
    main[ e(0x53)           ] = push;
    main[ e(0x54)           ] = push;
    main[ e(0x55)           ] = push;
    main[ e(0x56)           ] = push;
    main[ e(0x57)           ] = push;
    main[ e(0x0f) >> e(0xa0) ] = push;
    main[ e(0x0f) >> e(0xa8) ] = push;

    if(Bits <= 32)
    {
        main[ e(0x0e) ] = push;
        main[ e(0x1e) ] = push;
        main[ e(0x06) ] = push;
        main[ e(0x16) ] = push;
    }

    main[ e(0x6a) >> imm8 ] = push;
    main[ e(0x68) >> imm  ] = push;

    // PUSHA*
    if(Bits != 64)
        main[ e(0x60) ] = pusha;

    // PUSHF*
    main[ e(0x9d) ] = unary("push",decode_m,pushf);

    // RCL
    main[ e(0xd0) >> rmbyte2         ] = binary("rcl",decode_m,rvalue(constant(1)),rcl);
    main[ e(0xd1) >> rm2             ] = binary("rcl",decode_m,rvalue(constant(1)),rcl);
    main[ e(0xd2) >> rmbyte2         ] = binary("rcl",decode_m,CF,rcl);
    main[ e(0xd3) >> rm2             ] = binary("rcl",decode_m,CF,rcl);
    main[ e(0xc0) >> rmbyte2 >> imm8 ] = binary("rcl",decode_mi,rcl);
    main[ e(0xc1) >> rm2 >> imm8     ] = binary("rcl",decode_mi,rcl);

    // RCR
    main[ e(0xd0) >> rmbyte3         ] = binary("rcr",decode_m,rvalue(constant(1)),rcr);
    main[ e(0xd1) >> rm3             ] = binary("rcr",decode_m,rvalue(constant(1)),rcr);
    main[ e(0xd2) >> rmbyte3         ] = binary("rcr",decode_m,CF,rcr);
    main[ e(0xd3) >> rm3             ] = binary("rcr",decode_m,CF,rcr);
    main[ e(0xc0) >> rmbyte3 >> imm8 ] = binary("rcr",decode_mi,rcr);
    main[ e(0xc1) >> rm3 >> imm8     ] = binary("rcr",decode_mi,rcr);

    // RET*
    main[ e(0xc3)          ] = unary("ret",rvalue(constant(0)),ret);
    main[ e(0xcb)          ] = unary("retf",rvalue(constant(0)),retf);
    main[ e(0xc2) >> imm16 ] = unary("ret",decode_imm,ret);
    main[ e(0xca) >> imm16 ] = unary("retf",decode_imm,retf);

    // ROL
    main[ e(0xd0) >> rmbyte0         ] = binary("rol",decode_m,rvalue(constant(1)),rol);
    main[ e(0xd1) >> rm0             ] = binary("rol",decode_m,rvalue(constant(1)),rol);
    main[ e(0xd2) >> rmbyte0         ] = binary("rol",decode_m,CF,rol);
    main[ e(0xd3) >> rm0             ] = binary("rol",decode_m,CF,rol);
    main[ e(0xc0) >> rmbyte0 >> imm8 ] = binary("rol",decode_mi,rol);
    main[ e(0xc1) >> rm0 >> imm8     ] = binary("rol",decode_mi,rol);

    // ROR
    main[ e(0xd0) >> rmbyte1         ] = binary("ror",decode_m,rvalue(constant(1)),ror);
    main[ e(0xd1) >> rm1             ] = binary("ror",decode_m,rvalue(constant(1)),ror);
    main[ e(0xd2) >> rmbyte1         ] = binary("ror",decode_m,CF,ror);
    main[ e(0xd3) >> rm1             ] = binary("ror",decode_m,CF,ror);
    main[ e(0xc0) >> rmbyte1 >> imm8 ] = binary("ror",decode_mi,ror);
    main[ e(0xc1) >> rm1 >> imm8     ] = binary("ror",decode_mi,ror);

    // SAHF
    main[ e(0x9e) ] = nonary("sahf",sahf);

    // SAL
    main[ e(0xd0) >> rmbyte4         ] = binary("sal",decode_m,rvalue(constant(1)),sal);
    main[ e(0xd1) >> rm4             ] = binary("sal",decode_m,rvalue(constant(1)),sal);
    main[ e(0xd2) >> rmbyte4         ] = binary("sal",decode_m,CF,sal);
    main[ e(0xd3) >> rm4             ] = binary("sal",decode_m,CF,sal);
    main[ e(0xc0) >> rmbyte4 >> imm8 ] = binary("sal",decode_mi,sal);
    main[ e(0xc1) >> rm4 >> imm8     ] = binary("sal",decode_mi,sal);

    // SALC/SETALC
    main[ e(0xd6) ] = nonary("salc",salc);

    // SAR
    main[ e(0xd0) >> rmbyte7         ] = binary("sar",decode_m,rvalue(constant(1)),sar);
    main[ e(0xd1) >> rm7             ] = binary("sar",decode_m,rvalue(constant(1)),sar);
    main[ e(0xd2) >> rmbyte7         ] = binary("sar",decode_m,CF,sar);
    main[ e(0xd3) >> rm7             ] = binary("sar",decode_m,CF,sar);
    main[ e(0xc0) >> rmbyte7 >> imm8 ] = binary("sar",decode_mi,sar);
    main[ e(0xc1) >> rm7 >> imm8     ] = binary("sar",decode_mi,sar);

    // SBB
    main[ *lock_prfx >> e(0x1c) >> imm8            ] = binary("sbb",al,decode_imm,sbb);
    main[ *lock_prfx >> e(0x1d) >> imm             ] = binary("sbb",decode_i,sbb);
    main[ *lock_prfx >> e(0x80) >> rmbyte3 >> imm8 ] = binary("sbb",decode_mi,sbb);
    main[ *lock_prfx >> e(0x81) >> rm3 >> imm      ] = binary("sbb",decode_mi,sbb);
    main[ *lock_prfx >> e(0x83) >> rm3 >> imm8	    ] = binary("sbb",decode_mi,sbb);
    main[ *lock_prfx >> e(0x18) >> rmbyte          ] = binary("sbb",decode_mr,sbb);
    main[ *lock_prfx >> e(0x19) >> rm              ] = binary("sbb",decode_mr,sbb);
    main[ *lock_prfx >> e(0x1a) >> rmbyte          ] = binary("sbb",decode_rm,sbb);
    main[ *lock_prfx >> e(0x1b) >> rm              ] = binary("sbb",decode_rm,sbb);

    // SCAS* (rep*)
    mainrep[ e(0xae) ] = scas;
    mainrep[ e(0xaf) ] = scas;

    // SETcc
    main[ e(0x0f) >> e(0x90) >> rmbyte ] = unary("seto",decode_m,std::bind(setcc,pls::_1,pls::_2,Overflow));
    main[ e(0x0f) >> e(0x91) >> rmbyte ] = unary("setno",decode_m,std::bind(setcc,pls::_1,pls::_2,NotOverflow));
    main[ e(0x0f) >> e(0x92) >> rmbyte ] = unary("setc",decode_m,std::bind(setcc,pls::_1,pls::_2,Carry));
    main[ e(0x0f) >> e(0x93) >> rmbyte ] = unary("setae",decode_m,std::bind(setcc,pls::_1,pls::_2,AboveEqual));
    main[ e(0x0f) >> e(0x94) >> rmbyte ] = unary("sete",decode_m,std::bind(setcc,pls::_1,pls::_2,Equal));
    main[ e(0x0f) >> e(0x95) >> rmbyte ] = unary("setne",decode_m,std::bind(setcc,pls::_1,pls::_2,NotEqual));
    main[ e(0x0f) >> e(0x96) >> rmbyte ] = unary("setbe",decode_m,std::bind(setcc,pls::_1,pls::_2,BelowEqual));
    main[ e(0x0f) >> e(0x97) >> rmbyte ] = unary("seta",decode_m,std::bind(setcc,pls::_1,pls::_2,Above));
    main[ e(0x0f) >> e(0x98) >> rmbyte ] = unary("sets",decode_m,std::bind(setcc,pls::_1,pls::_2,Sign));
    main[ e(0x0f) >> e(0x99) >> rmbyte ] = unary("setns",decode_m,std::bind(setcc,pls::_1,pls::_2,NotSign));
    main[ e(0x0f) >> e(0x9a) >> rmbyte ] = unary("setp",decode_m,std::bind(setcc,pls::_1,pls::_2,Parity));
    main[ e(0x0f) >> e(0x9b) >> rmbyte ] = unary("setnp",decode_m,std::bind(setcc,pls::_1,pls::_2,NotParity));
    main[ e(0x0f) >> e(0x9c) >> rmbyte ] = unary("setl",decode_m,std::bind(setcc,pls::_1,pls::_2,Less));
    main[ e(0x0f) >> e(0x9d) >> rmbyte ] = unary("setge",decode_m,std::bind(setcc,pls::_1,pls::_2,GreaterEqual));
    main[ e(0x0f) >> e(0x9e) >> rmbyte ] = unary("setle",decode_m,std::bind(setcc,pls::_1,pls::_2,LessEqual));
    main[ e(0x0f) >> e(0x9f) >> rmbyte ] = unary("setg",decode_m,std::bind(setcc,pls::_1,pls::_2,Greater));

    // SHLD
    main[ e(0x0f) >> e(0xa4) >> rm >> imm8 ] = trinary("shld",decode_mri,shld);
    main[ e(0x0f) >> e(0xa5) >> rm         ] = trinary("shld",decode_mr,CF,shld);

    // SHR
    main[ e(0xd0) >> rmbyte5         ] = binary("shr",decode_m,rvalue(constant(1)),shr);
    main[ e(0xd1) >> rm5             ] = binary("shr",decode_m,rvalue(constant(1)),shr);
    main[ e(0xd2) >> rmbyte5         ] = binary("shr",decode_m,CF,shr);
    main[ e(0xd3) >> rm5             ] = binary("shr",decode_m,CF,shr);
    main[ e(0xc0) >> rmbyte5 >> imm8 ] = binary("shr",decode_mi,shr);
    main[ e(0xc1) >> rm5 >> imm8     ] = binary("shr",decode_mi,shr);

    // SHRD
    main[ e(0x0f) >> e(0xac) >> rm >> imm8 ] = trinary("shrd",decode_mri,shrd);
    main[ e(0x0f) >> e(0xad) >> rm         ] = trinary("shrd",decode_mr,CF,shrd);

    // STC
    main[ e(0xf9) ] = nonary("stc",std::bind(flagwr,pls::_1,to_variable(CF),true));

    // STD
    main[ e(0xfd) ] = nonary("std",std::bind(flagwr,pls::_1,to_variable(DF),true));

    // STI
    main[ e(0xfb) ] = nonary("sti",std::bind(flagwr,pls::_1,to_variable(IF),true));

    // STOS* (rep)
    mainrep[ e(0xaa) ] = stos;
    mainrep[ e(0xab) ] = stos;

    // SUB
    main[ *lock_prfx >> e(0x2c) >> imm8        ] = binary("sub",al,decode_imm,sub);
    main[ *lock_prfx >> e(0x2d) >> imm         ] = binary("sub",decode_i,sub);
    main[ *lock_prfx >> e(0x81) >> rm5 >> imm  ] = binary("sub",decode_mi,sub);
    main[ *lock_prfx >> e(0x83) >> rm5 >> imm8 ] = binary("sub",decode_mi,sub);
    main[ *lock_prfx >> e(0x28) >> rmbyte      ] = binary("sub",decode_mr,sub);
    main[ *lock_prfx >> e(0x29) >> rm          ] = binary("sub",decode_mr,sub);
    main[ *lock_prfx >> e(0x2a) >> rmbyte      ] = binary("sub",decode_rm,sub);
    main[ *lock_prfx >> e(0x2b) >> rm          ] = binary("sub",decode_rm,sub);

    // TEST
    main[ e(0xa8) >> imm8            ] = binary("test",al,decode_imm,test);
    main[ e(0xa9) >> imm             ] = binary("test",decode_i,test);
    main[ e(0xf6) >> rmbyte0 >> imm8 ] = binary("test",decode_mi,test);
    main[ e(0xf7) >> rm0 >> imm      ] = binary("test",decode_mi,test);
    main[ e(0x84) >> rmbyte           ] = binary("test",decode_mr,test);
    main[ e(0x85) >> rm              ] = binary("test",decode_mr,test);

    // UD1
    main[ e(0x0f) >> e(0xb9) ] = nonary("ud1",ud1);

    // UD2
    main[ e(0x0f) >> e(0x0b) ] = nonary("ud2",ud2);

    // XADD (lock)
    main[ e(0x0f) >> e(0xc0) >> rmbyte ] = binary("xadd",decode_mr,xadd);
    main[ e(0x0f) >> e(0xc1) >> rm     ] = binary("xadd",decode_mr,xadd);

    // XCHG (lock)
    main[ e(0x90)           ] = binary("xchg",regb_a,regd_a,xchg);
    main[ e(0x91)           ] = binary("xchg",regb_a,regd_c,xchg);
    main[ e(0x92)           ] = binary("xchg",regb_a,regd_d,xchg);
    main[ e(0x93)           ] = binary("xchg",regb_a,regd_b,xchg);
    main[ e(0x94)           ] = binary("xchg",regb_a,regd_sp,xchg);
    main[ e(0x95)           ] = binary("xchg",regb_a,regd_bp,xchg);
    main[ e(0x96)           ] = binary("xchg",regb_a,regd_si,xchg);
    main[ e(0x97)           ] = binary("xchg",regb_a,regd_di,xchg);
    main[ e(0x86) >> rmbyte ] = binary("xchg",decode_mr,xchg);
    main[ e(0x87) >> rm     ] = binary("xchg",decode_mr,xchg);

    // XOR
    main[ *lock_prfx >> e(0x34) >> imm8        ] = binary("xor",al,decode_imm,xor_);
    main[ *lock_prfx >> e(0x35) >> imm         ] = binary("xor",decode_i,xor_);
    main[ *lock_prfx >> e(0x81) >> rm6 >> imm  ] = binary("xor",decode_mi,xor_);
    main[ *lock_prfx >> e(0x83) >> rm6 >> imm8 ] = binary("xor",decode_mi,xor_);
    main[ *lock_prfx >> e(0x30) >> rmbyte      ] = binary("xor",decode_mr,xor_);
    main[ *lock_prfx >> e(0x31) >> rm          ] = binary("xor",decode_mr,xor_);
    main[ *lock_prfx >> e(0x32) >> rmbyte      ] = binary("xor",decode_rm,xor_);
    main[ *lock_prfx >> e(0x33) >> rm          ] = binary("xor",decode_rm,xor_);*/
}
