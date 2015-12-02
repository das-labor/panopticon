/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashSet;
use std::path::Path;
use mnemonic::Bound;
use layer::{Layer,OpaqueLayer,LayerIter};
use graph_algos::adjacency_list::{AdjacencyListEdgeDescriptor,AdjacencyListVertexDescriptor};
use graph_algos::{AdjacencyList,GraphTrait,VertexListGraphTrait,MutableGraphTrait,IncidenceGraphTrait};

/// Regions model continuous memory like RAM, flash memory or files. A region has a unique
/// name that is used to reference it and a size. The size is the number of cells in
/// a region. A cell either has a value between 0 and 255 or is undefined. Cells are
/// numbered in ascending order starting at 0.
///
/// Regions can be constructed from files or buffers in memory or be filled with
/// undefined values.
///
/// Reading from a region is done by calling :cpp:func:`read` on it. The function returns a
/// :cpp:class:`slab` instance that is a range of cells. Each cell is a :cpp:class:`tryte`
/// instance.
#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Region {
    stack: Vec<(Bound,Layer)>,
    name: String,
    size: u64,
}

pub type RegionGraph = AdjacencyList<Region,Bound>;
pub type RegionRef = AdjacencyListVertexDescriptor;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Regions {
    pub dependencies: RegionGraph,
    pub root: RegionRef,
}

impl Region {
    pub fn open(s: String, p: &Path) -> Option<Region> {
        if let Some(l) = OpaqueLayer::open(p) {
            Some(Region::new(s.clone(),l))
        } else {
            None
        }
    }

    pub fn wrap(s: String, d: Vec<u8>) -> Region {
        Region::new(s.clone(),OpaqueLayer::Defined(Box::new(d)))
    }

    pub fn undefined(s: String, l: u64) -> Region {
        Region::new(s.clone(),OpaqueLayer::Undefined(l))
    }

    pub fn new(s: String, r: OpaqueLayer) -> Region {
        let l = r.len();
        let b = Layer::Opaque(r);
        Region{
            stack: vec!((Bound::new(0,l),b)),
            name: s,
            size: l,
        }
    }

    pub fn cover(&mut self, b: Bound, l: Layer) -> bool {
        if b.end <= self.stack[0].0.end {
            if let Some(o) = l.as_opaque() {
                if b.end - b.start > o.len() {
                    return false;
                }
            }

            self.stack.push((b,l));
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> LayerIter {
        let mut ret = self.stack[0].1.as_opaque().unwrap().iter();

        for s in self.stack.iter().skip(1) {
            let &(ref area,ref layer) = s;

            let src = ret.cut(&(area.start..area.end));
            if src.len() != area.end - area.start {
                println!("{:?}",ret);
            }
            assert_eq!(src.len(),area.end - area.start);

            let mut tmp = layer.filter(src);

            if area.start != 0 {
                tmp = ret.cut(&(0..area.start)).append(tmp);
                assert_eq!(tmp.len(),area.end);
            }

            if area.end < ret.len() {
                tmp = tmp.append(ret.cut(&(area.end..(ret.len()))));
            }

            assert_eq!(ret.len(), tmp.len());
            ret = tmp;
        }

        ret
    }

    fn add<'a>(a: (Bound,&'a Layer),v: Vec<(Bound,&'a Layer)>) -> Vec<(Bound,&'a Layer)> {
        let mut ret = v.iter().fold(Vec::new(),|mut acc,x| {
            if x.0.start >= a.0.start && x.0.end <= a.0.end { // a covers x completly
                acc
            } else if x.0.start >= a.0.end || x.0.end <= a.0.start { // a and x don't touch
                acc.push(x.clone());
                acc
            } else if x.0.start > a.0.start && x.0.end >= a.0.end { // a covers start of x
                let bound = Bound::new(a.0.end,x.0.end);
                if bound.start < bound.end {
                    acc.push((bound,x.1));
                }
                acc
            } else if a.0.start > x.0.start && a.0.end >= x.0.end { // a covers end of x
                let bound = Bound::new(x.0.start,a.0.start);

                if bound.start < bound.end {
                    acc.push((bound,x.1));
                }
                acc
            } else { // a covers middle of x
                let bound1 = Bound::new(x.0.start,a.0.start);
                let bound2 = Bound::new(a.0.end,x.0.end);
                if bound1.start < bound1.end {
                    acc.push((bound1,x.1));
                }
                if bound2.start < bound2.end {
                    acc.push((bound2,x.1));
                }
                acc
            }
        });
        ret.push(a);
        ret
    }

    pub fn flatten(&self) -> Vec<(Bound,&Layer)> {
        let mut ret = Vec::new();
        for x in self.stack.iter() {
            ret = Self::add((x.0.clone(),&x.1),ret);
        }
        ret.sort_by(|a,b| a.0.start.cmp(&b.0.start));
        ret
    }

    pub fn stack(&self) -> &Vec<(Bound,Layer)> {
        &self.stack
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Regions {
    pub fn new(r: Region) -> Regions {
        let mut g = RegionGraph::new();
        let b = g.add_vertex(r);

        Regions{
            dependencies: g,
            root: b
        }
    }

    pub fn projection(&self) -> Vec<(Bound,RegionRef)> {
        let mut ret = Vec::<(Bound,RegionRef)>::new();
        let mut visited = HashSet::<RegionRef>::new();

        fn step(v: RegionRef, regs: &RegionGraph, ret: &mut Vec<(Bound,RegionRef)>, visited: &mut HashSet<RegionRef>) {
            let reg = regs.vertex_label(v).unwrap();
            let mut es = regs.out_edges(v).collect::<Vec<AdjacencyListEdgeDescriptor>>();
            let mut last = 0;

            es.sort_by(|&a,&b| regs.edge_label(a).unwrap().start.cmp(&regs.edge_label(b).unwrap().start));

            for e in es {
                let b = regs.edge_label(e).unwrap();
                let nx = regs.target(e);
                let free = Bound::new(last,b.start);

                if last < b.start {
                    ret.push((free,v));
                }
                last = b.end;

                if visited.insert(nx) {
                    step(nx,regs,ret,visited);
                }
            }

            if last < reg.size() {
                let free = Bound::new(last,reg.size());
                ret.push((free,v));
            }
        }

        if self.dependencies.num_vertices() > 0 {
            step(self.root,&self.dependencies,&mut ret,&mut visited);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mnemonic::Bound;
    use layer::Layer;
    use graph_algos::MutableGraphTrait;
    use tempdir::TempDir;
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;

    fn fixture<'a>() -> (RegionRef,RegionRef,RegionRef,Regions) {
        let mut regs = Regions::new(Region::undefined("base".to_string(),128));
        let r1 = regs.root;
        let r2 = regs.dependencies.add_vertex(Region::undefined("zlib".to_string(),64));
        let r3 = regs.dependencies.add_vertex(Region::undefined("aes".to_string(),48));

        regs.dependencies.add_edge(Bound::new(32,96),r1,r2);
        regs.dependencies.add_edge(Bound::new(16,32),r1,r3);
        regs.dependencies.add_edge(Bound::new(0,32),r2,r3);

        (r1,r2,r3,regs)
    }

    #[test]
    fn too_small_layer_cover() {
        let mut st = Region::undefined("".to_string(),12);

        assert!(!st.cover(Bound::new(0,6),Layer::wrap(vec!(1,2,3,4,5))));
    }

    #[test]
    fn too_large_layer_cover() {
        let mut st = Region::undefined("".to_string(),3);

        assert!(!st.cover(Bound::new(0,5),Layer::wrap(vec!(1,2,3,4,5))));
    }

    #[test]
    fn projection() {
        let f = fixture();
        let proj = f.3.projection();
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
        let mut s1 = r1.iter();

        assert_eq!(s1.len(), 128);
        assert!(s1.all(|x| x.is_none()));
    }

    #[test]
    fn read_one_layer() {
        if let Ok(ref tmpdir) = TempDir::new("test-panop") {
            let p1 = tmpdir.path().join(Path::new("test"));

            let mut r1 = Region::undefined("test".to_string(),128);

            {
                let fd = File::create(p1.clone());
                assert!(fd.unwrap().write_all(b"Hello, World").is_ok());
            }

            assert!(r1.cover(Bound::new(1,8),Layer::wrap(vec!(1,2,3,4,5,6,7))));
            assert!(r1.cover(Bound::new(50,62),Layer::wrap(vec!(1,2,3,4,5,6,6,5,4,3,2,1))));
            assert!(r1.cover(Bound::new(62,63),Layer::wrap(vec!(1))));
            assert!(r1.cover(Bound::new(70,82),Layer::open(&p1).unwrap()));

            let s = r1.iter();
            let mut idx = 0;

            assert_eq!(s.len(), 128);

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

    #[test]
    fn flatten() {
        let mut st = Region::undefined("".to_string(),140);

        let xor1 = Layer::undefined(64);
        let add = Layer::undefined(27);
        let zlib = Layer::undefined(48);
        let aes = Layer::undefined(32);

        assert!(st.cover(Bound::new(0,64),xor1));
        assert!(st.cover(Bound::new(45,72),add));
        assert!(st.cover(Bound::new(80,128),zlib));
        assert!(st.cover(Bound::new(102,134),aes));

        let proj = st.flatten();

        assert_eq!(proj.len(), 6);
        assert_eq!(proj[0].0, Bound::new(0,45));
        assert_eq!(proj[0].1.as_opaque().unwrap().iter().len(), 64);
        assert_eq!(proj[1].0, Bound::new(45,72));
        assert_eq!(proj[1].1.as_opaque().unwrap().iter().len(), 27);
        assert_eq!(proj[2].0, Bound::new(72,80));
        assert_eq!(proj[2].1.as_opaque().unwrap().iter().len(), 140);
        assert_eq!(proj[3].0, Bound::new(80,102));
        assert_eq!(proj[3].1.as_opaque().unwrap().iter().len(), 48);
        assert_eq!(proj[4].0, Bound::new(102,134));
        assert_eq!(proj[4].1.as_opaque().unwrap().iter().len(), 32);
        assert_eq!(proj[5].0, Bound::new(134,140));
        assert_eq!(proj[5].1.as_opaque().unwrap().iter().len(), 140);
    }
 }
