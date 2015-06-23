use std::collections::HashMap;
use std::path::Path;
use std::iter::{Enumerate,Take,Skip,Chain};
use std::slice::Iter;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

pub type Cell = Option<u8>;

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub enum OpaqueLayer {
    Undefined(u64),
    Defined(Box<Vec<u8>>),
}

#[derive(Clone)]
pub enum LayerIter<'a> {
    Undefined(Range<u64>),
    Defined(Iter<'a,u8>),
    Sparse{ map: &'a HashMap<u64,Cell>, mapped: Box<Enumerate<LayerIter<'a>>> },
    Concat{ car: Box<LayerIter<'a>>, cdr: Box<LayerIter<'a>> },
    Take(Box<Take<LayerIter<'a>>>),
    Skip(Box<Skip<LayerIter<'a>>>),
    Chain(Box<Chain<LayerIter<'a>,LayerIter<'a>>>),
}

impl<'a> Iterator for LayerIter<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        match self {
            &mut LayerIter::Undefined(ref mut r) => r.next().map(|_| None),
            &mut LayerIter::Defined(ref mut r) => r.cloned().next().map(|x| Some(x)),
            &mut LayerIter::Sparse{ map: ref m, mapped: ref mut i } => {
                if let Some((idx,covered)) = i.next() {
                    Some(*m.get(&(idx as u64)).unwrap_or(&covered))
                } else {
                    None
                }
            },
            &mut LayerIter::Concat{ car: ref mut a, cdr: ref mut b } => {
                if let Some(aa) = a.next() {
                    Some(aa)
                } else {
                    b.next()
                }
            },
            &mut LayerIter::Take(ref mut i) => i.next(),
            &mut LayerIter::Skip(ref mut i) => i.next(),
            &mut LayerIter::Chain(ref mut i) => i.next(),
        }
    }
}

impl<'a> LayerIter<'a> {
    pub fn cut(&self, r: &Range<u64>) -> LayerIter<'a> {
        if r.start > 0 {
            LayerIter::Take(Box::new(LayerIter::Skip(Box::new(self.clone().skip(r.start as usize))).clone().take((r.end - r.start) as usize)))
        } else {
            LayerIter::Take(Box::new(self.clone().take(r.end as usize)))
        }
    }

    pub fn append(&self, l: LayerIter<'a>) -> LayerIter<'a> {
        LayerIter::Chain(Box::new(self.clone().chain(l)))
    }
}

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub enum Layer {
    Opaque(OpaqueLayer),
    Sparse(HashMap<u64,Cell>)
}

impl OpaqueLayer {
    pub fn iter(&self) -> LayerIter {
        match self {
            &OpaqueLayer::Undefined(ref len) => LayerIter::Undefined(0..*len),
            &OpaqueLayer::Defined(ref v) => LayerIter::Defined(v.iter()),
        }
    }

    pub fn len(&self) -> u64 {
        match self {
            &OpaqueLayer::Undefined(ref len) => *len,
            &OpaqueLayer::Defined(ref v) => v.len() as u64,
        }
    }

    pub fn open(p: &Path) -> Option<OpaqueLayer> {
        let fd = File::open(p);

        if fd.is_ok() {
            let mut buf = Vec::<u8>::new();
            let len = fd.unwrap().read_to_end(&mut buf);

            if len.is_ok() {
                Some(Self::wrap(buf))
            } else {
                error!("can't read file '{:?}': {:?}",p,len);
                None
            }
        } else {
            error!("can't open file '{:?}",p);
            None
        }
    }

    pub fn wrap(d: Vec<u8>) -> OpaqueLayer {
        OpaqueLayer::Defined(Box::new(d))
    }

    pub fn undefined(l: u64) -> OpaqueLayer {
        OpaqueLayer::Undefined(l)
    }
}

impl Layer {
    pub fn filter<'a>(&'a self,i: LayerIter<'a>) -> LayerIter<'a> {
        match self {
            &Layer::Opaque(ref o) => o.iter(),
            &Layer::Sparse(ref m) => LayerIter::Sparse{ map: m, mapped: Box::new(i.enumerate()) },
        }
    }

    pub fn wrap(d: Vec<u8>) -> Layer {
        Layer::Opaque(OpaqueLayer::wrap(d))
    }

    pub fn undefined(l: u64) -> Layer {
        Layer::Opaque(OpaqueLayer::undefined(l))
    }

    pub fn open(p: &Path) -> Option<Layer> {
        OpaqueLayer::open(p).map(|x| Layer::Opaque(x))
    }

    pub fn writable() -> Layer {
        Layer::Sparse(HashMap::new())
    }

    pub fn write(&mut self, p: u64, c: Cell) -> bool {
        match self {
            &mut Layer::Sparse(ref mut m) => { m.insert(p,c); true },
            _ => false
        }
    }

    pub fn is_undefined(&self) -> bool {
        if let &Layer::Opaque(OpaqueLayer::Undefined(_)) = self {
            true
        } else {
            false
        }
    }

    pub fn is_writeable(&self) -> bool {
        if let &Layer::Sparse(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_opaque<'a>(&'a self) -> Option<&'a OpaqueLayer> {
        match self {
            &Layer::Opaque(ref o) => Some(o),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let l1 = OpaqueLayer::undefined(6);
        let l2 = OpaqueLayer::wrap(vec!(1,2,3));

        assert_eq!(l1.len(),6);
        assert_eq!(l2.len(),3);
    }

    #[test]
    fn append() {
        let l1 = OpaqueLayer::undefined(6);
        let l2 = OpaqueLayer::wrap(vec!(1,2,3));
        let l3 = OpaqueLayer::wrap(vec!(1,2,3));
        let l4 = OpaqueLayer::wrap(vec!(13,23,33,6,7));

        let s1 = l1.iter().append(l2.iter()).append(l3.iter()).append(l4.iter());

        assert_eq!(s1.collect::<Vec<Cell>>(), vec!(None,None,None,None,None,None,Some(1),Some(2),Some(3),Some(1),Some(2),Some(3),Some(13),Some(23),Some(33),Some(6),Some(7)));
    }

    #[test]
    fn slab() {
        let l1 = OpaqueLayer::wrap(vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16));
        let mut s1 = l1.iter();

        assert_eq!(s1.clone().count(), 16);
        assert_eq!(s1.next().unwrap(), Some(1));
        //assert_eq!(s1.idx(13).unwrap(), Some(14));
        assert_eq!(s1.next().unwrap(), Some(2));
        assert_eq!(s1.clone().count(), 14);
    }

    #[test]
    fn mutable() {
        let l1 = OpaqueLayer::wrap(vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16));
        let mut l2 = Layer::writable();
        let e = vec!(Some(1),Some(2),Some(3),Some(4),Some(5),Some(1),Some(1),Some(8),Some(9),Some(10),Some(11),Some(12),Some(13),Some(1),Some(15),Some(16));

        l2.write(5,Some(1));
        l2.write(6,Some(1));
        l2.write(13,Some(1));

        let s = l2.filter(l1.iter());
        assert_eq!(s.clone().count(), 16);
        assert_eq!(s.collect::<Vec<Cell>>(),e);
    }

    #[test]
    fn random_access_iter()
    {
        let l1 = OpaqueLayer::undefined(0xffffffff);
        let sl = l1.iter();

        // unused -> auto i = sl.begin();
        // unused -> slab::iterator j = i + 0xc0000000;

        let mut k = 100;
        while k > 0 {
            sl.append(sl.clone());
            k -= 1;
        }
    }
}
