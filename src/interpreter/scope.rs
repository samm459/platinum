use std::collections::HashMap;
use std::sync::Arc;

use super::*;

pub type Map<T> = HashMap<String, T>;

pub struct Scope {
    pub parent: Option<usize>,
    pub map: Map<Value>,
    pub type_map: Map<Type>,
}

impl Scope {
    pub fn global() -> Scope {
        let mut type_map = HashMap::new();
        let mut map = HashMap::new();

        type_map.insert(
            "inc".into(),
            Type::Closure(Box::new(Type::Number), Box::new(Type::Number)),
        );

        type_map.insert(
            "dec".into(),
            Type::Closure(Box::new(Type::Number), Box::new(Type::Number)),
        );

        map.insert(
            "inc".into(),
            Value::Closure(Arc::new(|value: Value, _: &mut Interpreter| {
                Value::Number(value.unwrap_number() + 1)
            })),
        );

        map.insert(
            "dec".into(),
            Value::Closure(Arc::new(|value: Value, _: &mut Interpreter| {
                Value::Number(value.unwrap_number() - 1)
            })),
        );

        Scope {
            parent: None,
            type_map,
            map,
        }
    }

    pub fn new(parent: usize) -> Scope {
        Scope {
            parent: Some(parent),
            type_map: HashMap::new(),
            map: HashMap::new(),
        }
    }
}
