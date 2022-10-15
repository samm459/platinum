use super::*;
use crate::core;
use std::collections::HashMap;

pub type Map<T> = HashMap<String, T>;

pub struct Scope {
    pub parent: Option<usize>,
    pub map: Map<Value>,
    pub type_map: Map<Type>,
}

impl Scope {
    pub fn global() -> Scope {
        let core = core::build();

        let mut type_map = HashMap::new();
        let mut map = HashMap::new();

        for module in core {
            type_map.insert(module.name.clone(), module.r#type);
            map.insert(module.name, module.value);
        }

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
