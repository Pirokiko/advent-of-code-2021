use crate::puzzle16::parsing::{parse, Version};

pub(crate) fn part1(content: &str) -> Version {
    let packet = parse(content);
    let result = packet.version_sum();
    println!("{}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::puzzle16::part1::part1;

    #[test]
    fn part1_works() {
        assert_eq!(16, part1("8A004A801A8002F478"));
        assert_eq!(12, part1("620080001611562C8802118E34"));
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }
}
