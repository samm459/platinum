pub mod clear;
pub mod escape_code;

use std::io;
use std::io::prelude::*;

use crate::clear;
use crate::interpreter::value::*;
use crate::interpreter::*;
use crate::syntax::*;

pub struct Repl {
    interpreter: Interpreter,
}

impl Repl {
    fn flush(c: Box<dyn Fn()>) {
        c();
        std::io::stdout().flush().unwrap();
    }

    fn clear() {
        Repl::flush(box || {
            clear!();
            print!("> ");
        });
    }

    fn new() -> Repl {
        Repl {
            interpreter: Interpreter::new(),
        }
    }

    pub fn r#loop(&mut self) {
        Repl::clear();

        for line in io::stdin().lock().lines() {
            let source = line.unwrap();

            if source == "#clear" {
                Repl::clear();
                continue;
            }

            if source == "#exit" {
                break;
            }

            let (syntax, syntax_errors) = parse(&source);

            if syntax_errors.len() > 0 {
                Repl::flush(box move || {
                    print!("{:?}\n>", syntax_errors[0]);
                });
                continue;
            }

            self.interpreter.set_source(&source);
            self.interpreter.bind(syntax.clone(), 0);

            let type_errors = self.interpreter.flush_errors();
            if type_errors.len() > 0 {
                Repl::flush(box move || {
                    print!("{:?}\n>", type_errors[0]);
                });
                continue;
            }

            let value = self.interpreter.eval(syntax.clone(), 0);

            match value {
                Value::None => {}
                _ => {
                    print!("{:?}\n", value);
                }
            }

            Repl::flush(box || {
                print!("> ");
            });
        }
    }

    pub fn start() {
        let mut repl = Repl::new();
        repl.r#loop();
    }
}
