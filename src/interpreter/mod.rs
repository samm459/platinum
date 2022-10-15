pub mod scope;
pub mod r#type;
pub mod value;

use self::r#type::*;
use self::scope::*;
use self::value::*;

use crate::error::*;
use crate::syntax::*;

use std::sync::Arc;

pub struct Interpreter(Vec<Scope>, Vec<Error>);

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter(vec![Scope::global()], vec![])
    }

    fn type_map(&mut self, scope: usize) -> &mut Map<Type> {
        &mut self.0[scope].type_map
    }

    fn map(&mut self, scope: usize) -> &mut Map<Value> {
        &mut self.0[scope].map
    }

    fn error(&mut self, error: Error) {
        self.1.push(error)
    }

    pub fn flush_errors(&mut self) -> Vec<Error> {
        let errors = self.1.clone();
        self.1 = vec![];
        errors
    }

    fn declare(&mut self, source: String, scope: usize, node: Node, r#type: Type) {
        self.type_map(scope).insert(
            String::from(&source[node.1.position..node.1.position + node.1.length]),
            r#type,
        );
    }

    fn lookup(&mut self, source: String, scope: usize, node: Node) -> Option<Type> {
        let parent = self.0.get(scope).unwrap().parent;
        let type_option: Option<Type> = match self
            .type_map(scope)
            .get(&source[node.1.position..node.1.position + node.1.length])
        {
            Some(r#type) => Some(r#type.clone()),
            None => None,
        };

        match type_option {
            Some(r#type) => Some(r#type),
            None => match parent {
                Some(parent) => self.lookup(source, parent, node),
                None => None,
            },
        }
    }

    fn get(&mut self, source: String, scope: usize, node: Node) -> Option<Value> {
        let parent = self.0.get(scope).unwrap().parent;
        let value_option = match self
            .map(scope)
            .get(&source[node.1.position..node.1.position + node.1.length])
        {
            Some(value) => Some(value.clone()),
            None => None,
        };

        match value_option {
            Some(value) => Some(value),
            None => match parent {
                Some(parent) => self.get(source, parent, node),
                None => None,
            },
        }
    }

    pub fn link(&mut self, source: String, syntax: Syntax, scope: usize) -> Type {
        match syntax {
            Syntax::Assignment(assignment) => {
                let expression_type = self.link(source.clone(), *assignment.expression, scope);
                if let Some(_) = self.lookup(source.clone(), scope, assignment.name) {
                    self.error(Error::Reassignment(
                        assignment.name.1.position
                            ..assignment.name.1.position + assignment.name.1.length,
                        String::from(
                            &source[assignment.name.1.position
                                ..assignment.name.1.position + assignment.name.1.length],
                        ),
                    ))
                } else {
                    self.declare(source.clone(), scope, assignment.name, expression_type);
                }
                Type::None
            }
            Syntax::Call(call) => {
                let left = self.link(source.clone(), *call.0, scope);
                let right = self.link(source.clone(), *call.1, scope);
                if let Type::Closure(param, r#return) = left {
                    if right != *param {
                        self.error(Error::UnexpectedType(0..0, *param, right));
                    }
                    *r#return
                } else {
                    panic!()
                }
            }
            Syntax::Name(name) => match self.lookup(source.clone(), scope, name.0) {
                Some(value) => value.clone(),
                None => {
                    self.error(Error::UnknownName(
                        name.0 .1.position..name.0 .1.position + name.0 .1.length,
                        String::from(
                            &source[name.0 .1.position..name.0 .1.position + name.0 .1.length],
                        ),
                    ));
                    Type::None
                }
            },
            Syntax::Literal(literal) => match literal.0 .0 {
                Token::String => Type::String,
                Token::Number => Type::Number,
                Token::Boolean => Type::Boolean,
                Token::None => Type::None,
                _ => panic!(),
            },
            Syntax::Closure(closure) => {
                let param = self
                    .lookup(source.clone(), scope, closure.r#type)
                    .unwrap()
                    .clone();
                let r#return = self.link(source.clone(), *closure.expression, scope);
                Type::Closure(Box::new(param), Box::new(r#return))
            }
        }
    }

    pub fn eval(&mut self, source: String, syntax: Syntax, scope: usize) -> Value {
        match syntax {
            Syntax::Name(name) => self.get(source, scope, name.0).unwrap().clone(),
            Syntax::Literal(literal) => match literal.0 .0 {
                Token::String => Value::String(String::from(
                    &source[literal.0 .1.position..literal.0 .1.position + literal.0 .1.length],
                )),
                Token::Number => Value::Number(
                    String::from(
                        &source[literal.0 .1.position..literal.0 .1.position + literal.0 .1.length],
                    )
                    .parse::<usize>()
                    .unwrap(),
                ),
                Token::Boolean => Value::Boolean(
                    String::from(
                        &source[literal.0 .1.position..literal.0 .1.position + literal.0 .1.length],
                    )
                    .parse::<bool>()
                    .unwrap(),
                ),
                Token::None => Value::None,
                _ => panic!(),
            },
            Syntax::Closure(closure) => {
                let name_position = closure.name.1.position.clone();
                let name_length = closure.name.1.length.clone();
                let scope = scope.clone();
                let expression = closure.expression.clone();
                let source = source.clone();

                let closure = move |value: Value, interpreter: &mut Interpreter| {
                    let mut scope = Scope::new(scope);
                    scope.map.insert(
                        String::from(&source[name_position..name_position + name_length]),
                        value,
                    );
                    interpreter.0.push(scope);
                    interpreter.eval(
                        String::from(&source),
                        *expression.clone(),
                        interpreter.0.len() - 1,
                    )
                };

                Value::Closure(Arc::new(closure))
            }
            Syntax::Call(call) => {
                let left = self.eval(source.clone(), *call.0, scope).unwrap_closure();
                let right = self.eval(source.clone(), *call.1, scope);
                left(right, self)
            }
            Syntax::Assignment(assignment) => {
                let name = String::from(
                    &source[assignment.name.1.position
                        ..assignment.name.1.position + assignment.name.1.length],
                );
                let value = self.eval(source, *assignment.expression, scope);
                self.map(scope).insert(name, value);
                Value::None
            }
        }
    }
}
