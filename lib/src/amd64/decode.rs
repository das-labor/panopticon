use value::{Lvalue,Rvalue,Endianess};
use disassembler::State;
use amd64::*;
use codegen::CodeGen;

fn byte(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 1,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn word(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 2,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn dword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 4,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn qword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 8,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn oword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 16,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

pub fn decode_m(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    sm.configuration.rm.as_ref().unwrap().to_rv()
}

pub fn decode_d(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    if let Some(Rvalue::Constant(c)) = sm.configuration.imm {
        if c <= 0xffffffff {
            Rvalue::Constant(c >> 16 | ((c & 0xffff) << 16))
        } else {
            Rvalue::Constant(c >> 32 | ((c & 0xffffffff) << 32))
        }
    } else {
        unreachable!()
    }
}

pub fn decode_imm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    sm.configuration.imm.clone().unwrap()
}

pub fn decode_moffs(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    sm.configuration.moffs.clone().unwrap()
}

pub fn decode_rm1(sm: &mut State<Amd64>,cg: &mut CodeGen) -> Rvalue {
    sm.configuration.rm.as_ref().unwrap().to_rv()
}

pub fn decode_i(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    match &sm.configuration.operand_size {
        &OperandSize::Eight => (ah.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::Sixteen => (ax.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::ThirtyTwo => (eax.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::SixtyFour => (rax.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::HundredTwentyEight => panic!("No 128 bit register in x86!")
    }
}

pub fn decode_rm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        (reg.to_rv(),rm.to_rv())
    } else {
        unreachable!();
    }
}

pub fn decode_mr(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    let (a,b) = decode_rm(sm,cg);
    (b,a)
}

pub fn decode_fd(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    (select_reg(&sm.configuration.operand_size,0,sm.configuration.rex).to_rv(),
    select_mem(&sm.configuration.operand_size,sm.configuration.moffs.clone().unwrap()).to_rv())
}

pub fn decode_td(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    let (a,b) = decode_fd(sm,cg);
    (b,a)
}

pub fn decode_msreg(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    let (a,b) = decode_sregm(sm,cg);
    (b,a)
}

pub fn decode_sregm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        if *reg == *ax || *reg == *eax  {
            (es.to_rv(),rm.to_rv())
        } else if *reg == *cx || *reg == *ecx  {
            (cs.to_rv(),rm.to_rv())
        } else if *reg == *dx || *reg == *edx  {
            (ss.to_rv(),rm.to_rv())
        } else if *reg == *bx || *reg == *ebx  {
            (ds.to_rv(),rm.to_rv())
        } else if *reg == *sp || *reg == *esp  {
            (fs.to_rv(),rm.to_rv())
        } else if *reg == *bp || *reg == *ebp  {
            (gs.to_rv(),rm.to_rv())
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn decode_dbgrm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        if *reg == *rax || *reg == *eax  {
            (dr0.to_rv(),rm.to_rv())
        } else if *reg == *rcx || *reg == *ecx  {
            (dr1.to_rv(),rm.to_rv())
        } else if *reg == *rdx || *reg == *edx  {
            (dr2.to_rv(),rm.to_rv())
        } else if *reg == *rbx || *reg == *ebx  {
            (dr3.to_rv(),rm.to_rv())
        } else if *reg == *rsp || *reg == *esp  {
            (dr4.to_rv(),rm.to_rv())
        } else if *reg == *rbp || *reg == *ebp  {
            (dr5.to_rv(),rm.to_rv())
        } else if *reg == *rdi || *reg == *edi  {
            (dr7.to_rv(),rm.to_rv())
        } else if *reg == *rsi || *reg == *esi  {
            (dr6.to_rv(),rm.to_rv())
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn decode_rmdbg(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    let (a,b) = decode_dbgrm(sm,cg);
    (b,a)
}

pub fn decode_ctrlrm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        if *reg == *rax || *reg == *eax  {
            (cr0.to_rv(),rm.to_rv())
        } else if *reg == *rdx || *reg == *edx  {
            (cr2.to_rv(),rm.to_rv())
        } else if *reg == *rbx || *reg == *ebx  {
            (cr3.to_rv(),rm.to_rv())
        } else if *reg == *rsp || *reg == *esp  {
            (cr4.to_rv(),rm.to_rv())
        } else if *reg == *r9 || *reg == *r9w  {
            (cr8.to_rv(),rm.to_rv())
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn decode_rmctrl(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    let (a,b) = decode_ctrlrm(sm,cg);
    (b,a)
}

pub fn decode_mi(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    if let (&Some(ref rm),&Some(ref imm)) = (&sm.configuration.rm,&sm.configuration.imm) {
        (rm.to_rv(),imm.clone())
    } else {
        unreachable!()
    }
}

pub fn decode_m1(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_mc(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_ii(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue) {
    if let &Some(Rvalue::Constant(c)) = &sm.configuration.imm {
        (Rvalue::Constant(c >> 8),Rvalue::Constant(c & 0xff))
    } else {
        unreachable!()
    }
}

pub fn decode_rvm(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmv(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmi(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm),&Some(ref imm)) = (&sm.configuration.reg,&sm.configuration.rm,&sm.configuration.imm) {
        (reg.to_rv(),rm.to_rv(),imm.clone())
    } else {
        unreachable!()
    }
}

pub fn decode_mri(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue) {
    let (a,b,c) = decode_rmi(sm,cg);
    (b,a,c)
}

pub fn decode_rvmi(sm: &mut State<Amd64>,cg: &mut CodeGen) -> (Rvalue,Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_reg8(r_reg: usize,rex: bool) -> Lvalue {
    match r_reg {
        0 => al.clone(),
        1 => cl.clone(),
        2 => dl.clone(),
        3 => bl.clone(),
        4 => if rex { spl.clone() } else { ah.clone() },
        5 => if rex { bpl.clone() } else { ch.clone() },
        6 => if rex { sil.clone() } else { dh.clone() },
        7 => if rex { dil.clone() } else { bh.clone() },
        8 => r8l.clone(),
        9 => r9l.clone(),
        10 => r10l.clone(),
        11 => r11l.clone(),
        12 => r12l.clone(),
        13 => r13l.clone(),
        14 => r14l.clone(),
        15 => r15l.clone(),
        _ => unreachable!()
    }
}

pub fn decode_reg16(r_reg: usize) -> Lvalue {
    match r_reg {
        0 => ax.clone(),
        1 => cx.clone(),
        2 => dx.clone(),
        3 => bx.clone(),
        4 => sp.clone(),
        5 => bp.clone(),
        6 => si.clone(),
        7 => di.clone(),
        8 => r8w.clone(),
        9 => r9w.clone(),
        10 => r10w.clone(),
        11 => r11w.clone(),
        12 => r12w.clone(),
        13 => r13w.clone(),
        14 => r14w.clone(),
        15 => r15w.clone(),
        _ => unreachable!()
    }
}

pub fn decode_reg32(r_reg: usize) -> Lvalue {
    match r_reg {
        0 => eax.clone(),
        1 => ecx.clone(),
        2 => edx.clone(),
        3 => ebx.clone(),
        4 => esp.clone(),
        5 => ebp.clone(),
        6 => esi.clone(),
        7 => edi.clone(),
        8 => r8d.clone(),
        9 => r9d.clone(),
        10 => r10d.clone(),
        11 => r11d.clone(),
        12 => r12d.clone(),
        13 => r13d.clone(),
        14 => r14d.clone(),
        15 => r15d.clone(),
        _ => unreachable!()
    }
}

pub fn decode_reg64(r_reg: usize) -> Lvalue {
    match r_reg {
        0 => rax.clone(),
        1 => rcx.clone(),
        2 => rdx.clone(),
        3 => rbx.clone(),
        4 => rsp.clone(),
        5 => rbp.clone(),
        6 => rsi.clone(),
        7 => rdi.clone(),
        8 => r8.clone(),
        9 => r9.clone(),
        10 => r10.clone(),
        11 => r11.clone(),
        12 => r12.clone(),
        13 => r13.clone(),
        14 => r14.clone(),
        15 => r15.clone(),
        _ => unreachable!()
    }
}

fn select_reg(os: &OperandSize,r: usize, rex: bool) -> Lvalue {
    match os {
        &OperandSize::Eight => decode_reg8(r,rex),
        &OperandSize::Sixteen => decode_reg16(r),
        &OperandSize::ThirtyTwo => decode_reg32(r),
        &OperandSize::SixtyFour => decode_reg64(r),
        &OperandSize::HundredTwentyEight => panic!("No 128 bit registers in x86!")
    }
}

pub fn select_mem(os: &OperandSize,o: Rvalue) -> Lvalue {
    match os {
        &OperandSize::Eight => byte(o),
        &OperandSize::Sixteen => word(o),
        &OperandSize::ThirtyTwo => dword(o),
        &OperandSize::SixtyFour => qword(o),
        &OperandSize::HundredTwentyEight => oword(o),
    }
}

fn decode_modrm(
        _mod: usize,
        b_rm: usize,    // B.R/M
        disp: Option<Rvalue>,
        sib: Option<(usize,usize,usize)>, // scale, X.index, B.base
        os: OperandSize,
        addrsz: AddressSize,
        mode: Mode,
        rex: bool,
        c: &mut CodeGen) -> Lvalue
{
    assert!(_mod < 0x4);
    assert!(b_rm < 0x10);

    match addrsz {
        AddressSize::Sixteen => {
            match _mod {
                0 | 1 | 2 => {
                    let tmp = new_temp(16);

                    if b_rm == 6 {
                        if _mod == 0 {
                            select_mem(&os,disp.unwrap())
                        } else {
                            c.add_i(&tmp,&select_mem(&os,bp.clone().to_rv()),&disp.unwrap());
                            tmp
                        }
                    } else {
                        let base = select_mem(&os,match b_rm {
                            0 => { c.add_i(&tmp,&*bx,&*si); tmp.clone() },
                            1 => { c.add_i(&tmp,&*bx,&*di); tmp.clone() },
                            2 => { c.add_i(&tmp,&*bp,&*si); tmp.clone() },
                            3 => { c.add_i(&tmp,&*bp,&*di); tmp.clone() },
                            4 => si.clone(),
                            5 => di.clone(),
                            7 => bx.clone(),
                            _ => unreachable!(),
                        }.to_rv());

                        if _mod == 0 {
                            base
                        } else {
                            c.add_i(&tmp,&base,&disp.unwrap());
                            tmp
                        }
                    }
                },
                3 => select_reg(&os,b_rm,rex),
                _ => unreachable!()
            }
        },
        AddressSize::ThirtyTwo | AddressSize::SixtyFour => {
            let base = match b_rm {
                0 | 1 | 2 | 3 |
                6 | 7 | 8 | 9 | 10 | 11 |
                14 | 15 => select_reg(&if _mod != 3 && addrsz == AddressSize::SixtyFour { OperandSize::SixtyFour } else { os.clone() },b_rm,rex),

                4 | 12 => if _mod == 3 {
                        select_reg(&os,b_rm,rex)
                    } else {
                        if let Some((scale,index,base)) = sib {
                            decode_sib(_mod,scale,index,base,disp.clone(),os.clone(),c)
                        } else {
                            unreachable!()
                        }
                    },

                5 | 13 => if _mod == 0 {
                    if mode == Mode::Long {
                        if addrsz == AddressSize::SixtyFour {
                            let tmp = new_temp(64);

                            c.add_i(&tmp,disp.as_ref().unwrap(),&*rip);
                            c.mod_i(&tmp,&tmp,&Rvalue::Constant(0xffffffffffffffff));
                            select_mem(&os,tmp.to_rv())
                        } else {
                            let tmp = new_temp(32);

                            c.add_i(&tmp,disp.as_ref().unwrap(),&*eip);
                            c.mod_i(&tmp,&tmp,&Rvalue::Constant(0xffffffff));
                            select_mem(&os,tmp.to_rv())
                        }
                    } else {
                        select_mem(&os,disp.clone().unwrap())
                    }
                } else {
                    select_reg(&if _mod != 3 && addrsz == AddressSize::SixtyFour { OperandSize::SixtyFour } else { os.clone() },b_rm,rex)
                },
                _ => unreachable!()
            };

            match _mod {
                0 => select_mem(&os,base.to_rv()),
                1 | 2 => {
                    let tmp = new_temp(os.num_bits());

                    c.add_i(&tmp,&base,disp.as_ref().unwrap());
                    select_mem(&os,tmp.to_rv())
                },
                3 => base,
                _ => unreachable!()
            }
        }
    }
}

fn decode_sib(
    _mod: usize,
    scale: usize,
    x_index: usize,
    b_base: usize,
    disp: Option<Rvalue>,
    os: OperandSize,
    c: &mut CodeGen) -> Lvalue
{
    assert!(_mod <= 3 && scale <= 3 && x_index <= 15 && b_base <= 15);

    match _mod {
        0 => match b_base {
            0 | 1 | 2 | 3 | 4 |
            6 | 7 | 8 | 9 | 10 | 11 | 12 |
            14 => match x_index {
                0 | 1 | 2 | 3 | 5...15 => {
                    let base = decode_reg64(b_base);
                    let index = decode_reg64(x_index);
                    let tmp = new_temp(64);

                    if scale > 0 {
                        c.mul_i(&tmp,&index,&Rvalue::Constant((1 << (scale & 3)) / 2));
                        c.add_i(&tmp,&base,&tmp);

                        select_mem(&os,tmp.to_rv())
                    } else {
                        c.add_i(&tmp,&base,&index);

                        select_mem(&os,tmp.to_rv())
                    }
                },
                4 => select_mem(&os,Rvalue::Constant((b_base & 7) as u64)),
                _ => unreachable!()
            },
            5 | 15 => match x_index {
                0...3 | 5...15 => {
                    let index = decode_reg64(x_index);
                    let tmp = new_temp(64);

                    if scale > 0 {
                        c.mul_i(&tmp,&index,&Rvalue::Constant((1 << (scale & 3)) / 2));
                        c.add_i(&tmp,&disp.unwrap(),&tmp);

                        select_mem(&os,tmp.to_rv())
                    } else {
                        c.add_i(&tmp,&disp.unwrap(),&index);

                        select_mem(&os,tmp.to_rv())
                    }
                },
                4 => select_mem(&os,disp.unwrap()),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        1 | 2 => match x_index {
            0...3 | 5...15 => {
                let base = decode_reg64(b_base);
                let index = decode_reg64(x_index);
                let tmp = new_temp(64);

                if scale > 0 {
                    c.mul_i(&tmp,&index,&Rvalue::Constant((1 << (scale & 3)) / 2));
                    c.add_i(&tmp,&tmp,&disp.unwrap());
                    c.add_i(&tmp,&base,&tmp);

                    select_mem(&os,tmp.to_rv())
                } else {
                    c.add_i(&tmp,&index,&disp.unwrap());
                    c.add_i(&tmp,&base,&tmp);

                    select_mem(&os,tmp.to_rv())
                }
            },
            4 => {
                let tmp = new_temp(64);

                c.add_i(&tmp,&decode_reg64(b_base),&disp.unwrap());
                select_mem(&os,tmp.to_rv())
            },
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

pub fn nonary(opcode: &str, sem: &Fn(&mut CodeGen)) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn unary(opcode: &str,
              decode: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
              sem: &Fn(&mut CodeGen, Rvalue)
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn unary_c(opcode: &str,
                  a: Rvalue,
                  sem: &Fn(&mut CodeGen, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary(opcode: &str,
              decode: fn(&mut State<Amd64>, &mut CodeGen) -> (Rvalue,Rvalue),
              sem: &Fn(&mut CodeGen, Rvalue, Rvalue)
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary_rv(opcode: &str,
                  a: &Lvalue,
                  decode: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
                  sem: &Fn(&mut CodeGen, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary_vr(opcode: &str,
                  decode: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
                  b: &Lvalue,
                  sem: &Fn(&mut CodeGen, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary_vc(opcode: &str,
                  decode: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
                  b: Rvalue,
                  sem: &Fn(&mut CodeGen, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary_rr(opcode: &str,
                  a: &Lvalue,
                  b: &Lvalue,
                  sem: &Fn(&mut CodeGen, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn binary_vv(opcode: &str,
                  decodea: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
                  decodeb: fn(&mut State<Amd64>, &mut CodeGen) -> Rvalue,
                  sem: &Fn(&mut CodeGen, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn trinary(opcode: &str,
              decode: fn(&mut State<Amd64>, &mut CodeGen) -> (Rvalue,Rvalue,Rvalue),
              sem: &Fn(&mut CodeGen, Rvalue, Rvalue, Rvalue)
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

pub fn trinary_vr(opcode: &str,
                  decode: fn(&mut State<Amd64>, &mut CodeGen) -> (Rvalue,Rvalue),
                  c: &Lvalue,
                  sem: &Fn(&mut CodeGen, Rvalue, Rvalue, Rvalue)) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        false
    })
}

macro_rules! reg {
    ( $a:ident,$I:expr ) => {
        pub fn $a(st: &mut State<Amd64>, cg: &mut CodeGen) -> Rvalue {
            let r = if st.has_group("b") && st.get_group("b") == 1 { 8 } else { 0 } + $I;
            select_reg(&st.configuration.operand_size,r,st.configuration.rex).to_rv()
        }
    }
}

macro_rules! regd {
    ( $a:ident,$I:expr ) => {
        pub fn $a(st: &mut State<Amd64>, cg: &mut CodeGen) -> Rvalue {
            let r = if st.has_group("b") && st.get_group("b") == 1 { 8 } else { 0 } + $I;
            let opsz = if st.configuration.mode == Mode::Long && st.configuration.operand_size == OperandSize::ThirtyTwo {
                OperandSize::SixtyFour
            } else {
                st.configuration.operand_size
            };
            select_reg(&opsz,r,st.configuration.rex).to_rv()
        }
    }
}

macro_rules! regb {
    ( $a:ident,$I:expr ) => {
        pub fn $a(st: &mut State<Amd64>, cg: &mut CodeGen) -> Rvalue {
            let r = if st.has_group("b") && st.get_group("b") == 1 { 8 } else { 0 } + $I;
            select_reg(&OperandSize::Eight,r,st.configuration.rex).to_rv()
        }
    }
}

reg!(reg_a,0);
reg!(reg_c,1);
reg!(reg_d,2);
reg!(reg_b,3);
reg!(reg_sp,4);
reg!(reg_bp,5);
reg!(reg_si,6);
reg!(reg_di,7);

regd!(regd_a,0);
regd!(regd_c,1);
regd!(regd_d,2);
regd!(regd_b,3);
regd!(regd_sp,4);
regd!(regd_bp,5);
regd!(regd_si,6);
regd!(regd_di,7);

regb!(regb_a,0);
regb!(regb_c,1);
regb!(regb_d,2);
regb!(regb_b,3);
regb!(regb_sp,4);
regb!(regb_bp,5);
regb!(regb_si,6);
regb!(regb_di,7);

/*
sem_action po::amd64::nonary(std::string const& op, std::function<void(cg&)> func)
{
    return [op,func](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"",[func,st,op](cg& c)
        {
            func(c);

            std::cout << op << std::endl;
            return std::list<rvalue>({});
        });
        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::unary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode, std::function<void(cg&,rvalue)> func)
{
    return [op,func,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64}",[decode,func,st,op](cg& c)
        {
            rvalue a = decode(st,c);
            func(c,a);

            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << a << std::endl;
            return std::list<rvalue>({a});
        });
        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::unary(std::string const& op, rvalue arg, std::function<void(cg&,rvalue)> func)
{
    return [op,func,arg](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64}",[arg,func,st,op](cg& c)
        {
            func(c,arg);

            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg << std::endl;
            return std::list<rvalue>({arg});
        });
        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, std::function<std::pair<rvalue,rvalue>(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[decode,func,st,op](cg& c)
        {
            rvalue a,b;

            std::tie(a,b) = decode(st,c);
            func(c,a,b);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << a << ", " << b << std::endl;
            return std::list<rvalue>({a,b});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode, rvalue arg2, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,decode,arg2](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[arg2,decode,func,st,op](cg& c)
        {
            rvalue arg1 = decode(st,c);
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, rvalue arg1, std::function<rvalue(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,arg1,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[arg1,decode,func,st,op](cg& c)
        {
            rvalue arg2 = decode(st,c);
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, rvalue arg1, rvalue arg2, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,arg1,arg2](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[arg1,arg2,func,st,op](cg& c)
        {
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode1, std::function<rvalue(sm const&,cg&)> decode2, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,decode1,decode2](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[decode1,decode2,func,st,op](cg& c)
        {
            rvalue arg1 = decode1(st,c);
            rvalue arg2 = decode2(st,c);
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::branch(std::string const& m, rvalue flag, bool set)
{
    return [m,flag,set](sm &st)
    {
        /*int64_t _k = st.capture_groups["k"] * 2;
        guard g(flag,relation::Eq,set ? Rvalue::Constant(1) : Rvalue::Constant(0));
        Rvalue::Constant k((int8_t)(_k <= 63 ? _k : _k - 128));*/

        st.mnemonic(st.tokens.size() * 2,m,"");
        st.jump(st.address + st.tokens.size());//,g.negation());
        //st.jump(undefined(),g);//st.address + k.content() + 2,g);
        return true;
    };
}

sem_action po::amd64::trinary(std::string const& op, std::function<std::tuple<rvalue,rvalue,rvalue>(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue,rvalue)> func)
{
    return [op,func,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64} {64}",[decode,func,st,op](cg& d)
        {
            rvalue a,b,c;

            std::tie(a,b,c) = decode(st,d);
            func(d,a,b,c);

            std::cout << op << " " << a << ", " << b << ", " << c << std::endl;
            return std::list<rvalue>({a,b,c});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::trinary(std::string const& op, std::function<std::pair<rvalue,rvalue>(sm const&,cg&)> decode, rvalue arg3, std::function<void(cg&,rvalue,rvalue,rvalue)> func)
{
    return [op,func,decode,arg3](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64} {64}",[decode,arg3,func,st,op](cg& d)
        {
            rvalue a,b;

            std::tie(a,b) = decode(st,d);
            func(d,a,b,arg3);

            std::cout << op << " " << a << ", " << b << ", " << arg3 << std::endl;
            return std::list<rvalue>({a,b,arg3});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}
*/
