use crate::puzzle16::packet::Value;
use crate::puzzle16::parsing::parse;

pub(crate) fn part2(content: &str) -> Value {
    let packet = parse(content);
    let result = packet.calculate();
    println!("Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::puzzle16::part2::part2;

    #[test]
    fn part1_works() {
        assert_eq!(3, part2("C200B40A82"));
        assert_eq!(54, part2("04005AC33890"));
        assert_eq!(7, part2("880086C3E88112"));
        assert_eq!(9, part2("CE00C43D881120"));
        assert_eq!(1, part2("D8005AC2A8F0"));
        assert_eq!(0, part2("F600BC2D8F"));
        assert_eq!(0, part2("9C005AC2F8F0"));
        assert_eq!(1, part2("9C0141080250320F1802104A08"));
    }
}
