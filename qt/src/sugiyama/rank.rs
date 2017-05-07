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
use std::{isize,usize};
use std::iter::FromIterator;

use graph_algos::adjacency_list::{
    AdjacencyListVertexDescriptor
};

use graph_algos::{
    VertexListGraphTrait,
    EdgeListGraphTrait,
    BidirectionalGraphTrait,
    AdjacencyList,
    GraphTrait,
    MutableGraphTrait,
};

use graph_algos::search::{
    depth_first_visit,
    VertexEvent,
    EdgeKind,
};

/// Ensures a cycle-free graph has only a single entry.
///
/// This function checks whenever all vertices in `graph`
/// can be reached from `maybe_entry`. If `maybe_entry`
/// is `None` or if unreachable vertices are found, a new
/// vertex is added from which all vertices are reachable.
///
/// Returns the vertex from which all vertices are reachable.
///
/// # panics
/// If `graph` is not cycle-free.
pub fn ensure_single_entry(maybe_entry: Option<&AdjacencyListVertexDescriptor>,
                           graph: &mut AdjacencyList<usize,usize>) -> AdjacencyListVertexDescriptor {
    let mut heads = vec![];
    let mut seen = HashSet::new();

    if let Some(entry) = maybe_entry {
        heads.push(*entry);
        depth_first_visit(&mut|vx,ev| match ev {
            VertexEvent::Discovered => { seen.insert(*vx); },
            _ => {},
        },&mut|_,_| {},entry,graph);
    }

    while seen.len() < graph.num_vertices() {
        let maybe_h = graph.vertices().find(|x| {
            !seen.contains(x) && graph.in_degree(*x) == 0
        });

        if let Some(h) = maybe_h {
            heads.push(h);
            depth_first_visit(&mut|vx,ev| match ev {
                VertexEvent::Discovered => { seen.insert(*vx); },
                _ => {}
            },&mut|_,_| {},&h,graph);
        } else {
            if let Some(h) = graph.vertices().find(|x| !seen.contains(x)) {
                depth_first_visit(&mut|vx,ev| match ev {
                    VertexEvent::Discovered => { seen.insert(*vx); },
                    _ => {}
                },&mut|_,_| {},&h,graph);
            } else {
                unreachable!()
            }
        }
    }

    match heads.len() {
        0 => unreachable!(),
        1 => heads[0],
        _ => {
            let m = graph.vertices().filter_map(|x| graph.vertex_label(x)).max().map(|x| x + 1).unwrap_or(0);
            let ret = graph.add_vertex(m);

            for vx in heads {
                graph.add_edge(usize::MAX,ret,vx);
            }

            ret
        }
    }
}

pub fn remove_cycles(head: &AdjacencyListVertexDescriptor,graph: &mut AdjacencyList<usize,usize>) -> HashSet<usize> {
    let mut to_flip = vec![];
    let mut ret = HashSet::new();

    depth_first_visit::<usize,usize,AdjacencyList<usize,usize>>(&mut |_,_| {},&mut |e,k| if k == EdgeKind::Backward { to_flip.push(e.clone()) },head,graph);

    for e in to_flip {
        let from = graph.source(e);
        let to = graph.target(e);
        let lb = *graph.edge_label(e).unwrap();

        graph.remove_edge(e);
        graph.add_edge(lb,to,from);
        ret.insert(lb);
    }

    ret
}

pub fn remove_loops(graph: &mut AdjacencyList<usize,usize>) {
    let to_rm = graph.edges().filter(|&e| graph.source(e) == graph.target(e)).collect::<Vec<_>>();

    for e in to_rm {
        graph.remove_edge(e);
    }
}

pub fn remove_parallel_edges(graph: &mut AdjacencyList<usize,usize>) -> Vec<(usize,AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor)> {
    let mut seen_edges = HashSet::<(usize,usize)>::new();
    let to_rm = graph.edges().filter_map(|e| {
        let from = graph.vertex_label(graph.source(e)).unwrap();
        let to = graph.vertex_label(graph.target(e)).unwrap();


        if !seen_edges.insert((*from,*to)) {
            Some((e,*graph.edge_label(e).unwrap(),graph.source(e),graph.target(e)))
        } else {
            None
        }
    }).collect::<Vec<_>>();

    for e in to_rm.iter() {
        graph.remove_edge(e.0);
    }

    to_rm.iter().map(|&(_,a,b,c)| (a,b,c)).collect::<Vec<_>>()
}

/// Computes the ranks for all vertices.
pub fn compute_ranking(graph: &AdjacencyList<usize,usize>) -> HashMap<AdjacencyListVertexDescriptor,isize> {
    use cassowary::{ Solver, Variable };
    use cassowary::WeightedRelation::*;
    use cassowary::strength::{ WEAK, REQUIRED };

    let mut vx2var = HashMap::new();
    let mut solver = Solver::new();

    for vx in graph.vertices() {
        let var = Variable::new();

        // First rank is 0
        let _ = solver.add_constraint(var | GE(REQUIRED) | 0.0);

        // Minimize ranks
        let _ = solver.add_constraint(var | EQ(WEAK) | 0.0);

        vx2var.insert(vx,var);
    }

    for e in graph.edges() {
        let from_vx = vx2var[&graph.source(e)];
        let to_vx = vx2var[&graph.target(e)];

        // Adjacent vertices are at least on rank apart
        let _ = solver.add_constraint(to_vx - from_vx | GE(REQUIRED) | 1.0);
    }

    solver.update_variables();

    HashMap::from_iter(vx2var.iter().map(|(&vx,&var)| {
        let rank = solver.value_for(var);

        (vx,rank.unwrap_or(0.0) as isize)
    }))
}

pub fn add_virtual_vertices(rank: &mut HashMap<AdjacencyListVertexDescriptor,isize>,graph: &mut AdjacencyList<usize,usize>) -> (usize,usize) {
    let to_replace = graph.edges().filter(|&e| {
        let rank_from = rank.get(&graph.source(e)).unwrap();
        let rank_to = rank.get(&graph.target(e)).unwrap();

        assert!(rank_from <= rank_to);

        rank_to - rank_from > 1
    }).collect::<Vec<_>>();

    let mut next_label = graph.vertices().filter_map(|vx| graph.vertex_label(vx)).max().unwrap() + 1;
    let ret = next_label;
    for e in to_replace {
        let mut prev = graph.source(e);
        let last = graph.target(e);
        let lb = *graph.edge_label(e).unwrap();
        let rank_from = *rank.get(&prev).unwrap();
        let rank_to = *rank.get(&last).unwrap();

        for r in (rank_from + 1)..rank_to {
            let vx = graph.add_vertex(next_label);

            rank.insert(vx,r);
            graph.add_edge(lb,prev,vx);
            next_label += 1;
            prev = vx;

        }

        graph.add_edge(lb,prev,last);
        graph.remove_edge(e);
    }

    (ret,next_label)
}

pub fn normalize_rank(rank: &mut HashMap<AdjacencyListVertexDescriptor,isize>) {
    let offset = *rank.values().min().unwrap();

    for (_,v) in rank.iter_mut() {
        *v -= offset;
    }
}
