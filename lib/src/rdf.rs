use uuid::Uuid;
use std::hash::Hash;
use rand::chacha::ChaChaRng;
use rand::{Rand,SeedableRng};
use std::io::Write;
use std::mem::transmute;

#[derive(Eq,PartialEq,Debug,Hash,Clone)]
pub enum Node {
    Blank(Uuid),
    Iri(String),
    Literal{ value: String, ty: String}
}

#[derive(Eq,PartialEq,Debug,Hash,Clone)]
pub struct Statement {
    subject: Node,
    predicate: Node,
    object: Node,
}

impl Statement {
    pub fn new(s: Node, p: Node, o: Node) -> Statement {
        Statement{ subject: s, predicate: p, object: o }
    }
}

impl Node {
    pub fn from_uuid(u: &Uuid) -> Node {
        Node::Iri(u.to_urn_string())
    }

    pub fn from_ns(ns: &Node, b: &[u8]) -> Node {
        let buf_ns: &[u32] = unsafe { transmute(format!("{:?}",ns).as_bytes()) };
        let buf_sub: &[u32] = unsafe { transmute(b) };
        let mut cc = ChaChaRng::from_seed(&buf_ns.iter().chain(buf_sub.iter()).cloned().collect::<Vec<u32>>());

        Node::from_uuid(&Uuid::rand(&mut cc))
    }

    pub fn new_blank() -> Node {
        Node::Blank(Uuid::new_v4())
    }

    pub fn ns_po(s: &str) -> Node {
        Node::Iri("http://panopticon.re/rdf/v1/".to_string() + s)
    }

    pub fn ns_xsd(s: &str) -> Node {
        Node::Iri("http://www.w3.org/2001/XMLSchema#".to_string() + s)
    }

    pub fn ns_rdf(s: &str) -> Node {
        Node::Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string() + s)
    }

    pub fn lit(s: &String) -> Node {
        Node::Literal{ value: s.clone(), ty: "http://www.w3.org/2001/XMLSchema#string".to_string()}
    }

    pub fn unsigned(s: u64) -> Node {
        Node::Literal{ value: s.to_string(), ty: "http://www.w3.org/2001/XMLSchema#integer".to_string()}
    }

    pub fn signed(s: i64) -> Node {
        Node::Literal{ value: s.to_string(), ty: "http://www.w3.org/2001/XMLSchema#integer".to_string()}
    }

    pub fn iri(&self) -> Option<&String> {
        if let &Node::Iri(ref s) = self {
            Some(s)
        } else {
            None
        }
    }
}
