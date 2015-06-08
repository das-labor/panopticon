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
    fn fetch_blob(t: &Uuid) -> Option<Blob>;*/

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
            Some(&2) =>
                if let Some(l2) = Self::decode_varint(i) {
                    if let Ok(v) = String::from_utf8(i.take(l2).cloned().collect()) {
                        if let Some(l2) = Self::decode_varint(i) {
                            if let Ok(t) = String::from_utf8(i.take(l2).cloned().collect()) {
                                Some(Node::Literal{ value: v, ty: t })
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                },
            _ => None
        }
    }
}
/*
fn encode_key(const node& n) -> [u8];
fn decode_key(i: &mut Iterator<Item=u8>) -> Statement;
*/

#[cfg(test)]
mod tests {
    use super::*;
    use rdf::{Node,Statement};

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

}
