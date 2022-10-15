use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Range;

use crate::interpreter::r#type::*;
use crate::repl::escape_code::*;
use crate::syntax::*;

#[derive(Clone)]
pub enum Error {
    UnknownToken(Range<usize>, String),
    UnknownName(Range<usize>, String),
    UnexpectedType(Range<usize>, Type, Type),
    Reassignment(Range<usize>, String),
    UnexpectedToken(Range<usize>, Token, Token),
}

#[derive(Debug)]
pub enum Category {
    Syntax,
    Type,
}

impl Error {
    fn log(
        f: &mut Formatter<'_>,
        category: Category,
        range: Range<usize>,
        message: String,
    ) -> fmt::Result {
        write!(
            f,
            "{}{:?} Error: {}{}{}\n    at range {}..{}{}",
            RED, category, message, RESET, DIM, range.start, range.end, RESET
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownName(range, name) => Error::log(
                f,
                Category::Type,
                range.clone(),
                format!("Unknown name \"{}\"", name),
            ),
            Self::UnexpectedType(range, expected, recieved) => Error::log(
                f,
                Category::Type,
                range.clone(),
                format!("Unexpected type {:?}, expected a {:?}", recieved, expected),
            ),
            Self::Reassignment(range, name) => Error::log(
                f,
                Category::Type,
                range.clone(),
                format!("Cannot reassign name \"{}\"", name),
            ),
            Self::UnknownToken(range, name) => Error::log(
                f,
                Category::Syntax,
                range.clone(),
                format!("Unknown token {}", name),
            ),
            Self::UnexpectedToken(range, expected, recieved) => Error::log(
                f,
                Category::Syntax,
                range.clone(),
                format!("Unexpected token {:?}, expected {:?}", recieved, expected),
            ),
        }
    }
}
