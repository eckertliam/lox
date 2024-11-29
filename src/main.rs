use std::{env, io::Write};

use vm::VM;

mod opcode;
mod chunk;
#[cfg(feature = "debug")]
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;
mod object;
mod gc;
const VERSION: &str = "0.0.1";

fn repl() {
    let mut line = String::new();
    let mut vm = VM::default();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        vm.interpret(&input);
    }
}

fn run_file(path: &str) {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    let mut vm = VM::default();
    vm.interpret(&contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}
