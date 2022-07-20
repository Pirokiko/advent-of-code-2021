use std::ops::RangeInclusive;

pub struct Cell {
    value: usize,
    has_flashed: bool,
}

impl Cell {
    pub fn from(value: usize) -> Cell {
        Cell {
            value,
            has_flashed: false,
        }
    }

    fn increase(&mut self, incr: usize) {
        self.value += incr;
    }
    fn can_flash(&self) -> bool {
        self.value > 9 && !self.has_flashed
    }
    fn flash(&mut self) {
        self.has_flashed = true;
    }
    fn reset(&mut self) -> bool {
        if self.has_flashed {
            self.has_flashed = false;
            self.value = 0;
            return true;
        }
        false
    }
}

pub struct Board {
    data: Vec<Vec<Cell>>,
    nr_of_flashes: usize,
}

impl Board {
    pub fn from(data: Vec<Vec<usize>>) -> Board {
        Board {
            nr_of_flashes: 0,
            data: data
                .iter()
                .map(|row| row.iter().map(|val| Cell::from(*val)).collect())
                .collect(),
        }
    }

    pub fn flash_count(&self) -> usize {
        self.nr_of_flashes
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.data[x][y]
    }

    pub fn increase_all(&mut self, incr: usize) {
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                cell.value += incr;
            }
        }
    }

    pub fn process_flashes(&mut self) {
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                self.flash(x, y);
            }
        }
    }

    pub fn reset_flashed(&mut self) -> bool {
        let mut all_flashed = true;
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                all_flashed = self.get_mut(x, y).reset() && all_flashed;
            }
        }
        all_flashed
    }

    fn flash(&mut self, x: usize, y: usize) {
        let cell = self.get_mut(x, y);
        if !cell.can_flash() {
            return;
        }
        cell.flash();
        self.nr_of_flashes += 1;

        for dx in range(x, 1, 0, self.data.len() - 1) {
            for dy in range(y, 1, 0, self.data[x].len() - 1) {
                self.get_mut(dx, dy).increase(1);
                self.flash(dx, dy);
            }
        }
    }
}

fn range(init: usize, delta: usize, min: usize, max: usize) -> RangeInclusive<usize> {
    let low = if min + delta > init {
        min
    } else {
        init - delta
    };
    let high = if max - delta < init {
        max
    } else {
        init + delta
    };
    low..=high
}
