mod tokens;
mod keywords;
mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;
mod errors;
mod repl;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            let mut repl = repl::Repl::new();
            repl.run();
        }
        2 => {
            let filename = &args[1];
            run_file(filename);
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Erro ao ler arquivo '{}': {}", filename, err);
        process::exit(1);
    });

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(&contents);
}

fn print_usage() {
    println!("🚀 Lucas - Lucas Language");
    println!("\nUso:");
    println!("  lucas              - Inicia o REPL interativo");
    println!("  lucas <arquivo>    - Executa um arquivo .lucas");
    println!("\nExemplos:");
    println!("  lucas                    # REPL");
    println!("  lucas programa.lucas     # Executa arquivo");
    println!("  lucas exemplos/ola.lucas # Executa exemplo");
}