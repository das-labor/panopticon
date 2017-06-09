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

use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::usize;
use traits::{Graph, IncidenceGraph, VertexListGraph};

#[derive(PartialEq,Debug)]
pub enum HierarchicalOrdering<T: Clone> {
    Component(Vec<Box<HierarchicalOrdering<T>>>),
    Element(T),
}

/// Bourdoncle: "Efficient chaotic iteration strategies with widenings"
pub fn weak_topo_order<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
    root: G::Vertex,
    graph: &'a G,
) -> HierarchicalOrdering<G::Vertex>
where
    G::Vertex: Debug,
{
    fn visit<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
        vx: G::Vertex,
        graph: &'a G,
        ret: &mut Vec<Box<HierarchicalOrdering<G::Vertex>>>,
        stack: &mut Vec<G::Vertex>,
        dfn: &mut HashMap<G::Vertex, usize>,
        num: &mut usize,
    ) -> usize
    where
        <G as Graph<'a, V, E>>::Vertex: Debug,
    {
        stack.push(vx.clone());

        let mut _loop = false;

        *num += 1;
        dfn.insert(vx.clone(), *num);
        let mut head = dfn[&vx];

        for e in graph.out_edges(vx) {
            let succ = graph.target(e);
            let min = if dfn[&succ] == 0 {
                visit(succ, graph, ret, stack, dfn, num)
            } else {
                dfn[&succ]
            };

            if min <= head {
                head = min;
                _loop = true;
            }
        }

        if head == dfn[&vx] {
            dfn.insert(vx.clone(), usize::MAX);
            let mut element = stack.pop().unwrap();

            if _loop {
                while element != vx {
                    dfn.insert(element.clone(), 0);
                    element = stack.pop().unwrap();
                }

                ret.insert(0, Box::new(component(vx, graph, stack, dfn, num)));
            } else {
                ret.insert(0, Box::new(HierarchicalOrdering::Element(vx)));
            }
        }

        head
    }

    fn component<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
        vx: G::Vertex,
        graph: &'a G,
        stack: &mut Vec<G::Vertex>,
        dfn: &mut HashMap<G::Vertex, usize>,
        num: &mut usize,
    ) -> HierarchicalOrdering<G::Vertex>
    where
        <G as Graph<'a, V, E>>::Vertex: Debug,
    {
        let mut ret = Vec::<Box<HierarchicalOrdering<G::Vertex>>>::new();

        for e in graph.out_edges(vx) {
            let succ = graph.target(e);
            if dfn[&succ] == 0 {
                visit(succ, graph, &mut ret, stack, dfn, num);
            }
        }

        ret.insert(0, Box::new(HierarchicalOrdering::Element(vx)));
        HierarchicalOrdering::Component(ret)
    }

    let mut dfn = HashMap::<G::Vertex, usize>::from_iter(graph.vertices().map(|v| (v, 0)));
    let mut num = 0;
    let mut ret = Vec::<Box<HierarchicalOrdering<G::Vertex>>>::new();
    let mut stack = Vec::new();

    visit(root, graph, &mut ret, &mut stack, &mut dfn, &mut num);

    HierarchicalOrdering::Component(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use adjacency_list::AdjacencyList;
    use traits::MutableGraph;

    #[test]
    fn wto_double_loop() {
        let mut g = AdjacencyList::<(), ()>::new();
        let vx1 = g.add_vertex(());
        let vx2 = g.add_vertex(());
        let vx3 = g.add_vertex(());
        let vx4 = g.add_vertex(());
        let vx5 = g.add_vertex(());
        let vx6 = g.add_vertex(());
        let vx7 = g.add_vertex(());
        let vx8 = g.add_vertex(());

        g.add_edge((), vx1, vx2);
        g.add_edge((), vx2, vx3);
        g.add_edge((), vx3, vx4);
        g.add_edge((), vx4, vx5);
        g.add_edge((), vx5, vx6);
        g.add_edge((), vx6, vx5);
        g.add_edge((), vx6, vx7);
        g.add_edge((), vx7, vx8);
        g.add_edge((), vx2, vx8);
        g.add_edge((), vx7, vx3);
        g.add_edge((), vx4, vx7);

        let expected = HierarchicalOrdering::Component(
            vec![
                Box::new(HierarchicalOrdering::Element(vx1)),
                Box::new(HierarchicalOrdering::Element(vx2)),
                Box::new(
                    HierarchicalOrdering::Component(
                        vec![
                            Box::new(HierarchicalOrdering::Element(vx3)),
                            Box::new(HierarchicalOrdering::Element(vx4)),
                            Box::new(
                                HierarchicalOrdering::Component(
                                    vec![
                                        Box::new(HierarchicalOrdering::Element(vx5)),
                                        Box::new(HierarchicalOrdering::Element(vx6)),
                                    ]
                                )
                            ),
                            Box::new(HierarchicalOrdering::Element(vx7)),
                        ]
                    )
                ),
                Box::new(HierarchicalOrdering::Element(vx8)),
            ]
        );

        assert_eq!(weak_topo_order(vx1, &g), expected);
    }

    #[test]
    fn wto_factorial() {
        let mut g = AdjacencyList::<(), ()>::new();
        let vx1a = g.add_vertex(());
        let vx2a = g.add_vertex(());
        let vx3a = g.add_vertex(());
        let vx4a = g.add_vertex(());
        let vx5a = g.add_vertex(());
        let vx6a = g.add_vertex(());
        let vx1b = g.add_vertex(());
        let vx2b = g.add_vertex(());
        let vx3b = g.add_vertex(());
        let vx4b = g.add_vertex(());
        let vx5b = g.add_vertex(());
        let vx6b = g.add_vertex(());

        g.add_edge((), vx1a, vx2a);
        g.add_edge((), vx1a, vx4a);
        g.add_edge((), vx2a, vx3a);
        g.add_edge((), vx3a, vx6a);
        g.add_edge((), vx4a, vx1b);
        g.add_edge((), vx5a, vx6a);
        g.add_edge((), vx6a, vx5b);
        g.add_edge((), vx1b, vx2b);
        g.add_edge((), vx1b, vx4b);
        g.add_edge((), vx2b, vx3b);
        g.add_edge((), vx3b, vx6b);
        g.add_edge((), vx4b, vx1a);
        g.add_edge((), vx5b, vx6b);
        g.add_edge((), vx6b, vx5a);

        let expected1 = HierarchicalOrdering::Component(
            vec![
                Box::new(
                    HierarchicalOrdering::Component(
                        vec![
                            Box::new(HierarchicalOrdering::Element(vx1a)),
                            Box::new(HierarchicalOrdering::Element(vx4a)),
                            Box::new(HierarchicalOrdering::Element(vx1b)),
                            Box::new(HierarchicalOrdering::Element(vx4b)),
                        ]
                    )
                ),
                Box::new(HierarchicalOrdering::Element(vx2b)),
                Box::new(HierarchicalOrdering::Element(vx3b)),
                Box::new(HierarchicalOrdering::Element(vx2a)),
                Box::new(HierarchicalOrdering::Element(vx3a)),
                Box::new(
                    HierarchicalOrdering::Component(
                        vec![
                            Box::new(HierarchicalOrdering::Element(vx6a)),
                            Box::new(HierarchicalOrdering::Element(vx5b)),
                            Box::new(HierarchicalOrdering::Element(vx6b)),
                            Box::new(HierarchicalOrdering::Element(vx5a)),
                        ]
                    )
                ),
            ]
        );
        let expected2 = HierarchicalOrdering::Component(
            vec![
                Box::new(
                    HierarchicalOrdering::Component(
                        vec![
                            Box::new(HierarchicalOrdering::Element(vx1a)),
                            Box::new(HierarchicalOrdering::Element(vx4a)),
                            Box::new(HierarchicalOrdering::Element(vx1b)),
                            Box::new(HierarchicalOrdering::Element(vx4b)),
                        ]
                    )
                ),
                Box::new(HierarchicalOrdering::Element(vx2a)),
                Box::new(HierarchicalOrdering::Element(vx3a)),
                Box::new(HierarchicalOrdering::Element(vx2b)),
                Box::new(HierarchicalOrdering::Element(vx3b)),
                Box::new(
                    HierarchicalOrdering::Component(
                        vec![
                            Box::new(HierarchicalOrdering::Element(vx6b)),
                            Box::new(HierarchicalOrdering::Element(vx5a)),
                            Box::new(HierarchicalOrdering::Element(vx6a)),
                            Box::new(HierarchicalOrdering::Element(vx5b)),
                        ]
                    )
                ),
            ]
        );

        let got = weak_topo_order(vx1a, &g);
        assert!(got == expected1 || got == expected2);
    }
}
