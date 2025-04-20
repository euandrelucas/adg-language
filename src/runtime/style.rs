use std::collections::HashMap;
use crate::interpreter::Value;

// Cada função abaixo é um ponteiro de função compatível com NativeFunction

fn red(args: Vec<Value>) -> Value {
    apply_ansi("31", args)
}

fn green(args: Vec<Value>) -> Value {
    apply_ansi("32", args)
}

fn yellow(args: Vec<Value>) -> Value {
    apply_ansi("33", args)
}

fn blue(args: Vec<Value>) -> Value {
    apply_ansi("34", args)
}

fn magenta(args: Vec<Value>) -> Value {
    apply_ansi("35", args)
}

fn cyan(args: Vec<Value>) -> Value {
    apply_ansi("36", args)
}

fn white(args: Vec<Value>) -> Value {
    apply_ansi("37", args)
}

fn bold(args: Vec<Value>) -> Value {
    apply_ansi("1", args)
}

fn underline(args: Vec<Value>) -> Value {
    apply_ansi("4", args)
}

fn bg_red(args: Vec<Value>) -> Value {
    apply_ansi("41", args)
}

fn bg_green(args: Vec<Value>) -> Value {
    apply_ansi("42", args)
}

fn bg_yellow(args: Vec<Value>) -> Value {
    apply_ansi("43", args)
}

fn bg_blue(args: Vec<Value>) -> Value {
    apply_ansi("44", args)
}

// Função auxiliar comum
fn apply_ansi(code: &str, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("style.{} espera 1 argumento", code);
    }
    let text = args[0].as_string();
    Value::String(format!("\x1b[{}m{}\x1b[0m", code, text))
}

// Registrar tudo
pub fn get_style_module() -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("red".to_string(), Value::NativeFunction(red));
    map.insert("green".to_string(), Value::NativeFunction(green));
    map.insert("yellow".to_string(), Value::NativeFunction(yellow));
    map.insert("blue".to_string(), Value::NativeFunction(blue));
    map.insert("magenta".to_string(), Value::NativeFunction(magenta));
    map.insert("cyan".to_string(), Value::NativeFunction(cyan));
    map.insert("white".to_string(), Value::NativeFunction(white));

    map.insert("bold".to_string(), Value::NativeFunction(bold));
    map.insert("underline".to_string(), Value::NativeFunction(underline));

    map.insert("bgRed".to_string(), Value::NativeFunction(bg_red));
    map.insert("bgGreen".to_string(), Value::NativeFunction(bg_green));
    map.insert("bgYellow".to_string(), Value::NativeFunction(bg_yellow));
    map.insert("bgBlue".to_string(), Value::NativeFunction(bg_blue));

    map
}
