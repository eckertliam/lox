use std::env;

use vm::{InterpretResult, VM};

mod opcode;
mod chunk;
#[cfg(feature = "debug")]
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;

const VERSION: &str = "0.0.1";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // TODO: Implement repl
        unimplemented!("Implement repl");
    } else if args.len() == 2 {
        // TODO: Implement run file
        unimplemented!("Implement run file");
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}
