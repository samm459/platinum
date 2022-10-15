use std::sync::Arc;

use crate::{
    core::module::Module,
    interpreter::{r#type::Type, value::Value, Interpreter},
};

pub fn define() -> Module {
    let r#type = Type::Closure(
        Box::new(Type::Number),
        Box::new(Type::Closure(
            Box::new(Type::Number),
            Box::new(Type::Number),
        )),
    );

    let value = Value::Closure(Arc::new(|value1: Value, _: &mut Interpreter| {
        Value::Closure(Arc::new(move |value2: Value, _: &mut Interpreter| {
            let value1 = value1.clone();
            Value::Number(value1.unwrap_number() / value2.unwrap_number())
        }))
    }));

    Module::new("div", r#type, value)
}
