use std::collections::HashMap;
use crate::parser::{Expr, Literal, Stmt};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function(Vec<String>, Vec<Stmt>),
}

#[derive(Debug)]
pub struct Interpreter {
    pub variables: HashMap<String, (Value, bool)>,
    return_flag: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            return_flag: None,
        }
    }

    pub fn execute(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.exec_stmt(stmt);
            if self.return_flag.is_some() {
                break;
            }
        }
    }

    fn exec_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VarDecl { name, value, is_const } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, (val, is_const));
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value);
                if let Some((_, is_const)) = self.variables.get(&name) {
                    if *is_const {
                        panic!("Cannot reassign to constant '{}'.", name);
                    }
                }
                self.variables.insert(name, (val, false));
            }
            Stmt::Expression(expr) => {
                self.eval_expr(expr);
            }
            Stmt::If { condition, then_branch, else_branch } => {
                if self.eval_expr(condition).as_bool() {
                    for stmt in then_branch {
                        self.exec_stmt(stmt);
                        if self.return_flag.is_some() {
                            return;
                        }
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        self.exec_stmt(stmt);
                        if self.return_flag.is_some() {
                            return;
                        }
                    }
                }
            }
            Stmt::Looping { condition, body } => {
                while self.eval_expr(condition.clone()).as_bool() {
                    for stmt in &body {
                        self.exec_stmt(stmt.clone());
                        if self.return_flag.is_some() {
                            return;
                        }
                    }
                }
            }
            Stmt::Function { name, params, body } => {
                self.variables.insert(name, (Value::Function(params, body), true));
            }
            Stmt::Return(expr) => {
                let val = self.eval_expr(expr);
                self.return_flag = Some(val);
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.exec_stmt(stmt);
                    if self.return_flag.is_some() {
                        return;
                    }
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
                self.variables.get(&name).map(|(v, _)| v.clone()).unwrap_or_else(|| panic!("Variable '{}' not defined", name))
            }
            Expr::BinaryOp(lhs, op, rhs) => {
                let l = self.eval_expr(*lhs);
                let r = self.eval_expr(*rhs);
                match op.as_str() {
                    "+" => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::String(a), Value::String(b)) => Value::String(a + &b),
                        _ => panic!("Invalid '+' operation"),
                    },
                    "-" => Value::Number(l.as_number() - r.as_number()),
                    "*" => Value::Number(l.as_number() * r.as_number()),
                    "/" => Value::Number(l.as_number() / r.as_number()),
                    ">" => Value::Boolean(l.as_number() > r.as_number()),
                    ">=" => Value::Boolean(l.as_number() >= r.as_number()),
                    "<" => Value::Boolean(l.as_number() < r.as_number()),
                    "<=" => Value::Boolean(l.as_number() <= r.as_number()),
                    "==" => Value::Boolean(l == r),
                    "!=" => Value::Boolean(l != r),
                    _ => panic!("Unknown operator '{}'", op),
                }
            }
            Expr::Call(name, args) => {
                if name == "print" {
                    for arg in args {
                        println!("{}", self.eval_expr(arg).as_string());
                    }
                    return Value::Null;
                }

                let (params, body) = match self.variables.get(&name) {
                    Some((Value::Function(params, body), _)) => (params.clone(), body.clone()),
                    _ => panic!("Function '{}' not defined", name),
                };

                if args.len() != params.len() {
                    panic!("Function '{}' expects {} arguments, got {}", name, params.len(), args.len());
                }

                let mut new_scope = Interpreter::new();
                for (p, a) in params.into_iter().zip(args) {
                    let val = self.eval_expr(a);
                    new_scope.variables.insert(p, (val, false));
                }

                new_scope.execute(body);
                new_scope.return_flag.unwrap_or(Value::Null)
            }
        }
    }
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
            Value::Function(_, _) => "[function]".to_string(),
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
