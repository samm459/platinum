mod character;
mod keyword;
mod lexer;

use self::character::*;
use self::keyword::*;

pub use self::lexer::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Identifier,
    Space,
    Number,
    String,
    Boolean,
    Lambda,
    Equals,
    Unknown,
    OpenParenthesis,
    CloseParenthesis,
    Colon,
    EndOfFile,
    None,
}

pub fn register_tokens(lexer: &mut Lexer) {
    lexer.register(Token::identifier);
    lexer.register(Token::space);
    lexer.register(Token::number);
    lexer.register(Token::string);
    lexer.register(Token::boolean);
    lexer.register(Token::lambda);
    lexer.register(Token::open_parenthesis);
    lexer.register(Token::close_parenthesis);
    lexer.register(Token::colon);
    lexer.register(Token::equals);
    lexer.register(Token::none);
}

pub fn tokenize(source: &str) -> Lexer {
    let mut lexer = Lexer::new(source);

    while lexer.current() != character::TERMINATOR {
        lexer.next_token()
    }

    lexer
}

impl Token {
    pub fn identifier(lexer: &mut Lexer) {
        if lexer.current().is_alphabetic() {
            while lexer.current().is_alphanumeric() {
                lexer.step()
            }
        }

        if !KEYWORDS.contains(&lexer.span().as_str()) {
            lexer.add(Token::Identifier);
        }
    }

    pub fn space(lexer: &mut Lexer) {
        while lexer.current().is_whitespace() {
            lexer.step()
        }
        lexer.add(Token::Space);
    }

    pub fn number(lexer: &mut Lexer) {
        while lexer.current().is_numeric() {
            lexer.step()
        }
        lexer.add(Token::Number);
    }

    pub fn string(lexer: &mut Lexer) {
        if lexer.current() == DOUBLE_QUOTE {
            lexer.step();
            while lexer.current() != DOUBLE_QUOTE
                && lexer.current() != TERMINATOR
                && lexer.current() != NEWLINE
            {
                lexer.step()
            }
        }
        if lexer.current() == DOUBLE_QUOTE {
            lexer.step()
        }
        lexer.add(Token::String);
    }

    pub fn boolean(lexer: &mut Lexer) {
        while lexer.current().is_alphabetic() {
            lexer.step()
        }

        if &lexer.span() == &keyword::TRUE || &lexer.span() == &keyword::FALSE {
            lexer.add(Token::Boolean)
        }
    }

    pub fn none(lexer: &mut Lexer) {
        while lexer.current().is_alphabetic() {
            lexer.step()
        }

        if &lexer.span() == &keyword::NONE {
            lexer.add(Token::None)
        }
    }

    pub fn lambda(lexer: &mut Lexer) {
        if lexer.current() == LAMBDA {
            lexer.step()
        }

        lexer.add(Token::Lambda)
    }

    pub fn equals(lexer: &mut Lexer) {
        if lexer.current() == EQUALS {
            lexer.step()
        }

        lexer.add(Token::Equals)
    }

    pub fn open_parenthesis(lexer: &mut Lexer) {
        if lexer.current() == OPEN_PARENTHESIS {
            lexer.step()
        }

        lexer.add(Token::OpenParenthesis)
    }

    pub fn close_parenthesis(lexer: &mut Lexer) {
        if lexer.current() == CLOSE_PARENTHESIS {
            lexer.step()
        }

        lexer.add(Token::CloseParenthesis)
    }

    pub fn colon(lexer: &mut Lexer) {
        if lexer.current() == COLON {
            lexer.step()
        }

        lexer.add(Token::Colon)
    }

    pub fn unknown(lexer: &mut Lexer) {
        lexer.step();
        lexer.add(Token::Unknown);
    }
}
