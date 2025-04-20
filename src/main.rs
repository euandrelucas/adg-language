mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let code = r#"
        let nome = "André";
        const idade = 18;
        if (idade >= 18) {
            print("maior de idade");
        } else {
            print("menor de idade");
        }

        let i = 0;
        looping (i < 99999) {
            print(i);
            i = i + 1;
        }
    "#;

    let mut lexer = Lexer::new(code);
    let mut tokens = vec![];
    loop {
        let tok = lexer.next_token();
        if tok == lexer::Token::EOF { break; }
        tokens.push(tok);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("\n🌳 AST:");
    for stmt in &ast {
        println!("{:#?}", stmt);
    }

    println!("\n🧠 Execução:");
    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
}