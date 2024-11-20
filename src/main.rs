use chunk::Chunk;
use opcode::OpCode;
use debug::disassemble_chunk;

mod opcode;
mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::new("test");
    chunk.add_const(1.2);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(0, 123);
    chunk.write(OpCode::Return as u8, 123);

    disassemble_chunk(&chunk);
}
