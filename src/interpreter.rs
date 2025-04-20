use std::collections::HashMap;
use crate::parser::{Expr, Literal, Stmt};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {
    fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Expected number, got {:?}", self),
        }
    }

    fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => panic!("Expected boolean, got {:?}", self),
        }
    }

    fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, (Value, bool)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.exec_stmt(stmt);
        }
    }

    fn exec_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VarDecl { name, value, is_const } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, (val, is_const));
            }

            Stmt::Assign { name, value } => {
                if let Some((_, true)) = self.variables.get(&name) {
                    panic!("Cannot reassign to constant '{}'", name);
                }
                let val = self.eval_expr(value);
                self.variables.insert(name, (val, false));
            }

            Stmt::Expression(expr) => {
                self.eval_expr(expr);
            }

            Stmt::Looping { condition, body } => {
                while self.eval_expr(condition.clone()).as_bool() {
                    for stmt in &body {
                        self.exec_stmt(stmt.clone());
                    }
                }
            }

            Stmt::If { condition, then_branch, else_branch } => {
                if self.eval_expr(condition).as_bool() {
                    for stmt in then_branch {
                        self.exec_stmt(stmt);
                    }
                } else if let Some(else_block) = else_branch {
                    for stmt in else_block {
                        self.exec_stmt(stmt);
                    }
                }
            }

            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.exec_stmt(stmt);
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Number(n) => Value::Number(n),
                Literal::String(s) => Value::String(s),
                Literal::Boolean(b) => Value::Boolean(b),
                Literal::Null => Value::Null,
            },

            Expr::Variable(name) => {
                self.variables.get(&name)
                    .map(|(v, _)| v.clone())
                    .unwrap_or_else(|| panic!("Undefined variable '{}'", name))
            }

            Expr::BinaryOp(lhs, op, rhs) => {
                let l = self.eval_expr(*lhs);
                let r = self.eval_expr(*rhs);

                match op.as_str() {
                    "+" => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::String(a), Value::String(b)) => Value::String(a + &b),
                        (Value::String(a), b) => Value::String(a + &b.as_string()),
                        (a, Value::String(b)) => Value::String(a.as_string() + &b),
                        _ => panic!("Invalid '+' operation"),
                    },
                    "-" => Value::Number(l.as_number() - r.as_number()),
                    "*" => Value::Number(l.as_number() * r.as_number()),
                    "/" => Value::Number(l.as_number() / r.as_number()),
                    ">" => Value::Boolean(l.as_number() > r.as_number()),
                    "<" => Value::Boolean(l.as_number() < r.as_number()),
                    ">=" => Value::Boolean(l.as_number() >= r.as_number()),
                    "<=" => Value::Boolean(l.as_number() <= r.as_number()),
                    "==" => Value::Boolean(l == r),
                    "!=" => Value::Boolean(l != r),
                    _ => panic!("Unknown operator '{}'", op),
                }
            }

            Expr::Call(name, args) => {
                let args = args.into_iter().map(|e| self.eval_expr(e)).collect::<Vec<_>>();
                match name.as_str() {
                    "print" => {
                        for a in args {
                            println!("{}", a.as_string());
                        }
                        Value::Null
                    }
                    _ => panic!("Unknown function '{}'", name),
                }
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
