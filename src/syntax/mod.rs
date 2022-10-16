pub mod assignment;
pub mod call;
pub mod closure;
pub mod literal;
pub mod name;
pub mod parser;
pub mod token;
pub mod type_expression;

pub use self::assignment::*;
pub use self::call::*;
pub use self::closure::*;
pub use self::literal::*;
pub use self::name::*;
pub use self::parser::*;
pub use self::token::*;
pub use self::type_expression::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Syntax {
    Name(NameSyntax),
    Literal(LiteralSyntax),
    Call(CallSyntax),
    Closure(ClosureSyntax),
    Assignment(AssignmentSyntax),
    TypeExpression(TypeExpressionSyntax),
}

impl Syntax {
    pub fn parse_with_type(parser: &mut Parser) -> Syntax {
        let name = parser.expect(Token::Identifier);
        let type_expression = TypeExpressionSyntax::parse(parser);
        match parser.current() {
            Token::Equals => Syntax::Assignment(AssignmentSyntax::parse_with_type(
                name,
                type_expression,
                parser,
            )),
            Token::Lambda => Syntax::Closure(ClosureSyntax::parse(name, type_expression, parser)),
            _ => panic!(),
        }
    }

    pub fn parse(parser: &mut Parser) -> Syntax {
        match parser.peek(1) {
            Token::Colon => Syntax::parse_with_type(parser),
            Token::Equals => Syntax::Assignment(AssignmentSyntax::parse(parser)),
            _ => CallSyntax::parse(parser),
        }
    }
}
