/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

use std;
use std::clone::Clone;
use std::collections::HashMap;

use traits::*;

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug,PartialOrd,Ord,Serialize,Deserialize)]
pub struct AdjacencyListVertexDescriptor(pub usize);

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug,PartialOrd,Ord,Serialize,Deserialize)]
pub struct AdjacencyListEdgeDescriptor(pub usize);

#[derive(Clone, Debug, Serialize,Deserialize)]
pub struct AdjacencyList<N, E> {
    vertex_labels: HashMap<AdjacencyListVertexDescriptor, N>,
    edge_labels: HashMap<AdjacencyListEdgeDescriptor, E>,
    out_edges: HashMap<AdjacencyListVertexDescriptor, Vec<AdjacencyListEdgeDescriptor>>,
    in_edges: HashMap<AdjacencyListVertexDescriptor, Vec<AdjacencyListEdgeDescriptor>>,
    edges: HashMap<AdjacencyListEdgeDescriptor, (AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)>,
    next_edge: AdjacencyListEdgeDescriptor,
    next_vertex: AdjacencyListVertexDescriptor,
}

#[derive(Debug)]
pub struct AdjacencyListAdjacency {
    adj: Box<Vec<AdjacencyListVertexDescriptor>>,
}

impl Iterator for AdjacencyListAdjacency {
    type Item = AdjacencyListVertexDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        return self.adj.pop();
    }
}

impl<V, E> AdjacencyList<V, E> {
    pub fn new() -> Self {
        return AdjacencyList {
                   vertex_labels: HashMap::new(),
                   edge_labels: HashMap::new(),
                   out_edges: HashMap::new(),
                   in_edges: HashMap::new(),
                   edges: HashMap::new(),
                   next_edge: AdjacencyListEdgeDescriptor(0),
                   next_vertex: AdjacencyListVertexDescriptor(0),
               };
    }
}

impl<'a, V, E> Graph<'a, V, E> for AdjacencyList<V, E> {
    type Vertex = AdjacencyListVertexDescriptor;
    type Edge = AdjacencyListEdgeDescriptor;

    #[inline]
    fn vertex_label(&self, n: Self::Vertex) -> Option<&V> {
        return self.vertex_labels.get(&n);
    }

    fn edge_label(&self, n: Self::Edge) -> Option<&E> {
        return self.edge_labels.get(&n);
    }

    fn source(&self, e: Self::Edge) -> Self::Vertex {
        return self.edges.get(&e).unwrap().0;
    }

    fn target(&self, e: Self::Edge) -> Self::Vertex {
        return self.edges.get(&e).unwrap().1;
    }
}

impl<'a, V, E> IncidenceGraph<'a, V, E> for AdjacencyList<V, E> {
    type Incidence = std::iter::Map<std::slice::Iter<'a, Self::Edge>, fn(&Self::Edge) -> Self::Edge>;

    fn out_degree(&self, v: Self::Vertex) -> usize {
        return self.out_edges.get(&v).map_or(0, |ref x| return x.len());
    }

    fn out_edges(&'a self, v: Self::Vertex) -> Self::Incidence {
        return self.out_edges.get(&v).unwrap().iter().map(std::clone::Clone::clone);
    }
}

impl<'a, V, E> BidirectionalGraph<'a, V, E> for AdjacencyList<V, E> {
    fn in_degree(&self, v: Self::Vertex) -> usize {
        return self.in_edges.get(&v).map_or(0, |ref x| return x.len());
    }

    fn degree(&self, v: Self::Vertex) -> usize {
        return self.in_degree(v) + self.out_degree(v);
    }

    fn in_edges(&'a self, v: Self::Vertex) -> Self::Incidence {
        return self.in_edges.get(&v).unwrap().iter().map(std::clone::Clone::clone);
    }
}

impl<'a, V, E> AdjacencyGraph<'a, V, E> for AdjacencyList<V, E> {
    type Adjacency = AdjacencyListAdjacency;

    fn adjacent_vertices(&self, v: Self::Vertex) -> Self::Adjacency {
        let i = self.out_edges.get(&v).unwrap().iter().map(|&x| { return self.target(x); });
        let o = self.in_edges.get(&v).unwrap().iter().map(|&x| return self.source(x));
        let mut raw = i.chain(o).collect::<Vec<AdjacencyListVertexDescriptor>>();

        raw.sort();
        raw.dedup();

        return AdjacencyListAdjacency { adj: Box::new(raw) };
    }
}

impl<'a, V: 'a, E> VertexListGraph<'a, V, E> for AdjacencyList<V, E> {
    type Vertices = std::iter::Map<std::collections::hash_map::Keys<'a, Self::Vertex, V>, fn(&Self::Vertex) -> Self::Vertex>;

    fn num_vertices(&self) -> usize {
        return self.vertex_labels.len();
    }

    fn vertices(&'a self) -> Self::Vertices {
        return self.vertex_labels.keys().map(std::clone::Clone::clone);
    }
}

impl<'a, V, E: 'a> EdgeListGraph<'a, V, E> for AdjacencyList<V, E> {
    type Edges = std::iter::Map<std::collections::hash_map::Keys<'a, Self::Edge, E>, fn(&Self::Edge) -> Self::Edge>;

    fn num_edges(&self) -> usize {
        return self.edge_labels.len();
    }

    fn edges(&'a self) -> Self::Edges {
        return self.edge_labels.keys().map(std::clone::Clone::clone);
    }
}


impl<'a, V, E> AdjacencyMatrixGraph<'a, V, E> for AdjacencyList<V, E> {
    fn edge(&'a self, from: Self::Vertex, to: Self::Vertex) -> Option<Self::Edge> {
        match self.out_edges(from).find(|&e| self.target(e) == to) {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
}

impl<'a, V, E> MutableGraph<'a, V, E> for AdjacencyList<V, E> {
    fn add_vertex(&mut self, lb: V) -> Self::Vertex {
        let n = self.next_vertex;

        self.next_vertex.0 += 1;
        self.vertex_labels.insert(n, lb);
        self.out_edges.insert(n, Vec::new());
        self.in_edges.insert(n, Vec::new());

        return n;
    }

    fn add_edge(&mut self, lb: E, from: Self::Vertex, to: Self::Vertex) -> Option<Self::Edge> {
        if self.vertex_labels.contains_key(&from) && self.vertex_labels.contains_key(&to) {
            let e = self.next_edge;

            self.next_edge.0 += 1;
            self.edge_labels.insert(e, lb);
            self.edges.insert(e, (from, to));

            let ie = self.in_edges.get_mut(&to);
            let oe = self.out_edges.get_mut(&from);

            if ie.is_some() && oe.is_some() {
                ie.unwrap().push(e);
                oe.unwrap().push(e);
                return Some(e);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn remove_vertex(&mut self, v: Self::Vertex) -> Option<V> {
        let ret = self.vertex_labels.remove(&v);

        if ret.is_some() {
            let todel1 = match self.in_edges.get(&v) {
                Some(_v) => _v.iter().map(|&x| x.clone()).collect(),
                None => Vec::new(),
            };

            let todel2 = match self.out_edges.get(&v) {
                Some(_v) => _v.iter().map(|&x| x.clone()).collect(),
                None => Vec::new(),
            };

            for e in todel1.iter() {
                if !self.remove_edge(*e).is_some() {
                    return None;
                }
            }

            for e in todel2.iter() {
                if !self.remove_edge(*e).is_some() {
                    return None;
                }
            }

            if self.out_edges.remove(&v).is_some() && self.in_edges.remove(&v).is_some() {
                return ret;
            }
        }

        return None;
    }

    fn remove_edge(&mut self, e: Self::Edge) -> Option<E> {
        let ret = self.edge_labels.remove(&e);

        if ret.is_some() {
            let from = &self.source(e);
            let to = &self.target(e);

            if !self.edges.remove(&e).is_some() {
                return None;
            }

            let rm_adj = |cont: Option<&mut Vec<AdjacencyListEdgeDescriptor>>, e| -> bool {
                match cont {
                    None => return false,
                    Some(cont) => {
                        let o = cont.iter().position(|&x| x == e);

                        if o.is_some() {
                            if cont.swap_remove(o.unwrap()) != e {
                                return false;
                            } else {
                                return true;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            };

            if rm_adj(self.out_edges.get_mut(&from), e) && rm_adj(self.in_edges.get_mut(&to), e) {
                return ret;
            }
        }

        return None;
    }

    fn vertex_label_mut(&mut self, n: Self::Vertex) -> Option<&mut V> {
        return self.vertex_labels.get_mut(&n);
    }

    fn edge_label_mut(&mut self, n: Self::Edge) -> Option<&mut E> {
        return self.edge_labels.get_mut(&n);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_node_attribute() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);

        assert!(g.vertices().any(|x| (n1 != x) ^ (g.vertex_label(x) == Some(&42))));
        assert!(g.vertices().any(|x| (n2 != x) ^ (g.vertex_label(x) == Some(&13))));
        assert!(g.vertices().any(|x| (n3 != x) ^ (g.vertex_label(x) == Some(&1337))));
        assert!(g.vertices().any(|x| g.vertex_label(x) != Some(&69)));
    }

    #[test]
    fn test_usage() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e23 = g.add_edge("b".to_string(), n2, n3);
        let e31 = g.add_edge("c".to_string(), n3, n1);

        assert!(e12.is_some() && e23.is_some() && e31.is_some());
        assert!(n1 != n2);
        assert!(n1 != n3);
        assert!(n2 != n3);

        assert!(e12 != e23);
        assert!(e12 != e31);
        assert!(e23 != e31);

        assert!(g.vertex_label(n1) == Some(&42));
        assert!(g.vertex_label(n2) == Some(&13));
        assert!(g.vertex_label(n3) == Some(&1337));

        assert!(g.edge_label(e12.unwrap()) == Some(&"a".to_string()));
        assert!(g.edge_label(e23.unwrap()) == Some(&"b".to_string()));
        assert!(g.edge_label(e31.unwrap()) == Some(&"c".to_string()));

        assert_eq!(3, g.num_edges());
        assert_eq!(3, g.num_vertices());

        assert_eq!(g.source(e12.unwrap()), n1);
        assert_eq!(g.target(e12.unwrap()), n2);
        assert_eq!(g.source(e23.unwrap()), n2);
        assert_eq!(g.target(e23.unwrap()), n3);
        assert_eq!(g.source(e31.unwrap()), n3);
        assert_eq!(g.target(e31.unwrap()), n1);

        assert_eq!(g.out_degree(n1), 1);
        assert_eq!(g.out_degree(n2), 1);
        assert_eq!(g.out_degree(n3), 1);

        assert!(g.remove_edge(e12.unwrap()).is_some());

        assert!(g.remove_vertex(n1).is_some());
        assert!(g.remove_vertex(n2).is_some());
        assert!(g.remove_vertex(n3).is_some());

        assert_eq!(g.num_vertices(), 0);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_degree() {
        let mut g = AdjacencyList::<Option<isize>, String>::new();

        let n1 = g.add_vertex(Some(42));
        let n2 = g.add_vertex(None);
        let n3 = g.add_vertex(Some(42));

        assert!(g.add_edge("a".to_string(), n1, n2) != None);
        let e23 = g.add_edge("a".to_string(), n2, n3);

        assert!(e23.is_some());
        assert!(g.add_edge("a".to_string(), n3, n1) != None);

        assert_eq!(g.in_degree(n1), 1);
        assert_eq!(g.in_degree(n2), 1);
        assert_eq!(g.in_degree(n3), 1);

        assert_eq!(g.out_degree(n1), 1);
        assert_eq!(g.out_degree(n2), 1);
        assert_eq!(g.out_degree(n3), 1);

        let n4 = g.add_vertex(Some(42));
        assert!(g.add_edge("d".to_string(), n4, n1) != None);

        assert_eq!(g.in_degree(n1), 2);
        assert_eq!(g.in_degree(n2), 1);
        assert_eq!(g.in_degree(n3), 1);
        assert_eq!(g.in_degree(n4), 0);

        assert_eq!(g.out_degree(n1), 1);
        assert_eq!(g.out_degree(n2), 1);
        assert_eq!(g.out_degree(n3), 1);
        assert_eq!(g.out_degree(n4), 1);

        assert!(g.remove_edge(e23.unwrap()).is_some());
        g.add_edge("d1".to_string(), n3, n2);

        let n5 = g.add_vertex(None);
        g.add_edge("d1".to_string(), n2, n5);
        g.add_edge("d2".to_string(), n5, n3);
        g.add_edge("d2".to_string(), n5, n4);

        assert_eq!(g.in_degree(n1), 2);
        assert_eq!(g.in_degree(n2), 2);
        assert_eq!(g.in_degree(n3), 1);
        assert_eq!(g.in_degree(n4), 1);
        assert_eq!(g.in_degree(n5), 1);

        assert_eq!(g.out_degree(n1), 1);
        assert_eq!(g.out_degree(n2), 1);
        assert_eq!(g.out_degree(n3), 2);
        assert_eq!(g.out_degree(n4), 1);
        assert_eq!(g.out_degree(n5), 2);

        assert_eq!(g.edges().len(), 7);
    }

    #[test]
    fn test_out_iterator() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);
        let n4 = g.add_vertex(99);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e23 = g.add_edge("b".to_string(), n2, n3);
        let e21 = g.add_edge("c".to_string(), n2, n1);
        let e14 = g.add_edge("d".to_string(), n1, n4);

        assert!(e12.is_some() && e23.is_some() && e21.is_some() && e14.is_some());

        type EdgeVec<'a> = Vec<<AdjacencyList<isize, String> as Graph<'a, isize, String>>::Edge>;

        let i = g.out_edges(n1).collect::<EdgeVec>();
        assert!(i == vec![e12.unwrap(), e14.unwrap()] || i == vec![e14.unwrap(), e12.unwrap()]);

        let i = g.out_edges(n2).collect::<EdgeVec>();
        assert!(i == vec![e23.unwrap(), e21.unwrap()] || i == vec![e21.unwrap(), e23.unwrap()]);

        assert_eq!(g.out_edges(n3).next(), None);
        assert_eq!(g.out_edges(n4).next(), None);
    }

    #[test]
    fn test_in_iterator() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);
        let n4 = g.add_vertex(99);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e23 = g.add_edge("b".to_string(), n2, n3);
        let e21 = g.add_edge("c".to_string(), n2, n1);
        let e14 = g.add_edge("d".to_string(), n1, n4);

        assert!(e12.is_some() && e23.is_some() && e21.is_some() && e14.is_some());

        type EdgeVec<'a> = Vec<<AdjacencyList<isize, String> as Graph<'a, isize, String>>::Edge>;

        let i = g.in_edges(n1).collect::<EdgeVec>();
        assert!(i == vec![e21.unwrap()]);

        let i = g.in_edges(n2).collect::<EdgeVec>();
        assert!(i == vec![e12.unwrap()]);

        let i = g.in_edges(n3).collect::<EdgeVec>();
        assert!(i == vec![e23.unwrap()]);

        let i = g.in_edges(n4).collect::<EdgeVec>();
        assert!(i == vec![e14.unwrap()]);
    }

    #[test]
    fn test_adj_iterator() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);
        let n4 = g.add_vertex(99);

        g.add_edge("a".to_string(), n1, n2);
        g.add_edge("b".to_string(), n2, n3);
        g.add_edge("c".to_string(), n2, n1);
        g.add_edge("d".to_string(), n1, n4);

        type VertexVec<'a> = Vec<<AdjacencyList<isize, String> as Graph<'a, isize, String>>::Vertex>;

        let i = g.adjacent_vertices(n1).collect::<VertexVec>();
        assert!(i == vec![n2, n4] || i == vec![n4, n2]);

        let i = g.adjacent_vertices(n2).collect::<VertexVec>();
        assert!(i == vec![n1, n3] || i == vec![n3, n1]);

        let i = g.adjacent_vertices(n3).collect::<VertexVec>();
        assert!(i == vec![n2]);

        let i = g.adjacent_vertices(n4).collect::<VertexVec>();
        assert!(i == vec![n1]);
    }

    #[test]
    fn test_vertices_edges_iterators() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);
        let n4 = g.add_vertex(99);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e23 = g.add_edge("b".to_string(), n2, n3);
        let e21 = g.add_edge("c".to_string(), n2, n1);
        let e14 = g.add_edge("d".to_string(), n1, n4);

        assert!(e12.is_some() && e23.is_some() && e21.is_some() && e14.is_some());

        type EdgeSet<'a> = HashSet<<AdjacencyList<isize, String> as Graph<'a, isize, String>>::Edge>;
        type VertexSet<'a> = HashSet<<AdjacencyList<isize, String> as Graph<'a, isize, String>>::Vertex>;

        let vs = g.vertices().collect::<VertexSet>();
        assert!(vs.contains(&n1) && vs.contains(&n2) && vs.contains(&n3) && vs.contains(&n4));
        assert_eq!(vs.len(), 4);

        let es = g.edges().collect::<EdgeSet>();
        assert!(es.contains(&e12.unwrap()) && es.contains(&e23.unwrap()) && es.contains(&e21.unwrap()) && es.contains(&e14.unwrap()));
        assert_eq!(es.len(), 4);
    }

    #[test]
    fn test_duplicate_label() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(13);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e23 = g.add_edge("b".to_string(), n2, n3);

        assert!(e12.is_some() && e23.is_some());

        assert_eq!(g.num_vertices(), 3);
        assert_eq!(g.num_edges(), 2);
    }

    #[test]
    fn test_remove_edge_from_node_with_multiple_out_edges() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(12);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e13 = g.add_edge("b".to_string(), n1, n3);

        assert!(e12.is_some() && e13.is_some());

        assert_eq!(g.num_edges(), 2);
        assert_eq!(g.num_vertices(), 3);

        assert!(g.remove_edge(e12.unwrap()).is_some());

        assert_eq!(g.out_degree(n1), 1);
    }

    #[test]
    fn test_edge() {
        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);
        let n4 = g.add_vertex(99);

        let e12 = g.add_edge("a".to_string(), n1, n2);
        let e23 = g.add_edge("b".to_string(), n2, n3);
        let e21 = g.add_edge("c".to_string(), n2, n1);
        let e14 = g.add_edge("d".to_string(), n1, n4);

        assert!(e12.is_some() && e23.is_some() && e21.is_some() && e14.is_some());

        assert_eq!(g.edge(n1, n1), None);
        assert_eq!(g.edge(n1, n2), e12);
        assert_eq!(g.edge(n1, n3), None);
        assert_eq!(g.edge(n1, n4), e14);
        assert_eq!(g.edge(n2, n1), e21);
        assert_eq!(g.edge(n2, n2), None);
        assert_eq!(g.edge(n2, n3), e23);
        assert_eq!(g.edge(n2, n4), None);
        assert_eq!(g.edge(n3, n1), None);
        assert_eq!(g.edge(n3, n2), None);
        assert_eq!(g.edge(n3, n3), None);
        assert_eq!(g.edge(n3, n4), None);
        assert_eq!(g.edge(n4, n1), None);
        assert_eq!(g.edge(n4, n2), None);
        assert_eq!(g.edge(n4, n3), None);
        assert_eq!(g.edge(n4, n4), None);
    }

    #[test]
    fn serialize() {
        use rmp_serde::{Deserializer, Serializer};
        use serde::{Deserialize, Serialize};

        let mut g = AdjacencyList::<isize, String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);
        let n4 = g.add_vertex(99);

        let _ = g.add_edge("a".to_string(), n1, n2);
        let _ = g.add_edge("b".to_string(), n2, n3);
        let _ = g.add_edge("c".to_string(), n2, n1);
        let _ = g.add_edge("d".to_string(), n1, n4);

        let mut buf = Vec::new();

        {
            let mut enc = Serializer::new(&mut buf);
            g.serialize(&mut enc).unwrap();
        }

        let mut rmp = Deserializer::new(&buf[..]);
        let g2: AdjacencyList<isize, String> = Deserialize::deserialize(&mut rmp).unwrap();

        assert_eq!(g.num_vertices(), g2.num_vertices());
        assert_eq!(g.num_edges(), g2.num_edges());
    }
}
