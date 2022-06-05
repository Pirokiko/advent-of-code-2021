mod lib;
mod puzzle1;
mod puzzle10;
mod puzzle11;
mod puzzle12;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;

static CONTENT: &'static str = include_str!("puzzle12/puzzle12.txt");

fn main() {
    puzzle12::part1(CONTENT);
    puzzle12::part2(CONTENT);
}
