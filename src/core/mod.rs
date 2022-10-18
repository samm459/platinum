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
        ("String".into(), Type::String),
        ("Number".into(), Type::Number),
        ("Boolean".into(), Type::Boolean)
    ]
}
