mod library;
mod module;

use self::library::*;
use vec as core;

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
