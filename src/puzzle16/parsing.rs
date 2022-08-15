use crate::puzzle16::packet::{
    BasePacket, LiteralPacket, Operator, OperatorPacket, Packet, TypeID, Value, Version,
};

fn parse_version(data: &str) -> Version {
    // println!("parse_version({})", data);
    u8::from_str_radix(data, 2).expect("Should be binary data")
}
fn parse_type_id(data: &str) -> TypeID {
    // println!("parse_type_id({})", data);
    u8::from_str_radix(data, 2).expect("Should be binary data")
}

fn hex_to_binary(char: char) -> String {
    match char {
        '0' => String::from("0000"),
        '1' => String::from("0001"),
        '2' => String::from("0010"),
        '3' => String::from("0011"),
        '4' => String::from("0100"),
        '5' => String::from("0101"),
        '6' => String::from("0110"),
        '7' => String::from("0111"),
        '8' => String::from("1000"),
        '9' => String::from("1001"),
        'A' => String::from("1010"),
        'B' => String::from("1011"),
        'C' => String::from("1100"),
        'D' => String::from("1101"),
        'E' => String::from("1110"),
        'F' => String::from("1111"),
        _ => panic!("Invalid input given"),
    }
}

fn parse_to_binary(content: &str) -> String {
    content.chars().map(hex_to_binary).collect()
}

fn parse_literal_value(content: &str) -> (Value, usize) {
    let mut offset = 0;
    let mut binary_value = String::new();

    while content.as_bytes()[offset] == b'1' {
        binary_value += &content[offset + 1..offset + 5];
        offset += 5;
    }
    binary_value += &content[offset + 1..offset + 5];
    offset += 5;

    (
        Value::from_str_radix(&binary_value, 2).expect("Should be binary"),
        offset,
    )
}

fn parse_literal_packet(base_packet: BasePacket, content: &str) -> (Box<LiteralPacket>, usize) {
    let (value, length) = parse_literal_value(content);
    return (Box::new(LiteralPacket::new(base_packet, value)), length);
}

pub fn parse(content: &str) -> Box<dyn Packet> {
    let binary_content = parse_to_binary(content);
    // println!("binary: {}", &binary_content);
    let (packet, _) = parse_packet(&binary_content);
    packet
}
pub fn parse_packet(content: &str) -> (Box<dyn Packet>, usize) {
    // println!("parse_packet: content: {}", content);
    let version = parse_version(&content[0..3]);
    let type_id = parse_type_id(&content[3..6]);
    let base_packet = BasePacket::new(version, type_id);
    // println!("{:?}", base_packet);

    if type_id == 4 {
        let (packet, length) = parse_literal_packet(base_packet, &content[6..]);
        return (packet, length + 6);
    }
    let (packet, length) = parse_operator_packet(base_packet, &content[6..]);
    (packet, length + 6)
}

fn parse_nr_of_packets(content: &str, amount: usize) -> (Vec<Box<dyn Packet>>, usize) {
    let mut result = vec![];

    let mut offset = 0;
    for _ in 0..amount {
        let (packet, length) = parse_packet(&content[offset..]);
        result.push(packet);
        offset += length;
    }

    (result, offset)
}

fn parse_packets_with_length(content: &str) -> (Vec<Box<dyn Packet>>, usize) {
    // println!("parse_packets_with_length: content: {}", content);
    let mut result = vec![];

    let mut offset = 0;
    while offset < content.len() {
        let (packet, length) = parse_packet(&content[offset..]);
        // println!("parse_packets_with_length: length: {}", length);
        result.push(packet);
        offset += length;
    }

    (result, content.len())
}

fn parse_operator_packet(base_packet: BasePacket, content: &str) -> (Box<OperatorPacket>, usize) {
    let length_type_id = &content[0..1];
    // println!("parse_operator_packet: length_type_id: {}", length_type_id);

    let (children, length) = if length_type_id == "0" {
        // println!("parse length: {}", &content[1..16]);
        let length = usize::from_str_radix(&content[1..16], 2).expect("binary data");
        // println!("length: {}", length);
        let (packet, length) = parse_packets_with_length(&content[16..length + 16]);
        (packet, length + 16)
    } else {
        let nr_of_children = usize::from_str_radix(&content[1..12], 2).expect("binary data");
        // println!("nr_of_children: {}", nr_of_children);
        let (packet, length) = parse_nr_of_packets(&content[12..], nr_of_children);
        (packet, length + 12)
    };

    (
        Box::new(OperatorPacket::new(base_packet, Operator::ADD, children)),
        length,
    )
}
