use std::io::Write;

use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::opcode::OpCode;
use crate::value::Value;

macro_rules! binary_op {
    ($self:expr, $op:tt) => {
        {
            let b: Value = $self.stack.pop();
            let a: Value = $self.stack.pop();
            if let Ok(value) = a $op b {
                $self.stack.push(value);
            } else {
                return InterpretResult::RuntimeError;
            }
        }
    };
}

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
        self.values[self.top]
    }

    pub fn peek(&self, distance: usize) -> Value {
        self.values[self.top - distance - 1]
    }
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Stack,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 0, stack: Stack::new() }
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
                    self.stack.values[self.stack.top - 1] = if let Ok(value) = !self.stack.values[self.stack.top - 1] {
                        value
                    } else {
                        return InterpretResult::RuntimeError;
                    };
                }
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
                OpCode::Modulo => binary_op!(self, %),
                OpCode::Equal => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    self.stack.push(Value::Bool(a.equal(&b)));
                }
                OpCode::Greater => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    self.stack.push(Value::Bool(a.greater(&b)));
                }
                OpCode::Less => {
                    let b: Value = self.stack.pop();
                    let a: Value = self.stack.pop();
                    self.stack.push(Value::Bool(a.less(&b)));
                }
                OpCode::Negate => {
                    self.stack.values[self.stack.top - 1] = if let Ok(value) = -self.stack.values[self.stack.top - 1] {
                        value
                    } else {
                        return InterpretResult::RuntimeError;
                    };
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