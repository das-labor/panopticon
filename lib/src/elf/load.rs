use std::io::Read;
use std::fs::File;
use std::path::Path;

use project::Project;

use elf::*;
use elf::parse::*;

pub fn load(p: &Path) -> Result<Project,Error> {
    let mut fd = File::open(p).ok().unwrap();
    /*let ehdr =*/ try!(Ehdr::read(&mut fd));

 /*   match ehdr.file_type {
        Type::Core | Type::Executable => load_as_executable(&ehdr,&mut fd),
        _ => load_as_library(&ehdr,&mut fd),
    }*/

    unimplemented!()
}
