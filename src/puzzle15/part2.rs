use crate::puzzle15::parsing::{build_shortest_path, parse_graph_and_weights, parse_weight_map};

fn value_of(value: usize) -> usize {
    ((value - 1) % 9) + 1
}

fn map_full_cavern(weight_map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let size = weight_map.len();
    let mut full_map: Vec<Vec<usize>> = vec![vec![usize::MAX; size * 5]; size * 5];
    for dx in 0..5 {
        for dy in 0..5 {
            for (x, row) in weight_map.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    let pos_x = x + (dx * size);
                    let pos_y = y + (dy * size);
                    full_map[pos_x][pos_y] = value_of(cell + dx + dy);
                }
            }
        }
    }

    full_map
}

pub fn part2(content: &str) -> usize {
    let weight_map = parse_weight_map(content);
    let full_weight_map = map_full_cavern(weight_map);
    let (graph, weights) = parse_graph_and_weights(full_weight_map);
    let pathing = build_shortest_path(graph, weights);

    let cost = pathing.calculate_cost();
    println!("{:?}", cost);
    cost
}

#[cfg(test)]
mod tests {
    use crate::puzzle15::part2::part2;

    static TEST_CONTENT: &'static str = include_str!("test.txt");
    static PUZZLE_CONTENT: &'static str = include_str!("puzzle15.txt");

    #[test]
    fn part2_test_works() {
        assert_eq!(315, part2(TEST_CONTENT));
    }

    #[test]
    fn part2_puzzle_works() {
        assert_eq!(2952, part2(PUZZLE_CONTENT))
    }
}
