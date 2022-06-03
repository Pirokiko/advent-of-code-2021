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

fn first_corrupted_char(line: &str) -> Option<char> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for char in line.chars() {
        match char {
            '(' => stack.push_front(char),
            '[' => stack.push_front(char),
            '{' => stack.push_front(char),
            '<' => stack.push_front(char),
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

fn score_for_char(char: char) -> usize {
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
        .map(|item| score_for_char(item))
        .sum();

    println!("Sum of scores of corrupted lines: {}", result);
    result
}

pub fn part2(content: &str) -> usize {
    0
}
