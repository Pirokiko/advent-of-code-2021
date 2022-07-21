// type Counts = HashMap<char, usize>;

// Using a HashMap<char,usize> is magnitudes slower :(
// 3_000_000 additions on a hardcoded key took 1.9 seconds
// 3_000_000 additions through this case char as usize trick on a vector took 166 ms
struct Counts {
    data: Vec<usize>,
}

impl Counts {
    fn get(&self, key: char) -> usize {
        self.data[key as usize]
    }

    // fn insert(&mut self, key: char, value: usize) {
    //     self.ensure_key(&key);
    //     self.data[key as usize] = value;
    // }

    fn ensure_key(&mut self, key: &char) {
        if self.data.len() <= *key as usize {
            self.data.resize(*key as usize + 1, 0);
        }
    }

    fn increment(&mut self, key: &char) {
        self.ensure_key(key);
        self.data[*key as usize] += 1;
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
struct Data<T: Clone + PartialOrd + PartialEq> {
    b: T,
    c: T,
    f: T,
    h: T,
    k: T,
    n: T,
    o: T,
    p: T,
    s: T,
    v: T,
}

impl<T: Clone + PartialOrd + PartialEq> Data<T> {
    fn get(&self, key: &char) -> Option<&T> {
        match *key {
            'B' => Some(&self.b),
            'C' => Some(&self.c),
            'F' => Some(&self.f),
            'H' => Some(&self.h),
            'K' => Some(&self.k),
            'N' => Some(&self.n),
            'O' => Some(&self.o),
            'P' => Some(&self.p),
            'S' => Some(&self.s),
            'V' => Some(&self.v),
            _ => None,
        }
    }

    fn min_max(&self, ignore: &T) -> (Option<&T>, Option<&T>) {
        let keys = vec!['B', 'C', 'F', 'H', 'K', 'N', 'O', 'P', 'S', 'V'];

        let mut min: Option<&T> = None;
        let mut max: Option<&T> = None;

        for key in keys {
            let val = self.get(&key);

            if val.is_some() && val.unwrap().ne(ignore) {
                if min.is_none() || val.unwrap().lt(min.unwrap()) {
                    min = Some(val.unwrap());
                }
                if max.is_none() || val.unwrap().gt(max.unwrap()) {
                    max = Some(val.unwrap());
                }
            }
        }

        (min, max)
    }
}

type Counting = Data<usize>;

impl Counting {
    fn new() -> Counting {
        Counting {
            b: 0,
            c: 0,
            f: 0,
            h: 0,
            k: 0,
            n: 0,
            o: 0,
            p: 0,
            s: 0,
            v: 0,
        }
    }

    fn increment(&mut self, key: &char) {
        match *key {
            'B' => self.b += 1,
            'C' => self.c += 1,
            'F' => self.f += 1,
            'H' => self.h += 1,
            'K' => self.k += 1,
            'N' => self.n += 1,
            'O' => self.o += 1,
            'P' => self.p += 1,
            'S' => self.s += 1,
            'V' => self.v += 1,
            _ => {}
        }
    }
}

// type Swaps = Data<Data<char>>;
//
// impl Swaps {
//     fn get_for_pair(&self, first: &char, second: &char) -> Option<&char> {
//         self.get(first)
//             .map_or(None, |char_map| char_map.get(second))
//     }
// }

// HashMap access & update is really slow, so faking it using Vec
// type SwapMap = HashMap<char, HashMap<char, char>>;

struct SwapMap {
    data: Vec<Vec<char>>,
}

impl SwapMap {
    fn get(&self, first: &char, second: &char) -> Option<char> {
        self.data
            .get(*first as usize)
            .map_or(None, |char_map| char_map.get(*second as usize))
            .map_or(None, |char| Some(*char))
    }

    fn insert(&mut self, first: char, second: char, value: char) {
        self.ensure_keys(first, second);
        self.data[first as usize][second as usize] = value;
    }

    fn ensure_keys(&mut self, first: char, second: char) {
        if self.data.len() <= first as usize {
            self.data.resize(first as usize + 1, Vec::new());
        }

        if self.data[first as usize].len() <= second as usize {
            self.data[first as usize].resize(second as usize + 1, ' ');
        }
    }
}

struct PairInsertion {
    swaps: SwapMap,
    counts: Counting,
    // add_counter: usize, // Attempt to see what it is that scales exponentially with increase in iterations
}

impl PairInsertion {
    pub fn counts(&mut self, polymer: &Vec<char>, iteration_count: usize) -> &Counting {
        self.add(&polymer[0]);
        for idx in 0..polymer.len() - 1 {
            self.count_recursive(&polymer[idx], &polymer[idx + 1], iteration_count);
            self.add(&polymer[idx + 1]);
        }

        &self.counts
    }

    fn add(&mut self, char: &char) {
        // print!("{}", char);
        self.counts.increment(char);
        // self.add_counter += 1;
    }

    fn count_recursive(&mut self, first: &char, second: &char, iterations_left: usize) {
        if iterations_left == 0 {
            return;
        }

        match self.swaps.get(first, second) {
            None => {}
            Some(insert) => {
                self.count_recursive(first, &insert, iterations_left - 1);
                self.add(&insert);
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

    let mut swaps = SwapMap { data: vec![] };

    for (first, second, insert) in lines.map(parse_swap) {
        swaps.insert(first, second, insert);
    }

    (
        polymer,
        PairInsertion {
            swaps,
            // counts: Counts { data: vec![] },
            counts: Counting::new(),
            // add_counter: 0,
        },
    )
}

fn most_and_least(counts: &Counts) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut max = usize::MIN;

    for count in counts.data.iter().filter(|c| **c != 0) {
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

    // let (most, least) = most_and_least(counts);
    let (least, most) = counts.min_max(&0);

    // println!("Add count: {}", pair_insertion.add_counter);

    most.unwrap() - least.unwrap()
}

pub fn part2(content: &str) -> usize {
    let result = process(content, 26);
    println!("Part2 answer: {}", result);
    result
}

pub fn part1(content: &str) -> usize {
    let result = process(content, 10);
    println!("Part1 answer: {}", result);
    result
}

#[cfg(test)]
mod test_parts {
    use crate::puzzle14::part2_recursive::part1;
    use crate::puzzle14::part2_recursive::part2;
    use crate::puzzle14::part2_recursive::process;

    static TEST_CONTENT: &'static str = include_str!("test.txt");

    #[test]
    fn part2_works() {
        assert_eq!(2188189693529, part2(TEST_CONTENT));
    }

    #[test]
    fn part1_works() {
        assert_eq!(1588, part1(TEST_CONTENT));
    }

    #[test]
    fn process_works() {
        // B = 23;
        // C = 10;
        // H = 5;
        // N = 11;

        assert_eq!(18, process(TEST_CONTENT, 4))
    }
}

#[cfg(test)]
mod test_add {

    use std::collections::HashMap;
    use std::ops::AddAssign;

    static ITERATIONS: usize = 3145729;

    #[test]
    fn addition() {
        let mut count = 0;

        for _ in 0..ITERATIONS {
            count += 1;
        }

        assert_eq!(ITERATIONS, count);
    }

    #[test]
    fn addition_btreemap_get_mut() {
        let key = 'a';

        let mut data = std::collections::BTreeMap::new();
        data.insert(key, 0);

        for _ in 0..ITERATIONS {
            data.get_mut(&key).unwrap().add_assign(1);
        }

        assert_eq!(ITERATIONS, *data.get(&key).unwrap());
    }

    #[test]
    fn addition_map_get_mut() {
        let key = 'a';

        let mut data = HashMap::new();
        data.insert(key, 0);

        for _ in 0..ITERATIONS {
            *data.get_mut(&key).unwrap() += 1;
        }

        assert_eq!(ITERATIONS, *data.get_mut(&key).unwrap());
    }

    #[test]
    fn addition_map_entry_deref() {
        let key = 'a';

        let mut data = HashMap::new();
        data.insert(key, 0);

        for _ in 0..ITERATIONS {
            *data.entry(key).or_insert(0) += 1;
        }

        assert_eq!(ITERATIONS, *data.get(&key).unwrap());
    }

    #[test]
    fn addition_map_entry_add() {
        let key = 'a';

        let mut data = HashMap::new();
        data.insert(key, 0);

        for _ in 0..ITERATIONS {
            data.entry(key).and_modify(|x| *x += 1);
        }

        assert_eq!(ITERATIONS, *data.get(&key).unwrap());
    }

    #[test]
    fn addition_vector() {
        let key = 'a';
        let mut counts = vec![];
        if (counts.len() <= key as usize) {
            counts.resize(key as usize + 1, 0);
        }

        counts[key as usize] = 0;

        for _ in 0..ITERATIONS {
            counts[key as usize] += 1;
        }

        assert_eq!(ITERATIONS, counts[key as usize]);
    }
}

#[cfg(test)]
mod test_map {
    use std::collections::HashMap;

    struct VecMap {
        data: Vec<Vec<usize>>,
    }

    impl VecMap {
        fn get(&mut self, first: char, second: char) -> usize {
            // self.ensure_keys(first, second);
            self.data[first as usize][second as usize]
        }

        fn insert(&mut self, first: char, second: char, value: usize) {
            self.ensure_keys(first, second);
            self.data[first as usize][second as usize] = value;
        }

        fn increment(&mut self, first: char, second: char) {
            self.ensure_keys(first, second);
            self.data[first as usize][second as usize] += 1;
        }

        fn ensure_keys(&mut self, first: char, second: char) {
            if self.data.len() <= first as usize {
                self.data.resize(first as usize + 1, Vec::new());
            }

            if self.data[first as usize].len() <= second as usize {
                self.data[first as usize].resize(second as usize + 1, 0);
            }
        }
    }

    #[test]
    fn map_access() {
        let mut char_map = HashMap::new();
        char_map.insert('b', 0);
        let mut data = HashMap::new();
        data.insert('a', char_map);

        for _ in 0..3_000_000 {
            assert_eq!(0, *data.get(&'a').unwrap().get(&'b').unwrap());
        }
    }

    #[test]
    fn map_fake_with_vectors() {
        let mut data = VecMap { data: vec![] };

        data.insert('a', 'b', 12);

        for _ in 0..3_000_000 {
            assert_eq!(12, data.get('a', 'b'));
        }
    }
}
