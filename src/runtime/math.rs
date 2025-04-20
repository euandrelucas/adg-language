use std::collections::HashMap;
use crate::interpreter::Value;

pub fn get_math_module() -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("sqrt".to_string(), Value::NativeFunction(|args| {
        if args.len() != 1 {
            panic!("math.sqrt espera 1 argumento");
        }
        Value::Number(args[0].as_number().sqrt())
    }));

    map.insert("pow".to_string(), Value::NativeFunction(|args| {
        if args.len() != 2 {
            panic!("math.pow espera 2 argumentos");
        }
        Value::Number(args[0].as_number().powf(args[1].as_number()))
    }));

    map.insert("random".to_string(), Value::NativeFunction(|_| {
        Value::Number(rand::random::<f64>())
    }));

    map
}
