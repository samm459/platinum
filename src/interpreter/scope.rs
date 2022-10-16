use super::*;
use crate::core;
use std::collections::HashMap;

pub type Map<T> = HashMap<String, T>;

pub struct Scope {
    pub parent: Option<ScopeIndex>,
    pub map: Map<Value>,
    pub type_map: Map<Type>,
    pub type_definition_map: Map<Type>,
}

impl Scope {
    pub fn global() -> Scope {
        let core = core::build();
        let primitives = core::primitives();

        let mut type_definition_map = HashMap::new();
        let mut type_map = HashMap::new();
        let mut map = HashMap::new();

        for (name, r#type) in primitives {
            type_definition_map.insert(name.clone(), r#type);
        }

        for module in core {
            type_map.insert(module.name.clone(), module.r#type);
            map.insert(module.name, module.value);
        }

        Scope {
            parent: None,
            type_definition_map,
            type_map,
            map,
        }
    }

    pub fn new(parent: ScopeIndex) -> Scope {
        Scope {
            parent: Some(parent),
            type_map: HashMap::new(),
            type_definition_map: HashMap::new(),
            map: HashMap::new(),
        }
    }
}

pub type ScopeIndex = usize;
