use crate::interpreter::{
    self,
    r#type::Type,
    value::{inner_string, Value},
    Interpreter,
};

use super::{Leaf, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralSyntax(pub Leaf);

impl LiteralSyntax {
    pub fn bind(&self) -> Type {
        match self.0 .0 {
            Token::String => Type::String,
            Token::Number => Type::Number,
            Token::Boolean => Type::Boolean,
            Token::None => Type::None,
            _ => panic!(),
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter) -> Value {
        match self.0 .0 {
            Token::String => Value::String(inner_string(interpreter.source(self.0))),
            Token::Number => Value::Number(interpreter.source(self.0).parse::<usize>().unwrap()),
            Token::Boolean => Value::Boolean(interpreter.source(self.0).parse::<bool>().unwrap()),
            Token::None => Value::None,
            _ => panic!(),
        }
    }
}
