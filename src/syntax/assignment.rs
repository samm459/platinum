use super::{Branch, Leaf, Parser, Syntax, Token};
use crate::{
    error::Error,
    interpreter::{r#type::Type, *},
};

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub name: Leaf,
    pub equals: Leaf,
    pub expression: Branch,
}

impl AssignmentSyntax {
    pub fn parse(parser: &mut Parser) -> AssignmentSyntax {
        AssignmentSyntax {
            name: parser.expect(Token::Identifier),
            equals: parser.assert(Token::Equals),
            expression: Box::new(Syntax::parse(parser)),
        }
    }

    pub fn bind(&self, interpreter: &mut Interpreter, scope: usize) -> Type {
        let expression_type = interpreter.bind(*self.expression.clone(), scope);

        if let Some(_) = interpreter.lookup(scope, self.name) {
            interpreter.error(Error::Reassignment(
                interpreter.range(self.name),
                interpreter.source(self.name),
            ))
        } else {
            interpreter.declare(scope, self.name, expression_type);
        }

        Type::None
    }
}
