use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Looping,
    Let,
    Const,
    Fn,
    If,
    Else,
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Operator(String),
    Symbol(char),
    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            input: source.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = c.to_string();
                while let Some(&ch) = self.input.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        self.input.next();
                    } else {
                        break;
                    }
                }

                return match ident.as_str() {
                    "let" => Token::Let,
                    "const" => Token::Const,
                    "fn" => Token::Fn,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "looping" => Token::Looping,
                    "true" => Token::Boolean(true),
                    "false" => Token::Boolean(false),
                    _ => Token::Identifier(ident),
                };
            }

            Some(c) if c.is_ascii_digit() => {
                let mut number = c.to_string();
                while let Some(&ch) = self.input.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        number.push(ch);
                        self.input.next();
                    } else {
                        break;
                    }
                }
                return Token::Number(number.parse().unwrap_or(0.0));
            }

            Some('"') => {
                let mut string = String::new();
                while let Some(ch) = self.input.next() {
                    if ch == '"' {
                        break;
                    }
                    string.push(ch);
                }
                return Token::String(string);
            }

            Some(ch) => {
                match ch {
                    '=' | '!' | '<' | '>' => {
                        if let Some('=') = self.input.peek() {
                            self.input.next();
                            return Token::Operator(format!("{}=", ch));
                        }
                        return Token::Operator(ch.to_string());
                    }
                    '+' | '-' | '*' | '/' => return Token::Operator(ch.to_string()),
                    '{' | '}' | '(' | ')' | ';' | ',' => return Token::Symbol(ch),
                    _ => self.next_token(), // ignora caracteres desconhecidos
                }
            }

            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}
