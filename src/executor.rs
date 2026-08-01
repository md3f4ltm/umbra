use std::io;
use std::process::Command as ProcessCommand;

use crate::parser::Command;

pub fn execute(command: &Command) -> io::Result<i32> {
    let status = ProcessCommand::new(&command.program)
        .args(&command.args)
        .status();

    match status {
        Ok(status) => Ok(status.code().unwrap_or(1)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            eprintln!("umbra: command not fount {}", command.program);
            Ok(127)
        }
        Err(error) => {
            return Err(error);
        }
    }
}
