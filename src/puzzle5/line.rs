#[derive(PartialEq, Debug)]
pub(crate) struct Pos {
    pub x: i32,
    pub y: i32,
}

pub(crate) struct Line {
    pub start: Pos,
    pub end: Pos,
}

struct LinePositions<'line> {
    direction: Pos,
    count: i32,
    line: &'line Line,
}

impl<'line> LinePositions<'line> {
    fn new(direction: Pos, line: &'line Line) -> LinePositions<'line> {
        LinePositions {
            direction,
            line,
            count: 0,
        }
    }
}

impl<'line> Iterator for LinePositions<'line> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.line.start.x + self.count * self.direction.x;
        let y = self.line.start.y + self.count * self.direction.y;

        if self.line.on_line(x, y, &self.direction) {
            self.count += 1;
            return Some(Pos { x, y });
        }
        None
    }
}

fn has_passed(end: i32, pos: i32, direction: i32) -> bool {
    if direction == 0 {
        return end != pos;
    }
    if direction > 0 {
        return pos > end;
    }
    return pos < end;
}

impl Line {
    pub fn positions(&'_ self) -> impl IntoIterator<Item = Pos> + '_ {
        LinePositions::new(self.direction(), self)
    }

    fn direction(&self) -> Pos {
        Pos {
            x: calc_delta(self.start.x, self.end.x),
            y: calc_delta(self.start.y, self.end.y),
        }
    }

    fn on_line(&self, x: i32, y: i32, direction: &Pos) -> bool {
        !has_passed(self.end.x, x, direction.x) && !has_passed(self.end.y, y, direction.y)
    }
}

fn calc_delta(a: i32, b: i32) -> i32 {
    if a > b {
        return -1;
    }
    if a < b {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::puzzle5::line::{Line, Pos};

    #[test]
    fn it_includes_both_ends() {
        let mut pos_iter = Line {
            start: Pos { x: 1, y: 1 },
            end: Pos { x: 3, y: 3 },
        }
        .positions()
        .into_iter();
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 1 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 2 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 3, y: 3 }));
        assert_eq!(pos_iter.next(), None);
    }

    #[test]
    fn horizontal_lines_work() {
        let mut pos_iter = Line {
            start: Pos { x: 1, y: 1 },
            end: Pos { x: 3, y: 1 },
        }
        .positions()
        .into_iter();
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 1 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 1 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 3, y: 1 }));
        assert_eq!(pos_iter.next(), None);
    }

    #[test]
    fn vertical_lines_work() {
        let mut pos_iter = Line {
            start: Pos { x: 1, y: 1 },
            end: Pos { x: 1, y: 3 },
        }
        .positions()
        .into_iter();
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 1 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 2 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 3 }));
        assert_eq!(pos_iter.next(), None);
    }

    #[test]
    fn horizontal_lines_reverse_work() {
        let mut pos_iter = Line {
            start: Pos { x: 3, y: 1 },
            end: Pos { x: 1, y: 1 },
        }
        .positions()
        .into_iter();
        assert_eq!(pos_iter.next(), Some(Pos { x: 3, y: 1 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 1 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 1 }));
        assert_eq!(pos_iter.next(), None);
    }

    #[test]
    fn vertical_lines_reverse_work() {
        let mut pos_iter = Line {
            start: Pos { x: 1, y: 3 },
            end: Pos { x: 1, y: 1 },
        }
        .positions()
        .into_iter();
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 3 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 2 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 1 }));
        assert_eq!(pos_iter.next(), None);
    }
}
