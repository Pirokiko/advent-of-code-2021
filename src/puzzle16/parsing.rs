use std::isize;

pub type Value = isize;

pub type Version = u8;
pub type TypeID = u8;

pub trait Packet {
    fn base(&self) -> &BasePacket;

    fn version(&self) -> Version {
        self.base().version
    }
    fn type_id(&self) -> TypeID {
        self.base().type_id
    }

    fn calculate(&self) -> Value;

    fn version_sum(&self) -> Version;
}

struct BasePacket {
    version: Version,
    type_id: TypeID,
}

impl BasePacket {
    fn new(version: Version, type_id: TypeID) -> BasePacket {
        BasePacket { version, type_id }
    }
}

enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
}

struct LiteralPacket {
    base_packet: BasePacket,
    value: isize,
}

impl LiteralPacket {
    fn new(base_packet: BasePacket, value: Value) -> LiteralPacket {
        LiteralPacket { base_packet, value }
    }
}

struct OperatorPacket {
    base_packet: BasePacket,
    operator: Operator,
    children: Vec<Box<dyn Packet>>,
}

impl OperatorPacket {
    fn new(
        base_packet: BasePacket,
        operator: Operator,
        children: Vec<Box<dyn Packet>>,
    ) -> OperatorPacket {
        OperatorPacket {
            base_packet,
            operator,
            children,
        }
    }
}

impl Packet for LiteralPacket {
    fn base(&self) -> &BasePacket {
        &self.base_packet
    }

    fn calculate(&self) -> Value {
        self.value
    }

    fn version_sum(&self) -> Version {
        self.version()
    }
}

impl Packet for OperatorPacket {
    fn base(&self) -> &BasePacket {
        &self.base_packet
    }

    fn calculate(&self) -> Value {
        let t: u8 = 4;
        let s: u8 = 4;

        let q = t + s;
        match &self.operator {
            Operator::ADD => self
                .children
                .iter()
                .fold(0, |acc, child| acc + child.calculate()),
            Operator::SUBTRACT => self
                .children
                .iter()
                .fold(-0, |acc, child| acc - child.calculate()),
            Operator::MULTIPLY => 2,
        }
    }

    fn version_sum(&self) -> Version {
        self.version()
            + self
                .children
                .iter()
                .fold(0, |acc, child| acc + child.version())
    }
}

fn parse_version(data: &str) -> Version {
    u8::from_str_radix(data, 2).expect("Should be binary data")
}
fn parse_type_id(data: &str) -> TypeID {
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

pub fn parse(content: &str) -> Box<dyn Packet> {
    parse_packet(content)
}
pub fn parse_packet(content: &str) -> Box<dyn Packet> {
    let header = &content[0..6];
    let version = parse_version(&header[0..3]);
    let type_id = parse_type_id(&header[3..6]);
    let base_packet = BasePacket::new(version, type_id);

    if type_id == 4 {
        let (value, offset) = parse_literal_value(&content[6..]);
        return Box::new(LiteralPacket::new(base_packet, value));
    }

    parse_operator_packet(base_packet, &content[7..])
}

fn parse_nr_of_packets(content: &str, amount: usize) -> Vec<Box<dyn Packet>> {
    let result = vec![];

    result
}

fn parse_packets_with_length(content: &str) -> Vec<Box<dyn Packet>> {
    let result = vec![];

    result
}

fn parse_operator_packet(base_packet: BasePacket, content: &str) -> Box<OperatorPacket> {
    let length_type_id = content.as_bytes()[0];

    let children = if length_type_id == b'0' {
        let total_length = usize::from_str_radix(&content[1..16], 2).expect("binary data");
        parse_packets_with_length(&content[16..total_length + 16])
    } else {
        let nr_of_children = usize::from_str_radix(&content[1..16], 2).expect("binary data");
        parse_nr_of_packets(&content[16..], nr_of_children)
    };

    Box::new(OperatorPacket::new(base_packet, Operator::ADD, children))
}
