pub mod runtime;

mod lexer;
mod parser;
mod interpreter;

use std::fs;
use std::env;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: adg arquivo.adg");
        return;
    }

    let filename = &args[1];
    let code = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Não foi possível ler o arquivo {}", filename));

    let lexer = Lexer::new(&code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
}
