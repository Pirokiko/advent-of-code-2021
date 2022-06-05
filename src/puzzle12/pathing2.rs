use crate::lib::graph::{Graph, NodeIndex};
use crate::puzzle12::Cavern;

pub fn all_paths2(
    graph: &mut Graph<Cavern>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut paths: Vec<Vec<NodeIndex>> = vec![];
    let mut reporter = |path| paths.push(path);

    let mut cave_visited_twice = None;

    path2(
        graph,
        start,
        start,
        end,
        &mut cave_visited_twice,
        &mut vec![],
        &mut reporter,
    );

    paths
}

fn path2<F>(
    graph: &mut Graph<Cavern>,
    node: NodeIndex,
    start: NodeIndex,
    end: NodeIndex,
    mut cave_visited_twice: &mut Option<NodeIndex>,
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
            if node.large || node.visited == 0 || node.large {
                path2(
                    graph,
                    next_node,
                    start,
                    end,
                    cave_visited_twice,
                    so_far,
                    report_path,
                );
            } else if cave_visited_twice.is_none() && next_node != start {
                *cave_visited_twice = Some(next_node);
                path2(
                    graph,
                    next_node,
                    start,
                    end,
                    cave_visited_twice,
                    so_far,
                    report_path,
                );
            }
        }
    }

    if cave_visited_twice.is_some() && cave_visited_twice.unwrap() == node {
        *cave_visited_twice = None;
    }

    graph.get_node_mut(node).value_mut().visited -= 1;
    so_far.pop();
}
