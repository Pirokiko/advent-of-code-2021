use crate::lib::graph2::{Graph, VertexIndex};
use std::cmp::Reverse;

pub(crate) mod part1;
pub(crate) mod part2;

#[derive(Clone, Debug)]
struct Step {
    from: VertexIndex,
    to: VertexIndex,
    weight: usize,
    // totalCost: usize,
}
// type Path = Vec<Step>;

#[derive(Clone, Debug)]
struct Path {
    steps: Vec<Step>,
    cost: usize,
}

impl Path {
    pub fn new() -> Path {
        Path {
            steps: vec![],
            cost: 0,
        }
    }
    pub fn cost(&self) -> usize {
        self.cost
    }

    pub fn push(&mut self, step: Step) {
        self.cost += step.weight;
        self.steps.push(step);
    }

    pub fn pop(&mut self) {
        let step = self.steps.pop();
        match step {
            None => {}
            Some(s) => self.cost -= s.weight,
        }
    }
}

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

    pub fn shortest_path(&self, start: VertexIndex, end: VertexIndex) -> ShortestPath {
        ShortestPath::new(self, start, end)
    }
}

struct ShortestPath<'graph> {
    graph: &'graph Graph,
    end: VertexIndex,
    start: VertexIndex,
    visited: Vec<VertexIndex>,
    costs: Vec<VertexIndex>,
    path: Path,
    result: Option<Path>,
}

impl<'graph> ShortestPath<'graph> {
    pub fn new(graph: &'graph Graph, start: VertexIndex, end: VertexIndex) -> ShortestPath {
        ShortestPath {
            graph,
            start,
            end,
            visited: vec![],
            costs: vec![usize::MAX; graph.num_v()],
            path: Path::new(),
            result: None,
        }
    }
}

impl<'graph> ShortestPath<'graph> {
    pub fn calculate(&mut self, weights: &[usize]) -> Option<Path> {
        self.pathing(weights, self.start);

        match &self.result {
            None => None,
            Some(path) => Some(path.clone()),
        }
    }

    fn pathing(&mut self, weights: &[usize], at: VertexIndex) {
        if at == self.end {
            if self.result.is_none() || self.path.cost < self.result.as_ref().unwrap().cost {
                self.result = Some(self.path.clone())
            }
            return;
        }

        self.visited.push(at);
        self.costs[at] = self.path.cost;
        for (e, to) in self.graph.adj_list(at) {
            if !self.visited.contains(&to) {
                if self.costs[to] < self.path.cost + weights[e] {
                    continue;
                }

                self.path.push(Step {
                    from: at,
                    to,
                    weight: weights[e],
                });
                self.pathing(weights, to);
                self.path.pop();
            }
        }
        self.visited.pop();
    }
}
