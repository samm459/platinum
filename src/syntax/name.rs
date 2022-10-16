use crate::{
    error::Error,
    interpreter::{r#type::Type, scope::ScopeIndex, value::Value, Interpreter},
};

use super::Leaf;

#[derive(Debug, PartialEq, Clone)]
pub struct NameSyntax(pub Leaf);

impl NameSyntax {
    fn unknown_name_error(&self, interpreter: &mut Interpreter) -> Type {
        interpreter.error(Error::UnknownName(
            interpreter.range(self.0),
            interpreter.source(self.0),
        ));
        Type::None
    }
}

impl NameSyntax {
    pub fn bind(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Type {
        match interpreter.lookup(scope, self.0) {
            Some(value) => value.clone(),
            None => self.unknown_name_error(interpreter),
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Value {
        interpreter.get(scope, self.0).unwrap().clone()
    }
}
