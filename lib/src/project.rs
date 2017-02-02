/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2015  Kai Michaelis
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

//! The root of a Panopticon session.
//!
//! Projects are a set of `Program`s, associated memory `Region`s and comments.

use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read,Write};

use uuid::Uuid;
use rmp_serialize::{Encoder,Decoder};
use rustc_serialize::{Decodable,Encodable};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use byteorder::{
    ReadBytesExt,
    WriteBytesExt,
    BigEndian,
};

use {
    Program,
    CallGraphRef,
    Region,World,
    Function,
    Result,
    pe
};

/// Complete Panopticon session
#[derive(RustcDecodable,RustcEncodable,Debug)]
pub struct Project {
    /// Human-readable name
    pub name: String,
    /// Recognized code
    pub code: Vec<Program>,
    /// Memory regions
    pub data: World,
    /// Comments
    pub comments: HashMap<(String,u64),String>,
}

impl Project {
    /// Returns a new `Project` named `s` from memory `Region` `r`.
    pub fn new(s: String,r: Region) -> Project {
        Project{
            name: s,
            code: Vec::new(),
            data: World::new(r),
            comments: HashMap::new(),
        }
    }

    /// Reads a serialized project from disk.
    pub fn open(p: &Path) -> Result<Project> {
        let mut fd = match File::open(p) {
            Ok(fd) => fd,
            Err(e) => return Err(format!("failed to open file: {:?}",e).into())
        };
        let mut magic = [0u8;10];

        if try!(fd.read(&mut magic)) == 10 && magic == *b"PANOPTICON" {
            let version = try!(fd.read_u32::<BigEndian>());

            if version == 0 {
                let mut z = ZlibDecoder::new(fd);
                let mut rmp = Decoder::new(&mut z);
                let res = <Project as Decodable>::decode(&mut rmp);

                match res {
                    Ok(p) => Ok(p),
                    Err(_) => Err("project decoding failed".into())
                }
            } else {
                Err("wrong version".into())
            }
        } else {
            Err("wrong magic number".into())
        }
    }

    /// Creates a new project from a Windows PE file.
    pub fn pe(p: &Path) -> Option<Project> {
        pe::pe(p)
    }

    /// Returns the program with UUID `uu`
    pub fn find_program_by_uuid(&self,uu: &Uuid) -> Option<&Program> {
        self.code.iter().find(|x| x.uuid == *uu)
    }

    /// Returns the program with UUID `uu`
    pub fn find_program_by_uuid_mut(&mut self,uu: &Uuid) -> Option<&mut Program> {
        self.code.iter_mut().find(|x| x.uuid == *uu)
    }

    /// Returns function and enclosing program with UUID `uu`
    pub fn find_function_by_uuid<'a>(&'a self,uu: &Uuid) -> Option<&'a Function> {
        for p in self.code.iter() {
            if let Some(f) = p.find_function_by_uuid::<'a>(uu) {
                return Some(f);
            }
        }

        None
    }

    /// Returns function and enclosing program with UUID `uu`
    pub fn find_function_by_uuid_mut<'a>(&'a mut self,uu: &Uuid) -> Option<&'a mut Function> {
        for p in self.code.iter_mut() {
            if let Some(f) = p.find_function_by_uuid_mut::<'a>(uu) {
                return Some(f);
            }
        }

        None
    }

    /// Returns function/reference and enclosing program with UUID `uu`
    pub fn find_call_target_by_uuid<'a>(&'a self,uu: &Uuid) -> Option<(CallGraphRef,&'a Program)> {
        for p in self.code.iter() {
            if let Some(ct) = p.find_call_target_by_uuid::<'a>(uu) {
                return Some((ct,p));
            }
        }

        None
    }

    /// Returns function/reference and enclosing program with UUID `uu`
    pub fn find_call_target_by_uuid_mut<'a>(&'a mut self,uu: &Uuid) -> Option<(CallGraphRef,&'a mut Program)> {
        for p in self.code.iter_mut() {
            if let Some(ct) = p.find_call_target_by_uuid::<'a>(uu) {
                return Some((ct,p));
            }
        }

        None
    }

    /// Serializes the project into the file at `p`. The format looks like this:
    /// [u8;10] magic = "PANOPTICON"
    /// u32     version = 0
    /// zlib compressed MsgPack
    pub fn snapshot(&self,p: &Path) -> Result<()> {
        println!("snapshot to {:?}",p);
        let mut fd = try!(File::create(p));

        try!(fd.write(b"PANOPTICON"));
        try!(fd.write_u32::<BigEndian>(0));

        let mut z = ZlibEncoder::new(fd,Compression::Default);
        let mut enc = Encoder::new(&mut z);

        match self.encode(&mut enc) {
            Ok(()) => Ok(()),
            Err(_) => Err("failed to write to save file".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use region::Region;

    #[test]
    fn new() {
        let p = Project::new("test".to_string(),Region::undefined("base".to_string(),128));

        assert_eq!(p.name, "test".to_string());
        assert_eq!(p.code.len(), 0);
    }
}
