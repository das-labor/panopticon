/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
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

#[macro_use]
extern crate panopticon;

extern crate env_logger;
extern crate regex;

#[macro_use]
extern crate quickcheck;

use panopticon::{
    Region,
    Architecture,
    amd64,
    Result,
    Rvalue,
    Lvalue,
    execute,
    lift,
    Statement,
};
use panopticon::amd64::semantic;

use quickcheck::{Arbitrary,Gen};
use std::path::Path;

#[test]
fn amd64_opcodes() {
    let reg = Region::open("com".to_string(),Path::new("tests/data/amd64.com")).unwrap();
    let mut addr = 0;

    loop {
        let maybe_match = <amd64::Amd64 as Architecture>::decode(&reg,addr,&amd64::Mode::Long);

        if let Ok(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}",mne.area.start,mne.opcode);
                addr = mne.area.end;

                if addr >= reg.size() {
                    return;
                }
            }
        } else if addr < reg.size() {
            unreachable!("failed to match anything at {:x}",addr);
        } else {
            break;
        }
    }
}

#[test]
fn ia32_opcodes() {
    env_logger::init().unwrap();

    let reg = Region::open("com".to_string(),Path::new("tests/data/ia32.com")).unwrap();
    let mut addr = 0;

    loop {
        let maybe_match = amd64::Amd64::decode(&reg,addr,&amd64::Mode::Protected);

        if let Ok(match_st) = maybe_match {
            for mne in match_st.mnemonics {
                println!("{:x}: {}",mne.area.start,mne.opcode);
                addr = mne.area.end;

                if addr >= reg.size() {
                    return;
                }
            }
        } else if addr < reg.size() {
            unreachable!("failed to match anything at {:x}",addr);
        } else {
            break;
        }
    }
}

#[derive(Clone,Debug)]
enum Opcode {
    Adc,
    Add,
    Sub,
    Cmp,
    Or,
    Xor,
    And,
}

impl Into<&'static str> for Opcode {
    fn into(self) -> &'static str {
        match self {
            Opcode::Adc => "adc",
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Cmp => "cmp",
            Opcode::Or => "or",
            Opcode::Xor => "xor",
            Opcode::And => "and",
        }
    }
}

impl Opcode {
    pub fn all() -> Vec<Opcode> {
        vec![
            Opcode::Adc,
            Opcode::Add,
            Opcode::Sub,
            Opcode::Cmp,
            Opcode::Or,
            Opcode::Xor,
            Opcode::And,
        ]
    }

    fn rreil(&self, a: Rvalue, b: Rvalue) -> Result<Vec<Statement>> {
        match *self {
            Opcode::Adc => semantic::adc(a,b).map(|x| x.0),
            Opcode::Add => semantic::add(a,b).map(|x| x.0),
            Opcode::Sub => semantic::sub(a,b).map(|x| x.0),
            Opcode::Cmp => semantic::cmp(a,b).map(|x| x.0),
            Opcode::Or => semantic::or(a,b).map(|x| x.0),
            Opcode::Xor => semantic::xor(a,b).map(|x| x.0),
            Opcode::And => semantic::and(a,b).map(|x| x.0),
        }
    }
}

impl Arbitrary for Opcode {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        g.choose(&Opcode::all()).unwrap().clone()
    }
}

#[derive(Clone,Debug)]
struct Operand8(&'static str,u8);
#[derive(Clone,Debug)]
struct Operand16(&'static str,u16);
#[derive(Clone,Debug)]
struct Operand32(&'static str,u32);
#[derive(Clone,Debug)]
struct Operand64(&'static str,u64);

impl Arbitrary for Operand8 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Operand8(g.choose(&[
            //"AH","BH","CH","DH",
            "AL","BL","CL","DL","SIL","DIL","BPL",
            "R8B","R9B","R10B","R11B","R12B","R13B","R14B","R15B",]).unwrap(),g.gen())
    }
}
impl Arbitrary for Operand16 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Operand16(g.choose(&["AX","BX","CX","DX","SI","DI"]).unwrap(),g.gen())
    }
}

impl Arbitrary for Operand32 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Operand32(g.choose(&["EAX","EBX","ECX","EDX","ESI","EDI"]).unwrap(),g.gen())
    }
}

impl Arbitrary for Operand64 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Operand64(g.choose(&["RAX","RBX","RCX","RDX","RSI","RDI"]).unwrap(),g.gen())
    }
}

#[derive(Clone,Debug)]
struct Context {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    flags: u8,
}

impl Arbitrary for Context {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Context{
            rax: g.gen(),
            rbx: g.gen(),
            rcx: g.gen(),
            rdx: g.gen(),
            rsi: g.gen(),
            rdi: g.gen(),
            rbp: g.gen(),
            r8: g.gen(),
            r9: g.gen(),
            r10: g.gen(),
            r11: g.gen(),
            r12: g.gen(),
            r13: g.gen(),
            r14: g.gen(),
            r15: g.gen(),
            flags: g.gen(),
        }
    }
}

#[test]
fn xcheck_8bit_quickcheck() {
    fn rappel_xcheck(op: Opcode,a: Operand8, b: Operand8,start: Context) -> Result<bool> {
        use std::process::{Stdio,Command};
        use std::io::{Read,Write};
        use std::str::FromStr;
        use regex::Regex;
        use std::collections::HashMap;
        use std::borrow::Cow;

        let a_var = Rvalue::Variable{ name: a.0.into(), size: 8, subscript: None, offset: 0 };
        let b_var = Rvalue::Variable{ name: b.0.into(), size: 8, subscript: None, offset: 0 };
        let regs_re = Regex::new(r"(rax|rbx|rcx|rdx|rsi|rdi|r8|r9|r10|r11|r12|r13|r14|r15):(.......)?(0x................)").unwrap();
        let flags_re = Regex::new(r"(cf|zf|of|sf|pf|af):(.)").unwrap();
        let mut stmts = vec![];
        let mut child = Command::new("./rappel")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn().ok().unwrap();
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( AH:8 ),Rvalue::new_u8(start.flags)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::sahf().map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RAX:64 ),Rvalue::new_u64(start.rax)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RBX:64 ),Rvalue::new_u64(start.rbx)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RCX:64 ),Rvalue::new_u64(start.rcx)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RDX:64 ),Rvalue::new_u64(start.rdx)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RSI:64 ),Rvalue::new_u64(start.rsi)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RDI:64 ),Rvalue::new_u64(start.rdi)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( RBP:64 ),Rvalue::new_u64(start.rbp)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R8:64 ),Rvalue::new_u64(start.r8)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R9:64 ),Rvalue::new_u64(start.r9)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R10:64 ),Rvalue::new_u64(start.r10)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R11:64 ),Rvalue::new_u64(start.r11)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R12:64 ),Rvalue::new_u64(start.r12)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R13:64 ),Rvalue::new_u64(start.r13)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R14:64 ),Rvalue::new_u64(start.r14)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R15:64 ),Rvalue::new_u64(start.r15)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(rreil_rvalue!( R15:64 ),Rvalue::new_u64(start.r15)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(a_var.clone(),Rvalue::new_u8(a.1)).map(|x| x.0)));
        stmts.append(&mut try!(semantic::mov(b_var.clone(),Rvalue::new_u8(b.1)).map(|x| x.0)));
        stmts.append(&mut try!(op.rreil(a_var,b_var)));

        if let (&mut Some(ref mut stdin),&mut Some(ref mut stdout)) = (&mut child.stdin,&mut child.stdout) {
            let mne: &'static str = op.clone().into();

            let _ = try!(stdin.write(&format!("mov ah, 0x{:x}\n",start.flags).into_bytes()));
            let _ = try!(stdin.write(b"sahf\n"));
            let _ = try!(stdin.write(&format!("mov rax, 0x{:x}\n",start.rax).into_bytes()));
            let _ = try!(stdin.write(&format!("mov rbx, 0x{:x}\n",start.rbx).into_bytes()));
            let _ = try!(stdin.write(&format!("mov rcx, 0x{:x}\n",start.rcx).into_bytes()));
            let _ = try!(stdin.write(&format!("mov rdx, 0x{:x}\n",start.rdx).into_bytes()));
            let _ = try!(stdin.write(&format!("mov rsi, 0x{:x}\n",start.rsi).into_bytes()));
            let _ = try!(stdin.write(&format!("mov rdi, 0x{:x}\n",start.rdi).into_bytes()));
            let _ = try!(stdin.write(&format!("mov rbp, 0x{:x}\n",start.rbp).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r8, 0x{:x}\n",start.r8).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r9, 0x{:x}\n",start.r9).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r10, 0x{:x}\n",start.r10).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r11, 0x{:x}\n",start.r11).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r12, 0x{:x}\n",start.r12).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r13, 0x{:x}\n",start.r13).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r14, 0x{:x}\n",start.r14).into_bytes()));
            let _ = try!(stdin.write(&format!("mov r15, 0x{:x}\n",start.r15).into_bytes()));
            let _ = try!(stdin.write(&format!("mov {}, 0x{:x}\n",a.0,a.1).into_bytes()));
            let _ = try!(stdin.write(&format!("mov {}, 0x{:x}\n",b.0,b.1).into_bytes()));
            let _ = try!(stdin.write(&format!("{} {}, {}\n",mne,a.0,b.0).into_bytes()));
        }

        if !try!(child.wait()).success() {
            return Ok(false);
        }

        let mut out = String::new();
        try!(child.stdout.ok_or("No output")).read_to_string(&mut out);
        println!("{}",out);
        let regs = regs_re.captures_iter(&out).filter_map(|x| {
            if let (Some(ref nam),Some(ref s)) = (x.at(1),x.at(3)) {
                if let Ok(val) = u64::from_str_radix(&s[2..],16) {
                    Some((nam.to_string(),val))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect::<Vec<_>>();
        let flags = flags_re.captures_iter(&out).filter_map(|x| {
            if let (Some(ref nam),Some(ref s)) = (x.at(1),x.at(2)) {
                Some((nam.to_string(),*s != "0".to_string()))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        println!("regs: {:?}",regs);

        let mut ctx = HashMap::<Cow<'static,str>,u64>::new();

        for stmt in stmts {
            let s = lift(&stmt.op,&|rv| {
                if let &Rvalue::Variable{ ref name, ref offset, ref size,.. } = rv {
                    if let Some(val) = ctx.get(name.as_ref()) {
                        if *size < 64 {
                            Rvalue::Constant{ value: (*val >> *offset as usize) % (1 << *size), size: *size }
                        } else {
                            Rvalue::Constant{ value: (*val >> *offset), size: *size }
                        }
                    } else {
                        rv.clone()
                    }
                } else {
                    rv.clone()
                }
            });

            println!("{}",Statement{ assignee: stmt.assignee.clone(), op: s.clone() });

            if let Lvalue::Variable{ ref name,.. } = stmt.assignee {
                let res =  execute(s);
                println!("\t-> {}",res);

                match res {
                    Rvalue::Constant{ ref value,.. } => {
                        ctx.insert(name.clone(),*value);
                    }
                    Rvalue::Undefined => {
                        ctx.remove(name);
                    }
                    _ => {}
                }
            }
        }

        println!("{:?}",ctx);

        for (name,val) in regs {
            let key = Cow::Owned(name.clone().to_uppercase());

            if Some(val) != ctx.get(&key).map(|x| *x as u64) {
                println!("{:?} {:?}, {:?}:\n\tHardware = 0x{:x}\n\tSoftware = 0x{:x}",op,a,b,val,ctx.get(&key).unwrap_or(&0));
                return Ok(false);
            }
        }

        for (name,val) in flags {
            let key = Cow::Owned(name.clone().to_uppercase());
            let soft = ctx.get(&key).map(|x| *x as u64);

            if soft.is_some() && Some(if val { 1 } else { 0 }) != soft {
                println!("{}:\n\tHardware = {}\n\tSoftware = 0x{:x}",name,val,soft.unwrap());
                return Ok(false);
            }
        }

        Ok(true)
    }

    use quickcheck::QuickCheck;

    env_logger::init().unwrap();
    QuickCheck::new()
        .tests(1000)
        .quickcheck(rappel_xcheck as fn(Opcode,Operand8,Operand8,Context) -> Result<bool>);
}
