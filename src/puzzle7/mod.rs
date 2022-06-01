mod least_cost;

fn median_fuel(positions: Vec<isize>) -> usize {
    let median = positions[positions.len() / 2];

    let mut count: usize = 0;
    for pos in positions {
        let diff = median.abs_diff(pos);
        count += diff
    }
    count
}

pub fn part1(content: &str) {
    let mut data = parse(content);
    data.sort();
    let fuel = median_fuel(data);
    println!("Least fuel for median: {}", fuel);
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

// each move more cost the amount of the move in their chain of moves so: first moves costs 1, second move costs 2, third move costs 3, etc
// So moving 3 positions, costs 1 + 2 + 3 => 6 in total
pub fn part2(content: &str) {
    let mut data = parse(content);
    data.sort();
    let fuel = least_cost::transform_and_least_fuel(data);
    println!("Least fuel for mean: {}", fuel);
}

fn parse(content: &str) -> Vec<isize> {
    content
        .split(",")
        .map(|c| isize::from_str_radix(c.trim(), 10).expect("to be a digit"))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::puzzle7::median_fuel;

    #[test]
    fn it_works() {
        assert_eq!(2, median_fuel(vec![1, 3, 3]));
        assert_eq!(4, median_fuel(vec![1, 4, 5]));
        assert_eq!(6, median_fuel(vec![0, 1, 2, 3, 4]));
        assert_eq!(37, median_fuel(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]));
    }
}
