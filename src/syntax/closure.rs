use std::sync::Arc;

use crate::interpreter::{
    r#type::Type,
    scope::{Scope, ScopeIndex},
    value::Value,
    Interpreter,
};

use super::{Branch, Leaf, Parser, Syntax, Token, TypeExpressionSyntax};

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureSyntax {
    pub name: Leaf,
    pub type_expression: TypeExpressionSyntax,
    pub lambda: Leaf,
    pub expression: Branch,
}

impl ClosureSyntax {
    fn create_closure(
        &self,
        interpreter: &mut Interpreter,
        scope: ScopeIndex,
    ) -> Box<dyn Fn(Value, &mut Interpreter) -> Value> {
        let expression = self.expression.clone();
        let name = self.name.clone();
        let source = interpreter.source(name);

        box move |value: Value, interpreter: &mut Interpreter| {
            let mut scope = Scope::new(scope);
            scope.map.insert(source.clone(), value);
            interpreter.chain.push(scope);
            interpreter.eval(*expression.clone(), interpreter.chain.len() - 1)
        }
    }
}

impl ClosureSyntax {
    pub fn parse(
        name: Leaf,
        type_expression: TypeExpressionSyntax,
        parser: &mut Parser,
    ) -> ClosureSyntax {
        ClosureSyntax {
            name,
            type_expression,
            lambda: parser.expect(Token::Lambda),
            expression: Box::new(Syntax::parse(parser)),
        }
    }

    pub fn bind(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Type {
        let param = interpreter.bind(Syntax::TypeExpression(self.type_expression.clone()), scope);
        let mut scope = Scope::new(scope);

        scope
            .type_map
            .insert(interpreter.source(self.name), param.clone());

        interpreter.chain.push(scope);
        let r#return = interpreter.bind(*self.expression.clone(), interpreter.chain.len() - 1);
        Type::Closure(box param, box r#return)
    }

    pub fn eval(&self, interpreter: &mut Interpreter, scope: ScopeIndex) -> Value {
        Value::Closure(Arc::new(self.create_closure(interpreter, scope)))
    }
}
