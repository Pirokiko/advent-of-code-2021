use std::collections::HashMap;

type Counts = HashMap<char, usize>;

type SwapMap = HashMap<char, HashMap<char, char>>;

struct Swaps {
    swaps: SwapMap,
}

impl Swaps {
    fn get_insert(&self, first: &char, second: &char) -> Option<char> {
        self.swaps
            .get(first)
            .map_or(None, |char_map| char_map.get(second))
            .map_or(None, |char| Some(*char))
    }
}

struct PairInsertion {
    swaps: Swaps,
    counts: Counts,
    add_counter: usize,
}

impl PairInsertion {
    pub fn counts(&mut self, polymer: &Vec<char>, iteration_count: usize) -> &Counts {
        self.add(&polymer[0]);
        for idx in 0..polymer.len() - 1 {
            self.count_recursive(&polymer[idx], &polymer[idx + 1], iteration_count);
            self.add(&polymer[idx + 1]);
        }

        &self.counts
    }

    fn add(&mut self, char: &char) {
        *self.counts.get_mut(char).unwrap() += 1;

        // self.counts
        //     .insert(char, self.counts.get(&char).unwrap_or(&0) + 1);

        // *self.counts.entry(char).or_insert(0) += 1;
        self.add_counter += 1;
    }

    fn count_recursive(&mut self, first: &char, second: &char, iterations_left: usize) {
        if iterations_left == 0 {
            return;
        }

        match self.swaps.get_insert(first, second) {
            None => {}
            Some(insert) => {
                self.add(&insert);
                self.count_recursive(first, &insert, iterations_left - 1);
                self.count_recursive(&insert, second, iterations_left - 1);
            }
        }
    }
}

fn parse_swap(line: &str) -> (char, char, char) {
    let mut parts = line.split(" -> ").map(|part| part.trim());

    let mut pattern = parts.next().unwrap().chars();
    let insert = parts.next().unwrap().chars().next().unwrap();
    (pattern.next().unwrap(), pattern.next().unwrap(), insert)
}

fn parse(content: &str) -> (Vec<char>, PairInsertion) {
    let mut lines = content.lines();
    let polymer = lines.next().unwrap().chars().collect();
    lines.next(); // skip empty line

    let mut swaps = SwapMap::new();

    for (first, second, insert) in lines.map(parse_swap) {
        swaps
            .entry(first)
            .or_insert(HashMap::new())
            .entry(second)
            .or_insert(insert);
    }

    (
        polymer,
        PairInsertion {
            swaps: Swaps { swaps },
            counts: new_counts(),
            add_counter: 0,
        },
    )
}

fn new_counts() -> Counts {
    let mut counts = Counts::new();
    counts.insert('B', 0);
    counts.insert('C', 0);
    counts.insert('H', 0);
    counts.insert('N', 0);

    counts
}

fn most_and_least(counts: &Counts) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;

    for (_, count) in counts.iter() {
        if *count > max {
            max = *count;
        }

        if *count < min {
            min = *count;
        }
    }

    (max, min)
}

fn process(content: &str, iteration_count: usize) -> usize {
    let (polymer, mut pair_insertion) = parse(content);

    let counts = pair_insertion.counts(&polymer, iteration_count);

    let (most, least) = most_and_least(counts);

    println!("Add count: {}", pair_insertion.add_counter);

    most - least
}

pub fn part2(content: &str) -> usize {
    let result = process(content, 20);
    println!("Part2 answer: {}", result);
    result
}

pub fn part1(content: &str) -> usize {
    let result = process(content, 10);
    println!("Part1 answer: {}", result);
    result
}

#[cfg(test)]
mod test {
    use crate::puzzle14::part2_recursive2::part1;
    use crate::puzzle14::part2_recursive2::part2;

    static TEST_CONTENT: &'static str = include_str!("test.txt");

    #[test]
    fn part2_works() {
        assert_eq!(2188189693529, part2(TEST_CONTENT));
    }

    #[test]
    fn part1_works() {
        assert_eq!(1588, part1(TEST_CONTENT));
    }
}
