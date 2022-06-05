use itertools::Itertools;

mod graph_board;

pub fn part1(content: &str) -> usize {
    let board = graph_board::parse(content);
    let points = board.low_points();
    let result = points.iter().map(|node| node.value() + 1).sum();

    println!("Sum of low points: {}", result);

    result
}

pub fn part2(content: &str) -> usize {
    let board = graph_board::parse(content);
    let basins: Vec<Vec<usize>> = board.basins();

    // When 2 low_points are in the same basin, indices appear twice in one basin and the basin is duplicated
    // using .sort() and .unique() to de-duplicate
    // TODO: change the way .basin() works (check for multiple low_points in a single basin / mark each node with a basin value and collect results after first pass)
    let mut sorted_unique_basins: Vec<Vec<usize>> = basins
        .into_iter()
        .map(|basin| basin.into_iter().unique().sorted().collect())
        .unique()
        .collect();
    sorted_unique_basins.sort_by(|indices1, indices2| indices2.len().cmp(&indices1.len()));
    println!("{:?}", sorted_unique_basins);
    let result = sorted_unique_basins[0].len()
        * sorted_unique_basins[1].len()
        * sorted_unique_basins[2].len();

    println!("Multiplication of 3 largest basins: {}", result);

    result
}

#[cfg(test)]
mod tests {
    use crate::puzzle9::{part1, part2};
    use std::collections::VecDeque;

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

    #[test]
    fn part2_works() {
        assert_eq!(21, part2("91019\n09190\n91019\n09990\n91019"));

        assert_eq!(
            1134,
            part2(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            )
        );
    }

    #[test]
    fn vecdeque_works_as_expected() {
        let mut queue = VecDeque::from([]);
        queue.push_back(1);
        queue.push_back(2);
        assert_eq!(1, queue.pop_front().unwrap());
        queue.push_back(3);

        assert_eq!(2, queue.pop_front().unwrap());
        assert_eq!(3, queue.pop_front().unwrap());
    }
}
