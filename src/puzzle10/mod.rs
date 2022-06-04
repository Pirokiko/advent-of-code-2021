use itertools::Itertools;
use std::collections::VecDeque;

fn open_char_for(char: char) -> char {
    match char {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Invalid char given"),
    }
}

fn closing_char_for(char: char) -> char {
    match char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid char given"),
    }
}

fn closing_for_line(line: &str) -> String {
    let mut stack: VecDeque<char> = VecDeque::new();
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push_front(char),
            ')' | ']' | '}' | '>' => {
                stack.pop_front();
            }
            _ => {}
        }
    }
    stack
        .iter()
        .map(|open_char| closing_char_for(*open_char))
        .collect()
}

fn score_closing_line(line: &str) -> usize {
    println!("Closing line: {}", line);

    let mut score: usize = 0;
    for char in line.chars() {
        score *= 5;
        score += score_for_char_part2(char);
    }
    score
}

fn score_for_char_part2(char: char) -> usize {
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn first_corrupted_char(line: &str) -> Option<char> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push_front(char),
            ')' | ']' | '}' | '>' => match stack.pop_front() {
                Some(opening_char) => {
                    if opening_char != open_char_for(char) {
                        return Some(char);
                    }
                }
                None => {}
            },
            _ => {}
        }
    }
    None
}

fn score_for_char_part1(char: char) -> usize {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn part1(content: &str) -> usize {
    let result = content
        .lines()
        .map(|line| first_corrupted_char(line))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .map(|item| score_for_char_part1(item))
        .sum();

    println!("Sum of scores of corrupted lines: {}", result);
    result
}

pub fn part2(content: &str) -> usize {
    let scores: Vec<usize> = content
        .lines()
        .filter(|line| first_corrupted_char(line).is_none())
        .map(|line| closing_for_line(line))
        .map(|closing_line| score_closing_line(&closing_line))
        .sorted()
        .collect();

    println!("Scores: {:?}", scores);

    let result = scores[scores.len() / 2];
    println!("Median of scores: {}", result);
    result
}
