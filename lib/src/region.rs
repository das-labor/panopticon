use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::path::Path;
use mnemonic::Bound;
use std::iter::Repeat;
use std::slice::Iter;
use layer::{Cell,Slab,Layer};
use graph_algos::AdjacencyList;
use graph_algos::adjacency_list::AdjacencyListVertexDescriptor;
use graph_algos::{GraphTrait,MutableGraphTrait};

pub struct Region {
    base: Layer,
    stack: Vec<Layer>,
    name: String,
    size: u64,
}

pub type Regions = AdjacencyList<Region,Bound>;
pub type RegionRef = AdjacencyListVertexDescriptor;

impl Region {
    pub fn open(s: String, p: &Path) -> Region {
        unimplemented!();
    }

    pub fn wrap(s: String, d: Vec<u8>) -> Region {
        Region::new(
            s.clone(),
            Layer::Raw{
                name: s,
                data: d
            })
    }

    pub fn undefined(s: String, l: u64) -> Region {
        Region::new(
            s.clone(),
            Layer::Undefined{
                name: s,
                data: l
            })
    }

    pub fn new(s: String, r: Layer) -> Region {
        Region{
            stack: vec!(),
            name: s,
            size: r.filter(&Slab::empty()).length() as u64,
            base: r,
        }
    }

    pub fn cover(&mut self, b: Bound, l: Layer) -> bool {
        unimplemented!();
    }

    pub fn read(&self) -> Slab {
        unimplemented!();
    }

    pub fn flatten(&self) -> Vec<(Bound,&Layer)> {
        unimplemented!();
    }

    pub fn stack(&self) -> Vec<(Bound,&Layer)> {
        unimplemented!();
    }

    pub fn size(&self) -> usize {
        unimplemented!();
    }

    pub fn name(&self) -> &String {
        unimplemented!();
    }
}

pub fn projection(regs: &Regions) -> Vec<(Bound,RegionRef)> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use mnemonic::Bound;
    use layer::{Cell,Slab,Layer};
    use graph_algos::AdjacencyList;
    use graph_algos::{GraphTrait,MutableGraphTrait};
    use tempdir::TempDir;
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;

    fn fixture<'a>() -> (RegionRef,RegionRef,RegionRef,Regions) {
        let mut regs = AdjacencyList::<Region,Bound>::new();
        let r1 = regs.add_vertex(Region::undefined("base".to_string(),128));
        let r2 = regs.add_vertex(Region::undefined("zlib".to_string(),64));
        let r3 = regs.add_vertex(Region::undefined("aes".to_string(),48));

        regs.add_edge(Bound::new(32,96),r1,r2);
        regs.add_edge(Bound::new(16,32),r1,r3);
        regs.add_edge(Bound::new(0,32),r2,r3);

        (r1,r2,r3,regs)
    }

    #[test]
    fn too_small_layer_cover() {
        let mut st = Region::undefined("".to_string(),12);

        assert!(st.cover(Bound::new(0,6),Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5))));
    }

    #[test]
    fn projection_test() {
        let f = fixture();
        let proj = projection(&f.3);
        let expect = vec!(
            (Bound::new(0,16),f.0),
            (Bound::new(0,48),f.2),
            (Bound::new(32,64),f.1),
            (Bound::new(96,128),f.0));

        assert_eq!(proj,expect);
    }

    #[test]
    fn read_undefined() {
        let r1 = Region::undefined("test".to_string(),128);
        let mut s1 = r1.read();

        assert_eq!(s1.length(), 128);
        assert!(s1.all(|x| x.is_none()));
    }

    #[test]
    fn read_one_layer() {
        let p1 = TempDir::new("test-panop").unwrap();
        let mut r1 = Region::undefined("test".to_string(),128);

        {
            let mut fd = File::create(p1.path().join(Path::new("test"))).unwrap();
            fd.write_all(b"Hello, World");
        }

        r1.cover(Bound::new(1,8),Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5,6,7)));
        r1.cover(Bound::new(50,62),Layer::wrap("anon 2".to_string(),vec!(1,2,3,4,5,6,6,5,4,3,2,1)));
        r1.cover(Bound::new(62,63),Layer::wrap("anon 2".to_string(),vec!(1)));
        r1.cover(Bound::new(70,82),Layer::open("anon 2".to_string(),p1.path()));

        let mut s = r1.read();
        let mut idx = 0;

        assert_eq!(s.length(), 128);

        for i in s {
            if idx >= 1 && idx < 8 {
                assert_eq!(i, Some(idx));
            } else if idx >= 50 && idx < 56 {
                assert_eq!(i, Some(idx - 49));
            } else if idx >= 56 && idx < 62 {
                assert_eq!(i, Some(6 - (idx - 56)));
            } else if idx >= 70 && idx < 82 {
                assert_eq!(i, Some("Hello, World".to_string().into_bytes()[(idx - 70) as usize]));
            } else if idx == 62 {
                assert_eq!(i, Some(1));
            } else {
                assert_eq!(i, None);
            }
            idx += 1;
        }
    }
 }
