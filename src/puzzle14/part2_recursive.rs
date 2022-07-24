use std::collections::HashMap;

// Using a specialized datastructure with specific keys, was easily over 16 times faster
// than using HashMap & still orders faster than using Vec with char indexes

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
    fn new(default: T) -> Data<T> {
        Data {
            b: default.clone(),
            c: default.clone(),
            f: default.clone(),
            h: default.clone(),
            k: default.clone(),
            n: default.clone(),
            o: default.clone(),
            p: default.clone(),
            s: default.clone(),
            v: default.clone(),
        }
    }

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

    fn get_mut(&mut self, key: &char) -> Option<&mut T> {
        match *key {
            'B' => Some(&mut self.b),
            'C' => Some(&mut self.c),
            'F' => Some(&mut self.f),
            'H' => Some(&mut self.h),
            'K' => Some(&mut self.k),
            'N' => Some(&mut self.n),
            'O' => Some(&mut self.o),
            'P' => Some(&mut self.p),
            'S' => Some(&mut self.s),
            'V' => Some(&mut self.v),
            _ => None,
        }
    }

    fn set(&mut self, key: &char, value: T) {
        match *key {
            'B' => self.b = value,
            'C' => self.c = value,
            'F' => self.f = value,
            'H' => self.h = value,
            'K' => self.k = value,
            'N' => self.n = value,
            'O' => self.o = value,
            'P' => self.p = value,
            'S' => self.s = value,
            'V' => self.v = value,
            _ => {}
        }
    }

    fn min_max(&self, ignore: &T) -> (Option<&T>, Option<&T>) {
        let mut min: Option<&T> = None;
        let mut max: Option<&T> = None;

        for val in vec!['B', 'C', 'F', 'H', 'K', 'N', 'O', 'P', 'S', 'V']
            .iter()
            .map(|key| self.get(key))
            .filter(|opt| opt.is_some() && opt.unwrap().ne(ignore))
        {
            if min.is_none() || val.unwrap().lt(min.unwrap()) {
                min = Some(val.unwrap());
            }
            if max.is_none() || val.unwrap().gt(max.unwrap()) {
                max = Some(val.unwrap());
            }
        }

        (min, max)
    }
}

type Counting = Data<usize>;

impl Counting {
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

    fn merge(&mut self, counts: &Counting) {
        self.b += counts.b;
        self.c += counts.c;
        self.f += counts.f;
        self.h += counts.h;
        self.k += counts.k;
        self.n += counts.n;
        self.o += counts.o;
        self.p += counts.p;
        self.s += counts.s;
        self.v += counts.v;
    }
}

struct SwapMap {
    data: Data<Data<char>>,
}

impl SwapMap {
    fn get(&self, first: &char, second: &char) -> Option<char> {
        self.data
            .get(first)
            .map_or(None, |char_map| char_map.get(second))
            .map_or(None, |char| Some(*char))
    }

    fn insert(&mut self, first: char, second: char, value: char) {
        self.data.get_mut(&first).unwrap().set(&second, value);
    }
}

struct PairInsertion {
    swaps: SwapMap,
    counts: Counting,
    cache: HashMap<(char, char, usize), Counting>,
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
        self.counts.increment(char);
    }

    fn count_recursive(&mut self, first: &char, second: &char, iterations_left: usize) -> Counting {
        if iterations_left == 0 {
            return Counting::new(0);
        }

        match self.cache.get(&(*first, *second, iterations_left)) {
            None => {}
            Some(cached_counts) => {
                self.counts.merge(cached_counts);
                return cached_counts.clone();
            }
        }

        let cache = self.cache.get(&(*first, *second, iterations_left));
        if cache.is_some() {
            self.counts.merge(cache.unwrap());
            return cache.unwrap().clone();
        }

        let mut counting = Counting::new(0);
        match self.swaps.get(first, second) {
            None => {}
            Some(insert) => {
                let first_search = self.count_recursive(first, &insert, iterations_left - 1);
                counting.merge(&first_search);

                self.add(&insert);
                counting.increment(&insert);

                let after_search = self.count_recursive(&insert, second, iterations_left - 1);
                counting.merge(&after_search);
            }
        }

        let result = counting.clone();
        // Save for future use
        self.cache
            .insert((*first, *second, iterations_left), counting);
        result
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

    let mut swaps = SwapMap {
        data: Data::new(Data::new(' ')),
    };

    for (first, second, insert) in lines.map(parse_swap) {
        swaps.insert(first, second, insert);
    }

    (
        polymer,
        PairInsertion {
            swaps,
            counts: Counting::new(0),
            cache: HashMap::new(),
        },
    )
}

fn process(content: &str, iteration_count: usize) -> usize {
    let (polymer, mut pair_insertion) = parse(content);

    let counts = pair_insertion.counts(&polymer, iteration_count);

    let (least, most) = counts.min_max(&0);

    most.unwrap() - least.unwrap()
}

pub fn part2(content: &str) -> usize {
    let result = process(content, 40);
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
        if counts.len() <= key as usize {
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
