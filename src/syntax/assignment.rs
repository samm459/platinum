use super::{Branch, Leaf, Parser, Syntax, Token, TypeExpressionSyntax};
use crate::{
    error::Error,
    interpreter::{r#type::Type, scope::ScopeIndex, value::Value, *},
};

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub name: Leaf,
    pub type_expression: Option<TypeExpressionSyntax>,
    pub equals: Leaf,
    pub expression: Branch,
}

impl AssignmentSyntax {
    pub fn parse_with_type(
        name: Leaf,
        type_expression: TypeExpressionSyntax,
        parser: &mut Parser,
    ) -> AssignmentSyntax {
        AssignmentSyntax {
            name,
            type_expression: Some(type_expression),
            equals: parser.expect(Token::Equals),
            expression: Box::new(Syntax::parse(parser)),
        }
    }
}

impl AssignmentSyntax {
    pub fn parse(parser: &mut Parser) -> AssignmentSyntax {
        AssignmentSyntax {
            name: parser.expect(Token::Identifier),
            type_expression: None,
            equals: parser.assert(Token::Equals),
            expression: Box::new(Syntax::parse(parser)),
        }
    }

    pub fn bind(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Type {
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

    pub fn eval(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Value {
        let name = String::from(interpreter.source(self.name));
        let value = interpreter.eval(*self.expression.clone(), scope);
        interpreter.map(scope).insert(name, value);
        Value::None
    }
}
