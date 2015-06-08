use marshal::{Marshal,Archive};
use rdf::{Node,Statement};
use std::collections::HashSet;

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Endianess {
    LittleEndian,
    BigEndian,
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Rvalue {
    Constant(u64),
    Undefined,
    Variable{ width: u16, name: String, subscript: Option<u32> },
    Memory{ offset: Box<Rvalue>, bytes: u16, endianess: Endianess, name: String },
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Lvalue {
    Undefined,
    Variable{ width: u16, name: String, subscript: Option<u32> },
    Memory{ offset: Box<Rvalue>, bytes: u16, endianess: Endianess, name: String },
}

impl Rvalue {
    pub fn from_lvalue(rv: &Lvalue) -> Rvalue {
        match rv {
            &Lvalue::Undefined => Rvalue::Undefined,
            &Lvalue::Variable{ width: ref w, name: ref n, subscript: ref s} =>
                Rvalue::Variable{ width: w.clone(), name: n.clone(), subscript: s.clone()},
            &Lvalue::Memory{ offset: ref o, bytes: ref b, endianess: ref e, name: ref n} =>
                Rvalue::Memory{ offset: o.clone(), bytes: b.clone(), endianess: e.clone(), name: n.clone()},
        }
    }
}

impl Lvalue {
    pub fn from_rvalue(rv: &Rvalue) -> Option<Lvalue> {
        match rv {
            &Rvalue::Undefined => Some(Lvalue::Undefined),
            &Rvalue::Variable{ width: ref w, name: ref n, subscript: ref s} =>
                Some(Lvalue::Variable{ width: w.clone(), name: n.clone(), subscript: s.clone()}),
            &Rvalue::Memory{ offset: ref o, bytes: ref b, endianess: ref e, name: ref n} =>
                Some(Lvalue::Memory{ offset: o.clone(), bytes: b.clone(), endianess: e.clone(), name: n.clone()}),
            _ => None,
        }
    }
}

impl Marshal for Rvalue {
    fn marshal(&self, r: &Node) -> Archive {
        Archive{
            statements: match self {
                &Rvalue::Undefined => vec!(Statement::new(r.clone(),Node::ns_rdf("type"),Node::ns_po("Undefined"))),
                &Rvalue::Constant(ref v) => vec!(
                    Statement::new(r.clone(),Node::ns_rdf("type"),Node::ns_po("Constant")),
                    Statement::new(r.clone(),Node::ns_po("content"),Node::unsigned(*v))),
                &Rvalue::Variable{ name: ref n, width: ref w, subscript: ref s } => {
                    let mut ret = vec!(
                        Statement::new(r.clone(),Node::ns_rdf("type"),Node::ns_po("Variable")),
                        Statement::new(r.clone(),Node::ns_po("name"),Node::lit(n)),
                        Statement::new(r.clone(),Node::ns_po("width"),Node::unsigned(*w as u64)));

                    if s.is_some() {
                        ret.push(Statement::new(r.clone(),Node::ns_po("subscript"),Node::unsigned(s.unwrap() as u64)));
                    }
                    ret
                },
                &Rvalue::Memory{ name: ref n, offset: ref o, bytes: ref b, endianess: ref e } => {
                    let mut ret = vec!(
                        Statement::new(r.clone(),Node::ns_rdf("type"),Node::ns_po("Memory")),
                        Statement::new(r.clone(),Node::ns_po("name"),Node::lit(n)),
                        Statement::new(r.clone(),Node::ns_po("bytes"),Node::unsigned(*b as u64)));
                    match *e {
                        Endianess::LittleEndian => ret.push(Statement::new(r.clone(),Node::ns_po("endianess"),Node::ns_po("little-endian"))),
                        Endianess::BigEndian => ret.push(Statement::new(r.clone(),Node::ns_po("endianess"),Node::ns_po("big-endian"))),
                    }

                    let mut y = o.marshal(&Node::from_ns(r,"offset".as_bytes()));
                    ret.iter().cloned().chain(y.statements.iter().cloned()).collect()
                }
            }.iter().cloned().collect::<HashSet<Statement>>(),
            blobs: HashSet::new()
        }
    }

    fn unmarshal(a: &Archive) -> Rvalue {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use marshal::Marshal;
    use uuid::Uuid;
    use rdf::Node;

    #[test]
    fn construct() {
        let u = Rvalue::Undefined;
        let c = Rvalue::Constant(5);
        let v = Rvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let m = Rvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::LittleEndian, name: "ram".to_string() };

        let u2 = u.clone();
        let c2 = c.clone();
        let v2 = v.clone();
        let m2 = m.clone();

        println!("{:?} {:?} {:?} {:?}",u,c,v,m);

        assert_eq!(u,u2);
        assert_eq!(c,c2);
        assert_eq!(v,v2);
        assert_eq!(m,m2);
    }

    #[test]
    fn convert_lvalue_rvalue() {
        let ru = Rvalue::Undefined;
        let rc = Rvalue::Constant(5);
        let rv = Rvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let rm = Rvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::LittleEndian, name: "ram".to_string() };

        let lu = Lvalue::Undefined;
        let lv = Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let lm = Lvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::LittleEndian, name: "ram".to_string() };

        assert_eq!(Some(lu.clone()), Lvalue::from_rvalue(&ru));
        assert_eq!(Some(lv.clone()), Lvalue::from_rvalue(&rv));
        assert_eq!(Some(lm.clone()), Lvalue::from_rvalue(&rm));
        assert_eq!(None, Lvalue::from_rvalue(&rc));

        assert_eq!(ru, Rvalue::from_lvalue(&lu));
        assert_eq!(rv, Rvalue::from_lvalue(&lv));
        assert_eq!(rm, Rvalue::from_lvalue(&lm));
    }

    #[test]
    fn marshal() {
        let a = Rvalue::Undefined;
        let b = Rvalue::Constant(42);
        let c = Rvalue::Variable{ name: "test".to_string(), width: 8, subscript: Some(8) };
        let d = Rvalue::Memory{ offset: Box::new(Rvalue::Constant(5)), bytes: 2, endianess: Endianess::LittleEndian, name: "bank1".to_string()};

        let a2 = a.marshal(&Node::from_uuid(&Uuid::new_v4()));
        let b2 = b.marshal(&Node::from_uuid(&Uuid::new_v4()));
        let c2 = c.marshal(&Node::from_uuid(&Uuid::new_v4()));
        let d2 = d.marshal(&Node::from_uuid(&Uuid::new_v4()));

        let a3 = Rvalue::unmarshal(&a2);
        let b3 = Rvalue::unmarshal(&b2);
        let c3 = Rvalue::unmarshal(&c2);
        let d3 = Rvalue::unmarshal(&d2);

        assert_eq!(a, a3);
        assert_eq!(b, b3);
        assert_eq!(c, c3);
        assert_eq!(d, d3);
    }
}
