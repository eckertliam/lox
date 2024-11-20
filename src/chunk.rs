use crate::value::{Value, ValueArray};

pub struct Chunk<'chunk> {
    pub name: &'chunk str,
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: ValueArray,
}

impl<'chunk> Chunk<'chunk> {
    pub fn new(name: &'chunk str) -> Self {
        Self { 
            name, 
            code: Vec::new(), 
            lines: Vec::new(), 
            constants: ValueArray::new() 
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.constants.write(value)
    }

    pub fn get_const(&self, index: usize) -> Value {
        self.constants.get(index)
    }
}
