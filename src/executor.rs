use std::io;
use std::process::Command as ProcessCommand;

use crate::parser::Command;

pub fn execute(command: &Command) -> io::Result<()> {
    let result = ProcessCommand::new(&command.program)
        .args(&command.args)
        .status();

    match result {
        Ok(status) => {
            if !status.success() {
                eprintln!("umbra: {} exit with status {}", command.program, status);
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            eprintln!("umbra: command not fount {}", command.program);
        }
        Err(error) => {
            return Err(error);
        }
    }
    Ok(())
}
