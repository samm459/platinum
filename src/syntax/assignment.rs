use super::{Branch, Leaf, Syntax, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub name: Leaf,
    pub equals: Leaf,
    pub expression: Branch,
}

impl AssignmentSyntax {
    pub fn parse(parser: &mut super::Parser) -> AssignmentSyntax {
        AssignmentSyntax {
            name: parser.expect(Token::Identifier),
            equals: parser.assert(Token::Equals),
            expression: Box::new(Syntax::parse(parser)),
        }
    }
}
