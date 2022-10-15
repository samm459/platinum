use std::fmt::*;
use std::sync::Arc;

use super::*;
use crate::repl::escape_code::*;

pub enum Value {
    Number(usize),
    String(String),
    Boolean(bool),
    Closure(Arc<dyn Fn(Value, &mut Interpreter) -> Value>),
    None,
}

impl Value {
    pub fn unwrap_number(self) -> usize {
        match self {
            Value::Number(usize) => usize,
            _ => panic!(),
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            Value::String(string) => string,
            _ => panic!(),
        }
    }

    pub fn unwrap_bool(self) -> bool {
        match self {
            Value::Boolean(bool) => bool,
            _ => panic!(),
        }
    }

    pub fn unwrap_closure(self) -> Arc<dyn Fn(Value, &mut Interpreter) -> Value> {
        match self {
            Value::Closure(func) => func,
            _ => panic!(),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Number(usize) => Value::Number(usize.clone()),
            Value::String(string) => Value::String(string.clone()),
            Value::Boolean(bool) => Value::Boolean(bool.clone()),
            Value::Closure(func) => Value::Closure(func.clone()),
            Value::None => Value::None,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Number(usize) => write!(f, "{}{}{}", YELLOW, usize, RESET),
            Value::String(string) => write!(f, "{}{}{}", GREEN, string, RESET),
            Value::Boolean(bool) => write!(f, "{}{}{}", CYAN, bool, RESET),
            Value::Closure(_) => write!(f, "{}[Closure]{}", MAGENTA, RESET),
            Value::None => write!(f, "{}[None]{}", MAGENTA, RESET),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let other = other.clone();
        match self {
            Value::Number(usize) => usize == &other.unwrap_number(),
            Value::String(string) => string == &other.unwrap_string(),
            Value::Boolean(bool) => bool == &other.unwrap_bool(),
            Value::Closure(_) => false,
            Value::None => match other {
                Value::None => true,
                _ => false,
            },
        }
    }
}
