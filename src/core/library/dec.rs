use std::sync::Arc;

use crate::{
    core::module::Module,
    interpreter::{r#type::Type, value::Value, Interpreter},
};

pub fn define() -> Module {
    let r#type = Type::Closure(Box::new(Type::Number), Box::new(Type::Number));

    let value = Value::Closure(Arc::new(|value: Value, _: &mut Interpreter| {
        Value::Number(value.unwrap_number() - 1)
    }));

    Module::new("dec", r#type, value)
}
