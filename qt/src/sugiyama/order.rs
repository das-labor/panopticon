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

use std::collections::{HashSet,HashMap};
use std::{f32,isize,usize};
use std::borrow::Cow;
use std::cmp::{min,max,Ordering};
use std::mem::swap;
use std::iter::FromIterator;

use graph_algos::adjacency_list::{
    AdjacencyListEdgeDescriptor,
    AdjacencyListVertexDescriptor
};

use graph_algos::{
    VertexListGraphTrait,
    EdgeListGraphTrait,
    BidirectionalGraphTrait,
    AdjacencyList,
    GraphTrait,
    IncidenceGraphTrait,
    MutableGraphTrait,
};

use graph_algos::search::{
    depth_first_visit,
    is_connected,
    VertexEvent,
    EdgeKind,
};

pub fn radial_barycenter(order: &mut Vec<Vec<AdjacencyListVertexDescriptor>>,
                         rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                         graph: &AdjacencyList<usize,usize>) {
    let mut pos = HashMap::<AdjacencyListVertexDescriptor,(f32,f32)>::new();

    // initial rank
    if order[0].len() == 1 {
        pos.insert(order[0][0],(0.,0.));
    } else {
        let step = 2. * f32::consts::PI / order[0].len() as f32;
        for (idx,vx) in order[0].iter().enumerate() {
            let phi = step * idx as f32;
            pos.insert(*vx,(phi.cos(),phi.sin()));
        }
    }

    for idx in 0..order.len() - 1 {
        let fixed = order[idx].clone();
        let barycenters = {
            let vary = &mut order[idx + 1];
            let mut barycenters = vary.iter().map(|vx| {
                let deg = graph.in_degree(*vx) as f32;
                let (mut x,mut y) = graph.in_edges(*vx).filter_map(|e| {
                    let wx = graph.source(e);
                    pos.get(&wx)
                }).fold((0.,0.),|(acc_x,acc_y),&(x,y)| (acc_x + x,acc_y + y));

                fn h(x: f32) -> f32 { if x <= 0. { 0. } else { 1. } }

                x /= deg;
                y /= deg;

                (*vx,y.atan2(x) + f32::consts::PI * h(-x) * f32::signum(y))
            }).collect::<Vec<_>>();

            barycenters.sort_by(|&(_,a),&(_,b)| if a < b { Ordering::Less }
                                else { Ordering::Greater });

            barycenters
        };

        order[idx + 1] = barycenters.into_iter().map(|(vx,_)| vx).collect::<Vec<_>>();
    }
}

pub fn initial_ordering(rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                    start: &AdjacencyListVertexDescriptor,
                    graph: &AdjacencyList<usize,usize>) -> Vec<Vec<AdjacencyListVertexDescriptor>> {
    let mut ret = Vec::new();

    depth_first_visit(&mut |vx,k| {
        if k == VertexEvent::Discovered {
            let r = rank[vx];

            assert!(r >= 0);

            while ret.len() as isize <= r {
                ret.push(Vec::new());
            }

            assert!(r >= 0);
            ret[r as usize].push(*vx)
        }
    },&mut |_,_| {},start,graph);

    ret
}

/// collects all edge pairs that need to be checked for crossings
/// TODO: This does not need to be O(n^2)
fn bipartite_subgraphs(rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                       graph: &AdjacencyList<usize,usize>)
                       -> HashMap<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>> {
    let mut ret = HashMap::<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>>::new();

    for e1 in graph.edges() {
        let e1src = graph.source(e1);
        let e1tgt = graph.target(e1);

        assert!(rank[&e1src] >= 0 && rank[&e1tgt] >= 0);
        let e1_start_rank = rank[&e1src] as usize;
        let e1_end_rank = rank[&e1tgt] as usize;

        for e2 in graph.edges() {
            if e1 != e2 {
                let e2src = graph.source(e2);
                let e2tgt = graph.target(e2);

                assert!(rank[&e2src] >= 0 && rank[&e2tgt] >= 0);
                let e2_start_rank = rank[&e2src] as usize;
                let e2_end_rank = rank[&e2tgt] as usize;
                let mut ranks = vec![e1_start_rank,e2_start_rank,e1_end_rank,e2_end_rank];

                ranks.sort();
                ranks.dedup();

                match ranks.len() {
                    0 => unreachable!(),
                    1 => ret.entry((ranks[0],ranks[0])).or_insert(vec![]).push((e1,e2)),
                    2 if ranks[1] - ranks[0] == 1 =>
                        ret.entry((ranks[0],ranks[1])).or_insert(vec![]).push((e1,e2)),
                    _ => {},
                }
            }
        }
    }

    ret
}

pub fn optimize_ordering(order: &mut Vec<Vec<AdjacencyListVertexDescriptor>>,
                     rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                     graph: &AdjacencyList<usize,usize>) {
    let bipartite = bipartite_subgraphs(rank,graph);
    let mut xings = crossings(&bipartite,&order,graph);

    if xings == 0 {
        return;
    }

    for i in 0..6 {
        let mut alt = order.clone();

        wmedian(i,&mut alt,rank,graph);

        let alt_xings = crossings(&bipartite,&alt,graph);

        if alt_xings < xings {
            *order = alt;
            xings = alt_xings;

            if xings == 0 {
                return;
            }
        } else if alt_xings == xings {
            return;
        }
    }
}

/// Computes the number of edge crossings in graph.
fn crossings(bipartite: &HashMap<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>>,
             order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
             graph: &AdjacencyList<usize,usize>) -> usize {
    let mut ret = 0;

    for (&(r_top,r_bot),v) in bipartite.iter() {
        assert!(r_top <= r_bot);

        let ord_top = &order[r_top];
        let ord_bot = &order[r_bot];

        // sum #crossings of all edge pairs between adjacent ranks
        for &(e1,e2) in v.iter() {
            let e1src = graph.source(e1);
            let e1tgt = graph.target(e1);
            let e2src = graph.source(e2);
            let e2tgt = graph.target(e2);

            // edges can't cross if they share a vertex
            if !(e1src == e2src || e1src == e2tgt || e1tgt == e2src || e1tgt == e2tgt) {
                let mut vert_set1 = vec![];
                let mut vert_set2 = vec![];

                // sort vertices from the upper rank into vert_set1 and from the lower into vert_set2
                // contents of the vectors are pairs (order,edge)
                for (v,e) in vec![(e1src,e1),(e1tgt,e1),(e2src,e2),(e2tgt,e2)] {
                    if let Some(o) = ord_top.iter().position(|&x| x == v) {
                        vert_set1.push((o,e));
                    } else {
                        vert_set2.push((ord_bot.iter().position(|&x| x == v).unwrap(),e));
                    }
                }

                // sort by x
                vert_set1.sort_by(|x,y| x.0.cmp(&y.0));
                vert_set2.sort_by(|x,y| x.0.cmp(&y.0));

                // ensure |vert_set1| < |vert_set2|. This saves cases in the following match
                if vert_set1.len() > vert_set2.len() {
                    swap(&mut vert_set1,&mut vert_set2);
                }

                // case split based on how the vertices are distributed along the adjacent ranks.
                // a and b are the resp. endpoints of the two edges.
                let has_crossing = match (vert_set1.len(),vert_set2.len()) {
                    // all vertices are in the same rank, edges cross if [a,b,a,b]
                    (0,4) => vert_set2[0].1 == vert_set2[2].1,
                    // all but one vertex are in the same rank, edges cross if [b] and [a,b,a]
                    (1,3) => vert_set1[0].1 == vert_set2[1].1,
                    // edges are split even between the two ranks, edges cross if [a,b] and [b,a]
                    (2,2) => vert_set2[0].1 == vert_set1[1].1 && vert_set2[1].1 == vert_set1[0].1,
                    _ => unreachable!()
                };

                if has_crossing {
                    ret += 1;
                }
            }
        }
    }

    ret
}

fn adj_positions(vx: &AdjacencyListVertexDescriptor,
                 adj_rank: usize,
                 order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                 rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                 graph: &AdjacencyList<usize,usize>) -> Vec<usize> {
    let mut ret = vec![];

    for e in graph.out_edges(*vx).chain(graph.in_edges(*vx)) {
        let other = if graph.source(e) == *vx {
            graph.target(e)
        } else {
            graph.source(e)
        };

        assert!(rank[&other] >= 0);
        let or = rank[&other] as usize;

        if or == adj_rank {
            ret.push(order[or].iter().position(|&x| x == other).unwrap());
        }
    }

    ret.sort();
    ret
}

fn median_value(vx: &AdjacencyListVertexDescriptor,
                adj_rank: usize,
                order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                graph: &AdjacencyList<usize,usize>) -> f32 {
    let p = adj_positions(vx,adj_rank,order,rank,graph);
    let m = (p.len() as f32 / 2.0).floor();

    assert!(m >= 0.0);

    if p.len() == 0 {
        -1.0
    } else if p.len() % 2 == 1 {
        p[m as usize] as f32
    } else if p.len() == 2 {
        (p[0] + p[1]) as f32 / 2.0
    } else {
        assert!(m >= 1.0);
        let left = (p[(m-1.0) as usize] - p[0]) as f32;
        let right = (p.last().unwrap() - p[m as usize]) as f32;

		((p[(m-1.0) as usize] as f32) * right + (p[m as usize] as f32) * left) / (left + right)
    }
}

fn wmedian(iter: usize,
           order: &mut Vec<Vec<AdjacencyListVertexDescriptor>>,
           rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
           graph: &AdjacencyList<usize,usize>) {
    let dir = iter % 2 == 0; // true -> torwards higher ranks
    let mut rank_idx = if dir { 0 } else { assert!(order.len() >= 1); order.len() - 1 } as usize;

    while rank_idx < order.len() {
        if (rank_idx < order.len() - 1 && !dir) || (rank_idx > 0 && dir) {
            let prev_rank = if dir { rank_idx - 1 } else { rank_idx + 1 };
            let mut new_order = HashMap::new();

            for vx in order[rank_idx].iter() {
                new_order.insert(vx.clone(),median_value(vx,prev_rank,order,rank,graph) as isize);
            }

            order[rank_idx].sort_by(|a,b| new_order[a].cmp(&new_order[b]));
        }

        if dir {
            rank_idx += 1;
        } else {
            if rank_idx == 0 {
                break;
            } else {
                rank_idx -= 1;
            }
        }
    }
}
