/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015,2017  Panopticon authors
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

use errors::Error;

use panopticon_graph_algos::{AdjacencyList, BidirectionalGraphTrait, EdgeListGraphTrait, GraphTrait, IncidenceGraphTrait, MutableGraphTrait,
                             VertexListGraphTrait};
use panopticon_graph_algos::adjacency_list::{AdjacencyListEdgeDescriptor, AdjacencyListVertexDescriptor};

use panopticon_graph_algos::search::is_connected;
use std::{f32, isize, usize};
use std::cmp::{Ordering, max, min};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use sugiyama::order::{crossings, initial_ordering, optimize_ordering_once};

use sugiyama::rank::{add_virtual_vertices, compute_ranking, ensure_single_entry, normalize_rank, remove_cycles, remove_loops, remove_parallel_edges};

fn partial_max(a: f32, b: f32) -> f32 {
   if a < b { b } else { a }
}

fn partial_min(a: f32, b: f32) -> f32 {
   if a > b { b } else { a }
}

#[derive(Clone)]
pub enum LinearLayout {
   Cooked {
      graph: AdjacencyList<usize, usize>,
      rev: HashMap<usize, AdjacencyListVertexDescriptor>,
      head: AdjacencyListVertexDescriptor,
      revd_edge_labels: HashSet<usize>,
      revd_parallel_edges: Vec<(usize, AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)>,
   },
   Ranked {
      graph: AdjacencyList<usize, usize>,
      rev: HashMap<usize, AdjacencyListVertexDescriptor>,
      head: AdjacencyListVertexDescriptor,
      virt_start: usize,
      rank: HashMap<AdjacencyListVertexDescriptor, isize>,
      revd_edge_labels: HashSet<usize>,
   },
   Ordering {
      iterations_left: usize,
      graph: AdjacencyList<usize, usize>,
      rev: HashMap<usize, AdjacencyListVertexDescriptor>,
      head: AdjacencyListVertexDescriptor,
      virt_start: usize,
      revd_edge_labels: HashSet<usize>,
      bipartite: HashMap<(usize, usize), Vec<(AdjacencyListEdgeDescriptor, AdjacencyListEdgeDescriptor)>>,
      order: Vec<Vec<AdjacencyListVertexDescriptor>>,
      rank: HashMap<AdjacencyListVertexDescriptor, isize>,
      xings: usize,
   },
}

#[cfg(test)]
pub fn linear_layout(
   vertices: &Vec<usize>,
   edges: &Vec<(usize, usize)>,
   dims: &HashMap<usize, (f32, f32)>,
   entry: Option<usize>,
   node_spacing: f32,
   rank_spacing: f32,
   port_spacing: f32,
   loop_spacing: f32,
   entry_spacing: f32,
   block_spacing: f32,
) -> Result<(HashMap<usize, (f32, f32)>, HashMap<usize, (Vec<(f32, f32, f32, f32)>, (f32, f32), (f32, f32))>), Error> {
   let layout = linear_layout_structural(vertices, edges, entry)?;
   linear_layout_placement(
      vertices,
      edges,
      &layout,
      dims,
      node_spacing,
      rank_spacing,
      port_spacing,
      loop_spacing,
      entry_spacing,
      block_spacing,
   )
}
pub fn linear_layout_start(vertices: &Vec<usize>, edges: &Vec<(usize, usize)>, entry: Option<usize>) -> Result<LinearLayout, Error> {
   let mut graph = AdjacencyList::<usize, usize>::new();
   let mut rev = HashMap::<usize, AdjacencyListVertexDescriptor>::new();
   let mut maybe_entry = None;

   for &n in vertices.iter() {
      let vx = graph.add_vertex(n);

      rev.insert(n, vx);
      if entry == Some(n) {
         maybe_entry = Some(rev[&n].clone());
      }
   }

   for (idx, e) in edges.iter().enumerate() {
      graph.add_edge(idx, rev[&e.0], rev[&e.1]);
   }

   if !is_connected(&graph) {
      return Err("Input graph is not connected".into());
   }

   if vertices.is_empty() {
      return Err("Input graph is empty".into());
   }

   // normalize graph to DAG with single entry "head"
   let head = ensure_single_entry(maybe_entry.as_ref(), &mut graph);
   let revd_edge_labels = remove_cycles(&head, &mut graph);
   remove_loops(&mut graph);
   let revd_parallel_edges = remove_parallel_edges(&mut graph);

   Ok(
      LinearLayout::Cooked {
         graph: graph,
         rev: rev,
         head: head,
         revd_parallel_edges: revd_parallel_edges,
         revd_edge_labels: revd_edge_labels,
      }
   )
}

pub fn linear_layout_rank(layout: LinearLayout) -> Result<LinearLayout, Error> {
   if let LinearLayout::Cooked { mut graph, rev, head, revd_parallel_edges, revd_edge_labels } = layout {
      // Desc -> Rank
      let mut rank = compute_ranking(&graph);

      // restore parallel edges
      for e in revd_parallel_edges {
         graph.add_edge(e.0, e.1, e.2);
      }

      if rank.len() != graph.num_vertices() {
         return Err("Internal error while ranking".into());
      }

      // split edges spanning multiple ranks
      let (virt_start, mut next_virt) = add_virtual_vertices(&mut rank, &mut graph);
      assert!(virt_start <= next_virt);

      // add vertices for edges going from higher to lower ranks
      let to_extend = graph
         .edges()
         .filter(
            |&e| {
               let slb = *graph.vertex_label(graph.source(e)).unwrap();
               let tlb = *graph.vertex_label(graph.target(e)).unwrap();
               let elb = &graph.edge_label(e).unwrap();

               revd_edge_labels.contains(&elb) && (slb < virt_start || tlb < virt_start)
            }
         )
         .collect::<Vec<_>>();
      for e in to_extend {
         let s = graph.source(e);
         let t = graph.target(e);
         let lb = *graph.edge_label(e).unwrap();
         let s_lb = *graph.vertex_label(s).unwrap();
         let t_lb = *graph.vertex_label(t).unwrap();

         match (s_lb < virt_start, t_lb < virt_start) {
            (true, false) | (false, true) => {
               let v = graph.add_vertex(next_virt);
               let v_rank = if s_lb < virt_start {
                  rank[&s]
               } else {
                  rank[&t]
               };

               next_virt += 1;
               graph.remove_edge(e);
               graph.add_edge(lb, s, v);
               graph.add_edge(lb, v, t);
               rank.insert(v, v_rank);
            }
            (true, true) => {
               let vs = graph.add_vertex(next_virt);
               let vt = graph.add_vertex(next_virt + 1);
               let vs_rank = rank[&s];
               let vt_rank = rank[&t];

               next_virt += 2;
               graph.remove_edge(e);
               graph.add_edge(lb, s, vs);
               graph.add_edge(lb, vs, vt);
               graph.add_edge(lb, vt, t);
               rank.insert(vs, vs_rank);
               rank.insert(vt, vt_rank);
            }
            (false, false) => return Err("Internal error while edge inverting".into()),
         }
      }

      if rank.len() != graph.num_vertices() {
         return Err("Internal error after edge inverting".into());
      }

      normalize_rank(&mut rank);

      if rank.len() != graph.num_vertices() {
         return Err("Internal error after normalization".into());
      }

      for e in graph.edges() {
         let from = graph.source(e);
         let to = graph.target(e);

         if !(rank[&from] + 1 == rank[&to] || rank[&from] == rank[&to]) {
            return Err("Internal error after normalization".into());
         }
      }

      Ok(
         LinearLayout::Ranked {
            graph: graph,
            rev: rev,
            head: head,
            virt_start: virt_start,
            rank: rank,
            revd_edge_labels: revd_edge_labels,
         }
      )
   } else {
      Err("Wrong state. Expected Cooked.".into())
   }
}

pub fn linear_layout_initial_order(layout: LinearLayout) -> Result<LinearLayout, Error> {
   if let LinearLayout::Ranked { graph, rev, head, virt_start, rank, revd_edge_labels } = layout {
      // logical intra-rank ordering
      let (order, bipartite) = initial_ordering(&rank, &head, &graph);

      if !(order[0].len() == 1 || order[0][0] != order[0][1]) {
         Err("Internal error after initial ordering".into())
      } else {
         let xings = crossings(&bipartite, &order, &graph);

         Ok(
            LinearLayout::Ordering {
               iterations_left: 6,
               graph: graph,
               rev: rev,
               head: head,
               virt_start: virt_start,
               revd_edge_labels: revd_edge_labels,
               bipartite: bipartite,
               order: order,
               rank: rank,
               xings: xings,
            }
         )
      }
   } else {
      Err("Wrong state. Expected Ranked.".into())
   }
}

pub fn linear_layout_order(mut layout: LinearLayout) -> Result<LinearLayout, Error> {
   match layout {
      LinearLayout::Ordering { iterations_left: 0, .. } => {}
      LinearLayout::Ordering {
         iterations_left: ref mut iters,
         ref mut order,
         ref rank,
         ref graph,
         ref bipartite,
         ref mut xings,
         ..
      } => {
         optimize_ordering_once(*iters, bipartite, xings, order, rank, graph);
         if *xings == 0 {
            *iters = 0;
         } else {
            *iters -= 1;
         }
      }
      _ => return Err("Wrong state. Expected Ordering.".into()),
   }

   Ok(layout)
}

#[cfg(test)]
pub fn linear_layout_structural(vertices: &Vec<usize>, edges: &Vec<(usize, usize)>, entry: Option<usize>) -> Result<LinearLayout, Error> {
   let cooked = linear_layout_start(vertices, edges, entry)?;
   let ranked = linear_layout_rank(cooked)?;
   let mut ordered = linear_layout_initial_order(ranked)?;

   loop {
      let cont = if let LinearLayout::Ordering { iterations_left, .. } = ordered {
         iterations_left > 0
      } else {
         return Err("Wrong state. Expected Ordering.".into());
      };

      if cont {
         ordered = linear_layout_order(ordered)?;
      } else {
         break;
      }
   }

   Ok(ordered)
}

pub fn linear_layout_placement(
   vertices: &Vec<usize>,
   edges: &Vec<(usize, usize)>,
   layout: &LinearLayout,
   dims: &HashMap<usize, (f32, f32)>,
   node_spacing: f32,
   rank_spacing: f32,
   port_spacing: f32,
   loop_spacing: f32,
   entry_spacing: f32,
   block_spacing: f32,
) -> Result<(HashMap<usize, (f32, f32)>, HashMap<usize, (Vec<(f32, f32, f32, f32)>, (f32, f32), (f32, f32))>), Error> {
   if let &LinearLayout::Ordering {
             ref order,
             ref rank,
             ref graph,
             ref rev,
             virt_start,
             ref revd_edge_labels,
             ..
          } = layout {
      let mut graph = graph.clone();
      let rank_sep = rank_spacing + block_spacing + entry_spacing;
      let dims = HashMap::from_iter(graph.vertices().map(|vx| (vx, graph.vertex_label(vx).and_then(|x| dims.get(x).cloned()).unwrap_or((0., 0.)))));

      // (source x offset,target x offset)
      let mut x_off = HashMap::<AdjacencyListEdgeDescriptor, (f32, f32)>::from_iter(graph.edges().map(|e| (e, (0., 0.))));

      // offset initial and final edge segments to form node ports
      for n in vertices.iter() {
         let vx = rev[n];
         let mut up = graph
            .in_edges(vx)
            .filter_map(
               |x| if graph.source(x) != vx {
                  Some((x, graph.source(x)))
               } else {
                  None
               }
            )
            .collect::<Vec<_>>();
         let mut down = graph
            .out_edges(vx)
            .filter_map(
               |x| if graph.target(x) != vx {
                  Some((x, graph.target(x)))
               } else {
                  None
               }
            )
            .collect::<Vec<_>>();
         let sort_by_order =
            |a: &(AdjacencyListEdgeDescriptor, AdjacencyListVertexDescriptor), b: &(AdjacencyListEdgeDescriptor, AdjacencyListVertexDescriptor)| -> Ordering {
               let rank_a = rank[&a.1] as usize;
               let rank_b = rank[&b.1] as usize;
               let maybe_ord_a = order[rank_a].iter().position(|&x| x == a.1);
               let maybe_ord_b = order[rank_b].iter().position(|&x| x == b.1);

               if a.1 == vx {
                  Ordering::Less
               } else if b.1 == vx {
                  Ordering::Greater
               } else {
                  maybe_ord_b.partial_cmp(&maybe_ord_a).unwrap_or(Ordering::Equal)
               }
            };

         up.sort_by(&sort_by_order);
         down.sort_by(&sort_by_order);

         if up.len() > 1 {
            let mut off = -1.0 * ((up.len() - 1) as f32) * (port_spacing as f32) / 2.0;
            for w in up.iter() {
               x_off.entry(w.0).or_insert((0., 0.)).1 = off;
               off += port_spacing as f32;
            }
         }

         if down.len() > 1 {
            let mut off = -1.0 * ((down.len() - 1) as f32) * (port_spacing as f32) / 2.0;
            for w in down.iter() {
               x_off.entry(w.0).or_insert((0., 0.)).0 = off;
               off += port_spacing as f32;
            }
         }
      }

      // intra-rank positions
      let x_pos = compute_x_coordinates(
         order,
         rank,
         &graph,
         &dims,
         &x_off,
         &|_| node_spacing,
         virt_start,
      );

      let rank_offsets = order
         .iter()
         .map(
            |r| {
               r.iter()
                  .fold(
                     0., |acc, vx| {
                        let h = dims
                           .get(vx)
                           .map(
                              |x| {
                                 assert!(x.1 >= 0.0);
                                 x.1
                              }
                           )
                           .unwrap_or(0.);
                        partial_max(h, acc)
                     }
                  )
            }
         )
         .fold(
            vec![0.], |acc, x| {
               let mut ret = acc.clone();
               ret.push(acc.last().unwrap() + x + (2. * rank_sep));
               ret
            }
         );

      // restore reversed edges
      let mut revd_edges = vec![];
      for e in graph.edges() {
         if revd_edge_labels.contains(graph.edge_label(e).unwrap()) {
            revd_edges.push(e);
         }
      }
      for e in revd_edges {
         graph.remove_edge(e);
      }

      // position original vertices (basic blocks)
      let mut ret_v = HashMap::new();
      for n in vertices.iter() {
         let vx = rev[n];

         if !(rank[&vx] >= 0) {
            return Err("Internal error".into());
         }

         let r = rank[&vx] as usize;
         let rank_start = rank_offsets[r];
         let rank_end = rank_offsets[r + 1] - rank_sep;

         ret_v.insert(*n, (x_pos[&vx], rank_start + (rank_end - rank_start) / 2.0));
      }

      // build end point list (edge label -> (start,end))
      let mut end_points = HashMap::<usize, (AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)>::new();
      for vx in graph.vertices() {
         if *graph.vertex_label(vx).unwrap() < virt_start {
            for out in graph.out_edges(vx) {
               end_points.entry(*graph.edge_label(out).unwrap()).or_insert((vx, vx)).0 = vx;
            }
            for _in in graph.in_edges(vx) {
               end_points.entry(*graph.edge_label(_in).unwrap()).or_insert((vx, vx)).1 = vx;
            }
         }
      }

      // edge label -> (high pos,low pos)
      let mut revd_edge_pos = HashMap::<usize, (f32, f32)>::new();
      for lb in revd_edge_labels.iter() {
         if let Some(&(start, end)) = end_points.get(lb) {
            let start_r = rank[&start] as usize;
            let end_r = rank[&end] as usize;
            let post_start = graph.target(graph.out_edges(start).find(|&f| *graph.edge_label(f).unwrap() == *lb).unwrap());
            let pre_end = graph.source(graph.in_edges(end).find(|&f| *graph.edge_label(f).unwrap() == *lb).unwrap());

            let ps_pos = order[start_r].iter().position(|&x| start == x).unwrap();
            let s_pos = order[start_r].iter().position(|&x| post_start == x).unwrap();
            let s = order[start_r]
               .iter()
               .skip(min(ps_pos, s_pos))
               .take(max(ps_pos, s_pos) - min(ps_pos, s_pos))
               .fold(
                  0.0, |acc, x| if acc < dims.get(x).unwrap_or(&(0., 0.)).1 {
                     dims.get(x).unwrap_or(&(0., 0.)).1
                  } else {
                     acc
                  }
               );

            let pe_pos = order[end_r].iter().position(|&x| end == x).unwrap();
            let e_pos = order[end_r].iter().position(|&x| pre_end == x).unwrap();
            let e = order[end_r]
               .iter()
               .skip(min(pe_pos, e_pos))
               .take(max(pe_pos, e_pos) - min(pe_pos, e_pos))
               .fold(
                  0.0, |acc, x| if acc < dims.get(x).unwrap_or(&(0., 0.)).1 {
                     dims.get(x).unwrap_or(&(0., 0.)).1
                  } else {
                     acc
                  }
               );

            revd_edge_pos.insert(*lb, (s, e));
         }
      }

      // build edge list
      let mut ret_e = HashMap::<usize, (Vec<(f32, f32, f32, f32)>, (f32, f32), (f32, f32))>::new();
      for (idx, _e) in edges.iter().enumerate() {
         let start = rev[&_e.0];
         let end = rev[&_e.1];
         let maybe_e = graph.out_edges(start).chain(graph.out_edges(end)).find(|x| *graph.edge_label(*x).unwrap() == idx);
         let mut ret = vec![];
         let mut start_arrow_off = None;
         let mut end_arrow_off = None;

         match maybe_e {
            None => {
               // loops
               let (w, h) = *dims.get(&start).unwrap_or(&(0., 0.));
               let xr = x_pos[&start] - w / 2. + loop_spacing; // right x
               let xl = x_pos[&start] - w / 2. - loop_spacing; // left x
               let r = rank[&start] as usize; // rank
               let rs = rank_offsets[r]; // rank top y
               let re = rank_offsets[r + 1] - rank_sep; // rank + 1 top y
               let rm = rs + (re - rs) / 2.; // rank center y
               let bes = rm + h / 2. + block_spacing; // edge start x
               let bee = rm + h / 2. + block_spacing + entry_spacing; // edge entry part end x
               let tee = rm - h / 2. - block_spacing; // edge start x
               let tes = rm - h / 2. - block_spacing - entry_spacing; // edge entry part end x
               let segs = vec![
                  (xr, bes, xr, bee),
                  (xr, bee, xl, bee),
                  (xl, bee, xl, tes),
                  (xl, tes, xr, tes),
                  (xr, tes, xr, tee),
               ];
               let start_arrow_off = (xr, bes + (bee - bes) / 2.);
               let end_arrow_off = (xr, tee);

               ret_e.insert(idx, (segs, start_arrow_off, end_arrow_off));
            }
            Some(mut e) => {

               loop {
                  let s = graph.source(e);
                  let t = graph.target(e);
                  let lb = graph.edge_label(e).unwrap();
                  let sx = x_pos[&s] + x_off.get(&e).map(|x| x.0).unwrap_or(0.0); // source center x
                  let tx = x_pos[&t] + x_off.get(&e).map(|x| x.1).unwrap_or(0.0); // target center x
                  let sr = rank[&s] as usize; // source rank
                  let tr = rank[&t] as usize; // target rank
                  let srs = rank_offsets[sr]; // source rank top y
                  let sre = rank_offsets[sr + 1] - rank_sep; // source rank + 1 top y
                  let srm = srs + (sre - srs) / 2.; // source rank center y
                  let ses = srm + dims.get(&s).map(|x| x.1).unwrap_or(0.) / 2. + block_spacing; // edge start x
                  let see = srm + dims.get(&s).map(|x| x.1).unwrap_or(0.) / 2. + block_spacing + entry_spacing; // edge entry part end x
                  let trs = rank_offsets[tr];
                  let tre = rank_offsets[tr + 1] - rank_sep;
                  let trm = trs + (tre - trs) / 2.;
                  let tee = trm - dims.get(&t).map(|x| x.1).unwrap_or(0.) / 2. - block_spacing - entry_spacing;
                  let tes = trm - dims.get(&t).map(|x| x.1).unwrap_or(0.) / 2. - block_spacing;
                  let mx = sx + (tx - sx) / 2.;
                  let my = sre + (trs - sre) / 2.;

                  // arrow tail position
                  if start_arrow_off.is_none() {
                     start_arrow_off = Some((sx, ses + (see - ses) / 2.));
                  }

                  if revd_edge_labels.contains(&lb) {
                     unreachable!();
                     /*
                            // back edges
                            if *graph.vertex_label(s).unwrap() < virt_start {
                            ret.push((sx,srs + (sre - srs) / 2.0 + revd_edge_pos[lb].0 / 2.0 + port_spacing as f32,sx,srs + (sre - srs) / 2.0));
                            } else {
                            if sr == tr {
                            let y = srs + (sre - srs) / 2.0 - revd_edge_pos[lb].1 / 2.0 - port_spacing as f32;
                            ret.push((sx,srs + (sre - srs) / 2.0,sx,y));
                            ret.push((sx,y,tx,y));
                            } else {
                            ret.push((sx,srs + (sre - srs) / 2.0,mx,my));
                            }
                            }

                            if *graph.vertex_label(t).unwrap() < virt_start {
                            ret.push((tx,trs + (tre - trs) / 2.0 - revd_edge_pos[lb].1 / 2.0 - port_spacing as f32,tx,trs + (tre - trs) / 2.0));
                            } else {
                            if sr == tr {
                            let y = trs + (tre - trs) / 2.0 + revd_edge_pos[lb].0 / 2.0 + port_spacing as f32;
                            ret.push((tx,trs + (tre - trs) / 2.0,tx,y));
                            ret.push((tx,y,sx,y));
                            } else {
                            ret.push((mx,my,tx,trs + (tre - trs) / 2.0));
                            }
                            }*/
                  } else {
                     // forward edges
                     if *graph.vertex_label(s).unwrap() < virt_start {
                        ret.push((sx, ses, sx, see));
                        //ret.push((sx,see,sx,sre));
                        ret.push((sx, see, mx, my));
                     } else {
                        ret.push((sx, ses, sx, see));
                        ret.push((sx, see, mx, my));
                     }

                     if *graph.vertex_label(t).unwrap() < virt_start {
                        ret.push((mx, my, tx, tee));
                        //ret.push((tx,trs,tx,tee));
                        ret.push((tx, tee, tx, tes));
                     } else {
                        ret.push((mx, my, tx, tee));
                        ret.push((tx, tee, tx, tes));
                     }
                  }

                  // next segment
                  match graph.out_edges(t).find(|x| *graph.edge_label(*x).unwrap() == idx) {
                     Some(_e) => e = _e,
                     None => {
                        if !(end_arrow_off.is_none()) {
                           return Err("Internal error while final edge routing".into());
                        }

                        end_arrow_off = Some((tx, tes));
                        break;
                     }
                  }
               }

               ret_e.insert(idx, (ret, start_arrow_off.unwrap(), end_arrow_off.unwrap()));
            }
         }
      }

      Ok((ret_v, ret_e))
   } else {
      Err("Wrong state. Expected Ordering.".into())
   }
}

pub fn mark_type1_conflicts(
   virt_start: usize,
   order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
   rank: &HashMap<AdjacencyListVertexDescriptor, isize>,
   graph: &AdjacencyList<usize, usize>,
) -> Vec<(AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)> {

   let mut ret = Vec::<(AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)>::new();

   if order.len() < 3 {
      return ret;
   }

   let (from, to, delta) = (1, order.len() as isize - 2, 1);

   let mut i = from;
   while i != to {
      assert!(i + delta >= 0 && i + delta < order.len() as isize);

      let mut k0 = 0;
      let mut l = 0;
      let next = (i + delta) as usize;

      for l1 in 0..order[next].len() {
         let v = order[next][l1];
         let mut edges = graph.in_edges(v);
         let maybe_upn = edges.find(
            |&e| {
               let w = graph.source(e);
               rank[&w] == i as isize && *graph.vertex_label(w).unwrap() >= virt_start && *graph.vertex_label(v).unwrap() >= virt_start
            }
         );

         if l1 == order[next].len() || maybe_upn.is_some() {
            assert!(i >= 0);

            let k1 = if let Some(upn) = maybe_upn {
               order[i as usize].iter().position(|&x| (x == graph.source(upn))).unwrap()
            } else {
               order[i as usize].len() - 1
            };

            while l <= l1 {
               let edges = graph.in_edges(v);
               for e in edges {
                  let w = graph.source(e);

                  if rank[&w] == i && *graph.vertex_label(w).unwrap() >= virt_start {
                     assert!(i >= 0);
                     let k = order[i as usize].iter().position(|&x| x == w).unwrap();

                     if k < k0 || k > k1 {
                        ret.push((w, graph.target(e)));
                     }
                  }
               }

               l += 1;
            }

            k0 = k1;
         }
      }

      i += delta;
   }
   ret
}

pub fn vertical_alignment
   (
   order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
   type1: &Vec<(AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)>,
   graph: &AdjacencyList<usize, usize>,
   up_to_down: bool,
   left_to_right: bool,
) -> (HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>, HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>) {
   let mut root = HashMap::<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>::new();
   let mut align = HashMap::<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>::new();

   for v in graph.vertices() {
      root.insert(v, v);
      align.insert(v, v);
   }

   let (from, to, delta) = if up_to_down {
      (1, order.len() as isize, 1)
   } else {
      (order.len() as isize - 2, -1, -1isize)
   };

   let mut i = from;
   while i != to {
      let mut r = if left_to_right { -1 } else { isize::MAX };
      let (lev_from, lev_to, lev_delta) = if left_to_right {
         (0, order[i as usize].len() as isize, 1isize)
      } else {
         (order[i as usize].len() as isize - 1, -1isize, -1isize)
      };
      let mut k = lev_from;

      while k != lev_to {
         assert!(i - delta >= 0 && i - delta < order.len() as isize && i >= 0 && k >= 0);

         let prev = (i - delta) as usize;
         let v = order[i as usize][k as usize];
         let upn = if up_to_down {
            order[prev].iter().filter(|&&w| graph.out_edges(w).any(|e| graph.target(e) == v)).cloned().collect::<Vec<_>>()
         } else {
            order[prev].iter().filter(|&&w| graph.in_edges(w).any(|e| graph.source(e) == v)).cloned().collect::<Vec<_>>()
         };

         if upn.len() > 0 {
            let medians = if upn.len() % 2 == 0 {
               vec![((upn.len() - 1) as f32 / 2.).floor(), ((upn.len() - 1) as f32 / 2.).ceil()]
            } else {
               vec![(upn.len() - 1) as f32 / 2.]
            };

            for m in medians {
               if align[&v] == v {
                  assert!(m >= 0.0);
                  let um = upn[m as usize];
                  let pos = order[prev].iter().position(|&x| x == um).unwrap() as isize;

                  if !type1.contains(&(um, v)) && !type1.contains(&(v, um)) && ((left_to_right && r < pos) || (!left_to_right && r > pos)) {
                     align.insert(um, v);

                     let a = root[&um];
                     root.insert(v, a);

                     let b = root[&v];
                     align.insert(v, b);
                     r = pos;

                  }
               }
            }
         }

         k += lev_delta;
      }

      i += delta;
   }

   (root, align)
}

fn calculate_threshold(
   v: AdjacencyListVertexDescriptor,
   w: AdjacencyListVertexDescriptor,
   old_threshold: f32,
   _: bool,
   queue: &mut Vec<(AdjacencyListVertexDescriptor, AdjacencyListEdgeDescriptor)>,
   placed: &HashSet<AdjacencyListVertexDescriptor>,
   x: &HashMap<AdjacencyListVertexDescriptor, f32>,
   port_offset: &HashMap<AdjacencyListEdgeDescriptor, (f32, f32)>,
   align: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   root: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   inner_shift: &HashMap<AdjacencyListVertexDescriptor, f32>,
   graph: &AdjacencyList<usize, usize>,
) -> f32 {
   use std::f32;
   let mut threshold = old_threshold;

   if v == w {
      // pick incoming edge to straighten (p,q)
      if let Some(e) = graph.in_edges(w).next() {
         let p = graph.source(e);
         let q = graph.target(e);
         let r = root[&p];
         if placed.contains(&r) {
            threshold = x[&r] + inner_shift[&p] + port_offset.get(&e).map(|x| x.0).unwrap_or(0.) - inner_shift[&q] -
                        port_offset.get(&e).map(|x| x.1).unwrap_or(0.);
         } else {
            queue.push((w, e));
         }
      }
   }

   if threshold <= f32::MIN && align[&w] == v {
      // pick outgoing edge to straighten (p,q)
      if let Some(e) = graph.out_edges(w).next() {
         let p = graph.source(e);
         let q = graph.target(e);
         let r = root[&p];
         if placed.contains(&r) {
            threshold = x[&r] + inner_shift[&p] + port_offset.get(&e).map(|x| x.0).unwrap_or(0.) - inner_shift[&q] -
                        port_offset.get(&e).map(|x| x.1).unwrap_or(0.);
         } else {
            queue.push((w, e));
         }
      }
   }

   threshold
}

fn place_block(
   v: AdjacencyListVertexDescriptor,
   order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
   rank: &HashMap<AdjacencyListVertexDescriptor, isize>,
   sink: &mut HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   shift: &mut HashMap<AdjacencyListVertexDescriptor, f32>,
   x: &mut HashMap<AdjacencyListVertexDescriptor, f32>,
   port_offset: &HashMap<AdjacencyListEdgeDescriptor, (f32, f32)>,
   align: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   root: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   dims: &HashMap<AdjacencyListVertexDescriptor, (f32, f32)>,
   inner_shift: &HashMap<AdjacencyListVertexDescriptor, f32>,
   placed: &mut HashSet<AdjacencyListVertexDescriptor>,
   queue: &mut Vec<(AdjacencyListVertexDescriptor, AdjacencyListEdgeDescriptor)>,
   graph: &AdjacencyList<usize, usize>,
   node_spacing: &Fn(usize) -> f32,
   left_to_right: bool,
) {
   use std::f32;

   let delta: isize = if left_to_right { 1 } else { -1isize };

   if !x.contains_key(&v) {
      x.insert(v, 0.);
      let mut initial = true;
      let mut w = v;
      let mut threshold = if !left_to_right { f32::MIN } else { f32::MAX };

      loop {
         assert!(rank[&w] >= 0);
         let word: &Vec<AdjacencyListVertexDescriptor> = &order[rank[&w] as usize];
         let pos = word.iter().position(|&x| x == w).unwrap() as isize;

         if (left_to_right && *word.first().unwrap() != w) || (!left_to_right && *word.last().unwrap() != w) {

            let pred = pos - delta;
            assert!(pred >= 0);
            let n = word[pred as usize];
            let u = root[&n];

            place_block(
               u,
               order,
               rank,
               sink,
               shift,
               x,
               port_offset,
               align,
               root,
               dims,
               inner_shift,
               placed,
               queue,
               graph,
               node_spacing,
               left_to_right,
            );

            threshold = calculate_threshold(
               v,
               w,
               threshold,
               left_to_right,
               queue,
               placed,
               x,
               port_offset,
               align,
               root,
               inner_shift,
               graph,
            );

            if sink[&v] == v {
               let t = sink[&u];
               sink.insert(v, t);
            }

            let spacing = node_spacing(rank[&v] as usize);

            if sink[&v] != sink[&u] {
               let sinku = sink[&u].clone();
               let prev = shift[&sinku];

               if !left_to_right {
                  shift.insert(
                     sinku.clone(),
                     partial_min(
                        prev,
                        x[&v] + inner_shift[&w] - x[&u] - inner_shift[&n] - dims.get(&n).unwrap_or(&(0., 0.)).0 - spacing,
                     ),
                  );
               } else {
                  shift.insert(
                     sinku.clone(),
                     partial_max(
                        prev,
                        x[&v] + inner_shift[&w] - (x[&u] + inner_shift[&n]) + dims.get(&n).unwrap_or(&(0., 0.)).0 + spacing,
                     ),
                  );
               }
            } else {
               let sb = if !left_to_right {
                  partial_max(
                     threshold,
                     x[&u] + inner_shift[&n] + dims.get(&n).unwrap_or(&(0., 0.)).0 - inner_shift[&w] + spacing,
                  )
               } else {
                  partial_min(
                     threshold,
                     x[&u] + inner_shift[&n] - dims.get(&n).unwrap_or(&(0., 0.)).0 - inner_shift[&w] - spacing,
                  )
               };

               if initial {
                  x.insert(v, sb);
                  initial = false;
               } else {
                  let xv = x[&v];
                  if !left_to_right {
                     x.insert(v, partial_max(xv, sb));
                  } else {
                     x.insert(v, partial_min(xv, sb));
                  }
               }
            }
         } else {
            threshold = calculate_threshold(
               v,
               w,
               threshold,
               left_to_right,
               queue,
               placed,
               x,
               port_offset,
               align,
               root,
               inner_shift,
               graph,
            );
         }


         assert!(w != align[&w] || align[&w] == v);
         w = align[&w];

         if w == v {
            break;
         }
      }
   }

   placed.insert(v);
}

fn post_process(
   x: &mut HashMap<AdjacencyListVertexDescriptor, f32>,
   port_offset: &HashMap<AdjacencyListEdgeDescriptor, (f32, f32)>,
   align: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   root: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   inner_shift: &HashMap<AdjacencyListVertexDescriptor, f32>,
   queue: &Vec<(AdjacencyListVertexDescriptor, AdjacencyListEdgeDescriptor)>,
   graph: &AdjacencyList<usize, usize>,
   left_to_right: bool,
) {

   for &(vx, e) in queue {
      let p = graph.source(e);
      let q = graph.target(e);
      let rp = root[&p];
      let rq = root[&q];
      let t = x[&rp] + inner_shift[&p] + port_offset.get(&e).map(|x| x.0).unwrap_or(0.) - x[&rq] + inner_shift[&q] +
              port_offset.get(&e).map(|x| x.1).unwrap_or(0.);

      if (t > 0. && left_to_right) || (t < 0. && !left_to_right) {
         let r = root[&vx];
         let mut v = r;
         loop {
            let new_x = x[&v] + t * if !left_to_right { -1. } else { 1. };

            x.insert(v, new_x);
            v = align[&v];
            if v == r {
               break;
            }
         }
      }
   }
}

fn horizontal_compaction(
   order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
   rank: &HashMap<AdjacencyListVertexDescriptor, isize>,
   graph: &AdjacencyList<usize, usize>,
   align: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   root: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   dims: &HashMap<AdjacencyListVertexDescriptor, (f32, f32)>,
   port_offset: &HashMap<AdjacencyListEdgeDescriptor, (f32, f32)>,
   inner_shift: &HashMap<AdjacencyListVertexDescriptor, f32>,
   node_spacing: &Fn(usize) -> f32,
   up_to_down: bool,
   left_to_right: bool,
) -> HashMap<AdjacencyListVertexDescriptor, f32> {

   let mut sink = HashMap::<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>::new();
   let mut shift = HashMap::<AdjacencyListVertexDescriptor, f32>::new();
   let mut x = HashMap::<AdjacencyListVertexDescriptor, f32>::new();
   let mut placed = HashSet::<AdjacencyListVertexDescriptor>::new();
   let mut queue = vec![];

   for v in graph.vertices() {
      sink.insert(v, v);
      if !left_to_right {
         shift.insert(v, f32::MAX);
      } else {
         shift.insert(v, f32::MIN);
      }
   }

   let (vfrom, vto, vdelta) = if up_to_down {
      (0, order.len() as isize, 1isize)
   } else {
      (order.len() as isize - 1, -1, -1isize)
   };

   let mut vi = vfrom;
   while vi != vto {
      assert!(vi >= 0);

      let (hfrom, hto, hdelta) = if left_to_right {
         (0, order[vi as usize].len() as isize, 1isize)
      } else {
         (order[vi as usize].len() as isize - 1, -1, -1isize)
      };

      let mut hi = hfrom;
      while hi != hto {
         let v = order[vi as usize][hi as usize];

         if root[&v] == v {
            place_block(
               v,
               order,
               rank,
               &mut sink,
               &mut shift,
               &mut x,
               port_offset,
               &align,
               &root,
               dims,
               inner_shift,
               &mut placed,
               &mut queue,
               graph,
               node_spacing,
               left_to_right,
            );
         }

         hi += hdelta;
      }
      vi += vdelta;
   }

   vi = vfrom;
   let mut d = 0.;
   while vi != vto {
      assert!(vi >= 0);

      let v = *if left_to_right {
                  order[vi as usize].first().unwrap()
               } else {
                  order[vi as usize].last().unwrap()
               };

      if v == sink[&root[&v]] {
         let old_shift = shift[&v];
         if (!left_to_right && old_shift < f32::MAX) || (left_to_right && old_shift > f32::MIN) {
            shift.insert(v, old_shift + d);
            d += old_shift;
         } else {
            shift.insert(v, 0.);
         }
      }

      vi += vdelta;
   }

   for v in graph.vertices() {
      let val = x[&root[&v]];
      x.insert(v, val);
   }

   for v in graph.vertices() {
      let val = x[&v];
      x.insert(v, val + shift[&sink[&root[&v]]]);
   }

   post_process(
      &mut x,
      port_offset,
      align,
      root,
      inner_shift,
      &queue,
      graph,
      left_to_right,
   );

   x
}

pub fn inner_shift(
   root: &HashMap<AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor>,
   dims: &HashMap<AdjacencyListVertexDescriptor, (f32, f32)>,
   port_offset: &HashMap<AdjacencyListEdgeDescriptor, (f32, f32)>,
   graph: &AdjacencyList<usize, usize>,
) -> (HashMap<AdjacencyListVertexDescriptor, f32>, HashMap<AdjacencyListVertexDescriptor, f32>) {
   let mut inner_shift = HashMap::from_iter(graph.vertices().map(|vx| (vx, 0.)));
   let mut left_right = HashMap::<AdjacencyListVertexDescriptor, (f32, f32)>::new();

   for e in graph.edges() {
      let p = graph.source(e);
      let q = graph.target(e);
      let r = root[&p];

      if r == root[&q] {
         let (mut left, mut right) = left_right.entry(r).or_insert((0., 0.)).clone();
         let s = inner_shift[&p] + port_offset.get(&e).map(|x| x.0).unwrap_or(0.) - port_offset.get(&e).map(|x| x.1).unwrap_or(0.);

         inner_shift.insert(q, s);
         left = partial_min(left, s);
         right = partial_max(right, s + dims.get(&q).unwrap_or(&(0., 0.)).0);

         left_right.insert(r, (left, right));
      }
   }

   for vx in graph.vertices() {
      let r = root[&vx];
      let (left, _) = left_right.entry(r).or_insert((0., 0.)).clone();
      let s = inner_shift[&vx];

      inner_shift.insert(vx, s - left);
   }

   let block_size = HashMap::from_iter(left_right.into_iter().map(|(k, (l, r))| (k, (r - l))));

   (inner_shift, block_size)
}

pub fn compute_x_coordinates(
   order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
   rank: &HashMap<AdjacencyListVertexDescriptor, isize>,
   graph: &AdjacencyList<usize, usize>,
   dims: &HashMap<AdjacencyListVertexDescriptor, (f32, f32)>,
   port_offset: &HashMap<AdjacencyListEdgeDescriptor, (f32, f32)>,
   node_spacing: &Fn(usize) -> f32,
   virt_start: usize,
) -> HashMap<AdjacencyListVertexDescriptor, f32> {

   let type1 = mark_type1_conflicts(virt_start, &order, &rank, graph);
   let layouts = (0..4)
      .map(
         |k| {
            let up_to_down = k <= 1;
            let left_to_right = k % 2 == 0;
            let (root, align) = vertical_alignment(&order, &type1, &graph, up_to_down, left_to_right);
            let (inner_shift, block_size) = inner_shift(&root, &dims, &port_offset, &graph);
            let position = horizontal_compaction(
               &order,
               &rank,
               &graph,
               &align,
               &root,
               &dims,
               &port_offset,
               &inner_shift,
               node_spacing,
               up_to_down,
               left_to_right,
            );

            (root, align, position, inner_shift, block_size)
         }
      )
      .collect::<Vec<_>>();

   // balance layouts
   let min_max = layouts
      .iter()
      .enumerate()
      .map(
         |(_, &(_, _, ref pos, ref inner_shift, _))| {
            let min = pos.iter().min_by(|&(vx, x), &(wx, y)| (x + inner_shift[vx]).partial_cmp(&(y + inner_shift[wx])).unwrap_or(Ordering::Equal)).unwrap();
            let max = pos.iter()
               .max_by(
                  |&(vx, x), &(wx, y)| {
                     (dims.get(vx).unwrap_or(&(0., 0.)).0 + x + inner_shift[vx])
                        .partial_cmp(&(dims.get(wx).unwrap_or(&(0., 0.)).0 + y + inner_shift[wx]))
                        .unwrap_or(Ordering::Equal)
                  }
               )
               .unwrap();

            (min, max)
         }
      )
      .collect::<Vec<_>>();

   let (smallest_layout, &((_, _), (_, _))) = min_max
      .iter()
      .enumerate()
      .min_by(|&(_, &((_, min1), (_, max1))), &(_, &((_, min2), (_, max2)))| (max1 - min1).partial_cmp(&(max2 - min2)).unwrap_or(Ordering::Equal))
      .unwrap();

   // shifts
   let shift = min_max
      .iter()
      .enumerate()
      .map(
         |(k, &((_, min), (_, max)))| {
            let left_to_right = k % 2 == 0;
            if left_to_right {
               (min_max[smallest_layout].0).1 - min
            } else {
               (min_max[smallest_layout].1).1 - max
            }
         }
      )
      .collect::<Vec<_>>();

   let final_position = HashMap::from_iter(
      graph
         .vertices()
         .map(
            |vx| {
               let mut candidate_pos = layouts
                  .iter()
                  .zip(shift.iter())
                  .map(|(&(_, _, ref position, ref inner_shift, _), shift)| position[&vx] + inner_shift[&vx] + shift)
                  .collect::<Vec<_>>();

               candidate_pos.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal));
               (vx, (candidate_pos[1] as f32 + candidate_pos[2] as f32) / 2.)
            }
         )
   );

   final_position
}
