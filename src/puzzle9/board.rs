use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Cell<'a> {
    value: usize,

    up: Option<&'a Cell<'a>>,
    down: Option<&'a Cell<'a>>,
    left: Option<&'a Cell<'a>>,
    right: Option<&'a Cell<'a>>,
}

fn lower_than(cell: &Cell, against: &Option<&Cell>) -> bool {
    against.is_none() || against.unwrap().value > cell.value
}

impl<'a> Cell<'a> {
    fn from(value: usize) -> Cell<'a> {
        Cell {
            value,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }

    fn low_point(&self) -> bool {
        lower_than(self, &self.up)
            && lower_than(self, &self.down)
            && lower_than(self, &self.left)
            && lower_than(self, &self.right)
    }
}

struct Board<'a> {
    rows: Vec<Vec<Cell<'a>>>,
}

fn link_left(mut cell: &mut Cell, mut left: &mut Cell) {
    cell.left = Some(left);

    left.right = Some(cell);
}

impl Board {
    fn from_lines<'a>(row_size: usize, content: &'a String) -> Board {
        let mut rows: Vec<Vec<Cell>> = vec![];
        for (row, line) in content.lines().enumerate() {
            let mut row_cells: Vec<Cell> = vec![];
            for (column, digit) in line.split("").enumerate() {
                let mut cell = Cell::from(usize::from_str_radix(digit, 10).expect("to be a digit"));
                row_cells.push(cell);

                if (row > 0) {
                    cell.up = Some(
                        rows.get(row - 1)
                            .expect("there")
                            .get(column)
                            .expect("there"),
                    )
                }
                if (column > 0) {
                    cell.left = Some(
                        rows.get(row - 1)
                            .expect("there")
                            .get_mut(column - 1)
                            .expect("there"),
                    )
                }
            }
            rows.push(row_cells);
        }

        let mut row = 0;
        Board {
            rows: content
                .lines()
                .enumerate()
                .map(|(idx, line)| line.split("").enumerate().map(|(idx, str)| {}).collect())
                .collect::<Vec<Vec<Cell<'a>>>>(),
        }
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
