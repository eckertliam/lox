use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::opcode::OpCode;
use crate::value::Value;

const STACK_MAX: usize = 256;

struct Stack {
    pub values: [Value; STACK_MAX],
    pub top: usize,
}

impl Stack {
    fn new() -> Self {
        const NIL: Value = Value::Nil;
        Self { values: [NIL; STACK_MAX], top: 0 }
    }

    pub fn reset(&mut self) {
        self.top = 0;
    }

    pub fn push(&mut self, value: Value) {
        self.values[self.top] = value;
        self.top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.top -= 1;
        std::mem::replace(&mut self.values[self.top], Value::Nil)
    }

    pub fn peek(&self, distance: usize) -> &Value {
        &self.values[self.top - distance - 1]
    }
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Stack,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self { 
            chunk, 
            ip: 0, 
            stack: Stack::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let chunk = match compile(source) {
            Ok(chunk) => chunk,
            Err(_) => return InterpretResult::CompileError,
        };
        self.chunk = chunk;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = OpCode::from(self.chunk.code[self.ip]);
            self.ip += 1;

            #[cfg(feature = "debug")]
            {
                use crate::debug::disassemble_instruction;
                print!("          ");
                for i in 0..self.stack.top {
                    print!("[ {} ]", self.stack.values[i]);
                }
                println!();
                disassemble_instruction(&self.chunk, self.ip);
            }

            match instruction {
                OpCode::Return => {
                    println!("{}", self.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::Not => {
                    let value: Value = self.stack.pop();
                    match value.not() {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Add => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    match a.add(&b) {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Subtract => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    match a.subtract(&b) {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Multiply => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    match a.multiply(&b) {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Divide => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    match a.divide(&b) {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Modulo => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    match a.modulo(&b) {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Equal => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    self.stack.push(a.equal(&b));
                }
                OpCode::Greater => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    self.stack.push(a.greater(&b));
                }
                OpCode::Less => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    self.stack.push(a.less(&b));
                }
                OpCode::Negate => {
                    let value: Value = self.stack.pop();
                    match value.negate() {
                        Ok(value) => self.stack.push(value),
                        Err(msg) => {
                            eprintln!("{}", msg);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Constant => {
                    let const_idx: u8 = self.chunk.code[self.ip];
                    self.ip += 1;
                    let value: Value = self.chunk.get_const(const_idx as usize);
                    self.stack.push(value);
                }
            }
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new(Chunk::default())
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}