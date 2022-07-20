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

fn parse(content: &str) -> Vec<Act> {
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

pub fn part1(content: &str) {
    let actions = parse(content);

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

pub fn part2(content: &str) {
    let actions = parse(content);

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
