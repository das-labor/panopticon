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
use std::borrow::Cow;

use program::{CallTarget,Program,CallGraphRef};
use region::{Region,Regions};
use function::{Function};
use target::Target;
use pe;

use uuid::Uuid;
use rmp_serialize::{Encoder,Decoder};
use rustc_serialize::{Decodable,Encodable};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use graph_algos::MutableGraphTrait;

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

    pub fn open(p: &Path) -> Result<Project,Cow<str>> {
        let fd = match File::open(p) {
            Ok(fd) => fd,
            Err(e) => return Err(Cow::Owned(format!("failed to open file: {:?}",e)))
        };

        let mut z = ZlibDecoder::new(fd);
        let mut rmp = Decoder::new(&mut z);
        let res: Result<Project,_> = <Project as Decodable>::decode(&mut rmp);

        match res {
            Ok(p) => Ok(p),
            Err(_) => Err(Cow::Borrowed("session encoding failed"))
        }
    }

    pub fn raw(t: Target, p: &Path) -> Option<Project> {
        if let Some(nam) = p.file_name().and_then(|x| x.to_str()).or(p.to_str()) {
            if let Some(r) = Region::open(nam.to_string(),p) {
                let mut proj = Project{
                    name: nam.to_string(),
                    code: Vec::new(),
                    sources: Regions::new(r),
                    comments: HashMap::new(),
                };
                let mut prog = Program::new("prog0",t);

                for &(name,off,cmnt) in t.interrupt_vec().iter() {
                    let uu =  Uuid::new_v4();

                    prog.call_graph.add_vertex(CallTarget::Todo(off,Some(name.to_string()),uu));
                    proj.comments.insert((nam.to_string(),off),cmnt.to_string());
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

    pub fn snapshot(&self,p: &Path) -> Result<(),Cow<str>> {
        let fd = try!(match File::create(p) {
            Ok(fd) => Ok(fd),
            Err(e) => Err(Cow::Owned(format!("failed to open save file: {:?}",e)))
        });

        let mut z = ZlibEncoder::new(fd,Compression::Default);
        let mut enc = Encoder::new(&mut z);

        match self.encode(&mut enc) {
            Ok(()) => Ok(()),
            Err(_) => Err(Cow::Borrowed("failed to write to save file"))
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
