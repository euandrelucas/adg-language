#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Looping,
    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    Throw,
    Try,
    Catch,
    Switch,
    Case,
    Break,
    Continue,
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
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current = self.input[self.position];

        // Número
        if current.is_ascii_digit() {
            return self.lex_number();
        }

        // Identificador ou palavra-chave
        if current.is_ascii_alphabetic() || current == '_' {
            return self.lex_identifier_or_keyword();
        }

        // String
        if current == '"' {
            return self.lex_string();
        }

        // Operadores e símbolos
        match current {
            '+' | '-' | '*' | '/' => {
                self.position += 1;
                Token::Operator(current.to_string())
            }

            '=' | '!' | '<' | '>' => {
                let mut op = current.to_string();
                if self.peek_char() == Some('=') {
                    self.position += 1;
                    op.push('=');
                }
                self.position += 1;
                Token::Operator(op)
            }

            '{' | '}' | '(' | ')' | '[' | ']' | ';' | ':' | ',' | '.' => {
                self.position += 1;
                Token::Symbol(current)
            }

            _ => {
                self.position += 1;
                Token::Unknown(current.to_string())
            }
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }

        if self.position < self.input.len() && self.input[self.position] == '.' {
            self.position += 1;
            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
        }

        let num_str: String = self.input[start..self.position].iter().collect();
        Token::Number(num_str.parse().unwrap_or(0.0))
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && (self.input[self.position].is_ascii_alphanumeric()
                || self.input[self.position] == '_')
        {
            self.position += 1;
        }

        let ident: String = self.input[start..self.position].iter().collect();

        match ident.as_str() {
            "looping" => Token::Looping,
            "let" => Token::Let,
            "const" => Token::Const,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "return" => Token::Return,
            "throw" => Token::Throw,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(ident),
        }
    }

    fn lex_string(&mut self) -> Token {
        self.position += 1; // pula o "
        let start = self.position;

        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.position += 1;
        }

        let content: String = self.input[start..self.position].iter().collect();
        self.position += 1; // pula o fechamento "

        Token::String(content)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }
}
