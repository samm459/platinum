#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    String,
    Boolean,
    Closure(Box<Type>, Box<Type>),
    None,
}
