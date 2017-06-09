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

use std::collections::{HashMap, HashSet};
use traits::{BidirectionalGraph, Graph, IncidenceGraph, VertexListGraph};

#[derive(Clone,Copy,PartialEq)]
pub enum TraversalOrder {
   Preorder,
   Postorder,
}

pub struct TreeIterator<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>> {
   order: TraversalOrder,
   stack: Vec<G::Vertex>,
   seen: HashSet<G::Vertex>,
   position: G::Vertex,
   graph: &'a G,
}

impl<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>> TreeIterator<'a, V, E, G> {
   pub fn new(start: G::Vertex, order: TraversalOrder, g: &'a G) -> Self {
      TreeIterator {
         order: order,
         stack: if order == TraversalOrder::Postorder {
            vec![start]
         } else {
            vec![]
         },
         seen: HashSet::<_>::new(),
         position: start,
         graph: g,
      }
   }

   fn next_preorder(&mut self) -> Option<G::Vertex> {
      if !self.seen.contains(&self.position) {
         self.seen.insert(self.position);

         return Some(self.position);
      } else {
         loop {
            for out in self.graph.out_edges(self.position) {
               let vx = self.graph.target(out);
               if !self.seen.contains(&vx) {
                  self.seen.insert(vx);
                  self.stack.push(self.position);
                  self.position = vx;
                  return Some(vx);
               }
            }

            if let Some(parent) = self.stack.pop() {
               self.position = parent;
            } else {
               return None;
            }
         }
      }
   }

   fn next_postorder(&mut self) -> Option<G::Vertex> {
      loop {
         let mut cont = false;
         for out in self.graph.out_edges(self.position) {
            let vx = self.graph.target(out);
            if !self.seen.contains(&vx) {
               self.seen.insert(vx);
               self.stack.push(self.position);
               self.position = vx;
               cont = true;
               break;
            }
         }

         if cont {
            continue;
         }

         if let Some(parent) = self.stack.pop() {
            let t = self.position;
            self.position = parent;
            return Some(t);
         } else {
            return None;
         }
      }
   }
}

impl<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>> Iterator for TreeIterator<'a, V, E, G> {
   type Item = G::Vertex;
   fn next(&mut self) -> Option<G::Vertex> {
      match self.order {
         TraversalOrder::Preorder => self.next_preorder(),
         TraversalOrder::Postorder => self.next_postorder(),
      }
   }
}

pub fn is_connected<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + BidirectionalGraph<'a, V, E> + VertexListGraph<'a, V, E>>(graph: &'a G)
   -> bool {
   let mut seen = HashSet::<G::Vertex>::new();

   if let Some(s) = graph.vertices().next() {
      let mut stack = vec![s];

      while !stack.is_empty() {
         let vx = stack.pop().unwrap().clone();

         seen.insert(vx);
         let ed = graph.out_edges(vx).map(|out| graph.target(out)).chain(graph.in_edges(vx).map(|_in| graph.source(_in))).collect::<Vec<_>>();

         for s in ed {
            if !seen.contains(&s) {
               stack.push(s);
            }
         }
      }

      assert!(seen.len() <= graph.num_vertices());
      seen.len() == graph.num_vertices()
   } else {
      true
   }
}

#[derive(PartialEq,Eq,Debug)]
pub enum EdgeKind {
   Tree,
   ForwardOrCross,
   Backward,
}

#[derive(PartialEq,Eq,Debug)]
pub enum VertexEvent {
   Discovered,
   Finished,
}

#[derive(PartialEq,Eq,Hash,Debug)]
pub enum VertexColor {
   White,
   Gray,
   Black,
}

pub fn depth_first_visit<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
   vertex_visitor: &mut FnMut(&G::Vertex, VertexEvent),
   edge_visitor: &mut FnMut(&G::Edge, EdgeKind),
   start: &G::Vertex,
   graph: &'a G,
) {
   let mut color = HashMap::new();

   for v in graph.vertices() {
      color.insert(v, VertexColor::White);
   }

   fn visit<'a, V, E, G: 'a + Graph<'a, V, E> + IncidenceGraph<'a, V, E> + VertexListGraph<'a, V, E>>(
      vx: &G::Vertex,
      color: &mut HashMap<G::Vertex, VertexColor>,
      vertex_visitor: &mut FnMut(&G::Vertex, VertexEvent),
      edge_visitor: &mut FnMut(&G::Edge, EdgeKind),
      graph: &'a G,
   ) {
      color.insert(vx.clone(), VertexColor::Gray);

      vertex_visitor(&vx, VertexEvent::Discovered);

      for e in graph.out_edges(vx.clone()) {
         let wx = graph.target(e);

         match color[&wx] {
            VertexColor::White => {
               edge_visitor(&e, EdgeKind::Tree);
               visit(&wx, color, vertex_visitor, edge_visitor, graph);
            }
            VertexColor::Gray => edge_visitor(&e, EdgeKind::Backward),
            VertexColor::Black => edge_visitor(&e, EdgeKind::ForwardOrCross),
         }
      }

      color.insert(vx.clone(), VertexColor::Black);
      vertex_visitor(&vx, VertexEvent::Finished);
   }

   visit::<V, E, G>(start, &mut color, vertex_visitor, edge_visitor, graph);
}

#[cfg(test)]
mod tests {
   use super::*;
   use adjacency_list::{AdjacencyList, AdjacencyListVertexDescriptor};
   use std::collections::HashMap;
   use traits::{Graph, MutableGraph};

   #[test]
   fn dfs_connected() {
      {
         let mut g = AdjacencyList::<usize, ()>::new();
         let vx1 = g.add_vertex(1);
         let vx2 = g.add_vertex(2);
         let vx3 = g.add_vertex(3);
         let vx4 = g.add_vertex(4);
         let vx5 = g.add_vertex(5);

         g.add_edge((), vx1, vx2);
         g.add_edge((), vx1, vx3);
         g.add_edge((), vx3, vx4);
         g.add_edge((), vx4, vx4);
         g.add_edge((), vx5, vx4);
         g.add_edge((), vx5, vx1);

         assert!(is_connected(&g));
      }

      {
         let g = AdjacencyList::<usize, ()>::new();
         assert!(is_connected(&g));
      }

      {
         let mut g = AdjacencyList::<usize, ()>::new();
         let vx1 = g.add_vertex(1);
         let vx2 = g.add_vertex(2);
         let vx3 = g.add_vertex(3);
         let vx4 = g.add_vertex(4);
         let vx5 = g.add_vertex(5);

         g.add_edge((), vx1, vx2);
         g.add_edge((), vx1, vx3);
         g.add_edge((), vx4, vx4);
         g.add_edge((), vx5, vx4);

         assert!(!is_connected(&g));
      }
   }

   #[test]
   fn dfs_visit() {
      let mut g = AdjacencyList::<usize, ()>::new();
      let vx1 = g.add_vertex(1);
      let vx2 = g.add_vertex(2);
      let vx3 = g.add_vertex(3);
      let vx4 = g.add_vertex(4);
      let vx5 = g.add_vertex(5);

      let e12 = g.add_edge((), vx1, vx2).unwrap();
      let e13 = g.add_edge((), vx1, vx3).unwrap();
      let e34 = g.add_edge((), vx3, vx4).unwrap();
      let e44 = g.add_edge((), vx4, vx4).unwrap();
      let e54 = g.add_edge((), vx5, vx4).unwrap();
      let e51 = g.add_edge((), vx5, vx1).unwrap();

      let mut vertices = HashMap::<AdjacencyListVertexDescriptor, bool>::new();

      depth_first_visit(
         &mut |vx: &AdjacencyListVertexDescriptor, ev| match ev {
                 VertexEvent::Discovered => assert!(vertices.insert(vx.clone(), false).is_none()),
                 VertexEvent::Finished => assert!(vertices.insert(vx.clone(), true) == Some(false)),
              },
         &mut |&ed, ev| if ed == e12 {
                 assert_eq!(ev, EdgeKind::Tree);
              } else if ed == e13 {
                 assert_eq!(ev, EdgeKind::Tree);
              } else if ed == e34 {
                 assert_eq!(ev, EdgeKind::Tree);
              } else if ed == e44 {
                 assert_eq!(ev, EdgeKind::Backward);
              } else if ed == e54 {
                 assert_eq!(ev, EdgeKind::ForwardOrCross);
              } else if ed == e51 {
                 assert_eq!(ev, EdgeKind::ForwardOrCross);
              } else {
                 unreachable!();
              },
         &vx1,
         &g,
      );
   }

   #[test]
   fn preorder() {
      let mut tree = AdjacencyList::<&'static str, ()>::new();
      let a = tree.add_vertex("a");
      let b = tree.add_vertex("b");
      let c = tree.add_vertex("c");
      let d = tree.add_vertex("d");
      let e = tree.add_vertex("e");
      let f = tree.add_vertex("f");
      let g = tree.add_vertex("g");
      let h = tree.add_vertex("h");
      let i = tree.add_vertex("i");

      tree.add_edge((), f, b);
      tree.add_edge((), b, a);
      tree.add_edge((), b, d);
      tree.add_edge((), d, c);
      tree.add_edge((), d, e);
      tree.add_edge((), f, g);
      tree.add_edge((), g, i);
      tree.add_edge((), i, h);

      let iter = TreeIterator::new(f, TraversalOrder::Preorder, &tree);
      let preorder = iter.map(|vx| tree.vertex_label(vx).unwrap().to_string()).collect::<Vec<_>>();
      let expect = vec!["f", "b", "a", "d", "c", "e", "g", "i", "h"].iter().map(|x| x.to_string()).collect::<Vec<_>>();

      assert_eq!(preorder, expect);
   }

   #[test]
   fn postorder() {
      let mut tree = AdjacencyList::<&'static str, ()>::new();
      let a = tree.add_vertex("a");
      let b = tree.add_vertex("b");
      let c = tree.add_vertex("c");
      let d = tree.add_vertex("d");
      let e = tree.add_vertex("e");
      let f = tree.add_vertex("f");
      let g = tree.add_vertex("g");
      let h = tree.add_vertex("h");
      let i = tree.add_vertex("i");

      tree.add_edge((), f, b);
      tree.add_edge((), b, a);
      tree.add_edge((), b, d);
      tree.add_edge((), d, c);
      tree.add_edge((), d, e);
      tree.add_edge((), f, g);
      tree.add_edge((), g, i);
      tree.add_edge((), i, h);

      let iter = TreeIterator::new(f, TraversalOrder::Postorder, &tree);
      let preorder = iter.map(|vx| tree.vertex_label(vx).unwrap().to_string()).collect::<Vec<_>>();
      let expect = vec!["a", "c", "e", "d", "b", "h", "i", "g", "f"].iter().map(|x| x.to_string()).collect::<Vec<_>>();

      assert_eq!(preorder, expect);
   }
}
