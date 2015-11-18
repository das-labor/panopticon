use std::borrow::Cow;

pub mod parse;
pub mod load;

#[derive(Debug)]
pub struct Error {
    pub msg: Cow<'static,str>
}

impl Error {
    pub fn new(s: &'static str) -> Error {
        Error{ msg: Cow::Borrowed(s) }
    }

    pub fn new_owned(s: String) -> Error {
        Error{ msg: Cow::Owned(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs::File;

    #[test]
    fn elf_load_self() {
        let mut fd = File::open(Path::new("target/debug/qtpanopticon")).ok().unwrap();

        match parse::Ehdr::read(&mut fd) {
            Ok(ehdr) => {
                println!("{:?}",ehdr);
                for p in ehdr.progam_headers.iter() {
                    println!("{:?}",p);
                }
                for s in ehdr.segment_headers.iter() {
                    println!("{:?}",s);
                }
            },
            Err(e) => { panic!(e) }
        }
    }
}
