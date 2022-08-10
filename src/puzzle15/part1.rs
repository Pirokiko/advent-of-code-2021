use crate::lib::graph::NodeIndex;
use crate::lib::graph2::Graph;
use crate::puzzle15::parsing::{build_shortest_path, parse_graph_and_weights, parse_weight_map};
use crate::puzzle15::ShortestPath;

pub fn part1(content: &str) -> usize {
    let weight_map = parse_weight_map(content);
    let (graph, weights) = parse_graph_and_weights(weight_map);
    let pathing = build_shortest_path(graph, weights);

    let cost = pathing.calculate_cost();

    println!("{:?}", cost);
    cost
}

#[cfg(test)]
mod tests {
    use crate::puzzle15::part1::part1;

    static TEST_CONTENT: &'static str = include_str!("test.txt");
    static PUZZLE_CONTENT: &'static str = include_str!("puzzle15.txt");

    #[test]
    fn part1_works() {
        assert_eq!(40, part1(TEST_CONTENT));
    }

    #[test]
    fn part1_puzzle_works() {
        assert_eq!(696, part1(PUZZLE_CONTENT))
    }
}
