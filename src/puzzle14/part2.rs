// use crate::lib::linked_list::LinkedList;
// use std::collections::HashMap;
//
// struct Swap {
//     pattern: String,
//     insert: char,
// }
//
// fn parse_swap(line: &str) -> Swap {
//     let mut parts = line.split(" -> ").map(|part| part.trim());
//     let pattern = String::from(parts.next().unwrap());
//     let insert = parts.next().unwrap().chars().next().unwrap();
//     Swap { pattern, insert }
// }
//
// fn parse_polymer(line: &str) -> LinkedList<char> {
//     let mut list = LinkedList::new();
//     for char in line.chars() {
//         list.append(char);
//     }
//     list
// }
//
// fn parse(content: &str) -> (LinkedList<char>, Vec<Swap>) {
//     let mut lines = content.lines();
//     let polymer = parse_polymer(lines.next().unwrap());
//     lines.next(); // skip empy line
//
//     let swaps = lines.map(parse_swap).collect();
//
//     (polymer, swaps)
// }
//
// fn insert(polymer: &mut LinkedList<char>, swaps: &Vec<Swap>) {
//     let mut iter = polymer.iter_mut();
//     let mut prev = iter.next().unwrap();
//     let mut index = 0;
//     for node in iter {
//         let pattern = format!("{}{}", prev.value, node.value);
//
//         let swap = swaps.iter().find(|swap| swap.pattern.eq(&pattern));
//         match swap {
//             None => {}
//             Some(swap) => {
//                 polymer.before(swap.insert, node.index);
//             }
//         }
//
//         index += 1;
//         prev = node;
//     }
// }
//
// fn most_and_least(polymer: &LinkedList<char>) -> (usize, usize) {
//     let mut map: HashMap<char, usize> = HashMap::new();
//
//     for node in polymer.iter() {
//         *map.entry(node.value).or_insert(0) += 1;
//     }
//
//     (*map.values().max().unwrap(), *map.values().min().unwrap())
// }
//
// pub fn part2(content: &str) -> usize {
//     let (mut polymer, swaps) = parse(content);
//     for idx in 0..40 {
//         println!("Insert {}", idx);
//         insert(&mut polymer, &swaps);
//     }
//
//     let (most, least) = most_and_least(&polymer);
//     let result = most - least;
//     println!("Part1 answer: {}", result);
//     result
// }
//
// #[cfg(test)]
// mod test {
//     use crate::puzzle14::part2::part2;
//
//     static TEST_CONTENT: &'static str = include_str!("test.txt");
//
//     #[test]
//     fn part2_works() {
//         assert_eq!(2188189693529, part2(TEST_CONTENT));
//     }
// }
