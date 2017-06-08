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

use std::ops::Range;
use traits::*;

#[derive(Eq,Hash,PartialEq,Clone,Debug,Copy)]
pub struct AdjacencyMatrixEdgeDescriptor {
    from: usize,
    to: usize,
}

#[derive(Debug)]
pub struct AdjacencyMatrix<'a, V: 'a, E: 'a> {
    edges: &'a [&'a [Option<E>]],
    vertex_labels: &'a [V],
}

impl<'a, V, E> AdjacencyMatrix<'a, V, E> {
    pub fn new(edges: &'a [&'a [Option<E>]], vertices: &'a [V]) -> Self {
        assert_eq!(edges.len(), vertices.len());

        return AdjacencyMatrix {
                   edges: edges,
                   vertex_labels: vertices,
               };
    }
}

impl<'a, V, E> Graph<'a, V, E> for AdjacencyMatrix<'a, V, E> {
    type Vertex = usize;
    type Edge = AdjacencyMatrixEdgeDescriptor;

    fn edge_label(&self, e: Self::Edge) -> Option<&E> {
        return self.edges[e.from][e.to].as_ref();
    }

    fn vertex_label(&self, v: Self::Vertex) -> Option<&V> {
        return Some(&self.vertex_labels[v]);
    }

    fn source(&self, e: Self::Edge) -> Self::Vertex {
        return e.from;
    }

    fn target(&self, e: Self::Edge) -> Self::Vertex {
        return e.to;
    }
}

impl<'a, V, E> AdjacencyMatrixGraph<'a, V, E> for AdjacencyMatrix<'a, V, E> {
    fn edge(
        &'a self,
        from: Self::Vertex,
        to: Self::Vertex,
    ) -> Option<Self::Edge> {
        if self.edges[from][to].is_some() {
            return Some(AdjacencyMatrixEdgeDescriptor { from: from, to: to });
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
pub struct AdjacencyMatrixNeight<'a, V: 'a, E: 'a> {
    fix: usize,
    var: Range<usize>,
    mat: &'a AdjacencyMatrix<'a, V, E>,
    dir: bool, // true <=> Out
}

impl<'a, V, E> Iterator for AdjacencyMatrixNeight<'a, V, E> {
    type Item = AdjacencyMatrixEdgeDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = self.var.next();

        while n.is_some() {
            if self.dir {
                if self.mat.edges[self.fix][n.unwrap()].is_some() {
                    return Some(
                        AdjacencyMatrixEdgeDescriptor {
                            from: self.fix,
                            to: n.unwrap(),
                        }
                    );
                } else {
                    n = self.var.next();
                }
            } else {
                if self.mat.edges[n.unwrap()][self.fix].is_some() {
                    return Some(
                        AdjacencyMatrixEdgeDescriptor {
                            to: self.fix,
                            from: n.unwrap(),
                        }
                    );
                } else {
                    n = self.var.next();
                }
            }
        }

        return None;
    }
}

impl<'a, V, E> IncidenceGraph<'a, V, E> for AdjacencyMatrix<'a, V, E> {
    type Incidence = AdjacencyMatrixNeight<'a, V, E>;

    fn out_degree(&'a self, from: Self::Vertex) -> usize {
        return (0..self.vertex_labels.len()).fold(
            0, |acc, x| {
                return acc + if self.edges[from][x].is_some() { 1 } else { 0 };
            }
        );
    }

    fn out_edges(&'a self, from: Self::Vertex) -> Self::Incidence {
        return AdjacencyMatrixNeight {
                   fix: from,
                   var: (0..self.vertex_labels.len()),
                   mat: self,
                   dir: true,
               };
    }
}

impl<'a, V, E> BidirectionalGraph<'a, V, E> for AdjacencyMatrix<'a, V, E> {
    fn in_degree(&'a self, to: Self::Vertex) -> usize {
        return (0..self.vertex_labels.len()).fold(
            0, |acc, x| {
                return acc + if self.edges[x][to].is_some() { 1 } else { 0 };
            }
        );
    }

    fn in_edges(&'a self, to: Self::Vertex) -> Self::Incidence {
        return AdjacencyMatrixNeight {
                   fix: to,
                   var: (0..self.vertex_labels.len()),
                   mat: self,
                   dir: false,
               };
    }

    fn degree(&'a self, v: Self::Vertex) -> usize {
        return self.in_degree(v) + self.out_degree(v);
    }
}

#[derive(Debug)]
pub struct AdjacencyMatrixAdjacency {
    adj: Box<Vec<usize>>,
}

impl Iterator for AdjacencyMatrixAdjacency {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        return self.adj.pop();
    }
}

impl<'a, V, E> AdjacencyGraph<'a, V, E> for AdjacencyMatrix<'a, V, E> {
    type Adjacency = AdjacencyMatrixAdjacency;

    fn adjacent_vertices(&'a self, v: Self::Vertex) -> Self::Adjacency {
        let i = self.out_edges(v).map(|x| return self.target(x));
        let o = self.in_edges(v).map(|x| return self.source(x));
        let mut raw = i.chain(o).collect::<Vec<usize>>();

        raw.sort();
        raw.dedup();

        return AdjacencyMatrixAdjacency { adj: Box::new(raw) };
    }
}

impl<'a, V, E> VertexListGraph<'a, V, E> for AdjacencyMatrix<'a, V, E> {
    type Vertices = Range<usize>;

    fn vertices(&'a self) -> Self::Vertices {
        return 0..self.vertex_labels.len();
    }

    fn num_vertices(&self) -> usize {
        return self.vertex_labels.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use traits::*;

    #[test]
    fn test_edge() {
        let v = [42, 13, 1337, 99];
        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        assert_eq!(g.edge(0, 0), None);
        assert!(g.edge(0, 1).is_some());
        assert_eq!(g.edge(0, 2), None);
        assert!(g.edge(0, 3).is_some());
        assert!(g.edge(1, 0).is_some());
        assert_eq!(g.edge(1, 1), None);
        assert!(g.edge(1, 2).is_some());
        assert_eq!(g.edge(1, 3), None);
        assert_eq!(g.edge(2, 0), None);
        assert_eq!(g.edge(2, 1), None);
        assert_eq!(g.edge(2, 2), None);
        assert_eq!(g.edge(2, 3), None);
        assert_eq!(g.edge(3, 0), None);
        assert_eq!(g.edge(3, 1), None);
        assert_eq!(g.edge(3, 2), None);
        assert_eq!(g.edge(3, 3), None);
    }

    #[test]
    fn test_degree() {
        let v = [42, 13, 1337, 99];

        // 1 -> 2, 1 -> 4, 2 -> 1, 2 -> 3
        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let n4 = 3;

        assert_eq!(g.in_degree(n1), 1);
        assert_eq!(g.in_degree(n2), 1);
        assert_eq!(g.in_degree(n3), 1);
        assert_eq!(g.in_degree(n3), 1);

        assert_eq!(g.out_degree(n1), 2);
        assert_eq!(g.out_degree(n2), 2);
        assert_eq!(g.out_degree(n3), 0);
        assert_eq!(g.out_degree(n3), 0);

        assert_eq!(g.degree(n1), 3);
        assert_eq!(g.degree(n2), 3);
        assert_eq!(g.degree(n3), 1);
        assert_eq!(g.degree(n4), 1);
    }

    #[test]
    fn test_adj_iterator() {
        let v = [42, 13, 1337, 99];

        // 1 -> 2, 1 -> 4, 2 -> 1, 2 -> 3
        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let n4 = 3;

        type VertexVec<'a> = Vec<<AdjacencyMatrix<'a,isize,String> as Graph<'a,isize,String>>::Vertex>;

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
    fn test_node_attribute() {
        let v = [42, 13, 1337, 99];

        // 1 -> 2, 1 -> 4, 2 -> 1, 2 -> 3
        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;

        assert!(
            g.vertices().any(|x| (n1 != x) ^ (g.vertex_label(x) == Some(&42)))
        );
        assert!(
            g.vertices().any(|x| (n2 != x) ^ (g.vertex_label(x) == Some(&13)))
        );
        assert!(
            g.vertices().any(
                |x| (n3 != x) ^ (g.vertex_label(x) == Some(&1337))
            )
        );
        assert!(g.vertices().any(|x| g.vertex_label(x) != Some(&69)));
    }

    #[test]
    fn test_usage() {
        let v = [42, 13, 1337];

        let e = [
            &[None, Some("a".to_string()), None][..],
            &[None, None, Some("b".to_string())][..],
            &[Some("c".to_string()), None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let e12 = g.edge(n1, n2);
        let e23 = g.edge(n2, n3);
        let e31 = g.edge(n3, n1);

        assert!(e12.is_some() && e23.is_some() && e31.is_some());

        assert!(e12 != e23);
        assert!(e12 != e31);
        assert!(e23 != e31);

        assert!(g.vertex_label(n1) == Some(&42));
        assert!(g.vertex_label(n2) == Some(&13));
        assert!(g.vertex_label(n3) == Some(&1337));

        assert!(g.edge_label(e12.unwrap()) == Some(&"a".to_string()));
        assert!(g.edge_label(e23.unwrap()) == Some(&"b".to_string()));
        assert!(g.edge_label(e31.unwrap()) == Some(&"c".to_string()));

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
    }

    #[test]
    fn test_out_iterator() {
        let v = [42, 13, 1337, 99];

        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let n4 = 3;
        let e12 = g.edge(n1, n2);
        let e21 = g.edge(n2, n1);
        let e23 = g.edge(n2, n3);
        let e14 = g.edge(n1, n4);

        assert!(
            e12.is_some() && e21.is_some() && e23.is_some() && e14.is_some()
        );

        type EdgeVec<'a> = Vec<<AdjacencyMatrix<'a,isize,String> as Graph<'a,isize,String>>::Edge>;

        let i = g.out_edges(n1).collect::<EdgeVec>();
        assert!(
            i == vec![e12.unwrap(), e14.unwrap()] ||
            i == vec![e14.unwrap(), e12.unwrap()]
        );

        let i = g.out_edges(n2).collect::<EdgeVec>();
        assert!(
            i == vec![e23.unwrap(), e21.unwrap()] ||
            i == vec![e21.unwrap(), e23.unwrap()]
        );

        assert_eq!(g.out_edges(n3).next(), None);
        assert_eq!(g.out_edges(n4).next(), None);
    }

    #[test]
    fn test_in_iterator() {
        let v = [42, 13, 1337, 99];

        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let n4 = 3;
        let e12 = g.edge(n1, n2);
        let e21 = g.edge(n2, n1);
        let e23 = g.edge(n2, n3);
        let e14 = g.edge(n1, n4);

        assert!(
            e12.is_some() && e21.is_some() && e23.is_some() && e14.is_some()
        );

        type EdgeVec<'a> = Vec<<AdjacencyMatrix<'a,isize,String> as Graph<'a,isize,String>>::Edge>;

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
    fn test_vertices_edges_iterators() {
        let v = [42, 13, 1337, 99];

        let e = [
            &[None, Some("a".to_string()), None, Some("d".to_string())][..],
            &[Some("c".to_string()), None, Some("b".to_string()), None][..],
            &[None, None, None, None][..],
            &[None, None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let n4 = 3;
        let e12 = g.edge(n1, n2);
        let e21 = g.edge(n2, n1);
        let e23 = g.edge(n2, n3);
        let e14 = g.edge(n1, n4);

        assert!(
            e12.is_some() && e21.is_some() && e23.is_some() && e14.is_some()
        );

        type VertexSet<'a> = HashSet<<AdjacencyMatrix<'a,isize,String> as Graph<'a,isize,String>>::Vertex>;

        let vs = g.vertices().collect::<VertexSet>();
        assert!(
            vs.contains(&n1) && vs.contains(&n2) && vs.contains(&n3) &&
            vs.contains(&n4)
        );
        assert_eq!(vs.len(), 4);
    }

    #[test]
    fn test_duplicate_label() {
        let v = [42, 13, 13];

        // 1 -> 2, 2 -> 1, 2 -> 3
        let e = [
            &[None, Some("a".to_string()), None][..],
            &[None, None, Some("b".to_string())][..],
            &[None, None, None][..],
        ];

        let g = AdjacencyMatrix::<i16, String>::new(&e[..], &v);

        let n1 = 0;
        let n2 = 1;
        let n3 = 2;
        let e12 = g.edge(n1, n2);
        let e23 = g.edge(n2, n3);

        assert!(e12.is_some() && e23.is_some());

        assert_eq!(g.num_vertices(), 3);
    }
}
