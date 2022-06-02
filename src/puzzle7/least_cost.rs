#[derive(Debug, PartialEq)]
struct Position {
    original_value: isize,
    cost: usize,
    delta: isize,
}

impl Position {
    fn from(value: isize) -> Position {
        Position {
            original_value: value,
            cost: 0,
            delta: 0,
        }
    }

    fn value(&self) -> isize {
        self.original_value + self.delta
    }

    fn move_by(&mut self, increase: bool) {
        self.delta += if increase { 1 } else { -1 };
        self.cost += self.delta.abs() as usize;
    }

    fn move_cost(&self) -> usize {
        self.delta.abs() as usize + 1
    }
}

fn cost(nr: usize) -> usize {
    if nr == 0 {
        return 0;
    }
    if nr == 1 {
        return 1;
    }

    nr + cost(nr - 1)
}

fn move_cost_min(positions: &Vec<Position>) -> isize {
    let mut cost = 0;
    let mut index = 0;
    let min = positions[index].value();
    while index < positions.len() && positions[index].value() == min {
        cost += positions[index].delta.abs() + 1;
        index += 1;
    }
    cost
}

fn move_cost_max(positions: &Vec<Position>) -> isize {
    let mut index = positions.len() - 1;
    let mut cost = 0;
    let max = positions[index].value();

    while index > 0 && positions[index].value() == max {
        cost += positions[index].delta.abs() + 1;
        index -= 1;
    }
    cost
}

fn move_extreme<'a>(mut positions: impl Iterator<Item = &'a mut Position>, increase: bool) {
    let mut item = positions.next();
    if item.is_none() {
        return;
    }

    let val = item.as_ref().unwrap().value();

    while let Some(pos) = item {
        if pos.value() != val {
            break;
        }
        pos.move_by(increase);
        item = positions.next();
    }
}

fn move_min(positions: &mut Vec<Position>) {
    move_extreme(positions.iter_mut(), true)
}

fn move_max(positions: &mut Vec<Position>) {
    move_extreme(positions.iter_mut().rev(), false)
}

fn least_fuel(mut positions: Vec<Position>) -> usize {
    positions.sort_by(|a, b| a.original_value.cmp(&b.original_value));

    while positions[0].value() != positions[positions.len() - 1].value() {
        if move_cost_min(&positions) < move_cost_max(&positions) {
            move_min(&mut positions);
        } else {
            move_max(&mut positions);
        }
    }
    positions.iter().fold(0, |acc, pos| acc + pos.cost)
}

pub fn transform_and_least_fuel(pos_nrs: Vec<isize>) -> usize {
    let positions: Vec<Position> = pos_nrs.iter().map(|nr| Position::from(*nr)).collect();
    least_fuel(positions)
}

#[cfg(test)]
mod tests {
    use crate::puzzle7::least_cost::{
        cost, move_cost_max, move_cost_min, move_max, move_min, transform_and_least_fuel, Position,
    };

    #[test]
    fn least_cost_works() {
        assert_eq!(0, transform_and_least_fuel(vec![5]));
        assert_eq!(1, transform_and_least_fuel(vec![4, 5]));
        assert_eq!(2, transform_and_least_fuel(vec![3, 5]));
        assert_eq!(4, transform_and_least_fuel(vec![2, 5]));
        assert_eq!(6, transform_and_least_fuel(vec![1, 5]));
        assert_eq!(27, transform_and_least_fuel(vec![1, 1, 9]));
        assert_eq!(
            168,
            transform_and_least_fuel(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
        );
    }

    #[test]
    fn cost_works() {
        assert_eq!(0, cost(0));
        assert_eq!(1, cost(1));
        assert_eq!(3, cost(2));
        assert_eq!(6, cost(3));
        assert_eq!(55, cost(10));
        assert_eq!(120, cost(15));
    }

    fn positions(values: Vec<isize>) -> Vec<Position> {
        values.iter().map(|value| Position::from(*value)).collect()
    }

    fn pos(value: isize, steps: isize) -> Position {
        Position {
            original_value: value,
            delta: steps,
            cost: cost(steps.abs_diff(0)),
        }
    }

    #[test]
    fn move_cost_min_works() {
        assert_eq!(1, move_cost_min(&positions(vec![2, 4])));
        assert_eq!(2, move_cost_min(&positions(vec![2, 2, 4])));
        assert_eq!(2, move_cost_min(&positions(vec![2, 2, 4, 4, 4])));
        assert_eq!(4, move_cost_min(&vec![pos(2, 3), pos(24, 8)]));
        assert_eq!(7, move_cost_min(&vec![pos(2, 3), pos(3, 2), pos(24, 8)]));
    }

    #[test]
    fn move_min_works() {
        let mut pos_list = positions(vec![2, 4]);
        move_min(&mut pos_list);
        assert_eq!(vec![pos(2, 1), pos(4, 0)], pos_list);

        pos_list = vec![pos(3, 3), pos(14, -5)];
        move_min(&mut pos_list);
        assert_eq!(vec![pos(3, 4), pos(14, -5)], pos_list);

        pos_list = vec![pos(3, 3), pos(5, 1), pos(14, -5)];
        move_min(&mut pos_list);
        assert_eq!(vec![pos(3, 4), pos(5, 2), pos(14, -5)], pos_list);
    }

    #[test]
    fn move_cost_max_works() {
        assert_eq!(1, move_cost_max(&positions(vec![2, 4])));
        assert_eq!(1, move_cost_max(&positions(vec![2, 2, 4])));
        assert_eq!(3, move_cost_max(&positions(vec![2, 2, 4, 4, 4])));
        assert_eq!(9, move_cost_max(&vec![pos(2, 3), pos(24, -8)]));
        assert_eq!(
            16,
            move_cost_max(&vec![pos(2, 3), pos(22, -6), pos(24, -8)])
        );
    }

    #[test]
    fn move_max_works() {
        let mut pos_list = positions(vec![2, 4]);
        move_max(&mut pos_list);
        assert_eq!(vec![pos(2, 0), pos(4, -1)], pos_list);

        pos_list = vec![pos(3, 3), pos(14, -5)];
        move_max(&mut pos_list);
        assert_eq!(vec![pos(3, 3), pos(14, -6)], pos_list);

        pos_list = vec![pos(3, 3), pos(14, -5), pos(12, -3)];
        move_max(&mut pos_list);
        assert_eq!(vec![pos(3, 3), pos(14, -6), pos(12, -4)], pos_list);
    }
}
