pub type Value = isize;

pub type Version = usize;
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

#[derive(Debug)]
pub struct BasePacket {
    version: Version,
    type_id: TypeID,
}

impl BasePacket {
    pub fn new(version: Version, type_id: TypeID) -> BasePacket {
        BasePacket { version, type_id }
    }

    pub fn type_id(&self) -> TypeID {
        self.type_id
    }
}

#[derive(Debug)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
pub struct LiteralPacket {
    base_packet: BasePacket,
    value: isize,
}

impl LiteralPacket {
    pub fn new(base_packet: BasePacket, value: Value) -> LiteralPacket {
        LiteralPacket { base_packet, value }
    }
}

pub struct OperatorPacket {
    base_packet: BasePacket,
    operator: Operator,
    children: Vec<Box<dyn Packet>>,
}

impl OperatorPacket {
    pub fn new(
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
        match &self.operator {
            Operator::Sum => self
                .children
                .iter()
                .fold(0, |acc, child| acc + child.calculate()),
            Operator::Product => self
                .children
                .iter()
                .fold(1, |acc, child| acc * child.calculate()),
            Operator::Minimum => self
                .children
                .iter()
                .map(|child| child.calculate())
                .min()
                .unwrap(),
            Operator::Maximum => self
                .children
                .iter()
                .map(|child| child.calculate())
                .max()
                .unwrap(),
            Operator::GreaterThan => {
                let mut child_iter = self.children.iter();
                let first = child_iter.next().unwrap();
                let second = child_iter.next().unwrap();
                if first.calculate() > second.calculate() {
                    1
                } else {
                    0
                }
            }
            Operator::LessThan => {
                let mut child_iter = self.children.iter();
                let first = child_iter.next().unwrap();
                let second = child_iter.next().unwrap();
                if first.calculate() < second.calculate() {
                    1
                } else {
                    0
                }
            }
            Operator::EqualTo => {
                let mut child_iter = self.children.iter();
                let first = child_iter.next().unwrap();
                let second = child_iter.next().unwrap();
                if first.calculate() == second.calculate() {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn version_sum(&self) -> Version {
        self.version()
            + self
                .children
                .iter()
                .fold(0, |acc, child| acc + child.version_sum())
    }
}
