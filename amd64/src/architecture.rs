/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015, 2017  Panopticon authors
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

use panopticon_core::{Architecture, Match, Region, Result};

#[derive(Clone,Debug)]
pub enum Amd64 {}

#[derive(Clone,PartialEq,Copy,Debug)]
pub enum Mode {
    Real, // Real mode / Virtual 8086 mode
    Protected, // Protected mode / Long compatibility mode
    Long, // Long 64-bit mode
}

impl Mode {
    pub fn alt_bits(&self) -> usize {
        match self {
            &Mode::Real => 32,
            &Mode::Protected => 16,
            &Mode::Long => 16,
        }
    }

    pub fn bits(&self) -> usize {
        match self {
            &Mode::Real => 16,
            &Mode::Protected => 32,
            &Mode::Long => 64,
        }
    }
}

impl Architecture for Amd64 {
    type Token = u8;
    type Configuration = Mode;

    fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
        Ok(vec![])
    }

    fn decode(reg: &Region, start: u64, cfg: &Self::Configuration) -> Result<Match<Self>> {
        let data = reg.iter(start)?;
        let mut buf: [u8; 15] = [0; 15];
        let mut len = 0;
        for byte in data {
            buf[len] = *byte;
            len += 1;
            if len == 15 {
                break;
            }
        }
        let buf = &buf[0..len];
        debug!("disass @ {:#x}: {:?}", start, buf);

        let ret = ::disassembler::read(*cfg, buf, start).and_then(
            |(_len, mne, mut jmp)| {
                Ok(
                    Match::<Amd64> {
                        //tokens: buf[0..len as usize].to_vec(),
                        mnemonics: vec![mne],
                        jumps: jmp.drain(..).map(|x| (start, x.0, x.1)).collect::<Vec<_>>(),
                        configuration: cfg.clone(),
                    }
                )
            }
        );

        debug!("    res: {:?}", ret);

        ret
    }
}
