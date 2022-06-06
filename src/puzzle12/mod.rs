mod pathing;
mod pathing2;

use crate::lib::graph::{Graph, NodeIndex};
use crate::puzzle12::pathing::all_paths;
use crate::puzzle12::pathing2::Pathing;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Default)]
pub struct Cavern {
    id: String,
    large: bool,
    visited: usize,
}

impl Cavern {
    fn from(id: String, large: bool) -> Cavern {
        Cavern {
            id,
            large,
            visited: 0,
        }
    }
}

pub fn parse(content: &str) -> (Graph<Cavern>, NodeIndex, NodeIndex) {
    let mut graph = Graph::default();
    let mut start = 0;
    let mut end = 0;

    let mut nodes = HashMap::new();
    content.lines().for_each(|line| {
        let mut caverns = line
            .split("-")
            .map(|node_id| {
                if nodes.contains_key(node_id) {
                    return *nodes.get(node_id).unwrap();
                }
                let node_index = graph.add_node(Cavern::from(
                    String::from(node_id),
                    node_id.chars().all(|char| char.is_uppercase()),
                ));
                nodes.insert(node_id, node_index);
                if node_id.eq("start") {
                    start = node_index;
                }
                if node_id.eq("end") {
                    end = node_index;
                }
                node_index
            })
            .collect::<Vec<_>>();

        let cavern1 = caverns.pop().unwrap();
        let cavern2 = caverns.pop().unwrap();
        graph.add_edge(cavern1, cavern2);
        graph.add_edge(cavern2, cavern1);
    });

    (graph, start, end)
}

pub fn part1(content: &str) -> usize {
    let (mut graph, start, end) = parse(content);
    let paths = all_paths(&mut graph, start, end);
    let result = paths.len();
    println!("Answer part 1: {}", result);
    result
}

pub fn part2(content: &str) -> usize {
    let (graph, start, end) = parse(content);
    let result = Pathing::from(graph, start, end).all_paths();

    println!("Answer part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::puzzle12::{part1, part2};

    static TEST_1: &'static str = include_str!("test1.txt");
    static TEST_2: &'static str = include_str!("test2.txt");
    static TEST_3: &'static str = include_str!("test3.txt");

    #[test]
    fn part1_works() {
        assert_eq!(10, part1(TEST_1));
        assert_eq!(19, part1(TEST_2));
        assert_eq!(226, part1(TEST_3));
    }

    #[test]
    fn part2_works() {
        assert_eq!(3509, part2(TEST_3));
        assert_eq!(103, part2(TEST_2));
        assert_eq!(36, part2(TEST_1));
    }
}
