pub mod clear;
pub mod escape_code;

use std::io;
use std::io::prelude::*;

use crate::clear;
use crate::interpreter::value::*;
use crate::interpreter::*;
use crate::syntax::*;

pub fn start() {
    let mut chain = Interpreter::new();

    clear!();
    print!("> ");
    std::io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        let source = line.unwrap();
        let (syntax, syntax_errors) = parse(&source);

        if syntax_errors.len() > 0 {
            syntax_errors.iter().for_each(|error| print!("{:?}", error));
            print!("\n> ");
            std::io::stdout().flush().unwrap();
            continue;
        }

        chain.link(source.clone(), syntax.clone(), 0);

        let type_errors = chain.flush_errors();
        if type_errors.len() > 0 {
            type_errors.iter().for_each(|error| print!("{:?}", error));
            print!("\n> ");
            std::io::stdout().flush().unwrap();
            continue;
        }

        let value = chain.eval(source.clone(), syntax.clone(), 0);

        match value {
            Value::None => {}
            _ => {
                print!("{:?}\n", value);
            }
        }

        print!("> ");
        std::io::stdout().flush().unwrap();
    }
}
