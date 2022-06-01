use std::collections::HashMap;

fn get_for_age(fishes: &mut HashMap<usize, usize>, key: usize) -> usize {
    *fishes.get(&key).expect(&format!("{} should exist", key))
}

fn move_value(fishes: &mut HashMap<usize, usize>, from: usize, to: usize) {
    let value = get_for_age(fishes, from);
    fishes.entry(to).and_modify(|v| *v = value);
}

fn process_fishes(fishes: &mut HashMap<usize, usize>) {
    let spawn = *fishes.get(&0).expect("0 should exist");

    move_value(fishes, 1, 0);
    move_value(fishes, 2, 1);
    move_value(fishes, 3, 2);
    move_value(fishes, 4, 3);
    move_value(fishes, 5, 4);
    move_value(fishes, 6, 5);
    move_value(fishes, 7, 6);
    move_value(fishes, 8, 7);
    fishes.entry(8).and_modify(|v| *v = spawn);
    fishes.entry(6).and_modify(|v| *v += spawn);
}

fn empty_fishes() -> HashMap<usize, usize> {
    HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ])
}

fn parse(content: &str) -> HashMap<usize, usize> {
    let mut fishes = empty_fishes();

    let ages = content.split(",").map(|item| {
        usize::from_str_radix(item.trim(), 10).expect(&format!("should be a digit: {}", item))
    });

    for age in ages {
        fishes.entry(age).and_modify(|v| *v += 1).or_insert(1);
    }

    fishes
}

fn simulate_fishes(fishes: &mut HashMap<usize, usize>, days: usize) {
    for day in 1..=days {
        process_fishes(fishes);
        println!("After {:03} days: {}", day, fishes.values().sum::<usize>());
    }
}

fn parse_simulate_count(content: &str, days: usize) -> usize {
    let mut fishes = parse(content);

    simulate_fishes(&mut fishes, days);

    fishes.values().sum()
}

pub fn part1(content: &str) {
    let fish_count = parse_simulate_count(content, 80);

    println!("Nr of fishes after 80 days: {}", fish_count);
}

pub fn part2(content: &str) {
    let fish_count = parse_simulate_count(content, 256);

    println!("Nr of fishes after 256 days: {}", fish_count);
}

#[cfg(test)]
mod tests {
    use crate::puzzle6::parse_simulate_count;

    fn test_content() -> String {
        String::from("3,4,3,1,2")
    }

    #[test]
    fn simulate_fishes_works() {
        assert_eq!(5934, parse_simulate_count(&test_content(), 80))
    }
    #[test]
    fn simulate_fishes_works_long_time() {
        assert_eq!(26984457539, parse_simulate_count(&test_content(), 256))
    }
    #[test]
    fn simulate_fishes_works_other() {
        assert_eq!(4, parse_simulate_count("4", 18));
        assert_eq!(5, parse_simulate_count("4", 19));
    }
}
