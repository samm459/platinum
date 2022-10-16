use super::*;
use crate::error::Error;

#[derive(Debug)]
pub struct Parser {
    pub position: usize,
    pub nodes: Vec<Node>,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn new(source: &str, start: usize) -> Parser {
        let lexer = tokenize(source, start);
        let errors = lexer.errors.clone();
        Parser {
            position: 0,
            nodes: lexer.into(),
            errors,
        }
    }

    pub fn peek(&self, ahead: usize) -> Token {
        match self.nodes.get(self.position + ahead) {
            Some((token, _)) => *token,
            None => Token::EndOfFile,
        }
    }

    pub fn next(&mut self) -> Node {
        self.position += 1;
        match self.nodes.get(self.position - 1) {
            Some(_) => self.nodes[self.position - 1],
            None => (
                Token::EndOfFile,
                Description {
                    position: self.nodes.last().unwrap().1.position
                        + self.nodes.last().unwrap().1.position
                        + self.nodes.last().unwrap().1.length,
                    length: 0,
                },
            ),
        }
    }

    pub fn expect(&mut self, expected: Token) -> Node {
        let (token, description) = self.next();
        if token == expected {
            (token, description)
        } else {
            self.errors.push(Error::UnexpectedToken(
                description.position..description.position + description.length,
                expected,
                token,
            ));
            (expected, description)
        }
    }

    pub fn assert(&mut self, expected: Token) -> Node {
        let (token, description) = self.next();
        if token == expected {
            (token, description)
        } else {
            panic!()
        }
    }

    pub fn current(&self) -> Token {
        match self.nodes.get(self.position) {
            Some((token, _description)) => *token,
            None => Token::EndOfFile,
        }
    }

    pub fn primary(&mut self) -> Syntax {
        if self.current() == Token::OpenParenthesis {
            self.next();
            let expression = Syntax::parse(self);
            self.expect(Token::CloseParenthesis);
            return expression;
        }

        if self.current() == Token::Number
            || self.current() == Token::String
            || self.current() == Token::Boolean
            || self.current() == Token::None
        {
            return Syntax::Literal(LiteralSyntax(self.next()));
        }

        Syntax::Name(NameSyntax(self.expect(Token::Identifier)))
    }
}

pub fn parse(source: &str, start: usize) -> (Syntax, Vec<Error>) {
    let mut parser = Parser::new(source, start);
    let syntax = Syntax::parse(&mut parser);
    (syntax, parser.errors)
}

impl From<Lexer> for Vec<Node> {
    fn from(lexer: Lexer) -> Self {
        lexer
            .tokens
            .into_iter()
            .enumerate()
            .map(|(i, t)| (t, lexer.description[i]))
            .filter(|(t, _)| *t != Token::Space)
            .collect::<Vec<Node>>()
    }
}

pub type Leaf = (Token, Description);
pub type Node = Leaf;
pub type Branch = Box<Syntax>;
