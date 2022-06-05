use crate::lib::graph::{Graph, NodeIndex};
use crate::puzzle12::Cavern;

pub fn all_paths(
    graph: &mut Graph<Cavern>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut paths: Vec<Vec<NodeIndex>> = vec![];
    let mut reporter = |path| paths.push(path);

    path(graph, start, end, &mut vec![], &mut reporter);

    paths
}

fn path<F>(
    graph: &mut Graph<Cavern>,
    node: NodeIndex,
    end: NodeIndex,
    so_far: &mut Vec<NodeIndex>,
    report_path: &mut F,
) where
    F: FnMut(Vec<NodeIndex>) -> (),
{
    so_far.push(node);
    graph.get_node_mut(node).value_mut().visited += 1;

    if node == end {
        report_path(so_far.clone());
    } else {
        let successor_nodes = graph.successors(node).collect::<Vec<NodeIndex>>();
        for next_node in successor_nodes {
            let node = graph.get_node(next_node).value();
            if node.visited == 0 || node.large {
                path(graph, next_node, end, so_far, report_path);
            }
        }
    }

    graph.get_node_mut(node).value_mut().visited -= 1;
    so_far.pop();
}
