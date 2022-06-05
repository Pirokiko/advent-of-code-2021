// mod puzzle1;
// mod puzzle10;
mod puzzle11;
// mod puzzle2;
// mod puzzle3;
// mod puzzle4;
// mod puzzle5;
// mod puzzle6;
// mod puzzle7;
// mod puzzle8;
// mod puzzle9;

static CONTENT: &'static str = include_str!("puzzle11/puzzle11.txt");

fn main() {
    puzzle11::part1(CONTENT);
    puzzle11::part2(CONTENT);
}
