/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016 Kai Michaelis
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

use std::rc::Rc;
use std::borrow::Cow;

use disassembler::*;
use super::*;
use super::semantic::*;

pub fn disassembler() -> Rc<Disassembler<Avr>> {
    let skip = new_disassembler!(Avr =>
        [ "1111 110 sr@..... 0 sb@..." ] = skip("sbrc",false),
        [ "1111 111 sr@..... 0 sb@..." ] = skip("sbrs",true),
        [ "000100 cr@. cd@..... cr@...." ] = cpse,
        [ "1001 1001 sA@..... sb@..." ] = skip("sbic",false),
        [ "1001 1011 sA@..... sb@..." ] = skip("sbis",true)
    );

    let main = new_disassembler!(Avr =>
        [ "000111 r@. d@..... r@...." ] = binary("adc",adc),
        [ "0000 11 r@. d@..... r@...." ] = binary("add",add),
        [ "10010110 K@.. d@.. K@...." ] = adiw,
        [ "0010 00 r@. d@..... r@...." ] = binary("and",and),
        [ "0111 K@.... d@.... K@...." ] = binary("andi",and),
        [ "11110 0 k@....... 000" ] = branch("brlo",&rreil_lvalue!{ C:1 },true),
        [ "11110 1 k@....... 000" ] = branch("brsh",&rreil_lvalue!{ C:1 },false),
        [ "11110 0 k@....... 001" ] = branch("breq",&rreil_lvalue!{ Z:1 },true),
        [ "11110 1 k@....... 001" ] = branch("brne",&rreil_lvalue!{ Z:1 },false),
        [ "11110 0 k@....... 010" ] = branch("brns",&rreil_lvalue!{ N:1 },true),
        [ "11110 1 k@....... 010" ] = branch("brnc",&rreil_lvalue!{ N:1 },false),
        [ "11110 0 k@....... 011" ] = branch("brvs",&rreil_lvalue!{ V:1 },true),
        [ "11110 1 k@....... 011" ] = branch("brvc",&rreil_lvalue!{ V:1 },false),
        [ "11110 0 k@....... 100" ] = branch("brge",&rreil_lvalue!{ S:1 },true),
        [ "11110 1 k@....... 100" ] = branch("brlt",&rreil_lvalue!{ S:1 },false),
        [ "11110 0 k@....... 101" ] = branch("brhs",&rreil_lvalue!{ H:1 },true),
        [ "11110 1 k@....... 101" ] = branch("brhc",&rreil_lvalue!{ H:1 },false),
        [ "11110 0 k@....... 110" ] = branch("brts",&rreil_lvalue!{ T:1 },true),
        [ "11110 1 k@....... 110" ] = branch("brtc",&rreil_lvalue!{ T:1 },false),
        [ "11110 0 k@....... 111" ] = branch("bris",&rreil_lvalue!{ I:1 },true),
        [ "11110 1 k@....... 111" ] = branch("bric",&rreil_lvalue!{ I:1 },false),
        [ "1111 100 d@..... 0 b@..." ] = binary_imm("bld",bld),
        [ 0x9598 ] = nonary("break",_break),
        [ "1111 101 d@..... 0 b@..." ] = binary_imm("bst",bst),
        [ "1001010 k@..... 111 k@.", "k@................" ] = call,
        [ "10101100 A@..... b@..." ] = binary_imm("cbi",cbx),
        [ 0x9488 ] = flag("clc",&rreil_lvalue!{ C:1 },false),
        [ 0x94d8 ] = flag("clh",&rreil_lvalue!{ H:1 },false),
        [ 0x94f8 ] = flag("cli",&rreil_lvalue!{ I:1 },false),
        [ 0x94a8 ] = flag("cln",&rreil_lvalue!{ N:1 },false),
        [ 0x94c8 ] = flag("cls",&rreil_lvalue!{ S:1 },false),
        [ 0x94e8 ] = flag("clt",&rreil_lvalue!{ T:1 },false),
        [ 0x94b8 ] = flag("clv",&rreil_lvalue!{ V:1 },false),
        [ 0x9498 ] = flag("clz",&rreil_lvalue!{ Z:1 },false),
        [ "1001010 d@..... 0000" ] = unary("com",com),
        [ "000101 r@. d@..... r@...." ] = binary("cp",cp),
        [ "000001 r@. d@..... r@...." ] = binary("cpc",cpc),
        [ "0011 K@.... d@.... K@...." ] = binary("cpi",cp),
        [ "1001010 d@..... 1010" ] = unary("dec",dec),
        [ "10010100 K@.... 1011" ] = des,
        [ "1001 0101 0001 1001" ] = nonary("eicall",eicall),
        [ "1001 0100 0001 1001" ] = eijmp,
        [ "1001 0101 1101 1000" ] = elpm1,
        [ "1001 000 d@..... 0110" ] = elpm2,
        [ "1001 000 d@..... 0111" ] = elpm3,
        [ "0010 01 r@. d@..... r@...." ] = binary("eor",eor),
        [ "0000 0011 0 d@... 1 r@..." ] = binary("fmul",fmul),
        [ "0000 0011 1 d@... 0 r@..." ] = binary("fmuls",fmuls),
        [ "0000 0011 1 d@... 1 r@..." ] = binary("fmulsu",fmulsu),
        [ 0x9509 ] = icall,
        [ 0x9409 ] = ijmp,
        [ "10110 A@.. d@..... A@...." ] = binary("in",_in),
        [ "1001010 d@..... 0011" ] = unary("inc",inc),
        [ "1001010 k@..... 110 k@.", "k@................" ] = jmp,
        [ "1001001 r@..... 0110" ] = binary_ptr("lac",lac,AddressRegister::Z,AddressOffset::None,true),
        [ "1001001 r@..... 0101" ] = binary_ptr("las",las,AddressRegister::Z,AddressOffset::None,true),
        [ "1001001 r@..... 0111" ] = binary_ptr("lat",lat,AddressRegister::Z,AddressOffset::None,true),
        [ "1001 000 d@..... 1100" ] = binary_ptr("ld",ld,AddressRegister::X,AddressOffset::None,false),
        [ "1001 000 d@..... 1110" ] = binary_ptr("ld",ld,AddressRegister::X,AddressOffset::Predecrement,false),
        [ "1001 000 d@..... 1101" ] = binary_ptr("ld",ld,AddressRegister::X,AddressOffset::Postincrement,false),
        [ "1000 000 d@..... 1000" ] = binary_ptr("ld",ld,AddressRegister::Y,AddressOffset::None,false),
        [ "1001 000 d@..... 1010" ] = binary_ptr("ld",ld,AddressRegister::Y,AddressOffset::Predecrement,false),
        [ "1001 000 d@..... 1001" ] = binary_ptr("ld",ld,AddressRegister::Y,AddressOffset::Postincrement,false),
        [ "1000 000 d@..... 0000" ] = binary_ptr("ld",ld,AddressRegister::Z,AddressOffset::None,false),
        [ "1001 000 d@..... 0010" ] = binary_ptr("ld",ld,AddressRegister::Z,AddressOffset::Predecrement,false),
        [ "1001 000 d@..... 0001" ] = binary_ptr("ld",ld,AddressRegister::Z,AddressOffset::Postincrement,false),
        [ "10 q@. 0 q@.. 0 d@..... 1 q@..." ] = binary_ptr("ldd",ld,AddressRegister::Y,AddressOffset::Displacement,false),
        [ "10 q@. 0 q@.. 0 d@..... 0 q@..." ] = binary_ptr("ldd",ld,AddressRegister::Z,AddressOffset::Displacement,false),
        [ "1110 K@.... d@.... K@...." ] = binary_imm("ldi",ldi),
        [ "1001000 d@..... 0000", "k@................" ] = binary_imm("lds",lds1),
        [ "10100 k@... d@.... k@...." ] =  binary_imm("lds",lds2),
        [ 0x95c8 ] = lpm1,
        [ "1001 000 d@..... 0100" ] = lpm2,
        [ "1001 000 d@..... 0101" ] = lpm3,
        [ "1001010 d@..... 0110" ] = unary("lsr",lsr),
        [ "001011 r@. d@..... r@...." ] = binary("mov",mov),
        [ "00000001 d@.... r@...." ] = binary("movw",movw),
        [ "1001 11 r@. d@..... r@...." ] = binary("mul",mul),
        [ "0000 0010 d@.... r@...." ] = binary("muls",muls),
        [ "0000 0011 0 d@... 0 r@..." ] = binary("mulsu",mulsu),
        [ "1001 010 d@..... 0001" ] = unary("neg",neg),
        [ 0 ] = nonary("nop",nop),
        [ "0010 10 r@. d@..... r@...." ] = binary("or",or),
        [ "0110 K@.... d@.... K@...." ] = binary("ori",or),
        [ "10111 A@.. r@..... A@...." ] = binary("out",out),
        [ "1001000 d@..... 1111" ] = unary("pop",pop),
        [ "1001001 d@..... 1111" ] = unary("push",push),
        [ "1101 k@............" ] = rcall,
        [ 0x9508 ] = nonary("ret",ret),
        [ 0x9518 ] = nonary("reti",ret),
        [ "1100 k@............" ] = rjmp,
        [ "1001010 d@..... 0111" ] = unary("ror",ror),
        [ "000010 r@. d@..... r@...." ] = binary("sbc",sbc),
        [ "0100 K@.... d@.... K@...." ] = binary("sbci",sbc),
        [ "1001 1010 A@..... b@..." ] = binary_imm("sbi",sbi),
        [ "10010111 K@.. d@.. K@...." ] = sbiw,
        [ 0x9408 ] = flag("sec",&rreil_lvalue!{ C:1 },true),
        [ 0x9458 ] = flag("seh",&rreil_lvalue!{ H:1 },true),
        [ 0x9478 ] = flag("sei",&rreil_lvalue!{ I:1 },true),
        [ 0x9428 ] = flag("sen",&rreil_lvalue!{ N:1 },true),
        [ 0x9448 ] = flag("ses",&rreil_lvalue!{ S:1 },true),
        [ 0x9468 ] = flag("set",&rreil_lvalue!{ T:1 },true),
        [ 0x9438 ] = flag("set",&rreil_lvalue!{ V:1 },true),
        [ 0x9418 ] = flag("sez",&rreil_lvalue!{ Z:1 },true),
        [ 0x9588 ] = nonary("sleep",sleep),
        [ 0x95e8 ] = spm1,
        [ 0x95f8 ] = spm2,
        [ "1001 001 r@. r@.... 1100" ] = binary_ptr("st",st,AddressRegister::X,AddressOffset::None,true),
        [ "1001 001 r@. r@.... 1110" ] = binary_ptr("st",st,AddressRegister::X,AddressOffset::Predecrement,true),
        [ "1001 001 r@. r@.... 1101" ] = binary_ptr("st",st,AddressRegister::X,AddressOffset::Postincrement,true),
        [ "1001 001 r@. r@.... 1000" ] = binary_ptr("st",st,AddressRegister::Y,AddressOffset::None,true),
        [ "1001 001 r@. r@.... 1010" ] = binary_ptr("st",st,AddressRegister::Y,AddressOffset::Predecrement,true),
        [ "1001 001 r@. r@.... 1001" ] = binary_ptr("st",st,AddressRegister::Y,AddressOffset::Postincrement,true),
        [ "1001 001 r@. r@.... 0000" ] = binary_ptr("st",st,AddressRegister::Z,AddressOffset::None,true),
        [ "1001 001 r@. r@.... 0010" ] = binary_ptr("st",st,AddressRegister::Z,AddressOffset::Predecrement,true),
        [ "1001 001 r@. r@.... 0001" ] = binary_ptr("st",st,AddressRegister::Z,AddressOffset::Postincrement,true),
        [ "10 q@. 0 q@.. 1 r@..... 0 q@..." ] = binary_ptr("std",st,AddressRegister::Y,AddressOffset::Displacement,true),
        [ "10 q@. 0 q@.. 1 r@..... 1 q@..." ] = binary_ptr("std",st,AddressRegister::Z,AddressOffset::Displacement,true),
        [ "1001001 d@..... 0000", "k@................" ] = binary_imm("sts",sts1),
        [ "10101 k@... d@.... k@...." ] = binary_imm("sts",sts2),
        [ "000110 r@. d@..... r@...." ] = binary("sub",sub),
        [ "0101 K@.... d@.... K@...." ] = binary("subi",sub),
        [ "1001010 d@..... 0010" ] = unary("swap",swap),
        [ 0x95a8 ] = nonary("wdr",wdr),
        [ "1001001 r@..... 0100" ] = binary_ptr("xch",xch,AddressRegister::Z,AddressOffset::None,true)
    );

    new_disassembler!(Avr =>
        [ opt!(skip), main ] = |_: &mut State<Avr>| { true },
        [ skip ] = |_: &mut State<Avr>| { true }
    )
}
