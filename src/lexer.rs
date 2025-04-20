#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Const,
    Fn,
    Return,
    If,
    Else,
    Looping,
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Operator(String),
    Symbol(char),
    EOF,
    Unknown(String),
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(&c) = self.input.get(self.pos) {
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    self.pos += 1;
                }
                '0'..='9' => tokens.push(self.lex_number()),
                '"' => tokens.push(self.lex_string()),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.lex_identifier()),
                '+' | '-' | '*' | '/' | '>' | '<' | '=' | '!' => tokens.push(self.lex_operator()),
                '{' | '}' | '(' | ')' | ';' | ',' => {
                    tokens.push(Token::Symbol(c));
                    self.pos += 1;
                }
                _ => {
                    tokens.push(Token::Unknown(c.to_string()));
                    self.pos += 1;
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        while let Some(c) = self.input.get(self.pos) {
            if c.is_ascii_digit() || *c == '.' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let num_str: String = self.input[start..self.pos].iter().collect();
        let number = num_str.parse().unwrap_or(0.0);
        Token::Number(number)
    }

    fn lex_string(&mut self) -> Token {
        self.pos += 1; // skip opening quote
        let start = self.pos;
        while let Some(&c) = self.input.get(self.pos) {
            if c == '"' {
                break;
            }
            self.pos += 1;
        }
        let content: String = self.input[start..self.pos].iter().collect();
        self.pos += 1; // skip closing quote
        Token::String(content)
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.pos;
        while let Some(&c) = self.input.get(self.pos) {
            if c.is_alphanumeric() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }

        let ident: String = self.input[start..self.pos].iter().collect();
        match ident.as_str() {
            "let" => Token::Let,
            "const" => Token::Const,
            "fn" => Token::Fn,
            "giveback" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "looping" => Token::Looping,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(ident),
        }
    }

    fn lex_operator(&mut self) -> Token {
        let mut op = self.input[self.pos].to_string();
        if let Some(next) = self.input.get(self.pos + 1) {
            if *next == '=' {
                op.push('=');
                self.pos += 1;
            }
        }
        self.pos += 1;
        Token::Operator(op)
    }
}
