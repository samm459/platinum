use super::*;
use crate::error::Error;

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pub position: usize,
    pub description: Vec<Description>,
    pub tokens: Vec<Token>,
    pub reach: usize,
    pub flag: usize,
    pub errors: Vec<Error>,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: String::from(source),
            position: 0,
            description: Vec::new(),
            tokens: Vec::new(),
            reach: 0,
            flag: 0,
            errors: vec![],
        }
    }

    pub fn next(&mut self, token: Token, description: Description) {
        self.reach = 0;
        self.position += description.length;
        self.tokens.push(token);
        self.description.push(description);
    }

    pub fn chars(&self) -> Vec<char> {
        String::from(&self.source).chars().collect()
    }

    pub fn push(&mut self, token: Token, length: usize) {
        let description = Description {
            position: self.position,
            length,
        };
        self.next(token, description);
    }

    pub fn step(&mut self) {
        self.reach += 1;
    }

    pub fn add(&mut self, token: Token) {
        if self.reach > 0 {
            self.push(token, self.reach)
        }
    }

    pub fn current(&self) -> char {
        match self.chars().get(self.position + self.reach) {
            Some(char) => *char,
            None => TERMINATOR,
        }
    }

    pub fn span(&self) -> String {
        match self.chars().get(self.position..self.position + self.reach) {
            Some(slice) => slice.iter().collect(),
            None => String::new(),
        }
    }

    pub fn flag(&mut self) {
        self.flag = self.position
    }

    pub fn check(&mut self) {
        if self.flag == self.position {
            self.errors.push(Error::UnknownToken(
                self.position..self.position + 1,
                String::from(&self.source[self.position..self.position + 1]),
            ));
            Token::unknown(self);
        }

        self.flag = 0
    }

    pub fn register(&mut self, token: fn(&mut Lexer)) {
        token(self);
        self.reach = 0;
    }

    pub fn next_token(&mut self) {
        self.flag();
        register_tokens(self);
        self.check();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Description {
    pub position: usize,
    pub length: usize,
}
