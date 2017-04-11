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

use sugiyama::order::{
    optimize_ordering,
    initial_ordering,
    radial_barycenter,
};

use sugiyama::rank::{
    ensure_single_entry,
    remove_cycles,
    remove_loops,
    remove_parallel_edges,
    compute_ranking,
    add_virtual_vertices,
    normalize_rank,
};

use sugiyama::linear::compute_x_coordinates;

pub fn radial_layout(vertices: &Vec<usize>,
              edges: &Vec<(usize,usize)>,
              entry: Option<usize>,
              rank_spacing: f32) -> Result<(HashMap<usize,(f32,f32)>,Vec<(f32,f32,f32,f32)>),Cow<'static,str>> {
    let mut graph = AdjacencyList::<usize,usize>::new();
    let mut rev = HashMap::<usize,AdjacencyListVertexDescriptor>::new();
    let mut maybe_entry = None;

    // radius
    let mut dims = HashMap::<AdjacencyListVertexDescriptor,(f32,f32)>::new();

    for &n in vertices.iter() {
        let vx = graph.add_vertex(n);

        rev.insert(n,vx);
        dims.insert(vx,(1.,1.));

        if entry == Some(n) {
            maybe_entry = Some(rev[&n].clone());
        }
    }

    for (idx,e) in edges.iter().enumerate() {
        graph.add_edge(idx,rev[&e.0],rev[&e.1]);
    }

    if !is_connected(&graph) {
        return Err("Input graph is not connected".into());
    }

    // normalize graph to DAG with single entry "head"
    let head = ensure_single_entry(maybe_entry.as_ref(),&mut graph);
    let revd_edge_labels = remove_cycles(&head,&mut graph);
    remove_loops(&mut graph);
    let revd_parallel_edge = remove_parallel_edges(&mut graph);

    // Desc -> Rank
    let mut rank = compute_ranking(&graph);

    // restore parallel edges
    for e in revd_parallel_edge {
        graph.add_edge(e.0,e.1,e.2);
    }

    if rank.len() != graph.num_vertices() {
        return Err("Internal error while ranking".into());
    }

    // split edges spanning multiple ranks
    let (virt_start,mut next_virt) = add_virtual_vertices(&mut rank,&mut graph);
    assert!(virt_start <= next_virt);

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

    // logical intra-rank ordering
    let mut order = initial_ordering(&rank,&head,&graph);

    if !(order[0].len() == 1 || order[0][0] != order[0][1]) {
        return Err("Internal error after initial ordering".into());
    }

    radial_barycenter(&mut order,&rank,&graph);

    let x_pos = compute_x_coordinates(&order,&rank,&graph,&dims,&HashMap::new(),&|r| 1./(r as f32 + 1.),virt_start);
    let z = order.iter().enumerate().fold(f32::NEG_INFINITY,|z,(rank_ord,rank)| {
        let min = rank.iter().min_by(|&a,&b| if x_pos[a] < x_pos[b] { Ordering::Less } else { Ordering::Greater });
        let max = rank.iter().max_by(|&a,&b| {
            let xa = x_pos[a] + dims.get(a).map(|x| x.0).unwrap_or(0.);
            let xb = x_pos[b] + dims.get(b).map(|x| x.0).unwrap_or(0.);
            if  xa < xb { Ordering::Less } else { Ordering::Greater }
        });

        if let (Some(min),Some(max)) = (min,max) {
            f32::max(z,x_pos[max] - x_pos[min] + 1. + (1. / (rank_ord as f32 + 1.)))
        } else {
            z
        }
    });

    use std::f32::consts::PI;
    let modu = |a,b| ((a % b) + b) % b;
    let twopi = 2.*PI;

    // position original vertices (basic blocks)
    let mut ret_v = HashMap::new();
    for n in vertices.iter() {

        let vx = rev[n];
        let radius = rank[&vx] as f32 * rank_spacing;
        let phi = modu(twopi*(x_pos[&vx] / z),twopi);

        ret_v.insert(*n,(phi,radius));
    }

    let mut ret_e = Vec::<(f32,f32,f32,f32)>::new();
    for e in graph.edges() {
        let start = graph.source(e);
        let end = graph.target(e);

        let start_radius = rank[&start] as f32 * rank_spacing as f32;
        let start_phi = modu(x_pos[&start] * (twopi / z),twopi);

        let end_radius = rank[&end] as f32 * rank_spacing as f32;
        let end_phi = modu(x_pos[&end] * (twopi / z),twopi);

        ret_e.push((start_phi,start_radius,end_phi,end_radius));
    }

    Ok((ret_v,ret_e))
}
