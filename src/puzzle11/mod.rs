use crate::puzzle11::board::Board;

mod board;

fn parse(content: &str) -> Board {
    Board::from(
        content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| usize::from_str_radix(&char.to_string(), 10).unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect(),
    )
}

fn step(board: &mut Board) {
    board.increase_all(1);
    board.process_flashes();
    board.reset_flashed();
}
fn simulate(board: &mut Board) {
    for _ in 0..100 {
        step(board);
    }
}

pub fn part1(content: &str) -> usize {
    let mut board = parse(content);
    simulate(&mut board);

    println!("Answer part 1: {}", board.flash_count());

    board.flash_count()
}

pub fn part2(content: &str) -> usize {
    let board = parse(content);
    board.flash_count()
}

#[cfg(test)]
mod tests {
    use crate::puzzle11::part1;

    #[test]
    fn part1_works() {
        assert_eq!(
            1656,
            part1(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            )
        )
    }
}
