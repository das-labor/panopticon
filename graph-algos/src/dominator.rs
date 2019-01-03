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

use bit_set::BitSet;
use crate::search::{TraversalOrder, TreeIterator};
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::traits::{BidirectionalGraph, Graph, VertexListGraph};

pub fn dominators<'a, V: 'a, E, G: 'a + Graph<'a, V, E> + BidirectionalGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
    start: G::Vertex,
    graph: &'a G,
) -> HashMap<G::Vertex, Vec<G::Vertex>> {
    let vertex_idx = HashMap::<G::Vertex, usize>::from_iter(graph.vertices().enumerate().map(|(a, b)| (b, a)));
    let vertex_ridx = HashMap::<usize, G::Vertex>::from_iter(graph.vertices().enumerate());
    let mut fixpoint = false;
    let mut all_set = BitSet::new();

    for vx in 0..vertex_idx.len() {
        all_set.insert(vx);
    }

    let mut cur_dom = vec![all_set; vertex_idx.len()];

    while !fixpoint {
        let mut next_dom = vec![BitSet::new(); vertex_idx.len()];

        for vx in graph.vertices() {
            let mut my_dom: Option<BitSet> = None;

            if vx != start {
                for e in graph.in_edges(vx) {
                    let prev = &cur_dom[vertex_idx[&graph.source(e)]];

                    if let Some(ref mut s) = my_dom {
                        s.intersect_with(&prev);
                    } else {
                        my_dom = Some(prev.clone());
                    }
                }
            }

            let mut final_dom = my_dom.unwrap_or(BitSet::new());
            let vx_idx = vertex_idx[&vx];

            final_dom.insert(vx_idx);
            next_dom[vx_idx] = final_dom;
        }

        fixpoint = next_dom == cur_dom;
        cur_dom = next_dom;
    }

    let mut ret = HashMap::<G::Vertex, Vec<G::Vertex>>::new();
    for (vx, idx) in vertex_idx.iter() {
        let mut res = cur_dom[*idx].iter().map(|a| vertex_ridx[&a]).collect::<Vec<G::Vertex>>();

        res.sort();
        ret.insert(*vx, res);
    }

    ret
}

/// Cooper, Harvey, Kennedy: "A Simple, Fast Dominance Algorithm"
pub fn dominance_frontiers<'a, V: 'a, E, G: 'a + Graph<'a, V, E> + BidirectionalGraph<'a, V, E> + VertexListGraph<'a, V, E>>
    (
    idom: &HashMap<G::Vertex, G::Vertex>,
    graph: &'a G,
) -> HashMap<G::Vertex, Vec<G::Vertex>> {
    let mut ret = HashMap::<G::Vertex, Vec<G::Vertex>>::from_iter(graph.vertices().map(|v| (v, vec![])));

    for b in graph.vertices() {
        let pred = {
            let mut ret = graph.in_edges(b).map(|e| graph.source(e)).filter(|&x| x != b).collect::<Vec<G::Vertex>>();
            ret.sort();
            ret.dedup();
            ret
        };

        if pred.len() >= 2 {
            for p in pred {
                let mut runner = p;

                while runner != idom[&b] {
                    ret.entry(runner).or_insert(vec![]).push(b);
                    runner = idom[&runner];
                }
            }
        }
    }

    for (_, v) in ret.iter_mut() {
        v.sort();
        v.dedup();
    }

    ret
}

/// Cooper, Harvey, Kennedy: "A Simple, Fast Dominance Algorithm"
pub fn immediate_dominator<'a, V: 'a, E, G: 'a + Graph<'a, V, E> + BidirectionalGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
    start: G::Vertex,
    graph: &'a G,
) -> HashMap<G::Vertex, G::Vertex> {
    let postorder = TreeIterator::new(start, TraversalOrder::Postorder, graph).collect::<Vec<_>>();
    let po_idx = HashMap::<G::Vertex, usize>::from_iter(postorder.iter().enumerate().map(|(a, b)| (b.clone(), a)));
    fn intersect<'a, V: 'a, E, G: 'a + Graph<'a, V, E> + BidirectionalGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
        b1: G::Vertex,
        b2: G::Vertex,
        po_idx: &HashMap<G::Vertex, usize>,
        postorder: &Vec<G::Vertex>,
        ret: &HashMap<G::Vertex, G::Vertex>,
    ) -> G::Vertex {
        let mut f1 = po_idx[&b1];
        let mut f2 = po_idx[&b2];

        while f1 != f2 {
            while f1 < f2 {
                f1 = po_idx[&ret[&postorder[f1]]];
            }
            while f2 < f1 {
                f2 = po_idx[&ret[&postorder[f2]]];
            }
        }

        postorder[f1]
    };

    let mut ret = HashMap::<G::Vertex, G::Vertex>::new();
    let mut fixpoint = false;

    ret.insert(start, start);

    while !fixpoint {
        fixpoint = true;

        for b in postorder.iter().rev().filter(|&&v| v != start) {
            let pred = {
                let mut ret = graph.in_edges(*b).map(|e| graph.source(e)).filter(|&x| x != *b).collect::<Vec<G::Vertex>>();
                ret.sort();
                ret.dedup();
                ret
            };
            let mut new_idom = *pred.iter().find(|&x| ret.contains_key(x)).unwrap();

            for p in pred.iter().filter(move |&&x| x != new_idom) {
                if ret.contains_key(&p) {
                    new_idom = intersect::<V, E, G>(*p, new_idom, &po_idx, &postorder, &ret);
                }
            }

            if ret.get(b) != Some(&new_idom) {
                ret.insert(*b, new_idom);
                fixpoint = false;
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adjacency_list::AdjacencyList;
    use crate::traits::MutableGraph;

    #[test]
    fn dom() {
        let mut g = AdjacencyList::<usize, ()>::new();
        let v1 = g.add_vertex(1);
        let v2 = g.add_vertex(2);
        let v3 = g.add_vertex(3);
        let v4 = g.add_vertex(4);
        let v5 = g.add_vertex(5);
        let v6 = g.add_vertex(6);

        g.add_edge((), v1, v2);
        g.add_edge((), v2, v3);
        g.add_edge((), v2, v4);
        g.add_edge((), v2, v6);
        g.add_edge((), v3, v5);
        g.add_edge((), v4, v5);
        g.add_edge((), v5, v2);

        let dom = dominators(v1, &g);

        assert_eq!(dom.len(), 6);
        assert_eq!(dom[&v1], vec![v1]);
        assert_eq!(dom[&v2], vec![v1, v2]);
        assert_eq!(dom[&v3], vec![v1, v2, v3]);
        assert_eq!(dom[&v4], vec![v1, v2, v4]);
        assert_eq!(dom[&v5], vec![v1, v2, v5]);
        assert_eq!(dom[&v6], vec![v1, v2, v6]);
    }

    #[test]
    fn idom() {
        let mut g = AdjacencyList::<usize, ()>::new();
        let v1 = g.add_vertex(1);
        let v2 = g.add_vertex(2);
        let v3 = g.add_vertex(3);
        let v4 = g.add_vertex(4);
        let v5 = g.add_vertex(5);
        let v6 = g.add_vertex(6);

        g.add_edge((), v6, v5);
        g.add_edge((), v6, v4);
        g.add_edge((), v5, v1);
        g.add_edge((), v4, v2);
        g.add_edge((), v4, v3);
        g.add_edge((), v3, v2);
        g.add_edge((), v2, v3);
        g.add_edge((), v1, v2);
        g.add_edge((), v2, v1);

        let doms = immediate_dominator(v6, &g);

        assert_eq!(doms.len(), 6);
        assert_eq!(doms[&v1], v6);
        assert_eq!(doms[&v2], v6);
        assert_eq!(doms[&v3], v6);
        assert_eq!(doms[&v4], v6);
        assert_eq!(doms[&v5], v6);
        assert_eq!(doms[&v6], v6);

        let mut g2 = AdjacencyList::<usize, ()>::new();
        let v7 = g2.add_vertex(7);
        g2.add_edge((), v7, v7);
        let doms2 = immediate_dominator(v7, &g2);

        assert_eq!(doms2.len(), 1);
        assert_eq!(doms2[&v7], v7);
    }

    #[test]
    fn issue_5() {
        let mut g = AdjacencyList::<usize, ()>::new();

        let v0 = g.add_vertex(0);
        let v1 = g.add_vertex(1);
        let v2 = g.add_vertex(2);
        let v3 = g.add_vertex(3);
        let v4 = g.add_vertex(4);
        let v5 = g.add_vertex(5);
        let v6 = g.add_vertex(6);
        let v7 = g.add_vertex(7);
        let v8 = g.add_vertex(8);
        let v9 = g.add_vertex(9);
        let v10 = g.add_vertex(10);
        let v11 = g.add_vertex(11);

        g.add_edge((), v0, v2);
        g.add_edge((), v6, v7);
        g.add_edge((), v4, v3);
        g.add_edge((), v1, v9);
        g.add_edge((), v5, v7);
        g.add_edge((), v3, v4);
        g.add_edge((), v10, v11);
        g.add_edge((), v9, v0);
        g.add_edge((), v7, v6);
        g.add_edge((), v2, v4);
        g.add_edge((), v11, v11);
        g.add_edge((), v4, v5);
        g.add_edge((), v8, v10);
        g.add_edge((), v7, v8);

        let doms = immediate_dominator(v1, &g);

        assert_eq!(doms.len(), 12);
    }

    #[test]
    fn frontiers() {
        let mut g = AdjacencyList::<usize, ()>::new();
        let a = g.add_vertex(0);
        let b = g.add_vertex(1);
        let c = g.add_vertex(2);
        let d = g.add_vertex(3);
        let e = g.add_vertex(4);
        let f = g.add_vertex(5);

        g.add_edge((), a, b);
        g.add_edge((), b, c);
        g.add_edge((), b, d);
        g.add_edge((), c, e);
        g.add_edge((), d, e);
        g.add_edge((), e, f);
        g.add_edge((), a, f);

        let idom = immediate_dominator(a, &g);

        assert_eq!(idom.len(), 6);
        assert_eq!(idom[&a], a);
        assert_eq!(idom[&b], a);
        assert_eq!(idom[&c], b);
        assert_eq!(idom[&d], b);
        assert_eq!(idom[&e], b);
        assert_eq!(idom[&f], a);

        let fron = dominance_frontiers(&idom, &g);

        assert_eq!(fron.len(), 6);
        assert_eq!(fron[&a], vec![]);
        assert_eq!(fron[&b], vec![f]);
        assert_eq!(fron[&c], vec![e]);
        assert_eq!(fron[&d], vec![e]);
        assert_eq!(fron[&e], vec![f]);
        assert_eq!(fron[&f], vec![]);
    }
}
