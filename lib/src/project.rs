use std::path::Path;

use program::Program;

#[derive(RustcDecodable,RustcEncodable,PartialEq,Eq,Debug)]
pub struct Project {
    name: String,
    code: Vec<Program>,
    //data: Vec<Structure>,
    //sources: AdjacencyList<Region,Bound>,
    //comments: Vec<Comment>,
}

impl Project {
    fn new(s: String) -> Project {
        Project{ name: s, code: Vec::new() }
    }

    fn open(p: &Path) -> Option<Project> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn new() {
        let p = Project::new("test".to_string());

        assert_eq!(p.name, "test".to_string());
        assert_eq!(p.code, Vec::new());
    }
}
