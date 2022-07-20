use itertools::Itertools;

#[derive(PartialEq)]
enum Axis {
    X,
    Y,
}

struct Fold {
    axis: Axis,
    value: usize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn parse(content: &str) -> (Vec<Point>, Vec<Fold>) {
    let mut lines = content.lines();

    let points: Vec<Point> = lines
        .by_ref()
        .take_while(|line| (*line).ne(""))
        .map(|line| line.split(","))
        .map(|mut parts| Point {
            x: usize::from_str_radix(parts.next().unwrap(), 10).unwrap(),
            y: usize::from_str_radix(parts.next().unwrap(), 10).unwrap(),
        })
        .collect();

    let folds: Vec<Fold> = lines
        .map(|line| {
            let info = line.replace("fold along", "");
            let mut parts = info.trim().split("=");
            Fold {
                axis: if "x".eq(parts.next().unwrap()) {
                    Axis::X
                } else {
                    Axis::Y
                },
                value: usize::from_str_radix(parts.next().unwrap(), 10).unwrap(),
            }
        })
        .collect();

    (points, folds)
}

fn fold_point(point: &mut Point, fold: &Fold) {
    if Axis::X == fold.axis {
        let diff = fold.value.abs_diff(point.x);
        if point.x > fold.value {
            point.x = fold.value - diff;
        }
    } else {
        let diff = fold.value.abs_diff(point.y);
        if point.y > fold.value {
            point.y = fold.value - diff;
        }
    }
}

fn fold_points(points: &mut Vec<Point>, fold: &Fold) {
    for point in points {
        fold_point(point, fold);
    }
}

pub fn part1(content: &str) -> usize {
    let (mut points, folds) = parse(content);

    fold_points(&mut points, folds.first().unwrap());

    let result = points.iter().unique().count();

    println!("Part1 answer: {}", result);

    result
}

fn points_to_chars(points: Vec<Point>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];
    for point in points.iter().unique() {
        if result.len() <= point.y {
            for _ in result.len()..=point.y {
                result.push(vec![]);
            }
        }
        let row = &mut result[point.y];
        if row.len() <= point.x {
            for _ in row.len()..=point.x {
                row.push(' ');
            }
        }
        row[point.x] = '#';
    }
    result
}

fn visualize(lines: Vec<Vec<char>>) {
    for line in lines {
        for char in line {
            print!("{}", char);
        }
        println!();
    }
}

pub fn part2(content: &str) {
    let (mut points, folds) = parse(content);

    for fold in folds.iter() {
        fold_points(&mut points, fold);
    }

    let lines: Vec<Vec<char>> = points_to_chars(points);
    println!("Part2 answer");
    visualize(lines);
}
