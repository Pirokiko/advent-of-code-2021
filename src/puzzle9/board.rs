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

// A way to model the board using a Graph??
// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

impl Board<'_> {
    fn from_lines<'a>(row_size: usize, content: &'a String) -> Board {
        let mut rows: Vec<Vec<Cell>> = vec![];
        for (row, line) in content.lines().enumerate() {
            let mut row_cells: Vec<Cell> = vec![];
            let mut left_cell: Option<&mut Cell> = None;
            for (column, digit) in line.split("").enumerate() {
                let mut cell = Cell::from(usize::from_str_radix(digit, 10).expect("to be a digit"));
                row_cells.push(cell);

                if row > 0 {
                    let up: &mut Cell = rows
                        .get_mut(row - 1)
                        .expect("there")
                        .get_mut(column)
                        .expect("there");
                    cell.up = Some(up);
                    up.down = Some(&cell);
                }
                if let Some(left) = left_cell {
                    cell.left = Some(left);
                    left.right = Some(&cell);
                }
                left_cell = Some(&mut cell);
            }
            rows.push(row_cells);
        }

        Board { rows }
    }
}
