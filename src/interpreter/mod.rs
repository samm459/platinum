pub mod scope;
pub mod r#type;
pub mod value;

use self::r#type::*;
use self::scope::*;
use self::value::*;

use crate::error::*;
use crate::syntax::*;

use std::ops::Range;
use std::sync::Arc;

pub struct Interpreter {
    source: String,
    chain: Vec<Scope>,
    errors: Vec<Error>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            source: String::new(),
            chain: vec![Scope::global()],
            errors: vec![],
        }
    }

    fn type_map(&mut self, scope: usize) -> &mut Map<Type> {
        &mut self.chain[scope].type_map
    }

    fn map(&mut self, scope: usize) -> &mut Map<Value> {
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

    pub fn declare(&mut self, scope: usize, node: Node, r#type: Type) {
        let source = self.source(node);
        self.type_map(scope).insert(source, r#type);
    }

    pub fn lookup(&mut self, scope: usize, node: Node) -> Option<Type> {
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

    fn get(&mut self, scope: usize, node: Node) -> Option<Value> {
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

    pub fn bind(&mut self, syntax: Syntax, scope: usize) -> Type {
        match syntax {
            Syntax::Assignment(assignment) => assignment.bind(self, scope),
            Syntax::Call(call) => call.bind(self, scope),
            Syntax::Name(name) => name.bind(self, scope),
            Syntax::Literal(literal) => literal.bind(),
            Syntax::Closure(closure) => closure.bind(self, scope),
        }
    }

    pub fn eval(&mut self, syntax: Syntax, scope: usize) -> Value {
        match syntax {
            Syntax::Name(name) => self.get(scope, name.0).unwrap().clone(),
            Syntax::Literal(literal) => match literal.0 .0 {
                Token::String => Value::String(inner_string(self.source(literal.0))),
                Token::Number => Value::Number(self.source(literal.0).parse::<usize>().unwrap()),
                Token::Boolean => Value::Boolean(self.source(literal.0).parse::<bool>().unwrap()),
                Token::None => Value::None,
                _ => panic!(),
            },
            Syntax::Closure(closure) => {
                let name = closure.name.clone();
                let expression = closure.expression.clone();
                let closure = move |value: Value, interpreter: &mut Interpreter| {
                    let mut scope = Scope::new(scope);
                    scope.map.insert(interpreter.source(name), value);
                    interpreter.chain.push(scope);
                    interpreter.eval(*expression.clone(), interpreter.chain.len() - 1)
                };
                Value::Closure(Arc::new(closure))
            }
            Syntax::Call(call) => {
                let left = self.eval(*call.0, scope).unwrap_closure();
                let right = self.eval(*call.1, scope);
                left(right, self)
            }
            Syntax::Assignment(assignment) => {
                let name = String::from(self.source(assignment.name));
                let value = self.eval(*assignment.expression, scope);
                self.map(scope).insert(name, value);
                Value::None
            }
        }
    }
}
