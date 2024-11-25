use crate::{opcode::OpCode, value::{Value, ValueArray}};

pub struct Chunk {
    pub name: String,
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(), 
            code: Vec::new(), 
            lines: Vec::new(), 
            constants: ValueArray::new() 
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.write(opcode as u8, line);
    }

    pub fn add_const(&mut self, value: Value) -> u8 {
        self.constants.write(value)
    }

    pub fn get_const(&self, index: u8) -> Value {
        self.constants.get(index as usize)
    }
}


impl Default for Chunk {
    fn default() -> Self {
        Self::new("chunk")
    }
}
