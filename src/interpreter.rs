use std::collections::HashMap;
use crate::parser::{Expr, Literal, Stmt};
use crate::runtime::math::get_math_module;
use crate::runtime::style::get_style_module; // 👈 Adicionado
use crate::runtime::filebox::get_filebox_module;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Env,
    },
    NativeFunction(fn(Vec<Value>) -> Value),
    Array(Vec<Value>),
}

impl Value {
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Expected number, got {:?}", self),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => panic!("Expected boolean, got {:?}", self),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.as_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Function { .. } => "[Function]".to_string(),
            Value::NativeFunction(_) => "[NativeFunction]".to_string(),
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

type Env = HashMap<String, Value>;

pub struct Interpreter {
    globals: Env,
    locals: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = HashMap::new();

        // Funções nativas
        globals.insert("print".to_string(), Value::NativeFunction(|args| {
            for arg in args {
                println!("{}", arg.as_string());
            }
            Value::Null
        }));

        // Módulo filebox (fb)
        let filebox = get_filebox_module();
        for (k, v) in filebox {
            globals.insert(format!("fb.{}", k), v);
        }

        // Módulo math
        let math = get_math_module();
        for (k, v) in math {
            globals.insert(format!("math.{}", k), v);
        }

        // Módulo style
        let style = get_style_module();
        for (k, v) in style {
            globals.insert(format!("style.{}", k), v);
        }

        Interpreter {
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn execute(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.exec_stmt(stmt);
        }
    }

    fn exec_stmt(&mut self, stmt: Stmt) -> Option<Value> {
        match stmt {
            Stmt::VarDecl { name, value, .. } => {
                let val = self.eval_expr(value);
                self.locals.insert(name, val);
                None
            }
            Stmt::Assignment(name, expr) => {
                let val = self.eval_expr(*expr);
                self.locals.insert(name, val);
                None
            }
            Stmt::Expression(expr) => {
                self.eval_expr(expr);
                None
            }
            Stmt::If { condition, then_branch, else_branch } => {
                if self.eval_expr(condition).as_bool() {
                    for stmt in then_branch {
                        self.exec_stmt(stmt);
                    }
                } else if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.exec_stmt(stmt);
                    }
                }
                None
            }
            Stmt::Looping { condition, body } => {
                while self.eval_expr(condition.clone()).as_bool() {
                    for stmt in &body {
                        if let Some(Value::String(s)) = self.exec_stmt(stmt.clone()) {
                            if s == "break" {
                                return None;
                            } else if s == "continue" {
                                break;
                            }
                        }
                    }
                }
                None
            }
            Stmt::For { init, condition, update, body } => {
                self.exec_stmt(*init);
                while self.eval_expr(condition.clone()).as_bool() {
                    for stmt in &body {
                        if let Some(Value::String(s)) = self.exec_stmt(stmt.clone()) {
                            if s == "break" {
                                return None;
                            } else if s == "continue" {
                                break;
                            }
                        }
                    }
                    self.eval_expr(update.clone());
                }
                None
            }
            Stmt::Break => Some(Value::String("break".into())),
            Stmt::Continue => Some(Value::String("continue".into())),
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    params,
                    body,
                    env: self.locals.clone(),
                };
                self.locals.insert(name, func);
                None
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    Some(self.eval_expr(e))
                } else {
                    Some(Value::Null)
                }
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.exec_stmt(stmt);
                }
                None
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Index(array_expr, index_expr) => {
                let array = self.eval_expr(*array_expr);
                let index = self.eval_expr(*index_expr).as_number() as usize;
                match array {
                    Value::Array(items) => items.get(index).cloned().unwrap_or(Value::Null),
                    _ => panic!("Expected array for indexing, got {:?}", array),
                }
            }
            Expr::Literal(lit) => match lit {
                Literal::Number(n) => Value::Number(n),
                Literal::String(s) => Value::String(s),
                Literal::Boolean(b) => Value::Boolean(b),
                Literal::Null => Value::Null,
                Literal::Array(items) => {
                    let values = items.into_iter().map(|e| self.eval_expr(e)).collect();
                    Value::Array(values)
                }
            },
            Expr::Variable(name) => {
                self.locals.get(&name)
                    .or_else(|| self.globals.get(&name))
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined variable '{}'", name))
            }
            Expr::Assignment(name, expr) => {
                let value = self.eval_expr(*expr);
                self.locals.insert(name.clone(), value.clone());
                value
            }
            Expr::BinaryOp(left, op, right) => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);
                match op.as_str() {
                    "+" => match (left, right) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (a, b) => Value::String(a.as_string() + &b.as_string()),
                    },
                    "-" => Value::Number(left.as_number() - right.as_number()),
                    "*" => Value::Number(left.as_number() * right.as_number()),
                    "/" => Value::Number(left.as_number() / right.as_number()),
                    ">" => Value::Boolean(left.as_number() > right.as_number()),
                    "<" => Value::Boolean(left.as_number() < right.as_number()),
                    ">=" => Value::Boolean(left.as_number() >= right.as_number()),
                    "<=" => Value::Boolean(left.as_number() <= right.as_number()),
                    "==" => Value::Boolean(left == right),
                    "!=" => Value::Boolean(left != right),
                    _ => panic!("Unknown binary operator '{}'", op),
                }
            }
            Expr::Call(name, args) => {
                let args = args.into_iter().map(|e| self.eval_expr(e)).collect::<Vec<_>>();
                if let Some(val) = self.locals.get(&name).or_else(|| self.globals.get(&name)) {
                    match val {
                        Value::Function { params, body, env } => {
                            let mut local = env.clone();
                            for (param, arg) in params.iter().zip(args) {
                                local.insert(param.clone(), arg);
                            }
                            let mut sub = Interpreter {
                                globals: self.globals.clone(),
                                locals: local,
                            };
                            for stmt in body.clone() {
                                if let Some(v) = sub.exec_stmt(stmt) {
                                    return v;
                                }
                            }
                            Value::Null
                        }
                        Value::NativeFunction(f) => f(args),
                        _ => panic!("'{}' is not a function", name),
                    }
                } else {
                    panic!("Function '{}' not found", name);
                }
            }
        }
    }
}
