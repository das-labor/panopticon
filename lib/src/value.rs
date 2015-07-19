#[derive(Clone,Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
pub enum Endianess {
    Little,
    Big,
}

#[derive(Clone,Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
pub enum Rvalue {
    Constant(u64),
    Undefined,
    Variable{ width: u16, name: String, subscript: Option<u32> },
    Memory{ offset: Box<Rvalue>, bytes: u16, endianess: Endianess, name: String },
}

#[derive(Clone,Debug,PartialEq,Eq,Hash,RustcEncodable,RustcDecodable)]
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

    pub fn to_rv(&self) -> Rvalue {
        Rvalue::from_lvalue(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use msgpack;

    #[test]
    fn construct() {
        let u = Rvalue::Undefined;
        let c = Rvalue::Constant(5);
        let v = Rvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let m = Rvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::Little, name: "ram".to_string() };

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
        let rm = Rvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::Little, name: "ram".to_string() };

        let lu = Lvalue::Undefined;
        let lv = Lvalue::Variable{ name: "n".to_string(), width: 32, subscript: None };
        let lm = Lvalue::Memory{ offset: Box::new(Rvalue::Undefined), bytes: 1, endianess: Endianess::Little, name: "ram".to_string() };

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
        let d = Rvalue::Memory{ offset: Box::new(Rvalue::Constant(5)), bytes: 2, endianess: Endianess::Little, name: "bank1".to_string()};

        let a2 = msgpack::Encoder::to_msgpack(&a).ok().unwrap();
        let b2 = msgpack::Encoder::to_msgpack(&b).ok().unwrap();
        let c2 = msgpack::Encoder::to_msgpack(&c).ok().unwrap();
        let d2 = msgpack::Encoder::to_msgpack(&d).ok().unwrap();

        let a3 = msgpack::from_msgpack(&a2).ok().unwrap();
        let b3 = msgpack::from_msgpack(&b2).ok().unwrap();
        let c3 = msgpack::from_msgpack(&c2).ok().unwrap();
        let d3 = msgpack::from_msgpack(&d2).ok().unwrap();

        assert_eq!(a, a3);
        assert_eq!(b, b3);
        assert_eq!(c, c3);
        assert_eq!(d, d3);
    }
}
