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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while self.current != Token::EOF {
            stmts.push(self.statement());
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        match &self.current {
            Token::Let | Token::Const => self.var_decl(),
            Token::If => self.if_stmt(),
            Token::Looping => self.looping_stmt(),
            Token::For => self.for_stmt(),
            Token::Break => {
                self.advance();
                self.expect(&Token::Symbol(';'));
                Stmt::Break
            }
            Token::Continue => {
                self.advance();
                self.expect(&Token::Symbol(';'));
                Stmt::Continue
            }
            Token::Fn => self.function_stmt(),
            Token::Return => self.return_stmt(),
            Token::Symbol('{') => self.block(),
            _ => self.expression_stmt(),
        }
    }

    fn var_decl(&mut self) -> Stmt {
        let is_const = self.current == Token::Const;
        self.advance();

        let name = match &self.current {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected identifier after let/const"),
        };
        self.advance();
        self.expect(&Token::Operator("=".to_string()));
        let value = self.expression();
        self.expect(&Token::Symbol(';'));

        Stmt::VarDecl { name, value, is_const }
    }

    fn if_stmt(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));
        let condition = self.expression();
        self.expect(&Token::Symbol(')'));
        let then_branch = match self.statement() {
            Stmt::Block(stmts) => stmts,
            s => vec![s],
        };

        let else_branch = if self.current == Token::Else {
            self.advance();
            let stmts = match self.statement() {
                Stmt::Block(stmts) => stmts,
                s => vec![s],
            };
            Some(stmts)
        } else {
            None
        };

        Stmt::If { condition, then_branch, else_branch }
    }

    fn looping_stmt(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));
        let condition = self.expression();
        self.expect(&Token::Symbol(')'));
        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            s => vec![s],
        };
        Stmt::Looping { condition, body }
    }

    fn for_stmt(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));
        let init = Box::new(self.var_decl());
        let condition = self.expression();
        self.expect(&Token::Symbol(';'));
        let update = self.expression();
        self.expect(&Token::Symbol(')'));
        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            s => vec![s],
        };
        Stmt::For { init, condition, update, body }
    }

    fn function_stmt(&mut self) -> Stmt {
        self.advance();

        let name = match &self.current {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected function name"),
        };
        self.advance();

        self.expect(&Token::Symbol('('));
        let mut params = vec![];
        while self.current != Token::Symbol(')') {
            if let Token::Identifier(param) = &self.current {
                params.push(param.clone());
                self.advance();
                if self.current == Token::Symbol(',') {
                    self.advance();
                }
            } else {
                panic!("Expected parameter name, got {:?}", self.current);
            }
        }
        self.expect(&Token::Symbol(')'));

        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            s => vec![s],
        };

        Stmt::Function { name, params, body }
    }

    fn return_stmt(&mut self) -> Stmt {
        self.advance();
        let expr = if self.current == Token::Symbol(';') {
            None
        } else {
            Some(self.expression())
        };
        self.expect(&Token::Symbol(';'));
        Stmt::Return(expr)
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

    fn expression_stmt(&mut self) -> Stmt {
        let expr = self.expression();
        self.expect(&Token::Symbol(';'));
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
            if op == "=" {
                break;
            }
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
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
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
            unexpected => panic!("Unexpected expression: {:?}", unexpected),
        }
    }
}
