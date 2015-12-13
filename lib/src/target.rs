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

use amd64;
use amd64::{Config,Mode,Amd64};
use avr;
use avr::{Avr,Mcu};
use mos;
use mos::Mos;
use elf::parse::Machine;
use function::Function;
use layer::LayerIter;

#[derive(RustcEncodable,RustcDecodable,Clone,Copy,PartialEq)]
pub enum Target {
    __Test,
    Atmega103,
    Atmega88,
    Atmega8,
    Amd64,
    Ia32,
    Ia16,
    Mos6502,
}

impl Target {
    pub fn all() -> Vec<Target> {
        vec![
            Target::Atmega103,
            Target::Atmega88,
            Target::Atmega8,
            Target::Amd64,
            Target::Ia32,
            Target::Ia16,
            Target::Mos6502,
        ]
    }

    pub fn for_name(n: &str) -> Option<Target> {
        match n {
            "Test" => Some(Target::__Test),
            "ATmega103" => Some(Target::Atmega103),
            "ATmega88" => Some(Target::Atmega88),
            "ATmega8" => Some(Target::Atmega8),
            "AMD64" => Some(Target::Amd64),
            "IA-32" => Some(Target::Ia32),
            "80286" => Some(Target::Ia16),
            "MOS 6502" => Some(Target::Mos6502),
            _ => None,
        }
    }

    pub fn for_elf(m: Machine) -> Vec<Target> {
        match m {
            Machine::i386 => vec![Target::Ia32],
            Machine::X86_64 => vec![Target::Amd64],
            Machine::AVR => vec![
                Target::Atmega103,
                Target::Atmega88,
                Target::Atmega8,
            ],
            _ => vec![],
        }
    }
/*
    pub fn for_pe_machine(m: pe::Machine) -> Vec<Target> {
        // XXX
        vec![]
    }*/

    pub fn name(&self) -> &'static str {
        match self {
            &Target::__Test => "Test",
            &Target::Atmega103 => "ATmega103",
            &Target::Atmega88 => "ATmega88",
            &Target::Atmega8 => "ATmega8",
            &Target::Amd64 => "AMD64",
            &Target::Ia32 => "IA-32",
            &Target::Ia16 => "80286",
            &Target::Mos6502 => "MOS 6502",
        }
    }

    pub fn interrupt_vec(&self) -> Vec<(&'static str,u64,&'static str)> {
        match self {
            &Target::Atmega103 => Mcu::atmega103().int_vec,
            &Target::Atmega88 => Mcu::atmega88().int_vec,
            &Target::Atmega8 => Mcu::atmega8().int_vec,
            &Target::Amd64 => vec![("RESET",0xFFFFFFF0,"Reset vector")],
            &Target::Ia32 => vec![("RESET",0xFFFFFFF0,"Reset vector")],
            &Target::Ia16 => vec![("RESET",0xFFFF0,"Reset vector")],
            &Target::Mos6502 => mos::Variant::mos6502().int_vec,
            &Target::__Test => vec![],
        }
    }

    pub fn disassemble(&self, cont: Option<Function>, i: LayerIter, start: u64, reg: String) -> Function {
        match self {
            &Target::Atmega103 => Function::disassemble::<Avr>(cont,avr::syntax::disassembler(),Mcu::atmega103(),i,start,reg),
            &Target::Atmega88 => Function::disassemble::<Avr>(cont,avr::syntax::disassembler(),Mcu::atmega88(),i,start,reg),
            &Target::Atmega8 => Function::disassemble::<Avr>(cont,avr::syntax::disassembler(),Mcu::atmega8(),i,start,reg),
            &Target::Amd64 => Function::disassemble::<Amd64>(cont,amd64::disassembler(Mode::Long),Config::new(Mode::Long),i,start,reg),
            &Target::Ia32 => Function::disassemble::<Amd64>(cont,amd64::disassembler(Mode::Protected),Config::new(Mode::Protected),i,start,reg),
            &Target::Ia16 => Function::disassemble::<Amd64>(cont,amd64::disassembler(Mode::Real),Config::new(Mode::Real),i,start,reg),
            &Target::Mos6502 => Function::disassemble::<Mos>(cont,mos::generic::disassembler(),mos::Variant::mos6502(),i,start,reg),
            &Target::__Test => panic!(),
        }
    }
}
