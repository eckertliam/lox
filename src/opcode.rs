#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Constant),
            1 => Ok(Self::Return),
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}
