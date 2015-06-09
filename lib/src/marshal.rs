use uuid::Uuid;
use std::collections::HashSet;
use rdf::{Statement,Node};
use std::sync::Arc;
use lmdb_rs::core::{Environment,DbHandle,EnvBuilder,DbFlags};
use tempdir::TempDir;

#[derive(Eq,PartialEq,Debug,Hash,Clone)]
pub struct Blob {
    data: Arc<Vec<u8>>
}

pub struct Archive {
    pub statements: HashSet<Statement>,
    pub blobs: HashSet<Blob>,
}

pub struct Storage {
    env: Environment,
    database: DbHandle,
    tempdir: TempDir,
    blobs: Vec<Blob>,
}

pub trait Marshal {
    fn marshal(&self, n: &Node) -> Archive;
    fn unmarshal(a: &Archive) -> Self;
}

impl Storage {
    pub fn new() -> Option<Storage> {
        if let Ok(tmp) = TempDir::new("panopticon") {
            debug!("Open temporary directory {:?}.",tmp.path());
            if let Ok(mut env) = EnvBuilder::new().open(&tmp.path(), 0o777) {
                if let Ok(db) = env.get_default_db(DbFlags::empty()) {
                    return Some(Storage{
                        env: env,
                        database: db,
                        tempdir: tmp,
                        blobs: Vec::new() })
                }
            }
        }
        error!("Failed to open temporary directory in Storage::new.");
        None
    }

/*
    fn open(p: &Path) -> Storage
*/
    fn insert(&self, st: Statement) -> bool {
        if let Ok(txn) = self.env.new_transaction() {
            {
                let db = txn.bind(&self.database);
                db.set(&Self::encode_statement(&st),&Vec::new());
            }
            txn.commit().is_ok()
        } else {
            error!("Failed to aquire transaction in Storage::insert. Returning false.");
            false
        }
    }

/*
    fn register_blob(b: Blob) -> bool;
    fn unregister_blob(t: &Uuid) -> Option<Blob>;
*/

    fn has(&self, st: &Statement) -> bool {
        let key = Self::encode_statement(st);

        if let Ok(reader) = self.env.get_reader() {
            let db = reader.bind(&self.database);
            let ret = {
                if let Ok(ref mut i) = db.keyrange(&key,&key) {
                    i.next().is_some()
                } else {
                    error!("Failed to get database iterator in Storage::has. Returning false.");
                    false
                }
            };

            ret
        } else {
            error!("Failed to aquire transaction in Storage::has. Returning false.");
            false
        }
    }

/*
    fn all() -> Vec<Statement>;
    fn find(s: &Node) -> Vec<Statement>;
    fn find(s: &Node, p: &Node) -> Vec<Statement>;
    fn first(s: &Node, p: &Node) -> Option<Statement>;
*/

    fn count(&self) -> usize {
        if let Ok(reader) = self.env.get_reader() {
            let db = reader.bind(&self.database);
            let ret = {
                if let Ok(i) = db.iter() {
                    i.count()
                } else {
                    error!("Failed to get database iterator in Storage::count. Returning 0.");
                    0
                }
            };

            ret
        } else {
            error!("Failed to aquire transaction in Storage::count. Returning 0.");
            0
        }
    }


/*
    fn snapshot(p: &Path);
    fn fetch_blob(t: &Uuid) -> Option<Blob>;
*/

    fn encode_statement(st: &Statement) -> Vec<u8> {
        let sub = Self::encode_node(&st.subject);
        let pred = Self::encode_node(&st.predicate);
        let obj = Self::encode_node(&st.object);

        Self::encode_varint(sub.len()).iter().cloned()
            .chain(sub.iter().cloned())
            .chain(Self::encode_varint(pred.len()).iter().cloned())
            .chain(pred.iter().cloned())
            .chain(Self::encode_varint(obj.len()).iter().cloned())
            .chain(obj.iter().cloned()).collect()
    }

    fn decode_statement<'a,I>(b: &mut I) -> Option<Statement> where I: Iterator<Item=&'a u8> + Clone {
        if let Some(sub_len) = Self::decode_varint(b) {
            let sub_buf = b.take(sub_len).cloned().collect::<Vec<u8>>();
            if let Some(sub) = Self::decode_node(&mut sub_buf.iter()) {
               if let Some(pred_len) = Self::decode_varint(b) {
                    let pred_buf = b.take(pred_len).cloned().collect::<Vec<u8>>();
                    if let Some(pred) = Self::decode_node(&mut pred_buf.iter()) {
                        if let Some(obj_len) = Self::decode_varint(b) {
                            let obj_buf = b.take(obj_len).cloned().collect::<Vec<u8>>();
                            if let Some(obj) = Self::decode_node(&mut obj_buf.iter()) {
                                return Some(Statement::new(sub,pred,obj));
                            }
                        }
                    }
               }
            }
        }
        None
    }

    fn encode_node(n: &Node) -> Vec<u8> {
        match n {
            &Node::Blank(ref u) =>
                vec!(0u8).iter().chain(u.as_bytes().iter()).cloned().collect::<Vec<u8>>(),
            &Node::Iri(ref s) =>
                vec!(1u8).iter().cloned()
                    .chain(Self::encode_varint(s.len()).iter().cloned())
                    .chain(s.bytes())
                    .collect::<Vec<u8>>(),
            &Node::Literal{ value: ref v, ty: ref t } =>
                vec!(2u8).iter().cloned()
                    .chain(Self::encode_varint(v.len()).iter().cloned())
                    .chain(v.bytes())
                    .chain(Self::encode_varint(t.len()).iter().cloned())
                    .chain(t.bytes())
                    .collect::<Vec<u8>>()
        }
    }

    fn encode_varint(n: usize) -> Vec<u8> {
        if n == 0 {
            vec!(0)
        } else {
            let mut rem = n;
            let mut ret = Vec::<u8>::new();

            while rem != 0 {
                if rem != n {
                    ret.push(((rem as u8) & 0x7f) | 0x80);
                } else {
                    ret.push((rem as u8) & 0x7f);
                }
                rem = rem >> 7;
            }

            ret.iter().rev().cloned().collect()
        }
    }

    fn decode_varint<'a,I>(b: &mut I) -> Option<usize> where I: Iterator<Item=&'a u8> + Clone {
        if let Some(p) = b.clone().position(|x| *x <= 0x7f) {
            Some(b.take(p + 1).fold(0usize,|acc: usize,&x| (acc << 7) + (x as usize & 0x7f)))
        } else {
            None
        }
    }

    fn decode_node<'a,I>(i: &mut I) -> Option<Node> where I: Iterator<Item=&'a u8> + Clone {
        match i.next() {
            Some(&0) =>
                if let Some(uu) = Uuid::from_bytes(&(i.take(16).cloned().collect::<Vec<u8>>())) {
                    Some(Node::Blank(uu))
                } else {
                    None
                },
            Some(&1) =>
                if let Some(l2) = Self::decode_varint(i) {
                    if let Ok(v) = String::from_utf8(i.take(l2).cloned().collect()) {
                        Some(Node::Iri(v))
                    } else {
                        None
                    }
                } else {
                    None
                },
            Some(&2) => {
                if let Some(l2) = Self::decode_varint(i) {
                    if let Ok(v) = String::from_utf8(i.take(l2).cloned().collect()) {
                        if let Some(l2) = Self::decode_varint(i) {
                            if let Ok(t) = String::from_utf8(i.take(l2).cloned().collect()) {
                                return Some(Node::Literal{ value: v, ty: t })
                            }
                        }
                    }
                }
                None
            },
            Some(ref x) => {
                error!("Unknown node type while decoding: {}.",x);
                None
            },
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rdf::{Node,Statement};
    use uuid::Uuid;

    #[test]
    fn open_empty() {
        let maybe_st = Storage::new();

        assert!(maybe_st.is_some());
    }

    #[test]
    fn varint() {
        let mut a = Storage::encode_varint(1);
        assert_eq!(a, vec!(1));
        assert_eq!(Some(1), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0x7f);
        assert_eq!(vec!(0x7f), a);
        assert_eq!(Some(0x7f), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0x80);
        assert_eq!(vec!(0x81,0x00), a);
        assert_eq!(Some(0x80), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0x81);
        assert_eq!(a.len(), 2);
        assert_eq!(Some(0x81), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0x3fff);
        assert_eq!(a.len(), 2);
        assert_eq!(Some(0x3fff), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0x4000);
        assert_eq!(a.len(), 3);
        assert_eq!(Some(0x4000), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0x4001);
        assert_eq!(a.len(), 3);
        assert_eq!(Some(0x4001), Storage::decode_varint(&mut a.iter()));

        a = Storage::encode_varint(0);
        assert_eq!(a.len(), 1);
        assert_eq!(Some(0x0), Storage::decode_varint(&mut a.iter()));
    }

    #[test]
    fn node() {
        let a = Node::new_blank();
        let b = Node::ns_po("node");
        let c = Node::unsigned(1);
        let d = Node::lit(&"Hello".to_string());
        let e = Node::lit(&"".to_string());

        let a_enc = Storage::encode_node(&a);
        let b_enc = Storage::encode_node(&b);
        let c_enc = Storage::encode_node(&c);
        let d_enc = Storage::encode_node(&d);
        let e_enc = Storage::encode_node(&e);

        let a2 = Storage::decode_node(&mut a_enc.iter());
        let b2 = Storage::decode_node(&mut b_enc.iter());
        let c2 = Storage::decode_node(&mut c_enc.iter());
        let d2 = Storage::decode_node(&mut d_enc.iter());
        let e2 = Storage::decode_node(&mut e_enc.iter());

        assert_eq!(Some(a), a2);
        assert_eq!(Some(b), b2);
        assert_eq!(Some(c), c2);
        assert_eq!(Some(d), d2);
        assert_eq!(Some(e), e2);
    }

    #[test]
    fn statement() {
        let a = Node::new_blank();
        let b = Node::ns_po("node");
        let c = Node::unsigned(1);
        let d = Node::lit(&"Hello".to_string());
        let e = Node::lit(&"".to_string());
        let f = Node::unsigned(1);

        let st1 = Statement::new(a,b,c);
        let st2 = Statement::new(d,e,f);

        let st1_enc = Storage::encode_statement(&st1);
        let st2_enc = Storage::encode_statement(&st2);

        let st1_2 = Storage::decode_statement(&mut st1_enc.iter());
        let st2_2 = Storage::decode_statement(&mut st2_enc.iter());

        assert_eq!(Some(st1), st1_2);
        assert_eq!(Some(st2), st2_2);
    }

    #[test]
    fn insert() {
        let maybe_store = Storage::new();

        assert!(maybe_store.is_some());
        let store = maybe_store.unwrap();

        let a = Node::new_blank();
        let b = Node::ns_po("node");
        let c = Node::unsigned(1);
        let d = Node::lit(&"Hello".to_string());
        let e = Node::lit(&"".to_string());
        let f = Node::unsigned(1);

        let st1 = Statement::new(a,b,c);
        let st2 = Statement::new(d,e,f);

        assert!(store.insert(st1));
        assert!(store.insert(st2));
    }

    fn fixture_full_store() -> (Storage,Node,Node,Node,Node) {
        let maybe_ret = Storage::new();
        let ret = maybe_ret.unwrap();

        let root = Node::new_blank();
        let a1 = Node::from_uuid(&Uuid::new_v4());
        let a2 = Node::from_uuid(&Uuid::new_v4());
        let b = Node::from_uuid(&Uuid::new_v4());

        ret.insert(Statement::new(a1.clone(), Node::ns_rdf("type"), Node::ns_po("A")));
        ret.insert(Statement::new(a2.clone(), Node::ns_rdf("type"), Node::ns_po("A")));
		ret.insert(Statement::new(b.clone(), Node::ns_rdf("type"), Node::ns_po("B")));
		ret.insert(Statement::new(a1.clone(), Node::ns_po("name"), Node::lit(&"Mr. A".to_string())));
		ret.insert(Statement::new(a1.clone(), Node::ns_po("bs"), b.clone()));
		ret.insert(Statement::new(b.clone(), Node::ns_po("count"), Node::unsigned(42)));
		ret.insert(Statement::new(root.clone(), Node::ns_po("child"), a1.clone()));
		ret.insert(Statement::new(root.clone(), Node::ns_po("child"), a2.clone()));

        (ret,root,a1,a2,b)
	}

    #[test]
    fn empty_store() {
        let maybe_store = Storage::new();
        assert!(maybe_store.is_some());
        assert_eq!(maybe_store.unwrap().count(), 0);
    }

    #[test]
    fn add_single() {
        let maybe_store = Storage::new();

        assert!(maybe_store.is_some());

        let store = maybe_store.unwrap();

        assert!(store.insert(Statement::new(Node::new_blank(),Node::ns_po("test"),Node::new_blank())));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn add_multiple() {
        let maybe_store = Storage::new();

        assert!(maybe_store.is_some());

        let store = maybe_store.unwrap();

        assert!(store.insert(Statement::new(Node::new_blank(), Node::ns_po("test"), Node::new_blank())));
        assert!(store.insert(Statement::new(Node::new_blank(), Node::ns_po("test2"), Node::new_blank())));
        assert!(store.insert(Statement::new(Node::new_blank(), Node::ns_po("test3"), Node::new_blank())));
        assert_eq!(store.count(),3);
    }

    fn add_twice() {
        let maybe_store = Storage::new();

        assert!(maybe_store.is_some());

        let store = maybe_store.unwrap();

        assert!(store.insert(Statement::new(Node::ns_po("La"), Node::ns_po("test"), Node::ns_po("Lo"))));
        assert!(!store.insert(Statement::new(Node::ns_po("La"), Node::ns_po("test"), Node::ns_po("Lo"))));
        assert_eq!(store.count(),1);
    }

    fn find_single() {
        let (store,_,a1,_,_) = fixture_full_store();
        assert!(store.has(&Statement::new(a1, Node::ns_rdf("type"), Node::ns_po("A"))));
    }
/*
    fn find_multiple()
    {
        auto res = full_store->find(root, rdf::ns_po("child"));
        list<statement> exp({
            statement(root, rdf::ns_po("child"), a1),
            statement(root, rdf::ns_po("child"), a2)
        });

        res.sort();
        exp.sort();

        ASSERT_EQ(res,exp);
    }

    TEST_F(store,find_none)
    {
        ASSERT_FALSE(full_store->has(root, rdf::ns_po("child"), rdf::ns_po("NOPE")));
    }

    TEST_F(store,remove)
    {
        ASSERT_TRUE(full_store->remove(a1, rdf::ns_rdf("type"), rdf::ns_po("A")));
        ASSERT_EQ(full_store->count(),7);
        ASSERT_FALSE(full_store->has(a1, rdf::ns_rdf("type"), rdf::ns_po("A")));
    }

    TEST_F(store,find_subject)
    {
        auto res = full_store->find(a1);
        list<statement> exp({
            statement(a1, rdf::ns_rdf("type"), rdf::ns_po("A")),
            statement(a1, rdf::ns_po("name"), rdf::lit("Mr. A")),
            statement(a1, rdf::ns_po("bs"), b)
        });

        res.sort();
        exp.sort();

        ASSERT_EQ(res,exp);
    }*/
/*
    TEST_F(store,blob)
    {
        boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
        boost::filesystem::path p2 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");
        std::ofstream s1(p1.string());

        ASSERT_TRUE(s1.is_open());

        s1 << "Hello, World" << std::flush;
        s1.close();

        uuid u1;
        blob mf1(p1,u1);
        rdf::storage store1;

        ASSERT_TRUE(store1.register_blob(mf1));
        ASSERT_FALSE(store1.register_blob(mf1));

        blob mf2 = store1.fetch_blob(u1);
        ASSERT_EQ(mf1, mf2);

        store1.snapshot(p2);

        rdf::storage store2(p2);

        ASSERT_FALSE(store2.register_blob(mf1));
        blob mf3 = store2.fetch_blob(u1);

        ASSERT_EQ(mf1.size(), mf3.size());

        size_t i = 0;
        while(i < mf3.size())
        {
            ASSERT_EQ(mf1.data()[i], mf3.data()[i]);
            ++i;
        }
    }
*/
}
