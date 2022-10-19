#![feature(box_syntax)]
#![feature(never_type)]
#![feature(backtrace)]

pub mod core;
pub mod editor;
pub mod error;
pub mod interpreter;
pub mod syntax;

use editor::Editor;

fn main() {
    let mut editor = Editor::new();
    editor.start().unwrap();
}
