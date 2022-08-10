use crate::lib::graph2::{Graph, VertexIndex};
use std::cmp::Reverse;

pub(crate) mod part1;
pub(crate) mod part2;

pub struct ShortestPath {
    graph: Graph,
    end: VertexIndex,
    start: VertexIndex,
    weights: Vec<usize>,
}

impl ShortestPath {
    // Implementation of Dijkstra's algorithm for shortest path
    // Could maybe be optimized to never path beyond the end node, but would be small improvement
    pub fn calculate_cost(&self) -> usize {
        let mut costs = vec![usize::MAX; self.graph.num_v()];
        let mut heap = std::collections::BinaryHeap::new();

        costs[self.start] = 0;
        heap.push((Reverse(0), self.start));

        while let Some((Reverse(cost), u)) = heap.pop() {
            if costs[u] == cost {
                for (e, to) in self.graph.adj_list(u) {
                    let dist_to = cost + self.weights[e];
                    if costs[to] > dist_to {
                        costs[to] = dist_to;
                        heap.push((Reverse(dist_to), to));
                    }
                }
            }
        }
        // Return cost at the end position
        costs[self.end]
    }
}
