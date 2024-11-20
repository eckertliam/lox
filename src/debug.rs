use crate::{chunk::Chunk, opcode::OpCode, value::Value};

pub fn disassemble_chunk(chunk: &Chunk) {
    println!("== {} ==", chunk.name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

macro_rules! simple_instruction {
    ($name:tt, $offset:expr) => {
        {
            println!("{}", stringify!($name));
            $offset + 1
        }
    };
}

macro_rules! constant_instruction {
    ($name:tt, $offset:expr, $chunk:expr) => {
        {
            let const_idx: u8 = $chunk.code[$offset + 1];
            let value = $chunk.get_const(const_idx as usize);
            println!("{}    {} {}", stringify!($name), const_idx, value);
            $offset + 2
        }
    };
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let byte = chunk.code[offset];
    
    let instr = if let Ok(instr) = OpCode::try_from(byte) {
        instr
    } else {
        eprintln!("Invalid opcode: {}", byte);
        return offset + 1;
    };

    match instr {
        OpCode::Constant => constant_instruction!(CONSTANT, offset, chunk),
        OpCode::Return => simple_instruction!(RETURN, offset),
    }
}
