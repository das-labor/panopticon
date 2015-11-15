/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
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
use std::{f32,isize};
use std::ptr;
use std::cmp::{min,max,Ordering};
use libc::{c_int,c_double};

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
    MutableGraphTrait
};

use glpk;

pub fn layout(vertices: &Vec<usize>,
              edges: &Vec<(usize,usize)>,
              _dims: &HashMap<usize,(f32,f32)>,
              entry: Option<usize>,
              node_spacing: usize,
              rank_spacing: usize) -> (HashMap<usize,(f32,f32)>,Vec<(f32,f32,f32,f32)>) {
    let mut graph = AdjacencyList::<usize,()>::new();
    let mut rev = HashMap::<usize,AdjacencyListVertexDescriptor>::new();
    let mut maybe_entry = None;
    let mut dims = HashMap::<AdjacencyListVertexDescriptor,(f32,f32)>::new();

    for &n in vertices.iter() {
        let vx = graph.add_vertex(n);

        rev.insert(n,vx);
        if entry == Some(n) {
            maybe_entry = Some(rev[&n].clone());
        }

        dims.insert(vx,_dims[&n]);
    }

    for e in edges.iter() {
        graph.add_edge((),rev[&e.0],rev[&e.1]);
    }

    assert!(is_connected(&graph));

    // normalize graph to DAG with single entry "head"
    let head = ensure_single_entry(maybe_entry.as_ref(),&mut graph);
    remove_cycles(&head,&mut graph);
    remove_loops(&mut graph);
    remove_parallel_edges(&mut graph);

    // rank assignment
    let rank_vec = {
        let (a,b,c,lb,ub) = build_ranking_integer_program(&graph);
        solve_integer_program(&a,&b,&c,&lb,&ub).iter().take(graph.num_vertices()).cloned().collect::<Vec<isize>>()
    };

    let mut rank = HashMap::new();  // Desc -> Rank
    for vx in graph.vertices() {
        let lb = *graph.vertex_label(vx).unwrap();
        rank.insert(vx,rank_vec[lb]);
    }
    assert_eq!(rank.len(), graph.num_vertices());

    let virt_start = add_virtual_vertices(&mut rank,&mut graph);
    assert_eq!(rank.len(), graph.num_vertices());
    normalize_rank(&mut rank);

    assert_eq!(rank.len(), graph.num_vertices());
    for e in graph.edges() {
        let from = graph.source(e);
        let to = graph.target(e);

        assert_eq!(rank[&from] + 1, rank[&to]);
    }

    let mut order = initial_ordering(&rank,&head,&graph);
    assert!(order[0].len() == 1 || order[0][0] != order[0][1]);
    optimize_ordering(&mut order,&rank,&graph);

    let x_pos = compute_x_coordinates(&order,&rank,&graph,&dims,node_spacing,virt_start);

    let rank_offsets = order.iter()
        .map(|r| r.iter().fold(0usize,|acc,vx| max(dims.get(vx).map(|x| { assert!(x.1 >= 0.0); x.1 }).unwrap_or(0.0) as usize,acc)))
        .fold(vec![0usize],|acc,x| { let mut ret = acc.clone(); ret.push(acc.last().unwrap() + x + (rank_spacing as usize)); ret });

    let mut ret_v = HashMap::new();
    for n in vertices.iter() {
        let vx = rev[n];

        assert!(rank[&vx] >= 0);
        let r = rank[&vx] as usize;
        let rank_start = rank_offsets[r] as f32;
        let rank_end = rank_offsets[r + 1] as f32;

        ret_v.insert(*n,(x_pos[&vx] as f32,(rank_start + ((rank_end - rank_start) / 2.0)) as f32));
    }

    let mut ret_e = Vec::<_>::new();
    for e in graph.edges() {
        let s = graph.source(e);
        let t = graph.target(e);
        let sx = x_pos[&s];
        let tx = x_pos[&t];

        assert!(rank[&s] >= 0 && rank[&t] >= 0);
        let sr = rank[&s] as usize;
        let tr = rank[&t] as usize;
        let srs = rank_offsets[sr] as f32;
        let sre = rank_offsets[sr + 1] as f32;
        let trs = rank_offsets[tr] as f32;
        let tre = rank_offsets[tr + 1] as f32;

        ret_e.push((sx,srs + (sre - srs) / 2.0,tx,trs + (tre - trs) / 2.0));
    }

    (ret_v,ret_e)
}

fn depth_first_search(seen: &mut HashSet<AdjacencyListVertexDescriptor>,
                      start: &AdjacencyListVertexDescriptor,
                      graph: &AdjacencyList<usize,()>) {
    if seen.contains(start) {
        return;
    }

    let mut stack = vec![start.clone()];

    while !stack.is_empty() {
        let vx = stack.pop().unwrap().clone();

        seen.insert(vx);
        for out in graph.out_edges(vx) {
            let s = graph.target(out);

            if !seen.contains(&s) {
                stack.push(s);
            }
        }
    }
}

fn is_connected(graph: &AdjacencyList<usize,()>) -> bool {
    if let Some(s) = graph.vertices().next() {
        let mut seen = HashSet::<AdjacencyListVertexDescriptor>::new();
        let mut stack = vec![s];

        while !stack.is_empty() {
            let vx = stack.pop().unwrap().clone();

            seen.insert(vx);
            let ed = graph.out_edges(vx).map(|out| graph.target(out)).chain(
                graph.in_edges(vx).map(|_in| graph.source(_in))).collect::<Vec<_>>();

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

#[derive(PartialEq,Eq)]
enum EdgeKind {
    Tree,
    ForwardOrCross,
    Backward,
}

#[derive(PartialEq,Eq)]
enum VertexEvent {
    Discovered,
    Finished,
}

#[derive(PartialEq,Eq,Hash)]
enum VertexColor {
    White,
    Gray,
    Black,
}

fn depth_first_visit(vertex_visitor: &mut FnMut(&AdjacencyListVertexDescriptor,VertexEvent),
                     edge_visitor: &mut FnMut(&AdjacencyListEdgeDescriptor,EdgeKind),
                     start: &AdjacencyListVertexDescriptor,
                     graph: &AdjacencyList<usize,()>) {
    let mut color = HashMap::new();

    for v in graph.vertices() {
        color.insert(v,VertexColor::White);
    }

    fn visit(vx: AdjacencyListVertexDescriptor,
             color: &mut HashMap<AdjacencyListVertexDescriptor,VertexColor>,
             vertex_visitor: &mut FnMut(&AdjacencyListVertexDescriptor,VertexEvent),
             edge_visitor: &mut FnMut(&AdjacencyListEdgeDescriptor,EdgeKind),
             graph: &AdjacencyList<usize,()>) {
        color.insert(vx,VertexColor::Gray);

        vertex_visitor(&vx,VertexEvent::Discovered);

        for e in graph.out_edges(vx) {
            let wx = graph.target(e);

            match color[&wx] {
                VertexColor::White => {
                    edge_visitor(&e,EdgeKind::Tree);
                    visit(wx,color,vertex_visitor,edge_visitor,graph);
                },
                VertexColor::Gray => edge_visitor(&e,EdgeKind::Backward),
                VertexColor::Black => edge_visitor(&e,EdgeKind::ForwardOrCross),
            }
        }

        color.insert(vx,VertexColor::Black);
        vertex_visitor(&vx,VertexEvent::Finished);
    }

    visit(*start,&mut color,vertex_visitor,edge_visitor,graph);

    for v in graph.vertices() {
        if color[&v] == VertexColor::White {
            visit(v,&mut color,vertex_visitor,edge_visitor,graph);
        }
    }
}

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
                           graph: &mut AdjacencyList<usize,()>) -> AdjacencyListVertexDescriptor {
    let mut heads = vec![];
    let mut seen = HashSet::new();

    if let Some(entry) = maybe_entry {
        heads.push(*entry);
        depth_first_search(&mut seen,entry,graph);
    }

    while seen.len() < graph.num_vertices() {
        let maybe_h = graph.vertices().find(|x| {
            !seen.contains(x) && graph.in_degree(*x) == 0
        });

        if let Some(h) = maybe_h {
            heads.push(h);
            depth_first_search(&mut seen,&h,graph);
        } else {
            if let Some(h) = graph.vertices().find(|x| !seen.contains(x)) {
                depth_first_search(&mut seen,&h,graph);
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
                graph.add_edge((),ret,vx);
            }

            ret
        }
    }
}

pub fn remove_cycles(head: &AdjacencyListVertexDescriptor,graph: &mut AdjacencyList<usize,()>) {
    let mut to_flip = Vec::new();

    depth_first_visit(&mut |_,_| {},&mut |e,k| if k == EdgeKind::Backward { to_flip.push(e.clone()) },head,graph);

    for e in to_flip {
        let from = graph.source(e);
        let to = graph.target(e);

        graph.remove_edge(e);
        graph.add_edge((),to,from);
    }
}

pub fn remove_loops(graph: &mut AdjacencyList<usize,()>) {
    let to_rm = graph.edges().filter(|&e| graph.source(e) == graph.target(e)).collect::<Vec<_>>();

    for e in to_rm {
        graph.remove_edge(e);
    }
}

pub fn remove_parallel_edges(graph: &mut AdjacencyList<usize,()>) {
    let mut seen_edges = HashSet::<(usize,usize)>::new();
    let to_rm = graph.edges().filter(|&e| {
        let from = graph.vertex_label(graph.source(e)).unwrap();
        let to = graph.vertex_label(graph.target(e)).unwrap();

        !seen_edges.insert((*from,*to))
    }).collect::<Vec<_>>();

    for e in to_rm {
        graph.remove_edge(e);
    }
}

fn build_ranking_integer_program<'a>(graph: &AdjacencyList<usize,()>) -> (Vec<Vec<isize>>,Vec<isize>,Vec<isize>,Vec<isize>,Vec<isize>) {
    let mut a = Vec::new();
    let b = (0..graph.num_edges()).map(|_| 0).collect::<Vec<_>>();
    let c = (0..graph.num_vertices()).map(|_| 0).chain((0..graph.num_edges()).map(|_| 1)).collect::<Vec<_>>();
    let lb = (0..graph.num_vertices()).map(|_| 0).chain((0..graph.num_edges()).map(|_| 1)).collect::<Vec<_>>();
    let ub = (0..(graph.num_edges() + graph.num_vertices())).map(|_| graph.num_vertices() as isize).collect::<Vec<_>>();

    for (i,e) in graph.edges().enumerate() {
        let mut a_row = (0..(graph.num_edges() + graph.num_vertices())).map(|_| 0).collect::<Vec<_>>();
        let from = *graph.vertex_label(graph.source(e)).unwrap();
        let to = *graph.vertex_label(graph.target(e)).unwrap();

        a_row[from] = -1;
        a_row[to] = 1;
        a_row[graph.num_vertices() + i] = -1;
        a.push(a_row);
	}

    (a,b,c,lb,ub)
}

pub fn add_virtual_vertices(rank: &mut HashMap<AdjacencyListVertexDescriptor,isize>,graph: &mut AdjacencyList<usize,()>) -> usize {
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
        let rank_from = *rank.get(&prev).unwrap();
        let rank_to = *rank.get(&last).unwrap();

        for r in ((rank_from + 1)..(rank_to)) {
            let vx = graph.add_vertex(next_label);

            rank.insert(vx,r);
            graph.add_edge((),prev,vx);
            next_label += 1;
            prev = vx;

        }

        graph.add_edge((),prev,last);
        graph.remove_edge(e);
    }

    ret
}

fn normalize_rank(rank: &mut HashMap<AdjacencyListVertexDescriptor,isize>) {
    let offset = *rank.values().min().unwrap();

    for (_,v) in rank.iter_mut() {
        *v -= offset;
    }
}

fn initial_ordering(rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                    start: &AdjacencyListVertexDescriptor,
                    graph: &AdjacencyList<usize,()>) -> Vec<Vec<AdjacencyListVertexDescriptor>> {
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

fn bipartite_subgraphs(rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                       graph: &AdjacencyList<usize,()>)
                       -> HashMap<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>> {
    let mut ret = HashMap::<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>>::new();

    for e1 in graph.edges() {
        let e1src = graph.source(e1);
        let e1tgt = graph.target(e1);

        assert!(rank[&e1src] >= 0 && rank[&e1tgt] >= 0);
        let e1_start_rank = rank[&e1src] as usize;
        let e1_end_rank = rank[&e1tgt] as usize;

        for e2 in graph.edges() {
            let e2src = graph.source(e2);
            let e2tgt = graph.target(e2);

            assert!(rank[&e2src] >= 0 && rank[&e2tgt] >= 0);
            let e2_start_rank = rank[&e2src] as usize;
            let e2_end_rank = rank[&e2tgt] as usize;

			if e1_start_rank == e2_start_rank && e1_end_rank == e2_end_rank {
                ret.entry((e1_start_rank,e1_end_rank)).or_insert(vec![]).push((e1,e2))
            }
        }
    }

    ret
}

fn optimize_ordering(order: &mut Vec<Vec<AdjacencyListVertexDescriptor>>,
                     rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                     graph: &AdjacencyList<usize,()>) {

    let bipartite = bipartite_subgraphs(rank,graph);
    let mut xings = crossings(&bipartite,&order,graph);

    if xings == 0 {
        return;
    }

    for i in (0..6) {
        let mut alt = order.clone();

        wmedian(i,&mut alt,rank,graph);

        let mut alt_xings = crossings(&bipartite,&alt,graph);
        transpose(&mut alt_xings,&mut alt,&bipartite,graph);

        if alt_xings < xings {
            *order = alt;
            xings = alt_xings;

            if xings == 0 {
                return;
            }
        }
    }
}

fn crossings(bipartite: &HashMap<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>>,
             order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
             graph: &AdjacencyList<usize,()>) -> usize {
    let mut ret = 0;

    for (&(r_start,r_end),v) in bipartite.iter() {
        let ord_start = &order[r_start];
        let ord_end = &order[r_end];

        for &(e1,e2) in v.iter() {
            let e1src = graph.source(e1);
            let e1tgt = graph.target(e1);
            let e1_start_ord = ord_start.iter().position(|&x| x == e1src).unwrap();
            let e1_end_ord = ord_end.iter().position(|&x| x == e1tgt).unwrap();
            let e2src = graph.source(e2);
            let e2tgt = graph.target(e2);
            let e2_start_ord = ord_start.iter().position(|&x| x == e2src).unwrap();
            let e2_end_ord = ord_end.iter().position(|&x| x == e2tgt).unwrap();

            if (e1_start_ord != e1_end_ord) && (e2_start_ord != e2_end_ord) &&
                 ((e1_start_ord <= e1_end_ord) != (e2_start_ord <= e2_end_ord)) {
                ret += 1;
            }
        }
    }

    ret
}

fn adj_positions(vx: &AdjacencyListVertexDescriptor,
                 adj_rank: usize,
                 order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                 rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                 graph: &AdjacencyList<usize,()>) -> Vec<usize> {
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
                graph: &AdjacencyList<usize,()>) -> f32 {
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
           graph: &AdjacencyList<usize,()>) {
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

fn transpose(xings: &mut usize,
             order: &mut Vec<Vec<AdjacencyListVertexDescriptor>>,
             bipartite: &HashMap<(usize,usize),Vec<(AdjacencyListEdgeDescriptor,AdjacencyListEdgeDescriptor)>>,
             graph: &AdjacencyList<usize,()>) {
    let mut imp = true;
    let mut alt = order.clone();

    while imp {
        imp = false;

        for order_idx in (0..order.len()) {
            for rank_idx in (0..(order[order_idx].len()-1)) {
                let v = order[order_idx][rank_idx];
                let w = order[order_idx][rank_idx+1];

                alt[order_idx][rank_idx] = w;
                alt[order_idx][rank_idx+1] = v;

                let alt_xings = crossings(bipartite,&alt,graph);

                if alt_xings < *xings {
                    order[order_idx][rank_idx] = w;
                    order[order_idx][rank_idx+1] = v;

                    *xings = alt_xings;
                    imp = true;
                } else {
                    alt[order_idx][rank_idx] = v;
                    alt[order_idx][rank_idx+1] = w;
                }
            }
        }
    }
}

fn solve_integer_program(a: &Vec<Vec<isize>>,
                         b: &Vec<isize>,
                         c: &Vec<isize>,
                         lb: &Vec<isize>,
                         ub: &Vec<isize>) -> Vec<isize> {
    unsafe {
        glpk::glp_term_out(0 as c_int); // GLP_OFF
        let lp = glpk::glp_create_prob();

        assert_eq!(a.len(), b.len());
        assert_eq!(lb.len(), ub.len());
        assert_eq!(lb.len(), c.len());

        glpk::glp_set_obj_dir(lp, 1); // GLP_MIN
        glpk::glp_add_rows(lp, (a.len() + 1) as c_int);
        glpk::glp_add_cols(lp, a[0].len() as c_int);

        for (j,(l,u)) in lb.iter().zip(ub.iter()).enumerate() {
            glpk::glp_set_col_bnds(lp, (j + 1) as c_int, 4, *l as c_double, *u as c_double); // GLP_DB
            glpk::glp_set_obj_coef(lp, (j + 1) as c_int, c[j] as c_double);
        }

        for (i,&x) in b.iter().enumerate() {
            glpk::glp_set_row_bnds(lp, (i + 1) as c_int, 5 as c_int, x as c_double, x as c_double); // GLP_FX
        }

        for (i,row) in a.iter().enumerate() {
            let conv = [0].iter().chain(row.iter()).map(|x| *x as c_double).collect::<Vec<c_double>>();
            let idx = (0..(row.len()+1)).map(|x| x as c_int).collect::<Vec<c_int>>();

            assert_eq!(row.len(), c.len());
            assert_eq!(conv.len(),idx.len());

            glpk::glp_set_mat_row(lp, (i+1) as c_int, (conv.len() - 1) as c_int, idx.as_ptr(), conv.as_ptr());
        }

        glpk::glp_simplex(lp, ptr::null());

        let ret = a[0].iter().enumerate().map(|(j,_)| {
            glpk::glp_get_col_prim(lp, (j + 1) as c_int) as isize
        }).collect::<Vec<isize>>();

        glpk::glp_delete_prob(lp);

        ret
    }
}

pub fn mark_type1_conflicts(virt_start: usize,
                            order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                            rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                            graph: &AdjacencyList<usize,()>,
                            up_to_down: bool) -> Vec<(AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor)> {

    let mut ret = Vec::<(AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor)>::new();

    if order.len() < 3 { return ret; }

    let (from,to,delta) = if up_to_down {
        (1,order.len() as isize - 2,1)
    } else {
        (order.len() as isize - 3,0,-1)
    };

    let mut i = from;
    while i != to {
        assert!(i + delta >= 0 && i + delta < order.len() as isize);

        let mut k0 = 0;
        let mut l = 0;
        let next = (i + delta) as usize;

        for l1 in 0..order[next].len() {
            let v = order[next][l1];
            let mut edges = if up_to_down { graph.in_edges(v) } else { graph.out_edges(v) };
            let maybe_upn = edges.find(|&e| {
                let w = if up_to_down { graph.source(e) } else { graph.target(e) };
                rank[&w] == i as isize && *graph.vertex_label(w).unwrap() >= virt_start &&
                    *graph.vertex_label(v).unwrap() >= virt_start
            });

            if l1 == order[next].len() || maybe_upn.is_some() {
                assert!(i >= 0);

                let k1 = if let Some(upn) = maybe_upn {
                    order[i as usize].iter().position(|&x| {
                        (up_to_down && x == graph.source(upn)) ||
                        (!up_to_down && x == graph.target(upn))
                    }).unwrap()
                } else {
                    order[i as usize].len() - 1
                };

                while l <= l1 {
                    let edges = if up_to_down { graph.in_edges(v) } else { graph.out_edges(v) };
                    for e in edges {
                        let w = if up_to_down { graph.source(e) } else { graph.target(e) };

                        if rank[&w] == i && *graph.vertex_label(w).unwrap() >= virt_start {
                            assert!(i >= 0);
                            let k = order[i as usize].iter().position(|&x| x == w).unwrap();

                            if k < k0 || k > k1 {
                                if up_to_down {
                                    ret.push((w,graph.target(e)));
                                } else {
                                    ret.push((w,graph.source(e)));
                                }
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

pub fn vertical_alignment(order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                          type1: &Vec<(AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor)>,
                          graph: &AdjacencyList<usize,()>,
                          up_to_down: bool,
                          left_to_right: bool) -> (HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>, HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>) {

    let mut root = HashMap::<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>::new();
    let mut align = HashMap::<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>::new();

    for v in graph.vertices() {
        root.insert(v,v);
        align.insert(v,v);
    }

    let (from,to,delta) = if up_to_down {
        (1,order.len() as isize,1)
    } else {
        (order.len() as isize - 2,-1,-1isize)
    };

    let mut i = from;
    while i != to {
        let mut r = if left_to_right { -1 } else { isize::MAX };
        let (lev_from,lev_to,lev_delta) = if left_to_right {
            (0,order[i as usize].len() as isize,1isize)
        } else {
            (order[i as usize].len() as isize - 1,-1isize,-1isize)
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
                    vec![((upn.len() - 1) as f32 / 2.0).floor(),((upn.len() - 1) as f32 / 2.0).ceil()]
                } else {
                    vec![(upn.len() - 1) as f32 / 2.0]
                };

                for m in medians {
                    if align[&v] == v {
                        assert!(m >= 0.0);
                        let um = upn[m as usize];
                        let pos = order[prev].iter().position(|&x| x == um).unwrap() as isize;

                        if !type1.contains(&(um,v)) &&
                            !type1.contains(&(v,um)) &&
                            ((left_to_right && r < pos) || (!left_to_right && r > pos)) {
                            align.insert(um,v);

                            let a = root[&um];
                            root.insert(v,a);

                            let b = root[&v];
                            align.insert(v,b);
                            r = pos;

                        }
                    }
                }
            }

            k += lev_delta;
        }

        i += delta;
    }

    (root,align)
}

fn place_block(v: AdjacencyListVertexDescriptor,
               order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
               rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
               sink: &mut HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>,
               shift: &mut HashMap<AdjacencyListVertexDescriptor,isize>,
               x: &mut HashMap<AdjacencyListVertexDescriptor,isize>,
               align: &HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>,
               root: &HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>,
               widths: &HashMap<AdjacencyListVertexDescriptor,usize>,
               graph: &AdjacencyList<usize,()>,
               node_spacing: usize,
               left_to_right: bool) {

    let delta: isize = if left_to_right { 1 } else { -1isize };

    if !x.contains_key(&v) {
        x.insert(v,0);
        let mut w = v;

        loop {
            assert!(rank[&w] >= 0);
            let word: &Vec<AdjacencyListVertexDescriptor> = &order[rank[&w] as usize];

            if (left_to_right && *word.first().unwrap() != w) ||
                (!left_to_right && *word.last().unwrap() != w) {

                let pred = word.iter().position(|&x| x == w).unwrap() as isize - delta;
                assert!(pred >= 0);
                let u = root[&word[pred as usize]];

                place_block(u,order,rank,sink,shift,x,align,root,widths,graph,node_spacing,left_to_right);

                if sink[&v] == v {
                    let t = sink[&u];
                    sink.insert(v,t);
                }

                let sep: isize = ((widths[&u] as isize +
                                   widths[&v] as isize) / 2) as isize;

                if sink[&v] != sink[&u] {
                    let sinku = sink[&u].clone();
                    let prev = shift[&sinku];

                    if !left_to_right {
                        shift.insert(sinku.clone(),min(prev,x[&v] - x[&u] - node_spacing as isize - sep as isize));
                    } else {
                        shift.insert(sinku.clone(),max(prev,x[&v] - x[&u] + node_spacing as isize + sep as isize));
                    }
                } else {

                    let val = if left_to_right {
                        max(x[&v],x[&u] + node_spacing as isize + sep as isize)
                    } else {
                        min(x[&v],x[&u] - node_spacing as isize - sep as isize)
                    };
                    x.insert(v,val);
                }
            }

            assert!(w != align[&w] || align[&w] == v);
            w = align[&w];

            if w == v {
                break;
            }
        }
    }
}

fn horizontal_compaction(order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                         rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                         graph: &AdjacencyList<usize,()>,
                         align: &HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>,
                         root: &HashMap<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>,
                         widths: &HashMap<AdjacencyListVertexDescriptor,usize>,
                         node_spacing: usize,
                         up_to_down: bool,
                         left_to_right: bool) -> HashMap<AdjacencyListVertexDescriptor,isize> {

    let mut sink = HashMap::<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>::new();
    let mut shift = HashMap::<AdjacencyListVertexDescriptor,isize>::new();
    let mut x = HashMap::<AdjacencyListVertexDescriptor,isize>::new();

    for v in graph.vertices() {
        sink.insert(v,v);
        shift.insert(v,isize::MAX);
    }

    let (vfrom,vto,vdelta) = if up_to_down {
        (0,order.len() as isize,1isize)
    } else {
        (order.len() as isize - 1,-1,-1isize)
    };

    let mut vi = vfrom;
    while vi != vto {
        assert!(vi >= 0);

        let (hfrom,hto,hdelta) = if left_to_right {
            (0,order[vi as usize].len() as isize,1isize)
        } else {
            (order[vi as usize].len() as isize - 1,-1,-1isize)
        };

        let mut hi = hfrom;
        while hi != hto {
            let v = order[vi as usize][hi as usize];

            if root[&v] == v {
                place_block(v,order,rank,&mut sink,&mut shift,&mut x,&align,&root,widths,graph,node_spacing,left_to_right);
            }

            hi += hdelta;
        }
        vi += vdelta;
    }

    vi = vfrom;
    let mut d = 0;
    while vi != vto {
        assert!(vi >= 0);

        let v = *if left_to_right {
            order[vi as usize].first().unwrap()
        } else {
            order[vi as usize].last().unwrap()
        };

        if v == sink[&root[&v]] {
            let old_shift: isize = shift[&v];
            if old_shift < isize::MAX {
                shift.insert(v,old_shift + d);
                d += old_shift;
            } else {
                shift.insert(v,0);
            }
        }

        vi += vdelta;
    }

    for v in graph.vertices() {
        let val = x[&root[&v]];
        x.insert(v,val);
    }

    for v in graph.vertices() {
        let val = x[&v];
        x.insert(v,val + shift[&sink[&root[&v]]]);
    }

    x
}

pub fn compute_x_coordinates(order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                             rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                             graph: &AdjacencyList<usize,()>,
                             dims: &HashMap<AdjacencyListVertexDescriptor,(f32,f32)>,
                             node_spacing: usize,
                             virt_start: usize) -> HashMap<AdjacencyListVertexDescriptor,f32> {

    let mut root = vec![];
    let mut align = vec![];
    let mut x = vec![];
    let mut widths = vec![];

    for k in (0..4) {
        let up_to_down = k <= 1;
        let left_to_right = k % 2 == 0;
        let type1 = mark_type1_conflicts(virt_start,&order,&rank,graph,up_to_down);
        let (r,a) = vertical_alignment(&order,&type1,&graph,up_to_down,left_to_right);
        let mut w = HashMap::<AdjacencyListVertexDescriptor,usize>::new();

        for v in graph.vertices() {
            let _r = r[&v];
            let width = dims.get(&v).map(|x| x.0 as usize).unwrap_or(1);
            let val = *w.get(&_r).unwrap_or(&0);

            w.insert(_r,max(val,width));
        }

        let _x = horizontal_compaction(&order,&rank,&graph,&a,&r,&w,node_spacing,up_to_down,left_to_right);

        root.push(r);
        align.push(a);
        x.push(_x);
        widths.push(w);
    }

    let mut max = vec![];
    let mut min = vec![];
    let mut width = vec![];
    let mut global_min = 0;

    for k in (0..4) {
        let mut mi = f32::INFINITY;
        let mut ma = f32::NEG_INFINITY;

        for v in graph.vertices() {
            let bw = 0.5 * widths[k][&root[k][&v]] as f32;
            let xp = x[k][&v] as f32 - bw;
            let mxp = x[k][&v] as f32 + bw;

            if mi > xp { mi = xp; }
            if ma < mxp { ma = mxp; }
        }

        min.push(mi);
        max.push(ma);
        width.push(ma - mi);

        if width[global_min] > ma - mi {
            global_min = k;
        }
    }

    let mut shift = vec![];

    for k in (0..4) {
        if k % 2 == 0 {
            shift.push(min[global_min] - min[k]);
        } else {
            shift.push(max[global_min] - max[k]);
        }
    }

    let mut ret = HashMap::<AdjacencyListVertexDescriptor,f32>::new();

    for v in graph.vertices() {
        let mut sort = (0..4).map(|i| x[i][&v] as f32 + shift[i]).collect::<Vec<_>>();
        assert_eq!(sort.len(),4);
        sort.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        ret.insert(v,0.5f32 * (sort[1] + sort[2]));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph_algos::{
        AdjacencyList,
        GraphTrait,
        MutableGraphTrait,
        VertexListGraphTrait,
        EdgeListGraphTrait
    };
    use graph_algos::adjacency_list::{
        AdjacencyListEdgeDescriptor,
        AdjacencyListVertexDescriptor
    };

    use std::collections::{HashSet,HashMap};

    #[test]
    fn test_remove_loops() {
        let mut graph = AdjacencyList::<usize,()>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);
        let vx5 = graph.add_vertex(4);

        graph.add_edge((),vx1,vx2);
        graph.add_edge((),vx2,vx3);
        graph.add_edge((),vx3,vx4);
        graph.add_edge((),vx4,vx1);
        graph.add_edge((),vx3,vx5);

        let e11 = graph.add_edge((),vx1,vx1).unwrap();
        let e22 = graph.add_edge((),vx2,vx2).unwrap();
        let e22b = graph.add_edge((),vx2,vx2).unwrap();

        remove_loops(&mut graph);

        assert_eq!(graph.num_vertices(), 5);
        assert_eq!(graph.num_edges(), 5);

        assert_eq!(graph.edge_label(e11),None);
        assert_eq!(graph.edge_label(e22),None);
        assert_eq!(graph.edge_label(e22b),None);
    }

    #[test]
    fn test_single_entry() {
        let mut graph = AdjacencyList::<usize,()>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);
        let vx5 = graph.add_vertex(4);

        graph.add_edge((),vx1,vx2);
        graph.add_edge((),vx2,vx3);
        graph.add_edge((),vx2,vx4);
        graph.add_edge((),vx3,vx5);
        graph.add_edge((),vx4,vx5);

        assert_eq!(ensure_single_entry(Some(&vx1),&mut graph), vx1);
    }

    #[test]
    fn test_multi_entry() {
        let mut graph = AdjacencyList::<usize,()>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);
        let vx5 = graph.add_vertex(4);
        let vx6 = graph.add_vertex(5);

        graph.add_edge((),vx1,vx2);
        graph.add_edge((),vx2,vx3);
        graph.add_edge((),vx2,vx4);
        graph.add_edge((),vx4,vx5);
        graph.add_edge((),vx3,vx5);
        graph.add_edge((),vx6,vx3);

        let new_vx = ensure_single_entry(None,&mut graph);

        assert!(new_vx != vx1);
        assert!(new_vx != vx2);
        assert!(new_vx != vx3);
        assert!(new_vx != vx4);
        assert!(new_vx != vx5);
        assert!(new_vx != vx6);
        assert_eq!(graph.vertex_label(new_vx), Some(&6));
        assert_eq!(graph.num_vertices(), 7);
        assert_eq!(graph.num_edges(), 8);
    }

    #[test]
    fn remove_single_cycle() {
        let mut graph = AdjacencyList::<usize,()>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);

        graph.add_edge((),vx1,vx2);
        graph.add_edge((),vx2,vx3);
        graph.add_edge((),vx4,vx2);

        remove_cycles(&vx1,&mut graph);

        assert_eq!(graph.num_vertices(), 4);
        assert_eq!(graph.num_edges(), 3);
    }

    #[test]
    fn virtual_vertex_insertion() {
        let mut graph = AdjacencyList::<usize,()>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);

        graph.add_edge((),vx1,vx2);
        graph.add_edge((),vx1,vx3);
        graph.add_edge((),vx1,vx4);

        let mut ranks = HashMap::new();

        ranks.insert(vx1,0);
        ranks.insert(vx2,1);
        ranks.insert(vx3,2);
        ranks.insert(vx4,3);

        add_virtual_vertices(&mut ranks,&mut graph);

        assert_eq!(graph.num_vertices(), 7);
        assert_eq!(ranks.len(), 7);
        assert_eq!(graph.num_edges(), 6);
        assert!(graph.edges().all(|e| {
            let fr = ranks[&graph.source(e)];
            let tr = ranks[&graph.target(e)];
            tr - fr == 1
        }));
    }

    #[test]
    fn large_graph() {
        let nodes = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30];
        let edges = vec![(22,13),(1,23),(23,3),(27,16),(20,27),(17,12),(30,20),(9,25),(16,25),(4,17),(26,7),(28,26),(7,11),(8,26),(23,15),(2,10),(30,24),(19,18),(15,28),(10,5),(19,9),(5,6),(1,8),(4,30),(27,4),(13,16),(20,21),(12,19),(26,22),(7,16),(29,25),(18,14),(11,27),(24,19),(3,29),(0,12),(22,2),(25,10),(14,5),(21,0)];
        let mut dims = HashMap::<usize,(f32,f32)>::new();

        dims.insert(0,(178.0,139.375));
        dims.insert(1,(24.0,67.5));
        dims.insert(2,(24.0,67.5));
        dims.insert(3,(24.0,67.5));
        dims.insert(4,(52.0,89.0625));
        dims.insert(5,(52.0,125.0));
        dims.insert(6,(346.0,139.375));
        dims.insert(7,(52.0,132.1875));
        dims.insert(8,(108.0,96.25));
        dims.insert(9,(38.0,53.125));
        dims.insert(10,(38.0,67.5));
        dims.insert(11,(52.0,132.1875));
        dims.insert(12,(24.0,53.125));
        dims.insert(13,(24.0,67.5));
        dims.insert(14,(24.0,53.125));
        dims.insert(15,(24.0,67.5));
        dims.insert(16,(52.0,67.5));
        dims.insert(17,(234.0,132.1875));
        dims.insert(18,(24.0,67.5));
        dims.insert(19,(94.0,132.1875));
        dims.insert(20,(52.0,89.0625));
        dims.insert(21,(24.0,67.5));
        dims.insert(22,(94.0,96.25));
        dims.insert(23,(52.0,81.875));
        dims.insert(24,(206.0,132.1875));
        dims.insert(25,(24.0,67.5));
        dims.insert(26,(164.0,125.0));
        dims.insert(27,(38.0,89.0625));
        dims.insert(28,(94.0,96.25));
        dims.insert(29,(52.0,67.5));
        dims.insert(30,(38.0,89.0625));
        layout(&nodes,&edges,&dims,None,100,30);
    }

    // func_1130
    #[test]
    fn xcoord_computation() {
        let mut dims = HashMap::<AdjacencyListVertexDescriptor,(f32,f32)>::new();
        let mut rank = HashMap::new();  // Desc -> Rank
        let mut order: Vec<Vec<AdjacencyListVertexDescriptor>> = vec![];
        let mut graph = AdjacencyList::<usize,()>::new();

        let v15 = graph.add_vertex(15);
        let v11 = graph.add_vertex(11);
        let v17 = graph.add_vertex(17);
        let v5 = graph.add_vertex(5);
        let v16 = graph.add_vertex(16);
        let v12 = graph.add_vertex(12);
        let v25 = graph.add_vertex(25);
        let v7 = graph.add_vertex(7);
        let v1 = graph.add_vertex(1);
        let v8 = graph.add_vertex(8);
        let v14 = graph.add_vertex(14);
        let v9 = graph.add_vertex(9);
        let v13 = graph.add_vertex(13);
        let v19 = graph.add_vertex(19);
        let v21 = graph.add_vertex(21);
        let v23 = graph.add_vertex(23);
        let v0 = graph.add_vertex(0);
        let v10 = graph.add_vertex(10);
        let v26 = graph.add_vertex(26);
        let v22 = graph.add_vertex(22);
        let v20 = graph.add_vertex(20);
        let v24 = graph.add_vertex(24);
        let v6 = graph.add_vertex(6);
        let v18 = graph.add_vertex(18);
        let v4 = graph.add_vertex(4);
        let v3 = graph.add_vertex(3);
        let v2 = graph.add_vertex(2);
        graph.add_edge((),v15,v4);
        graph.add_edge((),v26,v12);
        graph.add_edge((),v1,v5);
        graph.add_edge((),v16,v25);
        graph.add_edge((),v21,v13);
        graph.add_edge((),v25,v12);
        graph.add_edge((),v7,v12);
        graph.add_edge((),v4,v16);
        graph.add_edge((),v10,v1);
        graph.add_edge((),v5,v18);
        graph.add_edge((),v24,v13);
        graph.add_edge((),v2,v8);
        graph.add_edge((),v23,v13);
        graph.add_edge((),v11,v14);
        graph.add_edge((),v6,v3);
        graph.add_edge((),v22,v2);
        graph.add_edge((),v16,v7);
        graph.add_edge((),v19,v24);
        graph.add_edge((),v3,v10);
        graph.add_edge((),v20,v15);
        graph.add_edge((),v18,v11);
        graph.add_edge((),v14,v23);
        graph.add_edge((),v0,v26);
        graph.add_edge((),v17,v20);
        graph.add_edge((),v11,v19);
        graph.add_edge((),v13,v2);
        graph.add_edge((),v6,v9);
        graph.add_edge((),v12,v22);
        graph.add_edge((),v19,v21);
        graph.add_edge((),v4,v0);
        graph.add_edge((),v9,v17);
        dims.insert(v21,(108.0,260.0));
        dims.insert(v3,(72.0,20.0));
        dims.insert(v12,(108.0,120.0));
        dims.insert(v19,(101.0,120.0));
        dims.insert(v9,(108.0,520.0));
        dims.insert(v8,(101.0,180.0));
        dims.insert(v17,(108.0,380.0));
        dims.insert(v2,(101.0,40.0));
        dims.insert(v4,(101.0,120.0));
        dims.insert(v6,(115.0,80.0));
        dims.insert(v13,(108.0,100.0));
        dims.insert(v1,(115.0,380.0));
        dims.insert(v11,(101.0,100.0));
        dims.insert(v18,(108.0,700.0));
        dims.insert(v14,(101.0,140.0));
        dims.insert(v15,(108.0,700.0));
        dims.insert(v5,(108.0,180.0));
        dims.insert(v10,(94.0,20.0));
        dims.insert(v7,(108.0,300.0));
        dims.insert(v0,(101.0,140.0));
        dims.insert(v16,(101.0,160.0));
        dims.insert(v20,(108.0,180.0));
        rank.insert(v5,4);
        rank.insert(v15,4);

        rank.insert(v12,8);
        rank.insert(v2,10);
        rank.insert(v1,3);
        rank.insert(v21,8);
        rank.insert(v23,8);
        rank.insert(v18,5);
        rank.insert(v17,2);
        rank.insert(v6,0);
        rank.insert(v0,6);
        rank.insert(v3,1);
        rank.insert(v14,7);
        rank.insert(v16,6);
        rank.insert(v22,9);
        rank.insert(v26,7);
        rank.insert(v7,7);
        rank.insert(v8,11);
        rank.insert(v13,9);
        rank.insert(v25,7);
        rank.insert(v9,1);
        rank.insert(v19,7);
        rank.insert(v20,3);
        rank.insert(v4,5);
        rank.insert(v11,6);
        rank.insert(v10,2);
        rank.insert(v24,8);
        order.push(vec![v6]);
        order.push(vec![v9, v3]);
        order.push(vec![v17, v10]);
        //order.push(vec![v20, v1]);
        order.push(vec![v1, v20]);
        order.push(vec![v15, v5]);
        //order.push(vec![v4, v18]);
        order.push(vec![v18, v4]);
        //order.push(vec![v16, v0, v11]);
        order.push(vec![v11, v16, v0]);
        //order.push(vec![v7, v25, v26, v14, v19]);
        order.push(vec![v25, v7, v14, v19, v26]);
        //order.push(vec![v12, v23, v24, v21]);
        order.push(vec![v12, v21, v23, v24]);
        //order.push(vec![v22, v13]);
        order.push(vec![v13, v22]);
        order.push(vec![v2]);
        order.push(vec![v8]);
        let virt_start = 22;//add_virtual_vertices(&mut rank,&mut graph);

        for o in order.iter() {
            println!("{:?}",o);
        }

        for e in graph.edges() {
            let from = graph.source(e);
            let to = graph.target(e);

            assert!(rank[&to] - rank[&from] <= 1);
        }

        let x_pos = compute_x_coordinates(&order,&rank,&mut graph,&dims,25,virt_start);

        let mut xxx = x_pos.iter().map(|e| (e.0,e.1)).collect::<Vec<(_,_)>>();
        xxx.sort_by(|a,b| (a.0).0.cmp(&(b.0).0));

        for e in xxx {
            println!("{:?}: pos x: {}",e.0,e.1);
        }

        let mut coll = HashSet::<(_,_)>::new();

        for (idx,r) in order.iter().enumerate() {
            for v in r.iter() {
                let v_lb = graph.vertex_label(*v).unwrap();
                let v_w = if *v_lb >= virt_start { 1.0 } else { dims[v].0 as f32 };
                let v_x = x_pos[v];

                for w in r.iter() {
                    if v != w {
                        let w_lb = graph.vertex_label(*w).unwrap();
                        let w_w = if *w_lb >= virt_start { 1.0 } else { dims[w].0 as f32 };
                        let w_x = x_pos[w];

                        if !(v_x + v_w / 2.0 < w_x - w_w / 2.0 || w_x + w_w / 2.0 < v_x - v_w / 2.0) {
                            if !coll.contains(&(*v,*w)) && !coll.contains(&(*w,*v)) {
                                println!("in rank {} overlap between {} and {}",idx,v_lb,w_lb);
                                println!("  {}: pos x: {} w: {}",v_lb,v_x,v_w);
                                println!("  {}: pos x: {} w: {}",w_lb,w_x,w_w);
                                coll.insert((*v,*w));
                            }
                        }
                    }
                }
            }
        }
    }
}
