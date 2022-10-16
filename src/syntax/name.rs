use crate::{
    error::Error,
    interpreter::{r#type::Type, value::Value, Interpreter},
};

use super::Leaf;

#[derive(Debug, PartialEq, Clone)]
pub struct NameSyntax(pub Leaf);

impl NameSyntax {
    pub fn bind(&self, interpreter: &mut Interpreter, scope: usize) -> Type {
        match interpreter.lookup(scope, self.0) {
            Some(value) => value.clone(),
            None => {
                interpreter.error(Error::UnknownName(
                    interpreter.range(self.0),
                    interpreter.source(self.0),
                ));
                Type::None
            }
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter, scope: usize) -> Value {
        interpreter.get(scope, self.0).unwrap().clone()
    }
}
