/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014, 2015  Panopticon authors
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

//! A regions model continuous memory like RAM, flash memory or files.
//!
//! A region has an unique name that is used to reference it and a size. The
//! size is the number of `Cell`s in a region. A cell either has a value
//! between 0 and 255 or is undefined. `Cell`s are numbered in ascending
//! order starting at 0.
//!
//! Regions can be constructed from files or buffers in memory or be filled with
//! undefined values.
//!
//! Examples
//! --------
//!
//! ```
//! use std::path::Path;
//! use panopticon_core::Region;
//! let file_region = Region::open("file".to_string(),Path::new("path/to/file"));
//! ```
//! This region is named "file" and is filled with the contents of "path/to/file"
//!
//! ```
//! use panopticon_core::Region;
//! let buf = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
//! let buf_region = Region::wrap("buf".to_string(),buf);
//! ```
//! This region is named "buf" and is initialized with the contents of buf.
//!
//! ```
//! use panopticon_core::Region;
//! let undefined_region = Region::undefined("undef".to_string(),4096);
//! ```
//! This region is named "undef" and is just 4k of undefined cells


use {Bound, Layer, LayerIter, OpaqueLayer, Result};
use petgraph::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// A continuous sequcence of `Cell`s
///
/// `Region`s are a stack of [`Layer`](../layer/index.html) inside a single address space. The
/// `Region` is the primary way panopticon handles data. They can be created from files or
/// in-memory buffers.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Region {
    stack: Vec<(Bound, Layer)>,
    name: String,
    size: u64,
}

/// Graph that models overlapping regions.
pub type RegionGraph = Graph<Region, Bound>;
/// Unstable reference for a node in a region graph.
pub type RegionRef = NodeIndex<u32>;
/// Unstable reference to an edge in a region graph.
pub type BoundRef = EdgeIndex<u32>;

/// A set of `Region`s
///
/// All `Region`s of a `Project` are collected into a `World` structure. The `Region`s in a `World`
/// can overlap. Unlike `Layer`s, overlapping `Region`s do not map `Cell`s one-to-one. The overlapping
/// `Region` has a different size than the area it overlaps. Also, iterating over the overlapped part
/// will not yield `Cell`s from the overlapping `Region`. For example, a compressed file inside a `Region`
/// would be overlapped with a new, larger `Region` that holds the result after decompression. A `Program`
/// inside the overlapped `Region` would still see only the compressed version.
#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct World {
    ///< Graph of all `Region`s with edges pointing from the overlapping to the overlapped `Region`.
    pub dependencies: RegionGraph,
    /// Lowest `Region` in the stack.
    pub root: RegionRef,
}

impl Region {
    /// Creates a new `Region` called `name` that is filled with the contents of the file at `path`.
    pub fn open(s: String, p: &Path) -> Result<Region> {
        let layer = OpaqueLayer::open(p)?;
        Ok(Region::new(s.clone(), layer))
    }

    /// Creates a new `Region` called `name`, filled with `data`.
    pub fn wrap(name: String, data: Vec<u8>) -> Region {
        Region::new(name, OpaqueLayer::Defined(Arc::new(data)))
    }

    /// Creates a new `Region` called `name`, of size `len` with all `Cell`s undefined.
    pub fn undefined(name: String, len: u64) -> Region {
        Region::new(name, OpaqueLayer::Undefined(len))
    }

    /// Creates a new `Region` called `name` with the contens of `root`.
    pub fn new(name: String, root: OpaqueLayer) -> Region {
        let l = root.len();
        let b = Layer::Opaque(root);
        Region { stack: vec![(Bound::new(0, l), b)], name: name, size: l }
    }

    /// Applies `layer` to the cells inside `area`.
    ///
    /// # Returns
    /// `false` if `area` is outside of `0..self.size()` of not compatible with `layer`, `true`
    /// otherwise.
    pub fn cover(&mut self, b: Bound, l: Layer) -> bool {
        if b.end <= self.stack[0].0.end {
            if let Some(o) = l.as_opaque() {
                if b.end - b.start > o.len() {
                    return false;
                }
            }

            self.stack.push((b, l));
            true
        } else {
            false
        }
    }

    /// Iterator over all `Cell`s, starting at 0.
    pub fn iter(&self) -> LayerIter {
        let mut ret = self.stack[0].1.as_opaque().unwrap().iter();

        for s in self.stack.iter().skip(1) {
            let &(ref area, ref layer) = s;

            let src = ret.cut(&(area.start..area.end));
            assert_eq!(src.len(), area.end - area.start);

            let mut tmp = layer.filter(src);

            if area.start != 0 {
                tmp = ret.cut(&(0..area.start)).append(tmp);
                assert_eq!(tmp.len(), area.end);
            }

            if area.end < ret.len() {
                tmp = tmp.append(ret.cut(&(area.end..(ret.len()))));
            }

            assert_eq!(ret.len(), tmp.len());
            ret = tmp;
        }

        ret
    }

    fn add<'a>(a: (Bound, &'a Layer), v: Vec<(Bound, &'a Layer)>) -> Vec<(Bound, &'a Layer)> {
        let mut ret = v.iter()
            .fold(
                Vec::new(), |mut acc, x| {
                    if x.0.start >= a.0.start && x.0.end <= a.0.end {
                        // a covers x completly
                        acc
                    } else if x.0.start >= a.0.end || x.0.end <= a.0.start {
                        // a and x don't touch
                        acc.push(x.clone());
                        acc
                    } else if x.0.start > a.0.start && x.0.end >= a.0.end {
                        // a covers start of x
                        let bound = Bound::new(a.0.end, x.0.end);
                        if bound.start < bound.end {
                            acc.push((bound, x.1));
                        }
                        acc
                    } else if a.0.start > x.0.start && a.0.end >= x.0.end {
                        // a covers end of x
                        let bound = Bound::new(x.0.start, a.0.start);

                        if bound.start < bound.end {
                            acc.push((bound, x.1));
                        }
                        acc
                    } else {
                        // a covers middle of x
                        let bound1 = Bound::new(x.0.start, a.0.start);
                        let bound2 = Bound::new(a.0.end, x.0.end);
                        if bound1.start < bound1.end {
                            acc.push((bound1, x.1));
                        }
                        if bound2.start < bound2.end {
                            acc.push((bound2, x.1));
                        }
                        acc
                    }
                }
            );
        ret.push(a);
        ret
    }

    /// Vector of all uncovered parts.
    pub fn flatten(&self) -> Vec<(Bound, &Layer)> {
        let mut ret = Vec::new();
        for x in self.stack.iter() {
            ret = Self::add((x.0.clone(), &x.1), ret);
        }
        ret.sort_by(|a, b| a.0.start.cmp(&b.0.start));
        ret
    }

    /// Stack of all `Layer` and covered area.
    pub fn stack(&self) -> &Vec<(Bound, Layer)> {
        &self.stack
    }

    /// Number of `Cell`s.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Name of the `Region`
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl World {
    /// Creates a new `World` with a single `Region` `reg`
    pub fn new(reg: Region) -> World {
        let mut g = RegionGraph::new();
        // this is no longer safe, since the graph isn't stable :/
        let b = g.add_node(reg);

        World { dependencies: g, root: b }
    }

    /// Vector of all `Region` in `self` and their uncovered area
    pub fn projection(&self) -> Vec<(Bound, RegionRef)> {
        let mut ret = Vec::<(Bound, RegionRef)>::new();
        let mut visited = HashSet::<RegionRef>::new();

        fn step(v: RegionRef, regs: &RegionGraph, ret: &mut Vec<(Bound, RegionRef)>, visited: &mut HashSet<RegionRef>) {
            let reg = regs.node_weight(v).unwrap();
            let mut es = regs.edges_directed(v, Direction::Outgoing).collect::<Vec<_>>();
            let mut last = 0;

            es.sort_by(|&a, &b| a.weight().start.cmp(&b.weight().start));

            for e in es {
                let b = e.weight();
                let nx = e.target();
                let free = Bound::new(last, b.start);

                if last < b.start {
                    ret.push((free, v));
                }
                last = b.end;

                if visited.insert(nx) {
                    step(nx, regs, ret, visited);
                }
            }

            if last < reg.size() {
                let free = Bound::new(last, reg.size());
                ret.push((free, v));
            }
        }

        if self.dependencies.node_count() > 0 {
            step(self.root, &self.dependencies, &mut ret, &mut visited);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use layer::Layer;
    use mnemonic::Bound;
    use panopticon_graph_algos::MutableGraphTrait;

    fn fixture<'a>() -> (RegionRef, RegionRef, RegionRef, World) {
        let mut regs = World::new(Region::undefined("base".to_string(), 128));
        let r1 = regs.root;
        let r2 = regs.dependencies.add_vertex(Region::undefined("zlib".to_string(), 64));
        let r3 = regs.dependencies.add_vertex(Region::undefined("aes".to_string(), 48));

        regs.dependencies.add_edge(Bound::new(32, 96), r1, r2);
        regs.dependencies.add_edge(Bound::new(16, 32), r1, r3);
        regs.dependencies.add_edge(Bound::new(0, 32), r2, r3);

        (r1, r2, r3, regs)
    }

    #[test]
    fn too_small_layer_cover() {
        let mut st = Region::undefined("".to_string(), 12);

        assert!(!st.cover(Bound::new(0, 6), Layer::wrap(vec![1, 2, 3, 4, 5])));
    }

    #[test]
    fn too_large_layer_cover() {
        let mut st = Region::undefined("".to_string(), 3);

        assert!(!st.cover(Bound::new(0, 5), Layer::wrap(vec![1, 2, 3, 4, 5])));
    }

    #[test]
    fn projection() {
        let f = fixture();
        let proj = f.3.projection();
        let expect = vec![
            (Bound::new(0, 16), f.0),
            (Bound::new(0, 48), f.2),
            (Bound::new(32, 64), f.1),
            (Bound::new(96, 128), f.0),
        ];

        assert_eq!(proj, expect);
    }

    #[test]
    fn read_undefined() {
        let r1 = Region::undefined("test".to_string(), 128);
        let mut s1 = r1.iter();

        assert_eq!(s1.len(), 128);
        assert!(s1.all(|x| x.is_none()));
    }

    #[test]
    fn flatten() {
        let mut st = Region::undefined("".to_string(), 140);

        let xor1 = Layer::undefined(64);
        let add = Layer::undefined(27);
        let zlib = Layer::undefined(48);
        let aes = Layer::undefined(32);

        assert!(st.cover(Bound::new(0, 64), xor1));
        assert!(st.cover(Bound::new(45, 72), add));
        assert!(st.cover(Bound::new(80, 128), zlib));
        assert!(st.cover(Bound::new(102, 134), aes));

        let proj = st.flatten();

        assert_eq!(proj.len(), 6);
        assert_eq!(proj[0].0, Bound::new(0, 45));
        assert_eq!(proj[0].1.as_opaque().unwrap().iter().len(), 64);
        assert_eq!(proj[1].0, Bound::new(45, 72));
        assert_eq!(proj[1].1.as_opaque().unwrap().iter().len(), 27);
        assert_eq!(proj[2].0, Bound::new(72, 80));
        assert_eq!(proj[2].1.as_opaque().unwrap().iter().len(), 140);
        assert_eq!(proj[3].0, Bound::new(80, 102));
        assert_eq!(proj[3].1.as_opaque().unwrap().iter().len(), 48);
        assert_eq!(proj[4].0, Bound::new(102, 134));
        assert_eq!(proj[4].1.as_opaque().unwrap().iter().len(), 32);
        assert_eq!(proj[5].0, Bound::new(134, 140));
        assert_eq!(proj[5].1.as_opaque().unwrap().iter().len(), 140);
    }
}
