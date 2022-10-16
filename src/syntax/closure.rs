use super::{Branch, Leaf, Syntax, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureSyntax {
    pub name: Leaf,
    pub colon: Leaf,
    pub r#type: Leaf,
    pub lambda: Leaf,
    pub expression: Branch,
}

impl ClosureSyntax {
    pub fn parse(parser: &mut super::Parser) -> Self {
        ClosureSyntax {
            name: parser.expect(Token::Identifier),
            lambda: parser.assert(Token::Lambda),
            colon: parser.expect(Token::Colon),
            r#type: parser.expect(Token::Identifier),
            expression: Box::new(Syntax::parse(parser)),
        }
    }
}
