use std::path::Path;
use std::collections::HashMap;

use program::{Program,CallGraphRef};
use region::{Region,Regions};
use function::Function;
use pe;

use uuid::Uuid;

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

    pub fn open(_: &Path) -> Option<Project> {
        unimplemented!()
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
