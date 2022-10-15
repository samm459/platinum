mod library;
mod module;

use self::library::*;
use self::module::*;

pub fn build() -> Vec<Module> {
    vec![
        inc::define(),
        dec::define(),
        add::define(),
        sub::define(),
        mul::define(),
        div::define(),
    ]
}
