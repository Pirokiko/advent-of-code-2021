use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Cell {
    value: usize,
    marked: bool,
}

struct Board {
    won_with: Option<usize>,
    row_size: usize,
    cells: Vec<Cell>,
}

impl Display for Board {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            println!(
                "{:?}",
                row.iter()
                    .map(|cell| {
                        return if cell.marked {
                            String::from("XX")
                        } else {
                            format!("{:02}", cell.value)
                        };
                    })
                    .collect::<Vec<String>>()
            );
        }
        Ok(())
    }
}

impl Board {
    fn from_lines(row_size: usize, lines: Vec<&str>) -> Board {
        Board::from_values(
            row_size,
            lines
                .iter()
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(|part| part.parse::<usize>().expect("should be a digit"))
                })
                .collect(),
        )
    }

    fn from_values(row_size: usize, values: Vec<usize>) -> Board {
        Board {
            won_with: None,
            row_size,
            cells: values
                .iter()
                .map(|value| Cell {
                    value: *value,
                    marked: false,
                })
                .collect(),
        }
    }

    fn rows(&self) -> Vec<&[Cell]> {
        self.cells.chunks(self.row_size).collect()
    }

    fn columns(&self) -> Vec<Vec<&Cell>> {
        let column_count = self.cells.len() / self.row_size;
        let mut columns = Vec::with_capacity(column_count);
        for _ in 0..column_count {
            columns.push(vec![]);
        }

        for (idx, cell) in self.cells.iter().enumerate() {
            let column_index = idx % self.row_size;
            columns[column_index].push(cell);
        }

        columns
    }

    fn check_won(&self) -> bool {
        for row in self.rows() {
            if row.iter().all(|cell| cell.marked) {
                return true;
            }
        }

        for column in self.columns() {
            if column.iter().all(|cell| cell.marked) {
                return true;
            }
        }

        false
    }

    fn process(&mut self, nr: usize) {
        match self.cells.iter_mut().find(|cell| cell.value == nr) {
            None => {}
            Some(mut cell) => cell.marked = true,
        }

        if self.check_won() {
            self.won_with = Some(nr);
        }
    }

    fn score(&self) -> usize {
        match self.won_with {
            None => 0,
            Some(nr) => {
                self.cells
                    .iter()
                    .filter(|cell| !cell.marked)
                    .map(|cell| cell.value)
                    .sum::<usize>()
                    * nr
            }
        }
    }

    fn won(&self) -> bool {
        match self.won_with {
            None => false,
            Some(_) => true,
        }
    }
}

fn parse(content: &str) -> (Vec<usize>, Vec<Board>) {
    let mut lines = content.lines().peekable();

    let nrs: Vec<usize> = lines
        .next()
        .expect("numbers to choose must be present")
        .split(",")
        .map(|digits| {
            digits
                .parse()
                .expect(&format!("'{}' to only contain digit chars", digits))
        })
        .collect();

    lines.next(); // Empty line

    let mut boards = vec![];
    while lines.peek().is_some() {
        boards.push(Board::from_lines(
            5,
            vec![
                lines.next().expect("should be there"),
                lines.next().expect("should be there"),
                lines.next().expect("should be there"),
                lines.next().expect("should be there"),
                lines.next().expect("should be there"),
            ],
        ));

        lines.next(); // Empty line
    }

    (nrs, boards)
}

// fn debug_boards(boards: &Vec<Board>) {
//     for board in boards {
//         debug_board(board);
//     }
// }
//
// fn debug_board(board: &Board) {
//     println!("{}", board)
// }

pub fn part1(content: &str) {
    let (nrs, mut boards) = parse(content);

    'outer: for nr in nrs {
        println!("Processing {}", nr);
        for board in boards.as_mut_slice() {
            board.process(nr);
            if board.won() {
                println!("First board to win with a score of: {}", board.score());
                break 'outer;
            }
        }
    }
}

pub fn part2(content: &str) {
    let (nrs, mut boards) = parse(content);

    let mut last_board_to_win = 0;

    for nr in nrs {
        println!("Processing {}", nr);
        for (idx, board) in boards.iter_mut().enumerate() {
            if !board.won() {
                board.process(nr);
                if board.won() {
                    last_board_to_win = idx;
                    println!("Board won with a score of: {}", board.score());
                }
            }
        }
    }

    println!(
        "Last board to win would get a score of: {}",
        boards[last_board_to_win].score()
    )
}
