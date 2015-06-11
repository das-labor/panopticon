extern crate panopticon;
use self::panopticon::project::Project;

#[test]
fn open() {
    let maybe_p = Project::path(SAVE_PANOP_PATH);

    assert!(maybe_p.is_some());

    let p = maybe_p.unwrap();
    assert_eq!(p.name, "test-project".to_string());
    assert_eq!(p.code, Vec::new());
}
