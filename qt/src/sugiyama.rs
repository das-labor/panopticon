use std::collections::{HashSet,HashMap};
use std::isize;
use std::ptr;
use std::cmp::max;
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
              dims: &HashMap<usize,(f32,f32)>,
              entry: Option<usize>,
              node_spacing: usize,
              rank_spacing: usize) -> HashMap<usize,(f32,f32)> {
    let mut graph = AdjacencyList::<usize,()>::new();
    let mut rev = HashMap::<usize,AdjacencyListVertexDescriptor>::new();
    let mut maybe_entry = None;

    for &n in vertices.iter() {
        rev.insert(n,graph.add_vertex(n));
        if entry == Some(n) {
            maybe_entry = Some(rev[&n].clone());
        }
        assert!(dims.contains_key(&n));
    }

    for e in edges.iter() {
        graph.add_edge((),rev[&e.0],rev[&e.1]);
    }

    // normalize graph to DAG with single entry "head"
    let head = ensure_single_entry(maybe_entry.as_ref(),&mut graph);

    remove_cycles(&head,&mut graph);
    remove_loops(&mut graph);

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


    add_virtual_vertices(&mut rank,&mut graph);
    normalize_rank(&mut rank);

    let mut order = initial_ordering(&rank,&head,&graph);
    optimize_ordering(&mut order,&rank,&graph);

    let placing = {
        let (a,b,c,lb,ub) = build_placing_integer_program(&order,&dims,node_spacing as usize,&graph);
        solve_integer_program(&a,&b,&c,&lb,&ub).iter().take(graph.num_vertices()).cloned().collect::<Vec<isize>>()
    };

    let mut x_pos = HashMap::new();
    for vx in graph.vertices() {
        let lb = *graph.vertex_label(vx).unwrap();
        x_pos.insert(vx,placing[lb]);
    }

    let rank_offsets = order.iter()
        .map(|r| r.iter().fold(0usize,|acc,vx| max(dims.get(graph.vertex_label(*vx).unwrap()).map(|x| x.1).unwrap_or(0.0) as usize,acc)))
        .fold(vec![0usize],|acc,x| { let mut ret = acc.clone(); ret.push(acc.last().unwrap() + x + (rank_spacing as usize)); ret });

    let mut ret = HashMap::new();
    for n in vertices.iter() {
        let vx = rev[n];
        let r = rank[&vx] as usize;
        let rank_start = rank_offsets[r] as f32;
        let rank_end = rank_offsets[r + 1] as f32;

        ret.insert(*n,(x_pos[&vx] as f32,(rank_start + ((rank_end - rank_start) / 2.0)) as f32));
    }

    ret
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

#[derive(PartialEq,Eq)]
enum EdgeKind {
    Tree,
    Forward,
    Backward,
    Cross
}

fn depth_first_visit(vertex_visitor: &mut FnMut(&AdjacencyListVertexDescriptor),
                     edge_visitor: &mut FnMut(&AdjacencyListEdgeDescriptor,EdgeKind),
                     start: &AdjacencyListVertexDescriptor,
                     graph: &AdjacencyList<usize,()>) {
    let mut seen = HashMap::new();
    let mut stack = vec![(start.clone(),0)];

    while !stack.is_empty() {
        let (vx,num) = stack.pop().unwrap().clone();

        vertex_visitor(&vx);
        seen.insert(vx,num);

        for out in graph.out_edges(vx) {
            let s = graph.target(out);

            if let Some(other_num) = seen.get(&s) {
                if *other_num > num {
                    edge_visitor(&out,EdgeKind::Forward);
                } else if *other_num == num {
                    edge_visitor(&out,EdgeKind::Cross);
                } else {
                    edge_visitor(&out,EdgeKind::Backward);
                }
            } else {
                edge_visitor(&out,EdgeKind::Tree);
                stack.push((s,num+1));
            }
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

    depth_first_visit(&mut |_| {},&mut |e,k| if k == EdgeKind::Backward { to_flip.push(e.clone()) },head,graph);

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

pub fn add_virtual_vertices(rank: &mut HashMap<AdjacencyListVertexDescriptor,isize>,graph: &mut AdjacencyList<usize,()>) {
    let to_replace = graph.edges().filter(|&e| {
        let rank_from = rank.get(&graph.source(e)).unwrap();
        let rank_to = rank.get(&graph.target(e)).unwrap();

        assert!(rank_from <= rank_to);

        rank_to - rank_from > 1
    }).collect::<Vec<_>>();

    let mut next_label = graph.vertices().filter_map(|vx| graph.vertex_label(vx)).max().unwrap() + 1;
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

    depth_first_visit(&mut |vx| {
        let r = rank[vx];

        assert!(r >= 0);

        while ret.len() as isize <= r {
            ret.push(Vec::new());
        }

        ret[r as usize].push(*vx)
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
        let e1_start_rank = rank[&e1src] as usize;
        let e1_end_rank = rank[&e1tgt] as usize;

        for e2 in graph.edges() {
            let e2src = graph.source(e2);
            let e2tgt = graph.target(e2);
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
    let m = (p.len() as f32 / 2.0).floor() as usize;

    if p.len() == 0 {
        -1.0
    } else if p.len() % 2 == 1 {
        p[m] as f32
    } else if p.len() == 2 {
        (p[0] + p[1]) as f32 / 2.0
    } else {
        let left = (p[m-1] - p[0]) as f32;
        let right = (p.last().unwrap() - p[m]) as f32;

		((p[m-1] as f32) * right + (p[m] as f32) * left) / (left + right)
    }
}

fn wmedian(iter: usize,
           order: &mut Vec<Vec<AdjacencyListVertexDescriptor>>,
           rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
           graph: &AdjacencyList<usize,()>) {
    let dir = iter % 2 == 0; // true -> torwards higher ranks
    let mut rank_idx = if dir { 0 } else { order.len() - 1 } as usize;

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

fn build_placing_integer_program(order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                                 dims: &HashMap<usize,(f32,f32)>,
                                 padding: usize,
                                 graph: &AdjacencyList<usize,()>) -> (Vec<Vec<isize>>,Vec<isize>,Vec<isize>,Vec<isize>,Vec<isize>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let c = (0..graph.num_vertices()).map(|_| 0).chain((0..(2*graph.num_vertices())).map(|_| 1).chain((0..graph.num_vertices()).map(|_| 0))).collect::<Vec<_>>();
    let lb = (0..(4*graph.num_vertices())).map(|_| 0).collect::<Vec<_>>();
    let ub = (0..(4*graph.num_vertices())).map(|_| isize::MAX).collect::<Vec<_>>();

    for (e_idx,e) in graph.edges().enumerate() {
        let mut a_row = (0..(4*graph.num_vertices())).map(|_| 0).collect::<Vec<_>>();
        let from_vx_idx = *graph.vertex_label(graph.source(e)).unwrap();
        let to_vx_idx = *graph.vertex_label(graph.target(e)).unwrap();
        let xab1_vx_idx = graph.num_vertices() + e_idx;
        let xab2_vx_idx = 2*graph.num_vertices() + e_idx;

        a_row[from_vx_idx] = -1;
        a_row[to_vx_idx] = 1;
        a_row[xab1_vx_idx] = 1;
        a_row[xab2_vx_idx] = -1;

        a.push(a_row);
        b.push(0);
    }

    for o in order.iter() {
        for o_idx in (0..(o.len()-1)) {
            let mut a_row = (0..(4*graph.num_vertices())).map(|_| 0).collect::<Vec<_>>();
            let left_vx = o[o_idx];
            let right_vx = o[o_idx + 1];
            let left_vx_idx = *graph.vertex_label(left_vx).unwrap();
            let right_vx_idx = *graph.vertex_label(right_vx).unwrap();
            let lr_cost_idx = 3*graph.num_vertices() + o_idx;

            a_row[left_vx_idx] = -1;
            a_row[right_vx_idx] = 1;
            a_row[lr_cost_idx] = -1;

            a.push(a_row);

            let left_w = dims.get(&left_vx_idx).map(|x| x.0).unwrap_or(0.0) as usize;
            let right_w = dims.get(&right_vx_idx).map(|x| x.0).unwrap_or(0.0) as usize;
            b.push(((left_w / 2) + (right_w / 2) + padding) as isize);
        }
    }

    (a,b,c,lb,ub)
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

#[cfg(test)]
mod tests {
    use super::*;
    use qmlrs::Variant;
    use graph_algos::{AdjacencyList,GraphTrait,MutableGraphTrait};
    use graph_algos::{VertexListGraphTrait,EdgeListGraphTrait};
    use std::collections::HashMap;

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
/* XXX
    #[test]
    fn large_graph() {
        let json = Variant::String("{\"dimensions\":{\"0\":{\"height\":178,\"width\":139.375},\"1\":{\"height\":24,\"width\":67.5},\"2\":{\"height\":24,\"width\":67.5},\"3\":{\"height\":24,\"width\":67.5},\"4\":{\"height\":52,\"width\":89.0625},\"5\":{\"height\":52,\"width\":125},\"6\":{\"height\":346,\"width\":139.375},\"7\":{\"height\":52,\"width\":132.1875},\"8\":{\"height\":108,\"width\":96.25},\"9\":{\"height\":38,\"width\":53.125},\"10\":{\"height\":38,\"width\":67.5},\"11\":{\"height\":52,\"width\":132.1875},\"12\":{\"height\":24,\"width\":53.125},\"13\":{\"height\":24,\"width\":67.5},\"14\":{\"height\":24,\"width\":53.125},\"15\":{\"height\":24,\"width\":67.5},\"16\":{\"height\":52,\"width\":67.5},\"17\":{\"height\":234,\"width\":132.1875},\"18\":{\"height\":24,\"width\":67.5},\"19\":{\"height\":94,\"width\":132.1875},\"20\":{\"height\":52,\"width\":89.0625},\"21\":{\"height\":24,\"width\":67.5},\"22\":{\"height\":94,\"width\":96.25},\"23\":{\"height\":52,\"width\":81.875},\"24\":{\"height\":206,\"width\":132.1875},\"25\":{\"height\":24,\"width\":67.5},\"26\":{\"height\":164,\"width\":125},\"27\":{\"height\":38,\"width\":89.0625},\"28\":{\"height\":94,\"width\":96.25},\"29\":{\"height\":52,\"width\":67.5},\"30\":{\"height\":38,\"width\":89.0625}},\"edges\":[{\"from\":22,\"to\":13},{\"from\":1,\"to\":23},{\"from\":23,\"to\":3},{\"from\":27,\"to\":16},{\"from\":20,\"to\":27},{\"from\":17,\"to\":12},{\"from\":30,\"to\":20},{\"from\":9,\"to\":25},{\"from\":16,\"to\":25},{\"from\":4,\"to\":17},{\"from\":26,\"to\":7},{\"from\":28,\"to\":26},{\"from\":7,\"to\":11},{\"from\":8,\"to\":26},{\"from\":23,\"to\":15},{\"from\":2,\"to\":10},{\"from\":30,\"to\":24},{\"from\":19,\"to\":18},{\"from\":15,\"to\":28},{\"from\":10,\"to\":5},{\"from\":19,\"to\":9},{\"from\":5,\"to\":6},{\"from\":1,\"to\":8},{\"from\":4,\"to\":30},{\"from\":27,\"to\":4},{\"from\":13,\"to\":16},{\"from\":20,\"to\":21},{\"from\":12,\"to\":19},{\"from\":26,\"to\":22},{\"from\":7,\"to\":16},{\"from\":29,\"to\":25},{\"from\":18,\"to\":14},{\"from\":11,\"to\":27},{\"from\":24,\"to\":19},{\"from\":3,\"to\":29},{\"from\":0,\"to\":12},{\"from\":22,\"to\":2},{\"from\":25,\"to\":10},{\"from\":14,\"to\":5},{\"from\":21,\"to\":0}],\"node_spacing\":30,\"nodes\":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30],\"rank_spacing\":100,\"entry\":1}".to_string());
        layout(&json);
    }*/
}
