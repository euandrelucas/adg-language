use crate::lexer::Token;

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
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, value: Expr, is_const: bool },
    Assign { name: String, value: Expr },
    Expression(Expr),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    Looping { condition: Expr, body: Vec<Stmt> },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Return(Expr),
    Block(Vec<Stmt>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens.get(self.current).cloned().unwrap_or(Token::EOF);
        self.current += 1;
        token
    }

    fn expect(&mut self, expected: &Token) {
        if self.peek() != expected {
            panic!("Expected {:?}, got {:?}", expected, self.peek());
        }
        self.advance();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while *self.peek() != Token::EOF {
            stmts.push(self.statement());
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        match self.peek() {
            Token::Let | Token::Const => self.var_decl(),
            Token::Fn => self.function_decl(),
            Token::Return => self.return_stmt(),
            Token::If => self.if_stmt(),
            Token::Looping => self.looping_stmt(),
            Token::Symbol('{') => self.block(),
            Token::Identifier(_) => self.assign_or_expr_stmt(),
            _ => panic!("Unexpected token in statement: {:?}", self.peek()),
        }
    }

    fn var_decl(&mut self) -> Stmt {
        let is_const = matches!(self.peek(), Token::Const);
        self.advance();

        let name = match self.advance() {
            Token::Identifier(name) => name,
            t => panic!("Expected identifier, got {:?}", t),
        };

        self.expect(&Token::Operator("=".to_string()));
        let value = self.expression();
        self.expect(&Token::Symbol(';'));

        Stmt::VarDecl { name, value, is_const }
    }

    fn assign_or_expr_stmt(&mut self) -> Stmt {
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => panic!("Expected identifier"),
        };

        if let Token::Operator(op) = self.peek() {
            if op == "=" {
                self.advance();
                let value = self.expression();
                self.expect(&Token::Symbol(';'));
                return Stmt::Assign { name, value };
            }
        }

        if *self.peek() == Token::Symbol('(') {
            self.advance();
            let mut args = vec![];
            if *self.peek() != Token::Symbol(')') {
                loop {
                    args.push(self.expression());
                    if *self.peek() == Token::Symbol(',') {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect(&Token::Symbol(')'));
            self.expect(&Token::Symbol(';'));
            return Stmt::Expression(Expr::Call(name, args));
        }

        panic!("Invalid expression statement starting with identifier '{}'.", name)
    }

    fn function_decl(&mut self) -> Stmt {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(n) => n,
            t => panic!("Expected function name, got {:?}", t),
        };

        self.expect(&Token::Symbol('('));
        let mut params = vec![];
        if *self.peek() != Token::Symbol(')') {
            loop {
                if let Token::Identifier(p) = self.advance() {
                    params.push(p);
                } else {
                    panic!("Expected parameter name");
                }
                if *self.peek() == Token::Symbol(',') {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(&Token::Symbol(')'));
        let body = match self.statement() {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        };

        Stmt::Function { name, params, body }
    }

    fn return_stmt(&mut self) -> Stmt {
        self.advance();
        let expr = self.expression();
        self.expect(&Token::Symbol(';'));
        Stmt::Return(expr)
    }

    fn if_stmt(&mut self) -> Stmt {
        self.advance();
        self.expect(&Token::Symbol('('));
        let condition = self.expression();
        self.expect(&Token::Symbol(')'));

        let then_branch = match self.statement() {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        };

        let else_branch = if let Token::Else = self.peek() {
            self.advance();
            Some(match self.statement() {
                Stmt::Block(stmts) => stmts,
                stmt => vec![stmt],
            })
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
            stmt => vec![stmt],
        };

        Stmt::Looping { condition, body }
    }

    fn block(&mut self) -> Stmt {
        self.expect(&Token::Symbol('{'));
        let mut stmts = vec![];
        while *self.peek() != Token::Symbol('}') && *self.peek() != Token::EOF {
            stmts.push(self.statement());
        }
        self.expect(&Token::Symbol('}'));
        Stmt::Block(stmts)
    }

    fn expression(&mut self) -> Expr {
        self.binary_expr()
    }

    fn binary_expr(&mut self) -> Expr {
        let mut expr = self.primary();

        while let Token::Operator(op) = self.peek() {
            let op = op.clone();
            self.advance();
            let right = self.primary();
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn primary(&mut self) -> Expr {
        match self.advance() {
            Token::Number(n) => Expr::Literal(Literal::Number(n)),
            Token::String(s) => Expr::Literal(Literal::String(s)),
            Token::Boolean(b) => Expr::Literal(Literal::Boolean(b)),
            Token::Identifier(name) => Expr::Variable(name),
            other => panic!("Unexpected expression: {:?}", other),
        }
    }
}
