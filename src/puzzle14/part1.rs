use std::collections::HashMap;

struct Swap {
    pattern: String,
    insert: char,
}

fn parse_swap(line: &str) -> Swap {
    let mut parts = line.split(" -> ").map(|part| part.trim());
    let pattern = String::from(parts.next().unwrap());
    let insert = parts.next().unwrap().chars().next().unwrap();
    Swap { pattern, insert }
}

fn parse(content: &str) -> (Vec<char>, Vec<Swap>) {
    let mut lines = content.lines();
    let polymer = lines.next().unwrap().chars().collect();
    lines.next();

    let swaps = lines.map(parse_swap).collect();

    (polymer, swaps)
}

fn insert(polymer: &mut Vec<char>, swaps: &Vec<Swap>) {
    let mut index = 0;
    while index < polymer.len() - 1 {
        let pattern = format!("{}{}", polymer[index], polymer[index + 1]);

        let swap = swaps.iter().find(|swap| swap.pattern.eq(&pattern));

        match swap {
            None => {}
            Some(swap) => {
                polymer.insert(index + 1, swap.insert);
                index += 1;
            }
        }
        index += 1;
    }
}

fn most_and_least(polymer: &Vec<char>) -> (usize, usize) {
    let mut map: HashMap<char, usize> = HashMap::new();

    for char in polymer {
        *map.entry(*char).or_insert(0) += 1;
    }

    (*map.values().max().unwrap(), *map.values().min().unwrap())
}

pub fn part1(content: &str) -> usize {
    let (mut polymer, swaps) = parse(content);
    for _ in 0..10 {
        insert(&mut polymer, &swaps);
    }

    let (most, least) = most_and_least(&polymer);
    let result = most - least;
    println!("Part1 answer: {}", result);
    result
}

#[cfg(test)]
mod test {
    use crate::puzzle14::part1::part1;

    static TEST_CONTENT: &'static str = include_str!("test.txt");
    #[test]
    fn part1_works() {
        assert_eq!(1588, part1(TEST_CONTENT));
    }
}
