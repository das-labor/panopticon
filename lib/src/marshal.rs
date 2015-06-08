use uuid::Uuid;
use std::collections::HashSet;
use rdf::{Statement,Node};
use std::sync::Arc;

#[derive(Eq,PartialEq,Debug,Hash,Clone)]
pub struct Blob {
    data: Arc<Vec<u8>>
}

pub struct Archive {
    pub statements: HashSet<Statement>,
    pub blobs: HashSet<Blob>,
}

pub struct Storage {
    //meta: leveldb::database::Database,
    //tempdir: TempDir,
    blobs: Vec<Blob>,
}

pub trait Marshal {
    fn marshal(&self, n: &Node) -> Archive;
    fn unmarshal(a: &Archive) -> Self;
}

/*impl Storage {
    fn new() -> Storage {
        let tmp = TempDir::new("panopticon-").ok().unwrap();
        Storage{
            meta: leveldb::database::Database::open(tmp.path().push("meta.db")),
            tempdir: tmp,
            blobs: Vec::<Blob>::new()
        }
    }

//    fn open(p: &Path) -> Storage {

    fn open();

    fn insert(st: Statement) -> bool;
    fn insert(s: Node,p: Node, o: Node) -> bool;
    fn register_blob(b: Blob) -> bool;
    fn unregister_blob(t: &Uuid) -> Option<Blob>;

    fn has(st: &Statement) -> bool;
    fn has(s: &Node, p: &Node, o: &Node) -> bool;
    fn all() -> Vec<Statement>;
    fn find(s: &Node) -> Vec<Statement>;
    fn find(s: &Node, p: &Node) -> Vec<Statement>;
    fn first(s: &Node, p: &Node) -> Option<Statement>;
    fn count() -> usize;

    fn snapshot(p: &Path);
    fn fetch_blob(t: &Uuid) -> Option<Blob>;
}
fn encode_node(const node& n) -> [u8];
fn decode_node(i: &mut Iterator<Item=u8>) -> Node;
fn encode_key(const node& n) -> [u8];
fn decode_key(i: &mut Iterator<Item=u8>) -> Statement;
fn encode_varint(const node& n) -> [u8];
fn decode_varint(i: &mut Iterator<Item=u8>) -> Statement;*/
