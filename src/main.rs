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

fn main() {
    let content = fs::read_to_string("F:\\AdventOfCode\\puzzle8.txt").expect("to read input file");
    puzzle8::part1(&content);
    puzzle8::part2(&content);
}
