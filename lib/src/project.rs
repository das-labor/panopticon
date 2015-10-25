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
use std::io::Write;
use std::borrow::Cow;
use std::fmt::{Arguments,Error};
use std::fmt::Write as WriteFmt;

use program::{Program,CallGraphRef};
use region::{Region,Regions};
use function::Function;
use pe;

use uuid::Uuid;
use rmp_serialize::{Encoder,Decoder};
use rustc_serialize::{Decodable,Encodable};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Project {
    pub name: String,
    pub code: Vec<Program>,
    //data: Vec<Structure>,
    pub sources: Regions,
    pub comments: HashMap<(String,u64),String>,
}

/*struct StringWrite<'a> {
    sink: &'a mut ::std::io::Write
}

impl<'a> StringWrite<'a> {
    pub fn new(w: &'a mut ::std::io::Write) -> StringWrite<'a> {
        StringWrite{ sink: w }
    }
}

impl<'a> ::std::fmt::Write for StringWrite<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        match self.sink.write(s.as_bytes()) {
            Ok(l) => if l == s.len() { Ok(()) } else { Err(Error) },
            Err(_) => Err(Error)
        }
    }

    fn write_char(&mut self, c: char) -> Result<(), Error> {
        let mut buf = String::new();

        buf.push(c);

        match self.sink.write(&buf.into_bytes()) {
            Ok(_l) => Ok(()),
            Err(_) => Err(Error)
        }
    }

    fn write_fmt(&mut self, args: Arguments) -> Result<(), Error> {
        match self.sink.write_fmt(args) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error)
        }
    }
}*/

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
        /*let j = match Json::from_reader(&mut z) {
            Ok(j) => j,
            Err(e) => return Err(Cow::Owned(format!("failed to parse file: {:?}",e))),
        };*/
        let mut rmp = Decoder::new(/*j*/&mut z);
        let res: Result<Project,_> = <Project as Decodable>::decode(&mut rmp);

        match res {
            Ok(p) => Ok(p),
            Err(_) => Err(Cow::Borrowed("session encoding failed"))
        }
    }

    pub fn raw(p: &Path) -> Option<Project> {
        if let Some(nam) = p.file_name().and_then(|x| x.to_str()).or(p.to_str()) {
            if let Some(r) = Region::open(nam.to_string(),p) {
                return Some(Project{
                    name: nam.to_string(),
                    code: Vec::new(),
                    sources: Regions::new(r),
                    comments: HashMap::new(),
                });
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
        let mut fd = try!(match File::create(p) {
            Ok(fd) => Ok(fd),
            Err(e) => Err(Cow::Owned(format!("failed to open save file: {:?}",e)))
        });

        let mut z = ZlibEncoder::new(fd,Compression::Default);
        //let mut bridge = StringWrite::new(&mut z);
        //let mut enc = Encoder::new(&mut bridge);
        let mut enc = Encoder::new(&mut z);

        match self.encode(&mut enc) {
            Ok(()) => Ok(()),
            Err(e) => Err(Cow::Borrowed("failed to write to save file"))
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
