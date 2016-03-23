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

use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read,Write};

use program::{CallTarget,Program,CallGraphRef};
use region::{Region,Regions};
use layer::{OpaqueLayer,Layer};
use function::{Function};
use target::Target;
use result::Result;
use value::Rvalue;
use mnemonic::Bound;
use pe;

use uuid::Uuid;
use rmp_serialize::{Encoder,Decoder};
use rustc_serialize::{Decodable,Encodable};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use graph_algos::MutableGraphTrait;
use byteorder::{
    ReadBytesExt,
    WriteBytesExt,
    BigEndian,
};

#[derive(RustcDecodable,RustcEncodable)]
pub struct Project {
    pub name: String,
    pub code: Vec<Program>,
    //data: Vec<Structure>,
    pub sources: Regions,
    pub comments: HashMap<(String,u64),String>,
}

impl Project {
    pub fn new(s: String,r: Region) -> Project {
        Project{
            name: s,
            code: Vec::new(),
            sources: Regions::new(r),
            comments: HashMap::new(),
        }
    }

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

    pub fn raw(p: &Path, t: Target,base: u64, entry: Option<u64>) -> Option<Project> {
        if let Some(nam) = p.file_name().and_then(|x| x.to_str()).or(p.to_str()) {
            if let Some(b) = OpaqueLayer::open(p) {
                let mut reg = Region::undefined(nam.to_string(),b.iter().len() + base);

                reg.cover(Bound::new(base,base + b.iter().len()),Layer::Opaque(b));

                let mut proj = Project{
                    name: nam.to_string(),
                    code: Vec::new(),
                    sources: Regions::new(reg),
                    comments: HashMap::new(),
                };
                let mut prog = Program::new("prog0",t);

                if let Some(e) = entry {
                    let uu =  Uuid::new_v4();
                    prog.call_graph.add_vertex(CallTarget::Todo(Rvalue::Constant(e),Some("main".to_string()),uu));
                } else {
                    for &(name,ref off,cmnt) in t.interrupt_vec().iter() {
                        let uu =  Uuid::new_v4();

                        prog.call_graph.add_vertex(CallTarget::Todo(off.clone(),Some(name.to_string()),uu));
                    }
                }

                proj.code.push(prog);

               return Some(proj);
            }
        }

        None
    }

    pub fn pe(p: &Path) -> Option<Project> {
        pe::pe(p)
    }

    pub fn find_program_by_uuid(&self,uu: &Uuid) -> Option<&Program> {
        self.code.iter().find(|x| x.uuid == *uu)
    }

    pub fn find_program_by_uuid_mut(&mut self,uu: &Uuid) -> Option<&mut Program> {
        self.code.iter_mut().find(|x| x.uuid == *uu)
    }

    pub fn find_function_by_uuid<'a>(&'a self,uu: &Uuid) -> Option<&'a Function> {
        for p in self.code.iter() {
            if let Some(f) = p.find_function_by_uuid::<'a>(uu) {
                return Some(f);
            }
        }

        None
    }

    pub fn find_function_by_uuid_mut<'a>(&'a mut self,uu: &Uuid) -> Option<&'a mut Function> {
        for p in self.code.iter_mut() {
            if let Some(f) = p.find_function_by_uuid_mut::<'a>(uu) {
                return Some(f);
            }
        }

        None
    }

    pub fn find_call_target_by_uuid<'a>(&'a self,uu: &Uuid) -> Option<(CallGraphRef,&'a Program)> {
        for p in self.code.iter() {
            if let Some(ct) = p.find_call_target_by_uuid::<'a>(uu) {
                return Some((ct,p));
            }
        }

        None
    }

    pub fn find_call_target_by_uuid_mut<'a>(&'a mut self,uu: &Uuid) -> Option<(CallGraphRef,&'a mut Program)> {
        for p in self.code.iter_mut() {
            if let Some(ct) = p.find_call_target_by_uuid::<'a>(uu) {
                return Some((ct,p));
            }
        }

        None
    }

    /**
     * [u8;10] magic
     * u32     version
     * (rest)
     */
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
