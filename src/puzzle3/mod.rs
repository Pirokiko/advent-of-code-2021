fn counts(lines: &Vec<&str>) -> Vec<i32> {
    let mut count = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in lines {
        for (idx, char) in line.chars().enumerate() {
            if char == '1' {
                count[idx] += 1;
            } else {
                count[idx] -= 1;
            }
        }
    }
    count
}

fn gamma_epsilon(lines: &Vec<&str>) -> (String, String) {
    calc_gamma_epsilon(lines, '0', '1')
}

fn calc_gamma_epsilon(lines: &Vec<&str>, base_gamma: char, base_epsilon: char) -> (String, String) {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    // Build the binary string representations
    for c in counts(lines) {
        if c > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else if c < 0 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push(base_gamma);
            epsilon.push(base_epsilon);
        }
    }

    (gamma, epsilon)
}

fn extract_rating(lines: &Vec<&str>, idx: usize, dominant: bool) -> i32 {
    let (dominant_bits, subservient_bits) = calc_gamma_epsilon(&lines, '1', '0');

    let criteria_char: char;
    if dominant {
        criteria_char = dominant_bits.chars().nth(idx).expect("char to exist");
    } else {
        criteria_char = subservient_bits.chars().nth(idx).expect(&format!(
            "{}th char to exist in {}",
            idx + 1,
            subservient_bits
        ));
    }

    let filtered_lines: Vec<&str> = lines
        .into_iter()
        .filter(|line| {
            let char = line.chars().nth(idx).expect("to exist");
            char == criteria_char
        })
        .map(|line| *line)
        .collect();

    if filtered_lines.len() == 1 {
        return i32::from_str_radix(filtered_lines.first().expect("first item to exist"), 2)
            .expect("i32 from binary string");
    }

    extract_rating(&filtered_lines, idx + 1, dominant)
}

pub fn part1(content: &str) {
    let lines: Vec<&str> = content.lines().collect();

    let (gamma, epsilon) = gamma_epsilon(&lines);

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);

    // Parse to decimals
    let gamma_decimal: i32 = i32::from_str_radix(&gamma, 2).expect("binary to be parsed as i32");
    let epsilon_decimal: i32 =
        i32::from_str_radix(&epsilon, 2).expect("binary to be parsed as i32");

    println!(
        "Gamma: {}, Epsilon: {}, Result: {}",
        gamma_decimal,
        epsilon_decimal,
        gamma_decimal * epsilon_decimal
    )
}

pub fn part2(content: &str) {
    let lines: Vec<&str> = content.lines().collect();

    let oxygen_rating = extract_rating(&lines, 0, true);
    let co2_scrubber_rating = extract_rating(&lines, 0, false);

    println!(
        "Oxygen: {}, CO2 scrubber: {}, Result: {}",
        oxygen_rating,
        co2_scrubber_rating,
        oxygen_rating * co2_scrubber_rating
    )
}
