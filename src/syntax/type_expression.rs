use crate::interpreter::{r#type::Type, scope::ScopeIndex, Interpreter};

use super::{Node, Parser, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeExpressionSyntax {
    colon: Node,
    name: Node,
}

impl TypeExpressionSyntax {
    pub fn parse(parser: &mut Parser) -> TypeExpressionSyntax {
        TypeExpressionSyntax {
            colon: parser.assert(Token::Colon),
            name: parser.expect(Token::Identifier),
        }
    }

    pub fn bind(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Type {
        match interpreter.lookup_type_definition(scope, self.name) {
            Some(definition) => definition,
            None => Type::None,
        }
    }
}
