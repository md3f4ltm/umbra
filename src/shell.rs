use crate::builtins;
use crate::builtins::BuiltinOutcome;
use crate::executor;
use crate::parser::{parse, Command};
use std::io::{self, Write};

pub struct Shell {
    prompt: String,
    continuation_prompt: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            prompt: "umbra> ".to_string(),
            continuation_prompt: "> ".to_string(),
        }
    }

    pub fn run(&mut self) -> io::Result<i32> {
        let mut last_status = 0;

        loop {
            let Some(command) = self.read_command()? else {
                // EOF or Ctrl-D
                return Ok(last_status);
            };

            let outcome = match builtins::execute(&command, last_status) {
                Some(result) => result?,
                None => {
                    let status = executor::execute(&command)?;
                    BuiltinOutcome::Status(status)
                }
            };

            match outcome {
                BuiltinOutcome::Status(status) => {
                    last_status = status;
                }
                BuiltinOutcome::Exit(code) => {
                    return Ok(code);
                }
            }
        }
    }

    fn read_command(&self) -> io::Result<Option<Command>> {
        let mut input = String::new();
        let mut continuing = false;

        loop {
            let prompt = if continuing {
                &self.continuation_prompt
            } else {
                &self.prompt
            };

            self.print_prompt(prompt)?;

            let mut line = String::new();

            let bytes_read = io::stdin().read_line(&mut line)?;

            if bytes_read == 0 {
                if !input.trim().is_empty() {
                    eprintln!("umbra: unexpected end of input");
                }
                return Ok(None);
            }

            input.push_str(&line);

            match parse(&input) {
                Ok(Some(command)) => {
                    return Ok(Some(command));
                }
                Ok(None) => {
                    input.clear();
                    continuing = false;
                }

                Err(error) if error.is_incomplete() => {
                    continuing = true;
                }
                Err(error) => {
                    eprintln!("umbra: syntax error: {error}");
                    input.clear();
                    continuing = false;
                }
            }
        }
    }

    fn print_prompt(&self, prompt: &str) -> io::Result<()> {
        print!("{prompt}");
        io::stdout().flush()
    }
}
