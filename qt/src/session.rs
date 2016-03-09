lazy_static! {
    pub static ref SESSION: RwLock<Option<Session>> = RwLock::new(None);
}

struct Session {
    project: Project,
    backing_file: Path,
    function_layouts: HashMap<Uuid,

}

impl Session {
    pub fn new(p: Project, q: Path) {}
    pub fn sync() {}

}

pub fn read_session<A>(f: Fn(&Session) -> A) -> A {}
pub fn write_session<A>(f: Fn(&mut Session) -> A) -> A {}
pub fn delete_session() {}
pub fn replace_session(p: Project, q: Option<Path>) {}
pub fn sync_session() {}


