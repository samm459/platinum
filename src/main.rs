#![feature(box_syntax)]

pub mod core;
pub mod error;
pub mod interpreter;
pub mod repl;
pub mod syntax;

fn main() {
    repl::start();
}
