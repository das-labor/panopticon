use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug,PartialEq,Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Debug,PartialEq,Clone)]
pub struct Segment {
    pub start: Point,
    pub end: Point
}

fn orientation(p: &Point, q: &Point, r: &Point) -> usize {
    if (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y) > 0.0 {
        1
    } else {
        2
    }
}

fn intersects(p: &Point, r: &Point, q: &Segment) -> bool {
    orientation(p,r,&q.start) != orientation(p,r,&q.end) &&
        orientation(&q.start,&q.end,p) != orientation(&q.start,&q.end,r)
}

fn close(p: &Point, q: &Point) -> bool {
    (p.x - q.x).abs() < 0.001 && (p.y - q.y).abs() < 0.001
}

pub fn visibility_graph(obstacles: &Vec<Segment>, points: &Vec<Point>) -> HashMap<usize,Vec<usize>> {
    let mut ret = HashMap::<usize,Vec<usize>>::new();
    let mut i_idx = 0;

    while i_idx < points.len() {
        let mut j_idx = 0;
        let i = &points[i_idx];

        while j_idx < points.len() {
            let j = &points[j_idx];
            let mut hit = false;

            for s in obstacles.iter() {
                if intersects(i,j,s) && !close(&s.start,i) && !close(&s.start,j) && !close(&s.end,i) && !close(&s.end,j) {
                    hit = true;
                    break;
                }
            }

            if !hit {
                ret.entry(i_idx).or_insert(vec![]).push(j_idx);
            }

            j_idx += 1;
        }

        i_idx += 1;
    }

    ret
}

pub fn dijkstra(start: usize, finish: usize, graph: &HashMap<usize,Vec<usize>>, points: &Vec<Point>) -> Vec<usize> {
    let mut dist = HashMap::<usize,f32>::new();
    let mut prev = HashMap::<usize,usize>::new();
    let mut q = Vec::<(f32,usize)>::new();

    q.push((0.0,start));

    while let Some((cost,u)) = q.pop() {
        if u == finish {
            let mut ret = vec![];
            let mut tmp = u;

            while tmp != start {
                ret.push(tmp);
                tmp = prev[&tmp];
            }
            ret.push(start);
            ret.reverse();

            return ret;
        }


        if let Some(n) = graph.get(&u) {
            for v in n.iter() {
                let a = points[u].x - points[*v].x;
                let b = points[u].y - points[*v].y;
                let alt = cost + (a*a + b*b).sqrt();
                if !dist.contains_key(v) || alt < dist[v] {
                    dist.insert(*v,alt);
                    prev.insert(*v,u);
                    q.push((alt,*v));
                    q.sort_by(|a,b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
                }
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let o = vec![
            Segment{ start: Point{ x: 0.0, y: 0.0 }, end: Point{ x: 0.0, y: 100.0 } },
            Segment{ start: Point{ x: 0.0, y: 100.0 }, end: Point{ x: 100.0, y: 100.0 } },
            Segment{ start: Point{ x: 100.0, y: 100.0 }, end: Point{ x: 100.0, y: 0.0 } },
            Segment{ start: Point{ x: 100.0, y: 0.0 }, end: Point{ x: 0.0, y: 0.0 } },

            Segment{ start: Point{ x: 250.0, y: 50.0 }, end: Point{ x: 250.0, y: 150.0 } },
            Segment{ start: Point{ x: 250.0, y: 150.0 }, end: Point{ x: 300.0, y: 150.0 } },
            Segment{ start: Point{ x: 300.0, y: 150.0 }, end: Point{ x: 300.0, y: 50.0 } },
            Segment{ start: Point{ x: 300.0, y: 50.0 }, end: Point{ x: 250.0, y: 50.0 } },
        ];

        let p = vec![
            Point{ x: -3.0, y: -3.0 },
            Point{ x: -3.0, y: 103.0 },
            Point{ x: 103.0, y: 103.0 },
            Point{ x: 103.0, y: -3.0 },
            Point{ x: 247.0, y: 47.0 },
            Point{ x: 247.0, y: 153.0 },
            Point{ x: 303.0, y: 153.0 },
            Point{ x: 303.0, y: 47.0 }
        ];

        assert_eq!(vec![1,2,4,7], dijkstra(1,7,&visibility_graph(&o,&p),&p));
    }
}
