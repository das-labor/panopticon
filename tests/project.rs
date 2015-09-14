extern crate panopticon;

use std::path::Path;
use panopticon::project::Project;

#[test]
fn open() {
    /*
    let maybe_p = Project::new("".to_string());

    assert!(maybe_p.is_some());

    let p = maybe_p.unwrap();
    assert_eq!(p.name, "test-project".to_string());
    assert_eq!(p.code, Vec::new());*/
    assert!(false);
}

#[test]
fn pe() {
    let maybe_project = Project::pe(Path::new("tests/data/test.exe"));

    assert!(maybe_project.is_some());
}
