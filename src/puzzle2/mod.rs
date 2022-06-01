use std::fs;
use std::process::exit;

enum Action {
    FORWARD,
    DOWN,
    UP,
}

struct Act {
    action: Action,
    value: i32,
}

fn parse() -> Vec<Act> {
    let content = fs::read_to_string("F:\\AdventOfCode\\puzzle2.txt").expect("to read input file");
    content
        .lines()
        .map(|line| {
            let (action_str, value_str) =
                line.split_once(" ").expect("line should contain a space");
            let value = value_str.parse().expect("value to be an integer");
            match action_str {
                "forward" => Act {
                    action: Action::FORWARD,
                    value,
                },
                "down" => Act {
                    action: Action::DOWN,
                    value,
                },
                "up" => Act {
                    action: Action::UP,
                    value,
                },
                _ => exit(1),
            }
        })
        .collect()
}

pub fn part1() {
    let actions = parse();

    let mut depth = 0;
    let mut pos = 0;

    for action in actions {
        match action.action {
            Action::FORWARD => pos += action.value,
            Action::DOWN => depth += action.value,
            Action::UP => depth -= action.value,
        }
    }

    println!(
        "Depth: {}, Position: {}, Result: {}",
        depth,
        pos,
        depth * pos
    )
}

pub fn part2() {
    let actions = parse();

    let mut aim = 0;
    let mut depth = 0;
    let mut pos = 0;

    for action in actions {
        match action.action {
            Action::FORWARD => {
                pos += action.value;
                depth += action.value * aim
            }
            Action::DOWN => aim += action.value,
            Action::UP => aim -= action.value,
        }
    }

    println!(
        "Depth: {}, Position: {}, Result: {}",
        depth,
        pos,
        depth * pos
    )
}
