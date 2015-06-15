use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::path::Path;
use mnemonic::Bound;
use std::iter::Repeat;
use std::slice::Iter;
pub type Cell = Option<u8>;

pub enum Slab<'a> {
    Undefined(Repeat<Cell>),
    Sparse(Values<'a,u64,Cell>),
    Raw(Iter<'a,u8>),
    Empty,
}

impl<'a> Iterator for Slab<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        match self {
            &mut Slab::Undefined(ref mut a) => a.next(),
            &mut Slab::Sparse(ref mut a) => a.next().cloned(),
            &mut Slab::Raw(ref mut a) => a.next().map(|a| Some(a.clone())),
            &mut Slab::Empty => None,
        }
    }
}

impl<'a> Slab<'a> {
    pub fn empty() -> Slab<'a> {
        Slab::Empty
    }

    pub fn idx(&mut self, index: usize) -> Option<Cell> {
        None
    }

    pub fn length(&self) -> usize {
        0
    }
}

pub enum Layer {
    Raw{ name: String, data: Vec<u8> },
    Undefined{ name: String, data: u64 },
    Sparse{ name: String, data: HashMap<u64,Cell> }
}

impl Layer {
    pub fn wrap(s: String, d: Vec<u8>) -> Layer {
        Layer::Raw{
            name: s,
            data: d
        }
    }

    pub fn undefined(s: String, l: u64) -> Layer {
        Layer::Undefined{
            name: s,
            data: l
        }
    }

    pub fn writable(s: String) -> Layer {
        Layer::Sparse{
            name: s,
            data: HashMap::new()
        }
    }

    pub fn filter(&self, s: &Slab) -> Slab {
        unimplemented!();
    }

    pub fn name(&self) -> &String {
        unimplemented!();
    }

    pub fn write(&self, p: u64, c: Cell) -> bool {
        unimplemented!();
    }

    pub fn is_undefined(&self) -> bool {
        unimplemented!();
    }

    pub fn is_writeable(&self) -> bool {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain() {
        let l1 = Layer::undefined("anon 1",6);
        let l2 = Layer::wrap("anon 2".to_string(),vec!(1,2,3));
        let l3 = Layer::wrap("anon 2".to_string(),vec!(1,2,3));
        let l4 = Layer::wrap("anon 2".to_string(),vec!(13,23,33,6,7));

        let s1 = l1.filter(Bound::new(2,4))
            .chain(l2.filter(Bound::new(0,3)))
            .chain(l3.filter(Bound::new(1,3)))
            .chain(l4.filter(Bound::new(0,5)));

        assert_eq!(s1.collect(), vec!(None,None,Some(1),Some(2),Some(3),Some(2),Some(3),Some(13),Some(23),Some(33),Some(6),Some(7)));
    }

    #[test]
    fn empty_slab() {
        let s1 = Slab::Empty();

        assert_eq!(s1.length(), 0);
        assert_eq!(s1.next(), None);
        assert_eq!(s1.idx(1337), None);
    }

    #[test]
    fn slab() {
        let l1 = Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16));

        assert_eq!(s1.length(), 16);
        assert_eq!(s1.next(), Some(1));
        assert_eq!(s1.idx(13), Some(14));
        assert_eq!(s1.next(), Some(2));
        assert_eq!(s1.length(), 14);
    }

    #[test]
    fn filter() {
        let l1 = Layer::undefined("anon 1".to_string(),128);
        let l2 = Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5,6));

        assert!(l1.is_undefined());
        assert!(!l2.is_undefined());
        assert_eq!(l1.filter(Slab::new()).length(), 128);
        assert_eq!(l2.filter(Slab::new()).length(), 6);

        assert_eq!(l2.filter(l1.filter(Slab::new())).take(9).collect(),
            vec!(Some(1),Some(2),Some(3),Some(4),Some(5),Some(6),None,None,None));
    }


    #[test]
    fn mutable() {
        let l1 = Layer::wrap("const",vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16));
        let l2 = Layer::writable("mut");
        let e = vec!(1,2,3,4,5,1,1,8,9,10,11,12,13,1,15,16);

        l2.write(5,1);
        l2.write(6,1);
        l2.write(13,1);

        let s = l2.filter(l1.filter(Slab::empty()));
        assert_eq!(s.length(), 16);
        assert_eq!(s.collect(),e);
    }

    #[test]
    fn add() {
        let mut st = Region::undefined("".to_string(),40);

        assert!(st.cover(Bound::new(0,6),Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5,6))));
        assert!(st.cover(Bound::new(10,39),Layer::wrap("anon 3".to_string(),vec!(1,2,3,4,5,6,8,9,10,11,12,13,14,15,16,17,18,19))));
        assert!(st.cover(Bound::new(4,12),Layer::wrap("anon 4".to_string(),vec!(1,2,3,4,5,6,7,8))));

        let proj = st.flatten();

        assert_eq!(proj.len(),4);
        assert_eq!(proj[0].0, Bound::new(0,4));
        assert_eq!(proj[0].1.name, "anon 2");
        assert_eq!(proj[1].0, Bound::new(4,10));
        assert_eq!(proj[1].1.name, "anon 3");
        assert_eq!(proj[2].0, Bound::new(10,39));
        assert_eq!(proj[2].1.name, "anon 4");
        assert_eq!(proj[3].0, Bound::new(39,40));
        assert_eq!(proj[3].1.name, "");
    }

    #[test]
    fn projection() {
        let mut st = Region::undefined("".to_string(),40);

        let base = Layer::undefined("base",128));
        let xor1 = Layer::undefined("xor",64));
        let add = Layer::undefined("add",27));
        let zlib = Layer::undefined("zlib",48));
        let aes = Layer::undefined("aes",32));

        assert!(st.write().add(bound(0,128),base);
        assert!(st.write().add(bound(0,64),xor1);
        assert!(st.write().add(bound(45,72),add);
        assert!(st.write().add(bound(80,128),zlib);
        st.write().add(bound(102,134),aes);

        auto proj = st->flatten();
        list<pair<bound,layer_wloc>> expect;

        expect.emplace_back(bound(0,45),layer_wloc(xor1));
        expect.emplace_back(bound(45,72),layer_wloc(add));
        expect.emplace_back(bound(72,80),layer_wloc(base));
        expect.emplace_back(bound(80,102),layer_wloc(zlib));
        expect.emplace_back(bound(102,134),layer_wloc(aes));

        std::cerr << "proj:" << std::endl;
        for(const std::pair<bound,layer_wloc> &p: proj)
            std::cerr << p.first << " => " << p.second->name() << std::endl;
        std::cerr << "expect:" << std::endl;
        for(const std::pair<bound,layer_wloc> &p: expect)
            std::cerr << p.first << " => " << p.second->name() << std::endl;
        ASSERT_TRUE(proj == expect);
    }*/

    #[test]
    fn random_access_iter()
    {
        let l1 = Layer::undefined("l1".to_string(),0xffffffff));
        let sl = l1.filter(Slab::empty());

        // unused -> auto i = sl.begin();
        // unused -> slab::iterator j = i + 0xc0000000;

        let mut k = 100;
        while(k--) {
            let s2 = sl.chain(sl);
        }
    }
}
