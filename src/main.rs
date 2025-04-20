mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let code = r#"
        fn greet(name) {
            print("Hello, ");
            print(name);
        }

        greet("André");

        let idade = 18;
        if (idade >= 18) {
            print("maior de idade");
        } else {
            print("menor de idade");
        }

        let i = 0;
        looping (i < 3) {
            print(i);
            i = i + 1;
        }
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    println!("\n🌳 AST:");
    for stmt in &program {
        println!("{:#?}", stmt);
    }

    println!("\n🧠 Execução:");
    let mut interpreter = Interpreter::new();
    interpreter.execute(program);
}