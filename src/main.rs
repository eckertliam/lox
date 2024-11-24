use chunk::Chunk;
use opcode::OpCode;
use vm::VM;

mod opcode;
mod chunk;
#[cfg(feature = "debug")]
mod debug;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new("test");
    let mut const_idx = chunk.add_const(1.2);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(const_idx, 123);
    const_idx = chunk.add_const(3.4);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(const_idx, 123);
    chunk.write_opcode(OpCode::Add, 123);
    const_idx = chunk.add_const(5.6);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(const_idx, 123);
    chunk.write_opcode(OpCode::Divide, 123);
    chunk.write_opcode(OpCode::Negate, 123);
    chunk.write_opcode(OpCode::Return, 123);

    let mut vm = VM::new(&chunk);
    vm.interpret();
    #[cfg(feature = "debug")]
    {
        debug::disassemble_chunk(&chunk);
    }
}
