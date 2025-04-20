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

    let code = if args.len() > 1 {
        let filename = &args[1];
        fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Não foi possível ler o arquivo {}", filename))
    } else {
        r#"
            fn hello(name) {
                print("Olá, " + name);
            }

            let nome = "André";
            hello(nome);

            const idade = 18;

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

            for (let j = 0; j < 2; j = j + 1) {
                print("j é " + j);
                if (j == 1) {
                    break;
                }
            }

            let raiz = math.sqrt(25);
            print("raiz: " + raiz);

            let potencia = math.pow(2, 3);
            print("potência: " + potencia);

            let aleatorio = math.random();
            print("aleatório: " + aleatorio);
        "#.to_string()
    };

    let lexer = Lexer::new(&code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    println!("🌳 AST:");
    for stmt in &ast {
        println!("{:#?}", stmt);
    }

    println!("\n🧠 Execução:");
    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
}
