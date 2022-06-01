use crate::puzzle8_2::number_segments::Number::{
    EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO,
};
use std::collections::HashMap;
use std::slice::Iter;

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 */

#[derive(Eq, PartialEq, Hash)]
enum Number {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl Number {
    fn from(str: &str) -> Result<Number, String> {
        let mut chars: Vec<char> = str.chars().collect();
        chars.sort();
        let test: String = String::from_iter(chars.iter());
        match test.as_str() {
            "cf" => Ok(ONE),
            "acf" => Ok(SEVEN),
            "bcdf" => Ok(FOUR),
            "abdfg" => Ok(FIVE),
            "acdeg" => Ok(TWO),
            "acdfg" => Ok(THREE),
            "abcdfg" => Ok(NINE),
            "abcefg" => Ok(ZERO),
            "abdefg" => Ok(SIX),
            "abcdefg" => Ok(EIGHT),
            _ => Err(format!("Can't match '{}' to a number", str)),
        }
    }
    fn pattern(&self) -> &str {
        match *self {
            ONE => "cf",
            SEVEN => "acf",
            FOUR => "bcdf",
            FIVE => "abdfg",
            TWO => "acdeg",
            THREE => "acdfg",
            NINE => "abcdfg",
            ZERO => "abcefg",
            SIX => "abdefg",
            EIGHT => "abcdefg",
        }
    }
    fn value(&self) -> usize {
        match *self {
            ZERO => 0,
            ONE => 1,
            TWO => 2,
            THREE => 3,
            FOUR => 4,
            FIVE => 5,
            SIX => 6,
            SEVEN => 7,
            EIGHT => 8,
            NINE => 9,
        }
    }
    fn char(&self) -> char {
        match *self {
            ZERO => '0',
            ONE => '1',
            TWO => '2',
            THREE => '3',
            FOUR => '4',
            FIVE => '5',
            SIX => '6',
            SEVEN => '7',
            EIGHT => '8',
            NINE => '9',
        }
    }
}

#[derive(Debug)]
struct Pattern {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
}

impl Pattern {
    fn resolve_char(&self, char: char) -> char {
        if self.a == char {
            return 'a';
        }
        if self.b == char {
            return 'b';
        }
        if self.c == char {
            return 'c';
        }
        if self.d == char {
            return 'd';
        }
        if self.e == char {
            return 'e';
        }
        if self.f == char {
            return 'f';
        }
        if self.g == char {
            return 'g';
        }

        panic!("Unresolvable char ({}) found", char)
    }
    fn resolve(&self, output: &str) -> String {
        output.chars().map(|char| self.resolve_char(char)).collect()
    }
}

struct Line {
    patterns: [String; 10],
    output: [Segment; 4],
}

impl Line {
    fn resolve(&self, pattern: Pattern) -> Result<usize, String> {
        let mut nrs: String = String::new();

        let mut index = 0;
        while let Ok(nr) = self.output[index].resolve(&pattern) {
            nrs.push(nr.char());
            index += 1;
            if index >= 4 {
                break;
            }
        }
        if (nrs.len() != 4) {
            return Err(String::from("Not all pattern char matched a Number"));
        }
        Ok(usize::from_str_radix(&nrs, 10).expect("to be a number"))
    }
}

#[derive(Default, Clone)]
struct Segment {
    output: String,
}

impl Segment {
    fn from(output: &str) -> Segment {
        Segment {
            output: String::from(output),
        }
    }
    fn resolve(&self, pattern: &Pattern) -> Result<Number, String> {
        let translated: String = pattern.resolve(&self.output);
        Number::from(&translated)
    }
}

fn string_parts(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|str| String::from(str))
        .collect()
}

impl Line {
    fn from(line: &str) -> Line {
        let mut parts = line.split("|").map(|part| part.trim());

        let mut patterns: [String; 10] = Default::default();
        patterns.clone_from_slice(&*string_parts(parts.next().expect("should exist")));

        let mut output: [Segment; 4] = Default::default();
        output.clone_from_slice(
            &*string_parts(parts.next().expect("should exist"))
                .iter()
                .map(|part| Segment::from(part))
                .collect::<Vec<Segment>>(),
        );

        Line { patterns, output }
    }
}

fn get_filtered_patterns<F>(patterns: Vec<&String>, filters: Vec<F>) -> Vec<&String>
where
    F: Fn(&String) -> bool,
{
    patterns
        .into_iter()
        .filter(|item| filters.iter().all(|filter| filter(item)))
        .collect()
}

fn get_patterns_filtered<P>(line: &Line, filter: P) -> Vec<&String>
where
    P: FnMut(&&String) -> bool,
{
    line.patterns.iter().filter(filter).collect()
}

fn get_patterns_with_length(line: &Line, len: usize) -> Vec<&String> {
    get_patterns_filtered(line, |pattern| pattern.len() == len)
}

fn get_pattern_for(line: &Line, nr: Number) -> &str {
    get_patterns_with_length(line, nr.pattern().len())
        .iter()
        .next()
        .expect(&format!(
            "A pattern for the number {} should exist",
            nr.value()
        ))
}

fn get_a(one_pattern: &str, seven_pattern: &str) -> char {
    let a: String = seven_pattern
        .chars()
        .filter(|char| !one_pattern.contains(*char))
        .collect();
    assert_eq!(a.len(), 1);
    a.chars().next().expect("should exist")
}

fn count_chars(patterns: &Vec<String>) -> HashMap<char, usize> {
    let mut count = HashMap::new();

    for p in patterns {
        for p_char in p.chars() {
            count.entry(p_char).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    count
}

fn remove_string_chars(pattern: &String, chars: &Vec<char>) -> String {
    pattern.chars().filter(|c| !chars.contains(c)).collect()
}

fn remove_chars(patterns: Iter<&String>, chars: &Vec<char>) -> Vec<String> {
    patterns.map(|p| remove_string_chars(p, chars)).collect()
}

fn char_in(char_options: &Vec<char>, pattern: &str) -> char {
    *char_options
        .iter()
        .find(|c| pattern.contains(**c))
        .expect("To exist")
}

fn get_c_d_e(a: char, line: &Line) -> (char, char, char) {
    let one_pattern: &str = get_pattern_for(line, ONE);
    let four_pattern: &str = get_pattern_for(line, FOUR);
    let patterns_of_length_six = remove_chars(get_patterns_with_length(line, 6).iter(), &vec![a]);
    assert_eq!(patterns_of_length_six.len(), 3);

    let count = count_chars(&patterns_of_length_six);

    // c, d & e
    let double_occurences = count
        .into_iter()
        .filter(|(_, cnt)| *cnt == 2)
        .map(|(char, _)| char)
        .collect::<Vec<char>>();
    assert_eq!(double_occurences.len(), 3);

    let c = char_in(&double_occurences, one_pattern);
    let d_and_e = double_occurences
        .into_iter()
        .filter(|char| *char != c)
        .collect::<Vec<char>>();

    let d = char_in(&d_and_e, four_pattern);

    let e = d_and_e
        .into_iter()
        .filter(|char| *char != d)
        .next()
        .expect("to expect");

    (c, d, e)
}

fn get_f(c: char, one_pattern: &str) -> char {
    let f: String = one_pattern.chars().filter(|char| *char != c).collect();
    assert_eq!(f.len(), 1);
    f.chars().next().expect("should exist")
}

fn get_b_and_g(a: char, c: char, d: char, e: char, f: char, line: &Line) -> (char, char) {
    let mut known_chars = vec![a, c, d, e, f];
    let pattern_for_four = String::from(get_pattern_for(line, FOUR));

    let last_char_for_four = remove_string_chars(&pattern_for_four, &known_chars);
    println!("{}", last_char_for_four);
    assert_eq!(last_char_for_four.len(), 1);
    let b = last_char_for_four.chars().next().expect("to exist");

    known_chars.push(b);

    let pattern_for_eight = String::from(get_pattern_for(line, EIGHT));
    let last_char_for_eight = remove_string_chars(&pattern_for_eight, &known_chars);
    assert_eq!(last_char_for_eight.len(), 1);
    let g = last_char_for_eight.chars().next().expect("to exist");
    (b, g)
}

fn setup_from_line(line: &Line) -> Pattern {
    let one_pattern = get_pattern_for(line, ONE);
    let seven_pattern = get_pattern_for(line, SEVEN);

    let patterns_of_length_five = get_patterns_with_length(line, 5);
    assert_eq!(patterns_of_length_five.len(), 3);
    let patterns_of_length_six = get_patterns_with_length(line, 6);
    assert_eq!(patterns_of_length_six.len(), 3);

    let a = get_a(one_pattern, seven_pattern);
    let (c, d, e) = get_c_d_e(a, line);
    let f = get_f(c, one_pattern);
    let (b, g) = get_b_and_g(a, c, d, e, f, line);

    Pattern {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
    }
}

pub fn determine_number(line_str: &str) -> usize {
    let line = Line::from(line_str);
    let setup = setup_from_line(&line);
    println!("{:?}", setup);
    line.resolve(setup).expect("Value to be possible")
}

#[cfg(test)]
mod tests {
    use crate::puzzle8_2::number_segments::determine_number;

    #[test]
    fn determine_number_works() {
        assert_eq!(5353,determine_number("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"));

        assert_eq!(8394,determine_number("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"));
        assert_eq!(9781,determine_number("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"));
        assert_eq!(
            1197,
            determine_number(
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            )
        );
        assert_eq!(9361,determine_number("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"));
        assert_eq!(4873,determine_number("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"));
        assert_eq!(8418,determine_number("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"));
        assert_eq!(4548,determine_number("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"));
        assert_eq!(1625,determine_number("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"));
        assert_eq!(
            8717,
            determine_number(
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            )
        );
        assert_eq!(
            4315,
            determine_number(
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )
        );
    }
}
