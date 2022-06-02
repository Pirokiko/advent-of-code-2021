mod graph;
mod graph_board;

pub fn part1(content: &str) -> usize {
    let board = graph_board::parse(content);
    let points = board.low_points();
    let result = points.iter().map(|node| node.value() + 1).sum();

    println!("Sum of low points: {}", result);

    result
}

pub fn part2(content: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::puzzle9::part1;

    #[test]
    fn part1_works() {
        assert_eq!(
            15,
            part1(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            )
        )
    }
}
