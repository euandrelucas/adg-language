use std::collections::HashMap;
use std::fs;
use crate::interpreter::Value;

pub fn get_filebox_module() -> HashMap<String, Value> {
    let mut map = HashMap::new();

    map.insert("readFile".to_string(), Value::NativeFunction(|args| {
        if args.len() != 1 {
            panic!("fb.readFile espera 1 argumento");
        }
        let path = args[0].as_string();
        match fs::read_to_string(&path) {
            Ok(content) => Value::String(content),
            Err(e) => panic!("Erro ao ler arquivo {}: {}", path, e),
        }
    }));

    map.insert("writeFile".to_string(), Value::NativeFunction(|args| {
        if args.len() != 2 {
            panic!("fb.writeFile espera 2 argumentos");
        }
        let path = args[0].as_string();
        let content = args[1].as_string();
        match fs::write(&path, content) {
            Ok(_) => Value::Null,
            Err(e) => panic!("Erro ao escrever no arquivo {}: {}", path, e),
        }
    }));

    map
}