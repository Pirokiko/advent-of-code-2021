mod lib;
mod puzzle1;
mod puzzle10;
mod puzzle11;
mod puzzle12;
mod puzzle13;
mod puzzle14;
mod puzzle15;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;

fn main() {
    if cfg!(puzzle = "puzzle1") {
        let content_puzzle1 = include_str!("puzzle1/puzzle1.txt");
        puzzle1::part1(content_puzzle1);
        puzzle1::part2(content_puzzle1);
    }

    if cfg!(puzzle = "puzzle2") {
        let content_puzzle2 = include_str!("puzzle2/puzzle2.txt");
        puzzle2::part1(content_puzzle2);
        puzzle2::part2(content_puzzle2);
    }

    if cfg!(puzzle = "puzzle3") {
        let content_puzzle3 = include_str!("puzzle3/puzzle3.txt");
        puzzle3::part1(content_puzzle3);
        puzzle3::part2(content_puzzle3);
    }

    if cfg!(puzzle = "puzzle4") {
        let content_puzzle4 = include_str!("puzzle4/puzzle4.txt");
        puzzle4::part1(content_puzzle4);
        puzzle4::part2(content_puzzle4);
    }

    if cfg!(puzzle = "puzzle5") {
        let content_puzzle5 = include_str!("puzzle5/puzzle5.txt");
        puzzle5::part1(content_puzzle5);
        puzzle5::part2(content_puzzle5);
    }

    if cfg!(puzzle = "puzzle6") {
        let content_puzzle6 = include_str!("puzzle6/puzzle6.txt");
        puzzle6::part1(content_puzzle6);
        puzzle6::part2(content_puzzle6);
    }

    if cfg!(puzzle = "puzzle7") {
        let content_puzzle7 = include_str!("puzzle7/puzzle7.txt");
        puzzle7::part1(content_puzzle7);
        puzzle7::part2(content_puzzle7);
    }

    if cfg!(puzzle = "puzzle8") {
        let content_puzzle8 = include_str!("puzzle8/puzzle8.txt");
        puzzle8::part1(content_puzzle8);
        puzzle8::part2(content_puzzle8);
    }

    if cfg!(puzzle = "puzzle9") {
        let content_puzzle9 = include_str!("puzzle9/puzzle9.txt");
        puzzle9::part1(content_puzzle9);
        puzzle9::part2(content_puzzle9);
    }

    if cfg!(puzzle = "puzzle10") {
        let content_puzzle10 = include_str!("puzzle10/puzzle10.txt");
        puzzle10::part1(content_puzzle10);
        puzzle10::part2(content_puzzle10);
    }

    if cfg!(puzzle = "puzzle11") {
        let content_puzzle11 = include_str!("puzzle11/puzzle11.txt");
        puzzle11::part1(content_puzzle11);
        puzzle11::part2(content_puzzle11);
    }

    if cfg!(puzzle = "puzzle12") {
        let content_puzzle12 = include_str!("puzzle12/puzzle12.txt");
        puzzle12::part1(content_puzzle12);
        puzzle12::part2(content_puzzle12);
    }

    if cfg!(puzzle = "puzzle13") {
        let content_puzzle13 = include_str!("puzzle13/puzzle13.txt");
        puzzle13::part1(content_puzzle13);
        puzzle13::part2(content_puzzle13);
    }

    if cfg!(puzzle = "puzzle14") {
        let content_puzzle14 = include_str!("puzzle14/puzzle14.txt");
        puzzle14::part2_recursive::part1(content_puzzle14);
        puzzle14::part2_recursive::part2(content_puzzle14);
    }

    // if cfg!(puzzle = "puzzle15") {
    let content_puzzle15 = include_str!("puzzle15/puzzle15.txt");
    puzzle15::part1::part1(content_puzzle15);
    // puzzle15::part2::part2(content_puzzle15);
    // }
}
