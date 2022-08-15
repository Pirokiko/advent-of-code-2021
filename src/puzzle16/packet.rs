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

#[derive(Debug)]
pub struct BasePacket {
    version: Version,
    type_id: TypeID,
}

impl BasePacket {
    pub fn new(version: Version, type_id: TypeID) -> BasePacket {
        BasePacket { version, type_id }
    }
}

#[derive(Debug)]
pub enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
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
            Operator::ADD => self
                .children
                .iter()
                .fold(0, |acc, child| acc + child.calculate()),
            Operator::SUBTRACT => self
                .children
                .iter()
                .fold(-0, |acc, child| acc - child.calculate()),
            Operator::MULTIPLY => self
                .children
                .iter()
                .fold(1, |acc, child| acc * child.calculate()),
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
