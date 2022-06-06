use crate::lib::graph::{Graph, NodeIndex};
use crate::puzzle12::Cavern;

pub struct Pathing {
    graph: Graph<Cavern>,
    start: NodeIndex,
    end: NodeIndex,
    paths: usize,
    visited_twice: Option<NodeIndex>,
}

// Initializer
impl Pathing {
    pub fn from(graph: Graph<Cavern>, start: NodeIndex, end: NodeIndex) -> Pathing {
        Pathing {
            graph,
            start,
            end,
            paths: 0,
            visited_twice: None,
        }
    }
}

// Algorithm
impl Pathing {
    pub fn all_paths(&mut self) -> usize {
        self.path(self.start, &mut vec![]);

        self.paths
    }

    fn further_step(&mut self, node: NodeIndex, so_far: &mut Vec<NodeIndex>) {
        so_far.push(node);
        self.graph.get_node_mut(node).value_mut().visited += 1;
    }
    fn revert_step(&mut self, node: NodeIndex, so_far: &mut Vec<NodeIndex>) {
        if self.visited_twice.is_some() && self.visited_twice.unwrap() == node {
            self.visited_twice = None;
        }

        self.graph.get_node_mut(node).value_mut().visited -= 1;
        so_far.pop();
    }

    fn node_validity(&self, node_index: NodeIndex) -> (bool, bool) {
        let node = self.graph.get_node(node_index).value();
        let allowed = node.large || node.visited == 0;
        let visit_twice_allowed = !node.large
            && node.visited == 1
            && self.visited_twice.is_none()
            && node_index != self.start;
        (allowed, visit_twice_allowed)
    }

    fn path(&mut self, node: NodeIndex, so_far: &mut Vec<NodeIndex>) {
        self.further_step(node, so_far);

        if node == self.end {
            self.paths += 1;
            self.revert_step(node, so_far);
            return;
        }
        // Must be collected first or the immutable borrow conflicts with the mutable borrow within the for loop
        let connected_nodes = self.graph.successors(node).collect::<Vec<NodeIndex>>();
        for next_node in connected_nodes {
            let (allowed, visit_twice) = self.node_validity(next_node);
            if allowed || visit_twice {
                if !allowed {
                    self.visited_twice = Some(next_node);
                }
                self.path(next_node, so_far);
            }
        }

        self.revert_step(node, so_far);
    }
}
