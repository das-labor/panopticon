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
    normalize_rank(&mut rank);

    let mut order = initial_ordering(&rank,&head,&graph);
    optimize_ordering(&mut order,&rank,&graph);

    let x_pos = compute_x_coordinates(&order,&rank,&graph,&dims,node_spacing,virt_start);
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

    depth_first_visit(&mut |vx,_| {
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
        let prev = (i + delta) as usize;

        for l1 in 0..order[prev].len() {
            let v = order[prev][l1];
            let mut edges = if up_to_down { graph.in_edges(v) } else { graph.out_edges(v) };
            let maybe_upn = edges.find(|&e| {
                let w = if up_to_down { graph.source(e) } else { graph.target(e) };
                rank[&w] == i as isize && *graph.vertex_label(w).unwrap() >= virt_start
            });

            if l1 == order[prev].len() || maybe_upn.is_some() {
                let k1 = if let Some(upn) = maybe_upn {
                    order[i as usize].iter().position(|&x| {
                        (up_to_down && x == graph.source(upn)) &&
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
            assert!(i - delta >= 0 && i - delta < order.len() as isize);

            let prev = (i - delta) as usize;
            let v = order[i as usize][k as usize];
            let upn = order[prev].iter().filter(|&&w| graph.out_edges(w).any(|e| graph.target(e) == v)).cloned().collect::<Vec<_>>();

            if !upn.len() == 0 {
                for m in vec![(upn.len() as f32 / 2.0).floor(),(upn.len() as f32 / 2.0).ceil()] {
                    if align[&v] == v {
                        let um = upn[m as usize];
                        let pos = order[prev].iter().position(|&x| x == um).unwrap() as isize;

                        if !type1.contains(&(um,v)) &&
                            !type1.contains(&(v,um)) &&
                            ((left_to_right && r < pos) || (!left_to_right && pos < r)) {

                            let a = root[&um];
                            let b = root[&v];

                            align.insert(um,v);
                            root.insert(v,a);
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

    let delta: isize = if left_to_right { -1isize } else { 1 };

    if !x.contains_key(&v) {
        x.insert(v,0);
        let mut w = v;

        loop {
            let word: &Vec<AdjacencyListVertexDescriptor> = &order[rank[&w] as usize];

            if (left_to_right && *word.first().unwrap() != w) ||
                (!left_to_right && *word.last().unwrap() != w) {

                let uidx = word.iter().position(|&x| x == w).unwrap() as isize + delta;
                let u = root[&word[uidx as usize]];

                place_block(u,order,rank,sink,shift,x,align,root,widths,graph,node_spacing,left_to_right);

                if sink[&v] == v {
                    sink.insert(v,u);
                }

                let sep: isize = ((widths[&u] as isize +
                                   widths[&v] as isize) / 2) as isize;

                if sink[&v] != u {
                    let sinku = sink[&u].clone();
                    let prev = shift[&sinku];

                    if left_to_right {
                        shift.insert(sinku,min(prev,x[&v] - x[&u] - node_spacing as isize - sep));
                    } else {
                        shift.insert(sinku,max(prev,x[&v] - x[&u] + node_spacing as isize + sep));
                    }
                } else {
                    let val = if left_to_right {
                        max(x[&v],x[&u] + node_spacing as isize + sep)
                    } else {
                        min(x[&v],x[&u] - node_spacing as isize - sep)
                    };

                    x.insert(v,val);
                }
            }

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

    for v in graph.vertices() {
        let val = x[&root[&v]];
        x.insert(v,val);

        if shift[&sink[&root[&v]]] < isize::MAX {
            let val = x[&v] + shift[&sink[&root[&v]]];
            x.insert(v,val);
        }
    }

    x
}

fn compute_x_coordinates(order: &Vec<Vec<AdjacencyListVertexDescriptor>>,
                         rank: &HashMap<AdjacencyListVertexDescriptor,isize>,
                         graph: &AdjacencyList<usize,()>,
                         dims: &HashMap<usize,(f32,f32)>,
                         node_spacing: usize,
                         virt_start: usize) -> HashMap<AdjacencyListVertexDescriptor,f32> {

    let type1 = mark_type1_conflicts(virt_start,&order,&rank,&graph,false);
    let mut root = vec![];
    let mut align = vec![];
    let mut x = vec![];
    let mut widths = vec![];

    for k in (0..4) {
        let up_to_down = k <= 1;
        let left_to_right = k % 2 == 1;
        let (r,a) = vertical_alignment(&order,&type1,&graph,up_to_down,left_to_right);
        let mut w = HashMap::<AdjacencyListVertexDescriptor,usize>::new();

        for v in graph.vertices() {
            let _r = r[&v];

            if w.contains_key(&_r) {
                let val = w[&_r];
                w.insert(_r,max(val,dims[graph.vertex_label(v).unwrap()].0 as usize));
            } else {
                w.insert(_r,dims[graph.vertex_label(v).unwrap()].0 as usize);
            }
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
        if k % 2 == 1 {
            shift.push(min[global_min] - min[k]);
        } else {
            shift.push(max[global_min] - max[k]);
        }
    }

    let mut ret = HashMap::<AdjacencyListVertexDescriptor,f32>::new();

    for v in graph.vertices() {
        let mut sort = (0..4).map(|i| x[i][&v] as f32 + shift[i]).collect::<Vec<_>>();
        sort.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        ret.insert(v,0.5f32 * (sort[1] + sort[2]));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
