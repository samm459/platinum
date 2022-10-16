use super::{Branch, Parser, Syntax, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct CallSyntax(pub Branch, pub Branch);

impl CallSyntax {
    pub fn parse(parser: &mut Parser) -> Syntax {
        let mut left = parser.primary();

        while parser.current() != Token::EndOfFile && parser.current() != Token::CloseParenthesis {
            let right = parser.primary();
            left = Syntax::Call(CallSyntax(Box::new(left), Box::new(right)))
        }

        left
    }
}
