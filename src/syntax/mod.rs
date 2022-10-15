pub mod parser;
pub mod token;

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

pub type Leaf = (Token, Description);
pub type Node = Leaf;
pub type Branch = Box<Syntax>;

#[derive(Debug, PartialEq, Clone)]
pub struct NameSyntax(pub Leaf);

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralSyntax(pub Leaf);

#[derive(Debug, PartialEq, Clone)]
pub struct CallSyntax(pub Branch, pub Branch);

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureSyntax {
    pub name: Leaf,
    pub colon: Leaf,
    pub r#type: Leaf,
    pub lambda: Leaf,
    pub expression: Branch,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub name: Leaf,
    pub equals: Leaf,
    pub expression: Branch,
}
