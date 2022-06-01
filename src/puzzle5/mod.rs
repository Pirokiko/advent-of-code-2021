mod line;

use line::Line;
use line::Pos;
use std::collections::HashMap;
use std::fs;

fn index_for_pos(pos: &Pos) -> i32 {
    pos.x * 10000 + pos.y
}

fn pos_count_at_least(min: i32, lines: impl Iterator<Item = Line>) -> usize {
    counts_for_pos(lines).values().fold(0, |acc, x| {
        if *x >= min {
            return acc + 1;
        }
        acc
    })
}

fn counts_for_pos(lines: impl Iterator<Item = Line>) -> HashMap<i32, i32> {
    let mut index_counts = HashMap::new();
    for line in lines {
        for pos in line.positions() {
            let index = index_for_pos(&pos);
            *index_counts.entry(index).or_insert(0) += 1;
        }
    }
    index_counts
}

fn parse_pos(pos_str: &str) -> Pos {
    let parts: Vec<&str> = pos_str.split(",").collect();
    Pos {
        x: i32::from_str_radix(parts[0], 10).expect("to be a number"),
        y: i32::from_str_radix(parts[1], 10).expect("to be a number"),
    }
}
fn parse_line(line_str: &str) -> Line {
    let positions: Vec<&str> = line_str.split(" -> ").collect();
    Line {
        start: parse_pos(positions[0]),
        end: parse_pos(positions[1]),
    }
}

fn parse<'a>(content: &'a str) -> impl Iterator<Item = Line> + 'a {
    content.lines().map(|line| parse_line(line))
}

pub fn part1(content: &str) -> usize {
    let lines = parse(content);

    let nr_of_positions_above_2 = pos_count_at_least(
        2,
        lines.filter(|line| line.start.x == line.end.x || line.start.y == line.end.y),
    );
    println!(
        "Nr of positions with 2 or more lines covering them: {} (SHOULD BE 5294)",
        nr_of_positions_above_2
    );
    nr_of_positions_above_2
}

pub fn part2(content: &str) -> usize {
    let lines = parse(&content);

    let nr_of_positions_above_2 = pos_count_at_least(2, lines);
    println!(
        "Nr of positions with 2 or more lines covering them: {}",
        nr_of_positions_above_2
    );
    nr_of_positions_above_2
}

#[cfg(test)]
mod tests {
    use crate::puzzle5::{part1, part2};

    fn test_content() -> String {
        String::from("2,2 -> 6,6\n8,8 -> 3,3\n2,4 -> 6,4\n3,3 -> 3,6")
    }

    #[test]
    fn part1_works() {
        assert_eq!(1, part1(&test_content()))
    }

    #[test]
    fn part2_works() {
        assert_eq!(5, part2(&test_content()))
    }
}
