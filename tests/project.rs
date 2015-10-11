extern crate panopticon;

use std::path::Path;
use panopticon::project::Project;

#[test]
fn pe() {
    let maybe_project = Project::pe(Path::new("tests/data/test.exe"));

    assert!(maybe_project.is_some());
}
