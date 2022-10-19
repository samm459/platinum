pub mod clear;
pub mod escape_code;

use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType::All},
    ExecutableCommand, Result,
};

struct Position {
    x: usize,
    y: usize,
}

pub struct Module {
    lines: Vec<String>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            lines: vec![String::new()],
        }
    }
}

pub struct Editor {
    modules: Vec<Module>,
    current: usize,
    position: Position,
}

impl Editor {
    fn render(&self) -> Result<()> {
        stdout().execute(Clear(All))?.execute(MoveTo(0, 0))?;

        for (i, line) in self.modules[self.current].lines.iter().enumerate() {
            let mut line = String::from(line);
            let length: i32 = line.len().try_into().unwrap();
            let x: i32 = self.position.x.try_into().unwrap();

            if self.position.y == i {
                if x > length - 1 {
                    for _ in 0..(x - (length - 1)) {
                        line += " "
                    }
                }

                line.replace_range(self.position.x..self.position.x + 1, "█");
            }

            let mut line_number = String::from("   ");
            line_number.replace_range(0..i.to_string().len() - 1, &i.to_string());

            stdout()
                .execute(Print(format!("{}| ", line_number).dim()))?
                .execute(Print(line))?
                .execute(Print("\r\n"))?;
        }

        Ok(())
    }
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            modules: vec![Module::new()],
            current: 0,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn start(&mut self) -> Result<()> {
        enable_raw_mode().unwrap();
        stdout().execute(Hide)?;

        std::panic::set_hook(Box::new(|info| {
            disable_raw_mode().unwrap();
            stdout().execute(Show).unwrap();
            println!("{}", info.to_string());
        }));

        loop {
            self.render()?;

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    self.position.y += 1;
                    self.position.x = 0;
                    self.modules[self.current]
                        .lines
                        .insert(self.position.y, String::new());
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    if self.position.x == 0 && self.position.y == 0 {
                        continue;
                    }

                    if self.position.x == 0 {
                        self.position.y -= 1;
                        self.position.x = self.modules[self.current].lines[self.position.y].len()
                    } else {
                        self.position.x -= 1;
                        self.modules[self.current].lines[self.position.y].remove(self.position.x);
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    self.modules[self.current].lines[self.position.y].insert(self.position.x, c);
                    self.position.x += 1
                }
                _ => {}
            }
        }

        stdout().execute(Show)?;
        Ok(())
    }
}
