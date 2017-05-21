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

mod order;
mod linear;
mod rank;

pub use self::linear::{
    linear_layout_placement,
    linear_layout_structural,
    LinearLayout,
};

#[cfg(test)]
mod tests {
    use std::{f32,isize,usize};
    use std::iter::FromIterator;
    use std::collections::{HashSet,HashMap};
    use graph_algos::{
        AdjacencyList,
        IncidenceGraphTrait,
        GraphTrait,
        MutableGraphTrait,
        VertexListGraphTrait,
        EdgeListGraphTrait
    };
    use graph_algos::adjacency_list::{
        AdjacencyListVertexDescriptor
    };
    use sugiyama::linear::{
        linear_layout,
        compute_x_coordinates,
    };
    use sugiyama::rank::{
        ensure_single_entry,
        remove_cycles,
        remove_loops,
        add_virtual_vertices,
    };

    #[test]
    fn test_remove_loops() {
        let mut graph = AdjacencyList::<usize,usize>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);
        let vx5 = graph.add_vertex(4);

        graph.add_edge(0,vx1,vx2);
        graph.add_edge(1,vx2,vx3);
        graph.add_edge(2,vx3,vx4);
        graph.add_edge(3,vx4,vx1);
        graph.add_edge(4,vx3,vx5);

        let e11 = graph.add_edge(5,vx1,vx1).unwrap();
        let e22 = graph.add_edge(6,vx2,vx2).unwrap();
        let e22b = graph.add_edge(7,vx2,vx2).unwrap();

        remove_loops(&mut graph);

        assert_eq!(graph.num_vertices(), 5);
        assert_eq!(graph.num_edges(), 5);

        assert_eq!(graph.edge_label(e11),None);
        assert_eq!(graph.edge_label(e22),None);
        assert_eq!(graph.edge_label(e22b),None);
    }

    #[test]
    fn test_single_entry() {
        let mut graph = AdjacencyList::<usize,usize>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);
        let vx5 = graph.add_vertex(4);

        graph.add_edge(1,vx1,vx2);
        graph.add_edge(2,vx2,vx3);
        graph.add_edge(3,vx2,vx4);
        graph.add_edge(4,vx3,vx5);
        graph.add_edge(5,vx4,vx5);

        assert_eq!(ensure_single_entry(Some(&vx1),&mut graph), vx1);
    }

    #[test]
    fn test_multi_entry() {
        let mut graph = AdjacencyList::<usize,usize>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);
        let vx5 = graph.add_vertex(4);
        let vx6 = graph.add_vertex(5);

        graph.add_edge(1,vx1,vx2);
        graph.add_edge(2,vx2,vx3);
        graph.add_edge(3,vx2,vx4);
        graph.add_edge(4,vx4,vx5);
        graph.add_edge(5,vx3,vx5);
        graph.add_edge(6,vx6,vx3);

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
        let mut graph = AdjacencyList::<usize,usize>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);

        graph.add_edge(1,vx1,vx2);
        graph.add_edge(2,vx2,vx3);
        graph.add_edge(3,vx4,vx2);

        remove_cycles(&vx1,&mut graph);

        assert_eq!(graph.num_vertices(), 4);
        assert_eq!(graph.num_edges(), 3);
    }

    #[test]
    fn virtual_vertex_insertion() {
        let mut graph = AdjacencyList::<usize,usize>::new();

        let vx1 = graph.add_vertex(0);
        let vx2 = graph.add_vertex(1);
        let vx3 = graph.add_vertex(2);
        let vx4 = graph.add_vertex(3);

        graph.add_edge(1,vx1,vx2);
        graph.add_edge(2,vx1,vx3);
        graph.add_edge(3,vx1,vx4);

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

        assert!(linear_layout(&nodes,&edges,&dims,None,100.,30.,2.,30.,30.,10.).ok().is_some());
    }

    // func_1130
    #[test]
    fn xcoord_computation() {
        let mut dims = HashMap::<AdjacencyListVertexDescriptor,(f32,f32)>::new();
        let mut rank = HashMap::new();  // Desc -> Rank
        let mut order: Vec<Vec<AdjacencyListVertexDescriptor>> = vec![];
        let mut graph = AdjacencyList::<usize,usize>::new();

        let v0 = graph.add_vertex(0);
        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);
        let v4 = graph.add_vertex(4);
        let v5 = graph.add_vertex(5);
        let v6 = graph.add_vertex(6);
        let v7 = graph.add_vertex(7);
        let v8 = graph.add_vertex(8);
        let v9 = graph.add_vertex(9);
        let v10 = graph.add_vertex(10);
        let v11 = graph.add_vertex(11);
        let v12 = graph.add_vertex(12);
        let v13 = graph.add_vertex(13);
        let v14 = graph.add_vertex(14);
        let v15 = graph.add_vertex(15);
        let v16 = graph.add_vertex(16);
        let v17 = graph.add_vertex(17);
        let v18 = graph.add_vertex(18);
        let v19 = graph.add_vertex(19);
        let v22 = graph.add_vertex(22);
        let v23 = graph.add_vertex(23);

        graph.add_edge(0,v1,v23);
        graph.add_edge(0,v3,v14);
        graph.add_edge(0,v12,v15);
        graph.add_edge(0,v1,v5);
        graph.add_edge(0,v10,v17);
        graph.add_edge(0,v22,v6);
        graph.add_edge(0,v0,v16);
        graph.add_edge(0,v3,v1);
        graph.add_edge(0,v8,v9);
        graph.add_edge(0,v15,v2);
        graph.add_edge(0,v19,v22);
        graph.add_edge(0,v11,v4);
        graph.add_edge(0,v13,v10);
        graph.add_edge(0,v8,v7);
        graph.add_edge(0,v4,v10);
        graph.add_edge(0,v16,v11);
        graph.add_edge(0,v16,v12);
        graph.add_edge(0,v5,v6);
        graph.add_edge(0,v14,v18);
        graph.add_edge(0,v17,v1);
        graph.add_edge(0,v18,v6);
        graph.add_edge(0,v23,v19);
        graph.add_edge(0,v10,v3);
        graph.add_edge(0,v11,v13);
        graph.add_edge(0,v6,v8);

        dims.insert(v0,(101.0,20.0));
        dims.insert(v1,(72.0,20.0));
        dims.insert(v2,(79.0,40.0));
        dims.insert(v3,(108.0,20.0));
        dims.insert(v4,(94.0,20.0));
        dims.insert(v5,(94.0,20.0));
        dims.insert(v6,(101.0,120.0));
        dims.insert(v7,(115.0,340.0));
        dims.insert(v8,(108.0,60.0));
        dims.insert(v9,(115.0,340.0));
        dims.insert(v10,(108.0,20.0));
        dims.insert(v11,(79.0,20.0));
        dims.insert(v12,(79.0,80.0));
        dims.insert(v13,(101.0,40.0));
        dims.insert(v14,(115.0,340.0));
        dims.insert(v15,(37.0,20.0));
        dims.insert(v16,(115.0,180.0));
        dims.insert(v17,(72.0,20.0));
        dims.insert(v18,(0.,0.));
        dims.insert(v19,(0.,0.));
        dims.insert(v22,(0.,0.));
        dims.insert(v23,(0.,0.));

        rank.insert(v0,0);
        rank.insert(v1,6);
        rank.insert(v2,4);
        rank.insert(v3,5);
        rank.insert(v4,3);
        rank.insert(v5,7);
        rank.insert(v6,8);
        rank.insert(v7,10);
        rank.insert(v8,9);
        rank.insert(v9,10);
        rank.insert(v10,4);
        rank.insert(v11,2);
        rank.insert(v12,2);
        rank.insert(v13,3);
        rank.insert(v14,6);
        rank.insert(v15,3);
        rank.insert(v16,1);
        rank.insert(v17,5);
        rank.insert(v18,7);
        rank.insert(v19,7);
        rank.insert(v22,8);
        rank.insert(v23,6);

        order.push(vec![v0]);
        order.push(vec![v16]);
        order.push(vec![v12, v11]);
        order.push(vec![v15, v13, v4]);
        order.push(vec![v2, v10]);
        order.push(vec![v3, v17]);
        order.push(vec![v1, v23, v14]);
        order.push(vec![v5, v19, v18]);
        order.push(vec![v6, v22]);
        order.push(vec![v8]);
        order.push(vec![v7, v9]);

        let virt_start = 18;//add_virtual_vertices(&mut rank,&mut graph);

        for o in order.iter() {
            println!("{:?}",o);
        }

        for e in graph.edges() {
            let from = graph.source(e);
            let to = graph.target(e);

            assert!(rank[&to] - rank[&from] <= 1);
        }

        let port_offset = HashMap::from_iter(graph.edges().map(|e| (e,(0.,0.))));
        let x_pos = compute_x_coordinates(&order,&rank,&mut graph,&dims,&port_offset,&|_| 25.,virt_start);
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

    #[test]
    fn xcoord_computation_paper() {
        let mut rank = HashMap::new();  // Desc -> Rank
        let mut order: Vec<Vec<AdjacencyListVertexDescriptor>> = vec![];
        let mut graph = AdjacencyList::<usize,usize>::new();
        let v = (0..58).into_iter().map(|n| graph.add_vertex(n)).collect::<Vec<_>>();
        let dims = HashMap::from_iter(v.iter().map(|&vx| (vx,(20f32,20f32))));
        let virt_start = 24;

        // Rank 0
        order.push(vec![v[0]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[0],v[1]);
        graph.add_edge(0,v[0],v[2]);

        // Rank 1
        order.push(vec![v[1],v[2]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[1],v[24]);
        graph.add_edge(0,v[1],v[25]);
        graph.add_edge(0,v[1],v[26]);
        graph.add_edge(0,v[1],v[3]);
        graph.add_edge(0,v[2],v[3]);
        graph.add_edge(0,v[2],v[27]);

        // Rank 2
        order.push(vec![v[24],v[25],v[26],v[3],v[27]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[24],v[28]);
        graph.add_edge(0,v[25],v[29]);
        graph.add_edge(0,v[26],v[4]);
        graph.add_edge(0,v[3],v[5]);
        graph.add_edge(0,v[3],v[30]);
        graph.add_edge(0,v[27],v[31]);

        // Rank 3
        order.push(vec![v[28],v[29],v[4],v[5],v[30],v[31]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[28],v[32]);
        graph.add_edge(0,v[29],v[33]);
        graph.add_edge(0,v[4],v[6]);
        graph.add_edge(0,v[5],v[7]);
        graph.add_edge(0,v[30],v[34]);
        graph.add_edge(0,v[31],v[35]);

        // Rank 4
        order.push(vec![v[32],v[33],v[6],v[7],v[34],v[35]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[32],v[36]);
        graph.add_edge(0,v[33],v[37]);
        graph.add_edge(0,v[6],v[8]);
        graph.add_edge(0,v[6],v[38]);
        graph.add_edge(0,v[6],v[39]);
        graph.add_edge(0,v[7],v[9]);
        graph.add_edge(0,v[34],v[40]);
        graph.add_edge(0,v[35],v[41]);

        // Rank 5
        order.push(vec![v[36],v[37],v[8],v[38],v[39],v[9],v[40],v[41]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[36],v[42]);
        graph.add_edge(0,v[37],v[43]);
        graph.add_edge(0,v[8],v[10]);
        graph.add_edge(0,v[8],v[11]);
        graph.add_edge(0,v[38],v[44]);
        graph.add_edge(0,v[39],v[45]);
        graph.add_edge(0,v[9],v[12]);
        graph.add_edge(0,v[40],v[46]);
        graph.add_edge(0,v[41],v[47]);

        // Rank 6
        order.push(vec![v[42],v[43],v[10],v[11],v[44],v[45],v[12],v[46],v[47]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[42],v[13]);
        graph.add_edge(0,v[43],v[48]);
        graph.add_edge(0,v[10],v[14]);
        graph.add_edge(0,v[10],v[15]);
        graph.add_edge(0,v[11],v[15]);
        graph.add_edge(0,v[11],v[16]);
        graph.add_edge(0,v[44],v[16]);
        graph.add_edge(0,v[45],v[49]);
        graph.add_edge(0,v[12],v[50]);
        graph.add_edge(0,v[46],v[51]);
        graph.add_edge(0,v[47],v[52]);

        // Rank 7
        order.push(vec![v[13],v[48],v[14],v[15],v[16],v[49],v[50],v[51],v[52]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[13],v[17]);
        graph.add_edge(0,v[48],v[53]);
        graph.add_edge(0,v[14],v[17]);
        graph.add_edge(0,v[14],v[18]);
        graph.add_edge(0,v[16],v[18]);
        graph.add_edge(0,v[16],v[19]);
        graph.add_edge(0,v[16],v[20]);
        graph.add_edge(0,v[49],v[54]);
        graph.add_edge(0,v[50],v[20]);
        graph.add_edge(0,v[51],v[55]);
        graph.add_edge(0,v[52],v[20]);

        // Rank 8
        order.push(vec![v[17],v[53],v[18],v[19],v[54],v[20],v[55]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[53],v[21]);
        graph.add_edge(0,v[18],v[21]);
        graph.add_edge(0,v[19],v[22]);
        graph.add_edge(0,v[54],v[56]);
        graph.add_edge(0,v[55],v[57]);

        // Rank 9
        order.push(vec![v[21],v[22],v[56],v[57]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));
        graph.add_edge(0,v[21],v[23]);
        graph.add_edge(0,v[22],v[23]);
        graph.add_edge(0,v[56],v[23]);
        graph.add_edge(0,v[57],v[23]);

        // Rank 10
        order.push(vec![v[23]]);
        rank.extend(order.last().unwrap().iter().map(|&vx| (vx,order.len() as isize - 1)));

        let mut ports = HashMap::from_iter(graph.edges().map(|e| (e,(0f32,0f32))));
        for vx in graph.vertices() {
            let deg = graph.out_degree(vx);

            if deg > 1 {
                let off = dims[&vx].0 / (deg + 1) as f32;
                for (i,e) in graph.out_edges(vx).enumerate() {
                    ports.insert(e,((i + 1) as f32 * off,0.));
                }
            }
        }


        compute_x_coordinates(&order,&rank,&mut graph,&dims,&ports,&|_| 25.,virt_start);
    }

    #[test]
    fn layout_all() {
        use std::path::Path;
        use panopticon::{
            loader,
            Machine,
            amd64,
            pipeline,
        };
        use futures::Stream;

        let _ = ::env_logger::init();

        for path in &["tests/data/static","tests/data/libbeef.dll","tests/data/libbeef.dylib"] {
            if let Ok((mut proj,machine)) = loader::load(&Path::new(path)) {
                let maybe_prog = proj.code.pop();
                let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap().clone();

                if let Some(prog) = maybe_prog {
                    let pipe: Box<_> = match machine {
                        Machine::Amd64 => pipeline::<amd64::Amd64>(prog,reg.clone(),amd64::Mode::Long),
                        Machine::Ia32 => pipeline::<amd64::Amd64>(prog,reg.clone(),amd64::Mode::Protected),
                        _ => unreachable!()
                    };

                    for i in pipe.wait() {
                        if let Ok(func) = i {
                            let cfg = &func.cflow_graph;
                            let vertices = HashMap::<AdjacencyListVertexDescriptor,usize>::from_iter(cfg.vertices().enumerate().map(|(idx,vx)| (vx,idx)));
                            let entry = func.entry_point.map(|x| vertices[&x]);
                            let edges: Vec<(usize,usize)> = cfg.edges().map(|e| {
                                (vertices[&cfg.source(e)],vertices[&cfg.target(e)])
                            }).collect();
                            let dims = HashMap::from_iter(vertices.iter().map(|(_,&x)| (x,(42.,23.))));

                            linear_layout(
                                &vertices.into_iter().map(|(_,x)| x).collect(),
                                &edges,
                                &dims,
                                entry,
                                5.,10.,2.,5.,5.,120.).unwrap();
                        } else {
                            unreachable!();
                        }
                    }
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }
    }
}
