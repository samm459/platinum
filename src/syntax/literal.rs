use crate::interpreter::r#type::Type;

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
}
