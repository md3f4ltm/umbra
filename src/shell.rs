use crate::executor;
use crate::parser;
use std::io::{self, Write};

pub struct Shell {
    prompt: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            prompt: "umbra> ".to_string(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            print!("{}", self.prompt);
            io::stdout().flush()?;

            let mut line = String::new();

            if io::stdin().read_line(&mut line)? == 0 {
                println!();
                break;
            }

            if let Some(command) = parser::parse(&line) {
                println!("Program: {}", command.program);
                println!("Args: {:?}", command.args);
                println!("------------------------------------");
                executor::execute(&command)?;
            }
        }

        Ok(())
    }
}
