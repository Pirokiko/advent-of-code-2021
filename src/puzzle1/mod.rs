fn nr_of_increases(nrs: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = i32::MAX;
    for nr in nrs {
        if nr > prev {
            count += 1;
        }
        prev = nr;
    }
    count
}

fn nrs(content: &str) -> Vec<i32> {
    content
        .lines()
        .map(|line| line.parse().expect("input to be string"))
        .collect()
}

pub fn part1(content: &str) {
    let count = nr_of_increases(nrs(content));

    println!("The slope count is: {}", count);
}

pub fn part2(content: &str) {
    let nrs = nrs(content);

    let mut windowed_nrs = nrs.clone();

    for (idx, nr) in nrs[1..].iter().enumerate() {
        windowed_nrs[idx] = windowed_nrs[idx] + nr;
    }

    for (idx, nr) in nrs[2..].iter().enumerate() {
        windowed_nrs[idx] = windowed_nrs[idx] + nr;
    }
    // Last two don't make a window by themselves
    windowed_nrs.pop();
    windowed_nrs.pop();

    let count = nr_of_increases(windowed_nrs);

    println!(
        "The slope count for three measurement windows is: {}",
        count
    );
}
