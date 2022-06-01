extern crate core;

use std::fs;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;

fn main() {
    let content = fs::read_to_string("F:\\AdventOfCode\\puzzle9.txt").expect("to read input file");
    puzzle9::part1(&content);
    puzzle9::part2(&content);
}
