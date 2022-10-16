use crate::interpreter::{
    r#type::Type,
    value::{inner_string, Value},
    Interpreter,
};

use super::{Description, Leaf, Node, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralSyntax(pub Leaf);

impl LiteralSyntax {
    pub fn token(&self) -> &Token {
        &self.node().0
    }

    pub fn description(&self) -> &Description {
        &self.node().1
    }

    pub fn node(&self) -> &Node {
        &self.0
    }
}

impl LiteralSyntax {
    pub fn bind(&self) -> Type {
        match self.token() {
            Token::String => Type::String,
            Token::Number => Type::Number,
            Token::Boolean => Type::Boolean,
            Token::None => Type::None,
            _ => panic!(),
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter) -> Value {
        match self.token() {
            Token::String => Value::String(inner_string(interpreter.source(self.0))),
            Token::Number => Value::Number(interpreter.source(self.0).parse::<usize>().unwrap()),
            Token::Boolean => Value::Boolean(interpreter.source(self.0).parse::<bool>().unwrap()),
            Token::None => Value::None,
            _ => panic!(),
        }
    }
}
