pub mod clear;
pub mod escape_code;

use std::io;
use std::io::prelude::*;

use crate::clear;
use crate::interpreter::value::*;
use crate::interpreter::*;
use crate::syntax::*;

pub fn start() {
    let mut interpreter = Interpreter::new();

    clear!();
    print!("> ");
    std::io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        let source = line.unwrap();
        let (syntax, syntax_errors) = parse(&source);

        if syntax_errors.len() > 0 {
            print!("{:?}", syntax_errors[0]);
            print!("\n> ");
            std::io::stdout().flush().unwrap();
            continue;
        }

        interpreter.set_source(&source);
        interpreter.bind(syntax.clone(), 0);

        let type_errors = interpreter.flush_errors();
        if type_errors.len() > 0 {
            print!("{:?}", type_errors[0]);
            print!("\n> ");
            std::io::stdout().flush().unwrap();
            continue;
        }

        let value = interpreter.eval(syntax.clone(), 0);

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
