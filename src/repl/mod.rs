pub mod clear;
pub mod escape_code;

use std::io;
use std::io::prelude::*;

use crate::clear;
use crate::interpreter::value::*;
use crate::interpreter::*;
use crate::syntax::*;

fn flush(c: Box<dyn Fn()>) {
    c();
    std::io::stdout().flush().unwrap();
}

fn clear() {
    flush(box || {
        clear!();
        print!("> ");
    });
}

pub fn start() {
    let mut interpreter = Interpreter::new();

    clear();

    for line in io::stdin().lock().lines() {
        let source = line.unwrap();

        if source == "#clear" {
            clear();
            continue;
        }

        if source == "#exit" {
            break;
        }

        let (syntax, syntax_errors) = parse(&source);

        if syntax_errors.len() > 0 {
            flush(box move || {
                print!("{:?}\n>", syntax_errors[0]);
            });
            continue;
        }

        interpreter.set_source(&source);
        interpreter.bind(syntax.clone(), 0);

        let type_errors = interpreter.flush_errors();
        if type_errors.len() > 0 {
            flush(box move || {
                print!("{:?}\n>", type_errors[0]);
            });
            continue;
        }

        let value = interpreter.eval(syntax.clone(), 0);

        match value {
            Value::None => {}
            _ => {
                print!("{:?}\n", value);
            }
        }

        flush(box || {
            print!("> ");
        });
    }
}
