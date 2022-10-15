use super::*;
use crate::error::Error;

#[derive(Debug)]
pub struct Parser {
    pub position: usize,
    pub nodes: Vec<Node>,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn new(source: &str) -> Parser {
        let lexer = tokenize(source);
        let errors = lexer.errors.clone();
        Parser {
            position: 0,
            nodes: lexer.into(),
            errors,
        }
    }

    fn peek(&self, ahead: usize) -> Token {
        match self.nodes.get(self.position + ahead) {
            Some((token, _)) => *token,
            None => Token::EndOfFile,
        }
    }

    fn next(&mut self) -> Node {
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

    fn expect(&mut self, expected: Token) -> Node {
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

    fn assert(&mut self, expected: Token) -> Node {
        let (token, description) = self.next();
        if token == expected {
            (token, description)
        } else {
            panic!()
        }
    }

    fn current(&self) -> Token {
        match self.nodes.get(self.position) {
            Some((token, _description)) => *token,
            None => Token::EndOfFile,
        }
    }

    fn assignment(&mut self) -> AssignmentSyntax {
        AssignmentSyntax {
            name: self.expect(Token::Identifier),
            equals: self.assert(Token::Equals),
            expression: Box::new(self.parse()),
        }
    }

    fn closure(&mut self) -> ClosureSyntax {
        ClosureSyntax {
            name: self.expect(Token::Identifier),
            lambda: self.assert(Token::Lambda),
            colon: self.expect(Token::Colon),
            r#type: self.expect(Token::Identifier),
            expression: Box::new(self.parse()),
        }
    }

    fn primary(&mut self) -> Syntax {
        if self.current() == Token::OpenParenthesis {
            self.next();
            let expression = self.parse();
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

    fn call(&mut self) -> Syntax {
        let mut left = self.primary();

        while self.current() != Token::EndOfFile && self.current() != Token::CloseParenthesis {
            let right = self.primary();
            left = Syntax::Call(CallSyntax(Box::new(left), Box::new(right)))
        }

        left
    }

    pub fn parse(&mut self) -> Syntax {
        match self.peek(1) {
            Token::Equals => Syntax::Assignment(self.assignment()),
            Token::Lambda => Syntax::Closure(self.closure()),
            _ => self.call(),
        }
    }
}

pub fn parse(source: &str) -> (Syntax, Vec<Error>) {
    let mut parser = Parser::new(source);
    let syntax = parser.parse();
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
