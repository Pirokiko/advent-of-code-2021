use crate::puzzle8::number_segments::Number::{
    EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO,
};
use std::collections::HashMap;

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

struct Setup {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
}

impl Setup {
    fn patterns(&self) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut chars: Vec<char> = vec![];
        for a in self.a.chars() {
            chars.push(a);
            for b in self.b.chars() {
                if chars.contains(&b) {
                    continue;
                }
                chars.push(b);
                for c in self.c.chars() {
                    if chars.contains(&c) {
                        continue;
                    }
                    chars.push(c);
                    for d in self.d.chars() {
                        if chars.contains(&d) {
                            continue;
                        }
                        chars.push(d);
                        for e in self.e.chars() {
                            if chars.contains(&e) {
                                continue;
                            }
                            chars.push(e);
                            for f in self.f.chars() {
                                if chars.contains(&f) {
                                    continue;
                                }
                                chars.push(f);
                                for g in self.g.chars() {
                                    if chars.contains(&g) {
                                        continue;
                                    }
                                    patterns.push(Pattern {
                                        a,
                                        b,
                                        c,
                                        d,
                                        e,
                                        f,
                                        g,
                                    })
                                }
                                chars.pop();
                            }
                            chars.pop();
                        }
                        chars.pop();
                    }
                    chars.pop();
                }
                chars.pop();
            }
            chars.pop();
        }

        patterns
    }
}

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
        match char {
            'a' => self.a,
            'b' => self.b,
            'c' => self.c,
            'd' => self.d,
            'e' => self.e,
            'f' => self.f,
            'g' => self.g,
            _ => panic!("Unresolvable char ({}) found", char),
        }
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

fn get_pattern_for(line: &Line, nr: Number) -> &str {
    line.patterns
        .iter()
        .filter(|pattern| pattern.len() == nr.pattern().len())
        .next()
        .expect(&format!(
            "A pattern for the number {} should exist",
            nr.value()
        ))
}

fn setup_from_line(line: &Line) -> Setup {
    let one_pattern = get_pattern_for(line, ONE);
    let four_pattern = get_pattern_for(line, FOUR);
    let seven_pattern = get_pattern_for(line, SEVEN);
    let eight_pattern = get_pattern_for(line, EIGHT);

    let a: String = seven_pattern
        .chars()
        .filter(|char| !one_pattern.contains(*char))
        .collect();
    assert!(a.len() == 1);

    // let eg: String = eight_pattern
    //     .chars()
    //     .filter(|char| !four_pattern.contains(*char))
    //     .filter(|char| !a.contains(*char))
    //     .collect();

    let others: String = "abcdefg"
        .chars()
        .filter(|char| !one_pattern.contains(*char))
        // .filter(|char| !eg.contains(*char))
        .filter(|char| !a.contains(*char))
        .collect();

    Setup {
        a,
        b: others.clone(),
        c: String::from(one_pattern),
        d: others.clone(),
        e: others.clone(),
        f: String::from(one_pattern),
        g: others.clone(),
    }
    //
    // let all: String = String::from("abcdefg");
    //
    // Setup {
    //     a: all.clone(),
    //     b: all.clone(),
    //     c: all.clone(),
    //     d: all.clone(),
    //     e: all.clone(),
    //     f: all.clone(),
    //     g: all.clone(),
    // }
}

fn extract_pattern(line: &Line, setup: &Setup) -> Result<usize, String> {
    for pattern in setup.patterns() {
        if let Ok(value) = line.resolve(pattern) {
            return Ok(value);
        }
    }
    Err(String::from("No matches found"))
}

pub fn determine_number(line_str: &str) -> usize {
    let line = Line::from(line_str);
    let setup = setup_from_line(&line);
    extract_pattern(&line, &setup).expect("Value to be possible")
}

#[cfg(test)]
mod tests {
    use crate::puzzle8::number_segments::determine_number;

    #[test]
    fn determine_number_works() {
        // assert_eq!(5353,determine_number("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"));

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
