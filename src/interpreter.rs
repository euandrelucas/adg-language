use std::collections::HashMap;
use crate::parser::{Stmt, Expr, Literal};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Return(Box<Value>),
}

impl Value {
    fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Esperado número, encontrou {:?}", self),
        }
    }

    fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => panic!("Esperado booleano, encontrou {:?}", self),
        }
    }

    fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Return(val) => val.as_string(),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, (Value, bool)>,
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    break_loop: bool,
    continue_loop: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            break_loop: false,
            continue_loop: false,
        }
    }

    pub fn execute(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            if let Some(Value::Return(_)) = self.exec_stmt(stmt) {
                break;
            }
            if self.break_loop {
                self.break_loop = false;
            }
        }
    }

    fn exec_stmt(&mut self, stmt: Stmt) -> Option<Value> {
        match stmt {
            Stmt::VarDecl { name, value, is_const } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, (val, is_const));
                None
            }

            Stmt::Assignment(name, expr) => {
                let val = self.eval_expr(*expr);
                if let Some((_, is_const)) = self.variables.get(&name) {
                    if *is_const {
                        panic!("Impossível reatribuir constante '{}'", name);
                    }
                }
                self.variables.insert(name, (val.clone(), false));
                None
            }

            Stmt::Expression(expr) => {
                self.eval_expr(expr);
                None
            }

            Stmt::If { condition, then_branch, else_branch } => {
                if self.eval_expr(condition).as_bool() {
                    for stmt in then_branch {
                        if let Some(ret) = self.exec_stmt(stmt) {
                            return Some(ret);
                        }
                        if self.break_loop || self.continue_loop {
                            break;
                        }
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        if let Some(ret) = self.exec_stmt(stmt) {
                            return Some(ret);
                        }
                        if self.break_loop || self.continue_loop {
                            break;
                        }
                    }
                }
                None
            }

            Stmt::Looping { condition, body } => {
                while self.eval_expr(condition.clone()).as_bool() {
                    for stmt in &body {
                        if let Some(ret) = self.exec_stmt(stmt.clone()) {
                            return Some(ret);
                        }
                        if self.break_loop {
                            self.break_loop = false;
                            return None;
                        }
                        if self.continue_loop {
                            self.continue_loop = false;
                            break;
                        }
                    }
                }
                None
            }

            Stmt::For { init, condition, update, body } => {
                self.exec_stmt(*init);
                while self.eval_expr(condition.clone()).as_bool() {
                    for stmt in &body {
                        if let Some(ret) = self.exec_stmt(stmt.clone()) {
                            return Some(ret);
                        }
                        if self.break_loop {
                            self.break_loop = false;
                            return None;
                        }
                        if self.continue_loop {
                            self.continue_loop = false;
                            break;
                        }
                    }
                    self.eval_expr(update.clone());
                }
                None
            }

            Stmt::Break => {
                self.break_loop = true;
                None
            }

            Stmt::Continue => {
                self.continue_loop = true;
                None
            }

            Stmt::Function { name, params, body } => {
                self.functions.insert(name, (params, body));
                None
            }

            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    let val = self.eval_expr(e);
                    Some(Value::Return(Box::new(val)))
                } else {
                    Some(Value::Return(Box::new(Value::Null)))
                }
            }

            Stmt::Block(stmts) => {
                for stmt in stmts {
                    if let Some(ret) = self.exec_stmt(stmt) {
                        return Some(ret);
                    }
                    if self.break_loop || self.continue_loop {
                        break;
                    }
                }
                None
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
                self.variables
                    .get(&name)
                    .map(|(v, _)| v.clone())
                    .unwrap_or_else(|| panic!("Variável '{}' não definida", name))
            }

            Expr::Assignment(name, value_expr) => {
                let value = self.eval_expr(*value_expr);
                if let Some((_, is_const)) = self.variables.get(&name) {
                    if *is_const {
                        panic!("Impossível reatribuir constante '{}'", name);
                    }
                }
                self.variables.insert(name.clone(), (value.clone(), false));
                value
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
                        _ => panic!("Operação '+' inválida"),
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
                    _ => panic!("Unknown binary operator '{}'", op),
                }
            }

            Expr::Call(name, args) => {
                let args = args.into_iter().map(|a| self.eval_expr(a)).collect::<Vec<_>>();

                if name == "print" {
                    for arg in args {
                        println!("{}", arg.as_string());
                    }
                    return Value::Null;
                }

                if let Some((params, body)) = self.functions.get(&name) {
                    if args.len() != params.len() {
                        panic!("Função '{}' esperava {} argumentos, recebeu {}", name, params.len(), args.len());
                    }

                    let mut new_scope = Interpreter::new();

                    for (i, param) in params.iter().enumerate() {
                        new_scope.variables.insert(param.clone(), (args[i].clone(), false));
                    }

                    for stmt in body {
                        if let Some(Value::Return(val)) = new_scope.exec_stmt(stmt.clone()) {
                            return *val;
                        }
                    }

                    Value::Null
                } else {
                    panic!("Função '{}' não definida", name);
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
