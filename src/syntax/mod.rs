pub mod assignment;
pub mod call;
pub mod closure;
pub mod literal;
pub mod name;
pub mod parser;
pub mod token;

pub use self::assignment::*;
pub use self::call::*;
pub use self::closure::*;
pub use self::literal::*;
pub use self::name::*;
pub use self::parser::*;
pub use self::token::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Syntax {
    Name(NameSyntax),
    Literal(LiteralSyntax),
    Call(CallSyntax),
    Closure(ClosureSyntax),
    Assignment(AssignmentSyntax),
}

impl Syntax {
    pub fn parse(parser: &mut Parser) -> Syntax {
        match parser.peek(1) {
            Token::Equals => Syntax::Assignment(AssignmentSyntax::parse(parser)),
            Token::Lambda => Syntax::Closure(ClosureSyntax::parse(parser)),
            _ => CallSyntax::parse(parser),
        }
    }
}
