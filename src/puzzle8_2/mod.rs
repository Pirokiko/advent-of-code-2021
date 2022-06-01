mod number_segments;

pub mod puzzle8 {}

fn part2_process(content: &str) -> usize {
    content
        .lines()
        .map(|line| number_segments::determine_number(line))
        .sum()
}

pub(crate) fn part1(content: &str) -> usize {
    let count = content
        .lines()
        .map(|line| {
            let output_values = line
                .split("|")
                .map(|part| part.trim())
                .skip(1)
                .next()
                .expect("output values to exist");
            output_values
                .split_whitespace()
                .map(|part| part.trim())
                .filter(|part| {
                    part.len() == 2 || part.len() == 3 || part.len() == 4 || part.len() == 7
                })
                .count()
        })
        .sum();

    println!("Part 1 outcome: {}", count);
    count
}

pub(crate) fn part2(content: &str) {
    println!("Part 2 outcome: {}", part2_process(content));
}

#[cfg(test)]
mod tests {
    use crate::puzzle8::part1;

    #[test]
    fn part1_works() {
        let content =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(26, part1(content));
    }
}
