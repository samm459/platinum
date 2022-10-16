mod library;
mod module;

use crate::interpreter::r#type::Type;

use self::library::*;
use vec as core;
use vec as primitives;

pub fn build() -> CoreLibrary {
    core![
        inc::define(),
        dec::define(),
        add::define(),
        sub::define(),
        mul::define(),
        div::define(),
        cat::define(),
    ]
}

pub fn primitives() -> Primatives {
    primitives![
        ("string".into(), Type::String),
        ("number".into(), Type::Number),
        ("boolean".into(), Type::Boolean)
    ]
}
