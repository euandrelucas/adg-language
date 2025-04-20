use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Call(String, Vec<Expr>),
    Assignment(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, value: Expr, is_const: bool },
    Assignment(String, Box<Expr>),
    Expression(Expr),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    Looping { condition: Expr, body: Vec<Stmt> },
    For { init: Box<Stmt>, condition: Expr, update: Expr, body: Vec<Stmt> },
    Break,
    Continue,
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Return(Option<Expr>),
    Block(Vec<Stmt>),
}

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, expected: &Token) {
        if &self.current != expected {
            panic!("Expected {:?}, but found {:?}", expected, self.current);
        }
        self.advance();
    }

    fn expect_operator(&mut self, expected: &str) {
        match &self.current {
            Token::Operator(op) if op == expected => self.advance(),
            other => panic!("Expected operator '{}', but found {:?}", expected, other),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while self.current != Token::EOF {
            stmts.push(self.statement());
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        match self.current {
            Token::Let | Token::Const => self.variable_decl(),
            Token::If => self.if_statement(),
            Token::Looping => self.looping_statement(),
            Token::For => self.for_statement(),
            Token::Break => { self.advance(); Stmt::Break },
            Token::Continue => { self.advance(); Stmt::Continue },
            Token::Fn => self.function_statement(),
            Token::Return => self.return_statement(),
            Token::Symbol('{') => self.block(),
            _ => self.expression_statement(),
        }
    }

    fn variable_decl(&mut self) -> Stmt {
        let is_const = matches!(self.current, Token::Const);
        self.advance();

        let name = match &self.current {
            Token::Identifier(name) => name.clone(),
            _ => panic!("Expected identifier after let/const"),
        };
        self.advance();

        self.expect_operator("=");
        let value = self.expression();

        self.expect(&Token::Symbol(';'));
        Stmt::VarDecl { name, value, is_const }
    }

    fn if_statement(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));
        let condition = self.expression();
        self.expect(&Token::Symbol(')'));

        let then_branch = match self.statement() {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        };

        let else_branch = if self.current == Token::Else {
            self.advance();
            let branch = match self.statement() {
                Stmt::Block(stmts) => stmts,
                stmt => vec![stmt],
            };
            Some(branch)
        } else {
            None
        };

        Stmt::If { condition, then_branch, else_branch }
    }

    fn looping_statement(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));
        let condition = self.expression();
        self.expect(&Token::Symbol(')'));

        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        };

        Stmt::Looping { condition, body }
    }

    fn for_statement(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));

        let init = Box::new(self.variable_decl());
        let condition = self.expression();
        self.expect(&Token::Symbol(';'));
        let update = self.expression();

        self.expect(&Token::Symbol(')'));
        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        };

        Stmt::For { init, condition, update, body }
    }

    fn function_statement(&mut self) -> Stmt {
        self.advance();

        let name = match &self.current {
            Token::Identifier(s) => s.clone(),
            _ => panic!("Expected function name"),
        };
        self.advance();

        self.expect(&Token::Symbol('('));
        let mut params = vec![];
        while self.current != Token::Symbol(')') {
            if let Token::Identifier(s) = &self.current {
                params.push(s.clone());
                self.advance();
                if self.current == Token::Symbol(',') {
                    self.advance();
                }
            } else {
                panic!("Expected parameter name");
            }
        }
        self.expect(&Token::Symbol(')'));

        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        };

        Stmt::Function { name, params, body }
    }

    fn return_statement(&mut self) -> Stmt {
        self.advance();
        if self.current == Token::Symbol(';') {
            self.advance();
            Stmt::Return(None)
        } else {
            let expr = self.expression();
            self.expect(&Token::Symbol(';'));
            Stmt::Return(Some(expr))
        }
    }

    fn block(&mut self) -> Stmt {
        self.expect(&Token::Symbol('{'));
        let mut stmts = vec![];
        while self.current != Token::Symbol('}') && self.current != Token::EOF {
            stmts.push(self.statement());
        }
        self.expect(&Token::Symbol('}'));
        Stmt::Block(stmts)
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        if self.current == Token::Symbol(';') {
            self.advance();
        } else if self.current != Token::Symbol('}') {
            panic!("Expected ';' after expression, but found {:?}", self.current);
        }
        Stmt::Expression(expr)
    }    

    fn expression(&mut self) -> Expr {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Expr {
        let expr = self.binary_expr();
    
        if let Token::Operator(op) = &self.current {
            if op == "=" {
                self.advance();
                if let Expr::Variable(name) = expr {
                    let value = self.assignment();
                    return Expr::Assignment(name, Box::new(value));
                } else {
                    panic!("Invalid assignment target");
                }
            }
        }
    
        expr
    }    

    fn binary_expr(&mut self) -> Expr {
        let mut left = self.primary();
    
        while let Token::Operator(op) = &self.current {
            if op == "=" { break; } // não trata '=' como operador aqui
            let op = op.clone();
            self.advance();
            let right = self.primary();
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
    
        left
    }    

    fn primary(&mut self) -> Expr {
        match &self.current {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Expr::Literal(Literal::Number(val))
            }
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                Expr::Literal(Literal::String(val))
            }
            Token::Boolean(b) => {
                let val = *b;
                self.advance();
                Expr::Literal(Literal::Boolean(val))
            }
            Token::Identifier(_) => self.parse_call_or_variable(),
            Token::Symbol(';') => {
                self.advance();
                Expr::Literal(Literal::Null)
            }
            unexpected => panic!("Unexpected expression: {:?}", unexpected),
        }
    }       

    fn parse_call_or_variable(&mut self) -> Expr {
        let mut name = match &self.current {
            Token::Identifier(s) => s.clone(),
            _ => panic!("Expected identifier"),
        };
        self.advance();

        while self.current == Token::Symbol('.') {
            self.advance();
            let part = match &self.current {
                Token::Identifier(s) => s.clone(),
                _ => panic!("Expected identifier after '.'"),
            };
            self.advance();
            name = format!("{}.{}", name, part);
        }

        if self.current == Token::Symbol('(') {
            self.advance();
            let mut args = vec![];
            while self.current != Token::Symbol(')') {
                args.push(self.expression());
                if self.current == Token::Symbol(',') {
                    self.advance();
                }
            }
            self.expect(&Token::Symbol(')'));
            Expr::Call(name, args)
        } else {
            Expr::Variable(name)
        }
    }
}
