use crate::{
    error::Error,
    interpreter::{r#type::Type, value::Value, Interpreter},
};

use super::{Branch, Parser, Syntax, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct CallSyntax(pub Branch, pub Branch);

impl CallSyntax {
    pub fn parse(parser: &mut Parser) -> Syntax {
        let mut left = parser.primary();

        while parser.current() != Token::EndOfFile && parser.current() != Token::CloseParenthesis {
            let right = parser.primary();
            left = Syntax::Call(CallSyntax(Box::new(left), Box::new(right)))
        }

        left
    }

    pub fn bind(&self, interpreter: &mut Interpreter, scope: usize) -> Type {
        let left = interpreter.bind(*self.0.clone(), scope);
        let right = interpreter.bind(*self.1.clone(), scope);

        if let Type::Closure(param, r#return) = left {
            if right != *param {
                interpreter.error(Error::UnexpectedType(0..0, *param, right));
            }
            *r#return
        } else {
            interpreter.error(Error::BadCall(0..0));
            Type::None
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter, scope: usize) -> Value {
        let left = interpreter.eval(*self.0.clone(), scope).unwrap_closure();
        let right = interpreter.eval(*self.1.clone(), scope);
        left(right, interpreter)
    }
}
