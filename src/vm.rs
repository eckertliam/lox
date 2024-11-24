use std::io::{Read, Write};

use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::opcode::OpCode;
use crate::value::Value;

macro_rules! binary_op {
    ($self:expr, $op:tt) => {
        {
            let b: Value = $self.stack.pop();
            let a: Value = $self.stack.pop();
            $self.stack.push(a $op b);
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
        Self { values: [0.0; STACK_MAX], top: 0 }
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
}

pub struct VM<'a> {
    chunk: Chunk<'a>,
    ip: usize,
    stack: Stack,
}

impl<'a> VM<'a> {
    pub fn new(chunk: Chunk<'a>) -> Self {
        Self { chunk, ip: 0, stack: Stack::new() }
    }

    pub fn interpret(&mut self, source: &'a str) -> InterpretResult {
        self.chunk = compile(source);
        self.ip = 0;
        return self.run();
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
                disassemble_instruction(self.chunk, self.ip);
            }

            match instruction {
                OpCode::Return => {
                    println!("{}", self.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
                OpCode::Modulo => binary_op!(self, %),
                OpCode::Negate => {
                    self.stack.values[self.stack.top - 1] = -self.stack.values[self.stack.top - 1];
                }
                OpCode::Constant => {
                    let const_idx: u8 = self.chunk.code[self.ip];
                    self.ip += 1;
                    let value: Value = self.chunk.get_const(const_idx);
                    self.stack.push(value);
                }
            }
        }
        return InterpretResult::Ok;
    }

    pub fn repl(&mut self) -> InterpretResult {
        let mut line = String::new();
        
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            line.clear();
            
            match std::io::stdin().read_line(&mut line) {
                Ok(0) => {// EOF reached
                    // print a new line and break
                    println!();
                    break;
                }
                Ok(_) => {
                    // TODO: call the interpreter with the line
                    unimplemented!();
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    continue;
                }
            }
        }
        
        return InterpretResult::Ok;
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
