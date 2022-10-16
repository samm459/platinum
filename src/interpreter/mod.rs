pub mod scope;
pub mod r#type;
pub mod value;

use self::r#type::*;
use self::scope::*;
use self::value::*;

use crate::error::*;
use crate::syntax::*;

use std::ops::Range;

pub struct Interpreter {
    source: String,
    pub chain: Vec<Scope>,
    pub errors: Vec<Error>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            source: String::new(),
            chain: vec![Scope::global()],
            errors: vec![],
        }
    }

    fn type_definition_map(&mut self, scope: ScopeIndex) -> &mut Map<Type> {
        &mut self.chain[scope].type_definition_map
    }

    fn type_map(&mut self, scope: ScopeIndex) -> &mut Map<Type> {
        &mut self.chain[scope].type_map
    }

    pub fn map(&mut self, scope: ScopeIndex) -> &mut Map<Value> {
        &mut self.chain[scope].map
    }

    pub fn error(&mut self, error: Error) {
        self.errors.push(error)
    }

    pub fn source(&self, node: Node) -> String {
        String::from(&self.source[node.1.position..node.1.position + node.1.length])
    }

    pub fn range(&self, node: Node) -> Range<usize> {
        node.1.position..node.1.position + node.1.length
    }

    pub fn set_source(&mut self, source: &str) {
        self.source = String::from(source)
    }

    pub fn flush_errors(&mut self) -> Vec<Error> {
        let errors = self.errors.clone();
        self.errors = vec![];
        errors
    }

    pub fn declare(&mut self, scope: ScopeIndex, node: Node, r#type: Type) {
        let source = self.source(node);
        self.type_map(scope).insert(source, r#type);
    }

    pub fn lookup(&mut self, scope: ScopeIndex, node: Node) -> Option<Type> {
        let parent = self.chain.get(scope).unwrap().parent;
        let source = self.source(node);
        let type_option: Option<Type> = match self.type_map(scope).get(&source) {
            Some(r#type) => Some(r#type.clone()),
            None => None,
        };

        match type_option {
            Some(r#type) => Some(r#type),
            None => match parent {
                Some(parent) => self.lookup(parent, node),
                None => None,
            },
        }
    }

    pub fn lookup_type_definition(&mut self, scope: ScopeIndex, node: Node) -> Option<Type> {
        let parent = self.chain.get(scope).unwrap().parent;
        let source = self.source(node);
        let type_option: Option<Type> = match self.type_definition_map(scope).get(&source) {
            Some(r#type) => Some(r#type.clone()),
            None => None,
        };

        match type_option {
            Some(r#type) => Some(r#type),
            None => match parent {
                Some(parent) => self.lookup_type_definition(parent, node),
                None => None,
            },
        }
    }

    pub fn get(&mut self, scope: ScopeIndex, node: Node) -> Option<Value> {
        let parent = self.chain.get(scope).unwrap().parent;
        let source = self.source(node);
        let value_option = match self.map(scope).get(&source) {
            Some(value) => Some(value.clone()),
            None => None,
        };

        match value_option {
            Some(value) => Some(value),
            None => match parent {
                Some(parent) => self.get(parent, node),
                None => None,
            },
        }
    }

    pub fn bind(&mut self, syntax: Syntax, scope: ScopeIndex) -> Type {
        match syntax {
            Syntax::Assignment(assignment) => assignment.bind(self, scope),
            Syntax::Call(call) => call.bind(self, scope),
            Syntax::Name(name) => name.bind(self, scope),
            Syntax::Literal(literal) => literal.bind(),
            Syntax::Closure(closure) => closure.bind(self, scope),
            Syntax::TypeExpression(type_expression) => type_expression.bind(self, scope),
        }
    }

    pub fn eval(&mut self, syntax: Syntax, scope: ScopeIndex) -> Value {
        match syntax {
            Syntax::Name(name) => name.eval(self, scope),
            Syntax::Literal(literal) => literal.eval(self),
            Syntax::Closure(closure) => closure.eval(self, scope),
            Syntax::Call(call) => call.eval(self, scope),
            Syntax::Assignment(assignment) => assignment.eval(self, scope),
            Syntax::TypeExpression(_) => Value::None,
        }
    }
}
