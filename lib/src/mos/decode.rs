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

use value::{Lvalue,Rvalue};
use disassembler::State;
use mos::*;
use codegen::CodeGen;
use guard::Guard;
use std::num::Wrapping;


pub fn nonary(opcode: &'static str, sem: fn(&mut CodeGen<Mos>)) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;

        st.mnemonic_dynargs(len, &opcode, "", &|c| {
            sem(c);
            vec![]
        });
        st.jump(Rvalue::Constant(next), Guard::always());
        true
    })
}

pub fn nonary_ret(opcode: &'static str, sem: fn(&mut CodeGen<Mos>)) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let len = st.tokens.len();

        st.mnemonic_dynargs(len, &opcode, "", &|c| {
            sem(c);
            vec![]
        });
        true
    })
}


pub fn unary_r(opcode: &'static str,
               _arg0: &Lvalue,
               sem: fn(&mut CodeGen<Mos>, Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    let arg0 = _arg0.clone();
    Box::new(move |st: &mut State<Mos>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        st.mnemonic_dynargs(len, &opcode, "{8}", &|c| {
            sem(c,arg0.to_rv());
            vec![arg0.to_rv()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn unary_i(opcode: &'static str,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"#{8}",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_z(opcode: &'static str,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"Z{8}",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_zr(opcode: &'static str,
               _arg1: &Lvalue,
                sem: fn(&mut CodeGen<Mos>,Rvalue,Rvalue)
               ) -> Box<Fn(&mut State<Mos>) -> bool> {
    let arg1 = _arg1.clone();
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg0) = _arg {
            st.mnemonic_dynargs(len,&opcode,"Z{8},{8}",&|c| {
                sem(c,arg0.clone(),arg1.to_rv());
                vec![arg0.clone(),arg1.to_rv()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}


pub fn unary_izx(opcode: &'static str,
                  sem: fn(&mut CodeGen<Mos>,Rvalue)
                 ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"(Z{8},X)",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_izy(opcode: &'static str,
                  sem: fn(&mut CodeGen<Mos>,Rvalue)
                 ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"(Z{8}),Y",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_a(opcode: &'static str,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"D{16}",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_ar(opcode: &'static str,
               _arg1: &Lvalue,
               sem: fn(&mut CodeGen<Mos>,Rvalue,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    let arg1 = _arg1.clone();
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg0) = _arg {
            st.mnemonic_dynargs(len,&opcode,"D{16},{8}",&|c| {
                sem(c,arg0.clone(),arg1.to_rv());
                vec![arg0.clone(),arg1.to_rv()]
            });
            st.jump(Rvalue::Constant(next),Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_call_a(opcode: &'static str,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"L{16}",&|c| {
	        c.call_i(&Lvalue::Undefined, &arg);
	        sem(c,arg.clone());
                vec![arg.clone()]
            });
	    st.jump(Rvalue::Constant(next), Guard::always());
            // st.jump(arg, Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_goto_a(opcode: &'static str,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"L{16}",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
            st.jump(arg, Guard::always());
            true
        } else {
            false
        }
    })
}

pub fn unary_goto_ind(opcode: &'static str,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        if let Some(arg) = _arg {
            st.mnemonic_dynargs(len,&opcode,"(D{16})",&|c| {
                sem(c,arg.clone());
                vec![arg.clone()]
            });
	    // FIXME: Maybe we can read the address out of memory and jump here.
	    // st.jump(arg, Guard::always());
            true
        } else {
            false
        }
    })
}

/* Relative branch.  */
pub fn unary_b(opcode: &'static str,
               _flag: &Lvalue,
	       _set: bool,
               sem: fn(&mut CodeGen<Mos>,Rvalue)
              ) -> Box<Fn(&mut State<Mos>) -> bool> {
    let flag = _flag.clone();
    let set = if _set { 1 } else { 0 };

    Box::new(move |st: &mut State<Mos>| -> bool {
        let _arg = st.configuration.arg0.clone();
        let len = st.tokens.len();
        let next = st.address + len as u64;

        let g = Guard::eq(&flag, &set);
	let fallthru = st.configuration.wrap(next);

        if let Some(arg) = _arg {
	    if let Rvalue::Constant(c) = arg {
                let k = (Wrapping(st.address + len as u64) + Wrapping((c as i8) as u64)).0;
                let kk = Rvalue::Constant(k as u64);
                st.mnemonic_dynargs(len,&opcode,"L{16}", &|c| {
                    sem(c, kk.clone());
                    vec![kk.clone()]
                });
	        st.jump(fallthru, g.negation());
                st.jump(kk, g);
                true
	    }
	    else {
	        false
            }
        } else {
            false
        }
    })
}
