use crate::lib::graph2::Graph;
use std::cmp::Reverse;

pub(crate) mod part1;
pub(crate) mod part2;

impl Graph {
    // Single-source shortest paths on a directed graph with non-negative weights
    pub fn dijkstra(&self, weights: &[usize], u: usize) -> Vec<usize> {
        assert_eq!(self.num_e(), weights.len());
        let mut dist = vec![usize::MAX; weights.len()];
        let mut heap = std::collections::BinaryHeap::new();

        dist[u] = 0;
        heap.push((Reverse(0), 0));
        while let Some((Reverse(dist_u), u)) = heap.pop() {
            if dist[u] == dist_u {
                for (e, v) in self.adj_list(u) {
                    let dist_v = dist_u + weights[e];
                    if dist[v] > dist_v {
                        dist[v] = dist_v;
                        heap.push((Reverse(dist_v), v));
                    }
                }
            }
        }
        dist
    }
}
