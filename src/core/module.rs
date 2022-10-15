use crate::interpreter::r#type::Type;
use crate::interpreter::value::Value;

pub struct Module {
    pub name: String,
    pub r#type: Type,
    pub value: Value,
}

impl Module {
    pub fn new(name: &str, r#type: Type, r#value: Value) -> Module {
        Module {
            name: String::from(name),
            r#type,
            value,
        }
    }
}
