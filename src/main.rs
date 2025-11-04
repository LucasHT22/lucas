mod tokens;
mod keywords;
mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;
mod errors;

use std::env;
use std::fs;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: lucaslang <arquivo.lucas>");
        eprintln!("Ex.: cargo run -- exemplos/ola.lucas");
        std::process::exit(1);
    }
    let path = &args[1];
    let src = fs::read_to_string(path).expect("Não foi possível ler o arquivo.");
    let mut interp = Interpreter::new();
    interp.run(&src);
}