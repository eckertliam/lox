use crate::{opcode::OpCode, value::Value};

#[derive(Debug)]
pub struct Chunk {
    pub name: String,
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(), 
            code: Vec::new(), 
            lines: Vec::new(), 
            constants: Vec::new() 
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.write(opcode as u8, line);
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_const(&self, index: usize) -> Value {
        self.constants[index].clone()
    }
}


impl Default for Chunk {
    fn default() -> Self {
        Self::new("chunk")
    }
}
